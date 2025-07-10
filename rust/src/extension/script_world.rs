use ::libc;


pub mod glibconfig_h {

    pub type guint32 = std::ffi::c_uint;

    pub type gint64 = std::ffi::c_long;

    pub type gsize = std::ffi::c_ulong;
}

pub mod gtypes_h {

    pub type gchar = std::ffi::c_char;

    pub type gint = std::ffi::c_int;

    pub type gboolean = gint;

    pub type guint = std::ffi::c_uint;

    pub type gpointer = *mut std::ffi::c_void;
}

pub mod garray_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GPtrArray {
        pub pdata: *mut gpointer,
        pub len: guint,
    }

    pub type GPtrArray = _GPtrArray;
    use super::gtypes_h::{gpointer, guint};
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
    unsafe extern "C" {

        pub type _GIConv;
    }
}

pub mod gdataset_h {

    pub type GData = _GData;
    unsafe extern "C" {

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
    unsafe extern "C" {

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
    #[derive(Copy, Clone)]
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






        pub use_buffer_do_encode_close_on_unref_is_readable_is_writeable_is_seekable: [u8; 1],

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
    use super::gtypes_h::guint;
    use super::gdataset_h::GData;
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
    unsafe extern "C" {

        pub type _WebKitScriptWorldPrivate;

        pub fn webkit_script_world_new() -> *mut WebKitScriptWorld;
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
    unsafe extern "C" {

        pub type _WebKitWebExtensionPrivate;
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
    use super::gtypes_h::{guint, gpointer, gboolean, gchar, gint};
    use super::garray_h::GPtrArray;
    use super::glibconfig_h::gsize;
    use super::giochannel_h::GIOChannel;
    use super::gqueue_h::GQueue;
}

pub mod extension_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _extension_t {
        pub ext: *mut WebKitWebExtension,
        pub ipc: *mut ipc_endpoint_t,
        pub script_world: *mut WebKitScriptWorld,
    }

    pub type extension_t = _extension_t;
    use super::WebKitWebExtension_h::WebKitWebExtension;
    use super::ipc_h::ipc_endpoint_t;
    use super::WebKitScriptWorld_h::WebKitScriptWorld;
    unsafe extern "C" {

        pub static mut extension: extension_t;
    }
}
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
pub use self::gqueue_h::{_GQueue, GQueue};
pub use self::gtype_h::{GType, _GTypeClass, GTypeClass, _GTypeInstance, GTypeInstance};
pub use self::gobject_h::{_GObject, GObject};
pub use self::WebKitScriptWorld_h::{
    _WebKitScriptWorld, WebKitScriptWorldPrivate, WebKitScriptWorld,
    _WebKitScriptWorldPrivate, webkit_script_world_new,
};
pub use self::WebKitWebExtension_h::{
    _WebKitWebExtension, WebKitWebExtensionPrivate, WebKitWebExtension,
    _WebKitWebExtensionPrivate,
};
pub use self::ipc_h::{
    ipc_type_t, IPC_TYPE_crash, IPC_TYPE_page_created, IPC_TYPE_log, IPC_TYPE_eval_js,
    IPC_TYPE_extension_init, IPC_TYPE_scroll, IPC_TYPE_lua_ipc,
    IPC_TYPE_lua_require_module, _ipc_header_t, ipc_header_t, _ipc_recv_state_t,
    ipc_recv_state_t, ipc_endpoint_status_t, IPC_ENDPOINT_FREED, IPC_ENDPOINT_CONNECTED,
    IPC_ENDPOINT_DISCONNECTED, _ipc_endpoint_t, ipc_endpoint_t,
};
pub use self::extension_h::{_extension_t, extension_t, extension};
#[unsafe(no_mangle)]

pub unsafe extern "C" fn web_script_world_init() {
    extension.script_world = webkit_script_world_new();
}
