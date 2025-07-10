use ::libc;
use ::c2rust_bitfields;

pub mod __stddef_ptrdiff_t_h {

    pub type ptrdiff_t = std::ffi::c_long;
}

pub mod __stddef_size_t_h {

    pub type size_t = std::ffi::c_ulong;
}

pub mod glibconfig_h {

    pub type guint8 = std::ffi::c_uchar;

    pub type guint32 = std::ffi::c_uint;

    pub type gint64 = std::ffi::c_long;

    pub type guint64 = std::ffi::c_ulong;

    pub type gsize = std::ffi::c_ulong;
}

pub mod types_h {

    pub type __pid_t = std::ffi::c_int;

    pub type __socklen_t = std::ffi::c_uint;
}

pub mod time_h {

    pub type pid_t = __pid_t;
    use super::types_h::__pid_t;
}

pub mod gtypes_h {

    pub type gchar = std::ffi::c_char;

    pub type glong = std::ffi::c_long;

    pub type gint = std::ffi::c_int;

    pub type gboolean = gint;

    pub type gulong = std::ffi::c_ulong;

    pub type guint = std::ffi::c_uint;

    pub type gfloat = std::ffi::c_float;

    pub type gdouble = std::ffi::c_double;

    pub type gpointer = *mut std::ffi::c_void;

    pub type GFunc = Option::<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
}

pub mod garray_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GPtrArray {
        pub pdata: *mut gpointer,
        pub len: guint,
    }

    pub type GPtrArray = _GPtrArray;
    use super::gtypes_h::{gpointer, guint, gboolean, GFunc};
    extern "C" {

        pub fn g_ptr_array_sized_new(reserved_size: guint) -> *mut GPtrArray;

        pub fn g_ptr_array_free(
            array: *mut GPtrArray,
            free_segment: gboolean,
        ) -> *mut gpointer;

        pub fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);

        pub fn g_ptr_array_foreach(
            array: *mut GPtrArray,
            func: GFunc,
            user_data: gpointer,
        );
    }
}

pub mod gquark_h {

    pub type GQuark = guint32;
    use super::glibconfig_h::guint32;
}

pub mod gerror_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GError {
        pub domain: GQuark,
        pub code: gint,
        pub message: *mut gchar,
    }

    pub type GError = _GError;
    use super::gquark_h::GQuark;
    use super::gtypes_h::{gint, gchar};
}

pub mod gconvert_h {

    pub type GIConv = *mut _GIConv;
    extern "C" {

        pub type _GIConv;
    }
}

pub mod gdataset_h {

    pub type GData = _GData;
    extern "C" {

        pub type _GData;
    }
}

pub mod glist_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GList {
        pub data: gpointer,
        pub next: *mut GList,
        pub prev: *mut GList,
    }

    pub type GList = _GList;
    use super::gtypes_h::gpointer;
}

pub mod gslist_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GSList {
        pub data: gpointer,
        pub next: *mut GSList,
    }

    pub type GSList = _GSList;
    use super::gtypes_h::gpointer;
}

pub mod gmain_h {

    pub type GIOCondition = std::ffi::c_uint;

    pub const G_IO_NVAL: GIOCondition = 32;

    pub const G_IO_HUP: GIOCondition = 16;

    pub const G_IO_ERR: GIOCondition = 8;

    pub const G_IO_PRI: GIOCondition = 2;

    pub const G_IO_OUT: GIOCondition = 4;

    pub const G_IO_IN: GIOCondition = 1;

    pub type GMainContext = _GMainContext;
    #[derive(Copy, Clone)]
    #[repr(C)]

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

    pub type GSourcePrivate = _GSourcePrivate;

    pub type GSource = _GSource;

    pub type GSourceFuncs = _GSourceFuncs;
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GSourceFuncs {
        pub prepare: GSourceFuncsPrepareFunc,
        pub check: GSourceFuncsCheckFunc,
        pub dispatch: GSourceFuncsDispatchFunc,
        pub finalize: GSourceFuncsFinalizeFunc,
        pub closure_callback: GSourceFunc,
        pub closure_marshal: GSourceDummyMarshal,
    }

    pub type GSourceDummyMarshal = Option::<unsafe extern "C" fn() -> ()>;

    pub type GSourceFunc = Option::<unsafe extern "C" fn(gpointer) -> gboolean>;

    pub type GSourceFuncsFinalizeFunc = Option::<
        unsafe extern "C" fn(*mut GSource) -> (),
    >;

    pub type GSourceFuncsDispatchFunc = Option::<
        unsafe extern "C" fn(*mut GSource, GSourceFunc, gpointer) -> gboolean,
    >;

    pub type GSourceFuncsCheckFunc = Option::<
        unsafe extern "C" fn(*mut GSource) -> gboolean,
    >;

    pub type GSourceFuncsPrepareFunc = Option::<
        unsafe extern "C" fn(*mut GSource, *mut gint) -> gboolean,
    >;

    pub type GSourceCallbackFuncs = _GSourceCallbackFuncs;
    #[derive(Copy, Clone)]
    #[repr(C)]

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

        pub type _GMainContext;

        pub type _GSourcePrivate;
    }
}

pub mod gstring_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GString {
        pub str_0: *mut gchar,
        pub len: gsize,
        pub allocated_len: gsize,
    }

    pub type GString = _GString;
    use super::gtypes_h::gchar;
    use super::glibconfig_h::gsize;
}

