use ::libc;
use ::c2rust_bitfields;
#[c2rust::header_src = "/usr/lib/clang/20/include/__stddef_size_t.h:19"]
pub mod __stddef_size_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type size_t = std::ffi::c_ulong;
}
#[c2rust::header_src = "/usr/include/luajit-2.1/lua.h:19"]
pub mod lua_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "51:16"]
        pub type lua_State;
        #[c2rust::src_loc = "122:1"]
        pub fn lua_settop(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "150:1"]
        pub fn lua_tolstring(
            L: *mut lua_State,
            idx: std::ffi::c_int,
            len: *mut size_t,
        ) -> *const std::ffi::c_char;
        #[c2rust::src_loc = "179:1"]
        pub fn lua_getfield(
            L: *mut lua_State,
            idx: std::ffi::c_int,
            k: *const std::ffi::c_char,
        );
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/common.h:19"]
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
    extern "C" {
        #[c2rust::src_loc = "29:17"]
        pub static mut common: common_t;
    }
}
#[c2rust::header_src = "/usr/lib/glib-2.0/include/glibconfig.h:19"]
pub mod glibconfig_h {
    #[c2rust::src_loc = "46:1"]
    pub type guint8 = std::ffi::c_uchar;
    #[c2rust::src_loc = "56:1"]
    pub type gint32 = std::ffi::c_int;
    #[c2rust::src_loc = "57:1"]
    pub type guint32 = std::ffi::c_uint;
    #[c2rust::src_loc = "66:1"]
    pub type gint64 = std::ffi::c_long;
    #[c2rust::src_loc = "67:1"]
    pub type guint64 = std::ffi::c_ulong;
    #[c2rust::src_loc = "83:1"]
    pub type gsize = std::ffi::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:19"]
pub mod types_h {
    #[c2rust::src_loc = "154:1"]
    pub type __pid_t = std::ffi::c_int;
    #[c2rust::src_loc = "210:1"]
    pub type __socklen_t = std::ffi::c_uint;
}
#[c2rust::header_src = "/usr/include/time.h:19"]
pub mod time_h {
    #[c2rust::src_loc = "54:1"]
    pub type pid_t = __pid_t;
    use super::types_h::__pid_t;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtypes.h:19"]
pub mod gtypes_h {
    #[c2rust::src_loc = "52:1"]
    pub type gchar = std::ffi::c_char;
    #[c2rust::src_loc = "54:1"]
    pub type glong = std::ffi::c_long;
    #[c2rust::src_loc = "55:1"]
    pub type gint = std::ffi::c_int;
    #[c2rust::src_loc = "56:1"]
    pub type gboolean = gint;
    #[c2rust::src_loc = "60:1"]
    pub type gulong = std::ffi::c_ulong;
    #[c2rust::src_loc = "61:1"]
    pub type guint = std::ffi::c_uint;
    #[c2rust::src_loc = "63:1"]
    pub type gfloat = std::ffi::c_float;
    #[c2rust::src_loc = "64:1"]
    pub type gdouble = std::ffi::c_double;
    #[c2rust::src_loc = "109:1"]
    pub type gpointer = *mut std::ffi::c_void;
    #[c2rust::src_loc = "141:1"]
    pub type GFunc = Option::<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/garray.h:19"]
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
    use super::gtypes_h::{gpointer, guint, GFunc};
    extern "C" {
        #[c2rust::src_loc = "252:1"]
        pub fn g_ptr_array_foreach(
            array: *mut GPtrArray,
            func: GFunc,
            user_data: gpointer,
        );
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gquark.h:19"]
pub mod gquark_h {
    #[c2rust::src_loc = "38:1"]
    pub type GQuark = guint32;
    use super::glibconfig_h::guint32;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gerror.h:19"]
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
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gthread.h:19"]
pub mod gthread_h {
    #[c2rust::src_loc = "49:1"]
    pub type GThreadFunc = Option::<unsafe extern "C" fn(gpointer) -> gpointer>;
    #[c2rust::src_loc = "51:1"]
    pub type GThread = _GThread;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "60:7"]
    pub union _GMutex {
        pub p: gpointer,
        pub i: [guint; 2],
    }
    #[c2rust::src_loc = "53:1"]
    pub type GMutex = _GMutex;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "74:8"]
    pub struct _GCond {
        pub p: gpointer,
        pub i: [guint; 2],
    }
    #[c2rust::src_loc = "56:1"]
    pub type GCond = _GCond;
    use super::gtypes_h::{gpointer, guint, gchar};
    use super::deprecated_gthread_h::_GThread;
    extern "C" {
        #[c2rust::src_loc = "150:1"]
        pub fn g_thread_new(
            name: *const gchar,
            func: GThreadFunc,
            data: gpointer,
        ) -> *mut GThread;
        #[c2rust::src_loc = "175:1"]
        pub fn g_mutex_lock(mutex: *mut GMutex);
        #[c2rust::src_loc = "179:1"]
        pub fn g_mutex_unlock(mutex: *mut GMutex);
        #[c2rust::src_loc = "214:1"]
        pub fn g_cond_wait(cond: *mut GCond, mutex: *mut GMutex);
        #[c2rust::src_loc = "217:1"]
        pub fn g_cond_signal(cond: *mut GCond);
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/deprecated/gthread.h:19"]
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
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gconvert.h:19"]
pub mod gconvert_h {
    #[c2rust::src_loc = "85:1"]
    pub type GIConv = *mut _GIConv;
    extern "C" {
        #[c2rust::src_loc = "85:16"]
        pub type _GIConv;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gdataset.h:19"]
pub mod gdataset_h {
    #[c2rust::src_loc = "38:1"]
    pub type GData = _GData;
    extern "C" {
        #[c2rust::src_loc = "38:16"]
        pub type _GData;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gfileutils.h:19"]
pub mod gfileutils_h {
    #[c2rust::src_loc = "68:9"]
    pub type GFileTest = std::ffi::c_uint;
    #[c2rust::src_loc = "74:3"]
    pub const G_FILE_TEST_EXISTS: GFileTest = 16;
    #[c2rust::src_loc = "73:3"]
    pub const G_FILE_TEST_IS_EXECUTABLE: GFileTest = 8;
    #[c2rust::src_loc = "72:3"]
    pub const G_FILE_TEST_IS_DIR: GFileTest = 4;
    #[c2rust::src_loc = "71:3"]
    pub const G_FILE_TEST_IS_SYMLINK: GFileTest = 2;
    #[c2rust::src_loc = "70:3"]
    pub const G_FILE_TEST_IS_REGULAR: GFileTest = 1;
    use super::gtypes_h::{gchar, gboolean};
    extern "C" {
        #[c2rust::src_loc = "116:1"]
        pub fn g_file_test(filename: *const gchar, test: GFileTest) -> gboolean;
        #[c2rust::src_loc = "174:1"]
        pub fn g_build_filename(first_element: *const gchar, _: ...) -> *mut gchar;
        #[c2rust::src_loc = "210:1"]
        pub fn g_get_current_dir() -> *mut gchar;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/glist.h:19"]
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
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gslist.h:19"]
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
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gmain.h:19"]
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
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gstring.h:19"]
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
#[c2rust::header_src = "/usr/include/glib-2.0/glib/giochannel.h:19"]
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
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gvariant.h:19"]
pub mod gvariant_h {
    #[c2rust::src_loc = "36:1"]
    pub type GVariant = _GVariant;
    use super::gtypes_h::gchar;
    extern "C" {
        #[c2rust::src_loc = "36:16"]
        pub type _GVariant;
        #[c2rust::src_loc = "443:1"]
        pub fn g_variant_new(format_string: *const gchar, _: ...) -> *mut GVariant;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gqueue.h:19"]
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
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtree.h:19"]
pub mod gtree_h {
    #[c2rust::src_loc = "40:1"]
    pub type GTree = _GTree;
    extern "C" {
        #[c2rust::src_loc = "40:16"]
        pub type _GTree;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/gobject/gtype.h:19"]
pub mod gtype_h {
    #[c2rust::src_loc = "427:1"]
    pub type GType = gsize;
    #[c2rust::src_loc = "431:1"]
    pub type GValue = _GValue;
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
    use super::gvalue_h::_GValue;
}
#[c2rust::header_src = "/usr/include/glib-2.0/gobject/gvalue.h:19"]
pub mod gvalue_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "113:8"]
    pub struct _GValue {
        pub g_type: GType,
        pub data: [C2RustUnnamed; 2],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "119:3"]
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
#[c2rust::header_src = "/usr/include/glib-2.0/gobject/gclosure.h:19"]
pub mod gclosure_h {
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "173:8"]
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
    #[c2rust::src_loc = "78:1"]
    pub type GClosureNotifyData = _GClosureNotifyData;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "167:8"]
    pub struct _GClosureNotifyData {
        pub data: gpointer,
        pub notify: GClosureNotify,
    }
    #[c2rust::src_loc = "101:1"]
    pub type GClosureNotify = Option::<
        unsafe extern "C" fn(gpointer, *mut GClosure) -> (),
    >;
    #[c2rust::src_loc = "77:1"]
    pub type GClosure = _GClosure;
    #[c2rust::src_loc = "92:1"]
    pub type GCallback = Option::<unsafe extern "C" fn() -> ()>;
    use super::gtypes_h::{guint, gpointer};
    use super::gtype_h::GValue;
}
#[c2rust::header_src = "/usr/include/glib-2.0/gobject/gsignal.h:19"]
pub mod gsignal_h {
    #[c2rust::src_loc = "195:9"]
    pub type GConnectFlags = std::ffi::c_uint;
    #[c2rust::src_loc = "199:3"]
    pub const G_CONNECT_SWAPPED: GConnectFlags = 2;
    #[c2rust::src_loc = "198:3"]
    pub const G_CONNECT_AFTER: GConnectFlags = 1;
    #[c2rust::src_loc = "197:3"]
    pub const G_CONNECT_DEFAULT: GConnectFlags = 0;
    use super::gtypes_h::{gpointer, gchar, gulong};
    use super::gclosure_h::{GCallback, GClosureNotify};
    extern "C" {
        #[c2rust::src_loc = "433:1"]
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
#[c2rust::header_src = "/usr/include/glib-2.0/gobject/gobject.h:19"]
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
    #[c2rust::src_loc = "194:1"]
    pub type GInitiallyUnowned = _GObject;
    use super::gtype_h::GTypeInstance;
    use super::gtypes_h::guint;
    use super::gdataset_h::GData;
}
#[c2rust::header_src = "/usr/include/glib-2.0/gio/gapplication.h:19"]
pub mod gapplication_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "46:8"]
    pub struct _GApplication {
        pub parent_instance: GObject,
        pub priv_0: *mut GApplicationPrivate,
    }
    #[c2rust::src_loc = "43:1"]
    pub type GApplicationPrivate = _GApplicationPrivate;
    use super::gobject_h::GObject;
    extern "C" {
        #[c2rust::src_loc = "43:16"]
        pub type _GApplicationPrivate;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/gio/giotypes.h:19"]
pub mod giotypes_h {
    #[c2rust::src_loc = "59:1"]
    pub type GApplication = _GApplication;
    use super::gapplication_h::_GApplication;
}
#[c2rust::header_src = "/usr/include/unistd.h:19"]
pub mod unistd_h {
    #[c2rust::src_loc = "274:1"]
    pub type socklen_t = __socklen_t;
    use super::types_h::{__socklen_t, __pid_t};
    extern "C" {
        #[c2rust::src_loc = "287:1"]
        pub fn access(
            __name: *const std::ffi::c_char,
            __type: std::ffi::c_int,
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "650:1"]
        pub fn getpid() -> __pid_t;
        #[c2rust::src_loc = "858:1"]
        pub fn unlink(__name: *const std::ffi::c_char) -> std::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/gtk-3.0/gtk/gtkwidget.h:19"]
pub mod gtkwidget_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "133:8"]
    pub struct _GtkWidget {
        pub parent_instance: GInitiallyUnowned,
        pub priv_0: *mut GtkWidgetPrivate,
    }
    #[c2rust::src_loc = "66:1"]
    pub type GtkWidgetPrivate = _GtkWidgetPrivate;
    use super::gobject_h::GInitiallyUnowned;
    extern "C" {
        #[c2rust::src_loc = "66:16"]
        pub type _GtkWidgetPrivate;
    }
}
#[c2rust::header_src = "/usr/include/gtk-3.0/gtk/gtktypes.h:19"]
pub mod gtktypes_h {
    #[c2rust::src_loc = "46:1"]
    pub type GtkWidget = _GtkWidget;
    use super::gtkwidget_h::_GtkWidget;
}
#[c2rust::header_src = "/usr/include/gtk-3.0/gtk/gtkapplication.h:19"]
pub mod gtkapplication_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "43:8"]
    pub struct _GtkApplication {
        pub parent: GApplication,
        pub priv_0: *mut GtkApplicationPrivate,
    }
    #[c2rust::src_loc = "41:1"]
    pub type GtkApplicationPrivate = _GtkApplicationPrivate;
    #[c2rust::src_loc = "39:1"]
    pub type GtkApplication = _GtkApplication;
    use super::giotypes_h::GApplication;
    extern "C" {
        #[c2rust::src_loc = "41:16"]
        pub type _GtkApplicationPrivate;
    }
}
#[c2rust::header_src = "/usr/include/gtk-3.0/gtk/gtkcssprovider.h:19"]
pub mod gtkcssprovider_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "68:8"]
    pub struct _GtkCssProvider {
        pub parent_instance: GObject,
        pub priv_0: *mut GtkCssProviderPrivate,
    }
    #[c2rust::src_loc = "66:1"]
    pub type GtkCssProviderPrivate = _GtkCssProviderPrivate;
    #[c2rust::src_loc = "64:1"]
    pub type GtkCssProvider = _GtkCssProvider;
    use super::gobject_h::GObject;
    extern "C" {
        #[c2rust::src_loc = "66:16"]
        pub type _GtkCssProviderPrivate;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/globalconf.h:19"]
pub mod globalconf_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "32:9"]
    pub struct globalconf_t {
        pub application: *mut GtkApplication,
        pub config_dir: *mut gchar,
        pub data_dir: *mut gchar,
        pub cache_dir: *mut gchar,
        pub profile: *mut gchar,
        pub confpath: *mut gchar,
        pub execpath: *mut gchar,
        pub nounique: gboolean,
        pub argv: *mut GPtrArray,
        pub windows: *mut GPtrArray,
        pub webviews: *mut GPtrArray,
        pub stylesheets: *mut GPtrArray,
        pub starttime: gdouble,
    }
    use super::gtkapplication_h::GtkApplication;
    use super::gtypes_h::{gchar, gboolean, gdouble};
    use super::garray_h::GPtrArray;
    extern "C" {
        #[c2rust::src_loc = "71:21"]
        pub static mut globalconf: globalconf_t;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/log.h:20"]
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
#[c2rust::header_src = "/home/daana/git/luakit/common/ipc.h:20"]
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
    #[c2rust::src_loc = "60:16"]
    pub struct _ipc_lua_ipc_t {
        pub arg: [gchar; 0],
    }
    #[c2rust::src_loc = "60:1"]
    pub type ipc_lua_ipc_t = _ipc_lua_ipc_t;
    #[c2rust::src_loc = "64:9"]
    pub type ipc_scroll_subtype_t = std::ffi::c_uint;
    #[c2rust::src_loc = "67:5"]
    pub const IPC_SCROLL_TYPE_scroll: ipc_scroll_subtype_t = 2;
    #[c2rust::src_loc = "66:5"]
    pub const IPC_SCROLL_TYPE_winresize: ipc_scroll_subtype_t = 1;
    #[c2rust::src_loc = "65:5"]
    pub const IPC_SCROLL_TYPE_docresize: ipc_scroll_subtype_t = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "70:16"]
    pub struct _ipc_scroll_t {
        pub h: gint,
        pub v: gint,
        pub page_id: guint64,
        pub subtype: ipc_scroll_subtype_t,
    }
    #[c2rust::src_loc = "70:1"]
    pub type ipc_scroll_t = _ipc_scroll_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "76:16"]
    pub struct _ipc_page_created_t {
        pub page_id: guint64,
        pub pid: pid_t,
    }
    #[c2rust::src_loc = "76:1"]
    pub type ipc_page_created_t = _ipc_page_created_t;
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
    use super::gtypes_h::{guint, gchar, gint, gpointer, gboolean};
    use super::glibconfig_h::{guint64, gsize};
    use super::time_h::pid_t;
    use super::garray_h::GPtrArray;
    use super::giochannel_h::GIOChannel;
    use super::gqueue_h::GQueue;
    extern "C" {
        #[c2rust::src_loc = "127:1"]
        pub fn ipc_endpoint_new(name: *const gchar) -> *mut ipc_endpoint_t;
        #[c2rust::src_loc = "128:1"]
        pub fn ipc_endpoint_connect_to_socket(
            ipc: *mut ipc_endpoint_t,
            sock: std::ffi::c_int,
        );
        #[c2rust::src_loc = "138:1"]
        pub fn ipc_send(
            ipc: *mut ipc_endpoint_t,
            header: *const ipc_header_t,
            data: *const std::ffi::c_void,
        );
    }
}
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/webkit/WebKitWebContext.h:23"]
pub mod WebKitWebContext_h {
    #[c2rust::src_loc = "49:1"]
    pub type WebKitWebContext = _WebKitWebContext;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:1"]
    pub struct _WebKitWebContext {
        pub parent: GObject,
        pub priv_0: *mut WebKitWebContextPrivate,
    }
    #[c2rust::src_loc = "49:1"]
    pub type WebKitWebContextPrivate = _WebKitWebContextPrivate;
    use super::gobject_h::GObject;
    use super::gtypes_h::gchar;
    use super::gvariant_h::GVariant;
    extern "C" {
        #[c2rust::src_loc = "49:1"]
        pub type _WebKitWebContextPrivate;
        #[c2rust::src_loc = "264:1"]
        pub fn webkit_web_context_set_web_extensions_directory(
            context: *mut WebKitWebContext,
            directory: *const gchar,
        );
        #[c2rust::src_loc = "284:1"]
        pub fn webkit_web_context_set_web_extensions_initialization_user_data(
            context: *mut WebKitWebContext,
            user_data: *mut GVariant,
        );
    }
}
#[c2rust::header_src = "/usr/include/sys/un.h:26"]
pub mod un_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "29:39"]
    pub struct sockaddr_un {
        pub sun_family: sa_family_t,
        pub sun_path: [std::ffi::c_char; 108],
    }
    use super::sockaddr_h::sa_family_t;
}
#[c2rust::header_src = "/usr/include/bits/sockaddr.h:24"]
pub mod sockaddr_h {
    #[c2rust::src_loc = "28:1"]
    pub type sa_family_t = std::ffi::c_ushort;
}
#[c2rust::header_src = "/usr/include/bits/socket.h:24"]
pub mod socket_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "184:39"]
    pub struct sockaddr {
        pub sa_family: sa_family_t,
        pub sa_data: [std::ffi::c_char; 14],
    }
    use super::sockaddr_h::sa_family_t;
}
#[c2rust::header_src = "/usr/include/bits/socket_type.h:24"]
pub mod socket_type_h {
    #[c2rust::src_loc = "26:3"]
    pub const SOCK_STREAM: __socket_type = 1;
    #[c2rust::src_loc = "24:1"]
    pub type __socket_type = std::ffi::c_uint;
    #[c2rust::src_loc = "52:3"]
    pub const SOCK_NONBLOCK: __socket_type = 2048;
    #[c2rust::src_loc = "49:3"]
    pub const SOCK_CLOEXEC: __socket_type = 524288;
    #[c2rust::src_loc = "41:3"]
    pub const SOCK_PACKET: __socket_type = 10;
    #[c2rust::src_loc = "39:3"]
    pub const SOCK_DCCP: __socket_type = 6;
    #[c2rust::src_loc = "36:3"]
    pub const SOCK_SEQPACKET: __socket_type = 5;
    #[c2rust::src_loc = "34:3"]
    pub const SOCK_RDM: __socket_type = 4;
    #[c2rust::src_loc = "32:3"]
    pub const SOCK_RAW: __socket_type = 3;
    #[c2rust::src_loc = "29:3"]
    pub const SOCK_DGRAM: __socket_type = 2;
}
#[c2rust::header_src = "/home/daana/git/luakit/common/signal.h:33"]
pub mod signal_h {
    #[c2rust::src_loc = "29:1"]
    pub type signal_t = GTree;
    use super::gtree_h::GTree;
}
#[c2rust::header_src = "/home/daana/git/luakit/common/tokenize.h:33"]
pub mod tokenize_h {
    #[c2rust::src_loc = "7:9"]
    pub type luakit_token_t = std::ffi::c_uint;
    #[c2rust::src_loc = "282:5"]
    pub const L_TK_ZOOM_TEXT_ONLY: luakit_token_t = 274;
    #[c2rust::src_loc = "281:5"]
    pub const L_TK_ZOOM_LEVEL: luakit_token_t = 273;
    #[c2rust::src_loc = "280:5"]
    pub const L_TK_YPAGE_SIZE: luakit_token_t = 272;
    #[c2rust::src_loc = "279:5"]
    pub const L_TK_YMAX: luakit_token_t = 271;
    #[c2rust::src_loc = "278:5"]
    pub const L_TK_Y: luakit_token_t = 270;
    #[c2rust::src_loc = "277:5"]
    pub const L_TK_XPAGE_SIZE: luakit_token_t = 269;
    #[c2rust::src_loc = "276:5"]
    pub const L_TK_XMAX: luakit_token_t = 268;
    #[c2rust::src_loc = "275:5"]
    pub const L_TK_X: luakit_token_t = 267;
    #[c2rust::src_loc = "274:5"]
    pub const L_TK_WRAP_JS: luakit_token_t = 266;
    #[c2rust::src_loc = "273:5"]
    pub const L_TK_WIN_XID: luakit_token_t = 265;
    #[c2rust::src_loc = "272:5"]
    pub const L_TK_WINDOWS: luakit_token_t = 264;
    #[c2rust::src_loc = "271:5"]
    pub const L_TK_WINDOW: luakit_token_t = 263;
    #[c2rust::src_loc = "270:5"]
    pub const L_TK_WIDTH: luakit_token_t = 262;
    #[c2rust::src_loc = "269:5"]
    pub const L_TK_WEB_PROCESS_ID: luakit_token_t = 261;
    #[c2rust::src_loc = "268:5"]
    pub const L_TK_WEBVIEW: luakit_token_t = 260;
    #[c2rust::src_loc = "267:5"]
    pub const L_TK_WEBSITE_DATA: luakit_token_t = 259;
    #[c2rust::src_loc = "266:5"]
    pub const L_TK_WEBKIT_VERSION: luakit_token_t = 258;
    #[c2rust::src_loc = "265:5"]
    pub const L_TK_WEBKIT_USER_AGENT_VERSION: luakit_token_t = 257;
    #[c2rust::src_loc = "264:5"]
    pub const L_TK_WEBKIT2: luakit_token_t = 256;
    #[c2rust::src_loc = "263:5"]
    pub const L_TK_VPANED: luakit_token_t = 255;
    #[c2rust::src_loc = "262:5"]
    pub const L_TK_VISIBLE_CHILD: luakit_token_t = 254;
    #[c2rust::src_loc = "261:5"]
    pub const L_TK_VISIBLE: luakit_token_t = 253;
    #[c2rust::src_loc = "260:5"]
    pub const L_TK_VIDEOS_DIR: luakit_token_t = 252;
    #[c2rust::src_loc = "259:5"]
    pub const L_TK_VERSION: luakit_token_t = 251;
    #[c2rust::src_loc = "258:5"]
    pub const L_TK_VERBOSE: luakit_token_t = 250;
    #[c2rust::src_loc = "257:5"]
    pub const L_TK_VBOX: luakit_token_t = 249;
    #[c2rust::src_loc = "256:5"]
    pub const L_TK_VALUE: luakit_token_t = 248;
    #[c2rust::src_loc = "255:5"]
    pub const L_TK_USER_AGENT: luakit_token_t = 247;
    #[c2rust::src_loc = "254:5"]
    pub const L_TK_URI: luakit_token_t = 246;
    #[c2rust::src_loc = "253:5"]
    pub const L_TK_URGENCY_HINT: luakit_token_t = 245;
    #[c2rust::src_loc = "252:5"]
    pub const L_TK_TYPE: luakit_token_t = 244;
    #[c2rust::src_loc = "251:5"]
    pub const L_TK_TOTAL_SIZE: luakit_token_t = 243;
    #[c2rust::src_loc = "250:5"]
    pub const L_TK_TOP: luakit_token_t = 242;
    #[c2rust::src_loc = "249:5"]
    pub const L_TK_TOOLTIP: luakit_token_t = 241;
    #[c2rust::src_loc = "248:5"]
    pub const L_TK_TITLE: luakit_token_t = 240;
    #[c2rust::src_loc = "247:5"]
    pub const L_TK_TEXT_CONTENT: luakit_token_t = 239;
    #[c2rust::src_loc = "246:5"]
    pub const L_TK_TEXTWIDTH: luakit_token_t = 238;
    #[c2rust::src_loc = "245:5"]
    pub const L_TK_TEXT: luakit_token_t = 237;
    #[c2rust::src_loc = "244:5"]
    pub const L_TK_TEMPLATES_DIR: luakit_token_t = 236;
    #[c2rust::src_loc = "243:5"]
    pub const L_TK_TAG_NAME: luakit_token_t = 235;
    #[c2rust::src_loc = "242:5"]
    pub const L_TK_SYSTEM_DATA_DIRS: luakit_token_t = 234;
    #[c2rust::src_loc = "241:5"]
    pub const L_TK_SYSTEM_CONFIG_DIRS: luakit_token_t = 233;
    #[c2rust::src_loc = "240:5"]
    pub const L_TK_SWITCH: luakit_token_t = 232;
    #[c2rust::src_loc = "239:5"]
    pub const L_TK_SUGGESTED_FILENAME: luakit_token_t = 231;
    #[c2rust::src_loc = "238:5"]
    pub const L_TK_SUBMIT: luakit_token_t = 230;
    #[c2rust::src_loc = "237:5"]
    pub const L_TK_STYLESHEETS: luakit_token_t = 229;
    #[c2rust::src_loc = "236:5"]
    pub const L_TK_STYLE: luakit_token_t = 228;
    #[c2rust::src_loc = "235:5"]
    pub const L_TK_STOP: luakit_token_t = 227;
    #[c2rust::src_loc = "234:5"]
    pub const L_TK_STATUS: luakit_token_t = 226;
    #[c2rust::src_loc = "233:5"]
    pub const L_TK_STARTED: luakit_token_t = 225;
    #[c2rust::src_loc = "232:5"]
    pub const L_TK_START: luakit_token_t = 224;
    #[c2rust::src_loc = "231:5"]
    pub const L_TK_STACK: luakit_token_t = 223;
    #[c2rust::src_loc = "230:5"]
    pub const L_TK_SSL_TRUSTED: luakit_token_t = 222;
    #[c2rust::src_loc = "229:5"]
    pub const L_TK_SRC: luakit_token_t = 221;
    #[c2rust::src_loc = "228:5"]
    pub const L_TK_SPINNER: luakit_token_t = 220;
    #[c2rust::src_loc = "227:5"]
    pub const L_TK_SPELL_CHECKING_LANGUAGES: luakit_token_t = 219;
    #[c2rust::src_loc = "226:5"]
    pub const L_TK_SPACING: luakit_token_t = 218;
    #[c2rust::src_loc = "225:5"]
    pub const L_TK_SOURCE: luakit_token_t = 217;
    #[c2rust::src_loc = "224:5"]
    pub const L_TK_SOCKET: luakit_token_t = 216;
    #[c2rust::src_loc = "223:5"]
    pub const L_TK_SHOW_TABS: luakit_token_t = 215;
    #[c2rust::src_loc = "222:5"]
    pub const L_TK_SHOW_INSPECTOR: luakit_token_t = 214;
    #[c2rust::src_loc = "221:5"]
    pub const L_TK_SHOW_FRAME: luakit_token_t = 213;
    #[c2rust::src_loc = "220:5"]
    pub const L_TK_SHOW_BORDER: luakit_token_t = 212;
    #[c2rust::src_loc = "219:5"]
    pub const L_TK_SHOW: luakit_token_t = 211;
    #[c2rust::src_loc = "218:5"]
    pub const L_TK_SET_TITLE: luakit_token_t = 210;
    #[c2rust::src_loc = "217:5"]
    pub const L_TK_SET_PDFJS: luakit_token_t = 209;
    #[c2rust::src_loc = "216:5"]
    pub const L_TK_SET_FAVICON_FOR_URI: luakit_token_t = 208;
    #[c2rust::src_loc = "215:5"]
    pub const L_TK_SET_DEFAULT_SIZE: luakit_token_t = 207;
    #[c2rust::src_loc = "214:5"]
    pub const L_TK_SET_DARK_MODE: luakit_token_t = 206;
    #[c2rust::src_loc = "213:5"]
    pub const L_TK_SESSION_STATE: luakit_token_t = 205;
    #[c2rust::src_loc = "212:5"]
    pub const L_TK_SERIF_FONT_FAMILY: luakit_token_t = 204;
    #[c2rust::src_loc = "211:5"]
    pub const L_TK_SEND_KEY: luakit_token_t = 203;
    #[c2rust::src_loc = "210:5"]
    pub const L_TK_SELECT_REGION: luakit_token_t = 202;
    #[c2rust::src_loc = "209:5"]
    pub const L_TK_SELECTION: luakit_token_t = 201;
    #[c2rust::src_loc = "208:5"]
    pub const L_TK_SELECTABLE: luakit_token_t = 200;
    #[c2rust::src_loc = "207:5"]
    pub const L_TK_SECONDARY: luakit_token_t = 199;
    #[c2rust::src_loc = "206:5"]
    pub const L_TK_SEARCH_PREVIOUS: luakit_token_t = 198;
    #[c2rust::src_loc = "205:5"]
    pub const L_TK_SEARCH_NEXT: luakit_token_t = 197;
    #[c2rust::src_loc = "204:5"]
    pub const L_TK_SEARCH: luakit_token_t = 196;
    #[c2rust::src_loc = "203:5"]
    pub const L_TK_SCROLL_Y: luakit_token_t = 195;
    #[c2rust::src_loc = "202:5"]
    pub const L_TK_SCROLL_X: luakit_token_t = 194;
    #[c2rust::src_loc = "201:5"]
    pub const L_TK_SCROLLED: luakit_token_t = 193;
    #[c2rust::src_loc = "200:5"]
    pub const L_TK_SCROLLBARS: luakit_token_t = 192;
    #[c2rust::src_loc = "199:5"]
    pub const L_TK_SCROLL: luakit_token_t = 191;
    #[c2rust::src_loc = "198:5"]
    pub const L_TK_SCREEN: luakit_token_t = 190;
    #[c2rust::src_loc = "197:5"]
    pub const L_TK_SCALE: luakit_token_t = 189;
    #[c2rust::src_loc = "196:5"]
    pub const L_TK_SAVE: luakit_token_t = 188;
    #[c2rust::src_loc = "195:5"]
    pub const L_TK_SANS_SERIF_FONT_FAMILY: luakit_token_t = 187;
    #[c2rust::src_loc = "194:5"]
    pub const L_TK_ROOT_WIN_XID: luakit_token_t = 186;
    #[c2rust::src_loc = "193:5"]
    pub const L_TK_RIGHT: luakit_token_t = 185;
    #[c2rust::src_loc = "192:5"]
    pub const L_TK_RESOURCE_PATH: luakit_token_t = 184;
    #[c2rust::src_loc = "191:5"]
    pub const L_TK_REPLACE: luakit_token_t = 183;
    #[c2rust::src_loc = "190:5"]
    pub const L_TK_REORDER: luakit_token_t = 182;
    #[c2rust::src_loc = "189:5"]
    pub const L_TK_REMOVE_EVENT_LISTENER: luakit_token_t = 181;
    #[c2rust::src_loc = "188:5"]
    pub const L_TK_REMOVE: luakit_token_t = 180;
    #[c2rust::src_loc = "187:5"]
    pub const L_TK_RELOAD_BYPASS_CACHE: luakit_token_t = 179;
    #[c2rust::src_loc = "186:5"]
    pub const L_TK_RELOAD: luakit_token_t = 178;
    #[c2rust::src_loc = "185:5"]
    pub const L_TK_RECT: luakit_token_t = 177;
    #[c2rust::src_loc = "184:5"]
    pub const L_TK_QUERY: luakit_token_t = 176;
    #[c2rust::src_loc = "183:5"]
    pub const L_TK_PUBLIC_SHARE_DIR: luakit_token_t = 175;
    #[c2rust::src_loc = "182:5"]
    pub const L_TK_PROXY_URI: luakit_token_t = 174;
    #[c2rust::src_loc = "181:5"]
    pub const L_TK_PROGRESS: luakit_token_t = 173;
    #[c2rust::src_loc = "180:5"]
    pub const L_TK_PROCESS_LIMIT: luakit_token_t = 172;
    #[c2rust::src_loc = "179:5"]
    pub const L_TK_PRIVATE: luakit_token_t = 171;
    #[c2rust::src_loc = "178:5"]
    pub const L_TK_PRINT_BACKGROUNDS: luakit_token_t = 170;
    #[c2rust::src_loc = "177:5"]
    pub const L_TK_PRIMARY: luakit_token_t = 169;
    #[c2rust::src_loc = "176:5"]
    pub const L_TK_PREV_SIBLING: luakit_token_t = 168;
    #[c2rust::src_loc = "175:5"]
    pub const L_TK_POSITION: luakit_token_t = 167;
    #[c2rust::src_loc = "174:5"]
    pub const L_TK_PLUGGED: luakit_token_t = 166;
    #[c2rust::src_loc = "173:5"]
    pub const L_TK_PICTURES_DIR: luakit_token_t = 165;
    #[c2rust::src_loc = "172:5"]
    pub const L_TK_PICTOGRAPH_FONT_FAMILY: luakit_token_t = 164;
    #[c2rust::src_loc = "171:5"]
    pub const L_TK_PATTERN: luakit_token_t = 163;
    #[c2rust::src_loc = "170:5"]
    pub const L_TK_PARENT: luakit_token_t = 162;
    #[c2rust::src_loc = "169:5"]
    pub const L_TK_PACK2: luakit_token_t = 161;
    #[c2rust::src_loc = "168:5"]
    pub const L_TK_PACK1: luakit_token_t = 160;
    #[c2rust::src_loc = "167:5"]
    pub const L_TK_PACK: luakit_token_t = 159;
    #[c2rust::src_loc = "166:5"]
    pub const L_TK_OWNER_DOCUMENT: luakit_token_t = 158;
    #[c2rust::src_loc = "165:5"]
    pub const L_TK_OVERLAY: luakit_token_t = 157;
    #[c2rust::src_loc = "164:5"]
    pub const L_TK_OPTIONS: luakit_token_t = 156;
    #[c2rust::src_loc = "163:5"]
    pub const L_TK_NOUNIQUE: luakit_token_t = 155;
    #[c2rust::src_loc = "162:5"]
    pub const L_TK_NOTEBOOK: luakit_token_t = 154;
    #[c2rust::src_loc = "161:5"]
    pub const L_TK_NEXT_SIBLING: luakit_token_t = 153;
    #[c2rust::src_loc = "160:5"]
    pub const L_TK_NAME: luakit_token_t = 152;
    #[c2rust::src_loc = "159:5"]
    pub const L_TK_MUSIC_DIR: luakit_token_t = 151;
    #[c2rust::src_loc = "158:5"]
    pub const L_TK_MONOSPACE_FONT_FAMILY: luakit_token_t = 150;
    #[c2rust::src_loc = "157:5"]
    pub const L_TK_MIN_SIZE: luakit_token_t = 149;
    #[c2rust::src_loc = "156:5"]
    pub const L_TK_MINIMUM_FONT_SIZE: luakit_token_t = 148;
    #[c2rust::src_loc = "155:5"]
    pub const L_TK_MIME_TYPE: luakit_token_t = 147;
    #[c2rust::src_loc = "154:5"]
    pub const L_TK_MEDIA_PLAYBACK_REQUIRES_GESTURE: luakit_token_t = 146;
    #[c2rust::src_loc = "153:5"]
    pub const L_TK_MEDIA_PLAYBACK_ALLOWS_INLINE: luakit_token_t = 145;
    #[c2rust::src_loc = "152:5"]
    pub const L_TK_MAXIMIZED: luakit_token_t = 144;
    #[c2rust::src_loc = "151:5"]
    pub const L_TK_MARGIN_TOP: luakit_token_t = 143;
    #[c2rust::src_loc = "150:5"]
    pub const L_TK_MARGIN_RIGHT: luakit_token_t = 142;
    #[c2rust::src_loc = "149:5"]
    pub const L_TK_MARGIN_LEFT: luakit_token_t = 141;
    #[c2rust::src_loc = "148:5"]
    pub const L_TK_MARGIN_BOTTOM: luakit_token_t = 140;
    #[c2rust::src_loc = "147:5"]
    pub const L_TK_MARGIN: luakit_token_t = 139;
    #[c2rust::src_loc = "146:5"]
    pub const L_TK_LOAD_STRING: luakit_token_t = 138;
    #[c2rust::src_loc = "145:5"]
    pub const L_TK_LOADING: luakit_token_t = 137;
    #[c2rust::src_loc = "144:5"]
    pub const L_TK_LEFT: luakit_token_t = 136;
    #[c2rust::src_loc = "143:5"]
    pub const L_TK_LAST_CHILD: luakit_token_t = 135;
    #[c2rust::src_loc = "142:5"]
    pub const L_TK_LABEL: luakit_token_t = 134;
    #[c2rust::src_loc = "141:5"]
    pub const L_TK_JAVASCRIPT_CAN_OPEN_WINDOWS_AUTOMATICALLY: luakit_token_t = 133;
    #[c2rust::src_loc = "140:5"]
    pub const L_TK_JAVASCRIPT_CAN_ACCESS_CLIPBOARD: luakit_token_t = 132;
    #[c2rust::src_loc = "139:5"]
    pub const L_TK_IS_PLAYING_AUDIO: luakit_token_t = 131;
    #[c2rust::src_loc = "138:5"]
    pub const L_TK_IS_LOADING: luakit_token_t = 130;
    #[c2rust::src_loc = "137:5"]
    pub const L_TK_IS_ALIVE: luakit_token_t = 129;
    #[c2rust::src_loc = "136:5"]
    pub const L_TK_INVALIDATE: luakit_token_t = 128;
    #[c2rust::src_loc = "135:5"]
    pub const L_TK_INTERVAL: luakit_token_t = 127;
    #[c2rust::src_loc = "134:5"]
    pub const L_TK_INSTALL_PATHS: luakit_token_t = 126;
    #[c2rust::src_loc = "133:5"]
    pub const L_TK_INSTALL_PATH: luakit_token_t = 125;
    #[c2rust::src_loc = "132:5"]
    pub const L_TK_INSPECTOR: luakit_token_t = 124;
    #[c2rust::src_loc = "131:5"]
    pub const L_TK_INSERT: luakit_token_t = 123;
    #[c2rust::src_loc = "130:5"]
    pub const L_TK_INNER_WIDTH: luakit_token_t = 122;
    #[c2rust::src_loc = "129:5"]
    pub const L_TK_INNER_HTML: luakit_token_t = 121;
    #[c2rust::src_loc = "128:5"]
    pub const L_TK_INNER_HEIGHT: luakit_token_t = 120;
    #[c2rust::src_loc = "127:5"]
    pub const L_TK_INDEXOF: luakit_token_t = 119;
    #[c2rust::src_loc = "126:5"]
    pub const L_TK_IMAGE: luakit_token_t = 118;
    #[c2rust::src_loc = "125:5"]
    pub const L_TK_ID: luakit_token_t = 117;
    #[c2rust::src_loc = "124:5"]
    pub const L_TK_ICON: luakit_token_t = 116;
    #[c2rust::src_loc = "123:5"]
    pub const L_TK_HREF: luakit_token_t = 115;
    #[c2rust::src_loc = "122:5"]
    pub const L_TK_HPANED: luakit_token_t = 114;
    #[c2rust::src_loc = "121:5"]
    pub const L_TK_HOVERED_URI: luakit_token_t = 113;
    #[c2rust::src_loc = "120:5"]
    pub const L_TK_HOMOGENEOUS: luakit_token_t = 112;
    #[c2rust::src_loc = "119:5"]
    pub const L_TK_HISTORY: luakit_token_t = 111;
    #[c2rust::src_loc = "118:5"]
    pub const L_TK_HIDE: luakit_token_t = 110;
    #[c2rust::src_loc = "117:5"]
    pub const L_TK_HEIGHT: luakit_token_t = 109;
    #[c2rust::src_loc = "116:5"]
    pub const L_TK_HBOX: luakit_token_t = 108;
    #[c2rust::src_loc = "115:5"]
    pub const L_TK_HARDWARE_ACCELERATION_POLICY: luakit_token_t = 107;
    #[c2rust::src_loc = "114:5"]
    pub const L_TK_GO_FORWARD: luakit_token_t = 106;
    #[c2rust::src_loc = "113:5"]
    pub const L_TK_GO_BACK: luakit_token_t = 105;
    #[c2rust::src_loc = "112:5"]
    pub const L_TK_GET_TITLE: luakit_token_t = 104;
    #[c2rust::src_loc = "111:5"]
    pub const L_TK_GET_SOURCE: luakit_token_t = 103;
    #[c2rust::src_loc = "110:5"]
    pub const L_TK_FULLSCREEN: luakit_token_t = 102;
    #[c2rust::src_loc = "109:5"]
    pub const L_TK_FONT: luakit_token_t = 101;
    #[c2rust::src_loc = "108:5"]
    pub const L_TK_FOCUSED: luakit_token_t = 100;
    #[c2rust::src_loc = "107:5"]
    pub const L_TK_FOCUS: luakit_token_t = 99;
    #[c2rust::src_loc = "106:5"]
    pub const L_TK_FIRST_CHILD: luakit_token_t = 98;
    #[c2rust::src_loc = "105:5"]
    pub const L_TK_FINISHED: luakit_token_t = 97;
    #[c2rust::src_loc = "104:5"]
    pub const L_TK_FILL: luakit_token_t = 96;
    #[c2rust::src_loc = "103:5"]
    pub const L_TK_FILENAME: luakit_token_t = 95;
    #[c2rust::src_loc = "102:5"]
    pub const L_TK_FG: luakit_token_t = 94;
    #[c2rust::src_loc = "101:5"]
    pub const L_TK_FETCH: luakit_token_t = 93;
    #[c2rust::src_loc = "100:5"]
    pub const L_TK_FANTASY_FONT_FAMILY: luakit_token_t = 92;
    #[c2rust::src_loc = "99:5"]
    pub const L_TK_EXECPATH: luakit_token_t = 91;
    #[c2rust::src_loc = "98:5"]
    pub const L_TK_EVENTBOX: luakit_token_t = 90;
    #[c2rust::src_loc = "97:5"]
    pub const L_TK_EVAL_JS: luakit_token_t = 89;
    #[c2rust::src_loc = "96:5"]
    pub const L_TK_ERROR: luakit_token_t = 88;
    #[c2rust::src_loc = "95:5"]
    pub const L_TK_ENTRY: luakit_token_t = 87;
    #[c2rust::src_loc = "94:5"]
    pub const L_TK_END: luakit_token_t = 86;
    #[c2rust::src_loc = "93:5"]
    pub const L_TK_ENABLE_XSS_AUDITOR: luakit_token_t = 85;
    #[c2rust::src_loc = "92:5"]
    pub const L_TK_ENABLE_WRITE_CONSOLE_MESSAGES_TO_STDOUT: luakit_token_t = 84;
    #[c2rust::src_loc = "91:5"]
    pub const L_TK_ENABLE_WEBGL: luakit_token_t = 83;
    #[c2rust::src_loc = "90:5"]
    pub const L_TK_ENABLE_WEBAUDIO: luakit_token_t = 82;
    #[c2rust::src_loc = "89:5"]
    pub const L_TK_ENABLE_TABS_TO_LINKS: luakit_token_t = 81;
    #[c2rust::src_loc = "88:5"]
    pub const L_TK_ENABLE_SPELL_CHECKING: luakit_token_t = 80;
    #[c2rust::src_loc = "87:5"]
    pub const L_TK_ENABLE_SPATIAL_NAVIGATION: luakit_token_t = 79;
    #[c2rust::src_loc = "86:5"]
    pub const L_TK_ENABLE_SMOOTH_SCROLLING: luakit_token_t = 78;
    #[c2rust::src_loc = "85:5"]
    pub const L_TK_ENABLE_SITE_SPECIFIC_QUIRKS: luakit_token_t = 77;
    #[c2rust::src_loc = "84:5"]
    pub const L_TK_ENABLE_SCRIPTS: luakit_token_t = 76;
    #[c2rust::src_loc = "83:5"]
    pub const L_TK_ENABLE_RESIZABLE_TEXT_AREAS: luakit_token_t = 75;
    #[c2rust::src_loc = "82:5"]
    pub const L_TK_ENABLE_PLUGINS: luakit_token_t = 74;
    #[c2rust::src_loc = "81:5"]
    pub const L_TK_ENABLE_PAGE_CACHE: luakit_token_t = 73;
    #[c2rust::src_loc = "80:5"]
    pub const L_TK_ENABLE_MEDIA_STREAM: luakit_token_t = 72;
    #[c2rust::src_loc = "79:5"]
    pub const L_TK_ENABLE_MEDIASOURCE: luakit_token_t = 71;
    #[c2rust::src_loc = "78:5"]
    pub const L_TK_ENABLE_JAVASCRIPT: luakit_token_t = 70;
    #[c2rust::src_loc = "77:5"]
    pub const L_TK_ENABLE_JAVA: luakit_token_t = 69;
    #[c2rust::src_loc = "76:5"]
    pub const L_TK_ENABLE_HYPERLINK_AUDITING: luakit_token_t = 68;
    #[c2rust::src_loc = "75:5"]
    pub const L_TK_ENABLE_HTML5_LOCAL_STORAGE: luakit_token_t = 67;
    #[c2rust::src_loc = "74:5"]
    pub const L_TK_ENABLE_HTML5_DATABASE: luakit_token_t = 66;
    #[c2rust::src_loc = "73:5"]
    pub const L_TK_ENABLE_FULLSCREEN: luakit_token_t = 65;
    #[c2rust::src_loc = "72:5"]
    pub const L_TK_ENABLE_FRAME_FLATTENING: luakit_token_t = 64;
    #[c2rust::src_loc = "71:5"]
    pub const L_TK_ENABLE_DNS_PREFETCHING: luakit_token_t = 63;
    #[c2rust::src_loc = "70:5"]
    pub const L_TK_ENABLE_DEVELOPER_EXTRAS: luakit_token_t = 62;
    #[c2rust::src_loc = "69:5"]
    pub const L_TK_ENABLE_CARET_BROWSING: luakit_token_t = 61;
    #[c2rust::src_loc = "68:5"]
    pub const L_TK_ENABLE_ACCELERATED_2D_CANVAS: luakit_token_t = 60;
    #[c2rust::src_loc = "67:5"]
    pub const L_TK_ELEMENT_FROM_POINT: luakit_token_t = 59;
    #[c2rust::src_loc = "66:5"]
    pub const L_TK_ELAPSED_TIME: luakit_token_t = 58;
    #[c2rust::src_loc = "65:5"]
    pub const L_TK_EDITABLE: luakit_token_t = 57;
    #[c2rust::src_loc = "64:5"]
    pub const L_TK_DRAW_COMPOSITING_INDICATORS: luakit_token_t = 56;
    #[c2rust::src_loc = "63:5"]
    pub const L_TK_DRAWING_AREA: luakit_token_t = 55;
    #[c2rust::src_loc = "62:5"]
    pub const L_TK_DOWNLOAD_DIR: luakit_token_t = 54;
    #[c2rust::src_loc = "61:5"]
    pub const L_TK_DOCUMENTS_DIR: luakit_token_t = 53;
    #[c2rust::src_loc = "60:5"]
    pub const L_TK_DOCUMENT: luakit_token_t = 52;
    #[c2rust::src_loc = "59:5"]
    pub const L_TK_DEV_PATHS: luakit_token_t = 51;
    #[c2rust::src_loc = "58:5"]
    pub const L_TK_DESTROY: luakit_token_t = 50;
    #[c2rust::src_loc = "57:5"]
    pub const L_TK_DESTINATION: luakit_token_t = 49;
    #[c2rust::src_loc = "56:5"]
    pub const L_TK_DESKTOP_DIR: luakit_token_t = 48;
    #[c2rust::src_loc = "55:5"]
    pub const L_TK_DEFAULT_MONOSPACE_FONT_SIZE: luakit_token_t = 47;
    #[c2rust::src_loc = "54:5"]
    pub const L_TK_DEFAULT_FONT_SIZE: luakit_token_t = 46;
    #[c2rust::src_loc = "53:5"]
    pub const L_TK_DEFAULT_FONT_FAMILY: luakit_token_t = 45;
    #[c2rust::src_loc = "52:5"]
    pub const L_TK_DEFAULT_CHARSET: luakit_token_t = 44;
    #[c2rust::src_loc = "51:5"]
    pub const L_TK_DECORATED: luakit_token_t = 43;
    #[c2rust::src_loc = "50:5"]
    pub const L_TK_DATA_DIR: luakit_token_t = 42;
    #[c2rust::src_loc = "49:5"]
    pub const L_TK_CURSIVE_FONT_FAMILY: luakit_token_t = 41;
    #[c2rust::src_loc = "48:5"]
    pub const L_TK_CURRENT_SIZE: luakit_token_t = 40;
    #[c2rust::src_loc = "47:5"]
    pub const L_TK_CURRENT: luakit_token_t = 39;
    #[c2rust::src_loc = "46:5"]
    pub const L_TK_CSS: luakit_token_t = 38;
    #[c2rust::src_loc = "45:5"]
    pub const L_TK_CREATE_ELEMENT: luakit_token_t = 37;
    #[c2rust::src_loc = "44:5"]
    pub const L_TK_CRASH: luakit_token_t = 36;
    #[c2rust::src_loc = "43:5"]
    pub const L_TK_COUNT: luakit_token_t = 35;
    #[c2rust::src_loc = "42:5"]
    pub const L_TK_COOKIES_STORAGE: luakit_token_t = 34;
    #[c2rust::src_loc = "41:5"]
    pub const L_TK_CONFPATH: luakit_token_t = 33;
    #[c2rust::src_loc = "40:5"]
    pub const L_TK_CONFIG_DIR: luakit_token_t = 32;
    #[c2rust::src_loc = "39:5"]
    pub const L_TK_CLOSE_INSPECTOR: luakit_token_t = 31;
    #[c2rust::src_loc = "38:5"]
    pub const L_TK_CLIPBOARD: luakit_token_t = 30;
    #[c2rust::src_loc = "37:5"]
    pub const L_TK_CLIENT_RECTS: luakit_token_t = 29;
    #[c2rust::src_loc = "36:5"]
    pub const L_TK_CLICK: luakit_token_t = 28;
    #[c2rust::src_loc = "35:5"]
    pub const L_TK_CLEAR_SEARCH: luakit_token_t = 27;
    #[c2rust::src_loc = "34:5"]
    pub const L_TK_CLEAR: luakit_token_t = 26;
    #[c2rust::src_loc = "33:5"]
    pub const L_TK_CHILD_COUNT: luakit_token_t = 25;
    #[c2rust::src_loc = "32:5"]
    pub const L_TK_CHILDREN: luakit_token_t = 24;
    #[c2rust::src_loc = "31:5"]
    pub const L_TK_CHILD: luakit_token_t = 23;
    #[c2rust::src_loc = "30:5"]
    pub const L_TK_CHECKED: luakit_token_t = 22;
    #[c2rust::src_loc = "29:5"]
    pub const L_TK_CERTIFICATE: luakit_token_t = 21;
    #[c2rust::src_loc = "28:5"]
    pub const L_TK_CENTER: luakit_token_t = 20;
    #[c2rust::src_loc = "27:5"]
    pub const L_TK_CAN_GO_FORWARD: luakit_token_t = 19;
    #[c2rust::src_loc = "26:5"]
    pub const L_TK_CAN_GO_BACK: luakit_token_t = 18;
    #[c2rust::src_loc = "25:5"]
    pub const L_TK_CAN_FOCUS: luakit_token_t = 17;
    #[c2rust::src_loc = "24:5"]
    pub const L_TK_CACHE_DIR: luakit_token_t = 16;
    #[c2rust::src_loc = "23:5"]
    pub const L_TK_BOTTOM: luakit_token_t = 15;
    #[c2rust::src_loc = "22:5"]
    pub const L_TK_BODY: luakit_token_t = 14;
    #[c2rust::src_loc = "21:5"]
    pub const L_TK_BG: luakit_token_t = 13;
    #[c2rust::src_loc = "20:5"]
    pub const L_TK_BASELINE: luakit_token_t = 12;
    #[c2rust::src_loc = "19:5"]
    pub const L_TK_AUTO_LOAD_IMAGES: luakit_token_t = 11;
    #[c2rust::src_loc = "18:5"]
    pub const L_TK_ATTR: luakit_token_t = 10;
    #[c2rust::src_loc = "17:5"]
    pub const L_TK_APPEND: luakit_token_t = 9;
    #[c2rust::src_loc = "16:5"]
    pub const L_TK_ALLOW_UNIVERSAL_ACCESS_FROM_FILE_URLS: luakit_token_t = 8;
    #[c2rust::src_loc = "15:5"]
    pub const L_TK_ALLOW_OVERWRITE: luakit_token_t = 7;
    #[c2rust::src_loc = "14:5"]
    pub const L_TK_ALLOW_MODAL_DIALOGS: luakit_token_t = 6;
    #[c2rust::src_loc = "13:5"]
    pub const L_TK_ALLOW_FILE_ACCESS_FROM_FILE_URLS: luakit_token_t = 5;
    #[c2rust::src_loc = "12:5"]
    pub const L_TK_ALLOW_CERTIFICATE: luakit_token_t = 4;
    #[c2rust::src_loc = "11:5"]
    pub const L_TK_ALIGN: luakit_token_t = 3;
    #[c2rust::src_loc = "10:5"]
    pub const L_TK_ADD_EVENT_LISTENER: luakit_token_t = 2;
    #[c2rust::src_loc = "9:5"]
    pub const L_TK_ACCEPT_POLICY: luakit_token_t = 1;
    #[c2rust::src_loc = "8:5"]
    pub const L_TK_UNKNOWN: luakit_token_t = 0;
}
#[c2rust::header_src = "/home/daana/git/luakit/clib/widget.h:34"]
pub mod widget_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "65:8"]
    pub struct widget_t {
        pub signals: *mut signal_t,
        pub info: *const widget_info_t,
        pub destructor: Option::<widget_destructor_t>,
        pub index: Option::<
            unsafe extern "C" fn(*mut lua_State, *mut widget_t, luakit_token_t) -> gint,
        >,
        pub newindex: Option::<
            unsafe extern "C" fn(*mut lua_State, *mut widget_t, luakit_token_t) -> gint,
        >,
        pub ref_0: gpointer,
        pub widget: *mut GtkWidget,
        pub provider: *mut GtkCssProvider,
        pub prev_width: gint,
        pub prev_height: gint,
        pub data: gpointer,
    }
    #[c2rust::src_loc = "41:1"]
    pub type widget_destructor_t = unsafe extern "C" fn(*mut widget_t) -> ();
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "58:15"]
    pub struct widget_info_t {
        pub tok: luakit_token_t,
        pub name: *const gchar,
        pub wc: Option::<widget_constructor_t>,
    }
    #[c2rust::src_loc = "40:1"]
    pub type widget_constructor_t = unsafe extern "C" fn(
        *mut lua_State,
        *mut widget_t,
        luakit_token_t,
    ) -> *mut widget_t;
    use super::signal_h::signal_t;
    use super::gtypes_h::{gint, gpointer, gchar};
    use super::lua_h::lua_State;
    use super::tokenize_h::luakit_token_t;
    use super::gtktypes_h::GtkWidget;
    use super::gtkcssprovider_h::GtkCssProvider;
}
#[c2rust::header_src = "/usr/include/string.h:19"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "61:14"]
        pub fn memset(
            _: *mut std::ffi::c_void,
            _: std::ffi::c_int,
            _: std::ffi::c_ulong,
        ) -> *mut std::ffi::c_void;
        #[c2rust::src_loc = "141:14"]
        pub fn strcpy(
            _: *mut std::ffi::c_char,
            _: *const std::ffi::c_char,
        ) -> *mut std::ffi::c_char;
        #[c2rust::src_loc = "407:15"]
        pub fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
        #[c2rust::src_loc = "419:14"]
        pub fn strerror(_: std::ffi::c_int) -> *mut std::ffi::c_char;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gutils.h:19"]
pub mod gutils_h {
    use super::gtypes_h::gchar;
    extern "C" {
        #[c2rust::src_loc = "45:1"]
        pub fn g_get_tmp_dir() -> *const gchar;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:19"]
pub mod stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "734:1"]
        pub fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> std::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gmem.h:19"]
pub mod gmem_h {
    use super::gtypes_h::gpointer;
    extern "C" {
        #[c2rust::src_loc = "73:1"]
        pub fn g_free(mem: gpointer);
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gstrfuncs.h:19"]
pub mod gstrfuncs_h {
    use super::gtypes_h::gchar;
    extern "C" {
        #[c2rust::src_loc = "285:1"]
        pub fn g_strdup_printf(format: *const gchar, _: ...) -> *mut gchar;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/grand.h:19"]
pub mod grand_h {
    use super::glibconfig_h::gint32;
    extern "C" {
        #[c2rust::src_loc = "89:1"]
        pub fn g_random_int_range(begin: gint32, end: gint32) -> gint32;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:19"]
pub mod errno_h {
    extern "C" {
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut std::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/sys/socket.h:24"]
pub mod sys_socket_h {
    use super::socket_h::sockaddr;
    use super::unistd_h::socklen_t;
    extern "C" {
        #[c2rust::src_loc = "102:1"]
        pub fn socket(
            __domain: std::ffi::c_int,
            __type: std::ffi::c_int,
            __protocol: std::ffi::c_int,
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "112:1"]
        pub fn bind(
            __fd: std::ffi::c_int,
            __addr: *const sockaddr,
            __len: socklen_t,
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "296:1"]
        pub fn listen(__fd: std::ffi::c_int, __n: std::ffi::c_int) -> std::ffi::c_int;
        #[c2rust::src_loc = "306:1"]
        pub fn accept(
            __fd: std::ffi::c_int,
            __addr: *mut sockaddr,
            __addr_len: *mut socklen_t,
        ) -> std::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gstdio.h:28"]
pub mod gstdio_h {
    use super::gtypes_h::gchar;
    extern "C" {
        #[c2rust::src_loc = "93:1"]
        pub fn g_unlink(filename: *const gchar) -> std::ffi::c_int;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/clib/web_module.h:32"]
pub mod web_module_h {
    use super::ipc_h::ipc_endpoint_t;
    extern "C" {
        #[c2rust::src_loc = "25:1"]
        pub fn web_module_load_modules_on_endpoint(ipc: *mut ipc_endpoint_t);
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/clib/ipc.h:36"]
pub mod clib_ipc_h {
    use super::lua_h::lua_State;
    use super::gtypes_h::{gchar, guint};
    extern "C" {
        #[c2rust::src_loc = "37:1"]
        pub fn ipc_channel_recv(L: *mut lua_State, arg: *const gchar, arglen: guint);
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/web_context.h:37"]
pub mod web_context_h {
    use super::WebKitWebContext_h::WebKitWebContext;
    extern "C" {
        #[c2rust::src_loc = "28:1"]
        pub fn web_context_get() -> *mut WebKitWebContext;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/widgets/webview.h:38"]
pub mod webview_h {
    use super::glibconfig_h::guint64;
    use super::widget_h::widget_t;
    use super::ipc_h::ipc_endpoint_t;
    use super::time_h::pid_t;
    extern "C" {
        #[c2rust::src_loc = "29:1"]
        pub fn webview_get_by_id(view_id: guint64) -> *mut widget_t;
        #[c2rust::src_loc = "30:1"]
        pub fn webview_connect_to_endpoint(w: *mut widget_t, ipc: *mut ipc_endpoint_t);
        #[c2rust::src_loc = "31:1"]
        pub fn webview_set_web_process_id(w: *mut widget_t, pid: pid_t);
    }
}
pub use self::__stddef_size_t_h::size_t;
use self::lua_h::{lua_State, lua_settop, lua_tolstring, lua_getfield};
pub use self::common_h::{_common_t, common_t, common};
pub use self::glibconfig_h::{guint8, gint32, guint32, gint64, guint64, gsize};
pub use self::types_h::{__pid_t, __socklen_t};
pub use self::time_h::pid_t;
pub use self::gtypes_h::{
    gchar, glong, gint, gboolean, gulong, guint, gfloat, gdouble, gpointer, GFunc,
};
pub use self::garray_h::{_GPtrArray, GPtrArray, g_ptr_array_foreach};
pub use self::gquark_h::GQuark;
pub use self::gerror_h::{_GError, GError};
pub use self::gthread_h::{
    GThreadFunc, GThread, _GMutex, GMutex, _GCond, GCond, g_thread_new, g_mutex_lock,
    g_mutex_unlock, g_cond_wait, g_cond_signal,
};
pub use self::deprecated_gthread_h::{
    _GThread, GThreadPriority, G_THREAD_PRIORITY_URGENT, G_THREAD_PRIORITY_HIGH,
    G_THREAD_PRIORITY_NORMAL, G_THREAD_PRIORITY_LOW,
};
pub use self::gconvert_h::{GIConv, _GIConv};
pub use self::gdataset_h::{GData, _GData};
pub use self::gfileutils_h::{
    GFileTest, G_FILE_TEST_EXISTS, G_FILE_TEST_IS_EXECUTABLE, G_FILE_TEST_IS_DIR,
    G_FILE_TEST_IS_SYMLINK, G_FILE_TEST_IS_REGULAR, g_file_test, g_build_filename,
    g_get_current_dir,
};
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
pub use self::gvariant_h::{GVariant, _GVariant, g_variant_new};
pub use self::gqueue_h::{_GQueue, GQueue};
pub use self::gtree_h::{GTree, _GTree};
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
pub use self::gobject_h::{_GObject, GObject, GInitiallyUnowned};
pub use self::gapplication_h::{_GApplication, GApplicationPrivate, _GApplicationPrivate};
pub use self::giotypes_h::GApplication;
pub use self::unistd_h::{socklen_t, access, getpid, unlink};
pub use self::gtkwidget_h::{_GtkWidget, GtkWidgetPrivate, _GtkWidgetPrivate};
pub use self::gtktypes_h::GtkWidget;
pub use self::gtkapplication_h::{
    _GtkApplication, GtkApplicationPrivate, GtkApplication, _GtkApplicationPrivate,
};
pub use self::gtkcssprovider_h::{
    _GtkCssProvider, GtkCssProviderPrivate, GtkCssProvider, _GtkCssProviderPrivate,
};
pub use self::globalconf_h::{globalconf_t, globalconf};
pub use self::log_h::{
    log_level_t, LOG_LEVEL_debug, LOG_LEVEL_verbose, LOG_LEVEL_info, LOG_LEVEL_warn,
    LOG_LEVEL_error, LOG_LEVEL_fatal, _log,
};
pub use self::ipc_h::{
    ipc_type_t, IPC_TYPE_crash, IPC_TYPE_page_created, IPC_TYPE_log, IPC_TYPE_eval_js,
    IPC_TYPE_extension_init, IPC_TYPE_scroll, IPC_TYPE_lua_ipc,
    IPC_TYPE_lua_require_module, _ipc_header_t, ipc_header_t, _ipc_lua_ipc_t,
    ipc_lua_ipc_t, ipc_scroll_subtype_t, IPC_SCROLL_TYPE_scroll,
    IPC_SCROLL_TYPE_winresize, IPC_SCROLL_TYPE_docresize, _ipc_scroll_t, ipc_scroll_t,
    _ipc_page_created_t, ipc_page_created_t, _ipc_recv_state_t, ipc_recv_state_t,
    ipc_endpoint_status_t, IPC_ENDPOINT_FREED, IPC_ENDPOINT_CONNECTED,
    IPC_ENDPOINT_DISCONNECTED, _ipc_endpoint_t, ipc_endpoint_t, ipc_endpoint_new,
    ipc_endpoint_connect_to_socket, ipc_send,
};
pub use self::WebKitWebContext_h::{
    WebKitWebContext, _WebKitWebContext, WebKitWebContextPrivate,
    _WebKitWebContextPrivate, webkit_web_context_set_web_extensions_directory,
    webkit_web_context_set_web_extensions_initialization_user_data,
};
pub use self::un_h::sockaddr_un;
pub use self::sockaddr_h::sa_family_t;
pub use self::socket_h::sockaddr;
pub use self::socket_type_h::{
    SOCK_STREAM, __socket_type, SOCK_NONBLOCK, SOCK_CLOEXEC, SOCK_PACKET, SOCK_DCCP,
    SOCK_SEQPACKET, SOCK_RDM, SOCK_RAW, SOCK_DGRAM,
};
pub use self::signal_h::signal_t;
pub use self::tokenize_h::{
    luakit_token_t, L_TK_ZOOM_TEXT_ONLY, L_TK_ZOOM_LEVEL, L_TK_YPAGE_SIZE, L_TK_YMAX,
    L_TK_Y, L_TK_XPAGE_SIZE, L_TK_XMAX, L_TK_X, L_TK_WRAP_JS, L_TK_WIN_XID, L_TK_WINDOWS,
    L_TK_WINDOW, L_TK_WIDTH, L_TK_WEB_PROCESS_ID, L_TK_WEBVIEW, L_TK_WEBSITE_DATA,
    L_TK_WEBKIT_VERSION, L_TK_WEBKIT_USER_AGENT_VERSION, L_TK_WEBKIT2, L_TK_VPANED,
    L_TK_VISIBLE_CHILD, L_TK_VISIBLE, L_TK_VIDEOS_DIR, L_TK_VERSION, L_TK_VERBOSE,
    L_TK_VBOX, L_TK_VALUE, L_TK_USER_AGENT, L_TK_URI, L_TK_URGENCY_HINT, L_TK_TYPE,
    L_TK_TOTAL_SIZE, L_TK_TOP, L_TK_TOOLTIP, L_TK_TITLE, L_TK_TEXT_CONTENT,
    L_TK_TEXTWIDTH, L_TK_TEXT, L_TK_TEMPLATES_DIR, L_TK_TAG_NAME, L_TK_SYSTEM_DATA_DIRS,
    L_TK_SYSTEM_CONFIG_DIRS, L_TK_SWITCH, L_TK_SUGGESTED_FILENAME, L_TK_SUBMIT,
    L_TK_STYLESHEETS, L_TK_STYLE, L_TK_STOP, L_TK_STATUS, L_TK_STARTED, L_TK_START,
    L_TK_STACK, L_TK_SSL_TRUSTED, L_TK_SRC, L_TK_SPINNER, L_TK_SPELL_CHECKING_LANGUAGES,
    L_TK_SPACING, L_TK_SOURCE, L_TK_SOCKET, L_TK_SHOW_TABS, L_TK_SHOW_INSPECTOR,
    L_TK_SHOW_FRAME, L_TK_SHOW_BORDER, L_TK_SHOW, L_TK_SET_TITLE, L_TK_SET_PDFJS,
    L_TK_SET_FAVICON_FOR_URI, L_TK_SET_DEFAULT_SIZE, L_TK_SET_DARK_MODE,
    L_TK_SESSION_STATE, L_TK_SERIF_FONT_FAMILY, L_TK_SEND_KEY, L_TK_SELECT_REGION,
    L_TK_SELECTION, L_TK_SELECTABLE, L_TK_SECONDARY, L_TK_SEARCH_PREVIOUS,
    L_TK_SEARCH_NEXT, L_TK_SEARCH, L_TK_SCROLL_Y, L_TK_SCROLL_X, L_TK_SCROLLED,
    L_TK_SCROLLBARS, L_TK_SCROLL, L_TK_SCREEN, L_TK_SCALE, L_TK_SAVE,
    L_TK_SANS_SERIF_FONT_FAMILY, L_TK_ROOT_WIN_XID, L_TK_RIGHT, L_TK_RESOURCE_PATH,
    L_TK_REPLACE, L_TK_REORDER, L_TK_REMOVE_EVENT_LISTENER, L_TK_REMOVE,
    L_TK_RELOAD_BYPASS_CACHE, L_TK_RELOAD, L_TK_RECT, L_TK_QUERY, L_TK_PUBLIC_SHARE_DIR,
    L_TK_PROXY_URI, L_TK_PROGRESS, L_TK_PROCESS_LIMIT, L_TK_PRIVATE,
    L_TK_PRINT_BACKGROUNDS, L_TK_PRIMARY, L_TK_PREV_SIBLING, L_TK_POSITION, L_TK_PLUGGED,
    L_TK_PICTURES_DIR, L_TK_PICTOGRAPH_FONT_FAMILY, L_TK_PATTERN, L_TK_PARENT,
    L_TK_PACK2, L_TK_PACK1, L_TK_PACK, L_TK_OWNER_DOCUMENT, L_TK_OVERLAY, L_TK_OPTIONS,
    L_TK_NOUNIQUE, L_TK_NOTEBOOK, L_TK_NEXT_SIBLING, L_TK_NAME, L_TK_MUSIC_DIR,
    L_TK_MONOSPACE_FONT_FAMILY, L_TK_MIN_SIZE, L_TK_MINIMUM_FONT_SIZE, L_TK_MIME_TYPE,
    L_TK_MEDIA_PLAYBACK_REQUIRES_GESTURE, L_TK_MEDIA_PLAYBACK_ALLOWS_INLINE,
    L_TK_MAXIMIZED, L_TK_MARGIN_TOP, L_TK_MARGIN_RIGHT, L_TK_MARGIN_LEFT,
    L_TK_MARGIN_BOTTOM, L_TK_MARGIN, L_TK_LOAD_STRING, L_TK_LOADING, L_TK_LEFT,
    L_TK_LAST_CHILD, L_TK_LABEL, L_TK_JAVASCRIPT_CAN_OPEN_WINDOWS_AUTOMATICALLY,
    L_TK_JAVASCRIPT_CAN_ACCESS_CLIPBOARD, L_TK_IS_PLAYING_AUDIO, L_TK_IS_LOADING,
    L_TK_IS_ALIVE, L_TK_INVALIDATE, L_TK_INTERVAL, L_TK_INSTALL_PATHS, L_TK_INSTALL_PATH,
    L_TK_INSPECTOR, L_TK_INSERT, L_TK_INNER_WIDTH, L_TK_INNER_HTML, L_TK_INNER_HEIGHT,
    L_TK_INDEXOF, L_TK_IMAGE, L_TK_ID, L_TK_ICON, L_TK_HREF, L_TK_HPANED,
    L_TK_HOVERED_URI, L_TK_HOMOGENEOUS, L_TK_HISTORY, L_TK_HIDE, L_TK_HEIGHT, L_TK_HBOX,
    L_TK_HARDWARE_ACCELERATION_POLICY, L_TK_GO_FORWARD, L_TK_GO_BACK, L_TK_GET_TITLE,
    L_TK_GET_SOURCE, L_TK_FULLSCREEN, L_TK_FONT, L_TK_FOCUSED, L_TK_FOCUS,
    L_TK_FIRST_CHILD, L_TK_FINISHED, L_TK_FILL, L_TK_FILENAME, L_TK_FG, L_TK_FETCH,
    L_TK_FANTASY_FONT_FAMILY, L_TK_EXECPATH, L_TK_EVENTBOX, L_TK_EVAL_JS, L_TK_ERROR,
    L_TK_ENTRY, L_TK_END, L_TK_ENABLE_XSS_AUDITOR,
    L_TK_ENABLE_WRITE_CONSOLE_MESSAGES_TO_STDOUT, L_TK_ENABLE_WEBGL,
    L_TK_ENABLE_WEBAUDIO, L_TK_ENABLE_TABS_TO_LINKS, L_TK_ENABLE_SPELL_CHECKING,
    L_TK_ENABLE_SPATIAL_NAVIGATION, L_TK_ENABLE_SMOOTH_SCROLLING,
    L_TK_ENABLE_SITE_SPECIFIC_QUIRKS, L_TK_ENABLE_SCRIPTS,
    L_TK_ENABLE_RESIZABLE_TEXT_AREAS, L_TK_ENABLE_PLUGINS, L_TK_ENABLE_PAGE_CACHE,
    L_TK_ENABLE_MEDIA_STREAM, L_TK_ENABLE_MEDIASOURCE, L_TK_ENABLE_JAVASCRIPT,
    L_TK_ENABLE_JAVA, L_TK_ENABLE_HYPERLINK_AUDITING, L_TK_ENABLE_HTML5_LOCAL_STORAGE,
    L_TK_ENABLE_HTML5_DATABASE, L_TK_ENABLE_FULLSCREEN, L_TK_ENABLE_FRAME_FLATTENING,
    L_TK_ENABLE_DNS_PREFETCHING, L_TK_ENABLE_DEVELOPER_EXTRAS,
    L_TK_ENABLE_CARET_BROWSING, L_TK_ENABLE_ACCELERATED_2D_CANVAS,
    L_TK_ELEMENT_FROM_POINT, L_TK_ELAPSED_TIME, L_TK_EDITABLE,
    L_TK_DRAW_COMPOSITING_INDICATORS, L_TK_DRAWING_AREA, L_TK_DOWNLOAD_DIR,
    L_TK_DOCUMENTS_DIR, L_TK_DOCUMENT, L_TK_DEV_PATHS, L_TK_DESTROY, L_TK_DESTINATION,
    L_TK_DESKTOP_DIR, L_TK_DEFAULT_MONOSPACE_FONT_SIZE, L_TK_DEFAULT_FONT_SIZE,
    L_TK_DEFAULT_FONT_FAMILY, L_TK_DEFAULT_CHARSET, L_TK_DECORATED, L_TK_DATA_DIR,
    L_TK_CURSIVE_FONT_FAMILY, L_TK_CURRENT_SIZE, L_TK_CURRENT, L_TK_CSS,
    L_TK_CREATE_ELEMENT, L_TK_CRASH, L_TK_COUNT, L_TK_COOKIES_STORAGE, L_TK_CONFPATH,
    L_TK_CONFIG_DIR, L_TK_CLOSE_INSPECTOR, L_TK_CLIPBOARD, L_TK_CLIENT_RECTS, L_TK_CLICK,
    L_TK_CLEAR_SEARCH, L_TK_CLEAR, L_TK_CHILD_COUNT, L_TK_CHILDREN, L_TK_CHILD,
    L_TK_CHECKED, L_TK_CERTIFICATE, L_TK_CENTER, L_TK_CAN_GO_FORWARD, L_TK_CAN_GO_BACK,
    L_TK_CAN_FOCUS, L_TK_CACHE_DIR, L_TK_BOTTOM, L_TK_BODY, L_TK_BG, L_TK_BASELINE,
    L_TK_AUTO_LOAD_IMAGES, L_TK_ATTR, L_TK_APPEND,
    L_TK_ALLOW_UNIVERSAL_ACCESS_FROM_FILE_URLS, L_TK_ALLOW_OVERWRITE,
    L_TK_ALLOW_MODAL_DIALOGS, L_TK_ALLOW_FILE_ACCESS_FROM_FILE_URLS,
    L_TK_ALLOW_CERTIFICATE, L_TK_ALIGN, L_TK_ADD_EVENT_LISTENER, L_TK_ACCEPT_POLICY,
    L_TK_UNKNOWN,
};
pub use self::widget_h::{
    widget_t, widget_destructor_t, widget_info_t, widget_constructor_t,
};
use self::string_h::{memset, strcpy, strlen, strerror};
use self::gutils_h::g_get_tmp_dir;
use self::stdlib_h::atexit;
use self::gmem_h::g_free;
use self::gstrfuncs_h::g_strdup_printf;
use self::grand_h::g_random_int_range;
use self::errno_h::__errno_location;
use self::sys_socket_h::{socket, bind, listen, accept};
use self::gstdio_h::g_unlink;
use self::web_module_h::web_module_load_modules_on_endpoint;
use self::clib_ipc_h::ipc_channel_recv;
use self::web_context_h::web_context_get;
use self::webview_h::{
    webview_get_by_id, webview_connect_to_endpoint, webview_set_web_process_id,
};
extern "C" {
    #[c2rust::src_loc = "40:1"]
    pub fn webview_scroll_recv(d: *mut std::ffi::c_void, ipc: *const ipc_scroll_t);
    #[c2rust::src_loc = "41:1"]
    pub fn run_javascript_finished(msg: *const guint8, length: guint);
}
#[c2rust::src_loc = "43:14"]
static mut socket_path: *mut std::ffi::c_char = 0 as *const std::ffi::c_char
    as *mut std::ffi::c_char;
#[no_mangle]
#[c2rust::src_loc = "44:8"]
pub static mut socket_path_lock: GMutex = _GMutex {
    p: 0 as *const std::ffi::c_void as *mut std::ffi::c_void,
};
#[no_mangle]
#[c2rust::src_loc = "45:7"]
pub static mut socket_path_cond: GCond = _GCond {
    p: 0 as *const std::ffi::c_void as *mut std::ffi::c_void,
    i: [0; 2],
};
#[no_mangle]
#[c2rust::src_loc = "47:1"]
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
#[no_mangle]
#[c2rust::src_loc = "48:1"]
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
#[no_mangle]
#[c2rust::src_loc = "49:1"]
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
#[no_mangle]
#[c2rust::src_loc = "51:1"]
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
#[no_mangle]
#[c2rust::src_loc = "61:1"]
pub unsafe extern "C" fn ipc_recv_lua_ipc(
    mut UNUSED_ipc: *mut ipc_endpoint_t,
    mut msg: *const ipc_lua_ipc_t,
    mut length: guint,
) {
    ipc_channel_recv(common.L, ((*msg).arg).as_ptr(), length);
}
#[no_mangle]
#[c2rust::src_loc = "67:1"]
pub unsafe extern "C" fn ipc_recv_scroll(
    mut UNUSED_ipc: *mut ipc_endpoint_t,
    mut msg: *mut ipc_scroll_t,
    mut UNUSED_length: guint,
) {
    g_ptr_array_foreach(
        globalconf.webviews,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut std::ffi::c_void, *const ipc_scroll_t) -> (),
            >,
            GFunc,
        >(
            Some(
                webview_scroll_recv
                    as unsafe extern "C" fn(
                        *mut std::ffi::c_void,
                        *const ipc_scroll_t,
                    ) -> (),
            ),
        ),
        msg as gpointer,
    );
}
#[no_mangle]
#[c2rust::src_loc = "73:1"]
pub unsafe extern "C" fn ipc_recv_eval_js(
    mut UNUSED_ipc: *mut ipc_endpoint_t,
    mut msg: *const guint8,
    mut length: guint,
) {
    run_javascript_finished(msg, length);
}
#[no_mangle]
#[c2rust::src_loc = "79:1"]
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
#[c2rust::src_loc = "91:1"]
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
        socket_path_0 = g_build_filename(
            g_get_tmp_dir(),
            socket_name,
            0 as *mut std::ffi::c_void,
        );
        g_free(socket_name as gpointer);
        if !(g_file_test(socket_path_0, G_FILE_TEST_EXISTS) != 0) {
            break;
        }
        g_free(socket_path_0 as gpointer);
    }
    return socket_path_0;
}
#[c2rust::src_loc = "113:1"]
unsafe extern "C" fn web_extension_connect_thread(
    mut UNUSED_data: gpointer,
) -> gpointer {
    let mut path: *mut gchar = build_socket_path();
    let mut sock: std::ffi::c_int = 0;
    sock = socket(
        1 as std::ffi::c_int,
        SOCK_STREAM as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
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
        ::core::mem::size_of::<sockaddr_un>() as std::ffi::c_ulong,
    );
    local.sun_family = 1 as std::ffi::c_int as sa_family_t;
    strcpy((local.sun_path).as_mut_ptr(), path);
    let mut len: std::ffi::c_int = (2 as std::ffi::c_ulong)
        .wrapping_add(strlen((local.sun_path).as_mut_ptr())) as std::ffi::c_int;
    unlink((local.sun_path).as_mut_ptr());
    if bind(sock, &mut local as *mut sockaddr_un as *mut sockaddr, len as socklen_t)
        == -(1 as std::ffi::c_int)
    {
        _log(
            LOG_LEVEL_fatal,
            b"ipc.c\0" as *const u8 as *const std::ffi::c_char,
            b"Error calling bind() on socket %s: %s\0" as *const u8
                as *const std::ffi::c_char,
            path,
            strerror(*__errno_location()),
        );
    }
    if listen(sock, 5 as std::ffi::c_int) == -(1 as std::ffi::c_int) {
        _log(
            LOG_LEVEL_fatal,
            b"ipc.c\0" as *const u8 as *const std::ffi::c_char,
            b"Error calling listen() on socket %s: %s\0" as *const u8
                as *const std::ffi::c_char,
            path,
            strerror(*__errno_location()),
        );
    }
    g_mutex_lock(&mut socket_path_lock);
    socket_path = path;
    g_cond_signal(&mut socket_path_cond);
    g_mutex_unlock(&mut socket_path_lock);
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
        let mut size: socklen_t = ::core::mem::size_of::<sockaddr_un>()
            as std::ffi::c_ulong as socklen_t;
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
        let mut ipc: *mut ipc_endpoint_t = ipc_endpoint_new(
            b"UI\0" as *const u8 as *const std::ffi::c_char,
        );
        ipc_endpoint_connect_to_socket(ipc, web_socket);
    }
    return 0 as *mut std::ffi::c_void;
}
#[c2rust::src_loc = "158:1"]
unsafe extern "C" fn initialize_web_extensions_cb(
    mut context: *mut WebKitWebContext,
    mut UNUSED_data: gpointer,
) {
    let mut dirs: [*mut std::ffi::c_char; 2] = [
        g_get_current_dir(),
        b"/usr/local/lib/luakit\0" as *const u8 as *const std::ffi::c_char
            as *mut std::ffi::c_char,
    ];
    let mut dir: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut i: std::ffi::c_uint = 0 as std::ffi::c_int as std::ffi::c_uint;
    while dir.is_null()
        && (i as std::ffi::c_ulong)
            < (::core::mem::size_of::<[*mut std::ffi::c_char; 2]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<*mut std::ffi::c_char>() as std::ffi::c_ulong,
                )
    {
        let mut extension_file: *mut std::ffi::c_char = g_build_filename(
            dirs[i as usize],
            b"luakit.so\0" as *const u8 as *const std::ffi::c_char,
            0 as *mut std::ffi::c_void,
        );
        _log(
            LOG_LEVEL_verbose,
            b"ipc.c\0" as *const u8 as *const std::ffi::c_char,
            b"checking for luakit extension at '%s'\0" as *const u8
                as *const std::ffi::c_char,
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
            b"cannot find luakit extension 'luakit.so'\0" as *const u8
                as *const std::ffi::c_char,
        );
    }
    let mut path: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    g_mutex_lock(&mut socket_path_lock);
    while socket_path.is_null() {
        g_cond_wait(&mut socket_path_cond, &mut socket_path_lock);
    }
    path = socket_path;
    g_mutex_unlock(&mut socket_path_lock);
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
    let mut package_path: *const std::ffi::c_char = lua_tolstring(
        common.L,
        -(1 as std::ffi::c_int),
        0 as *mut size_t,
    );
    lua_getfield(
        common.L,
        -(2 as std::ffi::c_int),
        b"cpath\0" as *const u8 as *const std::ffi::c_char,
    );
    let mut package_cpath: *const std::ffi::c_char = lua_tolstring(
        common.L,
        -(1 as std::ffi::c_int),
        0 as *mut size_t,
    );
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
#[no_mangle]
#[c2rust::src_loc = "197:1"]
pub unsafe extern "C" fn ipc_remove_socket_file() {
    g_mutex_lock(&mut socket_path_lock);
    g_unlink(socket_path);
    g_free(socket_path as gpointer);
    socket_path = 0 as *mut std::ffi::c_char;
    g_mutex_unlock(&mut socket_path_lock);
}
#[no_mangle]
#[c2rust::src_loc = "207:1"]
pub unsafe extern "C" fn ipc_init() {
    g_thread_new(
        b"accept_thread\0" as *const u8 as *const std::ffi::c_char,
        Some(web_extension_connect_thread as unsafe extern "C" fn(gpointer) -> gpointer),
        0 as *mut std::ffi::c_void,
    );
    g_signal_connect_data(
        web_context_get() as gpointer,
        b"initialize-web-extensions\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut WebKitWebContext, gpointer) -> ()>,
            GCallback,
        >(
            Some(
                initialize_web_extensions_cb
                    as unsafe extern "C" fn(*mut WebKitWebContext, gpointer) -> (),
            ),
        ),
        0 as *mut std::ffi::c_void,
        None,
        G_CONNECT_DEFAULT,
    );
    atexit(Some(ipc_remove_socket_file as unsafe extern "C" fn() -> ()));
}
