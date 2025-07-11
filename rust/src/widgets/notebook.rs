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

unsafe extern "C-unwind" fn luaH_notebook_current(mut L: *mut lua_State) -> gint {
    let mut w = luaH_checkwidget(L, 1 as std::ffi::c_int);
    let mut n = gtk_notebook_get_n_pages(g_type_check_instance_cast(
        (*w).widget as *mut GTypeInstance,
        gtk_notebook_get_type(),
    ) as *mut std::ffi::c_void as *mut GtkNotebook);
    if n == 1 as std::ffi::c_int {
        lua_pushnumber(L, 1 as std::ffi::c_int as lua_Number);
    } else {
        lua_pushnumber(
            L,
            (gtk_notebook_get_current_page(g_type_check_instance_cast(
                (*w).widget as *mut GTypeInstance,
                gtk_notebook_get_type(),
            ) as *mut std::ffi::c_void
                as *mut GtkNotebook)
                + 1 as std::ffi::c_int) as lua_Number,
        );
    }
    return 1 as std::ffi::c_int;
}

unsafe extern "C-unwind" fn luaH_notebook_atindex(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
    mut idx: gint,
) -> gint {
    if idx != -(1 as std::ffi::c_int) {
        idx -= 1;
        idx;
    }
    let mut widget = gtk_notebook_get_nth_page(
        g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_notebook_get_type())
            as *mut std::ffi::c_void as *mut GtkNotebook,
        idx,
    );
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

unsafe extern "C-unwind" fn luaH_notebook_indexof(mut L: *mut lua_State) -> gint {
    let mut w = luaH_checkwidget(L, 1 as std::ffi::c_int);
    let mut child = luaH_checkwidget(L, 2 as std::ffi::c_int);
    let mut i = gtk_notebook_page_num(
        g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_notebook_get_type())
            as *mut std::ffi::c_void as *mut GtkNotebook,
        (*child).widget,
    );
    i += 1;
    if i == 0 {
        return 0 as std::ffi::c_int;
    }
    lua_pushnumber(L, i as lua_Number);
    return 1 as std::ffi::c_int;
}

unsafe extern "C-unwind" fn luaH_notebook_insert(mut L: *mut lua_State) -> gint {
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
    pos = gtk_notebook_insert_page(
        g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_notebook_get_type())
            as *mut std::ffi::c_void as *mut GtkNotebook,
        g_type_check_instance_cast(
            (*luaH_checkwidget(L, idx)).widget as *mut GTypeInstance,
            gtk_widget_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkWidget,
        std::ptr::null_mut(),
        pos,
    );
    if pos == -(1 as std::ffi::c_int) {
        return 0 as std::ffi::c_int;
    }
    pos += 1;
    lua_pushnumber(L, pos as lua_Number);
    return 1 as std::ffi::c_int;
}

unsafe extern "C-unwind" fn luaH_notebook_count(mut L: *mut lua_State) -> gint {
    let mut w = luaH_checkwidget(L, 1 as std::ffi::c_int);
    lua_pushnumber(
        L,
        gtk_notebook_get_n_pages(g_type_check_instance_cast(
            (*w).widget as *mut GTypeInstance,
            gtk_notebook_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkNotebook) as lua_Number,
    );
    return 1 as std::ffi::c_int;
}

unsafe extern "C-unwind" fn luaH_notebook_set_title(mut L: *mut lua_State) -> gint {
    let mut len: size_t = 0;
    let mut w = luaH_checkwidget(L, 1 as std::ffi::c_int);
    let mut child = luaH_checkwidget(L, 2 as std::ffi::c_int);
    let mut title = luaL_checklstring(L, 3 as std::ffi::c_int, &mut len);
    let mut label = gtk_label_new(title);
    gtk_label_set_ellipsize(
        g_type_check_instance_cast(label as *mut GTypeInstance, gtk_label_get_type())
            as *mut std::ffi::c_void as *mut GtkLabel,
        PANGO_ELLIPSIZE_MIDDLE,
    );
    gtk_notebook_set_tab_label(
        g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_notebook_get_type())
            as *mut std::ffi::c_void as *mut GtkNotebook,
        (*child).widget,
        label,
    );
    gtk_container_child_set(
        g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_container_get_type())
            as *mut std::ffi::c_void as *mut GtkContainer,
        label,
        b"tab-expand\0" as *const u8 as *const std::ffi::c_char,
        GTRUE,
        b"tab-fill\0" as *const u8 as *const std::ffi::c_char,
        GTRUE,
        // std::ptr::null_mut(),
    );
    return 0 as std::ffi::c_int;
}

unsafe extern "C-unwind" fn luaH_notebook_get_title(mut L: *mut lua_State) -> gint {
    let mut w = luaH_checkwidget(L, 1 as std::ffi::c_int);
    let mut child = luaH_checkwidget(L, 2 as std::ffi::c_int);
    lua_pushstring(
        L,
        gtk_notebook_get_tab_label_text(
            g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_notebook_get_type())
                as *mut std::ffi::c_void as *mut GtkNotebook,
            (*child).widget,
        ),
    );
    return 1 as std::ffi::c_int;
}