pub mod giochannel_h {
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]

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

    pub type GIOFuncs = _GIOFuncs;
    #[derive(Copy, Clone)]
    #[repr(C)]

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

    pub type GIOChannel = _GIOChannel;

    pub type GIOFlags = std::ffi::c_uint;

    pub const G_IO_FLAG_SET_MASK: GIOFlags = 3;

    pub const G_IO_FLAG_GET_MASK: GIOFlags = 31;

    pub const G_IO_FLAG_MASK: GIOFlags = 31;

    pub const G_IO_FLAG_IS_SEEKABLE: GIOFlags = 16;

    pub const G_IO_FLAG_IS_WRITEABLE: GIOFlags = 8;

    pub const G_IO_FLAG_IS_WRITABLE: GIOFlags = 8;

    pub const G_IO_FLAG_IS_READABLE: GIOFlags = 4;

    pub const G_IO_FLAG_NONBLOCK: GIOFlags = 2;

    pub const G_IO_FLAG_APPEND: GIOFlags = 1;

    pub const G_IO_FLAG_NONE: GIOFlags = 0;

    pub type GIOStatus = std::ffi::c_uint;

    pub const G_IO_STATUS_AGAIN: GIOStatus = 3;

    pub const G_IO_STATUS_EOF: GIOStatus = 2;

    pub const G_IO_STATUS_NORMAL: GIOStatus = 1;

    pub const G_IO_STATUS_ERROR: GIOStatus = 0;

    pub type GSeekType = std::ffi::c_uint;

    pub const G_SEEK_END: GSeekType = 2;

    pub const G_SEEK_SET: GSeekType = 1;

    pub const G_SEEK_CUR: GSeekType = 0;
    use super::gtypes_h::{gint, gchar, guint, gpointer};
    use super::gconvert_h::GIConv;
    use super::glibconfig_h::{gsize, gint64};
    use super::gstring_h::GString;
    use super::gerror_h::GError;
    use super::gmain_h::{GSource, GIOCondition};
}

pub mod gqueue_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GQueue {
        pub head: *mut GList,
        pub tail: *mut GList,
        pub length: guint,
    }

    pub type GQueue = _GQueue;
    use super::glist_h::GList;
    use super::gtypes_h::guint;
}

pub mod gtype_h {

    pub type GType = gsize;

    pub type GValue = _GValue;
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GTypeClass {
        pub g_type: GType,
    }

    pub type GTypeClass = _GTypeClass;
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GTypeInstance {
        pub g_class: *mut GTypeClass,
    }

    pub type GTypeInstance = _GTypeInstance;
    use super::glibconfig_h::gsize;
    use super::gvalue_h::_GValue;
}

pub mod gvalue_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GValue {
        pub g_type: GType,
        pub data: [C2RustUnnamed; 2],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub union C2RustUnnamed {
        pub v_int: gint,
        pub v_uint: guint,
        pub v_long: glong,
        pub v_ulong: gulong,
        pub v_int64: gint64,
        pub v_uint64: guint64,
        pub v_float: gfloat,
        pub v_double: gdouble,
        pub v_pointer: gpointer,
    }
    use super::gtype_h::GType;
    use super::gtypes_h::{gint, guint, glong, gulong, gfloat, gdouble, gpointer};
    use super::glibconfig_h::{gint64, guint64};
}

pub mod gclosure_h {
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]

    pub struct _GClosure {
        #[bitfield(name = "ref_count", ty = "guint", bits = "0..=14")]
        #[bitfield(name = "meta_marshal_nouse", ty = "guint", bits = "15..=15")]
        #[bitfield(name = "n_guards", ty = "guint", bits = "16..=16")]
        #[bitfield(name = "n_fnotifiers", ty = "guint", bits = "17..=18")]
        #[bitfield(name = "n_inotifiers", ty = "guint", bits = "19..=26")]
        #[bitfield(name = "in_inotify", ty = "guint", bits = "27..=27")]
        #[bitfield(name = "floating", ty = "guint", bits = "28..=28")]
        #[bitfield(name = "derivative_flag", ty = "guint", bits = "29..=29")]
        #[bitfield(name = "in_marshal", ty = "guint", bits = "30..=30")]
        #[bitfield(name = "is_invalid", ty = "guint", bits = "31..=31")]
        pub ref_count_meta_marshal_nouse_n_guards_n_fnotifiers_n_inotifiers_in_inotify_floating_derivative_flag_in_marshal_is_invalid: [u8; 4],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 4],
        pub marshal: Option::<
            unsafe extern "C" fn(
                *mut GClosure,
                *mut GValue,
                guint,
                *const GValue,
                gpointer,
                gpointer,
            ) -> (),
        >,
        pub data: gpointer,
        pub notifiers: *mut GClosureNotifyData,
    }

    pub type GClosureNotifyData = _GClosureNotifyData;
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GClosureNotifyData {
        pub data: gpointer,
        pub notify: GClosureNotify,
    }

    pub type GClosureNotify = Option::<
        unsafe extern "C" fn(gpointer, *mut GClosure) -> (),
    >;

    pub type GClosure = _GClosure;

    pub type GCallback = Option::<unsafe extern "C" fn() -> ()>;
    use super::gtypes_h::{guint, gpointer};
    use super::gtype_h::GValue;
}

pub mod gsignal_h {

    pub type GConnectFlags = std::ffi::c_uint;

    pub const G_CONNECT_SWAPPED: GConnectFlags = 2;

    pub const G_CONNECT_AFTER: GConnectFlags = 1;

    pub const G_CONNECT_DEFAULT: GConnectFlags = 0;
    use super::gtypes_h::{gpointer, gchar, gulong};
    use super::gclosure_h::{GCallback, GClosureNotify};
    extern "C" {

        pub fn g_signal_connect_data(
            instance: gpointer,
            detailed_signal: *const gchar,
            c_handler: GCallback,
            data: gpointer,
            destroy_data: GClosureNotify,
            connect_flags: GConnectFlags,
        ) -> gulong;
    }
}

pub mod gobject_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GObject {
        pub g_type_instance: GTypeInstance,
        pub ref_count: guint,
        pub qdata: *mut GData,
    }

    pub type GObject = _GObject;
    use super::gtype_h::GTypeInstance;
    use super::gtypes_h::{guint, gpointer};
    use super::gdataset_h::GData;
    extern "C" {

        pub fn g_object_unref(object: gpointer);
    }
}

