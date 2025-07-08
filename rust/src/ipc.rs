pub mod ipc_h {
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
    use glib_sys::*;
    use libc::pid_t;
    unsafe extern "C" {
        pub fn ipc_endpoint_new(name: *const gchar) -> *mut ipc_endpoint_t;
        pub fn ipc_endpoint_connect_to_socket(ipc: *mut ipc_endpoint_t, sock: std::ffi::c_int);
        pub fn ipc_send(
            ipc: *mut ipc_endpoint_t,
            header: *const ipc_header_t,
            data: *const std::ffi::c_void,
        );
    }
}

use crate::{
    clib::{web_module::web_module_load_modules_on_endpoint, widget::widget_t},
    common::common,
    globalconf::globalconf,
    gtypes::*,
    log::{_log, LOG_LEVEL_debug, LOG_LEVEL_fatal, LOG_LEVEL_verbose},
    web_context::web_context_get,
};
use glib_sys::{
    G_FILE_TEST_EXISTS, GCond, GFunc, GIOChannel, GMutex, GPtrArray, GQueue, GVariant,
    g_build_filename, g_cond_signal, g_cond_wait, g_file_test, g_free, g_get_current_dir,
    g_get_tmp_dir, g_mutex_lock, g_mutex_unlock, g_ptr_array_foreach, g_random_int_range,
    g_strdup_printf, g_thread_new, g_unlink, g_variant_new, gboolean, gpointer,
};
use libc::{
    __errno_location, SOCK_STREAM, accept, access, atexit, bind, c_int, getpid, listen, memset,
    sa_family_t, size_t, sockaddr, sockaddr_un, socket, socklen_t, strcpy, strerror, strlen,
    unlink,
};
use lua::ffi::{lua_getfield, lua_settop, lua_tolstring};
use webkit2gtk::{
    ffi::{
        WebKitWebContext, webkit_web_context_set_web_extensions_directory,
        webkit_web_context_set_web_extensions_initialization_user_data,
    },
    glib::gobject_ffi::{G_CONNECT_DEFAULT, GCallback, GObject, g_signal_connect_data},
};

pub use self::ipc_h::{
    _ipc_endpoint_t, _ipc_header_t, _ipc_lua_ipc_t, _ipc_page_created_t, _ipc_recv_state_t,
    _ipc_scroll_t, IPC_ENDPOINT_CONNECTED, IPC_ENDPOINT_DISCONNECTED, IPC_ENDPOINT_FREED,
    IPC_SCROLL_TYPE_docresize, IPC_SCROLL_TYPE_scroll, IPC_SCROLL_TYPE_winresize, IPC_TYPE_crash,
    IPC_TYPE_eval_js, IPC_TYPE_extension_init, IPC_TYPE_log, IPC_TYPE_lua_ipc,
    IPC_TYPE_lua_require_module, IPC_TYPE_page_created, IPC_TYPE_scroll,
    ipc_endpoint_connect_to_socket, ipc_endpoint_new, ipc_endpoint_status_t, ipc_endpoint_t,
    ipc_header_t, ipc_lua_ipc_t, ipc_page_created_t, ipc_recv_state_t, ipc_scroll_subtype_t,
    ipc_scroll_t, ipc_send, ipc_type_t,
};
unsafe extern "C" {
    pub fn webview_scroll_recv(d: *mut std::ffi::c_void, ipc: *const ipc_scroll_t);
    pub fn run_javascript_finished(msg: *const guint8, length: guint);
}
static mut socket_path: *mut std::ffi::c_char =
    0 as *const std::ffi::c_char as *mut std::ffi::c_char;
