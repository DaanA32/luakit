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

unsafe extern "C" fn gtk_policy_from_string(
    mut str: *const gchar,
    mut out: *mut GtkPolicyType,
) -> gint {
    if strcmp(str, b"always\0" as *const u8 as *const std::ffi::c_char) == 0 {
        *out = GTK_POLICY_ALWAYS;
    } else if strcmp(str, b"auto\0" as *const u8 as *const std::ffi::c_char) == 0 {
        *out = GTK_POLICY_AUTOMATIC;
    } else if strcmp(str, b"never\0" as *const u8 as *const std::ffi::c_char) == 0 {
        *out = GTK_POLICY_NEVER;
    } else if strcmp(
        str,
        b"unsafe external\0" as *const u8 as *const std::ffi::c_char,
    ) == 0
    {
        *out = GTK_POLICY_EXTERNAL;
    } else {
        return 1 as std::ffi::c_int;
    }
    return 0 as std::ffi::c_int;
}

unsafe extern "C" fn string_from_gtk_policy(mut policy: GtkPolicyType) -> *const gchar {
    match policy as std::ffi::c_uint {
        0 => return b"always\0" as *const u8 as *const std::ffi::c_char,
        1 => return b"auto\0" as *const u8 as *const std::ffi::c_char,
        2 => return b"never\0" as *const u8 as *const std::ffi::c_char,
        3 => return b"unsafe external\0" as *const u8 as *const std::ffi::c_char,
        _ => return std::ptr::null_mut(),
    };
}
#[unsafe(no_mangle)]