pub mod JSCContext_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _JSCContext {
        pub parent: GObject,
        pub priv_0: *mut JSCContextPrivate,
    }

    pub type JSCContextPrivate = _JSCContextPrivate;
    use super::gobject_h::GObject;
    extern "C" {

        pub type _JSCContextPrivate;
    }
}

pub mod JSCValue_h {

    pub type JSCContext = _JSCContext;
    use super::JSCContext_h::_JSCContext;
}

pub mod socket_h {

    pub type socklen_t = __socklen_t;
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct sockaddr {
        pub sa_family: sa_family_t,
        pub sa_data: [std::ffi::c_char; 14],
    }

    pub const PF_LOCAL: std::ffi::c_int = 1 as std::ffi::c_int;

    pub const PF_UNIX: std::ffi::c_int = 1 as std::ffi::c_int;

    pub const AF_UNIX: std::ffi::c_int = 1 as std::ffi::c_int;
    use super::types_h::__socklen_t;
    use super::sockaddr_h::sa_family_t;
}

pub mod socket_type_h {

    pub type __socket_type = std::ffi::c_uint;

    pub const SOCK_NONBLOCK: __socket_type = 2048;

    pub const SOCK_CLOEXEC: __socket_type = 524288;

    pub const SOCK_PACKET: __socket_type = 10;

    pub const SOCK_DCCP: __socket_type = 6;

    pub const SOCK_SEQPACKET: __socket_type = 5;

    pub const SOCK_RDM: __socket_type = 4;

    pub const SOCK_RAW: __socket_type = 3;

    pub const SOCK_DGRAM: __socket_type = 2;

    pub const SOCK_STREAM: __socket_type = 1;

    pub const SOCK_STREAM_0: std::ffi::c_int = SOCK_STREAM as std::ffi::c_int;
}

pub mod sockaddr_h {

    pub type sa_family_t = std::ffi::c_ushort;
}

pub mod un_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct sockaddr_un {
        pub sun_family: sa_family_t,
        pub sun_path: [std::ffi::c_char; 108],
    }
    use super::sockaddr_h::sa_family_t;
}

pub mod WebKitScriptWorld_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _WebKitScriptWorld {
        pub parent: GObject,
        pub priv_0: *mut WebKitScriptWorldPrivate,
    }

    pub type WebKitScriptWorldPrivate = _WebKitScriptWorldPrivate;

    pub type WebKitScriptWorld = _WebKitScriptWorld;
    use super::gobject_h::GObject;
    extern "C" {

        pub type _WebKitScriptWorldPrivate;

        pub fn webkit_script_world_get_default() -> *mut WebKitScriptWorld;
    }
}

pub mod WebKitFrame_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _WebKitFrame {
        pub parent: GObject,
        pub priv_0: *mut WebKitFramePrivate,
    }

    pub type WebKitFramePrivate = _WebKitFramePrivate;

    pub type WebKitFrame = _WebKitFrame;
    use super::gobject_h::GObject;
    use super::WebKitScriptWorld_h::WebKitScriptWorld;
    use super::JSCValue_h::JSCContext;
    extern "C" {

        pub type _WebKitFramePrivate;

        pub fn webkit_frame_get_js_context_for_script_world(
            frame: *mut WebKitFrame,
            world: *mut WebKitScriptWorld,
        ) -> *mut JSCContext;
    }
}

pub mod WebKitWebPage_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _WebKitWebPage {
        pub parent: GObject,
        pub priv_0: *mut WebKitWebPagePrivate,
    }

    pub type WebKitWebPagePrivate = _WebKitWebPagePrivate;

    pub type WebKitWebPage = _WebKitWebPage;
    use super::gobject_h::GObject;
    use super::glibconfig_h::guint64;
    use super::WebKitFrame_h::WebKitFrame;
    extern "C" {

        pub type _WebKitWebPagePrivate;

        pub fn webkit_web_page_get_id(web_page: *mut WebKitWebPage) -> guint64;

        pub fn webkit_web_page_get_main_frame(
            web_page: *mut WebKitWebPage,
        ) -> *mut WebKitFrame;
    }
}

pub mod WebKitWebExtension_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _WebKitWebExtension {
        pub parent: GObject,
        pub priv_0: *mut WebKitWebExtensionPrivate,
    }

    pub type WebKitWebExtensionPrivate = _WebKitWebExtensionPrivate;

    pub type WebKitWebExtension = _WebKitWebExtension;
    use super::gobject_h::GObject;
    use super::glibconfig_h::guint64;
    use super::WebKitWebPage_h::WebKitWebPage;
    extern "C" {

        pub type _WebKitWebExtensionPrivate;

        pub fn webkit_web_extension_get_page(
            extension_0: *mut WebKitWebExtension,
            page_id: guint64,
        ) -> *mut WebKitWebPage;
    }
}

pub mod lua_h {

    pub type lua_CFunction = Option::<
        unsafe extern "C" fn(*mut lua_State) -> std::ffi::c_int,
    >;

    pub type lua_Integer = ptrdiff_t;

    pub const LUA_GLOBALSINDEX: std::ffi::c_int = -(10002 as std::ffi::c_int);
    use super::__stddef_ptrdiff_t_h::ptrdiff_t;
    use super::__stddef_size_t_h::size_t;
    extern "C" {

        pub type lua_State;

        pub fn lua_gettop(L: *mut lua_State) -> std::ffi::c_int;

        pub fn lua_settop(L: *mut lua_State, idx: std::ffi::c_int);

        pub fn lua_remove(L: *mut lua_State, idx: std::ffi::c_int);

        pub fn lua_insert(L: *mut lua_State, idx: std::ffi::c_int);

        pub fn lua_tointeger(L: *mut lua_State, idx: std::ffi::c_int) -> lua_Integer;

        pub fn lua_toboolean(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;

        pub fn lua_tolstring(
            L: *mut lua_State,
            idx: std::ffi::c_int,
            len: *mut size_t,
        ) -> *const std::ffi::c_char;

        pub fn lua_pushstring(L: *mut lua_State, s: *const std::ffi::c_char);

        pub fn lua_pushcclosure(
            L: *mut lua_State,
            fn_0: lua_CFunction,
            n: std::ffi::c_int,
        );

        pub fn lua_getfield(
            L: *mut lua_State,
            idx: std::ffi::c_int,
            k: *const std::ffi::c_char,
        );

        pub fn lua_pcall(
            L: *mut lua_State,
            nargs: std::ffi::c_int,
            nresults: std::ffi::c_int,
            errfunc: std::ffi::c_int,
        ) -> std::ffi::c_int;
    }
}

pub mod log_h {

