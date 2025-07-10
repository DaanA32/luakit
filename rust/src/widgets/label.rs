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

unsafe extern "C" fn luaH_label_get_align(mut L: *mut lua_State, mut w: *mut widget_t) -> gint {
    let mut xalign: gfloat = 0.;
    let mut yalign: gfloat = 0.;
    xalign = gtk_label_get_xalign(g_type_check_instance_cast(
        (*w).widget as *mut GTypeInstance,
        gtk_label_get_type(),
    ) as *mut std::ffi::c_void as *mut GtkLabel);
    yalign = gtk_label_get_yalign(g_type_check_instance_cast(
        (*w).widget as *mut GTypeInstance,
        gtk_label_get_type(),
    ) as *mut std::ffi::c_void as *mut GtkLabel);
    luaH_widget_get_align(L, w);
    lua_pushlstring(
        L,
        b"x\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 2]>() as std::ffi::c_ulong)
            .wrapping_div(::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong)
            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
    );
    lua_pushnumber(L, xalign as lua_Number);
    lua_rawset(L, -(3 as std::ffi::c_int));
    lua_pushlstring(
        L,
        b"y\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 2]>() as std::ffi::c_ulong)
            .wrapping_div(::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong)
            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
    );
    lua_pushnumber(L, yalign as lua_Number);
    lua_rawset(L, -(3 as std::ffi::c_int));
    return 1 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_label_set_align(mut L: *mut lua_State, mut w: *mut widget_t) -> gint {
    luaH_widget_set_align(L, w);
    let mut xalign: gfloat = 0.;
    let mut yalign: gfloat = 0.;
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
        b"x\0" as *const u8 as *const std::ffi::c_char,
    ) != 0
    {
        xalign = lua_tonumber(L, -(1 as std::ffi::c_int)) as gfloat;
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        gtk_label_set_xalign(
            g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_label_get_type())
                as *mut std::ffi::c_void as *mut GtkLabel,
            xalign,
        );
    }
    if luaH_rawfield(
        L,
        3 as std::ffi::c_int,
        b"y\0" as *const u8 as *const std::ffi::c_char,
    ) != 0
    {
        yalign = lua_tonumber(L, -(1 as std::ffi::c_int)) as gfloat;
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        gtk_label_set_yalign(
            g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_label_get_type())
                as *mut std::ffi::c_void as *mut GtkLabel,
            yalign,
        );
    }
    return 0 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_label_index(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
    mut token: luakit_token_t,
) -> gint {
    if token as std::ffi::c_uint == L_TK_ALIGN as std::ffi::c_int as std::ffi::c_uint {
        return luaH_label_get_align(L, w);
    }
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
        94 => {
            lua_pushstring(
                L,
                g_object_get_data(
                    g_type_check_instance_cast(
                        (*w).widget as *mut GTypeInstance,
                        ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
                    ) as *mut std::ffi::c_void as *mut GObject,
                    b"fg\0" as *const u8 as *const std::ffi::c_char,
                ) as *const std::ffi::c_char,
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
        101 => {
            lua_pushstring(
                L,
                g_object_get_data(
                    g_type_check_instance_cast(
                        (*w).widget as *mut GTypeInstance,
                        ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
                    ) as *mut std::ffi::c_void as *mut GObject,
                    b"font\0" as *const u8 as *const std::ffi::c_char,
                ) as *const std::ffi::c_char,
            );
            return 1 as std::ffi::c_int;
        }
        237 => {
            lua_pushstring(
                L,
                gtk_label_get_label(g_type_check_instance_cast(
                    (*w).widget as *mut GTypeInstance,
                    gtk_label_get_type(),
                ) as *mut std::ffi::c_void as *mut GtkLabel),
            );
            return 1 as std::ffi::c_int;
        }
        200 => {
            lua_pushboolean(
                L,
                gtk_label_get_selectable(g_type_check_instance_cast(
                    (*w).widget as *mut GTypeInstance,
                    gtk_label_get_type(),
                ) as *mut std::ffi::c_void
                    as *mut GtkLabel),
            );
            return 1 as std::ffi::c_int;
        }
        238 => {
            lua_pushinteger(
                L,
                gtk_label_get_width_chars(g_type_check_instance_cast(
                    (*w).widget as *mut GTypeInstance,
                    gtk_label_get_type(),
                ) as *mut std::ffi::c_void
                    as *mut GtkLabel) as lua_Integer,
            );
            return 1 as std::ffi::c_int;
        }
        _ => {}
    }
    return 0 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_label_newindex(
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
    let mut font = 0 as *mut PangoFontDescription;
    if token as std::ffi::c_uint == L_TK_ALIGN as std::ffi::c_int as std::ffi::c_uint {
        luaH_label_set_align(L, w);
        return luaH_object_property_signal(L, 1 as std::ffi::c_int, token);
    }
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
        237 => {
            gtk_label_set_markup(
                g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_label_get_type())
                    as *mut std::ffi::c_void as *mut GtkLabel,
                luaL_checklstring(L, 3 as std::ffi::c_int, &mut len),
            );
        }
        94 => {
            tmp = luaL_checklstring(L, 3 as std::ffi::c_int, &mut len);
            if gdk_rgba_parse(&mut c, tmp) == 0 {
                luaL_argerror(
                    L,
                    3 as std::ffi::c_int,
                    b"unable to parse color\0" as *const u8 as *const std::ffi::c_char,
                );
            }
            widget_set_css_properties(
                w,
                b"color\0" as *const u8 as *const std::ffi::c_char,
                tmp,
                NULL_0 as *mut std::ffi::c_void,
            );
            g_object_set_data_full(
                g_type_check_instance_cast(
                    (*w).widget as *mut GTypeInstance,
                    ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
                ) as *mut std::ffi::c_void as *mut GObject,
                b"fg\0" as *const u8 as *const std::ffi::c_char,
                g_strdup_inline(tmp) as gpointer,
                Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
            );
        }
        13 => {
            tmp = luaL_checklstring(L, 3 as std::ffi::c_int, &mut len);
            if gdk_rgba_parse(&mut c, tmp) == 0 {
                luaL_argerror(
                    L,
                    3 as std::ffi::c_int,
                    b"unable to parse color\0" as *const u8 as *const std::ffi::c_char,
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
        101 => {
            tmp = luaL_checklstring(L, 3 as std::ffi::c_int, &mut len);
            font = pango_font_description_from_string(tmp);
            widget_set_css_properties(
                w,
                b"font\0" as *const u8 as *const std::ffi::c_char,
                tmp,
                NULL_0 as *mut std::ffi::c_void,
            );
            pango_font_description_free(font);
            g_object_set_data_full(
                g_type_check_instance_cast(
                    (*w).widget as *mut GTypeInstance,
                    ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
                ) as *mut std::ffi::c_void as *mut GObject,
                b"font\0" as *const u8 as *const std::ffi::c_char,
                g_strdup_inline(tmp) as gpointer,
                Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
            );
        }
        200 => {
            gtk_label_set_selectable(
                g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_label_get_type())
                    as *mut std::ffi::c_void as *mut GtkLabel,
                luaH_checkboolean(L, 3 as std::ffi::c_int),
            );
        }
        238 => {
            gtk_label_set_width_chars(
                g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_label_get_type())
                    as *mut std::ffi::c_void as *mut GtkLabel,
                luaL_checknumber(L, 3 as std::ffi::c_int) as gint,
            );
        }
        _ => {
            luaH_warn(
                L,
                b"unknown property: %s\0" as *const u8 as *const std::ffi::c_char,
                luaL_checklstring(L, 2 as std::ffi::c_int, NULL_0 as *mut size_t),
            );
            return 0 as std::ffi::c_int;
        }
    }
    return luaH_object_property_signal(L, 1 as std::ffi::c_int, token);
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn widget_label(
    mut UNUSED_L: *mut lua_State,
    mut w: *mut widget_t,
    mut UNUSED_token: luakit_token_t,
) -> *mut widget_t {
    (*w).index = Some(
        luaH_label_index
            as unsafe extern "C" fn(*mut lua_State, *mut widget_t, luakit_token_t) -> gint,
    );
    (*w).newindex = Some(
        luaH_label_newindex
            as unsafe extern "C" fn(*mut lua_State, *mut widget_t, luakit_token_t) -> gint,
    );
    (*w).widget = gtk_label_new(NULL_0 as *const gchar);
    gtk_label_set_ellipsize(
        g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_label_get_type())
            as *mut std::ffi::c_void as *mut GtkLabel,
        PANGO_ELLIPSIZE_END,
    );
    gtk_label_set_selectable(
        g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_label_get_type())
            as *mut std::ffi::c_void as *mut GtkLabel,
        FALSE,
    );
    gtk_label_set_use_markup(
        g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_label_get_type())
            as *mut std::ffi::c_void as *mut GtkLabel,
        TRUE,
    );
    gtk_widget_set_halign(
        g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_widget_get_type())
            as *mut std::ffi::c_void as *mut GtkWidget,
        GTK_ALIGN_START,
    );
    gtk_widget_set_valign(
        g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_widget_get_type())
            as *mut std::ffi::c_void as *mut GtkWidget,
        GTK_ALIGN_START,
    );
    let mut margin = {
        let mut init = _GValue {
            g_type: 0 as std::ffi::c_int as GType,
            data: [
                C2RustUnnamed {
                    v_int: 0 as std::ffi::c_int,
                },
                C2RustUnnamed { v_int: 0 },
            ],
        };
        init
    };
    g_value_init(&mut margin, G_TYPE_INT as GType);
    g_value_set_int(&mut margin, 2 as std::ffi::c_int);
    g_object_set_property(
        g_type_check_instance_cast(
            (*w).widget as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject,
        b"margin\0" as *const u8 as *const std::ffi::c_char,
        &mut margin,
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
        NULL_0 as *mut std::ffi::c_void,
    );
    gtk_widget_show((*w).widget);
    return w;
}
