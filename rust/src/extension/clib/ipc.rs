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

pub mod gtree_h {

    pub type GTree = _GTree;
    extern "C" {

        pub type _GTree;
    }
}

pub mod lua_h {

    pub type lua_Integer = ptrdiff_t;

    pub const LUA_REGISTRYINDEX: std::ffi::c_int = -(10000 as std::ffi::c_int);

    pub const LUA_TNIL: std::ffi::c_int = 0 as std::ffi::c_int;
    use super::__stddef_ptrdiff_t_h::ptrdiff_t;
    use super::__stddef_size_t_h::size_t;
    extern "C" {

        pub type lua_State;

        pub fn lua_gettop(L: *mut lua_State) -> std::ffi::c_int;

        pub fn lua_settop(L: *mut lua_State, idx: std::ffi::c_int);

        pub fn lua_remove(L: *mut lua_State, idx: std::ffi::c_int);

        pub fn lua_insert(L: *mut lua_State, idx: std::ffi::c_int);

        pub fn lua_type(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;

        pub fn lua_tointeger(L: *mut lua_State, idx: std::ffi::c_int) -> lua_Integer;

        pub fn lua_tolstring(
            L: *mut lua_State,
            idx: std::ffi::c_int,
            len: *mut size_t,
        ) -> *const std::ffi::c_char;

        pub fn lua_pushnil(L: *mut lua_State);

        pub fn lua_pushstring(L: *mut lua_State, s: *const std::ffi::c_char);

        pub fn lua_rawget(L: *mut lua_State, idx: std::ffi::c_int);
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
    use super::lua_h::lua_State;
    extern "C" {

        pub fn ipc_send_lua(
            ipc: *mut ipc_endpoint_t,
            type_0: ipc_type_t,
            L: *mut lua_State,
            start: gint,
            end: gint,
        );
    }
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
    extern "C" {

        pub type _WebKitScriptWorldPrivate;
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
    extern "C" {

        pub type _WebKitWebPagePrivate;
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
    extern "C" {

        pub static mut extension: extension_t;
    }
}

pub mod signal_h {

    pub type signal_t = GTree;
    use super::gtree_h::GTree;
}

pub mod clib_ipc_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _ipc_channel_t {
        pub signals: *mut signal_t,
        pub name: *mut std::ffi::c_char,
    }

    pub type ipc_channel_t = _ipc_channel_t;
    use super::signal_h::signal_t;
    use super::lua_h::lua_State;
    use super::gtypes_h::gint;
    extern "C" {

        pub fn luaH_check_ipc_channel(
            L: *mut lua_State,
            idx: gint,
        ) -> *mut ipc_channel_t;
    }
}

pub mod lauxlib_h {
    use super::lua_h::lua_State;
    use super::__stddef_size_t_h::size_t;
    extern "C" {

        pub fn luaL_checklstring(
            L: *mut lua_State,
            numArg: std::ffi::c_int,
            l: *mut size_t,
        ) -> *const std::ffi::c_char;
    }
}

pub mod luaobject_h {
    use super::lua_h::lua_State;
    use super::gtypes_h::{gint, gchar};
    extern "C" {

        pub fn luaH_object_emit_signal(
            L: *mut lua_State,
            oud: gint,
            name: *const gchar,
            nargs: gint,
            nret: gint,
        ) -> gint;
    }
}

pub mod page_h {
    use super::lua_h::lua_State;
    use super::WebKitWebPage_h::WebKitWebPage;
    use super::gtypes_h::gint;
    extern "C" {

        pub fn luaH_page_from_web_page(
            L: *mut lua_State,
            web_page: *mut WebKitWebPage,
        ) -> gint;
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

pub mod __stddef_null_h {

    pub const NULL: std::ffi::c_int = 0 as std::ffi::c_int;
}
pub use self::__stddef_ptrdiff_t_h::ptrdiff_t;
pub use self::__stddef_size_t_h::size_t;
pub use self::glibconfig_h::{guint8, guint32, gint64, guint64, gsize};
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
pub use self::gtree_h::{GTree, _GTree};
pub use self::lua_h::{
    lua_Integer, LUA_REGISTRYINDEX, LUA_TNIL, lua_State, lua_gettop, lua_settop,
    lua_remove, lua_insert, lua_type, lua_tointeger, lua_tolstring, lua_pushnil,
    lua_pushstring, lua_rawget,
};
pub use self::ipc_h::{
    ipc_type_t, IPC_TYPE_crash, IPC_TYPE_page_created, IPC_TYPE_log, IPC_TYPE_eval_js,
    IPC_TYPE_extension_init, IPC_TYPE_scroll, IPC_TYPE_lua_ipc,
    IPC_TYPE_lua_require_module, _ipc_header_t, ipc_header_t, _ipc_recv_state_t,
    ipc_recv_state_t, ipc_endpoint_status_t, IPC_ENDPOINT_FREED, IPC_ENDPOINT_CONNECTED,
    IPC_ENDPOINT_DISCONNECTED, _ipc_endpoint_t, ipc_endpoint_t, ipc_send_lua,
};
pub use self::gtype_h::{GType, _GTypeClass, GTypeClass, _GTypeInstance, GTypeInstance};
pub use self::gobject_h::{_GObject, GObject};
pub use self::WebKitScriptWorld_h::{
    _WebKitScriptWorld, WebKitScriptWorldPrivate, WebKitScriptWorld,
    _WebKitScriptWorldPrivate,
};
pub use self::WebKitWebPage_h::{
    _WebKitWebPage, WebKitWebPagePrivate, WebKitWebPage, _WebKitWebPagePrivate,
};
pub use self::WebKitWebExtension_h::{
    _WebKitWebExtension, WebKitWebExtensionPrivate, WebKitWebExtension,
    _WebKitWebExtensionPrivate, webkit_web_extension_get_page,
};
pub use self::extension_h::{_extension_t, extension_t, extension};
pub use self::signal_h::signal_t;
pub use self::clib_ipc_h::{_ipc_channel_t, ipc_channel_t, luaH_check_ipc_channel};
use self::lauxlib_h::luaL_checklstring;
use self::luaobject_h::luaH_object_emit_signal;
use self::page_h::luaH_page_from_web_page;
use self::luaserialize_h::lua_deserialize_range;
pub use self::__stddef_null_h::NULL;

pub const REG_KEY: [std::ffi::c_char; 28] = unsafe {
    *::core::mem::transmute::<
        &[u8; 28],
        &[std::ffi::c_char; 28],
    >(b"luakit.registry.ipc_channel\0")
};
#[no_mangle]

pub unsafe extern "C" fn ipc_channel_send(mut L: *mut lua_State) -> gint {
    let mut ipc_channel = luaH_check_ipc_channel(L, 1 as std::ffi::c_int);
    luaL_checklstring(L, 2 as std::ffi::c_int, NULL as *mut size_t);
    lua_pushstring(L, (*ipc_channel).name);
    ipc_send_lua(
        extension.ipc,
        IPC_TYPE_lua_ipc,
        L,
        2 as std::ffi::c_int,
        lua_gettop(L),
    );
    return 0 as std::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn ipc_channel_recv(
    mut L: *mut lua_State,
    mut arg: *const gchar,
    mut arglen: guint,
) {
    let mut top = lua_gettop(L);
    let mut n = lua_deserialize_range(L, arg as *mut guint8, arglen);
    let mut signame = lua_tolstring(L, -n, NULL as *mut size_t);
    lua_remove(L, -n);
    let mut module_name = lua_tolstring(L, -(2 as std::ffi::c_int), NULL as *mut size_t);
    let mut page_id = lua_tointeger(L, -(1 as std::ffi::c_int)) as guint64;
    lua_settop(L, -(2 as std::ffi::c_int) - 1 as std::ffi::c_int);
    n -= 3 as std::ffi::c_int;
    if page_id != 0 {
        let mut web_page = webkit_web_extension_get_page(extension.ext, page_id);
        luaH_page_from_web_page(L, web_page);
    } else {
        lua_pushnil(L);
    }
    lua_insert(L, -n - 1 as std::ffi::c_int);
    n += 1;
    n;
    lua_pushstring(L, REG_KEY.as_ptr());
    lua_rawget(L, LUA_REGISTRYINDEX);
    lua_pushstring(L, module_name);
    lua_rawget(L, -(2 as std::ffi::c_int));
    lua_remove(L, -(2 as std::ffi::c_int));
    if !(lua_type(L, -(1 as std::ffi::c_int)) == LUA_TNIL) {
        lua_insert(L, -n - 1 as std::ffi::c_int);
        luaH_object_emit_signal(
            L,
            -n - 1 as std::ffi::c_int,
            signame,
            n,
            0 as std::ffi::c_int,
        );
    }
    lua_settop(L, top);
}
