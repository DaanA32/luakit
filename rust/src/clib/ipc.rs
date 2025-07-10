use gdk_sys::*;
use glib_sys::*;
use libc::*;
use mlua_sys::*;

use crate::clib::luakit::*;
use crate::common::clib::ipc::*;
use crate::common::clib::luakit::*;
use crate::common::common;
use crate::common::ipc::*;
use crate::common::luaclass::signal_h::*;
use crate::common::luaclass::*;
use crate::common::luah::*;
use crate::common::luaobject::*;
use crate::common::luaserialize::*;
use crate::common::luauniq::*;
use crate::common::tokenize::*;
use crate::globalconf::*;
use crate::ipc::*;
use crate::log::*;
use crate::luah::*;
use crate::web_context::*;
use crate::widgets::webview::*;
use crate::widgets::*;

use crate::gtypes::*;
use webkit2gtk::{ffi::*, glib::gobject_ffi::*};

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ipc_channel_send(mut L: *mut lua_State) -> gint {
    let mut ipc_channel: *mut ipc_channel_t = luaH_check_ipc_channel(L, 1 as std::ffi::c_int);
    let mut page_id: guint64 = 0 as std::ffi::c_int as guint64;
    let mut ipc: *mut ipc_endpoint_t = 0 as *mut ipc_endpoint_t;
    if lua_isuserdata(L, 2 as std::ffi::c_int) != 0 {
        let mut w: *mut widget_t = luaH_checkwebview(L, 2 as std::ffi::c_int);
        page_id = webkit_web_view_get_page_id(g_type_check_instance_cast(
            (*w).widget as *mut GTypeInstance,
            webkit_web_view_get_type(),
        ) as *mut std::ffi::c_void
            as *mut WebKitWebView);
        ipc = webview_get_endpoint(w);
        lua_remove(L, 2 as std::ffi::c_int);
    } else if lua_isnumber(L, 2 as std::ffi::c_int) != 0 {
        page_id = lua_tointeger(L, 2 as std::ffi::c_int) as guint64;
        let mut w_0: *mut widget_t = webview_get_by_id(page_id);
        ipc = webview_get_endpoint(w_0);
        lua_remove(L, 2 as std::ffi::c_int);
    }
    luaL_checklstring(L, 2 as std::ffi::c_int, 0 as *mut size_t);
    lua_pushstring(L, (*ipc_channel).name);
    lua_pushinteger(L, page_id as lua_Integer);
    if !ipc.is_null() {
        ipc_send_lua(
            ipc,
            IPC_TYPE_lua_ipc,
            L,
            2 as std::ffi::c_int,
            lua_gettop(L),
        );
    } else {
        let mut endpoints: *const GPtrArray = ipc_endpoints_get();
        let mut i: std::ffi::c_uint = 0 as std::ffi::c_int as std::ffi::c_uint;
        while i < (*endpoints).len {
            let mut ipc_0: *mut ipc_endpoint_t =
                *((*endpoints).pdata).offset(i as isize) as *mut ipc_endpoint_t;
            ipc_send_lua(
                ipc_0,
                IPC_TYPE_lua_ipc,
                L,
                2 as std::ffi::c_int,
                lua_gettop(L),
            );
            i = i.wrapping_add(1);
            i;
        }
    }
    return 0 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ipc_channel_recv(
    mut L: *mut lua_State,
    mut arg: *const gchar,
    mut arglen: guint,
) {
    let mut top: gint = lua_gettop(L);
    let mut n: std::ffi::c_int = lua_deserialize_range(L, arg as *mut guint8, arglen);
    let mut signame: *const std::ffi::c_char = lua_tolstring(L, -n, 0 as *mut size_t);
    luaH_uniq_get(
        L,
        b"luakit.registry.ipc_channel\0" as *const u8 as *const std::ffi::c_char,
        -(1 as std::ffi::c_int),
    );
    lua_remove(L, -n - 1 as std::ffi::c_int);
    lua_insert(L, -n);
    lua_remove(L, -(1 as std::ffi::c_int));
    if !(lua_type(L, -n + 1 as std::ffi::c_int) == 0 as std::ffi::c_int) {
        luaH_object_emit_signal(
            L,
            -n + 1 as std::ffi::c_int,
            signame,
            n - 2 as std::ffi::c_int,
            0 as std::ffi::c_int,
        );
    }
    lua_settop(L, top);
}
