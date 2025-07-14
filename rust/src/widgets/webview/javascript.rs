use cairo_sys::*;
use gdk_pixbuf_sys::*;
use gdk_sys::*;
use gio_sys::*;
use glib_sys::*;
use gobject_sys::*;
use gtk_sys::*;
use libc::*;
use luakit_common::common::lualib::luaH_dofunction;
use luakit_common::common::util::luaH_callerinfo;
use luakit_common::ipc::IPC_TYPE_eval_js;
use mlua::ffi::*;
use pango_sys::*;
use webkit2gtk_sys::*;

use crate::clib::widget::widget_set_css_properties;
use crate::common::common;
use crate::common::luaclass::*;
use crate::common::luah::*;
use crate::common::luaobject::*;
use crate::common::luaserialize::lua_deserialize_range;
use crate::common::resource::*;
use crate::common::tokenize::*;
use crate::gtypes::*;
use crate::ipc_common::ipc::ipc_send_lua;
use crate::log::*;
use crate::web_context_get;
use crate::widgets::common::*;
use crate::widgets::webview::luaH_checkwebview;
use crate::widgets::webview::webview_data_t;
use crate::widgets::webview::webview_get_by_id;
use crate::widgets::*;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn run_javascript_finished(mut msg: *const guint8, mut length: guint) {
    let mut L = common.L;
    let mut top = lua_gettop(L);
    let mut n = lua_deserialize_range(L, msg, length);
    let mut __n1 = n as gint64;
    let mut __n2 = 2 as std::ffi::c_int as gint64;
    if !(__n1 >= __n2) {
        g_assertion_message_cmpint(
            G_LOG_DOMAIN as *const std::ffi::c_char,
            b"./widgets/webview/javascript.c\0" as *const u8 as *const std::ffi::c_char,
            33 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 24], &[std::ffi::c_char; 24]>(
                b"run_javascript_finished\0",
            ))
            .as_ptr(),
            b"n >= 2\0" as *const u8 as *const std::ffi::c_char,
            __n1 as guint64,
            b">=\0" as *const u8 as *const std::ffi::c_char,
            __n2 as guint64,
            'i' as i32 as std::ffi::c_char,
        );
    }
    let mut __n1_0 = n as gint64;
    let mut __n2_0 = 4 as std::ffi::c_int as gint64;
    if !(__n1_0 <= __n2_0) {
        g_assertion_message_cmpint(
            G_LOG_DOMAIN as *const std::ffi::c_char,
            b"./widgets/webview/javascript.c\0" as *const u8 as *const std::ffi::c_char,
            34 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 24], &[std::ffi::c_char; 24]>(
                b"run_javascript_finished\0",
            ))
            .as_ptr(),
            b"n <= 4\0" as *const u8 as *const std::ffi::c_char,
            __n1_0 as guint64,
            b"<=\0" as *const u8 as *const std::ffi::c_char,
            __n2_0 as guint64,
            'i' as i32 as std::ffi::c_char,
        );
    }
    let mut w = webview_get_by_id(lua_tointeger(L, -n) as guint64);
    lua_remove(L, -n);
    n -= 1;
    n;
    let mut cb = lua_touserdata(L, -n);
    if cb.is_null() {
        _log(
            LOG_LEVEL_warn,
            b"./widgets/webview/javascript.c\0" as *const u8 as *const std::ffi::c_char,
            "javascript finshed called on non object",
        );
        return;
    }
    lua_remove(L, -n);
    n -= 1;
    n;
    if n == 2 as std::ffi::c_int {
        if lua_type(L, -(2 as std::ffi::c_int)) == 0 as std::ffi::c_int {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN as *const std::ffi::c_char,
                b"./widgets/webview/javascript.c\0" as *const u8 as *const std::ffi::c_char,
                49 as std::ffi::c_int,
                (*::core::mem::transmute::<&[u8; 24], &[std::ffi::c_char; 24]>(
                    b"run_javascript_finished\0",
                ))
                .as_ptr(),
                b"lua_isnil(L, -2)\0" as *const u8 as *const std::ffi::c_char,
            );
        }
        if lua_isstring(L, -(1 as std::ffi::c_int)) != 0 {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN as *const std::ffi::c_char,
                b"./widgets/webview/javascript.c\0" as *const u8 as *const std::ffi::c_char,
                50 as std::ffi::c_int,
                (*::core::mem::transmute::<&[u8; 24], &[std::ffi::c_char; 24]>(
                    b"run_javascript_finished\0",
                ))
                .as_ptr(),
                b"lua_isstring(L, -1)\0" as *const u8 as *const std::ffi::c_char,
            );
        }
    }
    if n >= 1 as std::ffi::c_int && !cb.is_null() && !w.is_null() {
        luaH_object_push(L, cb);
        luaH_dofunction(L, n, 0 as std::ffi::c_int);
    }
    if !w.is_null() && !cb.is_null() {
        g_signal_handlers_disconnect_matched(
            (*w).widget as *mut GObject,
            G_SIGNAL_MATCH_DATA,
            0 as std::ffi::c_int as guint,
            0 as std::ffi::c_int as GQuark,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            cb,
        );
        luaH_object_unref(L, cb);
    }
    lua_settop(L, top);
}
pub unsafe extern "C" fn run_javascript_webview_closed(
    mut UNUSED_view: *mut WebKitWebView,
    mut cb: gpointer,
) {
    luaH_object_unref(common.L, cb);
}
pub unsafe extern "C-unwind" fn luaH_webview_eval_js(mut L: *mut lua_State) -> gint {
    let mut cb = std::ptr::null_mut();
    let mut d = (*luaH_checkwebview(L, 1 as std::ffi::c_int)).data as *mut webview_data_t;
    let mut script = luaL_checklstring(L, 2 as std::ffi::c_int, std::ptr::null_mut());
    let mut usr_source = std::ptr::null();
    let mut source = std::ptr::null_mut();
    let mut no_return = false;
    if !(lua_type(L, 3 as std::ffi::c_int) == LUA_TTABLE) {
        luaL_typerror(
            L,
            3 as std::ffi::c_int,
            b"table\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    let mut top = lua_gettop(L);
    if luaH_rawfield(
        L,
        3 as std::ffi::c_int,
        b"source\0" as *const u8 as *const std::ffi::c_char,
    ) != 0
        && lua_isstring(L, -(1 as std::ffi::c_int)) != 0
    {
        usr_source = lua_tolstring(L, -(1 as std::ffi::c_int), std::ptr::null_mut());
    }
    if luaH_rawfield(
        L,
        3 as std::ffi::c_int,
        b"no_return\0" as *const u8 as *const std::ffi::c_char,
    ) != 0
    {
        no_return = lua_toboolean(L, -(1 as std::ffi::c_int)) != 0;
    }
    if luaH_rawfield(
        L,
        3 as std::ffi::c_int,
        b"callback\0" as *const u8 as *const std::ffi::c_char,
    ) != 0
    {
        if !(lua_type(L, -(1 as std::ffi::c_int)) == LUA_TFUNCTION) {
            luaL_typerror(
                L,
                -(1 as std::ffi::c_int),
                b"function\0" as *const u8 as *const std::ffi::c_char,
            );
        }
        cb = luaH_object_ref(L, -(1 as std::ffi::c_int));
    }
    lua_settop(L, top);
    if usr_source.is_null() {
        source = luaH_callerinfo(L);
    }
    lua_pushboolean(L, no_return as std::ffi::c_int);
    lua_pushstring(L, script);
    lua_pushstring(
        L,
        if !usr_source.is_null() {
            g_strdup(usr_source)
        } else {
            source
        },
    );
    lua_pushinteger(L, webkit_web_view_get_page_id((*d).view) as lua_Integer);
    lua_pushlightuserdata(L, cb);
    ipc_send_lua(
        (*d).ipc,
        IPC_TYPE_eval_js,
        L,
        -(5 as std::ffi::c_int),
        -(1 as std::ffi::c_int),
    );
    lua_settop(L, -(5 as std::ffi::c_int) - 1 as std::ffi::c_int);
    if !cb.is_null() {
        g_signal_connect_data(
            (*d).view as *mut GObject,
            b"destroy\0" as *const u8 as *const std::ffi::c_char,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut WebKitWebView, gpointer) -> ()>,
                GCallback,
            >(Some(
                run_javascript_webview_closed
                    as unsafe extern "C" fn(*mut WebKitWebView, gpointer) -> (),
            )),
            cb,
            None,
            G_CONNECT_DEFAULT,
        );
    }
    return GFALSE;
}
