use cairo_sys::*;
use gdk_pixbuf_sys::*;
use gdk_sys::*;
use gio_sys::*;
use glib_sys::*;
use gobject_sys::*;
use gtk_sys::*;
use libc::*;
use mlua_sys::*;
use webkit2gtk_sys::*;

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

unsafe extern "C-unwind" fn luaH_stack_pack(mut L: *mut lua_State) -> gint {
    let mut w = luaH_checkwidget(L, 1 as std::ffi::c_int);
    let mut child = luaH_checkwidget(L, 2 as std::ffi::c_int);
    gtk_container_add(
        g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_container_get_type())
            as *mut std::ffi::c_void as *mut GtkContainer,
        g_type_check_instance_cast((*child).widget as *mut GTypeInstance, gtk_widget_get_type())
            as *mut std::ffi::c_void as *mut GtkWidget,
    );
    return 0 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_stack_index(
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
            lua_pushcclosure(L, luaH_widget_replace, 0);
            return 1;
        }
        203 => {
            lua_pushcclosure(L, luaH_widget_send_key, 0 as std::ffi::c_int);
            return 1 as std::ffi::c_int;
        }
        180 => {
            lua_pushcclosure(L, luaH_widget_remove, 0 as std::ffi::c_int);
            return 1 as std::ffi::c_int;
        }
        159 => {
            lua_pushcclosure(L, luaH_stack_pack, 0 as std::ffi::c_int);
            return 1 as std::ffi::c_int;
        }
        112 => {
            lua_pushboolean(
                L,
                gtk_stack_get_homogeneous(g_type_check_instance_cast(
                    (*w).widget as *mut GTypeInstance,
                    gtk_stack_get_type(),
                ) as *mut std::ffi::c_void
                    as *mut GtkStack),
            );
            return 1 as std::ffi::c_int;
        }
        254 => {
            let mut widget = gtk_stack_get_visible_child(g_type_check_instance_cast(
                (*w).widget as *mut GTypeInstance,
                gtk_stack_get_type(),
            ) as *mut std::ffi::c_void
                as *mut GtkStack);
            if widget.is_null() {
                return 0 as std::ffi::c_int;
            }
            let mut child = g_object_get_data(
                g_type_check_instance_cast(
                    widget as *mut GTypeInstance,
                    ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
                ) as *mut std::ffi::c_void as *mut GObject,
                GOBJECT_LUAKIT_WIDGET_DATA_KEY.as_ptr(),
            ) as *mut widget_t;
            luaH_object_push(L, (*child).ref_0);
            return 1 as std::ffi::c_int;
        }
        _ => {}
    }
    return 0 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_stack_newindex(
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
        112 => {
            gtk_stack_set_homogeneous(
                g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_stack_get_type())
                    as *mut std::ffi::c_void as *mut GtkStack,
                luaH_checkboolean(L, 3 as std::ffi::c_int),
            );
        }
        254 => {
            let mut child = luaH_checkwidget(L, 3 as std::ffi::c_int);
            gtk_stack_set_visible_child(
                g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_stack_get_type())
                    as *mut std::ffi::c_void as *mut GtkStack,
                (*child).widget,
            );
        }
        _ => return 0 as std::ffi::c_int,
    }
    return luaH_object_property_signal(L, 1 as std::ffi::c_int, token);
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn widget_stack(
    mut UNUSED_L: *mut lua_State,
    mut w: *mut widget_t,
    mut UNUSED_token: luakit_token_t,
) -> *mut widget_t {
    (*w).index = Some(luaH_stack_index);
    (*w).newindex = Some(luaH_stack_newindex);
    (*w).widget = gtk_stack_new();
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