unsafe extern "C-unwind" fn luaH_notebook_switch(mut L: *mut lua_State) -> gint {
    let mut w = luaH_checkwidget(L, 1 as std::ffi::c_int);
    let mut i = luaL_checknumber(L, 2 as std::ffi::c_int) as gint;
    if i != -(1 as std::ffi::c_int) {
        i -= 1;
        i;
    }
    gtk_notebook_set_current_page(
        g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_notebook_get_type())
            as *mut std::ffi::c_void as *mut GtkNotebook,
        i,
    );
    lua_pushnumber(
        L,
        gtk_notebook_get_current_page(g_type_check_instance_cast(
            (*w).widget as *mut GTypeInstance,
            gtk_notebook_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkNotebook) as lua_Number,
    );
    return 1 as std::ffi::c_int;
}

unsafe extern "C-unwind" fn luaH_notebook_reorder(mut L: *mut lua_State) -> gint {
    let mut w = luaH_checkwidget(L, 1 as std::ffi::c_int);
    let mut child = luaH_checkwidget(L, 2 as std::ffi::c_int);
    let mut i = luaL_checknumber(L, 3 as std::ffi::c_int) as gint;
    if i != -(1 as std::ffi::c_int) {
        i -= 1;
        i;
    }
    gtk_notebook_reorder_child(
        g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_notebook_get_type())
            as *mut std::ffi::c_void as *mut GtkNotebook,
        (*child).widget,
        i,
    );
    lua_pushnumber(
        L,
        gtk_notebook_page_num(
            g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_notebook_get_type())
                as *mut std::ffi::c_void as *mut GtkNotebook,
            (*child).widget,
        ) as lua_Number,
    );
    return 1 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_notebook_index(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
    mut token: luakit_token_t,
) -> gint {
    if token as std::ffi::c_uint == L_TK_UNKNOWN as std::ffi::c_int as std::ffi::c_uint
        && lua_isnumber(L, 2 as std::ffi::c_int) != 0
    {
        return luaH_notebook_atindex(L, w, luaL_checknumber(L, 2 as std::ffi::c_int) as gint);
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
        180 => {
            lua_pushcclosure(L, luaH_widget_remove, 0 as std::ffi::c_int);
            return 1 as std::ffi::c_int;
        }
        35 => {
            lua_pushcclosure(L, luaH_notebook_count, 0 as std::ffi::c_int);
            return 1 as std::ffi::c_int;
        }
        39 => {
            lua_pushcclosure(L, luaH_notebook_current, 0 as std::ffi::c_int);
            return 1 as std::ffi::c_int;
        }
        104 => {
            lua_pushcclosure(L, luaH_notebook_get_title, 0 as std::ffi::c_int);
            return 1 as std::ffi::c_int;
        }
        119 => {
            lua_pushcclosure(L, luaH_notebook_indexof, 0 as std::ffi::c_int);
            return 1 as std::ffi::c_int;
        }
        123 => {
            lua_pushcclosure(L, luaH_notebook_insert, 0 as std::ffi::c_int);
            return 1 as std::ffi::c_int;
        }
        210 => {
            lua_pushcclosure(L, luaH_notebook_set_title, 0 as std::ffi::c_int);
            return 1 as std::ffi::c_int;
        }
        232 => {
            lua_pushcclosure(L, luaH_notebook_switch, 0 as std::ffi::c_int);
            return 1 as std::ffi::c_int;
        }
        182 => {
            lua_pushcclosure(L, luaH_notebook_reorder, 0 as std::ffi::c_int);
            return 1 as std::ffi::c_int;
        }
        215 => {
            lua_pushboolean(
                L,
                gtk_notebook_get_show_tabs(g_type_check_instance_cast(
                    (*w).widget as *mut GTypeInstance,
                    gtk_notebook_get_type(),
                ) as *mut std::ffi::c_void
                    as *mut GtkNotebook),
            );
            return 1 as std::ffi::c_int;
        }
        212 => {
            lua_pushboolean(
                L,
                gtk_notebook_get_show_border(g_type_check_instance_cast(
                    (*w).widget as *mut GTypeInstance,
                    gtk_notebook_get_type(),
                ) as *mut std::ffi::c_void
                    as *mut GtkNotebook),
            );
            return 1 as std::ffi::c_int;
        }
        _ => {}
    }
    return 0 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_notebook_newindex(
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
        215 => {
            gtk_notebook_set_show_tabs(
                g_type_check_instance_cast(
                    (*w).widget as *mut GTypeInstance,
                    gtk_notebook_get_type(),
                ) as *mut std::ffi::c_void as *mut GtkNotebook,
                luaH_checkboolean(L, 3 as std::ffi::c_int),
            );
        }
        212 => {
            gtk_notebook_set_show_border(
                g_type_check_instance_cast(
                    (*w).widget as *mut GTypeInstance,
                    gtk_notebook_get_type(),
                ) as *mut std::ffi::c_void as *mut GtkNotebook,
                luaH_checkboolean(L, 3 as std::ffi::c_int),
            );
        }
        _ => return 0 as std::ffi::c_int,
    }
    return luaH_object_property_signal(L, 1 as std::ffi::c_int, token);
}

