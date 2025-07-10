use glib_sys::{GFALSE, GTRUE, GType, g_free, gboolean, gpointer};
use gtk_sys::{
    GTK_ORIENTATION_HORIZONTAL, GTK_ORIENTATION_VERTICAL, GtkBox, GtkContainer, GtkOrientation,
    GtkWidget, gtk_box_get_homogeneous, gtk_box_get_spacing, gtk_box_get_type, gtk_box_new,
    gtk_box_pack_end, gtk_box_pack_start, gtk_box_reorder_child, gtk_box_set_homogeneous,
    gtk_box_set_spacing, gtk_widget_get_type, gtk_widget_show,
};
use mlua_sys::{
    LUA_TNIL, LUA_TTABLE, lua_Number, lua_State, lua_gettop, lua_pushboolean, lua_pushcclosure,
    lua_pushnumber, lua_pushstring, lua_settop, lua_toboolean, lua_tolstring, lua_tonumber,
    lua_type, luaL_argerror, luaL_checklstring, luaL_checknumber, luaL_typerror,
};

use crate::widgets::luaH_checkwidget;

unsafe extern "C" fn luaH_box_pack(mut L: *mut lua_State) -> gint {
    let mut w = luaH_checkwidget(L, 1 as std::ffi::c_int);
    let mut child = luaH_checkwidget(L, 2 as std::ffi::c_int);
    let mut top = lua_gettop(L);
    let mut expand = GFALSE;
    let mut fill = GFALSE;
    let mut start = GTRUE;
    let mut padding = 0 as std::ffi::c_int as guint;
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
            b"from\0" as *const u8 as *const std::ffi::c_char,
        ) != 0
        {
            start = if L_TK_END as std::ffi::c_int as std::ffi::c_uint
                == l_tokenize(lua_tolstring(
                    L,
                    -(1 as std::ffi::c_int),
                    NULL_0 as *mut size_t,
                )) as std::ffi::c_uint
            {
                FALSE
            } else {
                TRUE
            };
        }
        if luaH_rawfield(
            L,
            3 as std::ffi::c_int,
            b"expand\0" as *const u8 as *const std::ffi::c_char,
        ) != 0
        {
            expand = if lua_toboolean(L, -(1 as std::ffi::c_int)) != 0 {
                TRUE
            } else {
                FALSE
            };
        }
        if luaH_rawfield(
            L,
            3 as std::ffi::c_int,
            b"fill\0" as *const u8 as *const std::ffi::c_char,
        ) != 0
        {
            fill = if lua_toboolean(L, -(1 as std::ffi::c_int)) != 0 {
                TRUE
            } else {
                FALSE
            };
        }
        if luaH_rawfield(
            L,
            3 as std::ffi::c_int,
            b"padding\0" as *const u8 as *const std::ffi::c_char,
        ) != 0
        {
            padding = lua_tonumber(L, -(1 as std::ffi::c_int)) as guint;
        }
        lua_settop(L, top);
    }
    if start != 0 {
        gtk_box_pack_start(
            g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_box_get_type())
                as *mut std::ffi::c_void as *mut GtkBox,
            g_type_check_instance_cast((*child).widget as *mut GTypeInstance, gtk_widget_get_type())
                as *mut std::ffi::c_void as *mut GtkWidget,
            expand,
            fill,
            padding,
        );
    } else {
        gtk_box_pack_end(
            g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_box_get_type())
                as *mut std::ffi::c_void as *mut GtkBox,
            g_type_check_instance_cast((*child).widget as *mut GTypeInstance, gtk_widget_get_type())
                as *mut std::ffi::c_void as *mut GtkWidget,
            expand,
            fill,
            padding,
        );
    }
    return 0 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_box_reorder_child(mut L: *mut lua_State) -> gint {
    let mut w = luaH_checkwidget(L, 1 as std::ffi::c_int);
    let mut child = luaH_checkwidget(L, 2 as std::ffi::c_int);
    let mut pos = luaL_checknumber(L, 3 as std::ffi::c_int) as gint;
    gtk_box_reorder_child(
        g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_box_get_type())
            as *mut std::ffi::c_void as *mut GtkBox,
        g_type_check_instance_cast((*child).widget as *mut GTypeInstance, gtk_widget_get_type())
            as *mut std::ffi::c_void as *mut GtkWidget,
        pos,
    );
    return 0 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_box_index(
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
        180 => {
            lua_pushcclosure(
                L,
                Some(luaH_widget_remove as unsafe extern "C" fn(*mut lua_State) -> gint),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        159 => {
            lua_pushcclosure(
                L,
                Some(luaH_box_pack as unsafe extern "C" fn(*mut lua_State) -> gint),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        182 => {
            lua_pushcclosure(
                L,
                Some(luaH_box_reorder_child as unsafe extern "C" fn(*mut lua_State) -> gint),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        112 => {
            lua_pushboolean(
                L,
                gtk_box_get_homogeneous(g_type_check_instance_cast(
                    (*w).widget as *mut GTypeInstance,
                    gtk_box_get_type(),
                ) as *mut std::ffi::c_void as *mut GtkBox),
            );
            return 1 as std::ffi::c_int;
        }
        218 => {
            lua_pushnumber(
                L,
                gtk_box_get_spacing(g_type_check_instance_cast(
                    (*w).widget as *mut GTypeInstance,
                    gtk_box_get_type(),
                ) as *mut std::ffi::c_void as *mut GtkBox) as lua_Number,
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

unsafe extern "C" fn luaH_box_newindex(
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
        112 => {
            gtk_box_set_homogeneous(
                g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_box_get_type())
                    as *mut std::ffi::c_void as *mut GtkBox,
                luaH_checkboolean(L, 3 as std::ffi::c_int),
            );
        }
        218 => {
            gtk_box_set_spacing(
                g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_box_get_type())
                    as *mut std::ffi::c_void as *mut GtkBox,
                luaL_checknumber(L, 3 as std::ffi::c_int) as gint,
            );
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

pub unsafe extern "C" fn widget_box(
    mut UNUSED_L: *mut lua_State,
    mut w: *mut widget_t,
    mut token: luakit_token_t,
) -> *mut widget_t {
    (*w).index = Some(
        luaH_box_index
            as unsafe extern "C" fn(*mut lua_State, *mut widget_t, luakit_token_t) -> gint,
    );
    (*w).newindex = Some(
        luaH_box_newindex
            as unsafe extern "C" fn(*mut lua_State, *mut widget_t, luakit_token_t) -> gint,
    );
    (*w).widget = gtk_box_new(
        (if token as std::ffi::c_uint == L_TK_VBOX as std::ffi::c_int as std::ffi::c_uint {
            GTK_ORIENTATION_VERTICAL as std::ffi::c_int
        } else {
            GTK_ORIENTATION_HORIZONTAL as std::ffi::c_int
        }) as GtkOrientation,
        0 as std::ffi::c_int,
    );
    gtk_box_set_homogeneous(
        g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_box_get_type())
            as *mut std::ffi::c_void as *mut GtkBox,
        if token as std::ffi::c_uint == L_TK_VBOX as std::ffi::c_int as std::ffi::c_uint {
            FALSE
        } else {
            TRUE
        },
    );
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
        NULL_0 as *mut std::ffi::c_void,
    );
    gtk_widget_show((*w).widget);
    return w;
}
