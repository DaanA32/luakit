use cairo_sys::*;
use gdk_pixbuf_sys::*;
use gdk_sys::*;
use gio_sys::*;
use glib_sys::*;
use gobject_sys::*;
use gtk_sys::*;
use libc::*;
use mlua_sys::*;
use pango_sys::*;
use webkit2gtk_sys::*;

use crate::clib::widget::widget_set_css_properties;
use crate::common::common;
use crate::common::luaclass::*;
use crate::common::luah::*;
use crate::common::luaobject::*;
use crate::common::resource::*;
use crate::common::tokenize::*;
use crate::gtypes::*;
use crate::log::*;
use crate::web_context_get;
use crate::widgets::common::*;
use crate::widgets::*;

unsafe extern "C-unwind" fn luaH_overlay_pack(mut L: *mut lua_State) -> gint {
    let mut w = luaH_checkwidget(L, 1 as std::ffi::c_int);
    let mut child = luaH_checkwidget(L, 2 as std::ffi::c_int);
    let mut top = lua_gettop(L);
    let mut halign = GTK_ALIGN_FILL;
    let mut valign = GTK_ALIGN_FILL;
    if top > 2 as std::ffi::c_int && !(lua_type(L, 3 as std::ffi::c_int) == LUA_TNIL) {
        if !(lua_type(L, 3 as std::ffi::c_int) == LUA_TTABLE) {
            luaL_typerror(
                L,
                3 as std::ffi::c_int,
                b"table\0" as *const u8 as *const std::ffi::c_char,
            );
        }
        if luaH_rawfield(
            L,
            3 as std::ffi::c_int,
            b"halign\0" as *const u8 as *const std::ffi::c_char,
        ) != 0
        {
            match l_tokenize(lua_tolstring(
                L,
                -(1 as std::ffi::c_int),
                std::ptr::null_mut(),
            )) as std::ffi::c_uint
            {
                96 => {
                    halign = GTK_ALIGN_FILL;
                }
                224 => {
                    halign = GTK_ALIGN_START;
                }
                86 => {
                    halign = GTK_ALIGN_END;
                }
                20 => {
                    halign = GTK_ALIGN_CENTER;
                }
                12 => {
                    halign = GTK_ALIGN_BASELINE;
                }
                _ => {
                    return luaL_error(
                        L,
                        b"Bad alignment value (expected fill, start, end, center, or baseline)\0"
                            as *const u8 as *const std::ffi::c_char,
                    );
                }
            }
        }
        if luaH_rawfield(
            L,
            3 as std::ffi::c_int,
            b"valign\0" as *const u8 as *const std::ffi::c_char,
        ) != 0
        {
            match l_tokenize(lua_tolstring(
                L,
                -(1 as std::ffi::c_int),
                std::ptr::null_mut(),
            )) as std::ffi::c_uint
            {
                96 => {
                    valign = GTK_ALIGN_FILL;
                }
                224 => {
                    valign = GTK_ALIGN_START;
                }
                86 => {
                    valign = GTK_ALIGN_END;
                }
                20 => {
                    valign = GTK_ALIGN_CENTER;
                }
                12 => {
                    valign = GTK_ALIGN_BASELINE;
                }
                _ => {
                    return luaL_error(
                        L,
                        b"Bad alignment value (expected fill, start, end, center, or baseline)\0"
                            as *const u8 as *const std::ffi::c_char,
                    );
                }
            }
        }
        lua_settop(L, top);
    }
    gtk_widget_set_halign(
        g_type_check_instance_cast((*child).widget as *mut GTypeInstance, gtk_widget_get_type())
            as *mut std::ffi::c_void as *mut GtkWidget,
        halign,
    );
    gtk_widget_set_valign(
        g_type_check_instance_cast((*child).widget as *mut GTypeInstance, gtk_widget_get_type())
            as *mut std::ffi::c_void as *mut GtkWidget,
        valign,
    );
    gtk_overlay_add_overlay(
        g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_overlay_get_type())
            as *mut std::ffi::c_void as *mut GtkOverlay,
        g_type_check_instance_cast((*child).widget as *mut GTypeInstance, gtk_widget_get_type())
            as *mut std::ffi::c_void as *mut GtkWidget,
    );
    return 0 as std::ffi::c_int;
}

unsafe extern "C-unwind" fn luaH_overlay_reorder_child(mut L: *mut lua_State) -> gint {
    let mut w = luaH_checkwidget(L, 1 as std::ffi::c_int);
    let mut child = luaH_checkwidget(L, 2 as std::ffi::c_int);
    let mut pos = luaL_checknumber(L, 3 as std::ffi::c_int) as gint;
    gtk_overlay_reorder_overlay(
        g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_overlay_get_type())
            as *mut std::ffi::c_void as *mut GtkOverlay,
        g_type_check_instance_cast((*child).widget as *mut GTypeInstance, gtk_widget_get_type())
            as *mut std::ffi::c_void as *mut GtkWidget,
        pos,
    );
    return 0 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_overlay_index(
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
        23 => return luaH_widget_get_child(L, w),
        180 => {
            lua_pushcclosure(L, luaH_widget_remove, 0 as std::ffi::c_int);
            return 1 as std::ffi::c_int;
        }
        159 => {
            lua_pushcclosure(L, luaH_overlay_pack, 0 as std::ffi::c_int);
            return 1 as std::ffi::c_int;
        }
        182 => {
            lua_pushcclosure(L, luaH_overlay_reorder_child, 0 as std::ffi::c_int);
            return 1 as std::ffi::c_int;
        }
        _ => {}
    }
    return 0 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_overlay_newindex(
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
        23 => {
            luaH_widget_set_child(L, w);
        }
        _ => return 0 as std::ffi::c_int,
    }
    return luaH_object_property_signal(L, 1 as std::ffi::c_int, token);
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn widget_overlay(
    mut UNUSED_L: *mut lua_State,
    mut w: *mut widget_t,
    mut UNUSED_token: luakit_token_t,
) -> *mut widget_t {
    (*w).index = Some(luaH_overlay_index);
    (*w).newindex = Some(luaH_overlay_newindex);
    (*w).widget = gtk_overlay_new();
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
