use crate::common::luaobject::luaH_object_property_signal;
use crate::common::luaobject::luaH_object_push;
use crate::gtypes::*;
use crate::widgets::common::destroy_cb;
use crate::widgets::common::focus_cb;
use crate::widgets::common::luaH_widget_destroy;
use crate::widgets::common::luaH_widget_focus;
use crate::widgets::common::luaH_widget_get_align;
use crate::widgets::common::luaH_widget_get_children;
use crate::widgets::common::luaH_widget_get_focused;
use crate::widgets::common::luaH_widget_get_height;
use crate::widgets::common::luaH_widget_get_min_size;
use crate::widgets::common::luaH_widget_get_parent;
use crate::widgets::common::luaH_widget_get_tooltip;
use crate::widgets::common::luaH_widget_get_visible;
use crate::widgets::common::luaH_widget_get_width;
use crate::widgets::common::luaH_widget_hide;
use crate::widgets::common::luaH_widget_replace;
use crate::widgets::common::luaH_widget_send_key;
use crate::widgets::common::luaH_widget_set_align;
use crate::widgets::common::luaH_widget_set_min_size;
use crate::widgets::common::luaH_widget_set_tooltip;
use crate::widgets::common::luaH_widget_set_visible;
use crate::widgets::common::luaH_widget_show;
use crate::widgets::common::parent_set_cb;
use crate::widgets::common::resize_cb;
use cairo_sys::*;
use gdk_sys::*;
use glib_sys::*;
use gobject_sys::*;
use gtk_sys::*;
use libc::*;
use luakit_common::common::luaobject::luaH_object_emit_signal;
use luakit_common::common::luaobject::luaH_object_ref;
use mlua_sys::*;

use crate::common::common;
use crate::common::luaclass::*;
use crate::common::luah::*;
use crate::common::tokenize::*;
use crate::widgets::*;

static mut ffi_new_ref: gpointer = 0 as *const std::ffi::c_void as *mut std::ffi::c_void;

unsafe extern "C-unwind" fn luaH_drawing_area_invalidate(mut L: *mut lua_State) -> gint {
    let mut w = luaH_checkwidget(L, 1 as std::ffi::c_int);
    let mut width = gtk_widget_get_allocated_width((*w).widget) as guint;
    let mut height = gtk_widget_get_allocated_height((*w).widget) as guint;
    gtk_widget_queue_draw_area(
        (*w).widget,
        0 as std::ffi::c_int,
        0 as std::ffi::c_int,
        width as gint,
        height as gint,
    );
    return 0 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_drawing_area_index(
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
        128 => {
            lua_pushcclosure(L, luaH_drawing_area_invalidate, 0 as std::ffi::c_int);
            return 1 as std::ffi::c_int;
        }
        _ => {}
    }
    return 0 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_drawing_area_newindex(
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
        _ => {}
    }
    return luaH_object_property_signal(L, 1 as std::ffi::c_int, token);
}

unsafe extern "C" fn drawing_area_draw_cb(
    mut UNUSED_widget: *mut GtkWidget,
    mut cr: *mut cairo_t,
    mut w: *mut widget_t,
) -> gboolean {
    let mut L = common.L;
    luaH_object_push(L, (*w).ref_0);
    luaH_object_push(L, ffi_new_ref);
    lua_pushlstring(
        L,
        b"cairo_t *\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 10]>())
            .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
            .wrapping_sub(1),
    );
    lua_pushlightuserdata(L, cr as *mut std::ffi::c_void);
    let mut error = lua_pcall(
        L,
        2 as std::ffi::c_int,
        1 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    if error == 0 as std::ffi::c_int {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN as *const std::ffi::c_char,
            b"widgets/drawing_area.c\0" as *const u8 as *const std::ffi::c_char,
            68 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 21], &[std::ffi::c_char; 21]>(
                b"drawing_area_draw_cb\0",
            ))
            .as_ptr(),
            b"error == 0\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    luaH_object_emit_signal(
        L,
        -(2 as std::ffi::c_int),
        b"draw\0" as *const u8 as *const std::ffi::c_char,
        1 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    return GFALSE;
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn widget_drawing_area(
    mut UNUSED_L: *mut lua_State,
    mut w: *mut widget_t,
    mut UNUSED_token: luakit_token_t,
) -> *mut widget_t {
    (*w).index = Some(luaH_drawing_area_index);
    (*w).newindex = Some(luaH_drawing_area_newindex);
    if ffi_new_ref.is_null() {
        let mut L = common.L;
        lua_getfield(
            L,
            LUA_GLOBALSINDEX,
            b"require\0" as *const u8 as *const std::ffi::c_char,
        );
        lua_pushlstring(
            L,
            b"ffi\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 4]>())
                .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
                .wrapping_sub(1),
        );
        let mut error = lua_pcall(
            L,
            1 as std::ffi::c_int,
            1 as std::ffi::c_int,
            0 as std::ffi::c_int,
        );
        if error == 0 as std::ffi::c_int {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN as *const std::ffi::c_char,
                b"widgets/drawing_area.c\0" as *const u8 as *const std::ffi::c_char,
                88 as std::ffi::c_int,
                (*::core::mem::transmute::<&[u8; 20], &[std::ffi::c_char; 20]>(
                    b"widget_drawing_area\0",
                ))
                .as_ptr(),
                b"error == 0\0" as *const u8 as *const std::ffi::c_char,
            );
        }
        if !(lua_type(L, -(1 as std::ffi::c_int)) == LUA_TTABLE) {
            luaL_error(
                L,
                b"Cannot create/use drawing area without ffi\0" as *const u8
                    as *const std::ffi::c_char,
            );
        }
        lua_getfield(
            L,
            -(1 as std::ffi::c_int),
            b"new\0" as *const u8 as *const std::ffi::c_char,
        );
        ffi_new_ref = luaH_object_ref(L, -(1 as std::ffi::c_int));
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    }
    (*w).widget = gtk_drawing_area_new();
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
        b"draw\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GtkWidget, *mut cairo_t, *mut widget_t) -> gboolean>,
            GCallback,
        >(Some(
            drawing_area_draw_cb
                as unsafe extern "C" fn(*mut GtkWidget, *mut cairo_t, *mut widget_t) -> gboolean,
        )),
        w,
        // NULL as *mut std::ffi::c_void,
    );
    gtk_widget_show((*w).widget);
    return w;
}
