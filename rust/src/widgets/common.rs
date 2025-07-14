use std::ffi::CStr;

use crate::gtypes::*;
use crate::luah::luaH_keystr_push;
use crate::luah::luaH_modifier_table_push;
use gdk_sys::*;
use glib_sys::*;
use gobject_sys::*;
use gtk_sys::*;
use libc::*;
use mlua_sys::*;

use crate::clib::widget::*;
use crate::common::common;
use crate::common::luaclass::*;
use crate::common::luah::*;
use crate::common::luaobject::*;
use crate::common::tokenize::*;
use crate::gtypes::*;
use crate::log::*;
use crate::widgets::*;

pub struct widget_info_t {
    pub tok: luakit_token_t,
    pub name: *const gchar,
    pub wc: Option<widget_constructor_t>,
}

pub type widget_constructor_t =
    unsafe extern "C" fn(*mut lua_State, *mut widget_t, luakit_token_t) -> *mut widget_t;

pub const GOBJECT_LUAKIT_WIDGET_DATA_KEY: [std::ffi::c_char; 19] = unsafe {
    *::core::mem::transmute::<&[u8; 19], &[std::ffi::c_char; 19]>(b"luakit_widget_data\0")
};
#[inline]
pub unsafe extern "C" fn luaH_checkwidgetornil(
    mut L: *mut lua_State,
    mut udx: gint,
) -> *mut widget_t {
    if lua_type(L, udx) == LUA_TNIL {
        return std::ptr::null_mut();
    }
    return luaH_checkwidget(L, udx);
}

#[unsafe(no_mangle)]