#[unsafe(no_mangle)]
pub static mut socket_path_lock: *mut GMutex = std::ptr::null_mut();
#[unsafe(no_mangle)]
pub static mut socket_path_cond: *mut GCond = std::ptr::null_mut();
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ipc_recv_lua_require_module(
    mut ipc: *mut ipc_endpoint_t,
    UNUSED_msg: gpointer,
    mut UNUSED_length: guint,
) {
    _log(
        LOG_LEVEL_fatal,
        b"ipc.c\0" as *const u8 as *const std::ffi::c_char,
        b"process '%s': should never receive message of type %s\0" as *const u8
            as *const std::ffi::c_char,
        (*ipc).name,
        b"lua_require_module\0" as *const u8 as *const std::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ipc_recv_web_extension_loaded(
    mut ipc: *mut ipc_endpoint_t,
    UNUSED_msg: gpointer,
    mut UNUSED_length: guint,
) {
    _log(
        LOG_LEVEL_fatal,
        b"ipc.c\0" as *const u8 as *const std::ffi::c_char,
        b"process '%s': should never receive message of type %s\0" as *const u8
            as *const std::ffi::c_char,
        (*ipc).name,
        b"web_extension_loaded\0" as *const u8 as *const std::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ipc_recv_crash(
    mut ipc: *mut ipc_endpoint_t,
    UNUSED_msg: gpointer,
    mut UNUSED_length: guint,
) {
    _log(
        LOG_LEVEL_fatal,
        b"ipc.c\0" as *const u8 as *const std::ffi::c_char,
        b"process '%s': should never receive message of type %s\0" as *const u8
            as *const std::ffi::c_char,
        (*ipc).name,
        b"crash\0" as *const u8 as *const std::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ipc_recv_extension_init(
    mut ipc: *mut ipc_endpoint_t,
    UNUSED_msg: gpointer,
    mut UNUSED_length: guint,
) {
    web_module_load_modules_on_endpoint(ipc);
    let mut header: ipc_header_t = {
        let mut init = _ipc_header_t {
            length: 0 as std::ffi::c_int as guint,
            type_0: IPC_TYPE_extension_init,
        };
        init
    };
    ipc_send(ipc, &mut header, 0 as *const std::ffi::c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ipc_recv_lua_ipc(
    mut UNUSED_ipc: *mut ipc_endpoint_t,
    mut msg: *const ipc_lua_ipc_t,
    mut length: guint,
) {
    ipc_channel_recv(common.L, ((*msg).arg).as_ptr(), length);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ipc_recv_scroll(
    mut UNUSED_ipc: *mut ipc_endpoint_t,
    mut msg: *mut ipc_scroll_t,
    mut UNUSED_length: guint,
) {
    g_ptr_array_foreach(
        globalconf.webviews,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut std::ffi::c_void, *const ipc_scroll_t) -> ()>,
            GFunc,
        >(Some(
            webview_scroll_recv
                as unsafe extern "C" fn(*mut std::ffi::c_void, *const ipc_scroll_t) -> (),
        )),
        msg as gpointer,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ipc_recv_eval_js(
    mut UNUSED_ipc: *mut ipc_endpoint_t,
    mut msg: *const guint8,
    mut length: guint,
) {
    run_javascript_finished(msg, length);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ipc_recv_page_created(
    mut ipc: *mut ipc_endpoint_t,
    mut msg: *const ipc_page_created_t,
    mut UNUSED_length: guint,
) {
    let mut w: *mut widget_t = webview_get_by_id((*msg).page_id);
    if w.is_null() {
        return;
    }
    webview_connect_to_endpoint(w, ipc);
    webview_set_web_process_id(w, (*msg).pid);
}
unsafe extern "C" fn build_socket_path() -> *mut gchar {
    let mut socket_name: *mut gchar = 0 as *mut gchar;
    let mut socket_path_0: *mut gchar = 0 as *mut gchar;
    let mut suffix: [std::ffi::c_char; 11] = [
        0 as std::ffi::c_int as std::ffi::c_char,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    loop {
        let mut i: std::ffi::c_uint = 0 as std::ffi::c_int as std::ffi::c_uint;
        while (i as std::ffi::c_ulong)
            < (::core::mem::size_of::<[std::ffi::c_char; 11]>() as std::ffi::c_ulong)
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong)
        {
            let mut c: std::ffi::c_int = g_random_int_range(
                0 as std::ffi::c_int,
                10 as std::ffi::c_int + 26 as std::ffi::c_int + 26 as std::ffi::c_int,
            );
            let mut base: std::ffi::c_int = '0' as i32;
            if c >= 10 as std::ffi::c_int {
                base = 'A' as i32;
                c -= 10 as std::ffi::c_int;
            }
            if c >= 26 as std::ffi::c_int {
                base = 'a' as i32;
                c -= 26 as std::ffi::c_int;
            }
            suffix[i as usize] = (base + c) as std::ffi::c_char;
            i = i.wrapping_add(1);
            i;
        }
        socket_name = g_strdup_printf(
            b"luakit-ipc-%d-%s\0" as *const u8 as *const std::ffi::c_char,
            getpid(),
            suffix.as_mut_ptr(),
        );
        socket_path_0 = g_build_filename(g_get_tmp_dir(), socket_name, 0 as *mut std::ffi::c_void);
        g_free(socket_name as gpointer);
        if !(g_file_test(socket_path_0, G_FILE_TEST_EXISTS) != 0) {
            break;
        }
        g_free(socket_path_0 as gpointer);
    }
    return socket_path_0;
}
unsafe extern "C" fn web_extension_connect_thread(mut UNUSED_data: gpointer) -> gpointer {
    let mut path: *mut gchar = build_socket_path();
    let mut sock: std::ffi::c_int = 0;
    sock = socket(1 as std::ffi::c_int, SOCK_STREAM, 0 as std::ffi::c_int);
    if sock == -(1 as std::ffi::c_int) {
        _log(
            LOG_LEVEL_fatal,
            b"ipc.c\0" as *const u8 as *const std::ffi::c_char,
            b"Error calling socket(): %s\0" as *const u8 as *const std::ffi::c_char,
            strerror(*__errno_location()),
        );
    }
    let mut local: sockaddr_un = sockaddr_un {
        sun_family: 0,
        sun_path: [0; 108],
    };
    memset(
        &mut local as *mut sockaddr_un as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        ::core::mem::size_of::<sockaddr_un>(),
    );
    local.sun_family = 1 as std::ffi::c_int as sa_family_t;
    strcpy((local.sun_path).as_mut_ptr(), path);
    let mut len: std::ffi::c_int = (2 as std::ffi::c_ulong)
        .wrapping_add(strlen((local.sun_path).as_mut_ptr()) as u64)
        as std::ffi::c_int;
    unlink((local.sun_path).as_mut_ptr());
    if bind(
        sock,
        &mut local as *mut sockaddr_un as *mut sockaddr,
        len as socklen_t,
    ) == -(1 as std::ffi::c_int)
    {
        _log(
            LOG_LEVEL_fatal,
            b"ipc.c\0" as *const u8 as *const std::ffi::c_char,
            b"Error calling bind() on socket %s: %s\0" as *const u8 as *const std::ffi::c_char,
            path,
            strerror(*__errno_location()),
        );
    }
    if listen(sock, 5 as std::ffi::c_int) == -(1 as std::ffi::c_int) {
        _log(
            LOG_LEVEL_fatal,
            b"ipc.c\0" as *const u8 as *const std::ffi::c_char,
            b"Error calling listen() on socket %s: %s\0" as *const u8 as *const std::ffi::c_char,
            path,
            strerror(*__errno_location()),
        );
    }
    g_mutex_lock(socket_path_lock);
    socket_path = path;
    g_cond_signal(socket_path_cond);
    g_mutex_unlock(socket_path_lock);
    while 0 as std::ffi::c_int == 0 {
        _log(
            LOG_LEVEL_debug,
            b"ipc.c\0" as *const u8 as *const std::ffi::c_char,
            b"Waiting for a connection...\0" as *const u8 as *const std::ffi::c_char,
        );
        let mut web_socket: std::ffi::c_int = 0;
        let mut remote: sockaddr_un = sockaddr_un {
            sun_family: 0,
            sun_path: [0; 108],
        };
        let mut size: socklen_t =
            ::core::mem::size_of::<sockaddr_un>() as std::ffi::c_ulong as socklen_t;
        web_socket = accept(
            sock,
            &mut remote as *mut sockaddr_un as *mut sockaddr,
            &mut size,
        );
        if web_socket == -(1 as std::ffi::c_int) {
            _log(
                LOG_LEVEL_fatal,
                b"ipc.c\0" as *const u8 as *const std::ffi::c_char,
                b"Error calling accept(): %s\0" as *const u8 as *const std::ffi::c_char,
                strerror(*__errno_location()),
            );
        }
        let mut ipc: *mut ipc_endpoint_t =
            ipc_endpoint_new(b"UI\0" as *const u8 as *const std::ffi::c_char);
        ipc_endpoint_connect_to_socket(ipc, web_socket);
    }
    return 0 as *mut std::ffi::c_void;
}
unsafe extern "C" fn initialize_web_extensions_cb(
    mut context: *mut WebKitWebContext,
    mut UNUSED_data: gpointer,
) {
    let mut dirs: [*mut std::ffi::c_char; 2] = [
        g_get_current_dir(),
        b"/usr/local/lib/luakit\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char,
    ];
    let mut dir: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut i: std::ffi::c_uint = 0 as std::ffi::c_int as std::ffi::c_uint;
    while dir.is_null()
        && (i as std::ffi::c_ulong)
            < (::core::mem::size_of::<[*mut std::ffi::c_char; 2]>() as std::ffi::c_ulong)
                .wrapping_div(::core::mem::size_of::<*mut std::ffi::c_char>() as std::ffi::c_ulong)
    {
        let mut extension_file: *mut std::ffi::c_char = g_build_filename(
            dirs[i as usize],
            b"luakit.so\0" as *const u8 as *const std::ffi::c_char,
            0 as *mut std::ffi::c_void,
        );
        _log(
            LOG_LEVEL_verbose,
            b"ipc.c\0" as *const u8 as *const std::ffi::c_char,
            b"checking for luakit extension at '%s'\0" as *const u8 as *const std::ffi::c_char,
            dirs[i as usize],
        );
        if access(extension_file, 4 as std::ffi::c_int) == 0 {
            dir = dirs[i as usize];
        }
        g_free(extension_file as gpointer);
        i = i.wrapping_add(1);
        i;
    }
    if !dir.is_null() {
        _log(
            LOG_LEVEL_verbose,
            b"ipc.c\0" as *const u8 as *const std::ffi::c_char,
            b"found luakit extension at '%s'\0" as *const u8 as *const std::ffi::c_char,
            dir,
        );
    } else {
        _log(
            LOG_LEVEL_fatal,
            b"ipc.c\0" as *const u8 as *const std::ffi::c_char,
            b"cannot find luakit extension 'luakit.so'\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    let mut path: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    g_mutex_lock(socket_path_lock);
    while socket_path.is_null() {
        g_cond_wait(socket_path_cond, socket_path_lock);
    }
    path = socket_path;
    g_mutex_unlock(socket_path_lock);
    lua_getfield(
        common.L,
        -(10002 as std::ffi::c_int),
        b"package\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_getfield(
        common.L,
        -(1 as std::ffi::c_int),
        b"path\0" as *const u8 as *const std::ffi::c_char,
    );
    let mut package_path: *const std::ffi::c_char =
        lua_tolstring(common.L, -(1 as std::ffi::c_int), 0 as *mut size_t);
    lua_getfield(
        common.L,
        -(2 as std::ffi::c_int),
        b"cpath\0" as *const u8 as *const std::ffi::c_char,
    );
    let mut package_cpath: *const std::ffi::c_char =
        lua_tolstring(common.L, -(1 as std::ffi::c_int), 0 as *mut size_t);
    lua_settop(common.L, -(3 as std::ffi::c_int) - 1 as std::ffi::c_int);
    let mut payload: *mut GVariant = g_variant_new(
        b"(sss)\0" as *const u8 as *const std::ffi::c_char,
        path,
        package_path,
        package_cpath,
    );
    webkit_web_context_set_web_extensions_initialization_user_data(context, payload);
    webkit_web_context_set_web_extensions_directory(context, dir);
    g_free(dirs[0 as std::ffi::c_int as usize] as gpointer);
}
#[unsafe(no_mangle)]
pub extern "C" fn ipc_remove_socket_file() -> () {
    unsafe {
        g_mutex_lock(socket_path_lock);
        g_unlink(socket_path);
        g_free(socket_path as gpointer);
        socket_path = 0 as *mut std::ffi::c_char;
        g_mutex_unlock(socket_path_lock);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ipc_init() {
    g_thread_new(
        b"accept_thread\0" as *const u8 as *const std::ffi::c_char,
        Some(web_extension_connect_thread as unsafe extern "C" fn(gpointer) -> gpointer),
        0 as *mut std::ffi::c_void,
    );
    g_signal_connect_data(
        web_context_get() as *mut GObject,
        b"initialize-web-extensions\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut WebKitWebContext, gpointer) -> ()>,
            GCallback,
        >(Some(
            initialize_web_extensions_cb
                as unsafe extern "C" fn(*mut WebKitWebContext, gpointer) -> (),
        )),
        0 as *mut std::ffi::c_void,
        None,
        G_CONNECT_DEFAULT,
    );
    atexit(ipc_remove_socket_file);
}
