use cairo_sys::*;
use gdk_pixbuf_sys::*;
use gdk_sys::*;
use gio_sys::*;
use glib_sys::*;
use gobject_sys::*;
use gtk_sys::*;
use libc::*;
use luakit_common::common::luaobject::luaH_object_property_signal;
use mlua_sys::*;
use webkit2gtk_sys::*;

use crate::common::luaclass::*;
use crate::common::luah::*;
use crate::common::resource::*;
use crate::common::tokenize::*;
use crate::gtypes::*;
use crate::log::*;
use crate::web_context_get;
use crate::widgets::common::*;
use crate::widgets::*;

unsafe extern "C" fn luaH_checkimage(mut L: *mut lua_State, mut udx: gint) -> *mut widget_t {
    let mut w = luaH_checkwidget(L, udx);
    if (*(*w).info).tok as std::ffi::c_uint != L_TK_IMAGE as std::ffi::c_int as std::ffi::c_uint {
        luaL_argerror(
            L,
            udx,
            b"incorrect widget type (expected image)\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    return w;
}

unsafe extern "C-unwind" fn luaH_image_set_from_file_name(mut L: *mut lua_State) -> gint {
    let mut pixbuf: *mut GdkPixbuf = 0 as *mut GdkPixbuf;
    let mut w = luaH_checkimage(L, 1 as std::ffi::c_int);
    let mut path = luaL_checklstring(L, 2 as std::ffi::c_int, std::ptr::null_mut()) as *mut gchar;
    let mut x2_path = std::ptr::null_mut();
    let mut scale = gtk_widget_get_scale_factor((*w).widget) as std::ffi::c_float;
    path = resource_find_file(path);
    if path.is_null() {
        return luaL_error(
            L,
            b"unable to find image file\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    if scale == 2 as std::ffi::c_int as std::ffi::c_float {
        let ref mut fresh0 = strrchr(path, '.' as i32);
        let mut ext: *const gchar = if !(*fresh0).is_null() {
            *fresh0
        } else {
            &mut *path.offset(strlen(path) as isize) as *mut gchar
        };
        x2_path = g_strdup_printf(
            b"%.*s@2x%s\0" as *const u8 as *const std::ffi::c_char,
            ext.offset_from(path) as std::ffi::c_long as std::ffi::c_int,
            path,
            ext,
        );
        if g_file_test(x2_path, G_FILE_TEST_IS_REGULAR) == 0 {
            g_free(x2_path as gpointer);
            x2_path = std::ptr::null_mut();
        }
    }
    let mut error = 0 as *mut GError;
    loop {
        error = std::ptr::null_mut();
        pixbuf =
            gdk_pixbuf_new_from_file(if !x2_path.is_null() { x2_path } else { path }, &mut error);
        if !error.is_null() {
            _log(
                LOG_LEVEL_verbose,
                b"widgets/image.c\0" as *const u8 as *const std::ffi::c_char,
                b"unable to load image file: %s\0" as *const u8 as *const std::ffi::c_char,
                (*error).message,
            );
        }
        if !(!error.is_null() && !x2_path.is_null()) {
            break;
        }
        g_error_free(error);
        g_free(x2_path as gpointer);
        x2_path = std::ptr::null_mut();
    }
    if !error.is_null() {
        lua_pushstring(L, (*error).message);
        g_error_free(error);
        g_free(path as gpointer);
        return luaL_error(
            L,
            b"unable to load image file: %s\0" as *const u8 as *const std::ffi::c_char,
            lua_tolstring(L, -(1 as std::ffi::c_int), std::ptr::null_mut()),
        );
    }
    if !((*w).data).is_null() {
        g_cancellable_cancel((*w).data as *mut GCancellable);
        let mut _pp = &mut (*w).data;
        let mut _ptr = *_pp as *mut GObject;
        *_pp = std::ptr::null_mut();
        if !_ptr.is_null() {
            g_object_unref(_ptr);
        }
    }
    let mut source =
        gdk_cairo_surface_create_from_pixbuf(pixbuf, 1 as std::ffi::c_int, 0 as *mut GdkWindow);
    g_object_unref(g_type_check_instance_cast(
        pixbuf as *mut GTypeInstance,
        ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
    ) as *mut std::ffi::c_void as *mut GObject);
    let mut src_w = cairo_image_surface_get_width(source) as std::ffi::c_float;
    let mut src_h = cairo_image_surface_get_height(source) as std::ffi::c_float;
    let mut target = cairo_surface_create_similar(
        source,
        CONTENT_COLOR_ALPHA,
        src_w as std::ffi::c_int,
        src_h as std::ffi::c_int,
    );
    cairo_surface_set_device_scale(
        target,
        scale as std::ffi::c_double,
        scale as std::ffi::c_double,
    );
    let mut cr = cairo_create(target);
    cairo_scale(
        cr,
        (1 as std::ffi::c_int as std::ffi::c_float / scale) as std::ffi::c_double,
        (1 as std::ffi::c_int as std::ffi::c_float / scale) as std::ffi::c_double,
    );
    cairo_set_source_surface(
        cr,
        source,
        0 as std::ffi::c_int as std::ffi::c_double,
        0 as std::ffi::c_int as std::ffi::c_double,
    );
    cairo_surface_set_device_offset(
        source,
        0 as std::ffi::c_int as std::ffi::c_double,
        0 as std::ffi::c_int as std::ffi::c_double,
    );
    cairo_paint(cr);
    gtk_image_set_from_surface(
        g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_image_get_type())
            as *mut std::ffi::c_void as *mut GtkImage,
        target,
    );
    cairo_surface_destroy(source);
    cairo_surface_destroy(target);
    cairo_destroy(cr);
    g_free(path as gpointer);
    g_free(x2_path as gpointer);
    return 0 as std::ffi::c_int;
}

unsafe extern "C-unwind" fn luaH_image_set_from_icon_name(mut L: *mut lua_State) -> gint {
    let mut w = luaH_checkimage(L, 1 as std::ffi::c_int);
    let mut size = GTK_ICON_SIZE_INVALID;
    match luaL_checkinteger(L, 3 as std::ffi::c_int) as std::ffi::c_int {
        16 => {
            size = GTK_ICON_SIZE_SMALL_TOOLBAR;
        }
        24 => {
            size = GTK_ICON_SIZE_LARGE_TOOLBAR;
        }
        32 => {
            size = GTK_ICON_SIZE_DND;
        }
        48 => {
            size = GTK_ICON_SIZE_DIALOG;
        }
        _ => {
            return luaL_error(
                L,
                b"Bad icon size: must be 16, 24, 32, or 48.\0" as *const u8
                    as *const std::ffi::c_char,
            );
        }
    }
    if !((*w).data).is_null() {
        g_cancellable_cancel((*w).data as *mut GCancellable);
        let mut _pp = &mut (*w).data;
        let mut _ptr = *_pp as *mut GObject;
        *_pp = std::ptr::null_mut();
        if !_ptr.is_null() {
            g_object_unref(_ptr);
        }
    }
    gtk_image_set_from_icon_name(
        g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_image_get_type())
            as *mut std::ffi::c_void as *mut GtkImage,
        luaL_checklstring(L, 2 as std::ffi::c_int, std::ptr::null_mut()),
        size,
    );
    return 0 as std::ffi::c_int;
}

unsafe extern "C-unwind" fn luaH_image_scale(mut L: *mut lua_State) -> gint {
    let mut w = luaH_checkimage(L, 1 as std::ffi::c_int);
    let mut width = luaL_checkinteger(L, 2 as std::ffi::c_int) as std::ffi::c_int;
    let mut height = (if lua_type(L, 3 as std::ffi::c_int) == LUA_TNIL {
        width as lua_Integer
    } else {
        luaL_checkinteger(L, 3 as std::ffi::c_int)
    }) as std::ffi::c_int;
    if width <= 0 as std::ffi::c_int || height <= 0 as std::ffi::c_int {
        return luaL_error(
            L,
            b"Image dimensions must be positive\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    let mut pixbuf = gtk_image_get_pixbuf(g_type_check_instance_cast(
        (*w).widget as *mut GTypeInstance,
        gtk_image_get_type(),
    ) as *mut std::ffi::c_void as *mut GtkImage);
    let mut scaled_pixbuf = gdk_pixbuf_scale_simple(pixbuf, width, height, GDK_INTERP_BILINEAR);
    g_object_unref(pixbuf as *mut GObject);
    gtk_image_set_from_pixbuf(
        g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_image_get_type())
            as *mut std::ffi::c_void as *mut GtkImage,
        scaled_pixbuf,
    );
    g_object_unref(scaled_pixbuf as *mut GObject);
    return 0 as std::ffi::c_int;
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn luaH_image_set_favicon_for_uri_finished(
    mut fdb: *mut WebKitFaviconDatabase,
    mut res: *mut GAsyncResult,
    mut w: *mut widget_t,
) {
    let mut source = webkit_favicon_database_get_favicon_finish(fdb, res, std::ptr::null_mut());
    if source.is_null() {
        return;
    }
    let mut src_w = cairo_image_surface_get_width(source) as std::ffi::c_float;
    let mut src_h = cairo_image_surface_get_height(source) as std::ffi::c_float;
    let mut scale = gtk_widget_get_scale_factor((*w).widget) as std::ffi::c_float;
    let mut log_sz = 16 as std::ffi::c_int as std::ffi::c_float;
    let mut dev_sz = log_sz * scale;
    let mut target = cairo_surface_create_similar(
        source,
        CONTENT_COLOR_ALPHA,
        dev_sz as std::ffi::c_int,
        dev_sz as std::ffi::c_int,
    );
    cairo_surface_set_device_scale(
        target,
        scale as std::ffi::c_double,
        scale as std::ffi::c_double,
    );
    let mut cr = cairo_create(target);
    cairo_scale(
        cr,
        (log_sz / src_w) as std::ffi::c_double,
        (log_sz / src_h) as std::ffi::c_double,
    );
    cairo_set_source_surface(
        cr,
        source,
        0 as std::ffi::c_int as std::ffi::c_double,
        0 as std::ffi::c_int as std::ffi::c_double,
    );
    cairo_surface_set_device_offset(
        source,
        0 as std::ffi::c_int as std::ffi::c_double,
        0 as std::ffi::c_int as std::ffi::c_double,
    );
    cairo_paint(cr);
    gtk_image_set_from_surface(
        g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_image_get_type())
            as *mut std::ffi::c_void as *mut GtkImage,
        target,
    );
    cairo_surface_destroy(source);
    cairo_surface_destroy(target);
    cairo_destroy(cr);
}

