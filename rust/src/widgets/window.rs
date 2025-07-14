use gdk_sys::*;
use glib_sys::*;
use gobject_sys::*;
use gtk_sys::*;
use mlua::ffi::*;

use crate::common::common;
use crate::common::luaclass::*;
use crate::common::luah::*;
use crate::common::luaobject::*;
use crate::common::tokenize::*;
use crate::globalconf::globalconf;
use crate::gtypes::*;
use crate::widgets::common::*;
use crate::widgets::*;

pub struct window_data_t {
    pub widget: *mut widget_t,
    pub win: *mut GtkWindow,
    pub state: GdkWindowState,
    pub id: guint,
}
static mut window_id_next: std::ffi::c_int = 0 as std::ffi::c_int;

unsafe extern "C" fn luaH_checkwindow(mut L: *mut lua_State, mut udx: gint) -> *mut widget_t {
    let mut w = luaH_checkwidget(L, udx);
    if (*(*w).info).tok as std::ffi::c_uint != L_TK_WINDOW as std::ffi::c_int as std::ffi::c_uint {
        luaL_argerror(
            L,
            udx,
            b"expected window widget\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    return w;
}

unsafe extern "C" fn destroy_win_cb(mut UNUSED_win: *mut GtkWidget, mut w: *mut widget_t) {
    g_ptr_array_remove(globalconf.windows, w as gpointer);
}

unsafe extern "C" fn can_close_cb(
    mut UNUSED_win: *mut GtkWidget,
    mut UNUSED_event: *mut GdkEvent,
    mut w: *mut widget_t,
) -> gint {
    let mut L = common.L;
    luaH_object_push(L, (*w).ref_0);
    let mut ret = luaH_object_emit_signal(
        L,
        -(1 as std::ffi::c_int),
        b"can-close\0" as *const u8 as *const std::ffi::c_char,
        0 as std::ffi::c_int,
        1 as std::ffi::c_int,
    );
    let mut keep_open =
        (ret != 0 && lua_toboolean(L, -(1 as std::ffi::c_int)) == 0) as std::ffi::c_int;
    lua_settop(L, -(ret + 1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    return keep_open;
}

unsafe extern "C-unwind" fn luaH_window_set_dark_mode(mut L: *mut lua_State) -> gint {
    let mut d = (*luaH_checkwindow(L, 1 as std::ffi::c_int)).data as *mut window_data_t;
    let mut dark_mode = lua_toboolean(L, 2 as std::ffi::c_int);
    g_object_set(
        gtk_widget_get_settings(g_type_check_instance_cast(
            (*d).win as *mut GTypeInstance,
            gtk_widget_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkWidget) as *mut GObject,
        b"gtk-application-prefer-dark-theme\0" as *const u8 as *const std::ffi::c_char,
        dark_mode,
        // std::ptr::null_mut(),
    );
    return 0 as std::ffi::c_int;
}

unsafe extern "C-unwind" fn luaH_window_set_default_size(mut L: *mut lua_State) -> gint {
    let mut d = (*luaH_checkwindow(L, 1 as std::ffi::c_int)).data as *mut window_data_t;
    let mut width = luaL_checknumber(L, 2 as std::ffi::c_int) as gint;
    let mut height = luaL_checknumber(L, 3 as std::ffi::c_int) as gint;
    gtk_window_set_default_size((*d).win, width, height);
    return 0 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_window_index(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
    mut token: luakit_token_t,
) -> gint {
    let mut d = (*w).data as *mut window_data_t;
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
        207 => {
            lua_pushcclosure(L, luaH_window_set_default_size, 0 as std::ffi::c_int);
            return 1 as std::ffi::c_int;
        }
        240 => {
            lua_pushstring(L, gtk_window_get_title((*d).win));
            return 1 as std::ffi::c_int;
        }
        43 => {
            lua_pushboolean(L, gtk_window_get_decorated((*d).win));
            return 1 as std::ffi::c_int;
        }
        245 => {
            lua_pushboolean(L, gtk_window_get_urgency_hint((*d).win));
            return 1 as std::ffi::c_int;
        }
        102 => {
            lua_pushboolean(
                L,
                ((*d).state as std::ffi::c_uint
                    & GDK_WINDOW_STATE_FULLSCREEN as std::ffi::c_int as std::ffi::c_uint)
                    as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        144 => {
            lua_pushboolean(
                L,
                ((*d).state as std::ffi::c_uint
                    & GDK_WINDOW_STATE_MAXIMIZED as std::ffi::c_int as std::ffi::c_uint)
                    as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        206 => {
            lua_pushcclosure(L, luaH_window_set_dark_mode, 0 as std::ffi::c_int);
            return 1 as std::ffi::c_int;
        }
        117 => {
            lua_pushnumber(L, (*d).id as lua_Number);
            return 1 as std::ffi::c_int;
        }
        186 => {
            lua_pushlightuserdata(
                L,
                g_type_check_instance_cast(
                    gdk_screen_get_root_window(gtk_widget_get_screen(g_type_check_instance_cast(
                        (*d).win as *mut GTypeInstance,
                        gtk_widget_get_type(),
                    )
                        as *mut std::ffi::c_void
                        as *mut GtkWidget)) as *mut GTypeInstance,
                    gdk_window_get_type(),
                ) as *mut std::ffi::c_void as *mut GdkWindow
                    as *mut std::ffi::c_void,
            );
            return 1 as std::ffi::c_int;
        }
        265 => {
            lua_pushlightuserdata(
                L,
                g_type_check_instance_cast(
                    gtk_widget_get_window(g_type_check_instance_cast(
                        (*d).win as *mut GTypeInstance,
                        gtk_widget_get_type(),
                    ) as *mut std::ffi::c_void
                        as *mut GtkWidget) as *mut GTypeInstance,
                    gdk_window_get_type(),
                ) as *mut std::ffi::c_void as *mut GdkWindow
                    as *mut std::ffi::c_void,
            );
            return 1 as std::ffi::c_int;
        }
        190 => {
            lua_pushlightuserdata(L, gtk_window_get_screen((*d).win) as *mut std::ffi::c_void);
            return 1 as std::ffi::c_int;
        }
        _ => {}
    }
    return 0 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_window_newindex(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
    mut token: luakit_token_t,
) -> gint {
    let mut d = (*w).data as *mut window_data_t;
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
        43 => {
            gtk_window_set_decorated((*d).win, luaH_checkboolean(L, 3 as std::ffi::c_int));
        }
        245 => {
            gtk_window_set_urgency_hint((*d).win, luaH_checkboolean(L, 3 as std::ffi::c_int));
        }
        240 => {
            gtk_window_set_title(
                (*d).win,
                luaL_checklstring(L, 3 as std::ffi::c_int, std::ptr::null_mut()),
            );
        }
        116 => {
            gtk_window_set_icon_from_file(
                (*d).win,
                luaL_checklstring(L, 3 as std::ffi::c_int, std::ptr::null_mut()),
                std::ptr::null_mut(),
            );
        }
        190 => {
            if !(lua_type(L, 3 as std::ffi::c_int) == LUA_TLIGHTUSERDATA) {
                luaL_argerror(
                    L,
                    3 as std::ffi::c_int,
                    b"expected GdkScreen lightuserdata\0" as *const u8 as *const std::ffi::c_char,
                );
            }
            gtk_window_set_screen(
                (*d).win,
                lua_touserdata(L, 3 as std::ffi::c_int) as *mut GdkScreen,
            );
            gtk_window_present((*d).win);
        }
        102 => {
            if luaH_checkboolean(L, 3 as std::ffi::c_int) != 0 {
                gtk_window_fullscreen((*d).win);
            } else {
                gtk_window_unfullscreen((*d).win);
            }
            return 0 as std::ffi::c_int;
        }
        144 => {
            if luaH_checkboolean(L, 3 as std::ffi::c_int) != 0 {
                gtk_window_maximize((*d).win);
            } else {
                gtk_window_unmaximize((*d).win);
            }
            return 0 as std::ffi::c_int;
        }
        _ => return 0 as std::ffi::c_int,
    }
    return luaH_object_property_signal(L, 1 as std::ffi::c_int, token);
}

unsafe extern "C" fn window_state_cb(
    mut UNUSED_widget: *mut GtkWidget,
    mut ev: *mut GdkEventWindowState,
    mut w: *mut widget_t,
) -> gboolean {
    let mut d = (*w).data as *mut window_data_t;
    (*d).state = (*ev).new_window_state;
    let mut L = common.L;
    luaH_object_push(L, (*w).ref_0);
    if (*ev).changed_mask as std::ffi::c_uint
        & GDK_WINDOW_STATE_MAXIMIZED as std::ffi::c_int as std::ffi::c_uint
        != 0
    {
        luaH_object_property_signal(L, -(1 as std::ffi::c_int), L_TK_MAXIMIZED);
    }
    if (*ev).changed_mask as std::ffi::c_uint
        & GDK_WINDOW_STATE_FULLSCREEN as std::ffi::c_int as std::ffi::c_uint
        != 0
    {
        luaH_object_property_signal(L, -(1 as std::ffi::c_int), L_TK_FULLSCREEN);
    }
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    return GFALSE;
}

unsafe extern "C" fn window_destructor(mut w: *mut widget_t) {
    g_slice_free1(::core::mem::size_of::<window_data_t>(), (*w).data);
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn widget_window(
    mut UNUSED_L: *mut lua_State,
    mut w: *mut widget_t,
    mut UNUSED_token: luakit_token_t,
) -> *mut widget_t {
    (*w).index = Some(luaH_window_index);
    (*w).newindex = Some(luaH_window_newindex);
    (*w).destructor = Some(window_destructor as unsafe extern "C" fn(*mut widget_t) -> ());
    let mut d = g_slice_alloc0(::core::mem::size_of::<window_data_t>()) as *mut window_data_t;
    (*d).widget = w;
    (*w).data = d as gpointer;
    (*w).widget = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    (*d).win = g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_window_get_type())
        as *mut std::ffi::c_void as *mut GtkWindow;
    gtk_window_set_default_size((*d).win, 800 as std::ffi::c_int, 600 as std::ffi::c_int);
    gtk_window_set_title(
        (*d).win,
        b"luakit\0" as *const u8 as *const std::ffi::c_char,
    );
    if !(globalconf.application).is_null() {
        gtk_window_set_application((*d).win, globalconf.application);
    }
    let mut hints = GdkGeometry {
        min_width: 0,
        min_height: 0,
        max_width: 0,
        max_height: 0,
        base_width: 0,
        base_height: 0,
        width_inc: 0,
        height_inc: 0,
        min_aspect: 0.,
        max_aspect: 0.,
        win_gravity: 0 as GdkGravity,
    };
    hints.min_width = 1 as std::ffi::c_int;
    hints.min_height = 1 as std::ffi::c_int;
    gtk_window_set_geometry_hints(
        (*d).win,
        std::ptr::null_mut(),
        &mut hints,
        GDK_HINT_MIN_SIZE,
    );
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
            destroy_win_cb as unsafe extern "C" fn(*mut GtkWidget, *mut widget_t) -> (),
        )),
        w,
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
        b"signal::delete-event\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GtkWidget, *mut GdkEvent, *mut widget_t) -> gint>,
            GCallback,
        >(Some(
            can_close_cb
                as unsafe extern "C" fn(*mut GtkWidget, *mut GdkEvent, *mut widget_t) -> gint,
        )),
        w,
        b"signal::key-press-event\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(*mut GtkWidget, *mut GdkEventKey, *mut widget_t) -> gboolean,
            >,
            GCallback,
        >(Some(
            key_press_cb
                as unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut GdkEventKey,
                    *mut widget_t,
                ) -> gboolean,
        )),
        w,
        b"signal::remove\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GtkContainer, *mut GtkWidget, *mut widget_t) -> ()>,
            GCallback,
        >(Some(
            remove_cb
                as unsafe extern "C" fn(*mut GtkContainer, *mut GtkWidget, *mut widget_t) -> (),
        )),
        w,
        b"signal::window-state-event\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut GdkEventWindowState,
                    *mut widget_t,
                ) -> gboolean,
            >,
            GCallback,
        >(Some(
            window_state_cb
                as unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut GdkEventWindowState,
                    *mut widget_t,
                ) -> gboolean,
        )),
        w,
        // std::ptr::null_mut(),
    );
    window_id_next += 1;
    (*d).id = window_id_next as guint;
    g_ptr_array_add(globalconf.windows, w as gpointer);
    return w;
}