pub unsafe extern "C" fn key_press_cb(
    mut UNUSED_win: *mut GtkWidget,
    mut ev: *mut GdkEventKey,
    mut w: *mut widget_t,
) -> gboolean {
    let mut L = common.L;
    luaH_object_push(L, (*w).ref_0);
    luaH_modifier_table_push(L, (*ev).state);
    luaH_keystr_push(L, (*ev).keyval);
    lua_pushboolean(L, (*ev).send_event as std::ffi::c_int);
    let mut ret = luaH_object_emit_signal(
        L,
        -(4 as std::ffi::c_int),
        b"key-press\0" as *const u8 as *const std::ffi::c_char,
        3 as std::ffi::c_int,
        1 as std::ffi::c_int,
    );
    let mut catch = if ret != 0 && lua_toboolean(L, -(1 as std::ffi::c_int)) != 0 {
        GTRUE
    } else {
        GFALSE
    };
    lua_settop(L, -(ret + 1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    return catch;
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn button_cb(
    mut UNUSED_win: *mut GtkWidget,
    mut ev: *mut GdkEventButton,
    mut w: *mut widget_t,
) -> gboolean {
    let mut ret: gint = 0;
    let mut L = common.L;
    luaH_object_push(L, (*w).ref_0);
    luaH_modifier_table_push(L, (*ev).state);
    lua_pushinteger(L, (*ev).button as lua_Integer);
    match (*ev).type_ as std::ffi::c_int {
        5 => {
            ret = luaH_object_emit_signal(
                L,
                -(3 as std::ffi::c_int),
                b"button-double-click\0" as *const u8 as *const std::ffi::c_char,
                2 as std::ffi::c_int,
                1 as std::ffi::c_int,
            );
        }
        7 => {
            ret = luaH_object_emit_signal(
                L,
                -(3 as std::ffi::c_int),
                b"button-release\0" as *const u8 as *const std::ffi::c_char,
                2 as std::ffi::c_int,
                1 as std::ffi::c_int,
            );
        }
        _ => {
            ret = luaH_object_emit_signal(
                L,
                -(3 as std::ffi::c_int),
                b"button-press\0" as *const u8 as *const std::ffi::c_char,
                2 as std::ffi::c_int,
                1 as std::ffi::c_int,
            );
        }
    }
    let mut catch = if ret != 0 && lua_toboolean(L, -(1 as std::ffi::c_int)) != 0 {
        GTRUE
    } else {
        GFALSE
    };
    lua_settop(L, -(ret + 1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    return catch;
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn scroll_cb(
    mut UNUSED_wid: *mut GtkWidget,
    mut ev: *mut GdkEventScroll,
    mut w: *mut widget_t,
) -> gboolean {
    let mut dx: std::ffi::c_double = 0.;
    let mut dy: std::ffi::c_double = 0.;
    match (*ev).direction as std::ffi::c_uint {
        0 => {
            dx = 0 as std::ffi::c_int as std::ffi::c_double;
            dy = -(1 as std::ffi::c_int) as std::ffi::c_double;
        }
        1 => {
            dx = 0 as std::ffi::c_int as std::ffi::c_double;
            dy = 1 as std::ffi::c_int as std::ffi::c_double;
        }
        2 => {
            dx = -(1 as std::ffi::c_int) as std::ffi::c_double;
            dy = 0 as std::ffi::c_int as std::ffi::c_double;
        }
        3 => {
            dx = 1 as std::ffi::c_int as std::ffi::c_double;
            dy = 0 as std::ffi::c_int as std::ffi::c_double;
        }
        4 => {
            gdk_event_get_scroll_deltas(ev as *mut GdkEvent, &mut dx, &mut dy);
        }
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN as *const std::ffi::c_char,
                b"widgets/common.c\0" as *const u8 as *const std::ffi::c_char,
                79 as std::ffi::c_int,
                (*::core::mem::transmute::<&[u8; 10], &[std::ffi::c_char; 10]>(b"scroll_cb\0"))
                    .as_ptr(),
                std::ptr::null(),
            );
        }
    }
    let mut L = common.L;
    luaH_object_push(L, (*w).ref_0);
    luaH_modifier_table_push(L, (*ev).state);
    lua_pushnumber(L, dx);
    lua_pushnumber(L, dy);
    let mut ret = luaH_object_emit_signal(
        L,
        -(4 as std::ffi::c_int),
        b"scroll\0" as *const u8 as *const std::ffi::c_char,
        3 as std::ffi::c_int,
        1 as std::ffi::c_int,
    );
    lua_settop(L, -(ret + 1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    return ret;
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn mouse_cb(
    mut UNUSED_win: *mut GtkWidget,
    mut ev: *mut GdkEventCrossing,
    mut w: *mut widget_t,
) -> gboolean {
    let mut L = common.L;
    luaH_object_push(L, (*w).ref_0);
    luaH_modifier_table_push(L, (*ev).state);
    let mut type_0 = (*ev).type_;
    if type_0 as std::ffi::c_int == GDK_ENTER_NOTIFY as std::ffi::c_int
        || type_0 as std::ffi::c_int == GDK_LEAVE_NOTIFY as std::ffi::c_int
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN as *const std::ffi::c_char,
            b"widgets/common.c\0" as *const u8 as *const std::ffi::c_char,
            101 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 9], &[std::ffi::c_char; 9]>(b"mouse_cb\0")).as_ptr(),
            b"type == GDK_ENTER_NOTIFY || type == GDK_LEAVE_NOTIFY\0" as *const u8
                as *const std::ffi::c_char,
        );
    }
    let mut ret = luaH_object_emit_signal(
        L,
        -(2 as std::ffi::c_int),
        if type_0 as std::ffi::c_int == GDK_ENTER_NOTIFY as std::ffi::c_int {
            b"mouse-enter\0" as *const u8 as *const std::ffi::c_char
        } else {
            b"mouse-leave\0" as *const u8 as *const std::ffi::c_char
        },
        1 as std::ffi::c_int,
        1 as std::ffi::c_int,
    );
    let mut catch = if ret != 0 && lua_toboolean(L, -(1 as std::ffi::c_int)) != 0 {
        GTRUE
    } else {
        GFALSE
    };
    lua_settop(L, -(ret + 1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    return catch;
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn focus_cb(
    mut UNUSED_win: *mut GtkWidget,
    mut ev: *mut GdkEventFocus,
    mut w: *mut widget_t,
) -> gboolean {
    let mut L = common.L;
    luaH_object_push(L, (*w).ref_0);
    let mut ret: gint = 0;
    if (*ev).in_ != 0 {
        ret = luaH_object_emit_signal(
            L,
            -(1 as std::ffi::c_int),
            b"focus\0" as *const u8 as *const std::ffi::c_char,
            0 as std::ffi::c_int,
            1 as std::ffi::c_int,
        );
    } else {
        ret = luaH_object_emit_signal(
            L,
            -(1 as std::ffi::c_int),
            b"unfocus\0" as *const u8 as *const std::ffi::c_char,
            0 as std::ffi::c_int,
            1 as std::ffi::c_int,
        );
    }
    if ret != 0 && lua_toboolean(L, -(1 as std::ffi::c_int)) != 0 {
        lua_settop(L, -(ret + 1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        return GTRUE;
    }
    lua_settop(L, -(ret + 1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    return GFALSE;
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn add_cb(
    mut UNUSED_c: *mut GtkContainer,
    mut widget: *mut GtkWidget,
    mut w: *mut widget_t,
) {
    let mut child = g_object_get_data(
        g_type_check_instance_cast(
            widget as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject,
        GOBJECT_LUAKIT_WIDGET_DATA_KEY.as_ptr(),
    ) as *mut widget_t;
    let mut L = common.L;
    luaH_object_push(L, (*w).ref_0);
    luaH_object_push(L, (*child).ref_0);
    luaH_object_emit_signal(
        L,
        -(2 as std::ffi::c_int),
        b"add\0" as *const u8 as *const std::ffi::c_char,
        1 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn resize_cb(
    mut UNUSED_win: *mut GtkWidget,
    mut rect: *mut GdkRectangle,
    mut w: *mut widget_t,
) {
    let mut width = (*rect).width;
    let mut height = (*rect).height;
    if width == (*w).prev_width && height == (*w).prev_height {
        return;
    }
    (*w).prev_width = width;
    (*w).prev_height = height;
    let mut L = common.L;
    luaH_object_push(L, (*w).ref_0);
    lua_pushinteger(L, width as lua_Integer);
    lua_pushinteger(L, height as lua_Integer);
    luaH_object_emit_signal(
        L,
        -(3 as std::ffi::c_int),
        b"resize\0" as *const u8 as *const std::ffi::c_char,
        2 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn remove_cb(
    mut UNUSED_c: *mut GtkContainer,
    mut widget: *mut GtkWidget,
    mut w: *mut widget_t,
) {
    let mut child = g_object_get_data(
        g_type_check_instance_cast(
            widget as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject,
        GOBJECT_LUAKIT_WIDGET_DATA_KEY.as_ptr(),
    ) as *mut widget_t;
    let mut L = common.L;
    luaH_object_push(L, (*w).ref_0);
    luaH_object_push(L, (*child).ref_0);
    luaH_object_emit_signal(
        L,
        -(2 as std::ffi::c_int),
        b"remove\0" as *const u8 as *const std::ffi::c_char,
        1 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn parent_set_cb(
    mut widget: *mut GtkWidget,
    mut UNUSED_p: *mut GtkWidget,
    mut w: *mut widget_t,
) {
    let mut L = common.L;
    let mut parent = std::ptr::null();
    let mut new = 0 as *mut GtkContainer;
    g_object_get(
        g_type_check_instance_cast(
            widget as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject,
        b"parent\0" as *const u8 as *const std::ffi::c_char,
        &mut new as *mut *mut GtkContainer,
        // std::ptr::null(),
    );
    luaH_object_push(L, (*w).ref_0);
    if !new.is_null() && {
        parent = g_object_get_data(
            g_type_check_instance_cast(
                new as *mut GTypeInstance,
                ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
            ) as *mut std::ffi::c_void as *mut GObject,
            GOBJECT_LUAKIT_WIDGET_DATA_KEY.as_ptr(),
        ) as *mut widget_t;
        !parent.is_null()
    } {
        luaH_object_push(L, (*parent).ref_0);
    } else {
        lua_pushnil(L);
    }
    luaH_object_emit_signal(
        L,
        -(2 as std::ffi::c_int),
        b"parent-set\0" as *const u8 as *const std::ffi::c_char,
        1 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn destroy_cb(mut UNUSED_win: *mut GtkWidget, mut w: *mut widget_t) {
    let mut L = common.L;
    luaH_object_push(L, (*w).ref_0);
    luaH_object_emit_signal(
        L,
        -(1 as std::ffi::c_int),
        b"destroy\0" as *const u8 as *const std::ffi::c_char,
        0 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    _log(
        LOG_LEVEL_debug,
        b"widgets/common.c\0" as *const u8 as *const std::ffi::c_char,
        &format!(
            "destroy {} ({})",
            w as usize,
            CStr::from_ptr((*(*w).info).name).to_string_lossy(),
        ),
    );
    if ((*w).destructor).is_some() {
        ((*w).destructor).expect("non-null function pointer")(w);
    }
    (*w).destructor = None;
    (*w).widget = std::ptr::null_mut();
    luaH_object_unref(L, (*w).ref_0);
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn true_cb() -> gboolean {
    return GTRUE;
}
#[unsafe(no_mangle)]

pub unsafe extern "C-unwind" fn luaH_widget_set_child(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
) -> gint {
    let mut child = luaH_checkwidgetornil(L, 3 as std::ffi::c_int);
    let mut widget = gtk_bin_get_child(g_type_check_instance_cast(
        (*w).widget as *mut GTypeInstance,
        gtk_bin_get_type(),
    ) as *mut std::ffi::c_void as *mut GtkBin);
    if !widget.is_null() {
        g_object_ref(g_type_check_instance_cast(
            widget as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject);
        gtk_container_remove(
            g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_container_get_type())
                as *mut std::ffi::c_void as *mut GtkContainer,
            g_type_check_instance_cast(widget as *mut GTypeInstance, gtk_widget_get_type())
                as *mut std::ffi::c_void as *mut GtkWidget,
        );
    }
    if !child.is_null() {
        gtk_container_add(
            g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_container_get_type())
                as *mut std::ffi::c_void as *mut GtkContainer,
            g_type_check_instance_cast((*child).widget as *mut GTypeInstance, gtk_widget_get_type())
                as *mut std::ffi::c_void as *mut GtkWidget,
        );
    }
    return 0 as std::ffi::c_int;
}
#[unsafe(no_mangle)]

pub unsafe extern "C-unwind" fn luaH_widget_get_child(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
) -> gint {
    let mut widget = gtk_bin_get_child(g_type_check_instance_cast(
        (*w).widget as *mut GTypeInstance,
        gtk_bin_get_type(),
    ) as *mut std::ffi::c_void as *mut GtkBin);
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
#[unsafe(no_mangle)]

pub unsafe extern "C-unwind" fn luaH_widget_remove(mut L: *mut lua_State) -> gint {
    let mut w = luaH_checkwidget(L, 1 as std::ffi::c_int);
    let mut child = luaH_checkwidget(L, 2 as std::ffi::c_int);
    g_object_ref(g_type_check_instance_cast(
        (*child).widget as *mut GTypeInstance,
        ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
    ) as *mut std::ffi::c_void as *mut GObject);
    gtk_container_remove(
        g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_container_get_type())
            as *mut std::ffi::c_void as *mut GtkContainer,
        g_type_check_instance_cast((*child).widget as *mut GTypeInstance, gtk_widget_get_type())
            as *mut std::ffi::c_void as *mut GtkWidget,
    );
    return 0 as std::ffi::c_int;
}
#[unsafe(no_mangle)]

pub unsafe extern "C-unwind" fn luaH_widget_get_children(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
) -> gint {
    if ({
        let mut __inst = (*w).widget as *mut GTypeInstance;
        let mut __t = gtk_container_get_type();
        let mut __r: gboolean = 0;
        if __inst.is_null() {
            __r = GFALSE;
        } else if !((*__inst).g_class).is_null() && (*(*__inst).g_class).g_type == __t {
            __r = GTRUE;
        } else {
            __r = g_type_check_instance_is_a(__inst, __t);
        }
        __r
    }) == 0
    {
        return 0 as std::ffi::c_int;
    }
    let mut children = gtk_container_get_children(g_type_check_instance_cast(
        (*w).widget as *mut GTypeInstance,
        gtk_container_get_type(),
    ) as *mut std::ffi::c_void
        as *mut GtkContainer);
    let mut iter = children;
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    let mut i = 1;
    while !iter.is_null() {
        luaH_object_push(
            L,
            (*(g_object_get_data(
                g_type_check_instance_cast(
                    (*iter).data as *mut GTypeInstance,
                    ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
                ) as *mut std::ffi::c_void as *mut GObject,
                GOBJECT_LUAKIT_WIDGET_DATA_KEY.as_ptr(),
            ) as *mut widget_t))
                .ref_0,
        );
        let fresh0 = i;
        i = i + 1;
        lua_rawseti(L, -(2 as std::ffi::c_int), fresh0);
        iter = (*iter).next;
    }
    g_list_free(children);
    return 1 as std::ffi::c_int;
}
#[unsafe(no_mangle)]

pub unsafe extern "C-unwind" fn luaH_widget_replace(mut L: *mut lua_State) -> gint {
    let mut och = luaH_checkwidget(L, 1 as std::ffi::c_int);
    let mut nch = luaH_checkwidget(L, 2 as std::ffi::c_int);
    let mut parent = gtk_widget_get_parent(g_type_check_instance_cast(
        (*och).widget as *mut GTypeInstance,
        gtk_widget_get_type(),
    ) as *mut std::ffi::c_void as *mut GtkWidget);
    if parent.is_null() {
        return 0 as std::ffi::c_int;
    }
    let mut num_props: guint = 0;
    // let mut props = gtk_container_class_list_child_properties(
    let mut props: *mut *mut GList = todo!(
        "{:?}, {:?}",
        (*(parent as *mut GTypeInstance)).g_class as *mut GObjectClass,
        &mut num_props,
    );
    let mut values =
        g_malloc0_n(num_props as size_t, ::core::mem::size_of::<GValue>()) as *mut GValue;
    let mut i = 0 as std::ffi::c_int as guint;
    while i < num_props {
        g_value_init(
            &mut *values.offset(i as isize),
            (*(g_type_check_instance_cast(
                *props.offset(i as isize) as *mut GTypeInstance,
                ((19 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
            ) as *mut std::ffi::c_void as *mut GParamSpec))
                .value_type,
        );
        gtk_container_child_get_property(
            g_type_check_instance_cast(parent as *mut GTypeInstance, gtk_container_get_type())
                as *mut std::ffi::c_void as *mut GtkContainer,
            g_type_check_instance_cast((*och).widget as *mut GTypeInstance, gtk_widget_get_type())
                as *mut std::ffi::c_void as *mut GtkWidget,
            (**props.offset(i as isize)).data as *const i8,
            &mut *values.offset(i as isize),
        );
        i = i.wrapping_add(1);
        i;
    }
    g_object_ref(g_type_check_instance_cast(
        (*och).widget as *mut GTypeInstance,
        ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
    ) as *mut std::ffi::c_void as *mut GObject);
    gtk_container_remove(
        g_type_check_instance_cast(parent as *mut GTypeInstance, gtk_container_get_type())
            as *mut std::ffi::c_void as *mut GtkContainer,
        g_type_check_instance_cast((*och).widget as *mut GTypeInstance, gtk_widget_get_type())
            as *mut std::ffi::c_void as *mut GtkWidget,
    );
    gtk_container_add(
        g_type_check_instance_cast(parent as *mut GTypeInstance, gtk_container_get_type())
            as *mut std::ffi::c_void as *mut GtkContainer,
        g_type_check_instance_cast((*nch).widget as *mut GTypeInstance, gtk_widget_get_type())
            as *mut std::ffi::c_void as *mut GtkWidget,
    );
    let mut i_0 = 0 as std::ffi::c_int as guint;
    while i_0 < num_props {
        gtk_container_child_set_property(
            g_type_check_instance_cast(parent as *mut GTypeInstance, gtk_container_get_type())
                as *mut std::ffi::c_void as *mut GtkContainer,
            g_type_check_instance_cast((*nch).widget as *mut GTypeInstance, gtk_widget_get_type())
                as *mut std::ffi::c_void as *mut GtkWidget,
            (**props.offset(i_0 as isize)).data as *const i8,
            &mut *values.offset(i_0 as isize),
        );
        g_value_unset(&mut *values.offset(i_0 as isize));
        i_0 = i_0.wrapping_add(1);
        i_0;
    }
    g_free(props as gpointer);
    g_free(values as gpointer);
    return 0 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_widget_show(mut L: *mut lua_State) -> gint {
    let mut w = luaH_checkwidget(L, 1 as std::ffi::c_int);
    gtk_widget_show((*w).widget);
    return 0 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_widget_hide(mut L: *mut lua_State) -> gint {
    let mut w = luaH_checkwidget(L, 1 as std::ffi::c_int);
    gtk_widget_hide((*w).widget);
    return 0 as std::ffi::c_int;
}
#[unsafe(no_mangle)]

pub unsafe extern "C-unwind" fn luaH_widget_send_key(mut L: *mut lua_State) -> gint {
    let mut w = luaH_checkwidget(L, 1 as std::ffi::c_int);
    let mut key_name = luaL_checklstring(L, 2 as std::ffi::c_int, std::ptr::null_mut());
    if !(lua_type(L, 3 as std::ffi::c_int) == LUA_TTABLE) {
        lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
        lua_insert(L, 3 as std::ffi::c_int);
    }
    let is_release = lua_toboolean(L, 4 as std::ffi::c_int);
    if g_utf8_validate(
        key_name as *const u8,
        -(1 as std::ffi::c_int) as ssize_t,
        std::ptr::null_mut(),
    ) == 0
    {
        return luaL_error(
            L,
            b"key name isn't a utf-8 string\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    let mut keyval: guint = 0;
    if g_utf8_strlen(key_name, -(1 as std::ffi::c_int) as ssize_t) == 1 as std::ffi::c_int as glong
    {
        keyval = gdk_unicode_to_keyval(g_utf8_get_char(key_name));
    } else {
        keyval = gdk_keyval_from_name(key_name);
    }
    if keyval == 0 || keyval == GDK_KEY_VoidSymbol as guint {
        return luaL_error(
            L,
            b"failed to get a valid key value\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    let mut state = 0 as std::ffi::c_int as guint;
    let mut state_string = g_string_sized_new(32);
    lua_pushnil(L);
    while lua_next(L, 3 as std::ffi::c_int) != 0 {
        let mut mod_0 = luaL_checklstring(L, -(1 as std::ffi::c_int), std::ptr::null_mut());
        g_string_append_printf(
            state_string,
            b"%s-\0" as *const u8 as *const std::ffi::c_char,
            mod_0,
        );
        if strcmp(b"shift\0" as *const u8 as *const std::ffi::c_char, mod_0) == 0 as std::ffi::c_int
        {
            state = state | GDK_SHIFT_MASK as std::ffi::c_int as guint;
        }
        if strcmp(b"control\0" as *const u8 as *const std::ffi::c_char, mod_0)
            == 0 as std::ffi::c_int
        {
            state = state | GDK_CONTROL_MASK as std::ffi::c_int as guint;
        }
        if strcmp(b"lock\0" as *const u8 as *const std::ffi::c_char, mod_0) == 0 as std::ffi::c_int
        {
            state = state | GDK_LOCK_MASK as std::ffi::c_int as guint;
        }
        if strcmp(b"mod1\0" as *const u8 as *const std::ffi::c_char, mod_0) == 0 as std::ffi::c_int
        {
            state = state | GDK_MOD1_MASK as std::ffi::c_int as guint;
        }
        if strcmp(b"mod2\0" as *const u8 as *const std::ffi::c_char, mod_0) == 0 as std::ffi::c_int
        {
            state = state | GDK_MOD2_MASK as std::ffi::c_int as guint;
        }
        if strcmp(b"mod3\0" as *const u8 as *const std::ffi::c_char, mod_0) == 0 as std::ffi::c_int
        {
            state = state | GDK_MOD3_MASK as std::ffi::c_int as guint;
        }
        if strcmp(b"mod4\0" as *const u8 as *const std::ffi::c_char, mod_0) == 0 as std::ffi::c_int
        {
            state = state | GDK_MOD4_MASK as std::ffi::c_int as guint;
        }
        if strcmp(b"mod5\0" as *const u8 as *const std::ffi::c_char, mod_0) == 0 as std::ffi::c_int
        {
            state = state | GDK_MOD5_MASK as std::ffi::c_int as guint;
        }
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    }
    let mut keys: *mut GdkKeymapKey = std::ptr::null_mut();
    let mut n_keys: gint = 0;
    let mut keymap = gdk_keymap_get_for_display(gdk_display_get_default());
    if gdk_keymap_get_entries_for_keyval(keymap, keyval, &mut keys, &mut n_keys) == 0 {
        if 0 != 0 {
            if 0 as std::ffi::c_int == 0 {
                g_string_free(state_string, (0 as std::ffi::c_int == 0) as std::ffi::c_int);
            } else {
                g_string_free_and_steal(state_string);
            };
        } else {
            g_string_free(state_string, (0 as std::ffi::c_int == 0) as std::ffi::c_int);
        };
        return luaL_error(
            L,
            b"cannot type '%s' on current keyboard layout\0" as *const u8
                as *const std::ffi::c_char,
            key_name,
        );
    }
    let mut event = gdk_event_new(
        (if is_release != 0 {
            GDK_KEY_RELEASE as std::ffi::c_int
        } else {
            GDK_KEY_PRESS as std::ffi::c_int
        }) as GdkEventType,
    );
    let mut event_key = event as *mut GdkEventKey;
    (*event_key).window = gtk_widget_get_window((*w).widget);
    (*event_key).send_event = GTRUE as i8;
    (*event_key).time = GDK_CURRENT_TIME as guint32;
    (*event_key).state = state;
    (*event_key).keyval = keyval;
    (*event_key).hardware_keycode = (*keys.offset(0 as std::ffi::c_int as isize)).keycode as u16;
    (*event_key).group = (*keys.offset(0 as std::ffi::c_int as isize)).group as guint8;
    let mut kbd = std::ptr::null_mut();
    let mut seat = gdk_display_get_default_seat(gdk_display_get_default());
    kbd = gdk_seat_get_keyboard(seat);
    if kbd.is_null() {
        return luaL_error(
            L,
            b"failed to find a keyboard device\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    gdk_event_set_device(event, kbd);
    let mut ret: gboolean = 0;
    _log(
        LOG_LEVEL_debug,
        b"widgets/common.c\0" as *const u8 as *const std::ffi::c_char,
        &format!(
            "sending key '{}{}' to widget {}",
            CStr::from_ptr((*state_string).str).to_string_lossy(),
            CStr::from_ptr(key_name).to_string_lossy(),
            (*w).widget as usize,
        ),
    );
    g_signal_emit_by_name(
        (*w).widget as *mut GObject,
        if is_release != 0 {
            b"key-release-event\0" as *const u8 as *const std::ffi::c_char
        } else {
            b"key-press-event\0" as *const u8 as *const std::ffi::c_char
        },
        event,
        &mut ret as *mut gboolean,
    );
    if 0 != 0 {
        if 0 as std::ffi::c_int == 0 {
            g_string_free(state_string, (0 as std::ffi::c_int == 0) as std::ffi::c_int);
        } else {
            g_string_free_and_steal(state_string);
        };
    } else {
        g_string_free(state_string, (0 as std::ffi::c_int == 0) as std::ffi::c_int);
    };
    g_free(keys as gpointer);
    return 0 as std::ffi::c_int;
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn luaH_widget_set_visible(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
) -> gint {
    let mut visible = luaH_checkboolean(L, 3 as std::ffi::c_int);
    gtk_widget_set_visible((*w).widget, visible);
    if visible != 0
        && (*(*w).info).tok as std::ffi::c_uint
            == L_TK_WINDOW as std::ffi::c_int as std::ffi::c_uint
    {
        gdk_window_set_events(gtk_widget_get_window((*w).widget), GDK_ALL_EVENTS_MASK);
    }
    return 0 as std::ffi::c_int;
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn luaH_widget_get_min_size(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
) -> gint {
    let mut width: gint = 0;
    let mut height: gint = 0;
    gtk_widget_get_size_request((*w).widget, &mut width, &mut height);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_pushlstring(
        L,
        b"width\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 6]>())
            .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
            .wrapping_sub(1),
    );
    lua_pushinteger(L, width as lua_Integer);
    lua_rawset(L, -(3 as std::ffi::c_int));
    lua_pushlstring(
        L,
        b"height\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 7]>())
            .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
            .wrapping_sub(1),
    );
    lua_pushinteger(L, height as lua_Integer);
    lua_rawset(L, -(3 as std::ffi::c_int));
    return 1 as std::ffi::c_int;
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn luaH_widget_set_min_size(
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
    let mut width: gint = 0;
    let mut height: gint = 0;
    gtk_widget_get_size_request((*w).widget, &mut width, &mut height);
    let mut top = lua_gettop(L);
    if luaH_rawfield(
        L,
        3 as std::ffi::c_int,
        b"w\0" as *const u8 as *const std::ffi::c_char,
    ) != 0
    {
        width = lua_tonumber(L, -(1 as std::ffi::c_int)) as gint;
    }
    if luaH_rawfield(
        L,
        3 as std::ffi::c_int,
        b"h\0" as *const u8 as *const std::ffi::c_char,
    ) != 0
    {
        height = lua_tonumber(L, -(1 as std::ffi::c_int)) as gint;
    }
    lua_settop(L, top);
    gtk_widget_set_size_request((*w).widget, width, height);
    return 1 as std::ffi::c_int;
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn luaH_widget_get_align(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
) -> gint {
    let mut halign = gtk_widget_get_halign(g_type_check_instance_cast(
        (*w).widget as *mut GTypeInstance,
        gtk_widget_get_type(),
    ) as *mut std::ffi::c_void as *mut GtkWidget);
    let mut valign = gtk_widget_get_valign_with_baseline(g_type_check_instance_cast(
        (*w).widget as *mut GTypeInstance,
        gtk_widget_get_type(),
    ) as *mut std::ffi::c_void
        as *mut GtkWidget);
    lua_createtable(L, 0 as std::ffi::c_int, 2 as std::ffi::c_int);
    lua_pushlstring(
        L,
        b"h\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 2]>())
            .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
            .wrapping_sub(1),
    );
    lua_pushnumber(L, halign as lua_Number);
    lua_rawset(L, -(3 as std::ffi::c_int));
    lua_pushlstring(
        L,
        b"v\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 2]>())
            .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
            .wrapping_sub(1),
    );
    lua_pushnumber(L, valign as lua_Number);
    lua_rawset(L, -(3 as std::ffi::c_int));
    return 1 as std::ffi::c_int;
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn luaH_widget_set_align(
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
    let mut halign = gtk_widget_get_halign(g_type_check_instance_cast(
        (*w).widget as *mut GTypeInstance,
        gtk_widget_get_type(),
    ) as *mut std::ffi::c_void as *mut GtkWidget);
    let mut valign = gtk_widget_get_valign_with_baseline(g_type_check_instance_cast(
        (*w).widget as *mut GTypeInstance,
        gtk_widget_get_type(),
    ) as *mut std::ffi::c_void
        as *mut GtkWidget);
    if luaH_rawfield(
        L,
        3 as std::ffi::c_int,
        b"h\0" as *const u8 as *const std::ffi::c_char,
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
        b"v\0" as *const u8 as *const std::ffi::c_char,
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
    gtk_widget_set_halign(
        g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_widget_get_type())
            as *mut std::ffi::c_void as *mut GtkWidget,
        halign,
    );
    gtk_widget_set_valign(
        g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_widget_get_type())
            as *mut std::ffi::c_void as *mut GtkWidget,
        valign,
    );
    return 0 as std::ffi::c_int;
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn luaH_widget_set_tooltip(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
) -> gint {
    let ref mut fresh1 = lua_tolstring(L, 3 as std::ffi::c_int, std::ptr::null_mut());
    gtk_widget_set_tooltip_markup(
        (*w).widget,
        if !(*fresh1).is_null() {
            *fresh1
        } else {
            b"\0" as *const u8 as *const std::ffi::c_char
        },
    );
    return 0 as std::ffi::c_int;
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn luaH_widget_get_tooltip(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
) -> gint {
    lua_pushstring(L, gtk_widget_get_tooltip_markup((*w).widget));
    return 1 as std::ffi::c_int;
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn luaH_widget_get_parent(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
) -> gint {
    let mut widget = gtk_widget_get_parent(g_type_check_instance_cast(
        (*w).widget as *mut GTypeInstance,
        gtk_widget_get_type(),
    ) as *mut std::ffi::c_void as *mut GtkWidget);
    if widget.is_null() {
        return 0 as std::ffi::c_int;
    }
    let mut parent = g_object_get_data(
        g_type_check_instance_cast(
            widget as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject,
        GOBJECT_LUAKIT_WIDGET_DATA_KEY.as_ptr(),
    ) as *mut widget_t;
    luaH_object_push(L, (*parent).ref_0);
    return 1 as std::ffi::c_int;
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn luaH_widget_get_focused(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
) -> gint {
    let mut focused = if (*(*w).info).tok as std::ffi::c_uint
        == L_TK_WINDOW as std::ffi::c_int as std::ffi::c_uint
    {
        gtk_window_has_toplevel_focus(g_type_check_instance_cast(
            (*w).widget as *mut GTypeInstance,
            gtk_window_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkWindow)
    } else {
        gtk_widget_is_focus((*w).widget)
    };
    lua_pushboolean(L, focused);
    return 1 as std::ffi::c_int;
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn luaH_widget_get_visible(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
) -> gint {
    lua_pushboolean(L, gtk_widget_get_visible((*w).widget));
    return 1 as std::ffi::c_int;
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn luaH_widget_get_width(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
) -> gint {
    lua_pushnumber(L, gtk_widget_get_allocated_width((*w).widget) as lua_Number);
    return 1 as std::ffi::c_int;
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn luaH_widget_get_height(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
) -> gint {
    lua_pushnumber(
        L,
        gtk_widget_get_allocated_height((*w).widget) as lua_Number,
    );
    return 1 as std::ffi::c_int;
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_widget_focus(mut L: *mut lua_State) -> gint {
    let mut w = luaH_checkwidget(L, 1 as std::ffi::c_int);
    match (*(*w).info).tok as std::ffi::c_uint {
        263 => {
            gtk_window_set_focus(
                g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_window_get_type())
                    as *mut std::ffi::c_void as *mut GtkWindow,
                std::ptr::null_mut(),
            );
        }
        87 => {
            gtk_entry_grab_focus_without_selecting(g_type_check_instance_cast(
                (*w).widget as *mut GTypeInstance,
                gtk_entry_get_type(),
            ) as *mut std::ffi::c_void
                as *mut GtkEntry);
        }
        _ => {
            gtk_widget_grab_focus((*w).widget);
        }
    }
    return 0 as std::ffi::c_int;
}
#[unsafe(no_mangle)]

pub unsafe extern "C-unwind" fn luaH_widget_destroy(mut L: *mut lua_State) -> gint {
    let mut w = luaH_checkwidget(L, 1 as std::ffi::c_int);
    gtk_widget_destroy(g_type_check_instance_cast(
        (*w).widget as *mut GTypeInstance,
        gtk_widget_get_type(),
    ) as *mut std::ffi::c_void as *mut GtkWidget);
    return 0 as std::ffi::c_int;
}