    pub type log_level_t = std::ffi::c_uint;

    pub const LOG_LEVEL_debug: log_level_t = 5;

    pub const LOG_LEVEL_verbose: log_level_t = 4;

    pub const LOG_LEVEL_info: log_level_t = 3;

    pub const LOG_LEVEL_warn: log_level_t = 2;

    pub const LOG_LEVEL_error: log_level_t = 1;

    pub const LOG_LEVEL_fatal: log_level_t = 0;
    use super::gtypes_h::gchar;
    extern "C" {

        pub fn _log(lvl: log_level_t, _: *const gchar, _: *const gchar, _: ...);
    }
}

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

    pub struct _ipc_lua_require_module_t {
        pub module_name: [gchar; 0],
    }

    pub type ipc_lua_require_module_t = _ipc_lua_require_module_t;
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _ipc_lua_ipc_t {
        pub arg: [gchar; 0],
    }

    pub type ipc_lua_ipc_t = _ipc_lua_ipc_t;
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
    use super::gtypes_h::{guint, gchar, gpointer, gboolean, gint};
    use super::glibconfig_h::{guint64, gsize};
    use super::time_h::pid_t;
    use super::garray_h::GPtrArray;
    use super::giochannel_h::GIOChannel;
    use super::gqueue_h::GQueue;
    use super::lua_h::lua_State;
    extern "C" {

        pub fn ipc_endpoint_connect_to_socket(
            ipc: *mut ipc_endpoint_t,
            sock: std::ffi::c_int,
        );

        pub fn ipc_send_lua(
            ipc: *mut ipc_endpoint_t,
            type_0: ipc_type_t,
            L: *mut lua_State,
            start: gint,
            end: gint,
        );

        pub fn ipc_send(
            ipc: *mut ipc_endpoint_t,
            header: *const ipc_header_t,
            data: *const std::ffi::c_void,
        );
    }
}

pub mod extension_h {

    pub type extension_t = _extension_t;
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _extension_t {
        pub ext: *mut WebKitWebExtension,
        pub ipc: *mut ipc_endpoint_t,
        pub script_world: *mut WebKitScriptWorld,
    }
    use super::WebKitWebExtension_h::WebKitWebExtension;
    use super::ipc_h::ipc_endpoint_t;
    use super::WebKitScriptWorld_h::WebKitScriptWorld;
    extern "C" {

        pub static mut extension: extension_t;
    }
}

pub mod common_h {

    pub type common_t = _common_t;
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _common_t {
        pub L: *mut lua_State,
    }
    use super::lua_h::lua_State;
    extern "C" {

        pub static mut common: common_t;
    }
}

pub mod string_h {
    extern "C" {

        pub fn memset(
            _: *mut std::ffi::c_void,
            _: std::ffi::c_int,
            _: std::ffi::c_ulong,
        ) -> *mut std::ffi::c_void;

        pub fn strcpy(
            _: *mut std::ffi::c_char,
            _: *const std::ffi::c_char,
        ) -> *mut std::ffi::c_char;

        pub fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    }
}

pub mod signal_h {
    extern "C" {

        pub fn raise(__sig: std::ffi::c_int) -> std::ffi::c_int;
    }
}

pub mod gmessages_h {

    pub const G_LOG_DOMAIN: std::ffi::c_int = 0 as std::ffi::c_int;
}

pub mod gtestutils_h {
    use super::glibconfig_h::guint64;
    extern "C" {

        pub fn g_assertion_message_cmpint(
            domain: *const std::ffi::c_char,
            file: *const std::ffi::c_char,
            line: std::ffi::c_int,
            func: *const std::ffi::c_char,
            expr: *const std::ffi::c_char,
            arg1: guint64,
            cmp: *const std::ffi::c_char,
            arg2: guint64,
            numtype: std::ffi::c_char,
        );
    }
}

pub mod stdio_h {
    extern "C" {

        pub fn perror(__s: *const std::ffi::c_char);
    }
}

pub mod assert_h {

    pub const __ASSERT_FUNCTION: [std::ffi::c_char; 92] = unsafe {
        *::core::mem::transmute::<
            &[u8; 92],
            &[std::ffi::c_char; 92],
        >(
            b"void ipc_recv_lua_require_module(ipc_endpoint_t *, const ipc_lua_require_module_t *, guint)\0",
        )
    };
    extern "C" {

        pub fn __assert_fail(
            __assertion: *const std::ffi::c_char,
            __file: *const std::ffi::c_char,
            __line: std::ffi::c_uint,
            __function: *const std::ffi::c_char,
        ) -> !;
    }
}

pub mod sys_socket_h {
    use super::socket_h::{sockaddr, socklen_t};
    extern "C" {

        pub fn socket(
            __domain: std::ffi::c_int,
            __type: std::ffi::c_int,
            __protocol: std::ffi::c_int,
        ) -> std::ffi::c_int;

        pub fn connect(
            __fd: std::ffi::c_int,
            __addr: *const sockaddr,
            __len: socklen_t,
        ) -> std::ffi::c_int;
    }
}

pub mod unistd_h {
    use super::types_h::__pid_t;
    extern "C" {

        pub fn close(__fd: std::ffi::c_int) -> std::ffi::c_int;

        pub fn getpid() -> __pid_t;
    }
}

