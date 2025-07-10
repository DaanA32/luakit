use glib_sys::{GError, GType, g_free, gboolean, gpointer};
use gtk_sys::{
    GTK_STYLE_PROVIDER_PRIORITY_APPLICATION, GtkEditable, GtkEntry, GtkStyleProvider, GtkWidget,
    gtk_css_provider_load_from_data, gtk_css_provider_new, gtk_editable_get_position,
    gtk_editable_get_type, gtk_editable_insert_text, gtk_editable_select_region,
    gtk_editable_set_position, gtk_entry_get_has_frame, gtk_entry_get_text, gtk_entry_get_type,
    gtk_entry_new, gtk_entry_set_has_frame, gtk_entry_set_text, gtk_style_context_add_provider,
    gtk_style_provider_get_type, gtk_widget_get_style_context, gtk_widget_get_type,
    gtk_widget_show,
};
use mlua_sys::{
    lua_Integer, lua_State, lua_gettop, lua_pushboolean, lua_pushcclosure, lua_pushinteger,
    lua_pushstring, lua_settop, luaL_argerror, luaL_checklstring, luaL_checknumber,
};

use crate::widgets::luaH_checkwidget;
unsafe extern "C" fn luaH_entry_insert(mut L: *mut lua_State) -> gint {
    let mut w = luaH_checkwidget(L, 1 as std::ffi::c_int);
    let mut pos = -(1 as std::ffi::c_int);
    let mut idx = 2 as std::ffi::c_int;
    if lua_gettop(L) > 2 as std::ffi::c_int {
        let fresh0 = idx;
        idx = idx + 1;
        pos = luaL_checknumber(L, fresh0) as gint;
        if pos > 0 as std::ffi::c_int {
            pos -= 1;
            pos;
        }
    }
    gtk_editable_insert_text(
        g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_editable_get_type())
            as *mut std::ffi::c_void as *mut GtkEditable,
        luaL_checklstring(L, idx, NULL_0 as *mut size_t),
        -(1 as std::ffi::c_int),
        &mut pos,
    );
    return 0 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_entry_select_region(mut L: *mut lua_State) -> gint {
    let mut w = luaH_checkwidget(L, 1 as std::ffi::c_int);
    let mut startpos = luaL_checknumber(L, 2 as std::ffi::c_int) as gint;
    let mut endpos = -(1 as std::ffi::c_int);
    if lua_gettop(L) > 2 as std::ffi::c_int {
        endpos = luaL_checknumber(L, 3 as std::ffi::c_int) as gint;
    }
    gtk_editable_select_region(
        g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_editable_get_type())
            as *mut std::ffi::c_void as *mut GtkEditable,
        startpos,
        endpos,
    );
    return 0 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_entry_index(
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
        123 => {
            lua_pushcclosure(
                L,
                Some(luaH_entry_insert as unsafe extern "C" fn(*mut lua_State) -> gint),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        202 => {
            lua_pushcclosure(
                L,
                Some(luaH_entry_select_region as unsafe extern "C" fn(*mut lua_State) -> gint),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        167 => {
            lua_pushinteger(
                L,
                gtk_editable_get_position(g_type_check_instance_cast(
                    (*w).widget as *mut GTypeInstance,
                    gtk_editable_get_type(),
                ) as *mut std::ffi::c_void
                    as *mut GtkEditable) as lua_Integer,
            );
            return 1 as std::ffi::c_int;
        }
        237 => {
            lua_pushstring(
                L,
                gtk_entry_get_text(g_type_check_instance_cast(
                    (*w).widget as *mut GTypeInstance,
                    gtk_entry_get_type(),
                ) as *mut std::ffi::c_void as *mut GtkEntry),
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
        213 => {
            lua_pushboolean(
                L,
                gtk_entry_get_has_frame(g_type_check_instance_cast(
                    (*w).widget as *mut GTypeInstance,
                    gtk_entry_get_type(),
                ) as *mut std::ffi::c_void
                    as *mut GtkEntry),
            );
            return 1 as std::ffi::c_int;
        }
        _ => {}
    }
    return 0 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_entry_newindex(
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
        237 => {
            gtk_entry_set_text(
                g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_entry_get_type())
                    as *mut std::ffi::c_void as *mut GtkEntry,
                luaL_checklstring(L, 3 as std::ffi::c_int, &mut len),
            );
        }
        94 | 13 => {
            tmp = luaL_checklstring(L, 3 as std::ffi::c_int, &mut len);
            if gdk_rgba_parse(&mut c, tmp) == 0 {
                luaL_argerror(
                    L,
                    3 as std::ffi::c_int,
                    b"unable to parse color\0" as *const u8 as *const std::ffi::c_char,
                );
            }
            if token as std::ffi::c_uint == L_TK_FG as std::ffi::c_int as std::ffi::c_uint {
                widget_set_css_properties(
                    w,
                    b"color\0" as *const u8 as *const std::ffi::c_char,
                    tmp,
                    NULL_0 as *mut std::ffi::c_void,
                );
                widget_set_css_properties(
                    w,
                    b"caret-color\0" as *const u8 as *const std::ffi::c_char,
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
            } else {
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
        }
        213 => {
            gtk_entry_set_has_frame(
                g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_entry_get_type())
                    as *mut std::ffi::c_void as *mut GtkEntry,
                luaH_checkboolean(L, 3 as std::ffi::c_int),
            );
        }
        167 => {
            gtk_editable_set_position(
                g_type_check_instance_cast(
                    (*w).widget as *mut GTypeInstance,
                    gtk_editable_get_type(),
                ) as *mut std::ffi::c_void as *mut GtkEditable,
                luaL_checknumber(L, 3 as std::ffi::c_int) as gint,
            );
        }
        101 => {
            tmp = luaL_checklstring(L, 3 as std::ffi::c_int, &mut len);
            widget_set_css_properties(
                w,
                b"font\0" as *const u8 as *const std::ffi::c_char,
                tmp,
                NULL_0 as *mut std::ffi::c_void,
            );
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

unsafe extern "C" fn activate_cb(mut UNUSED_e: *mut GtkEntry, mut w: *mut widget_t) {
    let mut L = common.L;
    luaH_object_push(L, (*w).ref_0);
    luaH_object_emit_signal(
        L,
        -(1 as std::ffi::c_int),
        b"activate\0" as *const u8 as *const std::ffi::c_char,
        0 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
}

unsafe extern "C" fn changed_cb(mut w: *mut widget_t) {
    let mut L = common.L;
    luaH_object_push(L, (*w).ref_0);
    luaH_object_emit_signal(
        L,
        -(1 as std::ffi::c_int),
        b"changed\0" as *const u8 as *const std::ffi::c_char,
        0 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
}

unsafe extern "C" fn position_cb(
    mut UNUSED_e: *mut GtkEntry,
    mut UNUSED_ps: *mut GParamSpec,
    mut w: *mut widget_t,
) {
    let mut L = common.L;
    luaH_object_push(L, (*w).ref_0);
    luaH_object_emit_signal(
        L,
        -(1 as std::ffi::c_int),
        b"property::position\0" as *const u8 as *const std::ffi::c_char,
        0 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn widget_entry(
    mut UNUSED_L: *mut lua_State,
    mut w: *mut widget_t,
    mut UNUSED_token: luakit_token_t,
) -> *mut widget_t {
    (*w).index = Some(
        luaH_entry_index
            as unsafe extern "C" fn(*mut lua_State, *mut widget_t, luakit_token_t) -> gint,
    );
    (*w).newindex = Some(
        luaH_entry_newindex
            as unsafe extern "C" fn(*mut lua_State, *mut widget_t, luakit_token_t) -> gint,
    );
    (*w).widget = gtk_entry_new();
    let mut context = gtk_widget_get_style_context(g_type_check_instance_cast(
        (*w).widget as *mut GTypeInstance,
        gtk_widget_get_type(),
    ) as *mut std::ffi::c_void
        as *mut GtkWidget);
    let mut inputbar_css =
        b"GtkEntry {border: none; padding: 2px;}\0" as *const u8 as *const std::ffi::c_char;
    let mut provider = gtk_css_provider_new();
    gtk_css_provider_load_from_data(
        provider,
        inputbar_css,
        strlen(inputbar_css) as gssize,
        NULL_0 as *mut *mut GError,
    );
    gtk_style_context_add_provider(
        context,
        g_type_check_instance_cast(
            provider as *mut GTypeInstance,
            gtk_style_provider_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkStyleProvider,
        GTK_STYLE_PROVIDER_PRIORITY_APPLICATION as guint,
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
        b"signal::activate\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GtkEntry, *mut widget_t) -> ()>,
            GCallback,
        >(Some(
            activate_cb as unsafe extern "C" fn(*mut GtkEntry, *mut widget_t) -> (),
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
        b"signal::notify::cursor-position\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GtkEntry, *mut GParamSpec, *mut widget_t) -> ()>,
            GCallback,
        >(Some(
            position_cb
                as unsafe extern "C" fn(*mut GtkEntry, *mut GParamSpec, *mut widget_t) -> (),
        )),
        w,
        NULL_0 as *mut std::ffi::c_void,
    );
    g_object_connect(
        g_type_check_instance_cast(
            (*w).widget as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject as gpointer,
        b"swapped-signal::changed\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut widget_t) -> ()>, GCallback>(
            Some(changed_cb as unsafe extern "C" fn(*mut widget_t) -> ()),
        ),
        w,
        NULL_0 as *mut std::ffi::c_void,
    );
    gtk_widget_show((*w).widget);
    return w;
}
