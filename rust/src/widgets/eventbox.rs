use gdk_sys::*;
use glib_sys::*;
use gobject_sys::*;
use gtk_sys::*;
use mlua_sys::*;

use crate::common::luaclass::*;
use crate::common::luah::*;
use crate::common::tokenize::*;
use crate::gtypes::*;
use crate::widgets::common::*;
use crate::widgets::*;

unsafe extern "C" fn luaH_eventbox_index(
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
            lua_pushcclosure(
                L,
                Some(luaH_widget_show as unsafe extern "C" fn(*mut lua_State) -> gint),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        110 => {
            lua_pushcclosure(
                L,
                Some(luaH_widget_hide as unsafe extern "C" fn(*mut lua_State) -> gint),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        99 => {
            lua_pushcclosure(
                L,
                Some(luaH_widget_focus as unsafe extern "C" fn(*mut lua_State) -> gint),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        50 => {
            lua_pushcclosure(
                L,
                Some(luaH_widget_destroy as unsafe extern "C" fn(*mut lua_State) -> gint),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        183 => {
            lua_pushcclosure(
                L,
                Some(luaH_widget_replace as unsafe extern "C" fn(*mut lua_State) -> gint),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        203 => {
            lua_pushcclosure(
                L,
                Some(luaH_widget_send_key as unsafe extern "C" fn(*mut lua_State) -> gint),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        23 => return luaH_widget_get_child(L, w),
        180 => {
            lua_pushcclosure(
                L,
                Some(luaH_widget_remove as unsafe extern "C" fn(*mut lua_State) -> gint),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        13 => {
            lua_pushstring(
                L,
                g_object_get_data(
                    g_type_check_instance_cast(
                        (*w).widget as *mut GTypeInstance,
                        ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
                    ) as *mut std::ffi::c_void as *mut GObject,
                    b"bg\0" as *const u8 as *const std::ffi::c_char,
                ) as *const std::ffi::c_char,
            );
            return 1 as std::ffi::c_int;
        }
        _ => {}
    }
    return 0 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_eventbox_newindex(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
    mut token: luakit_token_t,
) -> gint {
    let mut len: size_t = 0;
    let mut tmp = 0 as *const gchar;
    let mut c = _GdkRGBA {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
    };
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
        13 => {
            tmp = luaL_checklstring(L, 3 as std::ffi::c_int, &mut len);
            if gdk_rgba_parse(&mut c, tmp) == 0 {
                luaL_argerror(
                    L,
                    3 as std::ffi::c_int,
                    b"unable to parse colour\0" as *const u8 as *const std::ffi::c_char,
                );
            }
            widget_set_css_properties(
                w,
                b"background-color\0" as *const u8 as *const std::ffi::c_char,
                tmp,
                NULL_0 as *mut std::ffi::c_void,
            );
            g_object_set_data_full(
                g_type_check_instance_cast(
                    (*w).widget as *mut GTypeInstance,
                    ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
                ) as *mut std::ffi::c_void as *mut GObject,
                b"bg\0" as *const u8 as *const std::ffi::c_char,
                g_strdup_inline(tmp) as gpointer,
                Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
            );
        }
        _ => return 0 as std::ffi::c_int,
    }
    return luaH_object_property_signal(L, 1 as std::ffi::c_int, token);
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn widget_eventbox(
    mut UNUSED_L: *mut lua_State,
    mut w: *mut widget_t,
    mut UNUSED_token: luakit_token_t,
) -> *mut widget_t {
    (*w).index = Some(
        luaH_eventbox_index
            as unsafe extern "C" fn(*mut lua_State, *mut widget_t, luakit_token_t) -> gint,
    );
    (*w).newindex = Some(
        luaH_eventbox_newindex
            as unsafe extern "C" fn(*mut lua_State, *mut widget_t, luakit_token_t) -> gint,
    );
    (*w).widget = gtk_event_box_new();
    gtk_widget_show((*w).widget);
    g_object_connect(
        g_type_check_instance_cast(
            (*w).widget as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject as gpointer,
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
        b"signal::add\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GtkContainer, *mut GtkWidget, *mut widget_t) -> ()>,
            GCallback,
        >(Some(
            add_cb as unsafe extern "C" fn(*mut GtkContainer, *mut GtkWidget, *mut widget_t) -> (),
        )),
        w,
        b"signal::button-press-event\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut GdkEventButton,
                    *mut widget_t,
                ) -> gboolean,
            >,
            GCallback,
        >(Some(
            button_cb
                as unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut GdkEventButton,
                    *mut widget_t,
                ) -> gboolean,
        )),
        w,
        b"signal::button-release-event\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut GdkEventButton,
                    *mut widget_t,
                ) -> gboolean,
            >,
            GCallback,
        >(Some(
            button_cb
                as unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut GdkEventButton,
                    *mut widget_t,
                ) -> gboolean,
        )),
        w,
        b"signal::scroll-event\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut GdkEventScroll,
                    *mut widget_t,
                ) -> gboolean,
            >,
            GCallback,
        >(Some(
            scroll_cb
                as unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut GdkEventScroll,
                    *mut widget_t,
                ) -> gboolean,
        )),
        w,
        b"signal::enter-notify-event\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut GdkEventCrossing,
                    *mut widget_t,
                ) -> gboolean,
            >,
            GCallback,
        >(Some(
            mouse_cb
                as unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut GdkEventCrossing,
                    *mut widget_t,
                ) -> gboolean,
        )),
        w,
        b"signal::leave-notify-event\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut GdkEventCrossing,
                    *mut widget_t,
                ) -> gboolean,
            >,
            GCallback,
        >(Some(
            mouse_cb
                as unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut GdkEventCrossing,
                    *mut widget_t,
                ) -> gboolean,
        )),
        w,
        NULL_0 as *mut std::ffi::c_void,
    );
    return w;
}