pub mod luakit_h {
    use super::lua_h::lua_State;
    extern "C" {

        pub fn luakit_lib_emit_pending_signals(L: *mut lua_State);
    }
}

pub mod scroll_h {
    use super::glibconfig_h::guint64;
    use super::gtypes_h::gint;
    extern "C" {

        pub fn web_scroll_to(page_id: guint64, scroll_x: gint, scroll_y: gint);
    }
}

pub mod luajs_h {
    use super::lua_h::lua_State;
    use super::JSCValue_h::JSCContext;
    use super::gtypes_h::guint;
    extern "C" {

        pub fn luajs_eval_js(
            L: *mut lua_State,
            ctx: *mut JSCContext,
            code: *const std::ffi::c_char,
            source: *const std::ffi::c_char,
            line: guint,
            no_return: bool,
        ) -> std::ffi::c_int;
    }
}

pub mod luaserialize_h {
    use super::lua_h::lua_State;
    use super::glibconfig_h::guint8;
    use super::gtypes_h::guint;
    extern "C" {

        pub fn lua_deserialize_range(
            L: *mut lua_State,
            in_0: *const guint8,
            length: guint,
        ) -> std::ffi::c_int;
    }
}

pub mod luautil_h {
    use super::lua_h::lua_State;
    use super::gtypes_h::gint;
    extern "C" {

        pub fn luaH_dofunction_on_error(L: *mut lua_State) -> gint;
    }
}

pub mod lualib_h {
    #[inline]

    pub unsafe extern "C" fn luaH_dofunction(
        mut L: *mut lua_State,
        mut nargs: gint,
        mut nret: gint,
    ) -> gboolean {
        lua_insert(L, -nargs - 1 as std::ffi::c_int);
        lua_pushcclosure(
            L,
            Some(
                luaH_dofunction_on_error as unsafe extern "C" fn(*mut lua_State) -> gint,
            ),
            0 as std::ffi::c_int,
        );
        lua_insert(L, -nargs - 2 as std::ffi::c_int);
        let mut error_func_pos = lua_gettop(L) - nargs - 1 as std::ffi::c_int;
        if lua_pcall(L, nargs, nret, -nargs - 2 as std::ffi::c_int) != 0 {
            _log(
                LOG_LEVEL_error,
                b"./common/lualib.h\0" as *const u8 as *const std::ffi::c_char,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                lua_tolstring(L, -(1 as std::ffi::c_int), NULL as *mut size_t),
            );
            lua_settop(L, -(2 as std::ffi::c_int) - 1 as std::ffi::c_int);
            return FALSE;
        }
        lua_remove(L, error_func_pos);
        return TRUE;
    }
    use super::lua_h::{
        lua_State, lua_insert, lua_pushcclosure, lua_gettop, lua_pcall, lua_tolstring,
        lua_settop, lua_remove,
    };
    use super::gtypes_h::{gint, gboolean};
    use super::luautil_h::luaH_dofunction_on_error;
    use super::log_h::{_log, LOG_LEVEL_error, log_level_t};
    use super::__stddef_null_h::NULL;
    use super::__stddef_size_t_h::size_t;
    use super::gmacros_h::{FALSE, TRUE};
}

pub mod clib_ipc_h {
    use super::lua_h::lua_State;
    use super::gtypes_h::{gchar, guint};
    extern "C" {

        pub fn ipc_channel_recv(L: *mut lua_State, arg: *const gchar, arglen: guint);
    }
}

pub mod gmacros_h {

    pub const FALSE: std::ffi::c_int = 0 as std::ffi::c_int;

    pub const TRUE: std::ffi::c_int = (FALSE == 0) as std::ffi::c_int;
}

pub mod __stddef_null_h {

    pub const NULL: std::ffi::c_int = 0 as std::ffi::c_int;
}

pub mod signum_generic_h {