unsafe extern "C-unwind" fn luaH_image_set_favicon_for_uri(mut L: *mut lua_State) -> gint {
    let mut w = luaH_checkimage(L, 1 as std::ffi::c_int);
    let mut uri = luaL_checklstring(L, 2 as std::ffi::c_int, std::ptr::null_mut());
    let mut main_ctx = web_context_get();
    let mut main_fdb = webkit_web_context_get_favicon_database(main_ctx);
    let mut f_uri = 0 as *mut gchar;
    let mut ok = GTRUE;
    f_uri = webkit_favicon_database_get_favicon_uri(main_fdb, uri);
    if !f_uri.is_null() {
        g_free(f_uri as gpointer);
        if !((*w).data).is_null() {
            g_cancellable_cancel((*w).data as *mut GCancellable);
            let mut _pp = &mut (*w).data;
            let mut _ptr = *_pp as *mut GObject;
            *_pp = std::ptr::null_mut();
            if !_ptr.is_null() {
                g_object_unref(_ptr);
            }
        }
        (*w).data = g_cancellable_new() as gpointer;
        webkit_favicon_database_get_favicon(
            main_fdb,
            uri,
            (*w).data as *mut GCancellable,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut WebKitFaviconDatabase,
                        *mut GAsyncResult,
                        *mut widget_t,
                    ) -> (),
                >,
                GAsyncReadyCallback,
            >(Some(luaH_image_set_favicon_for_uri_finished)),
            w as gpointer,
        );
    } else {
        ok = GFALSE;
    }
    lua_pushboolean(L, ok);
    return 1 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_image_index(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
    mut token: luakit_token_t,
) -> gint {
    match token as std::ffi::c_uint {
        162 => return luaH_widget_get_parent(L, w),
        100 => return luaH_widget_get_focused(L, w),
        253 => return luaH_widget_get_visible(L, w),
        241 => return luaH_widget_get_tooltip(L, w),
        262 => return luaH_widget_get_width(L, w),
        109 => return luaH_widget_get_height(L, w),
        149 => return luaH_widget_get_min_size(L, w),
        3 => return luaH_widget_get_align(L, w),
        24 => return luaH_widget_get_children(L, w),
        211 => {
            lua_pushcclosure(L, luaH_widget_show, 0 as std::ffi::c_int);
            return 1 as std::ffi::c_int;
        }
        110 => {
            lua_pushcclosure(L, luaH_widget_hide, 0 as std::ffi::c_int);
            return 1 as std::ffi::c_int;
        }
        99 => {
            lua_pushcclosure(L, luaH_widget_focus, 0 as std::ffi::c_int);
            return 1 as std::ffi::c_int;
        }
        50 => {
            lua_pushcclosure(L, luaH_widget_destroy, 0 as std::ffi::c_int);
            return 1 as std::ffi::c_int;
        }
        183 => {
            lua_pushcclosure(L, luaH_widget_replace, 0 as std::ffi::c_int);
            return 1 as std::ffi::c_int;
        }
        203 => {
            lua_pushcclosure(L, luaH_widget_send_key, 0 as std::ffi::c_int);
            return 1 as std::ffi::c_int;
        }
        95 => {
            lua_pushcclosure(L, luaH_image_set_from_file_name, 0 as std::ffi::c_int);
            return 1 as std::ffi::c_int;
        }
        116 => {
            lua_pushcclosure(L, luaH_image_set_from_icon_name, 0 as std::ffi::c_int);
            return 1 as std::ffi::c_int;
        }
        189 => {
            lua_pushcclosure(L, luaH_image_scale, 0 as std::ffi::c_int);
            return 1 as std::ffi::c_int;
        }
        208 => {
            lua_pushcclosure(L, luaH_image_set_favicon_for_uri, 0 as std::ffi::c_int);
            return 1 as std::ffi::c_int;
        }
        _ => {}
    }
    return 0 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_image_newindex(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
    mut token: luakit_token_t,
) -> gint {
    match token as std::ffi::c_uint {
        253 => {
            luaH_widget_set_visible(L, w);
        }
        241 => {
            luaH_widget_set_tooltip(L, w);
        }
        149 => {
            luaH_widget_set_min_size(L, w);
        }
        3 => {
            luaH_widget_set_align(L, w);
        }
        _ => return 0 as std::ffi::c_int,
    }
    return luaH_object_property_signal(L, 1 as std::ffi::c_int, token);
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn widget_image(
    mut UNUSED_L: *mut lua_State,
    mut w: *mut widget_t,
    mut UNUSED_token: luakit_token_t,
) -> *mut widget_t {
    (*w).index = Some(luaH_image_index);
    (*w).newindex = Some(luaH_image_newindex);
    (*w).widget = gtk_image_new();
    (*w).data = std::ptr::null_mut();
    g_object_connect(
        g_type_check_instance_cast(
            (*w).widget as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject,
        b"signal::destroy\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GtkWidget, *mut widget_t) -> ()>,
            GCallback,
        >(Some(
            destroy_cb as unsafe extern "C" fn(*mut GtkWidget, *mut widget_t) -> (),
        )),
        w,
        b"signal::size-allocate\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GtkWidget, *mut GdkRectangle, *mut widget_t) -> ()>,
            GCallback,
        >(Some(
            resize_cb
                as unsafe extern "C" fn(*mut GtkWidget, *mut GdkRectangle, *mut widget_t) -> (),
        )),
        w,
        b"signal::focus-in-event\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(*mut GtkWidget, *mut GdkEventFocus, *mut widget_t) -> gboolean,
            >,
            GCallback,
        >(Some(
            focus_cb
                as unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut GdkEventFocus,
                    *mut widget_t,
                ) -> gboolean,
        )),
        w,
        b"signal::focus-out-event\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(*mut GtkWidget, *mut GdkEventFocus, *mut widget_t) -> gboolean,
            >,
            GCallback,
        >(Some(
            focus_cb
                as unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut GdkEventFocus,
                    *mut widget_t,
                ) -> gboolean,
        )),
        w,
        b"signal::parent-set\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GtkWidget, *mut GtkWidget, *mut widget_t) -> ()>,
            GCallback,
        >(Some(
            parent_set_cb
                as unsafe extern "C" fn(*mut GtkWidget, *mut GtkWidget, *mut widget_t) -> (),
        )),
        w,
        // std::ptr::null_mut(),
    );
    gtk_widget_show((*w).widget);
    return w;
}