unsafe extern "C" fn page_added_cb(
    mut UNUSED_n: *mut GtkNotebook,
    mut widget: *mut GtkWidget,
    mut i: guint,
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
    lua_pushnumber(
        L,
        i.wrapping_add(1 as std::ffi::c_int as guint) as lua_Number,
    );
    luaH_object_emit_signal(
        L,
        -(3 as std::ffi::c_int),
        b"page-added\0" as *const u8 as *const std::ffi::c_char,
        2 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
}

unsafe extern "C" fn page_removed_cb(
    mut UNUSED_n: *mut GtkNotebook,
    mut widget: *mut GtkWidget,
    mut UNUSED_i: guint,
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
        b"page-removed\0" as *const u8 as *const std::ffi::c_char,
        1 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
}

unsafe extern "C" fn switch_cb(
    mut n: *mut GtkNotebook,
    mut UNUSED_p: *mut GtkWidget,
    mut i: guint,
    mut w: *mut widget_t,
) {
    let mut widget = gtk_notebook_get_nth_page(
        g_type_check_instance_cast(n as *mut GTypeInstance, gtk_notebook_get_type())
            as *mut std::ffi::c_void as *mut GtkNotebook,
        i as gint,
    );
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
    lua_pushnumber(
        L,
        i.wrapping_add(1 as std::ffi::c_int as guint) as lua_Number,
    );
    luaH_object_emit_signal(
        L,
        -(3 as std::ffi::c_int),
        b"switch-page\0" as *const u8 as *const std::ffi::c_char,
        2 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
}

unsafe extern "C" fn reorder_cb(
    mut UNUSED_n: *mut GtkNotebook,
    mut widget: *mut GtkWidget,
    mut i: guint,
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
    lua_pushnumber(
        L,
        i.wrapping_add(1 as std::ffi::c_int as guint) as lua_Number,
    );
    luaH_object_emit_signal(
        L,
        -(3 as std::ffi::c_int),
        b"page-reordered\0" as *const u8 as *const std::ffi::c_char,
        2 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn widget_notebook(
    mut UNUSED_L: *mut lua_State,
    mut w: *mut widget_t,
    mut UNUSED_token: luakit_token_t,
) -> *mut widget_t {
    (*w).index = Some(luaH_notebook_index);
    (*w).newindex = Some(luaH_notebook_newindex);
    (*w).widget = gtk_notebook_new();
    gtk_notebook_set_show_border(
        g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_notebook_get_type())
            as *mut std::ffi::c_void as *mut GtkNotebook,
        GFALSE,
    );
    gtk_notebook_set_scrollable(
        g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_notebook_get_type())
            as *mut std::ffi::c_void as *mut GtkNotebook,
        GTRUE,
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
        b"signal::page-added\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(*mut GtkNotebook, *mut GtkWidget, guint, *mut widget_t) -> (),
            >,
            GCallback,
        >(Some(
            page_added_cb
                as unsafe extern "C" fn(
                    *mut GtkNotebook,
                    *mut GtkWidget,
                    guint,
                    *mut widget_t,
                ) -> (),
        )),
        w,
        b"signal::page-removed\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(*mut GtkNotebook, *mut GtkWidget, guint, *mut widget_t) -> (),
            >,
            GCallback,
        >(Some(
            page_removed_cb
                as unsafe extern "C" fn(
                    *mut GtkNotebook,
                    *mut GtkWidget,
                    guint,
                    *mut widget_t,
                ) -> (),
        )),
        w,
        b"signal::page-reordered\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(*mut GtkNotebook, *mut GtkWidget, guint, *mut widget_t) -> (),
            >,
            GCallback,
        >(Some(
            reorder_cb
                as unsafe extern "C" fn(
                    *mut GtkNotebook,
                    *mut GtkWidget,
                    guint,
                    *mut widget_t,
                ) -> (),
        )),
        w,
        b"signal::switch-page\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(*mut GtkNotebook, *mut GtkWidget, guint, *mut widget_t) -> (),
            >,
            GCallback,
        >(Some(
            switch_cb
                as unsafe extern "C" fn(
                    *mut GtkNotebook,
                    *mut GtkWidget,
                    guint,
                    *mut widget_t,
                ) -> (),
        )),
        w,
        // std::ptr::null_mut(),
    );
    gtk_widget_show((*w).widget);
    return w;
}
