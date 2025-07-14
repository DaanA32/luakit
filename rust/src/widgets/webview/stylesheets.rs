use cairo_sys::*;
use gdk_pixbuf_sys::*;
use gdk_sys::*;
use gio_sys::*;
use glib_sys::*;
use gobject_sys::*;
use gtk_sys::*;
use libc::*;
use mlua::ffi::*;
use pango_sys::*;
use webkit2gtk_sys::*;

use crate::clib::stylesheet::lstylesheet_t;
use crate::clib::stylesheet::luaH_checkstylesheet;
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
use crate::widgets::webview::luaH_checkwebview;
use crate::widgets::webview::webview_data_t;
use crate::widgets::*;

pub static mut inside_stylesheet_cb: gboolean = GFALSE;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn webview_stylesheets_regenerate_stylesheet(
    mut w: *mut widget_t,
    mut stylesheet: *mut lstylesheet_t,
) {
    let mut d = (*w).data as *mut webview_data_t;
    if !(g_list_find((*d).stylesheets, stylesheet as gconstpointer)).is_null() {
        (*d).stylesheet_refreshed = GTRUE;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn webview_stylesheets_regenerate(mut w: *mut widget_t) {
    let mut d = (*w).data as *mut webview_data_t;
    if (*d).stylesheet_added != 0 || (*d).stylesheet_removed != 0 || (*d).stylesheet_refreshed != 0
    {
        webkit_user_content_manager_remove_all_style_sheets((*d).user_content);
        let mut l = 0 as *mut GList;
        l = (*d).stylesheets;
        while !l.is_null() {
            let mut stylesheet = (*l).data as *mut lstylesheet_t;
            webkit_user_content_manager_add_style_sheet(
                (*d).user_content,
                (*stylesheet).stylesheet,
            );
            l = (*l).next;
        }
        (*d).stylesheet_added = GFALSE;
        (*d).stylesheet_removed = GFALSE;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn webview_stylesheet_set_enabled(
    mut w: *mut widget_t,
    mut stylesheet: *mut lstylesheet_t,
    mut enable: gboolean,
) -> std::ffi::c_int {
    let mut d = (*w).data as *mut webview_data_t;
    let mut item = g_list_find((*d).stylesheets, stylesheet as gconstpointer);
    if enable == (item != std::ptr::null_mut()) as std::ffi::c_int {
        return 0 as std::ffi::c_int;
    }
    if enable != 0 {
        (*d).stylesheets = g_list_prepend((*d).stylesheets, stylesheet as gpointer);
        (*d).stylesheet_added = GTRUE;
    } else {
        (*d).stylesheets = g_list_remove_link((*d).stylesheets, item);
        (*d).stylesheet_removed = GTRUE;
    }
    if inside_stylesheet_cb == 0 {
        webview_stylesheets_regenerate(w);
    }
    return 0 as std::ffi::c_int;
}
pub unsafe extern "C-unwind" fn luaH_webview_stylesheets_index(mut L: *mut lua_State) -> gint {
    let mut d = (*luaH_checkwebview(L, -(10002 as std::ffi::c_int) - 1 as std::ffi::c_int)).data
        as *mut webview_data_t;
    let mut stylesheet = luaH_checkstylesheet(L, 2 as std::ffi::c_int) as *mut lstylesheet_t;
    let mut enabled = (g_list_find((*d).stylesheets, stylesheet as gconstpointer)
        != std::ptr::null_mut()) as std::ffi::c_int;
    lua_pushboolean(L, enabled);
    return 1 as std::ffi::c_int;
}
pub unsafe extern "C-unwind" fn luaH_webview_stylesheets_newindex(mut L: *mut lua_State) -> gint {
    let mut d = (*luaH_checkwebview(L, -(10002 as std::ffi::c_int) - 1 as std::ffi::c_int)).data
        as *mut webview_data_t;
    let mut stylesheet = luaH_checkstylesheet(L, 2 as std::ffi::c_int) as *mut lstylesheet_t;
    let mut enable = lua_toboolean(L, 3 as std::ffi::c_int);
    webview_stylesheet_set_enabled((*d).widget, stylesheet, enable);
    return 0 as std::ffi::c_int;
}
pub unsafe extern "C-unwind" fn luaH_webview_push_stylesheets_table(mut L: *mut lua_State) -> gint {
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_createtable(L, 0 as std::ffi::c_int, 2 as std::ffi::c_int);
    lua_pushlstring(
        L,
        b"__index\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 8]>())
            .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
            .wrapping_sub(1),
    );
    lua_pushvalue(L, 1 as std::ffi::c_int);
    lua_pushcclosure(L, luaH_webview_stylesheets_index, 1 as std::ffi::c_int);
    lua_rawset(L, -(3 as std::ffi::c_int));
    lua_pushlstring(
        L,
        b"__newindex\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 11]>())
            .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
            .wrapping_sub(1),
    );
    lua_pushvalue(L, 1 as std::ffi::c_int);
    lua_pushcclosure(L, luaH_webview_stylesheets_newindex, 1 as std::ffi::c_int);
    lua_rawset(L, -(3 as std::ffi::c_int));
    lua_setmetatable(L, -(2 as std::ffi::c_int));
    return 1 as std::ffi::c_int;
}
pub unsafe extern "C" fn webview_update_stylesheets(mut L: *mut lua_State, mut w: *mut widget_t) {
    let mut d = (*w).data as *mut webview_data_t;
    (*d).stylesheet_added = GFALSE;
    (*d).stylesheet_removed = GFALSE;
    inside_stylesheet_cb = GTRUE;
    luaH_object_push(L, (*w).ref_0);
    luaH_object_emit_signal(
        L,
        -(1 as std::ffi::c_int),
        b"stylesheet\0" as *const u8 as *const std::ffi::c_char,
        0 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    inside_stylesheet_cb = GFALSE;
    webview_stylesheets_regenerate(w);
}