    pub const SIGKILL: std::ffi::c_int = 9 as std::ffi::c_int;
}
pub use self::__stddef_ptrdiff_t_h::ptrdiff_t;
pub use self::__stddef_size_t_h::size_t;
pub use self::glibconfig_h::{guint8, guint32, gint64, guint64, gsize};
pub use self::types_h::{__pid_t, __socklen_t};
pub use self::time_h::pid_t;
pub use self::gtypes_h::{
    gchar, glong, gint, gboolean, gulong, guint, gfloat, gdouble, gpointer, GFunc,
};
pub use self::garray_h::{
    _GPtrArray, GPtrArray, g_ptr_array_sized_new, g_ptr_array_free, g_ptr_array_add,
    g_ptr_array_foreach,
};
pub use self::gquark_h::GQuark;
pub use self::gerror_h::{_GError, GError};
pub use self::gconvert_h::{GIConv, _GIConv};
pub use self::gdataset_h::{GData, _GData};
pub use self::glist_h::{_GList, GList};
pub use self::gslist_h::{_GSList, GSList};
pub use self::gmain_h::{
    GIOCondition, G_IO_NVAL, G_IO_HUP, G_IO_ERR, G_IO_PRI, G_IO_OUT, G_IO_IN,
    GMainContext, _GSource, GSourcePrivate, GSource, GSourceFuncs, _GSourceFuncs,
    GSourceDummyMarshal, GSourceFunc, GSourceFuncsFinalizeFunc, GSourceFuncsDispatchFunc,
    GSourceFuncsCheckFunc, GSourceFuncsPrepareFunc, GSourceCallbackFuncs,
    _GSourceCallbackFuncs, _GMainContext, _GSourcePrivate,
};
pub use self::gstring_h::{_GString, GString};
pub use self::giochannel_h::{
    _GIOChannel, GIOFuncs, _GIOFuncs, GIOChannel, GIOFlags, G_IO_FLAG_SET_MASK,
    G_IO_FLAG_GET_MASK, G_IO_FLAG_MASK, G_IO_FLAG_IS_SEEKABLE, G_IO_FLAG_IS_WRITEABLE,
    G_IO_FLAG_IS_WRITABLE, G_IO_FLAG_IS_READABLE, G_IO_FLAG_NONBLOCK, G_IO_FLAG_APPEND,
    G_IO_FLAG_NONE, GIOStatus, G_IO_STATUS_AGAIN, G_IO_STATUS_EOF, G_IO_STATUS_NORMAL,
    G_IO_STATUS_ERROR, GSeekType, G_SEEK_END, G_SEEK_SET, G_SEEK_CUR,
};
pub use self::gqueue_h::{_GQueue, GQueue};
pub use self::gtype_h::{
    GType, GValue, _GTypeClass, GTypeClass, _GTypeInstance, GTypeInstance,
};
pub use self::gvalue_h::{_GValue, C2RustUnnamed};
pub use self::gclosure_h::{
    _GClosure, GClosureNotifyData, _GClosureNotifyData, GClosureNotify, GClosure,
    GCallback,
};
pub use self::gsignal_h::{
    GConnectFlags, G_CONNECT_SWAPPED, G_CONNECT_AFTER, G_CONNECT_DEFAULT,
    g_signal_connect_data,
};
pub use self::gobject_h::{_GObject, GObject, g_object_unref};
pub use self::JSCContext_h::{_JSCContext, JSCContextPrivate, _JSCContextPrivate};
pub use self::JSCValue_h::JSCContext;
pub use self::socket_h::{socklen_t, sockaddr, PF_LOCAL, PF_UNIX, AF_UNIX};
pub use self::socket_type_h::{
    __socket_type, SOCK_NONBLOCK, SOCK_CLOEXEC, SOCK_PACKET, SOCK_DCCP, SOCK_SEQPACKET,
    SOCK_RDM, SOCK_RAW, SOCK_DGRAM, SOCK_STREAM, SOCK_STREAM_0,
};
pub use self::sockaddr_h::sa_family_t;
pub use self::un_h::sockaddr_un;
pub use self::WebKitScriptWorld_h::{
    _WebKitScriptWorld, WebKitScriptWorldPrivate, WebKitScriptWorld,
    _WebKitScriptWorldPrivate, webkit_script_world_get_default,
};
pub use self::WebKitFrame_h::{
    _WebKitFrame, WebKitFramePrivate, WebKitFrame, _WebKitFramePrivate,
    webkit_frame_get_js_context_for_script_world,
};
pub use self::WebKitWebPage_h::{
    _WebKitWebPage, WebKitWebPagePrivate, WebKitWebPage, _WebKitWebPagePrivate,
    webkit_web_page_get_id, webkit_web_page_get_main_frame,
};
pub use self::WebKitWebExtension_h::{
    _WebKitWebExtension, WebKitWebExtensionPrivate, WebKitWebExtension,
    _WebKitWebExtensionPrivate, webkit_web_extension_get_page,
};
pub use self::lua_h::{
    lua_CFunction, lua_Integer, LUA_GLOBALSINDEX, lua_State, lua_gettop, lua_settop,
    lua_remove, lua_insert, lua_tointeger, lua_toboolean, lua_tolstring, lua_pushstring,
    lua_pushcclosure, lua_getfield, lua_pcall,
};
pub use self::log_h::{
    log_level_t, LOG_LEVEL_debug, LOG_LEVEL_verbose, LOG_LEVEL_info, LOG_LEVEL_warn,
    LOG_LEVEL_error, LOG_LEVEL_fatal, _log,
};
pub use self::ipc_h::{
    ipc_type_t, IPC_TYPE_crash, IPC_TYPE_page_created, IPC_TYPE_log, IPC_TYPE_eval_js,
    IPC_TYPE_extension_init, IPC_TYPE_scroll, IPC_TYPE_lua_ipc,
    IPC_TYPE_lua_require_module, _ipc_header_t, ipc_header_t, _ipc_lua_require_module_t,
    ipc_lua_require_module_t, _ipc_lua_ipc_t, ipc_lua_ipc_t, _ipc_page_created_t,
    ipc_page_created_t, _ipc_recv_state_t, ipc_recv_state_t, ipc_endpoint_status_t,
    IPC_ENDPOINT_FREED, IPC_ENDPOINT_CONNECTED, IPC_ENDPOINT_DISCONNECTED,
    _ipc_endpoint_t, ipc_endpoint_t, ipc_endpoint_connect_to_socket, ipc_send_lua,
    ipc_send,
};
pub use self::extension_h::{extension_t, _extension_t, extension};
pub use self::common_h::{common_t, _common_t, common};
use self::string_h::{memset, strcpy, strlen};
use self::signal_h::raise;
pub use self::gmessages_h::G_LOG_DOMAIN;
use self::gtestutils_h::g_assertion_message_cmpint;
use self::stdio_h::perror;
pub use self::assert_h::{__ASSERT_FUNCTION, __assert_fail};
use self::sys_socket_h::{socket, connect};
use self::unistd_h::{close, getpid};
use self::luakit_h::luakit_lib_emit_pending_signals;
use self::scroll_h::web_scroll_to;
use self::luajs_h::luajs_eval_js;
use self::luaserialize_h::lua_deserialize_range;
use self::luautil_h::luaH_dofunction_on_error;
pub use self::lualib_h::luaH_dofunction;
use self::clib_ipc_h::ipc_channel_recv;
pub use self::gmacros_h::{FALSE, TRUE};
pub use self::__stddef_null_h::NULL;
pub use self::signum_generic_h::SIGKILL;

static mut queued_page_ipc: *mut GPtrArray = 0 as *const GPtrArray as *mut GPtrArray;
#[no_mangle]

pub unsafe extern "C" fn ipc_recv_page_created(
    mut ipc: *mut ipc_endpoint_t,
    UNUSED_msg: gpointer,
    mut UNUSED_length: guint,
) {
    _log(
        LOG_LEVEL_fatal,
        b"extension/ipc.c\0" as *const u8 as *const std::ffi::c_char,
        b"process '%s': should never receive message of type %s\0" as *const u8
            as *const std::ffi::c_char,
        (*ipc).name,
        b"page_created\0" as *const u8 as *const std::ffi::c_char,
    );
}
#[no_mangle]

