use cairo_sys::*;
use gdk_pixbuf_sys::*;
use gdk_sys::*;
use gio_sys::*;
use glib_sys::*;
use gobject_sys::*;
use gtk_sys::*;
use libc::*;
use luakit_common::ipc::IPC_TYPE_scroll;
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
use crate::ipc::ipc_scroll_t;
use crate::ipc_common::ipc::ipc_send_lua;
use crate::log::*;
use crate::web_context_get;
use crate::widgets::common::*;
use crate::widgets::webview::luaH_checkwebview;
use crate::widgets::webview::webview_data_t;
use crate::widgets::*;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn webview_scroll_recv(mut w: *mut widget_t, mut msg: *const ipc_scroll_t) {
    let mut d = (*w).data as *mut webview_data_t;
    if webkit_web_view_get_page_id((*d).view) != (*msg).page_id {
        return;
    }
    match (*msg).subtype as std::ffi::c_uint {
        0 => {
            (*d).doc_w = (*msg).h;
            (*d).doc_h = (*msg).v;
        }
        1 => {
            (*d).win_w = (*msg).h;
            (*d).win_h = (*msg).v;
        }
        2 => {
            (*d).scroll_x = (*msg).h;
            (*d).scroll_y = (*msg).v;
        }
        _ => {}
    };
}
pub unsafe extern "C-unwind" fn luaH_webview_scroll_newindex(mut L: *mut lua_State) -> gint {
    let mut d = (*luaH_checkwebview(L, -(10002 as std::ffi::c_int) - 1 as std::ffi::c_int)).data
        as *mut webview_data_t;
    let mut prop = luaL_checklstring(L, 2 as std::ffi::c_int, std::ptr::null_mut());
    let mut t = l_tokenize(prop);
    if t as std::ffi::c_uint == L_TK_X as std::ffi::c_int as std::ffi::c_uint {
        (*d).scroll_x = luaL_checknumber(L, 3 as std::ffi::c_int) as gint;
    } else if t as std::ffi::c_uint == L_TK_Y as std::ffi::c_int as std::ffi::c_uint {
        (*d).scroll_y = luaL_checknumber(L, 3 as std::ffi::c_int) as gint;
    } else {
        return 0 as std::ffi::c_int;
    }
    lua_pushinteger(L, webkit_web_view_get_page_id((*d).view) as lua_Integer);
    lua_pushinteger(L, (*d).scroll_x as lua_Integer);
    lua_pushinteger(L, (*d).scroll_y as lua_Integer);
    ipc_send_lua(
        (*d).ipc,
        IPC_TYPE_scroll,
        L,
        4 as std::ffi::c_int,
        6 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
pub unsafe extern "C-unwind" fn luaH_webview_scroll_index(mut L: *mut lua_State) -> gint {
    let mut d = (*luaH_checkwebview(L, -(10002 as std::ffi::c_int) - 1 as std::ffi::c_int)).data
        as *mut webview_data_t;
    let mut prop = luaL_checklstring(L, 2 as std::ffi::c_int, std::ptr::null_mut());
    let mut t = l_tokenize(prop);
    match t as std::ffi::c_uint {
        267 => {
            lua_pushnumber(L, (*d).scroll_x as lua_Number);
            return 1 as std::ffi::c_int;
        }
        270 => {
            lua_pushnumber(L, (*d).scroll_y as lua_Number);
            return 1 as std::ffi::c_int;
        }
        268 => {
            lua_pushnumber(L, ((*d).doc_w - (*d).win_w) as lua_Number);
            return 1 as std::ffi::c_int;
        }
        271 => {
            lua_pushnumber(L, ((*d).doc_h - (*d).win_h) as lua_Number);
            return 1 as std::ffi::c_int;
        }
        269 => {
            lua_pushnumber(L, (*d).win_w as lua_Number);
            return 1 as std::ffi::c_int;
        }
        272 => {
            lua_pushnumber(L, (*d).win_h as lua_Number);
            return 1 as std::ffi::c_int;
        }
        _ => return 0 as std::ffi::c_int,
    };
}
pub unsafe extern "C" fn luaH_webview_push_scroll_table(mut L: *mut lua_State) -> gint {
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
    lua_pushcclosure(L, luaH_webview_scroll_index, 1 as std::ffi::c_int);
    lua_rawset(L, -(3 as std::ffi::c_int));
    lua_pushlstring(
        L,
        b"__newindex\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 11]>())
            .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
            .wrapping_sub(1),
    );
    lua_pushvalue(L, 1 as std::ffi::c_int);
    lua_pushcclosure(L, luaH_webview_scroll_newindex, 1 as std::ffi::c_int);
    lua_rawset(L, -(3 as std::ffi::c_int));
    lua_setmetatable(L, -(2 as std::ffi::c_int));
    return 1 as std::ffi::c_int;
}
