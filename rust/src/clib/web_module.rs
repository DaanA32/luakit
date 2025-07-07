use ::c2rust_bitfields;
use ::libc;
pub mod __stddef_size_t_h {
    pub type size_t = std::ffi::c_ulong;
}
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
    unsafe extern "C" {
        pub fn g_ptr_array_new() -> *mut GPtrArray;
        pub fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
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
    use super::gtypes_h::{gchar, gint};
}
pub mod gconvert_h {
    pub type GIConv = *mut _GIConv;
    unsafe extern "C" {
        pub type _GIConv;
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
    pub type GSourceDummyMarshal = Option<unsafe extern "C" fn() -> ()>;
    pub type GSourceFunc = Option<unsafe extern "C" fn(gpointer) -> gboolean>;
    pub type GSourceFuncsFinalizeFunc = Option<unsafe extern "C" fn(*mut GSource) -> ()>;
    pub type GSourceFuncsDispatchFunc =
        Option<unsafe extern "C" fn(*mut GSource, GSourceFunc, gpointer) -> gboolean>;
    pub type GSourceFuncsCheckFunc = Option<unsafe extern "C" fn(*mut GSource) -> gboolean>;
    pub type GSourceFuncsPrepareFunc =
        Option<unsafe extern "C" fn(*mut GSource, *mut gint) -> gboolean>;
    pub type GSourceCallbackFuncs = _GSourceCallbackFuncs;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GSourceCallbackFuncs {
        pub ref_0: Option<unsafe extern "C" fn(gpointer) -> ()>,
        pub unref: Option<unsafe extern "C" fn(gpointer) -> ()>,
        pub get: Option<
            unsafe extern "C" fn(gpointer, *mut GSource, *mut GSourceFunc, *mut gpointer) -> (),
        >,
    }
    use super::gslist_h::GSList;
    use super::gtypes_h::{gboolean, gint, gpointer, guint};
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
    use super::glibconfig_h::gsize;
    use super::gtypes_h::gchar;
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
        pub io_read: Option<
            unsafe extern "C" fn(
                *mut GIOChannel,
                *mut gchar,
                gsize,
                *mut gsize,
                *mut *mut GError,
            ) -> GIOStatus,
        >,
        pub io_write: Option<
            unsafe extern "C" fn(
                *mut GIOChannel,
                *const gchar,
                gsize,
                *mut gsize,
                *mut *mut GError,
            ) -> GIOStatus,
        >,
        pub io_seek: Option<
            unsafe extern "C" fn(*mut GIOChannel, gint64, GSeekType, *mut *mut GError) -> GIOStatus,
        >,
        pub io_close: Option<unsafe extern "C" fn(*mut GIOChannel, *mut *mut GError) -> GIOStatus>,
        pub io_create_watch:
            Option<unsafe extern "C" fn(*mut GIOChannel, GIOCondition) -> *mut GSource>,
        pub io_free: Option<unsafe extern "C" fn(*mut GIOChannel) -> ()>,
        pub io_set_flags:
            Option<unsafe extern "C" fn(*mut GIOChannel, GIOFlags, *mut *mut GError) -> GIOStatus>,
        pub io_get_flags: Option<unsafe extern "C" fn(*mut GIOChannel) -> GIOFlags>,
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
    use c2rust_bitfields::BitfieldStruct;

    use super::gconvert_h::GIConv;
    use super::gerror_h::GError;
    use super::glibconfig_h::{gint64, gsize};
    use super::gmain_h::{GIOCondition, GSource};
    use super::gstring_h::GString;
    use super::gtypes_h::{gchar, gint, gpointer, guint};
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
pub mod lua_h {
    pub type lua_CFunction = Option<unsafe extern "C" fn(*mut lua_State) -> std::ffi::c_int>;
    unsafe extern "C" {
        pub type lua_State;
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
    use super::garray_h::GPtrArray;
    use super::giochannel_h::GIOChannel;
    use super::glibconfig_h::gsize;
    use super::gqueue_h::GQueue;
    use super::gtypes_h::{gboolean, gchar, gint, gpointer, guint};
    unsafe extern "C" {
        pub fn ipc_send(
            ipc: *mut ipc_endpoint_t,
            header: *const ipc_header_t,
            data: *const std::ffi::c_void,
        );
    }
}
pub mod lauxlib_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct luaL_Reg {
        pub name: *const std::ffi::c_char,
        pub func: lua_CFunction,
    }
    use super::__stddef_size_t_h::size_t;
    use super::lua_h::{lua_CFunction, lua_State};
    unsafe extern "C" {
        pub fn luaL_checklstring(
            L: *mut lua_State,
            numArg: std::ffi::c_int,
            l: *mut size_t,
        ) -> *const std::ffi::c_char;
    }
}
pub mod string_h {
    unsafe extern "C" {
        pub fn memcpy(
            _: *mut std::ffi::c_void,
            _: *const std::ffi::c_void,
            _: std::ffi::c_ulong,
        ) -> *mut std::ffi::c_void;
        pub fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    }
}
pub mod gmem_h {
    use super::glibconfig_h::gsize;
    use super::gtypes_h::gpointer;
    unsafe extern "C" {
        pub fn g_malloc(n_bytes: gsize) -> gpointer;
    }
}
pub mod gstrfuncs_h {
    #[inline(always)]
    pub unsafe extern "C" fn g_strdup_inline(
        mut str: *const std::ffi::c_char,
    ) -> *mut std::ffi::c_char {
        if 0 != 0 && str.is_null() {
            return NULL as *mut std::ffi::c_char;
        }
        if 0 != 0 && !str.is_null() && 0 != 0 {
            let len = (strlen(str)).wrapping_add(1 as std::ffi::c_int as std::ffi::c_ulong);
            let mut dup_str = g_malloc(len) as *mut std::ffi::c_char;
            return memcpy(
                dup_str as *mut std::ffi::c_void,
                str as *const std::ffi::c_void,
                len,
            ) as *mut std::ffi::c_char;
        }
        return g_strdup(str);
    }
    use super::__stddef_null_h::NULL;
    use super::__stddef_size_t_h::size_t;
    use super::gmem_h::g_malloc;
    use super::gtypes_h::gchar;
    use super::string_h::{memcpy, strlen};
    unsafe extern "C" {
        pub fn g_strdup(str: *const gchar) -> *mut gchar;
    }
}
pub mod luaclass_h {
    use super::gtypes_h::gchar;
    use super::lauxlib_h::luaL_Reg;
    use super::lua_h::lua_State;
    unsafe extern "C" {
        pub fn luaH_openlib(
            _: *mut lua_State,
            _: *const gchar,
            _: *const luaL_Reg,
            _: *const luaL_Reg,
        );
    }
}
pub mod clib_ipc_h {
    use super::gtypes_h::gint;
    use super::lua_h::lua_State;
    unsafe extern "C" {
        pub fn luaH_ipc_channel_new(L: *mut lua_State) -> gint;
    }
}
pub mod __stddef_null_h {
    pub const NULL: std::ffi::c_int = 0 as std::ffi::c_int;
    pub const NULL_0: std::ffi::c_int = 0 as std::ffi::c_int;
}
pub use self::__stddef_null_h::{NULL, NULL_0};
pub use self::__stddef_size_t_h::size_t;
use self::clib_ipc_h::luaH_ipc_channel_new;
pub use self::garray_h::{_GPtrArray, GPtrArray, g_ptr_array_add, g_ptr_array_new};
pub use self::gconvert_h::{_GIConv, GIConv};
pub use self::gerror_h::{_GError, GError};
pub use self::giochannel_h::{
    _GIOChannel, _GIOFuncs, G_IO_FLAG_APPEND, G_IO_FLAG_GET_MASK, G_IO_FLAG_IS_READABLE,
    G_IO_FLAG_IS_SEEKABLE, G_IO_FLAG_IS_WRITABLE, G_IO_FLAG_IS_WRITEABLE, G_IO_FLAG_MASK,
    G_IO_FLAG_NONBLOCK, G_IO_FLAG_NONE, G_IO_FLAG_SET_MASK, G_IO_STATUS_AGAIN, G_IO_STATUS_EOF,
    G_IO_STATUS_ERROR, G_IO_STATUS_NORMAL, G_SEEK_CUR, G_SEEK_END, G_SEEK_SET, GIOChannel,
    GIOFlags, GIOFuncs, GIOStatus, GSeekType,
};
pub use self::glibconfig_h::{gint64, gsize, guint32};
pub use self::glist_h::{_GList, GList};
pub use self::gmain_h::{
    _GMainContext, _GSource, _GSourceCallbackFuncs, _GSourceFuncs, _GSourcePrivate, G_IO_ERR,
    G_IO_HUP, G_IO_IN, G_IO_NVAL, G_IO_OUT, G_IO_PRI, GIOCondition, GMainContext, GSource,
    GSourceCallbackFuncs, GSourceDummyMarshal, GSourceFunc, GSourceFuncs, GSourceFuncsCheckFunc,
    GSourceFuncsDispatchFunc, GSourceFuncsFinalizeFunc, GSourceFuncsPrepareFunc, GSourcePrivate,
};
use self::gmem_h::g_malloc;
pub use self::gquark_h::GQuark;
pub use self::gqueue_h::{_GQueue, GQueue};
pub use self::gslist_h::{_GSList, GSList};
pub use self::gstrfuncs_h::{g_strdup, g_strdup_inline};
pub use self::gstring_h::{_GString, GString};
pub use self::gtypes_h::{gboolean, gchar, gint, gpointer, guint};
pub use self::ipc_h::{
    _ipc_endpoint_t, _ipc_header_t, _ipc_recv_state_t, IPC_ENDPOINT_CONNECTED,
    IPC_ENDPOINT_DISCONNECTED, IPC_ENDPOINT_FREED, IPC_TYPE_crash, IPC_TYPE_eval_js,
    IPC_TYPE_extension_init, IPC_TYPE_log, IPC_TYPE_lua_ipc, IPC_TYPE_lua_require_module,
    IPC_TYPE_page_created, IPC_TYPE_scroll, ipc_endpoint_status_t, ipc_endpoint_t, ipc_header_t,
    ipc_recv_state_t, ipc_send, ipc_type_t,
};
pub use self::lauxlib_h::{luaL_Reg, luaL_checklstring};
pub use self::lua_h::{lua_CFunction, lua_State};
use self::luaclass_h::luaH_openlib;
use self::string_h::{memcpy, strlen};
static mut required_web_modules: *mut GPtrArray = 0 as *const GPtrArray as *mut GPtrArray;
unsafe extern "C" fn luaH_require_web_module(mut L: *mut lua_State) -> std::ffi::c_int {
    let mut name = luaL_checklstring(L, -(1 as std::ffi::c_int), NULL_0 as *mut size_t);
    g_ptr_array_add(required_web_modules, g_strdup_inline(name) as gpointer);
    return luaH_ipc_channel_new(L);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn web_module_load_modules_on_endpoint(mut ipc: *mut ipc_endpoint_t) {
    let mut i = 0 as std::ffi::c_int as std::ffi::c_uint;
    while i < (*required_web_modules).len {
        let mut module_name = *((*required_web_modules).pdata).offset(i as isize) as *const gchar;
        let mut header = {
            let mut init = _ipc_header_t {
                length: (strlen(module_name))
                    .wrapping_add(1 as std::ffi::c_int as std::ffi::c_ulong)
                    as guint,
                type_0: IPC_TYPE_lua_require_module,
            };
            init
        };
        ipc_send(ipc, &mut header, module_name as *const std::ffi::c_void);
        i = i.wrapping_add(1);
        i;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn web_module_lib_setup(mut L: *mut lua_State) {
    static mut web_module_methods: [luaL_Reg; 2] = unsafe {
        [
            {
                let mut init = luaL_Reg {
                    name: b"__call\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_require_web_module
                            as unsafe extern "C" fn(*mut lua_State) -> std::ffi::c_int,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: NULL_0 as *const std::ffi::c_char,
                    func: ::core::mem::transmute::<libc::intptr_t, lua_CFunction>(
                        NULL_0 as libc::intptr_t,
                    ),
                };
                init
            },
        ]
    };
    luaH_openlib(
        L,
        b"require_web_module\0" as *const u8 as *const std::ffi::c_char,
        web_module_methods.as_ptr(),
        web_module_methods.as_ptr(),
    );
    required_web_modules = g_ptr_array_new();
}
