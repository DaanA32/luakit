use ::libc;
use ::c2rust_bitfields;
#[c2rust::header_src = "/usr/lib/glib-2.0/include/glibconfig.h:21"]
pub mod glibconfig_h {
    #[c2rust::src_loc = "46:1"]
    pub type guint8 = std::ffi::c_uchar;
    #[c2rust::src_loc = "57:1"]
    pub type guint32 = std::ffi::c_uint;
    #[c2rust::src_loc = "66:1"]
    pub type gint64 = std::ffi::c_long;
    #[c2rust::src_loc = "82:1"]
    pub type gssize = std::ffi::c_long;
    #[c2rust::src_loc = "83:1"]
    pub type gsize = std::ffi::c_ulong;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtypes.h:21"]
pub mod gtypes_h {
    #[c2rust::src_loc = "52:1"]
    pub type gchar = std::ffi::c_char;
    #[c2rust::src_loc = "55:1"]
    pub type gint = std::ffi::c_int;
    #[c2rust::src_loc = "56:1"]
    pub type gboolean = gint;
    #[c2rust::src_loc = "61:1"]
    pub type guint = std::ffi::c_uint;
    #[c2rust::src_loc = "109:1"]
    pub type gpointer = *mut std::ffi::c_void;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/garray.h:21"]
pub mod garray_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:8"]
    pub struct _GByteArray {
        pub data: *mut guint8,
        pub len: guint,
    }
    #[c2rust::src_loc = "40:1"]
    pub type GByteArray = _GByteArray;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "55:8"]
    pub struct _GPtrArray {
        pub pdata: *mut gpointer,
        pub len: guint,
    }
    #[c2rust::src_loc = "41:1"]
    pub type GPtrArray = _GPtrArray;
    use super::glibconfig_h::guint8;
    use super::gtypes_h::{guint, gpointer, gboolean};
    extern "C" {
        #[c2rust::src_loc = "150:1"]
        pub fn g_ptr_array_new() -> *mut GPtrArray;
        #[c2rust::src_loc = "171:1"]
        pub fn g_ptr_array_sized_new(reserved_size: guint) -> *mut GPtrArray;
        #[c2rust::src_loc = "216:1"]
        pub fn g_ptr_array_remove_fast(
            array: *mut GPtrArray,
            data: gpointer,
        ) -> gboolean;
        #[c2rust::src_loc = "223:1"]
        pub fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
        #[c2rust::src_loc = "273:1"]
        pub fn g_byte_array_new() -> *mut GByteArray;
        #[c2rust::src_loc = "290:1"]
        pub fn g_byte_array_unref(array: *mut GByteArray);
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gquark.h:21"]
pub mod gquark_h {
    #[c2rust::src_loc = "38:1"]
    pub type GQuark = guint32;
    use super::glibconfig_h::guint32;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gerror.h:21"]
pub mod gerror_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "45:8"]
    pub struct _GError {
        pub domain: GQuark,
        pub code: gint,
        pub message: *mut gchar,
    }
    #[c2rust::src_loc = "43:1"]
    pub type GError = _GError;
    use super::gquark_h::GQuark;
    use super::gtypes_h::{gint, gchar};
    extern "C" {
        #[c2rust::src_loc = "207:1"]
        pub fn g_error_free(error: *mut GError);
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gthread.h:21"]
pub mod gthread_h {
    #[c2rust::src_loc = "49:1"]
    pub type GThreadFunc = Option::<unsafe extern "C" fn(gpointer) -> gpointer>;
    #[c2rust::src_loc = "51:1"]
    pub type GThread = _GThread;
    use super::gtypes_h::{gpointer, gchar};
    use super::deprecated_gthread_h::_GThread;
    extern "C" {
        #[c2rust::src_loc = "150:1"]
        pub fn g_thread_new(
            name: *const gchar,
            func: GThreadFunc,
            data: gpointer,
        ) -> *mut GThread;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/deprecated/gthread.h:21"]
pub mod deprecated_gthread_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "48:9"]
    pub struct _GThread {
        pub func: GThreadFunc,
        pub data: gpointer,
        pub joinable: gboolean,
        pub priority: GThreadPriority,
    }
    #[c2rust::src_loc = "40:9"]
    pub type GThreadPriority = std::ffi::c_uint;
    #[c2rust::src_loc = "45:3"]
    pub const G_THREAD_PRIORITY_URGENT: GThreadPriority = 3;
    #[c2rust::src_loc = "44:3"]
    pub const G_THREAD_PRIORITY_HIGH: GThreadPriority = 2;
    #[c2rust::src_loc = "43:3"]
    pub const G_THREAD_PRIORITY_NORMAL: GThreadPriority = 1;
    #[c2rust::src_loc = "42:3"]
    pub const G_THREAD_PRIORITY_LOW: GThreadPriority = 0;
    use super::gthread_h::GThreadFunc;
    use super::gtypes_h::{gpointer, gboolean};
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gasyncqueue.h:21"]
pub mod gasyncqueue_h {
    #[c2rust::src_loc = "38:1"]
    pub type GAsyncQueue = _GAsyncQueue;
    use super::gtypes_h::gpointer;
    extern "C" {
        #[c2rust::src_loc = "38:16"]
        pub type _GAsyncQueue;
        #[c2rust::src_loc = "40:1"]
        pub fn g_async_queue_new() -> *mut GAsyncQueue;
        #[c2rust::src_loc = "59:1"]
        pub fn g_async_queue_push(queue: *mut GAsyncQueue, data: gpointer);
        #[c2rust::src_loc = "75:1"]
        pub fn g_async_queue_pop(queue: *mut GAsyncQueue) -> gpointer;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gconvert.h:21"]
pub mod gconvert_h {
    #[c2rust::src_loc = "85:1"]
    pub type GIConv = *mut _GIConv;
    extern "C" {
        #[c2rust::src_loc = "85:16"]
        pub type _GIConv;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/glist.h:21"]
pub mod glist_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "41:8"]
    pub struct _GList {
        pub data: gpointer,
        pub next: *mut GList,
        pub prev: *mut GList,
    }
    #[c2rust::src_loc = "39:1"]
    pub type GList = _GList;
    use super::gtypes_h::gpointer;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gslist.h:21"]
pub mod gslist_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "41:8"]
    pub struct _GSList {
        pub data: gpointer,
        pub next: *mut GSList,
    }
    #[c2rust::src_loc = "39:1"]
    pub type GSList = _GSList;
    use super::gtypes_h::gpointer;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gmain.h:21"]
pub mod gmain_h {
    #[c2rust::src_loc = "33:9"]
    pub type GIOCondition = std::ffi::c_uint;
    #[c2rust::src_loc = "40:3"]
    pub const G_IO_NVAL: GIOCondition = 32;
    #[c2rust::src_loc = "39:3"]
    pub const G_IO_HUP: GIOCondition = 16;
    #[c2rust::src_loc = "38:3"]
    pub const G_IO_ERR: GIOCondition = 8;
    #[c2rust::src_loc = "37:3"]
    pub const G_IO_PRI: GIOCondition = 2;
    #[c2rust::src_loc = "36:3"]
    pub const G_IO_OUT: GIOCondition = 4;
    #[c2rust::src_loc = "35:3"]
    pub const G_IO_IN: GIOCondition = 1;
    #[c2rust::src_loc = "70:1"]
    pub type GMainContext = _GMainContext;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "266:8"]
    pub struct _GSource {
        pub callback_data: gpointer,
        pub callback_funcs: *mut GSourceCallbackFuncs,
        pub source_funcs: *const GSourceFuncs,
        pub ref_count: guint,
        pub context: *mut GMainContext,
        pub priority: gint,
        pub flags: guint,
        pub source_id: guint,
        pub poll_fds: *mut GSList,
        pub prev: *mut GSource,
        pub next: *mut GSource,
        pub name: *mut std::ffi::c_char,
        pub priv_0: *mut GSourcePrivate,
    }
    #[c2rust::src_loc = "87:1"]
    pub type GSourcePrivate = _GSourcePrivate;
    #[c2rust::src_loc = "86:1"]
    pub type GSource = _GSource;
    #[c2rust::src_loc = "157:1"]
    pub type GSourceFuncs = _GSourceFuncs;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "398:8"]
    pub struct _GSourceFuncs {
        pub prepare: GSourceFuncsPrepareFunc,
        pub check: GSourceFuncsCheckFunc,
        pub dispatch: GSourceFuncsDispatchFunc,
        pub finalize: GSourceFuncsFinalizeFunc,
        pub closure_callback: GSourceFunc,
        pub closure_marshal: GSourceDummyMarshal,
    }
    #[c2rust::src_loc = "307:1"]
    pub type GSourceDummyMarshal = Option::<unsafe extern "C" fn() -> ()>;
    #[c2rust::src_loc = "199:1"]
    pub type GSourceFunc = Option::<unsafe extern "C" fn(gpointer) -> gboolean>;
    #[c2rust::src_loc = "396:1"]
    pub type GSourceFuncsFinalizeFunc = Option::<
        unsafe extern "C" fn(*mut GSource) -> (),
    >;
    #[c2rust::src_loc = "379:1"]
    pub type GSourceFuncsDispatchFunc = Option::<
        unsafe extern "C" fn(*mut GSource, GSourceFunc, gpointer) -> gboolean,
    >;
    #[c2rust::src_loc = "354:1"]
    pub type GSourceFuncsCheckFunc = Option::<
        unsafe extern "C" fn(*mut GSource) -> gboolean,
    >;
    #[c2rust::src_loc = "333:1"]
    pub type GSourceFuncsPrepareFunc = Option::<
        unsafe extern "C" fn(*mut GSource, *mut gint) -> gboolean,
    >;
    #[c2rust::src_loc = "99:1"]
    pub type GSourceCallbackFuncs = _GSourceCallbackFuncs;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "291:8"]
    pub struct _GSourceCallbackFuncs {
        pub ref_0: Option::<unsafe extern "C" fn(gpointer) -> ()>,
        pub unref: Option::<unsafe extern "C" fn(gpointer) -> ()>,
        pub get: Option::<
            unsafe extern "C" fn(
                gpointer,
                *mut GSource,
                *mut GSourceFunc,
                *mut gpointer,
            ) -> (),
        >,
    }
    use super::gtypes_h::{gpointer, guint, gint, gboolean};
    use super::gslist_h::GSList;
    extern "C" {
        #[c2rust::src_loc = "70:16"]
        pub type _GMainContext;
        #[c2rust::src_loc = "87:16"]
        pub type _GSourcePrivate;
        #[c2rust::src_loc = "835:1"]
        pub fn g_source_remove(tag: guint) -> gboolean;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gstring.h:21"]
pub mod gstring_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "45:8"]
    pub struct _GString {
        pub str_0: *mut gchar,
        pub len: gsize,
        pub allocated_len: gsize,
    }
    #[c2rust::src_loc = "43:1"]
    pub type GString = _GString;
    use super::gtypes_h::gchar;
    use super::glibconfig_h::gsize;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/giochannel.h:21"]
pub mod giochannel_h {
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "100:8"]
    pub struct _GIOChannel {
        pub ref_count: gint,
        pub funcs: *mut GIOFuncs,
        pub encoding: *mut gchar,
        pub read_cd: GIConv,
        pub write_cd: GIConv,
        pub line_term: *mut gchar,
        pub line_term_len: guint,
        pub buf_size: gsize,
        pub read_buf: *mut GString,
        pub encoded_read_buf: *mut GString,
        pub write_buf: *mut GString,
        pub partial_write_buf: [gchar; 6],
        #[bitfield(name = "use_buffer", ty = "guint", bits = "0..=0")]
        #[bitfield(name = "do_encode", ty = "guint", bits = "1..=1")]
        #[bitfield(name = "close_on_unref", ty = "guint", bits = "2..=2")]
        #[bitfield(name = "is_readable", ty = "guint", bits = "3..=3")]
        #[bitfield(name = "is_writeable", ty = "guint", bits = "4..=4")]
        #[bitfield(name = "is_seekable", ty = "guint", bits = "5..=5")]
        pub use_buffer_do_encode_close_on_unref_is_readable_is_writeable_is_seekable: [u8; 1],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 1],
        pub reserved1: gpointer,
        pub reserved2: gpointer,
    }
    #[c2rust::src_loc = "44:1"]
    pub type GIOFuncs = _GIOFuncs;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "134:8"]
    pub struct _GIOFuncs {
        pub io_read: Option::<
            unsafe extern "C" fn(
                *mut GIOChannel,
                *mut gchar,
                gsize,
                *mut gsize,
                *mut *mut GError,
            ) -> GIOStatus,
        >,
        pub io_write: Option::<
            unsafe extern "C" fn(
                *mut GIOChannel,
                *const gchar,
                gsize,
                *mut gsize,
                *mut *mut GError,
            ) -> GIOStatus,
        >,
        pub io_seek: Option::<
            unsafe extern "C" fn(
                *mut GIOChannel,
                gint64,
                GSeekType,
                *mut *mut GError,
            ) -> GIOStatus,
        >,
        pub io_close: Option::<
            unsafe extern "C" fn(*mut GIOChannel, *mut *mut GError) -> GIOStatus,
        >,
        pub io_create_watch: Option::<
            unsafe extern "C" fn(*mut GIOChannel, GIOCondition) -> *mut GSource,
        >,
        pub io_free: Option::<unsafe extern "C" fn(*mut GIOChannel) -> ()>,
        pub io_set_flags: Option::<
            unsafe extern "C" fn(
                *mut GIOChannel,
                GIOFlags,
                *mut *mut GError,
            ) -> GIOStatus,
        >,
        pub io_get_flags: Option::<unsafe extern "C" fn(*mut GIOChannel) -> GIOFlags>,
    }
    #[c2rust::src_loc = "43:1"]
    pub type GIOChannel = _GIOChannel;
    #[c2rust::src_loc = "86:9"]
    pub type GIOFlags = std::ffi::c_uint;
    #[c2rust::src_loc = "97:3"]
    pub const G_IO_FLAG_SET_MASK: GIOFlags = 3;
    #[c2rust::src_loc = "96:3"]
    pub const G_IO_FLAG_GET_MASK: GIOFlags = 31;
    #[c2rust::src_loc = "95:3"]
    pub const G_IO_FLAG_MASK: GIOFlags = 31;
    #[c2rust::src_loc = "94:3"]
    pub const G_IO_FLAG_IS_SEEKABLE: GIOFlags = 16;
    #[c2rust::src_loc = "93:3"]
    pub const G_IO_FLAG_IS_WRITEABLE: GIOFlags = 8;
    #[c2rust::src_loc = "92:3"]
    pub const G_IO_FLAG_IS_WRITABLE: GIOFlags = 8;
    #[c2rust::src_loc = "91:3"]
    pub const G_IO_FLAG_IS_READABLE: GIOFlags = 4;
    #[c2rust::src_loc = "90:3"]
    pub const G_IO_FLAG_NONBLOCK: GIOFlags = 2;
    #[c2rust::src_loc = "89:3"]
    pub const G_IO_FLAG_APPEND: GIOFlags = 1;
    #[c2rust::src_loc = "88:3"]
    pub const G_IO_FLAG_NONE: GIOFlags = 0;
    #[c2rust::src_loc = "71:9"]
    pub type GIOStatus = std::ffi::c_uint;
    #[c2rust::src_loc = "76:3"]
    pub const G_IO_STATUS_AGAIN: GIOStatus = 3;
    #[c2rust::src_loc = "75:3"]
    pub const G_IO_STATUS_EOF: GIOStatus = 2;
    #[c2rust::src_loc = "74:3"]
    pub const G_IO_STATUS_NORMAL: GIOStatus = 1;
    #[c2rust::src_loc = "73:3"]
    pub const G_IO_STATUS_ERROR: GIOStatus = 0;
    #[c2rust::src_loc = "79:9"]
    pub type GSeekType = std::ffi::c_uint;
    #[c2rust::src_loc = "83:3"]
    pub const G_SEEK_END: GSeekType = 2;
    #[c2rust::src_loc = "82:3"]
    pub const G_SEEK_SET: GSeekType = 1;
    #[c2rust::src_loc = "81:3"]
    pub const G_SEEK_CUR: GSeekType = 0;
    #[c2rust::src_loc = "131:1"]
    pub type GIOFunc = Option::<
        unsafe extern "C" fn(*mut GIOChannel, GIOCondition, gpointer) -> gboolean,
    >;
    use super::gtypes_h::{gint, gchar, guint, gpointer, gboolean};
    use super::gconvert_h::GIConv;
    use super::glibconfig_h::{gsize, gint64, gssize};
    use super::gstring_h::GString;
    use super::gerror_h::GError;
    use super::gmain_h::{GSource, GIOCondition};
    extern "C" {
        #[c2rust::src_loc = "188:1"]
        pub fn g_io_channel_shutdown(
            channel: *mut GIOChannel,
            flush: gboolean,
            err: *mut *mut GError,
        ) -> GIOStatus;
        #[c2rust::src_loc = "202:1"]
        pub fn g_io_add_watch(
            channel: *mut GIOChannel,
            condition: GIOCondition,
            func: GIOFunc,
            user_data: gpointer,
        ) -> guint;
        #[c2rust::src_loc = "231:1"]
        pub fn g_io_channel_set_buffered(channel: *mut GIOChannel, buffered: gboolean);
        #[c2rust::src_loc = "236:1"]
        pub fn g_io_channel_set_encoding(
            channel: *mut GIOChannel,
            encoding: *const gchar,
            error: *mut *mut GError,
        ) -> GIOStatus;
        #[c2rust::src_loc = "268:1"]
        pub fn g_io_channel_read_chars(
            channel: *mut GIOChannel,
            buf: *mut gchar,
            count: gsize,
            bytes_read: *mut gsize,
            error: *mut *mut GError,
        ) -> GIOStatus;
        #[c2rust::src_loc = "278:1"]
        pub fn g_io_channel_write_chars(
            channel: *mut GIOChannel,
            buf: *const gchar,
            count: gssize,
            bytes_written: *mut gsize,
            error: *mut *mut GError,
        ) -> GIOStatus;
        #[c2rust::src_loc = "323:1"]
        pub fn g_io_channel_unix_new(fd: std::ffi::c_int) -> *mut GIOChannel;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gqueue.h:21"]
pub mod gqueue_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:8"]
    pub struct _GQueue {
        pub head: *mut GList,
        pub tail: *mut GList,
        pub length: guint,
    }
    #[c2rust::src_loc = "38:1"]
    pub type GQueue = _GQueue;
    use super::glist_h::GList;
    use super::gtypes_h::{guint, gboolean, gpointer};
    extern "C" {
        #[c2rust::src_loc = "74:1"]
        pub fn g_queue_new() -> *mut GQueue;
        #[c2rust::src_loc = "76:1"]
        pub fn g_queue_free(queue: *mut GQueue);
        #[c2rust::src_loc = "85:1"]
        pub fn g_queue_is_empty(queue: *mut GQueue) -> gboolean;
        #[c2rust::src_loc = "115:1"]
        pub fn g_queue_push_tail(queue: *mut GQueue, data: gpointer);
        #[c2rust::src_loc = "122:1"]
        pub fn g_queue_pop_head(queue: *mut GQueue) -> gpointer;
    }
}
#[c2rust::header_src = "/usr/include/luajit-2.1/lua.h:21"]
pub mod lua_h {
    extern "C" {
        #[c2rust::src_loc = "51:16"]
        pub type lua_State;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/log.h:21"]
pub mod log_h {
    #[c2rust::src_loc = "36:9"]
    pub type log_level_t = std::ffi::c_uint;
    #[c2rust::src_loc = "36:16"]
    pub const LOG_LEVEL_debug: log_level_t = 5;
    #[c2rust::src_loc = "36:16"]
    pub const LOG_LEVEL_verbose: log_level_t = 4;
    #[c2rust::src_loc = "36:16"]
    pub const LOG_LEVEL_info: log_level_t = 3;
    #[c2rust::src_loc = "36:16"]
    pub const LOG_LEVEL_warn: log_level_t = 2;
    #[c2rust::src_loc = "36:16"]
    pub const LOG_LEVEL_error: log_level_t = 1;
    #[c2rust::src_loc = "36:16"]
    pub const LOG_LEVEL_fatal: log_level_t = 0;
    use super::gtypes_h::gchar;
    extern "C" {
        #[c2rust::src_loc = "54:1"]
        pub fn _log(lvl: log_level_t, _: *const gchar, _: *const gchar, _: ...);
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/ipc.h:23"]
pub mod ipc_h {
    #[c2rust::src_loc = "41:9"]
    pub type ipc_type_t = std::ffi::c_uint;
    #[c2rust::src_loc = "41:16"]
    pub const IPC_TYPE_crash: ipc_type_t = 128;
    #[c2rust::src_loc = "41:16"]
    pub const IPC_TYPE_page_created: ipc_type_t = 64;
    #[c2rust::src_loc = "41:16"]
    pub const IPC_TYPE_log: ipc_type_t = 32;
    #[c2rust::src_loc = "41:16"]
    pub const IPC_TYPE_eval_js: ipc_type_t = 16;
    #[c2rust::src_loc = "41:16"]
    pub const IPC_TYPE_extension_init: ipc_type_t = 8;
    #[c2rust::src_loc = "41:16"]
    pub const IPC_TYPE_scroll: ipc_type_t = 4;
    #[c2rust::src_loc = "41:16"]
    pub const IPC_TYPE_lua_ipc: ipc_type_t = 2;
    #[c2rust::src_loc = "41:16"]
    pub const IPC_TYPE_lua_require_module: ipc_type_t = 1;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "47:16"]
    pub struct _ipc_header_t {
        pub length: guint,
        pub type_0: ipc_type_t,
    }
    #[c2rust::src_loc = "47:1"]
    pub type ipc_header_t = _ipc_header_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "94:16"]
    pub struct _ipc_recv_state_t {
        pub watch_in_id: guint,
        pub watch_hup_id: guint,
        pub queued_ipcs: *mut GPtrArray,
        pub hdr: ipc_header_t,
        pub payload: gpointer,
        pub bytes_read: gsize,
        pub hdr_done: gboolean,
    }
    #[c2rust::src_loc = "94:1"]
    pub type ipc_recv_state_t = _ipc_recv_state_t;
    #[c2rust::src_loc = "104:9"]
    pub type ipc_endpoint_status_t = std::ffi::c_uint;
    #[c2rust::src_loc = "107:5"]
    pub const IPC_ENDPOINT_FREED: ipc_endpoint_status_t = 2;
    #[c2rust::src_loc = "106:5"]
    pub const IPC_ENDPOINT_CONNECTED: ipc_endpoint_status_t = 1;
    #[c2rust::src_loc = "105:5"]
    pub const IPC_ENDPOINT_DISCONNECTED: ipc_endpoint_status_t = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "110:16"]
    pub struct _ipc_endpoint_t {
        pub name: *mut gchar,
        pub status: ipc_endpoint_status_t,
        pub channel: *mut GIOChannel,
        pub queue: *mut GQueue,
        pub recv_state: ipc_recv_state_t,
        pub refcount: gint,
        pub creation_notified: gboolean,
    }
    #[c2rust::src_loc = "110:1"]
    pub type ipc_endpoint_t = _ipc_endpoint_t;
    #[inline]
    #[c2rust::src_loc = "82:1"]
    pub unsafe extern "C" fn ipc_type_name(
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
    use super::gtypes_h::{guint, gpointer, gboolean, gchar, gint};
    use super::garray_h::GPtrArray;
    use super::glibconfig_h::gsize;
    use super::giochannel_h::GIOChannel;
    use super::gqueue_h::GQueue;
}
#[c2rust::header_src = "/usr/include/string.h:21"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "43:14"]
        pub fn memcpy(
            _: *mut std::ffi::c_void,
            _: *const std::ffi::c_void,
            _: std::ffi::c_ulong,
        ) -> *mut std::ffi::c_void;
        #[c2rust::src_loc = "156:12"]
        pub fn strcmp(
            _: *const std::ffi::c_char,
            _: *const std::ffi::c_char,
        ) -> std::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gmem.h:21"]
pub mod gmem_h {
    use super::gtypes_h::gpointer;
    use super::glibconfig_h::gsize;
    extern "C" {
        #[c2rust::src_loc = "73:1"]
        pub fn g_free(mem: gpointer);
        #[c2rust::src_loc = "83:1"]
        pub fn g_malloc(n_bytes: gsize) -> gpointer;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gslice.h:21"]
pub mod gslice_h {
    use super::glibconfig_h::gsize;
    use super::gtypes_h::gpointer;
    extern "C" {
        #[c2rust::src_loc = "36:1"]
        pub fn g_slice_alloc0(block_size: gsize) -> gpointer;
        #[c2rust::src_loc = "41:1"]
        pub fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtestutils.h:21"]
pub mod gtestutils_h {
    extern "C" {
        #[c2rust::src_loc = "624:1"]
        pub fn g_assertion_message_expr(
            domain: *const std::ffi::c_char,
            file: *const std::ffi::c_char,
            line: std::ffi::c_int,
            func: *const std::ffi::c_char,
            expr: *const std::ffi::c_char,
        ) -> !;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/luaserialize.h:22"]
pub mod luaserialize_h {
    use super::lua_h::lua_State;
    use super::garray_h::GByteArray;
    use super::gtypes_h::gint;
    extern "C" {
        #[c2rust::src_loc = "25:1"]
        pub fn lua_serialize_range(
            L: *mut lua_State,
            out: *mut GByteArray,
            start: gint,
            end: gint,
        );
    }
}
pub use self::glibconfig_h::{guint8, guint32, gint64, gssize, gsize};
pub use self::gtypes_h::{gchar, gint, gboolean, guint, gpointer};
pub use self::garray_h::{
    _GByteArray, GByteArray, _GPtrArray, GPtrArray, g_ptr_array_new,
    g_ptr_array_sized_new, g_ptr_array_remove_fast, g_ptr_array_add, g_byte_array_new,
    g_byte_array_unref,
};
pub use self::gquark_h::GQuark;
pub use self::gerror_h::{_GError, GError, g_error_free};
pub use self::gthread_h::{GThreadFunc, GThread, g_thread_new};
pub use self::deprecated_gthread_h::{
    _GThread, GThreadPriority, G_THREAD_PRIORITY_URGENT, G_THREAD_PRIORITY_HIGH,
    G_THREAD_PRIORITY_NORMAL, G_THREAD_PRIORITY_LOW,
};
pub use self::gasyncqueue_h::{
    GAsyncQueue, _GAsyncQueue, g_async_queue_new, g_async_queue_push, g_async_queue_pop,
};
pub use self::gconvert_h::{GIConv, _GIConv};
pub use self::glist_h::{_GList, GList};
pub use self::gslist_h::{_GSList, GSList};
pub use self::gmain_h::{
    GIOCondition, G_IO_NVAL, G_IO_HUP, G_IO_ERR, G_IO_PRI, G_IO_OUT, G_IO_IN,
    GMainContext, _GSource, GSourcePrivate, GSource, GSourceFuncs, _GSourceFuncs,
    GSourceDummyMarshal, GSourceFunc, GSourceFuncsFinalizeFunc, GSourceFuncsDispatchFunc,
    GSourceFuncsCheckFunc, GSourceFuncsPrepareFunc, GSourceCallbackFuncs,
    _GSourceCallbackFuncs, _GMainContext, _GSourcePrivate, g_source_remove,
};
pub use self::gstring_h::{_GString, GString};
pub use self::giochannel_h::{
    _GIOChannel, GIOFuncs, _GIOFuncs, GIOChannel, GIOFlags, G_IO_FLAG_SET_MASK,
    G_IO_FLAG_GET_MASK, G_IO_FLAG_MASK, G_IO_FLAG_IS_SEEKABLE, G_IO_FLAG_IS_WRITEABLE,
    G_IO_FLAG_IS_WRITABLE, G_IO_FLAG_IS_READABLE, G_IO_FLAG_NONBLOCK, G_IO_FLAG_APPEND,
    G_IO_FLAG_NONE, GIOStatus, G_IO_STATUS_AGAIN, G_IO_STATUS_EOF, G_IO_STATUS_NORMAL,
    G_IO_STATUS_ERROR, GSeekType, G_SEEK_END, G_SEEK_SET, G_SEEK_CUR, GIOFunc,
    g_io_channel_shutdown, g_io_add_watch, g_io_channel_set_buffered,
    g_io_channel_set_encoding, g_io_channel_read_chars, g_io_channel_write_chars,
    g_io_channel_unix_new,
};
pub use self::gqueue_h::{
    _GQueue, GQueue, g_queue_new, g_queue_free, g_queue_is_empty, g_queue_push_tail,
    g_queue_pop_head,
};
use self::lua_h::lua_State;
pub use self::log_h::{
    log_level_t, LOG_LEVEL_debug, LOG_LEVEL_verbose, LOG_LEVEL_info, LOG_LEVEL_warn,
    LOG_LEVEL_error, LOG_LEVEL_fatal, _log,
};
pub use self::ipc_h::{
    ipc_type_t, IPC_TYPE_crash, IPC_TYPE_page_created, IPC_TYPE_log, IPC_TYPE_eval_js,
    IPC_TYPE_extension_init, IPC_TYPE_scroll, IPC_TYPE_lua_ipc,
    IPC_TYPE_lua_require_module, _ipc_header_t, ipc_header_t, _ipc_recv_state_t,
    ipc_recv_state_t, ipc_endpoint_status_t, IPC_ENDPOINT_FREED, IPC_ENDPOINT_CONNECTED,
    IPC_ENDPOINT_DISCONNECTED, _ipc_endpoint_t, ipc_endpoint_t, ipc_type_name,
};
use self::string_h::{memcpy, strcmp};
use self::gmem_h::{g_free, g_malloc};
use self::gslice_h::{g_slice_alloc0, g_slice_free1};
use self::gtestutils_h::g_assertion_message_expr;
use self::luaserialize_h::lua_serialize_range;
extern "C" {
    #[c2rust::src_loc = "28:5"]
    pub fn ipc_recv_crash(
        ipc: *mut ipc_endpoint_t,
        msg: *const std::ffi::c_void,
        length: guint,
    );
    #[c2rust::src_loc = "28:5"]
    pub fn ipc_recv_log(
        ipc: *mut ipc_endpoint_t,
        msg: *const std::ffi::c_void,
        length: guint,
    );
    #[c2rust::src_loc = "28:5"]
    pub fn ipc_recv_scroll(
        ipc: *mut ipc_endpoint_t,
        msg: *const std::ffi::c_void,
        length: guint,
    );
    #[c2rust::src_loc = "28:5"]
    pub fn ipc_recv_eval_js(
        ipc: *mut ipc_endpoint_t,
        msg: *const std::ffi::c_void,
        length: guint,
    );
    #[c2rust::src_loc = "28:5"]
    pub fn ipc_recv_extension_init(
        ipc: *mut ipc_endpoint_t,
        msg: *const std::ffi::c_void,
        length: guint,
    );
    #[c2rust::src_loc = "28:5"]
    pub fn ipc_recv_lua_ipc(
        ipc: *mut ipc_endpoint_t,
        msg: *const std::ffi::c_void,
        length: guint,
    );
    #[c2rust::src_loc = "28:5"]
    pub fn ipc_recv_lua_require_module(
        ipc: *mut ipc_endpoint_t,
        msg: *const std::ffi::c_void,
        length: guint,
    );
    #[c2rust::src_loc = "28:5"]
    pub fn ipc_recv_page_created(
        ipc: *mut ipc_endpoint_t,
        msg: *const std::ffi::c_void,
        length: guint,
    );
}
#[c2rust::src_loc = "43:1"]
pub type queued_ipc_t = _queued_ipc_t;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "43:16"]
pub struct _queued_ipc_t {
    pub header: ipc_header_t,
    pub ipc: *mut ipc_endpoint_t,
    pub payload: [std::ffi::c_char; 0],
}
#[c2rust::src_loc = "38:17"]
static mut send_thread: *mut GThread = 0 as *const GThread as *mut GThread;
#[c2rust::src_loc = "39:21"]
static mut send_queue: *mut GAsyncQueue = 0 as *const GAsyncQueue as *mut GAsyncQueue;
#[c2rust::src_loc = "41:19"]
static mut endpoints: *mut GPtrArray = 0 as *const GPtrArray as *mut GPtrArray;
#[no_mangle]
#[c2rust::src_loc = "49:1"]
pub unsafe extern "C" fn ipc_endpoints_get() -> *const GPtrArray {
    if endpoints.is_null() {
        endpoints = g_ptr_array_sized_new(1 as std::ffi::c_int as guint);
    }
    return endpoints;
}
#[c2rust::src_loc = "57:1"]
unsafe extern "C" fn ipc_dispatch(
    mut ipc: *mut ipc_endpoint_t,
    mut header: ipc_header_t,
    mut payload: gpointer,
) {
    if header.type_0 as std::ffi::c_uint
        != IPC_TYPE_log as std::ffi::c_int as std::ffi::c_uint
    {
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
            ipc_recv_lua_require_module(
                ipc,
                payload as *const std::ffi::c_void,
                header.length,
            );
        }
        2 => {
            ipc_recv_lua_ipc(ipc, payload as *const std::ffi::c_void, header.length);
        }
        4 => {
            ipc_recv_scroll(ipc, payload as *const std::ffi::c_void, header.length);
        }
        8 => {
            ipc_recv_extension_init(
                ipc,
                payload as *const std::ffi::c_void,
                header.length,
            );
        }
        16 => {
            ipc_recv_eval_js(ipc, payload as *const std::ffi::c_void, header.length);
        }
        32 => {
            ipc_recv_log(ipc, payload as *const std::ffi::c_void, header.length);
        }
        64 => {
            ipc_recv_page_created(
                ipc,
                payload as *const std::ffi::c_void,
                header.length,
            );
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
#[c2rust::src_loc = "72:1"]
unsafe extern "C" fn ipc_send_thread(mut UNUSED_user_data: gpointer) -> gpointer {
    while 0 as std::ffi::c_int == 0 {
        let mut out: *mut queued_ipc_t = g_async_queue_pop(send_queue)
            as *mut queued_ipc_t;
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
                b"Trying to send an ipc message, but the endpoint went away.\0"
                    as *const u8 as *const std::ffi::c_char,
            );
        }
        g_free(out as gpointer);
    }
    return 0 as *mut std::ffi::c_void;
}
#[no_mangle]
#[c2rust::src_loc = "99:1"]
pub unsafe extern "C" fn ipc_send(
    mut ipc: *mut ipc_endpoint_t,
    mut header: *const ipc_header_t,
    mut data: *const std::ffi::c_void,
) {
    if send_thread.is_null() {
        send_queue = g_async_queue_new();
        send_thread = g_thread_new(
            b"send_thread\0" as *const u8 as *const std::ffi::c_char,
            Some(ipc_send_thread as unsafe extern "C" fn(gpointer) -> gpointer),
            0 as *mut std::ffi::c_void,
        );
    }
    if ipc_endpoint_incref(ipc) == 0 {
        return;
    }
    if (*header).type_0 as std::ffi::c_uint
        != IPC_TYPE_log as std::ffi::c_int as std::ffi::c_uint
    {
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
        == (data == 0 as *mut std::ffi::c_void as *const std::ffi::c_void)
            as std::ffi::c_int
    {} else {
        g_assertion_message_expr(
            0 as *mut gchar,
            b"common/ipc.c\0" as *const u8 as *const std::ffi::c_char,
            115 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 9], &[std::ffi::c_char; 9]>(b"ipc_send\0"))
                .as_ptr(),
            b"(header->length == 0) == (data == NULL)\0" as *const u8
                as *const std::ffi::c_char,
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
#[c2rust::src_loc = "130:1"]
unsafe extern "C" fn ipc_recv_and_dispatch_or_enqueue(mut ipc: *mut ipc_endpoint_t) {
    if !ipc.is_null() {} else {
        g_assertion_message_expr(
            0 as *mut gchar,
            b"common/ipc.c\0" as *const u8 as *const std::ffi::c_char,
            133 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[std::ffi::c_char; 33],
            >(b"ipc_recv_and_dispatch_or_enqueue\0"))
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
            } else {};
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
                        b"g_io_channel_read_chars(): %s\0" as *const u8
                            as *const std::ffi::c_char,
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
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[std::ffi::c_char; 33],
                >(b"ipc_recv_and_dispatch_or_enqueue\0"))
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
#[c2rust::src_loc = "201:1"]
unsafe extern "C" fn ipc_recv(
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
#[c2rust::src_loc = "211:1"]
unsafe extern "C" fn ipc_hup(
    mut UNUSED_channel: *mut GIOChannel,
    mut UNUSED_cond: GIOCondition,
    mut ipc: *mut ipc_endpoint_t,
) -> gboolean {
    if (*ipc).status as std::ffi::c_uint
        == IPC_ENDPOINT_CONNECTED as std::ffi::c_int as std::ffi::c_uint
    {} else {
        g_assertion_message_expr(
            0 as *mut gchar,
            b"common/ipc.c\0" as *const u8 as *const std::ffi::c_char,
            214 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 8], &[std::ffi::c_char; 8]>(b"ipc_hup\0"))
                .as_ptr(),
            b"ipc->status == IPC_ENDPOINT_CONNECTED\0" as *const u8
                as *const std::ffi::c_char,
        );
    }
    if !((*ipc).channel).is_null() {} else {
        g_assertion_message_expr(
            0 as *mut gchar,
            b"common/ipc.c\0" as *const u8 as *const std::ffi::c_char,
            215 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 8], &[std::ffi::c_char; 8]>(b"ipc_hup\0"))
                .as_ptr(),
            b"ipc->channel\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    ipc_endpoint_decref(ipc);
    return (0 as std::ffi::c_int == 0) as std::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "220:1"]
pub unsafe extern "C" fn ipc_send_lua(
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
#[no_mangle]
#[c2rust::src_loc = "230:1"]
pub unsafe extern "C" fn ipc_endpoint_new(
    mut name: *const gchar,
) -> *mut ipc_endpoint_t {
    let mut ipc: *mut ipc_endpoint_t = g_slice_alloc0(
        ::core::mem::size_of::<ipc_endpoint_t>() as std::ffi::c_ulong,
    ) as *mut ipc_endpoint_t;
    (*ipc).name = name as *mut gchar;
    (*ipc).queue = g_queue_new();
    (*ipc).status = IPC_ENDPOINT_DISCONNECTED;
    (*ipc).refcount = 1 as std::ffi::c_int;
    (*ipc).creation_notified = 0 as std::ffi::c_int;
    return ipc;
}
#[no_mangle]
#[c2rust::src_loc = "244:1"]
pub unsafe extern "C" fn ipc_endpoint_incref(mut ipc: *mut ipc_endpoint_t) -> gboolean {
    let mut old: std::ffi::c_int = 0;
    loop {
        old = ({
            let mut gaig_temp: gint = 0;
            if 0 as std::ffi::c_int != 0 {
                (*ipc).refcount;
                (*ipc).refcount;
            } else {};
            *&mut gaig_temp = ::core::intrinsics::atomic_load_seqcst(
                &mut (*ipc).refcount as *mut gint,
            );
            gaig_temp
        });
        if old < 1 as std::ffi::c_int {
            return 0 as std::ffi::c_int;
        }
        if !(({
            let mut gaicae_oldval: gint = old;
            if 0 as std::ffi::c_int != 0 {
                (*ipc).refcount;
            } else {};
            let fresh0 = ::core::intrinsics::atomic_cxchg_seqcst_seqcst(
                &mut (*ipc).refcount as *mut gint,
                *(&mut gaicae_oldval as *mut gint as *mut std::ffi::c_void as *mut gint),
                old + 1 as std::ffi::c_int,
            );
            *(&mut gaicae_oldval as *mut gint as *mut std::ffi::c_void
                as *mut gint) = fresh0.0;
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
#[c2rust::src_loc = "257:1"]
unsafe extern "C" fn ipc_endpoint_incref_no_check(mut ipc: *mut ipc_endpoint_t) {
    if 0 as std::ffi::c_int != 0 {
        (*ipc).refcount;
        (*ipc).refcount;
    } else {};
    ::core::intrinsics::atomic_xadd_seqcst(&mut (*ipc).refcount, 1 as std::ffi::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "263:1"]
pub unsafe extern "C" fn ipc_endpoint_decref(mut ipc: *mut ipc_endpoint_t) {
    if ({
        if 0 as std::ffi::c_int != 0 {
            (*ipc).refcount;
            (*ipc).refcount;
        } else {};
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
            let mut msg: *mut queued_ipc_t = g_queue_pop_head((*ipc).queue)
                as *mut queued_ipc_t;
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
#[no_mangle]
#[c2rust::src_loc = "281:1"]
pub unsafe extern "C" fn ipc_endpoint_connect_to_socket(
    mut ipc: *mut ipc_endpoint_t,
    mut sock: std::ffi::c_int,
) {
    if !ipc.is_null() {} else {
        g_assertion_message_expr(
            0 as *mut gchar,
            b"common/ipc.c\0" as *const u8 as *const std::ffi::c_char,
            284 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[std::ffi::c_char; 31],
            >(b"ipc_endpoint_connect_to_socket\0"))
                .as_ptr(),
            b"ipc\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    if (*ipc).status as std::ffi::c_uint
        == IPC_ENDPOINT_DISCONNECTED as std::ffi::c_int as std::ffi::c_uint
    {} else {
        g_assertion_message_expr(
            0 as *mut gchar,
            b"common/ipc.c\0" as *const u8 as *const std::ffi::c_char,
            285 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[std::ffi::c_char; 31],
            >(b"ipc_endpoint_connect_to_socket\0"))
                .as_ptr(),
            b"ipc->status == IPC_ENDPOINT_DISCONNECTED\0" as *const u8
                as *const std::ffi::c_char,
        );
    }
    let mut state: *mut ipc_recv_state_t = &mut (*ipc).recv_state;
    (*state).queued_ipcs = g_ptr_array_new();
    let mut channel: *mut GIOChannel = g_io_channel_unix_new(sock);
    g_io_channel_set_encoding(channel, 0 as *const gchar, 0 as *mut *mut GError);
    g_io_channel_set_buffered(channel, 0 as std::ffi::c_int);
    (*state)
        .watch_in_id = g_io_add_watch(
        channel,
        G_IO_IN,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut GIOChannel,
                    GIOCondition,
                    *mut ipc_endpoint_t,
                ) -> gboolean,
            >,
            GIOFunc,
        >(
            Some(
                ipc_recv
                    as unsafe extern "C" fn(
                        *mut GIOChannel,
                        GIOCondition,
                        *mut ipc_endpoint_t,
                    ) -> gboolean,
            ),
        ),
        ipc as gpointer,
    );
    (*state)
        .watch_hup_id = g_io_add_watch(
        channel,
        G_IO_HUP,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut GIOChannel,
                    GIOCondition,
                    *mut ipc_endpoint_t,
                ) -> gboolean,
            >,
            GIOFunc,
        >(
            Some(
                ipc_hup
                    as unsafe extern "C" fn(
                        *mut GIOChannel,
                        GIOCondition,
                        *mut ipc_endpoint_t,
                    ) -> gboolean,
            ),
        ),
        ipc as gpointer,
    );
    let mut gaps_temp_atomic: *mut *mut GIOChannel = &mut (*ipc).channel;
    let mut gaps_temp_newval: *mut GIOChannel = channel;
    if 0 as std::ffi::c_int != 0 {
        (*ipc).channel;
    } else {};
    ::core::intrinsics::atomic_store_seqcst(gaps_temp_atomic, *&mut gaps_temp_newval);
    (*ipc).status = IPC_ENDPOINT_CONNECTED;
    if endpoints.is_null() {
        endpoints = g_ptr_array_sized_new(1 as std::ffi::c_int as guint);
    }
    if g_ptr_array_remove_fast(endpoints, ipc as gpointer) == 0 {} else {
        g_assertion_message_expr(
            0 as *mut gchar,
            b"common/ipc.c\0" as *const u8 as *const std::ffi::c_char,
            308 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[std::ffi::c_char; 31],
            >(b"ipc_endpoint_connect_to_socket\0"))
                .as_ptr(),
            b"!g_ptr_array_remove_fast(endpoints, ipc)\0" as *const u8
                as *const std::ffi::c_char,
        );
    }
    g_ptr_array_add(endpoints, ipc as gpointer);
}
#[no_mangle]
#[c2rust::src_loc = "312:1"]
pub unsafe extern "C" fn ipc_endpoint_replace(
    mut orig: *mut ipc_endpoint_t,
    mut new: *mut ipc_endpoint_t,
) -> *mut ipc_endpoint_t {
    if !orig.is_null() {} else {
        g_assertion_message_expr(
            0 as *mut gchar,
            b"common/ipc.c\0" as *const u8 as *const std::ffi::c_char,
            315 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 21],
                &[std::ffi::c_char; 21],
            >(b"ipc_endpoint_replace\0"))
                .as_ptr(),
            b"orig\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    if !new.is_null() {} else {
        g_assertion_message_expr(
            0 as *mut gchar,
            b"common/ipc.c\0" as *const u8 as *const std::ffi::c_char,
            316 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 21],
                &[std::ffi::c_char; 21],
            >(b"ipc_endpoint_replace\0"))
                .as_ptr(),
            b"new\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    if (*orig).status as std::ffi::c_uint
        == IPC_ENDPOINT_DISCONNECTED as std::ffi::c_int as std::ffi::c_uint
    {} else {
        g_assertion_message_expr(
            0 as *mut gchar,
            b"common/ipc.c\0" as *const u8 as *const std::ffi::c_char,
            317 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 21],
                &[std::ffi::c_char; 21],
            >(b"ipc_endpoint_replace\0"))
                .as_ptr(),
            b"orig->status == IPC_ENDPOINT_DISCONNECTED\0" as *const u8
                as *const std::ffi::c_char,
        );
    }
    if (*new).status as std::ffi::c_uint
        == IPC_ENDPOINT_CONNECTED as std::ffi::c_int as std::ffi::c_uint
    {} else {
        g_assertion_message_expr(
            0 as *mut gchar,
            b"common/ipc.c\0" as *const u8 as *const std::ffi::c_char,
            318 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 21],
                &[std::ffi::c_char; 21],
            >(b"ipc_endpoint_replace\0"))
                .as_ptr(),
            b"new->status == IPC_ENDPOINT_CONNECTED\0" as *const u8
                as *const std::ffi::c_char,
        );
    }
    ipc_endpoint_incref_no_check(new);
    if !((*orig).queue).is_null() {
        while g_queue_is_empty((*orig).queue) == 0 {
            let mut msg: *mut queued_ipc_t = g_queue_pop_head((*orig).queue)
                as *mut queued_ipc_t;
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
#[no_mangle]
#[c2rust::src_loc = "341:1"]
pub unsafe extern "C" fn ipc_endpoint_disconnect(mut ipc: *mut ipc_endpoint_t) {
    if (*ipc).status as std::ffi::c_uint
        == IPC_ENDPOINT_CONNECTED as std::ffi::c_int as std::ffi::c_uint
    {} else {
        g_assertion_message_expr(
            0 as *mut gchar,
            b"common/ipc.c\0" as *const u8 as *const std::ffi::c_char,
            344 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 24],
                &[std::ffi::c_char; 24],
            >(b"ipc_endpoint_disconnect\0"))
                .as_ptr(),
            b"ipc->status == IPC_ENDPOINT_CONNECTED\0" as *const u8
                as *const std::ffi::c_char,
        );
    }
    if !((*ipc).channel).is_null() {} else {
        g_assertion_message_expr(
            0 as *mut gchar,
            b"common/ipc.c\0" as *const u8 as *const std::ffi::c_char,
            345 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 24],
                &[std::ffi::c_char; 24],
            >(b"ipc_endpoint_disconnect\0"))
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