pub unsafe extern "C-unwind" fn luaH_widget_get_scrollbars(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
) -> gint {
    let mut horz = GTK_POLICY_ALWAYS;
    let mut vert = GTK_POLICY_ALWAYS;
    gtk_scrolled_window_get_policy(
        g_type_check_instance_cast(
            (*w).widget as *mut GTypeInstance,
            gtk_scrolled_window_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkScrolledWindow,
        &mut horz,
        &mut vert,
    );
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_pushlstring(
        L,
        b"h\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 2]>())
            .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
            .wrapping_sub(1),
    );
    lua_pushstring(L, string_from_gtk_policy(horz));
    lua_rawset(L, -(3 as std::ffi::c_int));
    lua_pushlstring(
        L,
        b"v\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 2]>())
            .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
            .wrapping_sub(1),
    );
    lua_pushstring(L, string_from_gtk_policy(vert));
    lua_rawset(L, -(3 as std::ffi::c_int));
    return 1 as std::ffi::c_int;
}
#[unsafe(no_mangle)]

pub unsafe extern "C-unwind" fn luaH_widget_set_scrollbars(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
) -> gint {
    if !(lua_type(L, 3 as std::ffi::c_int) == LUA_TTABLE) {
        luaL_typerror(
            L,
            3 as std::ffi::c_int,
            b"table\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    let mut horz = GTK_POLICY_ALWAYS;
    let mut vert = GTK_POLICY_ALWAYS;
    gtk_scrolled_window_get_policy(
        g_type_check_instance_cast(
            (*w).widget as *mut GTypeInstance,
            gtk_scrolled_window_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkScrolledWindow,
        &mut horz,
        &mut vert,
    );
    let mut top = lua_gettop(L);
    if luaH_rawfield(
        L,
        3 as std::ffi::c_int,
        b"h\0" as *const u8 as *const std::ffi::c_char,
    ) != 0
    {
        if gtk_policy_from_string(
            lua_tolstring(L, -(1 as std::ffi::c_int), std::ptr::null_mut()),
            &mut horz,
        ) != 0
        {
            luaL_error(
                L,
                b"Bad horizontal scrollbar policy\0" as *const u8 as *const std::ffi::c_char,
            );
        }
    }
    if luaH_rawfield(
        L,
        3 as std::ffi::c_int,
        b"v\0" as *const u8 as *const std::ffi::c_char,
    ) != 0
    {
        if gtk_policy_from_string(
            lua_tolstring(L, -(1 as std::ffi::c_int), std::ptr::null_mut()),
            &mut vert,
        ) != 0
        {
            luaL_error(
                L,
                b"Bad vertical scrollbar policy\0" as *const u8 as *const std::ffi::c_char,
            );
        }
    }
    lua_settop(L, top);
    gtk_scrolled_window_set_policy(
        g_type_check_instance_cast(
            (*w).widget as *mut GTypeInstance,
            gtk_scrolled_window_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkScrolledWindow,
        horz,
        vert,
    );
    return 1 as std::ffi::c_int;
}
#[unsafe(no_mangle)]

pub unsafe extern "C-unwind" fn luaH_scrolled_get_scroll(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
) -> gint {
    let mut horz = gtk_scrolled_window_get_hadjustment(g_type_check_instance_cast(
        (*w).widget as *mut GTypeInstance,
        gtk_scrolled_window_get_type(),
    ) as *mut std::ffi::c_void
        as *mut GtkScrolledWindow);
    let mut vert = gtk_scrolled_window_get_vadjustment(g_type_check_instance_cast(
        (*w).widget as *mut GTypeInstance,
        gtk_scrolled_window_get_type(),
    ) as *mut std::ffi::c_void
        as *mut GtkScrolledWindow);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_pushlstring(
        L,
        b"x\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 2]>())
            .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
            .wrapping_sub(1),
    );
    lua_pushnumber(L, gtk_adjustment_get_value(horz));
    lua_rawset(L, -(3 as std::ffi::c_int));
    lua_pushlstring(
        L,
        b"y\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 2]>())
            .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
            .wrapping_sub(1),
    );
    lua_pushnumber(L, gtk_adjustment_get_value(vert));
    lua_rawset(L, -(3 as std::ffi::c_int));
    lua_pushlstring(
        L,
        b"xmax\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 5]>())
            .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
            .wrapping_sub(1),
    );
    lua_pushnumber(L, gtk_adjustment_get_upper(horz));
    lua_rawset(L, -(3 as std::ffi::c_int));
    lua_pushlstring(
        L,
        b"ymax\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 5]>())
            .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
            .wrapping_sub(1),
    );
    lua_pushnumber(L, gtk_adjustment_get_upper(vert));
    lua_rawset(L, -(3));
    return 1;
}
#[unsafe(no_mangle)]

pub unsafe extern "C-unwind" fn luaH_scrolled_set_scroll(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
) -> gint {
    if !(lua_type(L, 3 as std::ffi::c_int) == LUA_TTABLE) {
        luaL_typerror(
            L,
            3 as std::ffi::c_int,
            b"table\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    let mut horz = gtk_scrolled_window_get_hadjustment(g_type_check_instance_cast(
        (*w).widget as *mut GTypeInstance,
        gtk_scrolled_window_get_type(),
    ) as *mut std::ffi::c_void
        as *mut GtkScrolledWindow);
    let mut vert = gtk_scrolled_window_get_vadjustment(g_type_check_instance_cast(
        (*w).widget as *mut GTypeInstance,
        gtk_scrolled_window_get_type(),
    ) as *mut std::ffi::c_void
        as *mut GtkScrolledWindow);
    let mut top = lua_gettop(L);
    if luaH_rawfield(
        L,
        3 as std::ffi::c_int,
        b"x\0" as *const u8 as *const std::ffi::c_char,
    ) != 0
    {
        gtk_adjustment_set_value(horz, lua_tonumber(L, -(1 as std::ffi::c_int)));
    }
    if luaH_rawfield(
        L,
        3 as std::ffi::c_int,
        b"y\0" as *const u8 as *const std::ffi::c_char,
    ) != 0
    {
        gtk_adjustment_set_value(vert, lua_tonumber(L, -(1 as std::ffi::c_int)));
    }
    lua_settop(L, top);
    return 0 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_scrolled_index(
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
        192 => return luaH_widget_get_scrollbars(L, w),
        191 => return luaH_scrolled_get_scroll(L, w),
        _ => {}
    }
    return 0 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_scrolled_newindex(
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
        192 => {
            luaH_widget_set_scrollbars(L, w);
        }
        191 => {
            luaH_scrolled_set_scroll(L, w);
        }
        _ => {}
    }
    return luaH_object_property_signal(L, 1 as std::ffi::c_int, token);
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn widget_scrolled(
    mut UNUSED_L: *mut lua_State,
    mut w: *mut widget_t,
    mut UNUSED_token: luakit_token_t,
) -> *mut widget_t {
    (*w).index = Some(
        luaH_scrolled_index
            as unsafe extern "C" fn(*mut lua_State, *mut widget_t, luakit_token_t) -> gint,
    );
    (*w).newindex = Some(
        luaH_scrolled_newindex
            as unsafe extern "C" fn(*mut lua_State, *mut widget_t, luakit_token_t) -> gint,
    );
    (*w).widget = gtk_scrolled_window_new(std::ptr::null_mut(), std::ptr::null_mut());
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
