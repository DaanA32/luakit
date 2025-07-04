use ::libc;
use ::c2rust_bitfields;
#[c2rust::header_src = "/usr/include/bits/types.h:19"]
pub mod types_h {
    #[c2rust::src_loc = "154:1"]
    pub type __pid_t = std::ffi::c_int;
}
#[c2rust::header_src = "/usr/lib/glib-2.0/include/glibconfig.h:21"]
pub mod glibconfig_h {
    #[c2rust::src_loc = "57:1"]
    pub type guint32 = std::ffi::c_uint;
    #[c2rust::src_loc = "66:1"]
    pub type gint64 = std::ffi::c_long;
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
    #[c2rust::src_loc = "55:8"]
    pub struct _GPtrArray {
        pub pdata: *mut gpointer,
        pub len: guint,
    }
    #[c2rust::src_loc = "41:1"]
    pub type GPtrArray = _GPtrArray;
    use super::gtypes_h::{gpointer, guint};
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
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gdataset.h:21"]
pub mod gdataset_h {
    #[c2rust::src_loc = "38:1"]
    pub type GData = _GData;
    extern "C" {
        #[c2rust::src_loc = "38:16"]
        pub type _GData;
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
    use super::gtypes_h::{gint, gchar, guint, gpointer};
    use super::gconvert_h::GIConv;
    use super::glibconfig_h::{gsize, gint64};
    use super::gstring_h::GString;
    use super::gerror_h::GError;
    use super::gmain_h::{GSource, GIOCondition};
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gvariant.h:21"]
pub mod gvariant_h {
    #[c2rust::src_loc = "36:1"]
    pub type GVariant = _GVariant;
    use super::gtypes_h::gchar;
    extern "C" {
        #[c2rust::src_loc = "36:16"]
        pub type _GVariant;
        #[c2rust::src_loc = "446:1"]
        pub fn g_variant_get(value: *mut GVariant, format_string: *const gchar, _: ...);
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
    use super::gtypes_h::guint;
}
#[c2rust::header_src = "/usr/include/luajit-2.1/lua.h:23"]
pub mod lua_h {
    #[c2rust::src_loc = "53:1"]
    pub type lua_CFunction = Option::<
        unsafe extern "C" fn(*mut lua_State) -> std::ffi::c_int,
    >;
    #[c2rust::src_loc = "38:9"]
    pub const LUA_GLOBALSINDEX: std::ffi::c_int = -(10002 as std::ffi::c_int);
    extern "C" {
        #[c2rust::src_loc = "51:16"]
        pub type lua_State;
        #[c2rust::src_loc = "115:1"]
        pub fn lua_atpanic(L: *mut lua_State, panicf: lua_CFunction) -> lua_CFunction;
        #[c2rust::src_loc = "122:1"]
        pub fn lua_settop(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "165:1"]
        pub fn lua_pushstring(L: *mut lua_State, s: *const std::ffi::c_char);
        #[c2rust::src_loc = "179:1"]
        pub fn lua_getfield(
            L: *mut lua_State,
            idx: std::ffi::c_int,
            k: *const std::ffi::c_char,
        );
        #[c2rust::src_loc = "192:1"]
        pub fn lua_setfield(
            L: *mut lua_State,
            idx: std::ffi::c_int,
            k: *const std::ffi::c_char,
        );
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/log.h:23"]
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
#[c2rust::header_src = "/usr/include/glib-2.0/gobject/gtype.h:24"]
pub mod gtype_h {
    #[c2rust::src_loc = "427:1"]
    pub type GType = gsize;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "451:8"]
    pub struct _GTypeClass {
        pub g_type: GType,
    }
    #[c2rust::src_loc = "434:1"]
    pub type GTypeClass = _GTypeClass;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "461:8"]
    pub struct _GTypeInstance {
        pub g_class: *mut GTypeClass,
    }
    #[c2rust::src_loc = "436:1"]
    pub type GTypeInstance = _GTypeInstance;
    use super::glibconfig_h::gsize;
}
#[c2rust::header_src = "/usr/include/glib-2.0/gobject/gobject.h:24"]
pub mod gobject_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "252:9"]
    pub struct _GObject {
        pub g_type_instance: GTypeInstance,
        pub ref_count: guint,
        pub qdata: *mut GData,
    }
    #[c2rust::src_loc = "192:1"]
    pub type GObject = _GObject;
    use super::gtype_h::GTypeInstance;
    use super::gtypes_h::guint;
    use super::gdataset_h::GData;
}
#[c2rust::header_src = "/home/daana/git/luakit/common/common.h:24"]
pub mod common_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "24:16"]
    pub struct _common_t {
        pub L: *mut lua_State,
    }
    #[c2rust::src_loc = "24:1"]
    pub type common_t = _common_t;
    use super::lua_h::lua_State;
}
#[c2rust::header_src = "/home/daana/git/luakit/common/ipc.h:27"]
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
    use super::gtypes_h::{guint, gpointer, gboolean, gchar, gint};
    use super::garray_h::GPtrArray;
    use super::glibconfig_h::gsize;
    use super::giochannel_h::GIOChannel;
    use super::gqueue_h::GQueue;
    extern "C" {
        #[c2rust::src_loc = "127:1"]
        pub fn ipc_endpoint_new(name: *const gchar) -> *mut ipc_endpoint_t;
        #[c2rust::src_loc = "138:1"]
        pub fn ipc_send(
            ipc: *mut ipc_endpoint_t,
            header: *const ipc_header_t,
            data: *const std::ffi::c_void,
        );
    }
}
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/webkit/WebKitScriptWorld.h:29"]
pub mod WebKitScriptWorld_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "52:1"]
    pub struct _WebKitScriptWorld {
        pub parent: GObject,
        pub priv_0: *mut WebKitScriptWorldPrivate,
    }
    #[c2rust::src_loc = "41:1"]
    pub type WebKitScriptWorldPrivate = _WebKitScriptWorldPrivate;
    #[c2rust::src_loc = "39:1"]
    pub type WebKitScriptWorld = _WebKitScriptWorld;
    use super::gobject_h::GObject;
    extern "C" {
        #[c2rust::src_loc = "41:16"]
        pub type _WebKitScriptWorldPrivate;
    }
}
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/webkit/WebKitWebExtension.h:29"]
pub mod WebKitWebExtension_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:1"]
    pub struct _WebKitWebExtension {
        pub parent: GObject,
        pub priv_0: *mut WebKitWebExtensionPrivate,
    }
    #[c2rust::src_loc = "43:1"]
    pub type WebKitWebExtensionPrivate = _WebKitWebExtensionPrivate;
    #[c2rust::src_loc = "41:1"]
    pub type WebKitWebExtension = _WebKitWebExtension;
    use super::gobject_h::GObject;
    extern "C" {
        #[c2rust::src_loc = "43:16"]
        pub type _WebKitWebExtensionPrivate;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/extension/extension.h:29"]
pub mod extension_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "35:16"]
    pub struct _extension_t {
        pub ext: *mut WebKitWebExtension,
        pub ipc: *mut ipc_endpoint_t,
        pub script_world: *mut WebKitScriptWorld,
    }
    #[c2rust::src_loc = "35:1"]
    pub type extension_t = _extension_t;
    use super::WebKitWebExtension_h::WebKitWebExtension;
    use super::ipc_h::ipc_endpoint_t;
    use super::WebKitScriptWorld_h::WebKitScriptWorld;
}
#[c2rust::header_src = "/usr/include/stdlib.h:19"]
pub mod stdlib_h {
    #[c2rust::src_loc = "92:9"]
    pub const EXIT_FAILURE: std::ffi::c_int = 1 as std::ffi::c_int;
    extern "C" {
        #[c2rust::src_loc = "756:13"]
        pub fn exit(_: std::ffi::c_int) -> !;
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:20"]
pub mod unistd_h {
    use super::types_h::__pid_t;
    extern "C" {
        #[c2rust::src_loc = "650:1"]
        pub fn getpid() -> __pid_t;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gstrfuncs.h:21"]
pub mod gstrfuncs_h {
    use super::gtypes_h::gchar;
    extern "C" {
        #[c2rust::src_loc = "285:1"]
        pub fn g_strdup_printf(format: *const gchar, _: ...) -> *mut gchar;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/util.h:23"]
pub mod util_h {
    use super::lua_h::lua_State;
    use super::gtypes_h::gint;
    extern "C" {
        #[c2rust::src_loc = "74:1"]
        pub fn luaH_panic(L: *mut lua_State) -> gint;
    }
}
#[c2rust::header_src = "/usr/include/luajit-2.1/lauxlib.h:24"]
pub mod lauxlib_h {
    use super::lua_h::lua_State;
    extern "C" {
        #[c2rust::src_loc = "70:1"]
        pub fn luaL_newstate() -> *mut lua_State;
    }
}
#[c2rust::header_src = "/usr/include/luajit-2.1/lualib.h:24"]
pub mod lualib_h {
    use super::lua_h::lua_State;
    extern "C" {
        #[c2rust::src_loc = "38:1"]
        pub fn luaL_openlibs(L: *mut lua_State);
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/luaobject.h:24"]
pub mod luaobject_h {
    use super::lua_h::lua_State;
    extern "C" {
        #[c2rust::src_loc = "39:1"]
        pub fn luaH_object_setup(L: *mut lua_State);
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/luah.h:24"]
pub mod luah_h {
    use super::lua_h::lua_State;
    extern "C" {
        #[c2rust::src_loc = "162:1"]
        pub fn luaH_fixups(L: *mut lua_State);
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/luauniq.h:26"]
pub mod luauniq_h {
    use super::lua_h::lua_State;
    use super::gtypes_h::gchar;
    extern "C" {
        #[c2rust::src_loc = "31:1"]
        pub fn luaH_uniq_setup(L: *mut lua_State, reg: *const gchar, mode: *const gchar);
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/extension/ipc.h:27"]
pub mod extension_ipc_h {
    use super::gtypes_h::gchar;
    extern "C" {
        #[c2rust::src_loc = "24:1"]
        pub fn web_extension_connect(socket_path: *const gchar) -> std::ffi::c_int;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/extension/clib/luakit.h:31"]
pub mod luakit_h {
    use super::lua_h::lua_State;
    extern "C" {
        #[c2rust::src_loc = "27:1"]
        pub fn luakit_lib_setup(L: *mut lua_State);
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/extension/clib/dom_document.h:32"]
pub mod dom_document_h {
    use super::lua_h::lua_State;
    extern "C" {
        #[c2rust::src_loc = "35:1"]
        pub fn dom_document_class_setup(_: *mut lua_State);
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/extension/clib/page.h:33"]
pub mod page_h {
    use super::lua_h::lua_State;
    extern "C" {
        #[c2rust::src_loc = "37:1"]
        pub fn page_class_setup(_: *mut lua_State);
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/extension/clib/dom_element.h:33"]
pub mod dom_element_h {
    use super::lua_h::lua_State;
    extern "C" {
        #[c2rust::src_loc = "55:1"]
        pub fn dom_element_class_setup(_: *mut lua_State);
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/extension/clib/soup.h:35"]
pub mod soup_h {
    use super::lua_h::lua_State;
    extern "C" {
        #[c2rust::src_loc = "26:1"]
        pub fn soup_lib_setup(L: *mut lua_State);
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/extension/clib/msg.h:36"]
pub mod msg_h {
    use super::lua_h::lua_State;
    extern "C" {
        #[c2rust::src_loc = "26:1"]
        pub fn msg_lib_setup(L: *mut lua_State);
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/clib/ipc.h:37"]
pub mod clib_ipc_h {
    use super::lua_h::lua_State;
    extern "C" {
        #[c2rust::src_loc = "39:1"]
        pub fn ipc_channel_class_setup(_: *mut lua_State);
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/clib/timer.h:38"]
pub mod timer_h {
    use super::lua_h::lua_State;
    extern "C" {
        #[c2rust::src_loc = "26:1"]
        pub fn timer_class_setup(_: *mut lua_State);
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/clib/regex.h:39"]
pub mod regex_h {
    use super::lua_h::lua_State;
    extern "C" {
        #[c2rust::src_loc = "26:1"]
        pub fn regex_class_setup(_: *mut lua_State);
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/clib/utf8.h:40"]
pub mod utf8_h {
    use super::lua_h::lua_State;
    extern "C" {
        #[c2rust::src_loc = "26:1"]
        pub fn utf8_lib_setup(_: *mut lua_State);
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/extension/scroll.h:42"]
pub mod scroll_h {
    extern "C" {
        #[c2rust::src_loc = "25:1"]
        pub fn web_scroll_init();
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/extension/luajs.h:43"]
pub mod luajs_h {
    extern "C" {
        #[c2rust::src_loc = "24:1"]
        pub fn web_luajs_init();
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/extension/script_world.h:44"]
pub mod script_world_h {
    extern "C" {
        #[c2rust::src_loc = "24:1"]
        pub fn web_script_world_init();
    }
}
#[c2rust::header_src = "/usr/lib/clang/20/include/__stddef_null.h:21"]
pub mod __stddef_null_h {
    #[c2rust::src_loc = "26:9"]
    pub const NULL: std::ffi::c_int = 0 as std::ffi::c_int;
}
pub use self::types_h::__pid_t;
pub use self::glibconfig_h::{guint32, gint64, gsize};
pub use self::gtypes_h::{gchar, gint, gboolean, guint, gpointer};
pub use self::garray_h::{_GPtrArray, GPtrArray};
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
pub use self::gvariant_h::{GVariant, _GVariant, g_variant_get};
pub use self::gqueue_h::{_GQueue, GQueue};
pub use self::lua_h::{
    lua_CFunction, LUA_GLOBALSINDEX, lua_State, lua_atpanic, lua_settop, lua_pushstring,
    lua_getfield, lua_setfield,
};
pub use self::log_h::{
    log_level_t, LOG_LEVEL_debug, LOG_LEVEL_verbose, LOG_LEVEL_info, LOG_LEVEL_warn,
    LOG_LEVEL_error, LOG_LEVEL_fatal, _log,
};
pub use self::gtype_h::{GType, _GTypeClass, GTypeClass, _GTypeInstance, GTypeInstance};
pub use self::gobject_h::{_GObject, GObject};
pub use self::common_h::{_common_t, common_t};
pub use self::ipc_h::{
    ipc_type_t, IPC_TYPE_crash, IPC_TYPE_page_created, IPC_TYPE_log, IPC_TYPE_eval_js,
    IPC_TYPE_extension_init, IPC_TYPE_scroll, IPC_TYPE_lua_ipc,
    IPC_TYPE_lua_require_module, _ipc_header_t, ipc_header_t, _ipc_recv_state_t,
    ipc_recv_state_t, ipc_endpoint_status_t, IPC_ENDPOINT_FREED, IPC_ENDPOINT_CONNECTED,
    IPC_ENDPOINT_DISCONNECTED, _ipc_endpoint_t, ipc_endpoint_t, ipc_endpoint_new,
    ipc_send,
};
pub use self::WebKitScriptWorld_h::{
    _WebKitScriptWorld, WebKitScriptWorldPrivate, WebKitScriptWorld,
    _WebKitScriptWorldPrivate,
};
pub use self::WebKitWebExtension_h::{
    _WebKitWebExtension, WebKitWebExtensionPrivate, WebKitWebExtension,
    _WebKitWebExtensionPrivate,
};
pub use self::extension_h::{_extension_t, extension_t};
pub use self::stdlib_h::{EXIT_FAILURE, exit};
use self::unistd_h::getpid;
use self::gstrfuncs_h::g_strdup_printf;
use self::util_h::luaH_panic;
use self::lauxlib_h::luaL_newstate;
use self::lualib_h::luaL_openlibs;
use self::luaobject_h::luaH_object_setup;
use self::luah_h::luaH_fixups;
use self::luauniq_h::luaH_uniq_setup;
use self::extension_ipc_h::web_extension_connect;
use self::luakit_h::luakit_lib_setup;
use self::dom_document_h::dom_document_class_setup;
use self::page_h::page_class_setup;
use self::dom_element_h::dom_element_class_setup;
use self::soup_h::soup_lib_setup;
use self::msg_h::msg_lib_setup;
use self::clib_ipc_h::ipc_channel_class_setup;
use self::timer_h::timer_class_setup;
use self::regex_h::regex_class_setup;
use self::utf8_h::utf8_lib_setup;
use self::scroll_h::web_scroll_init;
use self::luajs_h::web_luajs_init;
use self::script_world_h::web_script_world_init;
pub use self::__stddef_null_h::NULL;
#[no_mangle]
#[c2rust::src_loc = "50:10"]
pub static mut common: common_t = _common_t {
    L: 0 as *const lua_State as *mut lua_State,
};
#[no_mangle]
#[c2rust::src_loc = "53:13"]
pub static mut extension: extension_t = _extension_t {
    ext: 0 as *const WebKitWebExtension as *mut WebKitWebExtension,
    ipc: 0 as *const ipc_endpoint_t as *mut ipc_endpoint_t,
    script_world: 0 as *const WebKitScriptWorld as *mut WebKitScriptWorld,
};
#[c2rust::src_loc = "56:1"]
unsafe extern "C" fn web_lua_init(
    mut package_path: *const std::ffi::c_char,
    mut package_cpath: *const std::ffi::c_char,
) {
    _log(
        LOG_LEVEL_debug,
        b"extension/extension.c\0" as *const u8 as *const std::ffi::c_char,
        b"Lua initializing...\0" as *const u8 as *const std::ffi::c_char,
    );
    let mut L = common.L;
    lua_atpanic(L, Some(luaH_panic as unsafe extern "C" fn(*mut lua_State) -> gint));
    luaL_openlibs(L);
    luaH_fixups(L);
    luaH_object_setup(L);
    luaH_uniq_setup(
        L,
        NULL as *const gchar,
        b"v\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_getfield(
        L,
        LUA_GLOBALSINDEX,
        b"package\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_pushstring(L, package_path);
    lua_setfield(
        L,
        -(2 as std::ffi::c_int),
        b"path\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_pushstring(L, package_cpath);
    lua_setfield(
        L,
        -(2 as std::ffi::c_int),
        b"cpath\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    luakit_lib_setup(L);
    soup_lib_setup(L);
    ipc_channel_class_setup(L);
    timer_class_setup(L);
    regex_class_setup(L);
    utf8_lib_setup(L);
    dom_document_class_setup(L);
    dom_element_class_setup(L);
    page_class_setup(L);
    msg_lib_setup(L);
    _log(
        LOG_LEVEL_debug,
        b"extension/extension.c\0" as *const u8 as *const std::ffi::c_char,
        b"Lua initialized\0" as *const u8 as *const std::ffi::c_char,
    );
}
#[no_mangle]
#[c2rust::src_loc = "92:1"]
pub unsafe extern "C" fn webkit_web_extension_initialize_with_user_data(
    mut ext: *mut WebKitWebExtension,
    mut payload: *mut GVariant,
) {
    let mut socket_path = 0 as *mut gchar;
    let mut package_path = 0 as *mut gchar;
    let mut package_cpath = 0 as *mut gchar;
    g_variant_get(
        payload,
        b"(sss)\0" as *const u8 as *const std::ffi::c_char,
        &mut socket_path as *mut *mut gchar,
        &mut package_path as *mut *mut gchar,
        &mut package_cpath as *mut *mut gchar,
    );
    common.L = luaL_newstate();
    common.L = common.L;
    extension.ext = ext;
    extension
        .ipc = ipc_endpoint_new(
        g_strdup_printf(b"Web[%d]\0" as *const u8 as *const std::ffi::c_char, getpid()),
    );
    if web_extension_connect(socket_path) != 0 {
        _log(
            LOG_LEVEL_debug,
            b"extension/extension.c\0" as *const u8 as *const std::ffi::c_char,
            b"connecting to UI thread failed\0" as *const u8 as *const std::ffi::c_char,
        );
        exit(EXIT_FAILURE);
    }
    web_lua_init(package_path, package_cpath);
    web_scroll_init();
    web_luajs_init();
    web_script_world_init();
    _log(
        LOG_LEVEL_debug,
        b"extension/extension.c\0" as *const u8 as *const std::ffi::c_char,
        b"PID %d\0" as *const u8 as *const std::ffi::c_char,
        getpid(),
    );
    _log(
        LOG_LEVEL_debug,
        b"extension/extension.c\0" as *const u8 as *const std::ffi::c_char,
        b"ready for messages\0" as *const u8 as *const std::ffi::c_char,
    );
    let mut header = {
        let mut init = _ipc_header_t {
            length: 0 as std::ffi::c_int as guint,
            type_0: IPC_TYPE_extension_init,
        };
        init
    };
    ipc_send(extension.ipc, &mut header, NULL as *const std::ffi::c_void);
}
