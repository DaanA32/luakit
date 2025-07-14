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

use crate::clib::download::luaH_download_push;
use crate::clib::luakit::luakit_lib_get_luakit_class;
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

pub unsafe extern "C" fn download_start_cb(
    mut UNUSED_c: *mut WebKitWebContext,
    mut dl: *mut WebKitDownload,
    mut UNUSED_user_data: gpointer,
) -> gboolean {
    let mut dl_view = webkit_download_get_web_view(dl);
    let mut w = if !dl_view.is_null() {
        g_object_get_data(
            g_type_check_instance_cast(
                dl_view as *mut GTypeInstance,
                ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
            ) as *mut std::ffi::c_void as *mut GObject,
            GOBJECT_LUAKIT_WIDGET_DATA_KEY.as_ptr(),
        ) as *mut widget_t
    } else {
        std::ptr::null()
    };
    let mut L = common.L;
    let mut top = lua_gettop(L);
    luaH_download_push(L, dl);
    if !w.is_null() {
        luaH_object_push(L, (*w).ref_0);
    } else {
        lua_pushnil(L);
    }
    let mut luakit_class = luakit_lib_get_luakit_class();
    let mut ret = luaH_class_emit_signal(
        L,
        luakit_class,
        b"download-start\0" as *const u8 as *const std::ffi::c_char,
        2 as std::ffi::c_int,
        1 as std::ffi::c_int,
    );
    let mut handled = (ret != 0 && lua_toboolean(L, 2 as std::ffi::c_int) != 0) as std::ffi::c_int;
    lua_settop(L, top);
    return handled;
}
