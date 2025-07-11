pub type ipc_type_t = std::ffi::c_uint;
pub const IPC_TYPE_crash: ipc_type_t = 128;
pub const IPC_TYPE_page_created: ipc_type_t = 64;
pub const IPC_TYPE_log: ipc_type_t = 32;
pub const IPC_TYPE_eval_js: ipc_type_t = 16;
pub const IPC_TYPE_extension_init: ipc_type_t = 8;
pub const IPC_TYPE_scroll: ipc_type_t = 4;
pub const IPC_TYPE_lua_ipc: ipc_type_t = 2;
pub const IPC_TYPE_lua_require_module: ipc_type_t = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ipc_header_t {
    pub length: guint,
    pub type_0: ipc_type_t,
}
pub type ipc_header_t = _ipc_header_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ipc_lua_ipc_t {
    pub arg: [gchar; 0],
}
pub type ipc_lua_ipc_t = _ipc_lua_ipc_t;
pub type ipc_scroll_subtype_t = std::ffi::c_uint;
pub const IPC_SCROLL_TYPE_scroll: ipc_scroll_subtype_t = 2;
pub const IPC_SCROLL_TYPE_winresize: ipc_scroll_subtype_t = 1;
pub const IPC_SCROLL_TYPE_docresize: ipc_scroll_subtype_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ipc_scroll_t {
    pub h: gint,
    pub v: gint,
    pub page_id: guint64,
    pub subtype: ipc_scroll_subtype_t,
}
pub type ipc_scroll_t = _ipc_scroll_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ipc_page_created_t {
    pub page_id: guint64,
    pub pid: pid_t,
}
pub type ipc_page_created_t = _ipc_page_created_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ipc_recv_state_t {
    pub watch_in_id: guint,
    pub watch_hup_id: guint,
    pub queued_ipcs: *mut GPtrArray,
    pub hdr: ipc_header_t,
    pub payload: gpointer,
    pub bytes_read: gsize,
    pub hdr_done: gboolean,
}
pub type ipc_recv_state_t = _ipc_recv_state_t;
pub type ipc_endpoint_status_t = std::ffi::c_uint;
pub const IPC_ENDPOINT_FREED: ipc_endpoint_status_t = 2;
pub const IPC_ENDPOINT_CONNECTED: ipc_endpoint_status_t = 1;
pub const IPC_ENDPOINT_DISCONNECTED: ipc_endpoint_status_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ipc_endpoint_t {
    pub name: *mut gchar,
    pub status: ipc_endpoint_status_t,
    pub channel: *mut GIOChannel,
    pub queue: *mut GQueue,
    pub recv_state: ipc_recv_state_t,
    pub refcount: gint,
    pub creation_notified: gboolean,
}
pub type ipc_endpoint_t = _ipc_endpoint_t;
use crate::gtypes::{gchar, gint, gsize, guint, guint64};
use glib_sys::{
    G_FILE_TEST_EXISTS, GCond, GFunc, GIOChannel, GMutex, GPtrArray, GQueue, GVariant,
    g_build_filename, g_cond_signal, g_cond_wait, g_file_test, g_free, g_get_current_dir,
    g_get_tmp_dir, g_mutex_lock, g_mutex_unlock, g_ptr_array_foreach, g_random_int_range,
    g_strdup_printf, g_thread_new, g_unlink, g_variant_new, gboolean, gpointer,
};
use libc::pid_t;
unsafe extern "C" {
    pub fn ipc_endpoint_connect_to_socket(ipc: *mut ipc_endpoint_t, sock: std::ffi::c_int);
    pub fn ipc_send(
        ipc: *mut ipc_endpoint_t,
        header: *const ipc_header_t,
        data: *const std::ffi::c_void,
    );
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct queued_ipc_t {
    pub header: ipc_header_t,
    pub ipc: *mut ipc_endpoint_t,
    pub payload: [std::ffi::c_char; 0],
}

pub unsafe extern "C" fn ipc_type_name(mut type_0: ipc_type_t) -> *const std::ffi::c_char {
    match type_0 as std::ffi::c_uint {
        1 => return b"lua_require_module\0" as *const u8 as *const std::ffi::c_char,
        2 => return b"lua_ipc\0" as *const u8 as *const std::ffi::c_char,
        4 => return b"scroll\0" as *const u8 as *const std::ffi::c_char,
        8 => return b"extension_init\0" as *const u8 as *const std::ffi::c_char,
        16 => return b"eval_js\0" as *const u8 as *const std::ffi::c_char,
        32 => return b"log\0" as *const u8 as *const std::ffi::c_char,
        64 => return b"page_created\0" as *const u8 as *const std::ffi::c_char,
        128 => return b"crash\0" as *const u8 as *const std::ffi::c_char,
        _ => return b"UNKNOWN\0" as *const u8 as *const std::ffi::c_char,
    };
}
