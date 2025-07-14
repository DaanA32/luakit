use glib_sys::*;
use mlua::ffi::*;
use webkit2gtk_sys::*;

use crate::common::common;
use crate::common::luaobject::*;
use crate::gtypes::gint;
use crate::widgets::webview::luaH_checkwebview;
use crate::widgets::webview::webview_data_t;
use crate::widgets::widget_t;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn inspector_open_window_cb(
    mut UNUSED_inspector: *mut WebKitWebInspector,
    mut w: *mut widget_t,
) -> gboolean {
    let mut L = common.L;
    luaH_object_push(L, (*w).ref_0);
    let mut nret = luaH_object_emit_signal(
        L,
        -(1 as std::ffi::c_int),
        b"create-inspector-window\0" as *const u8 as *const std::ffi::c_char,
        0 as std::ffi::c_int,
        1 as std::ffi::c_int,
    );
    let mut ret = (nret != 0 && lua_toboolean(L, -(1 as std::ffi::c_int)) != 0) as std::ffi::c_int;
    lua_settop(L, -(1 as std::ffi::c_int + nret) - 1 as std::ffi::c_int);
    return ret;
}
pub unsafe extern "C" fn inspector_show_window_cb(
    mut UNUSED_inspector: *mut WebKitWebInspector,
    mut w: *mut widget_t,
) -> gboolean {
    let mut L = common.L;
    luaH_object_push(L, (*w).ref_0);
    let mut nret = luaH_object_emit_signal(
        L,
        -(1 as std::ffi::c_int),
        b"show-inspector\0" as *const u8 as *const std::ffi::c_char,
        0 as std::ffi::c_int,
        1 as std::ffi::c_int,
    );
    let mut ret = (nret != 0 && lua_toboolean(L, -(1 as std::ffi::c_int)) != 0) as std::ffi::c_int;
    lua_settop(L, -(1 as std::ffi::c_int + nret) - 1 as std::ffi::c_int);
    return ret;
}
pub unsafe extern "C" fn inspector_close_window_cb(
    mut UNUSED_inspector: *mut WebKitWebInspector,
    mut w: *mut widget_t,
) -> gboolean {
    let mut L = common.L;
    luaH_object_push(L, (*w).ref_0);
    let mut d = (*w).data as *mut webview_data_t;
    lua_pushnil(L);
    (*d).inspector_open = GFALSE;
    let mut nret = luaH_object_emit_signal(
        L,
        -(2 as std::ffi::c_int),
        b"close-inspector\0" as *const u8 as *const std::ffi::c_char,
        1 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    let mut ret = (nret != 0 && lua_toboolean(L, -(1 as std::ffi::c_int)) != 0) as std::ffi::c_int;
    lua_settop(L, -(1 as std::ffi::c_int + nret) - 1 as std::ffi::c_int);
    return ret;
}
pub unsafe extern "C" fn inspector_attach_window_cb(
    mut UNUSED_inspector: *mut WebKitWebInspector,
    mut w: *mut widget_t,
) -> gboolean {
    let mut L = common.L;
    let mut d = (*w).data as *mut webview_data_t;
    (*d).inspector_open = GTRUE;
    luaH_object_push(L, (*w).ref_0);
    let mut nret = luaH_object_emit_signal(
        L,
        -(1 as std::ffi::c_int),
        b"attach-inspector\0" as *const u8 as *const std::ffi::c_char,
        0 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    let mut ret = (nret != 0 && lua_toboolean(L, -(1 as std::ffi::c_int)) != 0) as std::ffi::c_int;
    lua_settop(L, -(1 as std::ffi::c_int + nret) - 1 as std::ffi::c_int);
    return ret;
}
pub unsafe extern "C" fn inspector_detach_window_cb(
    mut UNUSED_inspector: *mut WebKitWebInspector,
    mut w: *mut widget_t,
) -> gboolean {
    let mut L = common.L;
    luaH_object_push(L, (*w).ref_0);
    let mut nret = luaH_object_emit_signal(
        L,
        -(1 as std::ffi::c_int),
        b"detach-inspector\0" as *const u8 as *const std::ffi::c_char,
        0 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    let mut ret = (nret != 0 && lua_toboolean(L, -(1 as std::ffi::c_int)) != 0) as std::ffi::c_int;
    lua_settop(L, -(1 as std::ffi::c_int + nret) - 1 as std::ffi::c_int);
    return ret;
}
pub unsafe extern "C-unwind" fn luaH_webview_show_inspector(mut L: *mut lua_State) -> gint {
    let mut d = (*luaH_checkwebview(L, 1 as std::ffi::c_int)).data as *mut webview_data_t;
    webkit_web_inspector_show((*d).inspector);
    return 0 as std::ffi::c_int;
}
pub unsafe extern "C-unwind" fn luaH_webview_close_inspector(mut L: *mut lua_State) -> gint {
    let mut d = (*luaH_checkwebview(L, 1 as std::ffi::c_int)).data as *mut webview_data_t;
    webkit_web_inspector_close((*d).inspector);
    return 0 as std::ffi::c_int;
}