pub unsafe extern "C" fn ipc_recv_log(
    mut ipc: *mut ipc_endpoint_t,
    UNUSED_msg: gpointer,
    mut UNUSED_length: guint,
) {
    _log(
        LOG_LEVEL_fatal,
        b"extension/ipc.c\0" as *const u8 as *const std::ffi::c_char,
        b"process '%s': should never receive message of type %s\0" as *const u8
            as *const std::ffi::c_char,
        (*ipc).name,
        b"log\0" as *const u8 as *const std::ffi::c_char,
    );
}
#[no_mangle]

pub unsafe extern "C" fn ipc_recv_lua_require_module(
    mut UNUSED_ipc: *mut ipc_endpoint_t,
    mut msg: *const ipc_lua_require_module_t,
    mut length: guint,
) {
    let mut module_name = ((*msg).module_name).as_ptr();
    if strlen(module_name) > 0 as std::ffi::c_int as std::ffi::c_ulong {} else {
        __assert_fail(
            b"strlen(module_name) > 0\0" as *const u8 as *const std::ffi::c_char,
            b"extension/ipc.c\0" as *const u8 as *const std::ffi::c_char,
            46 as std::ffi::c_int as std::ffi::c_uint,
            __ASSERT_FUNCTION.as_ptr(),
        );
    };
    if strlen(module_name)
        == length.wrapping_sub(1 as std::ffi::c_int as guint) as std::ffi::c_ulong
    {} else {
        __assert_fail(
            b"strlen(module_name) == length-1\0" as *const u8 as *const std::ffi::c_char,
            b"extension/ipc.c\0" as *const u8 as *const std::ffi::c_char,
            47 as std::ffi::c_int as std::ffi::c_uint,
            __ASSERT_FUNCTION.as_ptr(),
        );
    };
    lua_pushstring(common.L, module_name);
    lua_getfield(
        common.L,
        LUA_GLOBALSINDEX,
        b"require\0" as *const u8 as *const std::ffi::c_char,
    );
    luaH_dofunction(common.L, 1 as std::ffi::c_int, 0 as std::ffi::c_int);
}
#[no_mangle]

pub unsafe extern "C" fn ipc_recv_lua_ipc(
    mut UNUSED_ipc: *mut ipc_endpoint_t,
    mut msg: *const ipc_lua_ipc_t,
    mut length: guint,
) {
    ipc_channel_recv(common.L, ((*msg).arg).as_ptr(), length);
}
#[no_mangle]

pub unsafe extern "C" fn ipc_recv_extension_init(
    mut UNUSED_ipc: *mut ipc_endpoint_t,
    mut UNUSED_msg: gpointer,
    mut UNUSED_length: guint,
) {
    emit_pending_page_creation_ipc();
    luakit_lib_emit_pending_signals(common.L);
}
#[no_mangle]

pub unsafe extern "C" fn ipc_recv_scroll(
    mut UNUSED_ipc: *mut ipc_endpoint_t,
    mut msg: *const guint8,
    mut length: guint,
) {
    let mut L = common.L;
    let mut n = lua_deserialize_range(L, msg, length);
    let mut __n1 = n as gint64;
    let mut __n2 = 3 as std::ffi::c_int as gint64;
    if !(__n1 == __n2) {
        g_assertion_message_cmpint(
            G_LOG_DOMAIN as *const std::ffi::c_char,
            b"extension/ipc.c\0" as *const u8 as *const std::ffi::c_char,
            72 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 16],
                &[std::ffi::c_char; 16],
            >(b"ipc_recv_scroll\0"))
                .as_ptr(),
            b"n == 3\0" as *const u8 as *const std::ffi::c_char,
            __n1 as guint64,
            b"==\0" as *const u8 as *const std::ffi::c_char,
            __n2 as guint64,
            'i' as i32 as std::ffi::c_char,
        );
    }
    let mut page_id = lua_tointeger(L, -(3 as std::ffi::c_int)) as guint64;
    let mut scroll_x = lua_tointeger(L, -(2 as std::ffi::c_int)) as gint;
    let mut scroll_y = lua_tointeger(L, -(1 as std::ffi::c_int)) as gint;
    web_scroll_to(page_id, scroll_x, scroll_y);
    lua_settop(L, -(3 as std::ffi::c_int) - 1 as std::ffi::c_int);
}
#[no_mangle]

pub unsafe extern "C" fn ipc_recv_eval_js(
    mut UNUSED_ipc: *mut ipc_endpoint_t,
    mut msg: *const guint8,
    mut length: guint,
) {
    let mut L = common.L;
    let mut top = lua_gettop(L);
    let mut n = lua_deserialize_range(L, msg, length);
    let mut __n1 = n as gint64;
    let mut __n2 = 5 as std::ffi::c_int as gint64;
    if !(__n1 == __n2) {
        g_assertion_message_cmpint(
            G_LOG_DOMAIN as *const std::ffi::c_char,
            b"extension/ipc.c\0" as *const u8 as *const std::ffi::c_char,
            89 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[std::ffi::c_char; 17],
            >(b"ipc_recv_eval_js\0"))
                .as_ptr(),
            b"n == 5\0" as *const u8 as *const std::ffi::c_char,
            __n1 as guint64,
            b"==\0" as *const u8 as *const std::ffi::c_char,
            __n2 as guint64,
            'i' as i32 as std::ffi::c_char,
        );
    }
    let mut no_return = lua_toboolean(L, -(5 as std::ffi::c_int));
    let mut script = lua_tolstring(L, -(4 as std::ffi::c_int), NULL as *mut size_t);
    let mut source = lua_tolstring(L, -(3 as std::ffi::c_int), NULL as *mut size_t);
    let mut page_id = lua_tointeger(L, -(2 as std::ffi::c_int)) as guint64;
    let mut page = webkit_web_extension_get_page(extension.ext, page_id);
    if page.is_null() {
        ipc_send_lua(
            extension.ipc,
            IPC_TYPE_eval_js,
            L,
            -(2 as std::ffi::c_int),
            -(1 as std::ffi::c_int),
        );
        lua_settop(L, top);
        return;
    }
    let mut frame = webkit_web_page_get_main_frame(page);
    let mut world = webkit_script_world_get_default();
    let mut ctx = webkit_frame_get_js_context_for_script_world(frame, world);
    n = luajs_eval_js(
        L,
        ctx,
        script,
        source,
        1 as std::ffi::c_int as guint,
        no_return != 0,
    );
    g_object_unref(ctx as gpointer);
    ipc_send_lua(
        extension.ipc,
        IPC_TYPE_eval_js,
        L,
        -n - 2 as std::ffi::c_int,
        -(1 as std::ffi::c_int),
    );
    lua_settop(L, top);
}
#[no_mangle]

