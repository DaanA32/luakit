use gdk_sys::*;
use glib_sys::*;
use libc::getenv;
use mlua_sys::*;

use crate::clib::luakit::*;
use crate::clib::msg::*;
use crate::clib::soup::*;
use crate::clib::sqlite3::*;
use crate::clib::stylesheet::*;
use crate::clib::web_module::*;
use crate::clib::widget::*;
use crate::common::luaclass::luaH_typename;
use crate::common::luah::*;
use crate::common::luaobject::*;
use crate::common::luautil::*;
use crate::common::util::*;
use crate::common::*;
use crate::globalconf::*;
use crate::gtypes::*;
use crate::ipc::*;
use crate::log::*;

pub mod ipc_h {
    use gdk_sys::*;
    use glib_sys::*;
    use libc::getenv;
    use mlua_sys::*;

    use crate::clib::luakit::*;
    use crate::clib::msg::*;
    use crate::clib::soup::*;
    use crate::clib::sqlite3::*;
    use crate::clib::stylesheet::*;
    use crate::clib::web_module::*;
    use crate::clib::widget::*;
    use crate::common::luaclass::luaH_typename;
    use crate::common::luah::*;
    use crate::common::luaobject::*;
    use crate::common::luautil::*;
    use crate::common::util::*;
    use crate::common::*;
    use crate::globalconf::*;
    use crate::gtypes::*;
    use crate::log::*;

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
    #[inline]
    pub unsafe extern "C-unwind" fn ipc_type_name(
        mut type_0: ipc_type_t,
    ) -> *const std::ffi::c_char {
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
}
pub use self::ipc_h::{
    _ipc_endpoint_t, _ipc_header_t, _ipc_recv_state_t, IPC_ENDPOINT_CONNECTED,
    IPC_ENDPOINT_DISCONNECTED, IPC_ENDPOINT_FREED, IPC_TYPE_crash, IPC_TYPE_eval_js,
    IPC_TYPE_extension_init, IPC_TYPE_log, IPC_TYPE_lua_ipc, IPC_TYPE_lua_require_module,
    IPC_TYPE_page_created, IPC_TYPE_scroll, ipc_endpoint_status_t, ipc_endpoint_t, ipc_header_t,
    ipc_recv_state_t, ipc_type_name, ipc_type_t,
};

static mut send_thread: *mut GThread = 0 as *const GThread as *mut GThread;
static mut send_queue: *mut GAsyncQueue = 0 as *const GAsyncQueue as *mut GAsyncQueue;
static mut endpoints: *mut GPtrArray = 0 as *const GPtrArray as *mut GPtrArray;
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ipc_endpoints_get() -> *const GPtrArray {
    if endpoints.is_null() {
        endpoints = g_ptr_array_sized_new(1 as std::ffi::c_int as guint);
    }
    return endpoints;
}

unsafe extern "C-unwind" fn ipc_dispatch(
    mut ipc: *mut ipc_endpoint_t,
    mut header: ipc_header_t,
    mut payload: gpointer,
) {
    if header.type_0 as std::ffi::c_uint != IPC_TYPE_log as std::ffi::c_int as std::ffi::c_uint {
        _log(
            LOG_LEVEL_debug,
            b"common/ipc.c\0" as *const u8 as *const std::ffi::c_char,
            b"Process '%s': recv \x1B[34m%s\x1B[0m message\0" as *const u8
                as *const std::ffi::c_char,
            (*ipc).name,
            ipc_type_name(header.type_0),
        );
    }
    match header.type_0 as std::ffi::c_uint {
        1 => {
            ipc_recv_lua_require_module(ipc, payload as *const std::ffi::c_void, header.length);
        }
        2 => {
            ipc_recv_lua_ipc(ipc, payload as *const std::ffi::c_void, header.length);
        }
        4 => {
            ipc_recv_scroll(ipc, payload as *const std::ffi::c_void, header.length);
        }
        8 => {
            ipc_recv_extension_init(ipc, payload as *const std::ffi::c_void, header.length);
        }
        16 => {
            ipc_recv_eval_js(ipc, payload as *const std::ffi::c_void, header.length);
        }
        32 => {
            ipc_recv_log(ipc, payload as *const std::ffi::c_void, header.length);
        }
        64 => {
            ipc_recv_page_created(ipc, payload as *const std::ffi::c_void, header.length);
        }
        128 => {
            ipc_recv_crash(ipc, payload as *const std::ffi::c_void, header.length);
        }
        _ => {
            _log(
                LOG_LEVEL_fatal,
                b"common/ipc.c\0" as *const u8 as *const std::ffi::c_char,
                b"Received message with invalid type 0x%x\0" as *const u8
                    as *const std::ffi::c_char,
                header.type_0 as std::ffi::c_uint,
            );
        }
    };
}
unsafe extern "C-unwind" fn ipc_send_thread(mut UNUSED_user_data: gpointer) -> gpointer {
    while 0 as std::ffi::c_int == 0 {
        let mut out: *mut queued_ipc_t = g_async_queue_pop(send_queue) as *mut queued_ipc_t;
        let mut ipc: *mut ipc_endpoint_t = (*out).ipc;
        let mut header: *mut ipc_header_t = &mut (*out).header;
        let mut data: gpointer = ((*out).payload).as_mut_ptr() as gpointer;
        if !((*ipc).channel).is_null()
            && (*ipc).status as std::ffi::c_uint
                == IPC_ENDPOINT_CONNECTED as std::ffi::c_int as std::ffi::c_uint
        {
            g_io_channel_write_chars(
                (*ipc).channel,
                header as *mut gchar,
                ::core::mem::size_of::<ipc_header_t>() as std::ffi::c_ulong as gssize,
                0 as *mut gsize,
                0 as *mut *mut GError,
            );
        }
        if !((*ipc).channel).is_null()
            && (*ipc).status as std::ffi::c_uint
                == IPC_ENDPOINT_CONNECTED as std::ffi::c_int as std::ffi::c_uint
        {
            g_io_channel_write_chars(
                (*ipc).channel,
                data as *mut gchar,
                (*header).length as gssize,
                0 as *mut gsize,
                0 as *mut *mut GError,
            );
        }
        if !((*ipc).channel).is_null()
            && (*ipc).status as std::ffi::c_uint
                == IPC_ENDPOINT_CONNECTED as std::ffi::c_int as std::ffi::c_uint
        {
            ipc_endpoint_decref(ipc);
        } else {
            _log(
                LOG_LEVEL_error,
                b"common/ipc.c\0" as *const u8 as *const std::ffi::c_char,
                b"Trying to send an ipc message, but the endpoint went away.\0" as *const u8
                    as *const std::ffi::c_char,
            );
        }
        g_free(out as gpointer);
    }
    return 0 as *mut std::ffi::c_void;
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ipc_send(
    mut ipc: *mut ipc_endpoint_t,
    mut header: *const ipc_header_t,
    mut data: *const std::ffi::c_void,
) {
    if send_thread.is_null() {
        send_queue = g_async_queue_new();
        send_thread = g_thread_new(
            b"send_thread\0" as *const u8 as *const std::ffi::c_char,
            Some(ipc_send_thread as unsafe extern "C-unwind" fn(gpointer) -> gpointer),
            0 as *mut std::ffi::c_void,
        );
    }
    if ipc_endpoint_incref(ipc) == 0 {
        return;
    }
    if (*header).type_0 as std::ffi::c_uint != IPC_TYPE_log as std::ffi::c_int as std::ffi::c_uint {
        _log(
            LOG_LEVEL_debug,
            b"common/ipc.c\0" as *const u8 as *const std::ffi::c_char,
            b"Process '%s': send \x1B[34m%s\x1B[0m message\0" as *const u8
                as *const std::ffi::c_char,
            (*ipc).name,
            ipc_type_name((*header).type_0),
        );
    }
    if ((*header).length == 0 as std::ffi::c_int as guint) as std::ffi::c_int
        == (data == 0 as *mut std::ffi::c_void as *const std::ffi::c_void) as std::ffi::c_int
    {
    } else {
        g_assertion_message_expr(
            0 as *mut gchar,
            b"common/ipc.c\0" as *const u8 as *const std::ffi::c_char,
            115 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 9], &[std::ffi::c_char; 9]>(b"ipc_send\0")).as_ptr(),
            b"(header->length == 0) == (data == NULL)\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    let mut msg: *mut queued_ipc_t = g_malloc(
        (::core::mem::size_of::<queued_ipc_t>() as std::ffi::c_ulong)
            .wrapping_add((*header).length as std::ffi::c_ulong),
    ) as *mut queued_ipc_t;
    (*msg).ipc = ipc;
    (*msg).header = *header;
    if (*header).length != 0 {
        memcpy(
            ((*msg).payload).as_mut_ptr() as *mut std::ffi::c_void,
            data,
            (*header).length as std::ffi::c_ulong,
        );
    }
    if !((*ipc).channel).is_null() {
        g_async_queue_push(send_queue, msg as gpointer);
    } else {
        g_queue_push_tail((*ipc).queue, msg as gpointer);
    };
}
unsafe extern "C-unwind" fn ipc_recv_and_dispatch_or_enqueue(mut ipc: *mut ipc_endpoint_t) {
    if !ipc.is_null() {
    } else {
        g_assertion_message_expr(
            0 as *mut gchar,
            b"common/ipc.c\0" as *const u8 as *const std::ffi::c_char,
            133 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 33], &[std::ffi::c_char; 33]>(
                b"ipc_recv_and_dispatch_or_enqueue\0",
            ))
            .as_ptr(),
            b"ipc\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    let mut state: *mut ipc_recv_state_t = &mut (*ipc).recv_state;
    let mut channel: *mut GIOChannel = (*ipc).channel;
    let mut buf: *mut gchar = (if (*state).hdr_done != 0 {
        (*state).payload
    } else {
        &mut (*state).hdr as *mut ipc_header_t as *mut std::ffi::c_void
    })
    .offset((*state).bytes_read as isize) as *mut gchar;
    let mut remaining: gsize = (if (*state).hdr_done != 0 {
        (*state).hdr.length as std::ffi::c_ulong
    } else {
        ::core::mem::size_of::<ipc_header_t>() as std::ffi::c_ulong
    })
    .wrapping_sub((*state).bytes_read);
    let mut bytes_read: gsize = 0;
    let mut error: *mut GError = 0 as *mut GError;
    match g_io_channel_read_chars(channel, buf, remaining, &mut bytes_read, &mut error)
        as std::ffi::c_uint
    {
        1 => {}
        3 => return,
        2 => {
            _log(
                LOG_LEVEL_verbose,
                b"common/ipc.c\0" as *const u8 as *const std::ffi::c_char,
                b"g_io_channel_read_chars(): End Of File received\0" as *const u8
                    as *const std::ffi::c_char,
            );
            if 0 as std::ffi::c_int != 0 {
                (*ipc).refcount;
                (*ipc).refcount;
            } else {
            };
            ::core::intrinsics::atomic_xsub_seqcst(
                &mut (*ipc).refcount as *mut gint,
                1 as std::ffi::c_int,
            );
            return;
        }
        0 => {
            if !(strcmp(
                (*ipc).name as *const std::ffi::c_char,
                b"UI\0" as *const u8 as *const std::ffi::c_char,
            ) == 0 as std::ffi::c_int)
            {
                if !(strcmp(
                    (*error).message as *const std::ffi::c_char,
                    b"Connection reset by peer\0" as *const u8 as *const std::ffi::c_char,
                ) == 0 as std::ffi::c_int)
                {
                    _log(
                        LOG_LEVEL_error,
                        b"common/ipc.c\0" as *const u8 as *const std::ffi::c_char,
                        b"g_io_channel_read_chars(): %s\0" as *const u8 as *const std::ffi::c_char,
                        (*error).message,
                    );
                }
            }
            g_error_free(error);
            return;
        }
        _ => {
            g_assertion_message_expr(
                0 as *mut gchar,
                b"common/ipc.c\0" as *const u8 as *const std::ffi::c_char,
                170 as std::ffi::c_int,
                (*::core::mem::transmute::<&[u8; 33], &[std::ffi::c_char; 33]>(
                    b"ipc_recv_and_dispatch_or_enqueue\0",
                ))
                .as_ptr(),
                0 as *const std::ffi::c_char,
            );
        }
    }
    (*state).bytes_read = ((*state).bytes_read).wrapping_add(bytes_read);
    remaining = remaining.wrapping_sub(bytes_read);
    if remaining > 0 as std::ffi::c_int as gsize {
        return;
    }
    if (*state).hdr_done == 0 {
        (*state).hdr_done = (0 as std::ffi::c_int == 0) as std::ffi::c_int;
        (*state).bytes_read = 0 as std::ffi::c_int as gsize;
        (*state).payload = g_malloc((*state).hdr.length as gsize);
        ipc_recv_and_dispatch_or_enqueue(ipc);
        return;
    }
    ipc_dispatch(ipc, (*state).hdr, (*state).payload);
    g_free((*state).payload);
    (*state).payload = 0 as *mut std::ffi::c_void;
    (*state).bytes_read = 0 as std::ffi::c_int as gsize;
    (*state).hdr_done = 0 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn ipc_recv(
    mut UNUSED_channel: *mut GIOChannel,
    mut UNUSED_cond: GIOCondition,
    mut ipc: *mut ipc_endpoint_t,
) -> gboolean {
    if ipc_endpoint_incref(ipc) == 0 {
        return (0 as std::ffi::c_int == 0) as std::ffi::c_int;
    }
    ipc_recv_and_dispatch_or_enqueue(ipc);
    ipc_endpoint_decref(ipc);
    return (0 as std::ffi::c_int == 0) as std::ffi::c_int;
}
unsafe extern "C-unwind" fn ipc_hup(
    mut UNUSED_channel: *mut GIOChannel,
    mut UNUSED_cond: GIOCondition,
    mut ipc: *mut ipc_endpoint_t,
) -> gboolean {
    if (*ipc).status as std::ffi::c_uint
        == IPC_ENDPOINT_CONNECTED as std::ffi::c_int as std::ffi::c_uint
    {
    } else {
        g_assertion_message_expr(
            0 as *mut gchar,
            b"common/ipc.c\0" as *const u8 as *const std::ffi::c_char,
            214 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 8], &[std::ffi::c_char; 8]>(b"ipc_hup\0")).as_ptr(),
            b"ipc->status == IPC_ENDPOINT_CONNECTED\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    if !((*ipc).channel).is_null() {
    } else {
        g_assertion_message_expr(
            0 as *mut gchar,
            b"common/ipc.c\0" as *const u8 as *const std::ffi::c_char,
            215 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 8], &[std::ffi::c_char; 8]>(b"ipc_hup\0")).as_ptr(),
            b"ipc->channel\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    ipc_endpoint_decref(ipc);
    return (0 as std::ffi::c_int == 0) as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ipc_send_lua(
    mut ipc: *mut ipc_endpoint_t,
    mut type_0: ipc_type_t,
    mut L: *mut lua_State,
    mut start: gint,
    mut end: gint,
) {
    let mut buf: *mut GByteArray = g_byte_array_new();
    lua_serialize_range(L, buf, start, end);
    let mut header: ipc_header_t = {
        let mut init = _ipc_header_t {
            length: (*buf).len,
            type_0: type_0,
        };
        init
    };
    ipc_send(ipc, &mut header, (*buf).data as *const std::ffi::c_void);
    g_byte_array_unref(buf);
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ipc_endpoint_new(mut name: *const gchar) -> *mut ipc_endpoint_t {
    let mut ipc: *mut ipc_endpoint_t =
        g_slice_alloc0(::core::mem::size_of::<ipc_endpoint_t>() as std::ffi::c_ulong)
            as *mut ipc_endpoint_t;
    (*ipc).name = name as *mut gchar;
    (*ipc).queue = g_queue_new();
    (*ipc).status = IPC_ENDPOINT_DISCONNECTED;
    (*ipc).refcount = 1 as std::ffi::c_int;
    (*ipc).creation_notified = 0 as std::ffi::c_int;
    return ipc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ipc_endpoint_incref(mut ipc: *mut ipc_endpoint_t) -> gboolean {
    let mut old: std::ffi::c_int = 0;
    loop {
        old = ({
            let mut gaig_temp: gint = 0;
            if 0 as std::ffi::c_int != 0 {
                (*ipc).refcount;
                (*ipc).refcount;
            } else {
            };
            *&mut gaig_temp =
                ::core::intrinsics::atomic_load_seqcst(&mut (*ipc).refcount as *mut gint);
            gaig_temp
        });
        if old < 1 as std::ffi::c_int {
            return 0 as std::ffi::c_int;
        }
        if !(({
            let mut gaicae_oldval: gint = old;
            if 0 as std::ffi::c_int != 0 {
                (*ipc).refcount;
            } else {
            };
            let fresh0 = ::core::intrinsics::atomic_cxchg_seqcst_seqcst(
                &mut (*ipc).refcount as *mut gint,
                *(&mut gaicae_oldval as *mut gint as *mut std::ffi::c_void as *mut gint),
                old + 1 as std::ffi::c_int,
            );
            *(&mut gaicae_oldval as *mut gint as *mut std::ffi::c_void as *mut gint) = fresh0.0;
            if fresh0.1 as std::ffi::c_int != 0 {
                (0 as std::ffi::c_int == 0) as std::ffi::c_int
            } else {
                0 as std::ffi::c_int
            }
        }) == 0)
        {
            break;
        }
    }
    return (0 as std::ffi::c_int == 0) as std::ffi::c_int;
}
unsafe extern "C-unwind" fn ipc_endpoint_incref_no_check(mut ipc: *mut ipc_endpoint_t) {
    if 0 as std::ffi::c_int != 0 {
        (*ipc).refcount;
        (*ipc).refcount;
    } else {
    };
    ::core::intrinsics::atomic_xadd_seqcst(&mut (*ipc).refcount, 1 as std::ffi::c_int);
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ipc_endpoint_decref(mut ipc: *mut ipc_endpoint_t) {
    if ({
        if 0 as std::ffi::c_int != 0 {
            (*ipc).refcount;
            (*ipc).refcount;
        } else {
        };
        (::core::intrinsics::atomic_xsub_seqcst(
            &mut (*ipc).refcount as *mut gint,
            1 as std::ffi::c_int,
        ) == 1 as std::ffi::c_int) as std::ffi::c_int
    }) == 0
    {
        return;
    }
    if (*ipc).status as std::ffi::c_uint
        == IPC_ENDPOINT_CONNECTED as std::ffi::c_int as std::ffi::c_uint
    {
        ipc_endpoint_disconnect(ipc);
    }
    if !((*ipc).queue).is_null() {
        while g_queue_is_empty((*ipc).queue) == 0 {
            let mut msg: *mut queued_ipc_t = g_queue_pop_head((*ipc).queue) as *mut queued_ipc_t;
            g_free(msg as gpointer);
        }
        g_queue_free((*ipc).queue);
    }
    (*ipc).status = IPC_ENDPOINT_FREED;
    g_slice_free1(
        ::core::mem::size_of::<ipc_endpoint_t>() as std::ffi::c_ulong,
        ipc as gpointer,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ipc_endpoint_connect_to_socket(
    mut ipc: *mut ipc_endpoint_t,
    mut sock: std::ffi::c_int,
) {
    if !ipc.is_null() {
    } else {
        g_assertion_message_expr(
            0 as *mut gchar,
            b"common/ipc.c\0" as *const u8 as *const std::ffi::c_char,
            284 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 31], &[std::ffi::c_char; 31]>(
                b"ipc_endpoint_connect_to_socket\0",
            ))
            .as_ptr(),
            b"ipc\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    if (*ipc).status as std::ffi::c_uint
        == IPC_ENDPOINT_DISCONNECTED as std::ffi::c_int as std::ffi::c_uint
    {
    } else {
        g_assertion_message_expr(
            0 as *mut gchar,
            b"common/ipc.c\0" as *const u8 as *const std::ffi::c_char,
            285 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 31], &[std::ffi::c_char; 31]>(
                b"ipc_endpoint_connect_to_socket\0",
            ))
            .as_ptr(),
            b"ipc->status == IPC_ENDPOINT_DISCONNECTED\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    let mut state: *mut ipc_recv_state_t = &mut (*ipc).recv_state;
    (*state).queued_ipcs = g_ptr_array_new();
    let mut channel: *mut GIOChannel = g_io_channel_unix_new(sock);
    g_io_channel_set_encoding(channel, 0 as *const gchar, 0 as *mut *mut GError);
    g_io_channel_set_buffered(channel, 0 as std::ffi::c_int);
    (*state).watch_in_id = g_io_add_watch(
        channel,
        G_IO_IN,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C-unwind" fn(
                    *mut GIOChannel,
                    GIOCondition,
                    *mut ipc_endpoint_t,
                ) -> gboolean,
            >,
            GIOFunc,
        >(Some(
            ipc_recv
                as unsafe extern "C-unwind" fn(
                    *mut GIOChannel,
                    GIOCondition,
                    *mut ipc_endpoint_t,
                ) -> gboolean,
        )),
        ipc as gpointer,
    );
    (*state).watch_hup_id = g_io_add_watch(
        channel,
        G_IO_HUP,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C-unwind" fn(
                    *mut GIOChannel,
                    GIOCondition,
                    *mut ipc_endpoint_t,
                ) -> gboolean,
            >,
            GIOFunc,
        >(Some(
            ipc_hup
                as unsafe extern "C-unwind" fn(
                    *mut GIOChannel,
                    GIOCondition,
                    *mut ipc_endpoint_t,
                ) -> gboolean,
        )),
        ipc as gpointer,
    );
    let mut gaps_temp_atomic: *mut *mut GIOChannel = &mut (*ipc).channel;
    let mut gaps_temp_newval: *mut GIOChannel = channel;
    if 0 as std::ffi::c_int != 0 {
        (*ipc).channel;
    } else {
    };
    ::core::intrinsics::atomic_store_seqcst(gaps_temp_atomic, *&mut gaps_temp_newval);
    (*ipc).status = IPC_ENDPOINT_CONNECTED;
    if endpoints.is_null() {
        endpoints = g_ptr_array_sized_new(1 as std::ffi::c_int as guint);
    }
    if g_ptr_array_remove_fast(endpoints, ipc as gpointer) == 0 {
    } else {
        g_assertion_message_expr(
            0 as *mut gchar,
            b"common/ipc.c\0" as *const u8 as *const std::ffi::c_char,
            308 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 31], &[std::ffi::c_char; 31]>(
                b"ipc_endpoint_connect_to_socket\0",
            ))
            .as_ptr(),
            b"!g_ptr_array_remove_fast(endpoints, ipc)\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    g_ptr_array_add(endpoints, ipc as gpointer);
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ipc_endpoint_replace(
    mut orig: *mut ipc_endpoint_t,
    mut new: *mut ipc_endpoint_t,
) -> *mut ipc_endpoint_t {
    if !orig.is_null() {
    } else {
        g_assertion_message_expr(
            0 as *mut gchar,
            b"common/ipc.c\0" as *const u8 as *const std::ffi::c_char,
            315 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 21], &[std::ffi::c_char; 21]>(
                b"ipc_endpoint_replace\0",
            ))
            .as_ptr(),
            b"orig\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    if !new.is_null() {
    } else {
        g_assertion_message_expr(
            0 as *mut gchar,
            b"common/ipc.c\0" as *const u8 as *const std::ffi::c_char,
            316 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 21], &[std::ffi::c_char; 21]>(
                b"ipc_endpoint_replace\0",
            ))
            .as_ptr(),
            b"new\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    if (*orig).status as std::ffi::c_uint
        == IPC_ENDPOINT_DISCONNECTED as std::ffi::c_int as std::ffi::c_uint
    {
    } else {
        g_assertion_message_expr(
            0 as *mut gchar,
            b"common/ipc.c\0" as *const u8 as *const std::ffi::c_char,
            317 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 21], &[std::ffi::c_char; 21]>(
                b"ipc_endpoint_replace\0",
            ))
            .as_ptr(),
            b"orig->status == IPC_ENDPOINT_DISCONNECTED\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    if (*new).status as std::ffi::c_uint
        == IPC_ENDPOINT_CONNECTED as std::ffi::c_int as std::ffi::c_uint
    {
    } else {
        g_assertion_message_expr(
            0 as *mut gchar,
            b"common/ipc.c\0" as *const u8 as *const std::ffi::c_char,
            318 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 21], &[std::ffi::c_char; 21]>(
                b"ipc_endpoint_replace\0",
            ))
            .as_ptr(),
            b"new->status == IPC_ENDPOINT_CONNECTED\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    ipc_endpoint_incref_no_check(new);
    if !((*orig).queue).is_null() {
        while g_queue_is_empty((*orig).queue) == 0 {
            let mut msg: *mut queued_ipc_t = g_queue_pop_head((*orig).queue) as *mut queued_ipc_t;
            (*msg).ipc = new;
            ipc_endpoint_incref_no_check(new);
            g_async_queue_push(send_queue, msg as gpointer);
        }
        g_queue_free((*orig).queue);
        (*orig).queue = 0 as *mut GQueue;
    }
    ipc_endpoint_decref(orig);
    return new;
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ipc_endpoint_disconnect(mut ipc: *mut ipc_endpoint_t) {
    if (*ipc).status as std::ffi::c_uint
        == IPC_ENDPOINT_CONNECTED as std::ffi::c_int as std::ffi::c_uint
    {
    } else {
        g_assertion_message_expr(
            0 as *mut gchar,
            b"common/ipc.c\0" as *const u8 as *const std::ffi::c_char,
            344 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 24], &[std::ffi::c_char; 24]>(
                b"ipc_endpoint_disconnect\0",
            ))
            .as_ptr(),
            b"ipc->status == IPC_ENDPOINT_CONNECTED\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    if !((*ipc).channel).is_null() {
    } else {
        g_assertion_message_expr(
            0 as *mut gchar,
            b"common/ipc.c\0" as *const u8 as *const std::ffi::c_char,
            345 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 24], &[std::ffi::c_char; 24]>(
                b"ipc_endpoint_disconnect\0",
            ))
            .as_ptr(),
            b"ipc->channel\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    g_ptr_array_remove_fast(endpoints, ipc as gpointer);
    let mut state: *mut ipc_recv_state_t = &mut (*ipc).recv_state;
    g_source_remove((*state).watch_in_id);
    g_source_remove((*state).watch_hup_id);
    g_io_channel_shutdown(
        (*ipc).channel,
        (0 as std::ffi::c_int == 0) as std::ffi::c_int,
        0 as *mut *mut GError,
    );
    (*ipc).status = IPC_ENDPOINT_DISCONNECTED;
    (*ipc).channel = 0 as *mut GIOChannel;
}