pub unsafe extern "C" fn ipc_recv_crash(
    mut UNUSED_ipc: *mut ipc_endpoint_t,
    mut UNUSED_msg: *const guint8,
    mut UNUSED_length: guint,
) {
    raise(SIGKILL);
}

unsafe extern "C" fn emit_page_created_ipc(
    mut web_page: *mut WebKitWebPage,
    mut UNUSED_user_data: gpointer,
) {
    let mut msg = {
        let mut init = _ipc_page_created_t {
            page_id: webkit_web_page_get_id(web_page),
            pid: getpid(),
        };
        init
    };
    let mut header = {
        let mut init = _ipc_header_t {
            length: ::core::mem::size_of::<ipc_page_created_t>() as std::ffi::c_ulong
                as guint,
            type_0: IPC_TYPE_page_created,
        };
        init
    };
    ipc_send(
        extension.ipc,
        &mut header,
        &mut msg as *mut ipc_page_created_t as *const std::ffi::c_void,
    );
}
#[no_mangle]

pub unsafe extern "C" fn emit_pending_page_creation_ipc() {
    if !queued_page_ipc.is_null() {
        g_ptr_array_foreach(
            queued_page_ipc,
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut WebKitWebPage, gpointer) -> ()>,
                GFunc,
            >(
                Some(
                    emit_page_created_ipc
                        as unsafe extern "C" fn(*mut WebKitWebPage, gpointer) -> (),
                ),
            ),
            NULL as *mut std::ffi::c_void,
        );
        g_ptr_array_free(queued_page_ipc, TRUE);
        queued_page_ipc = NULL as *mut GPtrArray;
    }
}

unsafe extern "C" fn web_page_created_cb(
    mut UNUSED_ext: *mut WebKitWebExtension,
    mut web_page: *mut WebKitWebPage,
    mut UNUSED_user_data: gpointer,
) {
    if !queued_page_ipc.is_null() {
        g_ptr_array_add(queued_page_ipc, web_page as gpointer);
    } else {
        emit_page_created_ipc(web_page, NULL as *mut std::ffi::c_void);
    };
}
#[no_mangle]

pub unsafe extern "C" fn web_extension_connect(
    mut socket_path: *const gchar,
) -> std::ffi::c_int {
    let mut sock: std::ffi::c_int = 0;
    let mut remote = sockaddr_un {
        sun_family: 0,
        sun_path: [0; 108],
    };
    memset(
        &mut remote as *mut sockaddr_un as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        ::core::mem::size_of::<sockaddr_un>() as std::ffi::c_ulong,
    );
    remote.sun_family = AF_UNIX as sa_family_t;
    strcpy((remote.sun_path).as_mut_ptr(), socket_path);
    let mut len = (2 as std::ffi::c_ulong)
        .wrapping_add(strlen((remote.sun_path).as_mut_ptr())) as std::ffi::c_int;
    _log(
        LOG_LEVEL_debug,
        b"extension/ipc.c\0" as *const u8 as *const std::ffi::c_char,
        b"luakit web process: connecting to %s\0" as *const u8
            as *const std::ffi::c_char,
        socket_path,
    );
    sock = socket(AF_UNIX, SOCK_STREAM_0, 0 as std::ffi::c_int);
    if sock == -(1 as std::ffi::c_int) {
        perror(b"socket\0" as *const u8 as *const std::ffi::c_char);
    } else if connect(
        sock,
        &mut remote as *mut sockaddr_un as *mut sockaddr,
        len as socklen_t,
    ) == -(1 as std::ffi::c_int)
    {
        perror(b"connect\0" as *const u8 as *const std::ffi::c_char);
        close(sock);
    } else {
        _log(
            LOG_LEVEL_debug,
            b"extension/ipc.c\0" as *const u8 as *const std::ffi::c_char,
            b"luakit web process: connected\0" as *const u8 as *const std::ffi::c_char,
        );
        ipc_endpoint_connect_to_socket(extension.ipc, sock);
        g_signal_connect_data(
            extension.ext as gpointer,
            b"page-created\0" as *const u8 as *const std::ffi::c_char,
            ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut WebKitWebExtension,
                        *mut WebKitWebPage,
                        gpointer,
                    ) -> (),
                >,
                GCallback,
            >(
                Some(
                    web_page_created_cb
                        as unsafe extern "C" fn(
                            *mut WebKitWebExtension,
                            *mut WebKitWebPage,
                            gpointer,
                        ) -> (),
                ),
            ),
            0 as *mut std::ffi::c_void,
            ::core::mem::transmute::<
                libc::intptr_t,
                GClosureNotify,
            >(NULL as libc::intptr_t),
            G_CONNECT_DEFAULT,
        );
        queued_page_ipc = g_ptr_array_sized_new(1 as std::ffi::c_int as guint);
        return 0 as std::ffi::c_int;
    }
    return 1 as std::ffi::c_int;
}
