use ::libc;
use ::c2rust_bitfields;
#[c2rust::header_src = "/usr/lib/clang/20/include/__stddef_ptrdiff_t.h:21"]
pub mod __stddef_ptrdiff_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type ptrdiff_t = std::ffi::c_long;
}
#[c2rust::header_src = "/usr/lib/clang/20/include/__stddef_size_t.h:21"]
pub mod __stddef_size_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type size_t = std::ffi::c_ulong;
}
#[c2rust::header_src = "/usr/lib/glib-2.0/include/glibconfig.h:21"]
pub mod glibconfig_h {
    #[c2rust::src_loc = "57:1"]
    pub type guint32 = std::ffi::c_uint;
    #[c2rust::src_loc = "66:1"]
    pub type gint64 = std::ffi::c_long;
    #[c2rust::src_loc = "67:1"]
    pub type guint64 = std::ffi::c_ulong;
    #[c2rust::src_loc = "82:1"]
    pub type gssize = std::ffi::c_long;
    #[c2rust::src_loc = "83:1"]
    pub type gsize = std::ffi::c_ulong;
    #[c2rust::src_loc = "201:1"]
    pub type GPid = std::ffi::c_int;
}
#[c2rust::header_src = "/usr/include/bits/types.h:21"]
pub mod types_h {
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = std::ffi::c_uint;
    #[c2rust::src_loc = "146:1"]
    pub type __uid_t = std::ffi::c_uint;
    #[c2rust::src_loc = "154:1"]
    pub type __pid_t = std::ffi::c_int;
    #[c2rust::src_loc = "156:1"]
    pub type __clock_t = std::ffi::c_long;
}
#[c2rust::header_src = "/usr/include/bits/types/__sigval_t.h:21"]
pub mod __sigval_t_h {
    #[c2rust::src_loc = "30:1"]
    pub type __sigval_t = sigval;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "24:7"]
    pub union sigval {
        pub sival_int: std::ffi::c_int,
        pub sival_ptr: *mut std::ffi::c_void,
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtypes.h:21"]
pub mod gtypes_h {
    #[c2rust::src_loc = "52:1"]
    pub type gchar = std::ffi::c_char;
    #[c2rust::src_loc = "55:1"]
    pub type gint = std::ffi::c_int;
    #[c2rust::src_loc = "56:1"]
    pub type gboolean = gint;
    #[c2rust::src_loc = "60:1"]
    pub type gulong = std::ffi::c_ulong;
    #[c2rust::src_loc = "61:1"]
    pub type guint = std::ffi::c_uint;
    #[c2rust::src_loc = "64:1"]
    pub type gdouble = std::ffi::c_double;
    #[c2rust::src_loc = "109:1"]
    pub type gpointer = *mut std::ffi::c_void;
    #[c2rust::src_loc = "110:1"]
    pub type gconstpointer = *const std::ffi::c_void;
    #[c2rust::src_loc = "114:1"]
    pub type GCompareDataFunc = Option::<
        unsafe extern "C" fn(gconstpointer, gconstpointer, gpointer) -> gint,
    >;
    #[c2rust::src_loc = "140:1"]
    pub type GDestroyNotify = Option::<unsafe extern "C" fn(gpointer) -> ()>;
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
    use super::gtypes_h::{gpointer, guint, gboolean};
    extern "C" {
        #[c2rust::src_loc = "188:1"]
        pub fn g_ptr_array_free(
            array: *mut GPtrArray,
            free_segment: gboolean,
        ) -> *mut gpointer;
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
        #[c2rust::src_loc = "240:1"]
        pub fn g_clear_error(err: *mut *mut GError);
    }
}
#[c2rust::header_src = "/usr/include/bits/types/__sigset_t.h:21"]
pub mod __sigset_t_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "5:9"]
    pub struct __sigset_t {
        pub __val: [std::ffi::c_ulong; 16],
    }
}
#[c2rust::header_src = "/usr/include/bits/types/sigset_t.h:21"]
pub mod sigset_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type sigset_t = __sigset_t;
    use super::__sigset_t_h::__sigset_t;
}
#[c2rust::header_src = "/usr/include/bits/types/siginfo_t.h:21"]
pub mod siginfo_t_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "36:9"]
    pub struct siginfo_t {
        pub si_signo: std::ffi::c_int,
        pub si_errno: std::ffi::c_int,
        pub si_code: std::ffi::c_int,
        pub __pad0: std::ffi::c_int,
        pub _sifields: C2RustUnnamed,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "51:5"]
    pub union C2RustUnnamed {
        pub _pad: [std::ffi::c_int; 28],
        pub _kill: C2RustUnnamed_8,
        pub _timer: C2RustUnnamed_7,
        pub _rt: C2RustUnnamed_6,
        pub _sigchld: C2RustUnnamed_5,
        pub _sigfault: C2RustUnnamed_2,
        pub _sigpoll: C2RustUnnamed_1,
        pub _sigsys: C2RustUnnamed_0,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "116:2"]
    pub struct C2RustUnnamed_0 {
        pub _call_addr: *mut std::ffi::c_void,
        pub _syscall: std::ffi::c_int,
        pub _arch: std::ffi::c_uint,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "108:2"]
    pub struct C2RustUnnamed_1 {
        pub si_band: std::ffi::c_long,
        pub si_fd: std::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "89:2"]
    pub struct C2RustUnnamed_2 {
        pub si_addr: *mut std::ffi::c_void,
        pub si_addr_lsb: std::ffi::c_short,
        pub _bounds: C2RustUnnamed_3,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "94:6"]
    pub union C2RustUnnamed_3 {
        pub _addr_bnd: C2RustUnnamed_4,
        pub _pkey: __uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "97:3"]
    pub struct C2RustUnnamed_4 {
        pub _lower: *mut std::ffi::c_void,
        pub _upper: *mut std::ffi::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "79:2"]
    pub struct C2RustUnnamed_5 {
        pub si_pid: __pid_t,
        pub si_uid: __uid_t,
        pub si_status: std::ffi::c_int,
        pub si_utime: __clock_t,
        pub si_stime: __clock_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "71:2"]
    pub struct C2RustUnnamed_6 {
        pub si_pid: __pid_t,
        pub si_uid: __uid_t,
        pub si_sigval: __sigval_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "63:2"]
    pub struct C2RustUnnamed_7 {
        pub si_tid: std::ffi::c_int,
        pub si_overrun: std::ffi::c_int,
        pub si_sigval: __sigval_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "56:2"]
    pub struct C2RustUnnamed_8 {
        pub si_pid: __pid_t,
        pub si_uid: __uid_t,
    }
    use super::types_h::{__uint32_t, __pid_t, __uid_t, __clock_t};
    use super::__sigval_t_h::__sigval_t;
}
#[c2rust::header_src = "/usr/include/signal.h:21"]
pub mod signal_h {
    #[c2rust::src_loc = "72:1"]
    pub type __sighandler_t = Option::<unsafe extern "C" fn(std::ffi::c_int) -> ()>;
    use super::sigset_t_h::sigset_t;
    use super::sigaction_h::sigaction;
    extern "C" {
        #[c2rust::src_loc = "199:1"]
        pub fn sigemptyset(__set: *mut sigset_t) -> std::ffi::c_int;
        #[c2rust::src_loc = "243:1"]
        pub fn sigaction(
            __sig: std::ffi::c_int,
            __act: *const sigaction,
            __oact: *mut sigaction,
        ) -> std::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/bits/sigaction.h:21"]
pub mod sigaction_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "27:8"]
    pub struct sigaction {
        pub __sigaction_handler: C2RustUnnamed_9,
        pub sa_mask: __sigset_t,
        pub sa_flags: std::ffi::c_int,
        pub sa_restorer: Option::<unsafe extern "C" fn() -> ()>,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "31:5"]
    pub union C2RustUnnamed_9 {
        pub sa_handler: __sighandler_t,
        pub sa_sigaction: Option::<
            unsafe extern "C" fn(
                std::ffi::c_int,
                *mut siginfo_t,
                *mut std::ffi::c_void,
            ) -> (),
        >,
    }
    use super::__sigset_t_h::__sigset_t;
    use super::signal_h::__sighandler_t;
    use super::siginfo_t_h::siginfo_t;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gdatetime.h:21"]
pub mod gdatetime_h {
    #[c2rust::src_loc = "89:1"]
    pub type GTimeSpan = gint64;
    use super::glibconfig_h::gint64;
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
    extern "C" {
        #[c2rust::src_loc = "52:1"]
        pub fn g_list_free(list: *mut GList);
        #[c2rust::src_loc = "99:1"]
        pub fn g_list_delete_link(list: *mut GList, link_: *mut GList) -> *mut GList;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/ghash.h:21"]
pub mod ghash_h {
    #[c2rust::src_loc = "40:1"]
    pub type GHashTable = _GHashTable;
    extern "C" {
        #[c2rust::src_loc = "40:16"]
        pub type _GHashTable;
    }
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
    #[c2rust::src_loc = "249:1"]
    pub type GChildWatchFunc = Option::<
        unsafe extern "C" fn(GPid, gint, gpointer) -> (),
    >;
    use super::gtypes_h::{gpointer, guint, gint, gboolean};
    use super::gslist_h::GSList;
    use super::glibconfig_h::GPid;
    extern "C" {
        #[c2rust::src_loc = "70:16"]
        pub type _GMainContext;
        #[c2rust::src_loc = "87:16"]
        pub type _GSourcePrivate;
        #[c2rust::src_loc = "958:1"]
        pub fn g_child_watch_add(
            pid: GPid,
            function: GChildWatchFunc,
            data: gpointer,
        ) -> guint;
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
    extern "C" {
        #[c2rust::src_loc = "165:1"]
        pub fn g_io_channel_unref(channel: *mut GIOChannel);
        #[c2rust::src_loc = "263:1"]
        pub fn g_io_channel_read_to_end(
            channel: *mut GIOChannel,
            str_return: *mut *mut gchar,
            length: *mut gsize,
            error: *mut *mut GError,
        ) -> GIOStatus;
        #[c2rust::src_loc = "323:1"]
        pub fn g_io_channel_unix_new(fd: std::ffi::c_int) -> *mut GIOChannel;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gregex.h:21"]
pub mod gregex_h {
    #[c2rust::src_loc = "304:9"]
    pub type GRegexCompileFlags = std::ffi::c_uint;
    #[c2rust::src_loc = "324:3"]
    pub const G_REGEX_JAVASCRIPT_COMPAT: GRegexCompileFlags = 33554432;
    #[c2rust::src_loc = "323:3"]
    pub const G_REGEX_BSR_ANYCRLF: GRegexCompileFlags = 8388608;
    #[c2rust::src_loc = "322:3"]
    pub const G_REGEX_NEWLINE_ANYCRLF: GRegexCompileFlags = 5242880;
    #[c2rust::src_loc = "321:3"]
    pub const G_REGEX_NEWLINE_CRLF: GRegexCompileFlags = 3145728;
    #[c2rust::src_loc = "320:3"]
    pub const G_REGEX_NEWLINE_LF: GRegexCompileFlags = 2097152;
    #[c2rust::src_loc = "319:3"]
    pub const G_REGEX_NEWLINE_CR: GRegexCompileFlags = 1048576;
    #[c2rust::src_loc = "318:3"]
    pub const G_REGEX_DUPNAMES: GRegexCompileFlags = 524288;
    #[c2rust::src_loc = "317:3"]
    pub const G_REGEX_FIRSTLINE: GRegexCompileFlags = 262144;
    #[c2rust::src_loc = "316:3"]
    pub const G_REGEX_OPTIMIZE: GRegexCompileFlags = 8192;
    #[c2rust::src_loc = "315:3"]
    pub const G_REGEX_NO_AUTO_CAPTURE: GRegexCompileFlags = 4096;
    #[c2rust::src_loc = "314:3"]
    pub const G_REGEX_RAW: GRegexCompileFlags = 2048;
    #[c2rust::src_loc = "313:3"]
    pub const G_REGEX_UNGREEDY: GRegexCompileFlags = 512;
    #[c2rust::src_loc = "312:3"]
    pub const G_REGEX_DOLLAR_ENDONLY: GRegexCompileFlags = 32;
    #[c2rust::src_loc = "311:3"]
    pub const G_REGEX_ANCHORED: GRegexCompileFlags = 16;
    #[c2rust::src_loc = "310:3"]
    pub const G_REGEX_EXTENDED: GRegexCompileFlags = 8;
    #[c2rust::src_loc = "309:3"]
    pub const G_REGEX_DOTALL: GRegexCompileFlags = 4;
    #[c2rust::src_loc = "308:3"]
    pub const G_REGEX_MULTILINE: GRegexCompileFlags = 2;
    #[c2rust::src_loc = "307:3"]
    pub const G_REGEX_CASELESS: GRegexCompileFlags = 1;
    #[c2rust::src_loc = "306:3"]
    pub const G_REGEX_DEFAULT: GRegexCompileFlags = 0;
    #[c2rust::src_loc = "396:9"]
    pub type GRegexMatchFlags = std::ffi::c_uint;
    #[c2rust::src_loc = "413:3"]
    pub const G_REGEX_MATCH_NOTEMPTY_ATSTART: GRegexMatchFlags = 268435456;
    #[c2rust::src_loc = "412:3"]
    pub const G_REGEX_MATCH_PARTIAL_HARD: GRegexMatchFlags = 134217728;
    #[c2rust::src_loc = "411:3"]
    pub const G_REGEX_MATCH_PARTIAL_SOFT: GRegexMatchFlags = 32768;
    #[c2rust::src_loc = "410:3"]
    pub const G_REGEX_MATCH_BSR_ANY: GRegexMatchFlags = 16777216;
    #[c2rust::src_loc = "409:3"]
    pub const G_REGEX_MATCH_BSR_ANYCRLF: GRegexMatchFlags = 8388608;
    #[c2rust::src_loc = "408:3"]
    pub const G_REGEX_MATCH_NEWLINE_ANYCRLF: GRegexMatchFlags = 5242880;
    #[c2rust::src_loc = "407:3"]
    pub const G_REGEX_MATCH_NEWLINE_ANY: GRegexMatchFlags = 4194304;
    #[c2rust::src_loc = "406:3"]
    pub const G_REGEX_MATCH_NEWLINE_CRLF: GRegexMatchFlags = 3145728;
    #[c2rust::src_loc = "405:3"]
    pub const G_REGEX_MATCH_NEWLINE_LF: GRegexMatchFlags = 2097152;
    #[c2rust::src_loc = "404:3"]
    pub const G_REGEX_MATCH_NEWLINE_CR: GRegexMatchFlags = 1048576;
    #[c2rust::src_loc = "403:3"]
    pub const G_REGEX_MATCH_PARTIAL: GRegexMatchFlags = 32768;
    #[c2rust::src_loc = "402:3"]
    pub const G_REGEX_MATCH_NOTEMPTY: GRegexMatchFlags = 1024;
    #[c2rust::src_loc = "401:3"]
    pub const G_REGEX_MATCH_NOTEOL: GRegexMatchFlags = 256;
    #[c2rust::src_loc = "400:3"]
    pub const G_REGEX_MATCH_NOTBOL: GRegexMatchFlags = 128;
    #[c2rust::src_loc = "399:3"]
    pub const G_REGEX_MATCH_ANCHORED: GRegexMatchFlags = 16;
    #[c2rust::src_loc = "398:3"]
    pub const G_REGEX_MATCH_DEFAULT: GRegexMatchFlags = 0;
    use super::gtypes_h::{gchar, gboolean};
    extern "C" {
        #[c2rust::src_loc = "484:1"]
        pub fn g_regex_match_simple(
            pattern: *const gchar,
            string: *const gchar,
            compile_options: GRegexCompileFlags,
            match_options: GRegexMatchFlags,
        ) -> gboolean;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gspawn.h:21"]
pub mod gspawn_h {
    #[c2rust::src_loc = "140:1"]
    pub type GSpawnChildSetupFunc = Option::<unsafe extern "C" fn(gpointer) -> ()>;
    #[c2rust::src_loc = "176:9"]
    pub type GSpawnFlags = std::ffi::c_uint;
    #[c2rust::src_loc = "216:3"]
    pub const G_SPAWN_STDIN_FROM_DEV_NULL: GSpawnFlags = 2048;
    #[c2rust::src_loc = "207:3"]
    pub const G_SPAWN_CHILD_INHERITS_STDERR: GSpawnFlags = 1024;
    #[c2rust::src_loc = "198:3"]
    pub const G_SPAWN_CHILD_INHERITS_STDOUT: GSpawnFlags = 512;
    #[c2rust::src_loc = "189:3"]
    pub const G_SPAWN_CLOEXEC_PIPES: GSpawnFlags = 256;
    #[c2rust::src_loc = "188:3"]
    pub const G_SPAWN_SEARCH_PATH_FROM_ENVP: GSpawnFlags = 128;
    #[c2rust::src_loc = "187:3"]
    pub const G_SPAWN_FILE_AND_ARGV_ZERO: GSpawnFlags = 64;
    #[c2rust::src_loc = "186:3"]
    pub const G_SPAWN_CHILD_INHERITS_STDIN: GSpawnFlags = 32;
    #[c2rust::src_loc = "185:3"]
    pub const G_SPAWN_STDERR_TO_DEV_NULL: GSpawnFlags = 16;
    #[c2rust::src_loc = "184:3"]
    pub const G_SPAWN_STDOUT_TO_DEV_NULL: GSpawnFlags = 8;
    #[c2rust::src_loc = "182:3"]
    pub const G_SPAWN_SEARCH_PATH: GSpawnFlags = 4;
    #[c2rust::src_loc = "180:3"]
    pub const G_SPAWN_DO_NOT_REAP_CHILD: GSpawnFlags = 2;
    #[c2rust::src_loc = "179:3"]
    pub const G_SPAWN_LEAVE_DESCRIPTORS_OPEN: GSpawnFlags = 1;
    #[c2rust::src_loc = "178:3"]
    pub const G_SPAWN_DEFAULT: GSpawnFlags = 0;
    use super::gtypes_h::{gpointer, gchar, gint, gboolean};
    use super::glibconfig_h::GPid;
    use super::gerror_h::GError;
    extern "C" {
        #[c2rust::src_loc = "238:1"]
        pub fn g_spawn_async_with_pipes(
            working_directory: *const gchar,
            argv: *mut *mut gchar,
            envp: *mut *mut gchar,
            flags: GSpawnFlags,
            child_setup: GSpawnChildSetupFunc,
            user_data: gpointer,
            child_pid: *mut GPid,
            standard_input: *mut gint,
            standard_output: *mut gint,
            standard_error: *mut gint,
            error: *mut *mut GError,
        ) -> gboolean;
        #[c2rust::src_loc = "300:1"]
        pub fn g_spawn_command_line_sync(
            command_line: *const gchar,
            standard_output: *mut *mut gchar,
            standard_error: *mut *mut gchar,
            wait_status: *mut gint,
            error: *mut *mut GError,
        ) -> gboolean;
        #[c2rust::src_loc = "318:1"]
        pub fn g_spawn_close_pid(pid: GPid);
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtree.h:21"]
pub mod gtree_h {
    #[c2rust::src_loc = "40:1"]
    pub type GTree = _GTree;
    use super::gtypes_h::{GCompareDataFunc, gpointer, GDestroyNotify};
    extern "C" {
        #[c2rust::src_loc = "40:16"]
        pub type _GTree;
        #[c2rust::src_loc = "78:1"]
        pub fn g_tree_new_full(
            key_compare_func: GCompareDataFunc,
            key_compare_data: gpointer,
            key_destroy_func: GDestroyNotify,
            value_destroy_func: GDestroyNotify,
        ) -> *mut GTree;
    }
}
#[c2rust::header_src = "/usr/include/luajit-2.1/lua.h:21"]
pub mod lua_h {
    #[c2rust::src_loc = "53:1"]
    pub type lua_CFunction = Option::<
        unsafe extern "C" fn(*mut lua_State) -> std::ffi::c_int,
    >;
    #[c2rust::src_loc = "100:1"]
    pub type lua_Number = std::ffi::c_double;
    #[c2rust::src_loc = "104:1"]
    pub type lua_Integer = ptrdiff_t;
    use super::__stddef_ptrdiff_t_h::ptrdiff_t;
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "51:16"]
        pub type lua_State;
        #[c2rust::src_loc = "121:1"]
        pub fn lua_gettop(L: *mut lua_State) -> std::ffi::c_int;
        #[c2rust::src_loc = "122:1"]
        pub fn lua_settop(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "123:1"]
        pub fn lua_pushvalue(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "124:1"]
        pub fn lua_remove(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "125:1"]
        pub fn lua_insert(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "137:1"]
        pub fn lua_isstring(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;
        #[c2rust::src_loc = "140:1"]
        pub fn lua_type(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;
        #[c2rust::src_loc = "141:1"]
        pub fn lua_typename(
            L: *mut lua_State,
            tp: std::ffi::c_int,
        ) -> *const std::ffi::c_char;
        #[c2rust::src_loc = "148:1"]
        pub fn lua_tointeger(L: *mut lua_State, idx: std::ffi::c_int) -> lua_Integer;
        #[c2rust::src_loc = "149:1"]
        pub fn lua_toboolean(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;
        #[c2rust::src_loc = "150:1"]
        pub fn lua_tolstring(
            L: *mut lua_State,
            idx: std::ffi::c_int,
            len: *mut size_t,
        ) -> *const std::ffi::c_char;
        #[c2rust::src_loc = "151:1"]
        pub fn lua_objlen(L: *mut lua_State, idx: std::ffi::c_int) -> size_t;
        #[c2rust::src_loc = "161:1"]
        pub fn lua_pushnil(L: *mut lua_State);
        #[c2rust::src_loc = "162:1"]
        pub fn lua_pushnumber(L: *mut lua_State, n: lua_Number);
        #[c2rust::src_loc = "163:1"]
        pub fn lua_pushinteger(L: *mut lua_State, n: lua_Integer);
        #[c2rust::src_loc = "164:1"]
        pub fn lua_pushlstring(L: *mut lua_State, s: *const std::ffi::c_char, l: size_t);
        #[c2rust::src_loc = "165:1"]
        pub fn lua_pushstring(L: *mut lua_State, s: *const std::ffi::c_char);
        #[c2rust::src_loc = "168:1"]
        pub fn lua_pushfstring(
            L: *mut lua_State,
            fmt: *const std::ffi::c_char,
            _: ...
        ) -> *const std::ffi::c_char;
        #[c2rust::src_loc = "169:1"]
        pub fn lua_pushcclosure(
            L: *mut lua_State,
            fn_0: lua_CFunction,
            n: std::ffi::c_int,
        );
        #[c2rust::src_loc = "170:1"]
        pub fn lua_pushboolean(L: *mut lua_State, b: std::ffi::c_int);
        #[c2rust::src_loc = "171:1"]
        pub fn lua_pushlightuserdata(L: *mut lua_State, p: *mut std::ffi::c_void);
        #[c2rust::src_loc = "180:1"]
        pub fn lua_rawget(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "181:1"]
        pub fn lua_rawgeti(L: *mut lua_State, idx: std::ffi::c_int, n: std::ffi::c_int);
        #[c2rust::src_loc = "182:1"]
        pub fn lua_createtable(
            L: *mut lua_State,
            narr: std::ffi::c_int,
            nrec: std::ffi::c_int,
        );
        #[c2rust::src_loc = "192:1"]
        pub fn lua_setfield(
            L: *mut lua_State,
            idx: std::ffi::c_int,
            k: *const std::ffi::c_char,
        );
        #[c2rust::src_loc = "193:1"]
        pub fn lua_rawset(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "194:1"]
        pub fn lua_rawseti(L: *mut lua_State, idx: std::ffi::c_int, n: std::ffi::c_int);
        #[c2rust::src_loc = "195:1"]
        pub fn lua_setmetatable(
            L: *mut lua_State,
            objindex: std::ffi::c_int,
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "203:1"]
        pub fn lua_pcall(
            L: *mut lua_State,
            nargs: std::ffi::c_int,
            nresults: std::ffi::c_int,
            errfunc: std::ffi::c_int,
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "216:1"]
        pub fn lua_status(L: *mut lua_State) -> std::ffi::c_int;
        #[c2rust::src_loc = "239:1"]
        pub fn lua_error(L: *mut lua_State) -> std::ffi::c_int;
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
#[c2rust::header_src = "/home/daana/git/luakit/common/signal.h:21"]
pub mod common_signal_h {
    #[c2rust::src_loc = "29:1"]
    pub type signal_t = GTree;
    #[inline]
    #[c2rust::src_loc = "33:1"]
    pub unsafe extern "C" fn signal_cmp(
        mut a: gconstpointer,
        mut b: gconstpointer,
        mut UNUSED_p: gpointer,
    ) -> gint {
        return g_strcmp0(a as *const std::ffi::c_char, b as *const std::ffi::c_char);
    }
    #[inline]
    #[c2rust::src_loc = "40:1"]
    pub unsafe extern "C" fn signal_array_destroy(mut sigfuncs: *mut gpointer) {
        g_ptr_array_free(
            sigfuncs as *mut GPtrArray,
            (0 as std::ffi::c_int == 0) as std::ffi::c_int,
        );
    }
    #[inline]
    #[c2rust::src_loc = "47:1"]
    pub unsafe extern "C" fn signal_new() -> *mut signal_t {
        return g_tree_new_full(
            ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(gconstpointer, gconstpointer, gpointer) -> gint,
                >,
                GCompareDataFunc,
            >(
                Some(
                    signal_cmp
                        as unsafe extern "C" fn(
                            gconstpointer,
                            gconstpointer,
                            gpointer,
                        ) -> gint,
                ),
            ),
            0 as *mut std::ffi::c_void,
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut gpointer) -> ()>,
                GDestroyNotify,
            >(Some(signal_array_destroy as unsafe extern "C" fn(*mut gpointer) -> ())),
        ) as *mut signal_t;
    }
    use super::gtree_h::{GTree, g_tree_new_full};
    use super::gtypes_h::{
        gconstpointer, gpointer, gint, GCompareDataFunc, GDestroyNotify,
    };
    use super::gtestutils_h::g_strcmp0;
    use super::garray_h::{g_ptr_array_free, GPtrArray};
    use super::gmem_h::g_free;
}
#[c2rust::header_src = "/home/daana/git/luakit/common/tokenize.h:21"]
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
    use super::gtypes_h::gchar;
    extern "C" {
        #[c2rust::src_loc = "285:1"]
        pub fn l_tokenize(_: *const gchar) -> luakit_token_t;
    }
}
#[c2rust::header_src = "/usr/include/luajit-2.1/lauxlib.h:21"]
pub mod lauxlib_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "21:16"]
    pub struct luaL_Reg {
        pub name: *const std::ffi::c_char,
        pub func: lua_CFunction,
    }
    use super::lua_h::{lua_CFunction, lua_State, lua_Integer};
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "32:1"]
        pub fn luaL_typerror(
            L: *mut lua_State,
            narg: std::ffi::c_int,
            tname: *const std::ffi::c_char,
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "33:1"]
        pub fn luaL_argerror(
            L: *mut lua_State,
            numarg: std::ffi::c_int,
            extramsg: *const std::ffi::c_char,
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "34:1"]
        pub fn luaL_checklstring(
            L: *mut lua_State,
            numArg: std::ffi::c_int,
            l: *mut size_t,
        ) -> *const std::ffi::c_char;
        #[c2rust::src_loc = "42:1"]
        pub fn luaL_optinteger(
            L: *mut lua_State,
            nArg: std::ffi::c_int,
            def: lua_Integer,
        ) -> lua_Integer;
        #[c2rust::src_loc = "53:1"]
        pub fn luaL_error(
            L: *mut lua_State,
            fmt: *const std::ffi::c_char,
            _: ...
        ) -> std::ffi::c_int;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/luaclass.h:21"]
pub mod luaclass_h {
    #[c2rust::src_loc = "32:1"]
    pub type lua_class_property_array_t = GHashTable;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "39:9"]
    pub struct lua_object_t {
        pub signals: *mut signal_t,
    }
    #[c2rust::src_loc = "43:1"]
    pub type lua_class_allocator_t = Option::<
        unsafe extern "C" fn(*mut lua_State) -> *mut lua_object_t,
    >;
    #[c2rust::src_loc = "45:1"]
    pub type lua_class_propfunc_t = Option::<
        unsafe extern "C" fn(*mut lua_State, *mut lua_object_t) -> gint,
    >;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "47:9"]
    pub struct lua_class_t {
        pub name: *const gchar,
        pub signals: *mut signal_t,
        pub allocator: lua_class_allocator_t,
        pub properties: *mut lua_class_property_array_t,
        pub index_miss_property: lua_class_propfunc_t,
        pub newindex_miss_property: lua_class_propfunc_t,
    }
    use super::ghash_h::GHashTable;
    use super::common_signal_h::signal_t;
    use super::lua_h::lua_State;
    use super::gtypes_h::{gint, gchar, gpointer};
    use super::lauxlib_h::luaL_Reg;
    extern "C" {
        #[c2rust::src_loc = "65:1"]
        pub fn luaH_class_add_signal(
            _: *mut lua_State,
            _: *mut lua_class_t,
            name: *const gchar,
            ud: gint,
        );
        #[c2rust::src_loc = "67:1"]
        pub fn luaH_class_remove_signal(
            _: *mut lua_State,
            _: *mut lua_class_t,
            name: *const gchar,
            ud: gint,
        );
        #[c2rust::src_loc = "69:1"]
        pub fn luaH_class_emit_signal(
            _: *mut lua_State,
            _: *mut lua_class_t,
            name: *const gchar,
            nargs: gint,
            nret: gint,
        ) -> gint;
        #[c2rust::src_loc = "74:1"]
        pub fn luaH_openlib(
            _: *mut lua_State,
            _: *const gchar,
            _: *const luaL_Reg,
            _: *const luaL_Reg,
        );
        #[c2rust::src_loc = "83:1"]
        pub fn luaH_usemetatable(_: *mut lua_State, _: gint, _: gint) -> gint;
        #[c2rust::src_loc = "88:1"]
        pub fn luaH_checkudata(
            _: *mut lua_State,
            _: gint,
            _: *mut lua_class_t,
        ) -> gpointer;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/clib/luakit.h:21"]
pub mod luakit_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "32:16"]
    pub struct proc_callback_data_t {
        pub cb_ref: gpointer,
        pub stdout_fd: gint,
        pub stderr_fd: gint,
    }
    use super::gtypes_h::{gpointer, gint};
}
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/webkit/WebKitFaviconDatabase.h:32"]
pub mod WebKitFaviconDatabase_h {
    #[c2rust::src_loc = "53:1"]
    pub type WebKitFaviconDatabase = _WebKitFaviconDatabase;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "53:1"]
    pub struct _WebKitFaviconDatabase {
        pub parent: GObject,
        pub priv_0: *mut WebKitFaviconDatabasePrivate,
    }
    #[c2rust::src_loc = "53:1"]
    pub type WebKitFaviconDatabasePrivate = _WebKitFaviconDatabasePrivate;
    use super::gobject_h::GObject;
    extern "C" {
        #[c2rust::src_loc = "53:1"]
        pub type _WebKitFaviconDatabasePrivate;
        #[c2rust::src_loc = "87:1"]
        pub fn webkit_favicon_database_clear(database: *mut WebKitFaviconDatabase);
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/gobject/gobject.h:23"]
pub mod gobject_h {
    #[c2rust::src_loc = "192:1"]
    pub type GObject = _GObject;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "252:9"]
    pub struct _GObject {
        pub g_type_instance: GTypeInstance,
        pub ref_count: guint,
        pub qdata: *mut GData,
    }
    #[c2rust::src_loc = "194:1"]
    pub type GInitiallyUnowned = _GObject;
    use super::gtype_h::GTypeInstance;
    use super::gtypes_h::{guint, gpointer};
    use super::gdataset_h::GData;
    extern "C" {
        #[c2rust::src_loc = "514:1"]
        pub fn g_object_unref(object: gpointer);
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/gobject/gtype.h:23"]
pub mod gtype_h {
    #[c2rust::src_loc = "436:1"]
    pub type GTypeInstance = _GTypeInstance;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "461:8"]
    pub struct _GTypeInstance {
        pub g_class: *mut GTypeClass,
    }
    #[c2rust::src_loc = "434:1"]
    pub type GTypeClass = _GTypeClass;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "451:8"]
    pub struct _GTypeClass {
        pub g_type: GType,
    }
    #[c2rust::src_loc = "427:1"]
    pub type GType = gsize;
    use super::glibconfig_h::gsize;
    use super::gtypes_h::gboolean;
    extern "C" {
        #[c2rust::src_loc = "2626:1"]
        pub fn g_type_check_instance_cast(
            instance: *mut GTypeInstance,
            iface_type: GType,
        ) -> *mut GTypeInstance;
        #[c2rust::src_loc = "2629:1"]
        pub fn g_type_check_instance_is_a(
            instance: *mut GTypeInstance,
            iface_type: GType,
        ) -> gboolean;
    }
}
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/webkit/WebKitWebContext.h:32"]
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
    #[c2rust::src_loc = "122:1"]
    pub type WebKitURISchemeRequestCallback = Option::<
        unsafe extern "C" fn(*mut WebKitURISchemeRequest, gpointer) -> (),
    >;
    use super::gobject_h::GObject;
    use super::WebKitURISchemeRequest_h::WebKitURISchemeRequest;
    use super::gtypes_h::{gpointer, gchar, GDestroyNotify, gboolean};
    use super::WebKitWebsiteDataManager_h::WebKitWebsiteDataManager;
    use super::WebKitFaviconDatabase_h::WebKitFaviconDatabase;
    use super::giotypes_h::GTlsCertificate;
    extern "C" {
        #[c2rust::src_loc = "49:1"]
        pub type _WebKitWebContextPrivate;
        #[c2rust::src_loc = "137:1"]
        pub fn webkit_web_context_get_website_data_manager(
            context: *mut WebKitWebContext,
        ) -> *mut WebKitWebsiteDataManager;
        #[c2rust::src_loc = "182:1"]
        pub fn webkit_web_context_get_favicon_database(
            context: *mut WebKitWebContext,
        ) -> *mut WebKitFaviconDatabase;
        #[c2rust::src_loc = "209:1"]
        pub fn webkit_web_context_register_uri_scheme(
            context: *mut WebKitWebContext,
            scheme: *const gchar,
            callback: WebKitURISchemeRequestCallback,
            user_data: gpointer,
            user_data_destroy_func: GDestroyNotify,
        );
        #[c2rust::src_loc = "228:1"]
        pub fn webkit_web_context_get_spell_checking_enabled(
            context: *mut WebKitWebContext,
        ) -> gboolean;
        #[c2rust::src_loc = "231:1"]
        pub fn webkit_web_context_set_spell_checking_enabled(
            context: *mut WebKitWebContext,
            enabled: gboolean,
        );
        #[c2rust::src_loc = "234:1"]
        pub fn webkit_web_context_get_spell_checking_languages(
            context: *mut WebKitWebContext,
        ) -> *const *const gchar;
        #[c2rust::src_loc = "237:1"]
        pub fn webkit_web_context_set_spell_checking_languages(
            context: *mut WebKitWebContext,
            languages: *const *const gchar,
        );
        #[c2rust::src_loc = "309:1"]
        pub fn webkit_web_context_allow_tls_certificate_for_host(
            context: *mut WebKitWebContext,
            certificate: *mut GTlsCertificate,
            host: *const gchar,
        );
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/gio/giotypes.h:23"]
pub mod giotypes_h {
    #[c2rust::src_loc = "153:1"]
    pub type GTlsCertificate = _GTlsCertificate;
    #[c2rust::src_loc = "59:1"]
    pub type GApplication = _GApplication;
    #[c2rust::src_loc = "190:1"]
    pub type GAsyncReadyCallback = Option::<
        unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
    >;
    #[c2rust::src_loc = "36:1"]
    pub type GAsyncResult = _GAsyncResult;
    #[c2rust::src_loc = "40:1"]
    pub type GCancellable = _GCancellable;
    use super::gtlscertificate_h::_GTlsCertificate;
    use super::gapplication_h::_GApplication;
    use super::gobject_h::GObject;
    use super::gtypes_h::gpointer;
    use super::gcancellable_h::_GCancellable;
    extern "C" {
        #[c2rust::src_loc = "36:16"]
        pub type _GAsyncResult;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/gio/gtlscertificate.h:23"]
pub mod gtlscertificate_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "42:8"]
    pub struct _GTlsCertificate {
        pub parent_instance: GObject,
        pub priv_0: *mut GTlsCertificatePrivate,
    }
    #[c2rust::src_loc = "40:1"]
    pub type GTlsCertificatePrivate = _GTlsCertificatePrivate;
    use super::gobject_h::GObject;
    use super::gtypes_h::gchar;
    use super::glibconfig_h::gssize;
    use super::gerror_h::GError;
    use super::giotypes_h::GTlsCertificate;
    extern "C" {
        #[c2rust::src_loc = "40:16"]
        pub type _GTlsCertificatePrivate;
        #[c2rust::src_loc = "64:1"]
        pub fn g_tls_certificate_new_from_pem(
            data: *const gchar,
            length: gssize,
            error: *mut *mut GError,
        ) -> *mut GTlsCertificate;
    }
}
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/webkit/WebKitURISchemeRequest.h:32"]
pub mod WebKitURISchemeRequest_h {
    #[c2rust::src_loc = "51:1"]
    pub type WebKitURISchemeRequest = _WebKitURISchemeRequest;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "51:1"]
    pub struct _WebKitURISchemeRequest {
        pub parent: GObject,
        pub priv_0: *mut WebKitURISchemeRequestPrivate,
    }
    #[c2rust::src_loc = "51:1"]
    pub type WebKitURISchemeRequestPrivate = _WebKitURISchemeRequestPrivate;
    use super::gobject_h::GObject;
    extern "C" {
        #[c2rust::src_loc = "51:1"]
        pub type _WebKitURISchemeRequestPrivate;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/common.h:23"]
pub mod common_h {
    #[c2rust::src_loc = "24:1"]
    pub type common_t = _common_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "24:16"]
    pub struct _common_t {
        pub L: *mut lua_State,
    }
    use super::lua_h::lua_State;
    extern "C" {
        #[c2rust::src_loc = "29:17"]
        pub static mut common: common_t;
    }
}
#[c2rust::header_src = "/usr/include/gtk-3.0/gtk/gtktypes.h:23"]
pub mod gtktypes_h {
    #[c2rust::src_loc = "46:1"]
    pub type GtkWidget = _GtkWidget;
    #[c2rust::src_loc = "48:1"]
    pub type GtkWindow = _GtkWindow;
    #[c2rust::src_loc = "36:1"]
    pub type GtkClipboard = _GtkClipboard;
    use super::gtkwidget_h::_GtkWidget;
    use super::gtkwindow_h::_GtkWindow;
    extern "C" {
        #[c2rust::src_loc = "36:16"]
        pub type _GtkClipboard;
    }
}
#[c2rust::header_src = "/usr/include/gtk-3.0/gtk/gtkwidget.h:23"]
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
    use super::gtktypes_h::GtkWidget;
    extern "C" {
        #[c2rust::src_loc = "66:16"]
        pub type _GtkWidgetPrivate;
        #[c2rust::src_loc = "618:1"]
        pub fn gtk_widget_destroy(widget: *mut GtkWidget);
    }
}
#[c2rust::header_src = "/usr/include/gtk-3.0/gtk/gtkdialog.h:23"]
pub mod gtkdialog_h {
    #[c2rust::src_loc = "77:3"]
    pub const GTK_RESPONSE_ACCEPT: C2RustUnnamed_10 = -3;
    #[c2rust::src_loc = "80:3"]
    pub const GTK_RESPONSE_CANCEL: C2RustUnnamed_10 = -6;
    #[c2rust::src_loc = "97:1"]
    pub type GtkDialog = _GtkDialog;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "107:8"]
    pub struct _GtkDialog {
        pub window: GtkWindow,
        pub priv_0: *mut GtkDialogPrivate,
    }
    #[c2rust::src_loc = "98:1"]
    pub type GtkDialogPrivate = _GtkDialogPrivate;
    #[c2rust::src_loc = "73:9"]
    pub type C2RustUnnamed_10 = std::ffi::c_int;
    #[c2rust::src_loc = "85:3"]
    pub const GTK_RESPONSE_HELP: C2RustUnnamed_10 = -11;
    #[c2rust::src_loc = "84:3"]
    pub const GTK_RESPONSE_APPLY: C2RustUnnamed_10 = -10;
    #[c2rust::src_loc = "83:3"]
    pub const GTK_RESPONSE_NO: C2RustUnnamed_10 = -9;
    #[c2rust::src_loc = "82:3"]
    pub const GTK_RESPONSE_YES: C2RustUnnamed_10 = -8;
    #[c2rust::src_loc = "81:3"]
    pub const GTK_RESPONSE_CLOSE: C2RustUnnamed_10 = -7;
    #[c2rust::src_loc = "79:3"]
    pub const GTK_RESPONSE_OK: C2RustUnnamed_10 = -5;
    #[c2rust::src_loc = "78:3"]
    pub const GTK_RESPONSE_DELETE_EVENT: C2RustUnnamed_10 = -4;
    #[c2rust::src_loc = "76:3"]
    pub const GTK_RESPONSE_REJECT: C2RustUnnamed_10 = -2;
    #[c2rust::src_loc = "75:3"]
    pub const GTK_RESPONSE_NONE: C2RustUnnamed_10 = -1;
    use super::gtktypes_h::GtkWindow;
    use super::gtype_h::GType;
    use super::gtypes_h::gint;
    extern "C" {
        #[c2rust::src_loc = "98:16"]
        pub type _GtkDialogPrivate;
        #[c2rust::src_loc = "143:1"]
        pub fn gtk_dialog_get_type() -> GType;
        #[c2rust::src_loc = "199:1"]
        pub fn gtk_dialog_run(dialog: *mut GtkDialog) -> gint;
    }
}
#[c2rust::header_src = "/usr/include/gtk-3.0/gtk/gtkfilechooser.h:23"]
pub mod gtkfilechooser_h {
    #[c2rust::src_loc = "54:9"]
    pub type GtkFileChooserAction = std::ffi::c_uint;
    #[c2rust::src_loc = "59:3"]
    pub const GTK_FILE_CHOOSER_ACTION_CREATE_FOLDER: GtkFileChooserAction = 3;
    #[c2rust::src_loc = "58:3"]
    pub const GTK_FILE_CHOOSER_ACTION_SELECT_FOLDER: GtkFileChooserAction = 2;
    #[c2rust::src_loc = "57:3"]
    pub const GTK_FILE_CHOOSER_ACTION_SAVE: GtkFileChooserAction = 1;
    #[c2rust::src_loc = "56:3"]
    pub const GTK_FILE_CHOOSER_ACTION_OPEN: GtkFileChooserAction = 0;
    #[c2rust::src_loc = "35:1"]
    pub type GtkFileChooser = _GtkFileChooser;
    use super::gtype_h::GType;
    use super::gtypes_h::{gboolean, gchar};
    extern "C" {
        #[c2rust::src_loc = "35:16"]
        pub type _GtkFileChooser;
        #[c2rust::src_loc = "86:1"]
        pub fn gtk_file_chooser_get_type() -> GType;
        #[c2rust::src_loc = "141:1"]
        pub fn gtk_file_chooser_set_do_overwrite_confirmation(
            chooser: *mut GtkFileChooser,
            do_overwrite_confirmation: gboolean,
        );
        #[c2rust::src_loc = "155:1"]
        pub fn gtk_file_chooser_set_current_name(
            chooser: *mut GtkFileChooser,
            name: *const gchar,
        );
        #[c2rust::src_loc = "163:1"]
        pub fn gtk_file_chooser_get_filename(chooser: *mut GtkFileChooser) -> *mut gchar;
        #[c2rust::src_loc = "180:1"]
        pub fn gtk_file_chooser_set_current_folder(
            chooser: *mut GtkFileChooser,
            filename: *const gchar,
        ) -> gboolean;
    }
}
#[c2rust::header_src = "/usr/include/gtk-3.0/gtk/gtkwindow.h:23"]
pub mod gtkwindow_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "53:8"]
    pub struct _GtkWindow {
        pub bin: GtkBin,
        pub priv_0: *mut GtkWindowPrivate,
    }
    #[c2rust::src_loc = "46:1"]
    pub type GtkWindowPrivate = _GtkWindowPrivate;
    use super::gtkbin_h::GtkBin;
    use super::gtype_h::GType;
    extern "C" {
        #[c2rust::src_loc = "46:16"]
        pub type _GtkWindowPrivate;
        #[c2rust::src_loc = "144:1"]
        pub fn gtk_window_get_type() -> GType;
    }
}
#[c2rust::header_src = "/usr/include/gtk-3.0/gtk/gtkbin.h:23"]
pub mod gtkbin_h {
    #[c2rust::src_loc = "45:1"]
    pub type GtkBin = _GtkBin;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:8"]
    pub struct _GtkBin {
        pub container: GtkContainer,
        pub priv_0: *mut GtkBinPrivate,
    }
    #[c2rust::src_loc = "46:1"]
    pub type GtkBinPrivate = _GtkBinPrivate;
    use super::gtkcontainer_h::GtkContainer;
    extern "C" {
        #[c2rust::src_loc = "46:16"]
        pub type _GtkBinPrivate;
    }
}
#[c2rust::header_src = "/usr/include/gtk-3.0/gtk/gtkcontainer.h:23"]
pub mod gtkcontainer_h {
    #[c2rust::src_loc = "45:1"]
    pub type GtkContainer = _GtkContainer;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:8"]
    pub struct _GtkContainer {
        pub widget: GtkWidget,
        pub priv_0: *mut GtkContainerPrivate,
    }
    #[c2rust::src_loc = "46:1"]
    pub type GtkContainerPrivate = _GtkContainerPrivate;
    use super::gtktypes_h::GtkWidget;
    extern "C" {
        #[c2rust::src_loc = "46:16"]
        pub type _GtkContainerPrivate;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/clib/widget.h:23"]
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
    use super::common_signal_h::signal_t;
    use super::gtypes_h::{gint, gpointer, gchar};
    use super::lua_h::lua_State;
    use super::tokenize_h::luakit_token_t;
    use super::gtktypes_h::GtkWidget;
    use super::gtkcssprovider_h::GtkCssProvider;
    use super::luaclass_h::{
        lua_class_t, lua_class_allocator_t, lua_class_property_array_t,
        lua_class_propfunc_t,
    };
    extern "C" {
        #[c2rust::src_loc = "90:20"]
        pub static mut widget_class: lua_class_t;
    }
}
#[c2rust::header_src = "/usr/include/gtk-3.0/gtk/gtkcssprovider.h:23"]
pub mod gtkcssprovider_h {
    #[c2rust::src_loc = "64:1"]
    pub type GtkCssProvider = _GtkCssProvider;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "68:8"]
    pub struct _GtkCssProvider {
        pub parent_instance: GObject,
        pub priv_0: *mut GtkCssProviderPrivate,
    }
    #[c2rust::src_loc = "66:1"]
    pub type GtkCssProviderPrivate = _GtkCssProviderPrivate;
    use super::gobject_h::GObject;
    extern "C" {
        #[c2rust::src_loc = "66:16"]
        pub type _GtkCssProviderPrivate;
    }
}
#[c2rust::header_src = "/usr/include/gtk-3.0/gdk/gdktypes.h:23"]
pub mod gdktypes_h {
    #[c2rust::src_loc = "102:1"]
    pub type GdkAtom = *mut _GdkAtom;
    extern "C" {
        #[c2rust::src_loc = "102:16"]
        pub type _GdkAtom;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/globalconf.h:23"]
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
#[c2rust::header_src = "/usr/include/gtk-3.0/gtk/gtkapplication.h:23"]
pub mod gtkapplication_h {
    #[c2rust::src_loc = "39:1"]
    pub type GtkApplication = _GtkApplication;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "43:8"]
    pub struct _GtkApplication {
        pub parent: GApplication,
        pub priv_0: *mut GtkApplicationPrivate,
    }
    #[c2rust::src_loc = "41:1"]
    pub type GtkApplicationPrivate = _GtkApplicationPrivate;
    use super::giotypes_h::GApplication;
    extern "C" {
        #[c2rust::src_loc = "41:16"]
        pub type _GtkApplicationPrivate;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/gio/gapplication.h:23"]
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
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/webkit/WebKitWebsiteDataManager.h:32"]
pub mod WebKitWebsiteDataManager_h {
    #[c2rust::src_loc = "53:1"]
    pub type WebKitWebsiteDataManager = _WebKitWebsiteDataManager;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "53:1"]
    pub struct _WebKitWebsiteDataManager {
        pub parent: GObject,
        pub priv_0: *mut WebKitWebsiteDataManagerPrivate,
    }
    #[c2rust::src_loc = "53:1"]
    pub type WebKitWebsiteDataManagerPrivate = _WebKitWebsiteDataManagerPrivate;
    use super::gobject_h::GObject;
    use super::WebKitWebsiteData_h::WebKitWebsiteDataTypes;
    use super::giotypes_h::{GCancellable, GAsyncReadyCallback, GAsyncResult};
    use super::gtypes_h::{gpointer, gboolean};
    use super::gerror_h::GError;
    use super::glist_h::GList;
    use super::gdatetime_h::GTimeSpan;
    extern "C" {
        #[c2rust::src_loc = "53:1"]
        pub type _WebKitWebsiteDataManagerPrivate;
        #[c2rust::src_loc = "142:1"]
        pub fn webkit_website_data_manager_fetch(
            manager: *mut WebKitWebsiteDataManager,
            types: WebKitWebsiteDataTypes,
            cancellable: *mut GCancellable,
            callback: GAsyncReadyCallback,
            user_data: gpointer,
        );
        #[c2rust::src_loc = "149:1"]
        pub fn webkit_website_data_manager_fetch_finish(
            manager: *mut WebKitWebsiteDataManager,
            result: *mut GAsyncResult,
            error: *mut *mut GError,
        ) -> *mut GList;
        #[c2rust::src_loc = "153:1"]
        pub fn webkit_website_data_manager_remove(
            manager: *mut WebKitWebsiteDataManager,
            types: WebKitWebsiteDataTypes,
            website_data: *mut GList,
            cancellable: *mut GCancellable,
            callback: GAsyncReadyCallback,
            user_data: gpointer,
        );
        #[c2rust::src_loc = "160:1"]
        pub fn webkit_website_data_manager_remove_finish(
            manager: *mut WebKitWebsiteDataManager,
            result: *mut GAsyncResult,
            error: *mut *mut GError,
        ) -> gboolean;
        #[c2rust::src_loc = "165:1"]
        pub fn webkit_website_data_manager_clear(
            manager: *mut WebKitWebsiteDataManager,
            types: WebKitWebsiteDataTypes,
            timespan: GTimeSpan,
            cancellable: *mut GCancellable,
            callback: GAsyncReadyCallback,
            user_data: gpointer,
        );
        #[c2rust::src_loc = "173:1"]
        pub fn webkit_website_data_manager_clear_finish(
            manager: *mut WebKitWebsiteDataManager,
            result: *mut GAsyncResult,
            error: *mut *mut GError,
        ) -> gboolean;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/gio/gcancellable.h:23"]
pub mod gcancellable_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "44:8"]
    pub struct _GCancellable {
        pub parent_instance: GObject,
        pub priv_0: *mut GCancellablePrivate,
    }
    #[c2rust::src_loc = "42:1"]
    pub type GCancellablePrivate = _GCancellablePrivate;
    use super::gobject_h::GObject;
    extern "C" {
        #[c2rust::src_loc = "42:16"]
        pub type _GCancellablePrivate;
    }
}
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/webkit/WebKitWebsiteData.h:32"]
pub mod WebKitWebsiteData_h {
    #[c2rust::src_loc = "36:9"]
    pub type WebKitWebsiteDataTypes = std::ffi::c_uint;
    #[c2rust::src_loc = "51:5"]
    pub const WEBKIT_WEBSITE_DATA_ALL: WebKitWebsiteDataTypes = 16383;
    #[c2rust::src_loc = "50:5"]
    pub const WEBKIT_WEBSITE_DATA_DOM_CACHE: WebKitWebsiteDataTypes = 8192;
    #[c2rust::src_loc = "49:5"]
    pub const WEBKIT_WEBSITE_DATA_SERVICE_WORKER_REGISTRATIONS: WebKitWebsiteDataTypes = 4096;
    #[c2rust::src_loc = "48:5"]
    pub const WEBKIT_WEBSITE_DATA_ITP: WebKitWebsiteDataTypes = 2048;
    #[c2rust::src_loc = "47:5"]
    pub const WEBKIT_WEBSITE_DATA_HSTS_CACHE: WebKitWebsiteDataTypes = 1024;
    #[c2rust::src_loc = "46:5"]
    pub const WEBKIT_WEBSITE_DATA_DEVICE_ID_HASH_SALT: WebKitWebsiteDataTypes = 512;
    #[c2rust::src_loc = "45:5"]
    pub const WEBKIT_WEBSITE_DATA_COOKIES: WebKitWebsiteDataTypes = 256;
    #[c2rust::src_loc = "44:5"]
    pub const WEBKIT_WEBSITE_DATA_PLUGIN_DATA: WebKitWebsiteDataTypes = 128;
    #[c2rust::src_loc = "43:5"]
    pub const WEBKIT_WEBSITE_DATA_INDEXEDDB_DATABASES: WebKitWebsiteDataTypes = 64;
    #[c2rust::src_loc = "42:5"]
    pub const WEBKIT_WEBSITE_DATA_WEBSQL_DATABASES: WebKitWebsiteDataTypes = 32;
    #[c2rust::src_loc = "41:5"]
    pub const WEBKIT_WEBSITE_DATA_LOCAL_STORAGE: WebKitWebsiteDataTypes = 16;
    #[c2rust::src_loc = "40:5"]
    pub const WEBKIT_WEBSITE_DATA_SESSION_STORAGE: WebKitWebsiteDataTypes = 8;
    #[c2rust::src_loc = "39:5"]
    pub const WEBKIT_WEBSITE_DATA_OFFLINE_APPLICATION_CACHE: WebKitWebsiteDataTypes = 4;
    #[c2rust::src_loc = "38:5"]
    pub const WEBKIT_WEBSITE_DATA_DISK_CACHE: WebKitWebsiteDataTypes = 2;
    #[c2rust::src_loc = "37:5"]
    pub const WEBKIT_WEBSITE_DATA_MEMORY_CACHE: WebKitWebsiteDataTypes = 1;
    #[c2rust::src_loc = "34:1"]
    pub type WebKitWebsiteData = _WebKitWebsiteData;
    use super::glibconfig_h::guint64;
    extern "C" {
        #[c2rust::src_loc = "34:16"]
        pub type _WebKitWebsiteData;
        #[c2rust::src_loc = "60:1"]
        pub fn webkit_website_data_unref(website_data: *mut WebKitWebsiteData);
        #[c2rust::src_loc = "63:1"]
        pub fn webkit_website_data_get_name(
            website_data: *mut WebKitWebsiteData,
        ) -> *const std::ffi::c_char;
        #[c2rust::src_loc = "66:1"]
        pub fn webkit_website_data_get_types(
            website_data: *mut WebKitWebsiteData,
        ) -> WebKitWebsiteDataTypes;
        #[c2rust::src_loc = "69:1"]
        pub fn webkit_website_data_get_size(
            website_data: *mut WebKitWebsiteData,
            types: WebKitWebsiteDataTypes,
        ) -> guint64;
    }
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
        #[c2rust::src_loc = "407:15"]
        pub fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:21"]
pub mod stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "756:13"]
        pub fn exit(_: std::ffi::c_int) -> !;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/genviron.h:21"]
pub mod genviron_h {
    use super::gtypes_h::gchar;
    extern "C" {
        #[c2rust::src_loc = "38:1"]
        pub fn g_getenv(variable: *const gchar) -> *const gchar;
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
        #[c2rust::src_loc = "101:1"]
        pub fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gstrfuncs.h:21"]
pub mod gstrfuncs_h {
    #[inline(always)]
    #[c2rust::src_loc = "308:1"]
    pub unsafe extern "C" fn g_strdup_inline(
        mut str: *const std::ffi::c_char,
    ) -> *mut std::ffi::c_char {
        if 0 != 0 && str.is_null() {
            return 0 as *mut std::ffi::c_char;
        }
        if 0 != 0 && !str.is_null() && 0 != 0 {
            let len: size_t = (strlen(str))
                .wrapping_add(1 as std::ffi::c_int as std::ffi::c_ulong);
            let mut dup_str: *mut std::ffi::c_char = g_malloc(len)
                as *mut std::ffi::c_char;
            return memcpy(
                dup_str as *mut std::ffi::c_void,
                str as *const std::ffi::c_void,
                len,
            ) as *mut std::ffi::c_char;
        }
        return g_strdup(str);
    }
    use super::gtypes_h::{gchar, gboolean};
    use super::string_h::{strlen, memcpy};
    use super::__stddef_size_t_h::size_t;
    use super::gmem_h::g_malloc;
    extern "C" {
        #[c2rust::src_loc = "283:1"]
        pub fn g_strdup(str: *const gchar) -> *mut gchar;
        #[c2rust::src_loc = "366:1"]
        pub fn g_strfreev(str_array: *mut *mut gchar);
        #[c2rust::src_loc = "391:1"]
        pub fn g_strv_contains(strv: *const *const gchar, str: *const gchar) -> gboolean;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gshell.h:21"]
pub mod gshell_h {
    use super::gtypes_h::{gchar, gint, gboolean};
    use super::gerror_h::GError;
    extern "C" {
        #[c2rust::src_loc = "51:1"]
        pub fn g_shell_parse_argv(
            command_line: *const gchar,
            argcp: *mut gint,
            argvp: *mut *mut *mut gchar,
            error: *mut *mut GError,
        ) -> gboolean;
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
    use super::glibconfig_h::guint64;
    extern "C" {
        #[c2rust::src_loc = "282:1"]
        pub fn g_strcmp0(
            str1: *const std::ffi::c_char,
            str2: *const std::ffi::c_char,
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "650:1"]
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
#[c2rust::header_src = "/usr/include/unistd.h:21"]
pub mod unistd_h {
    extern "C" {
        #[c2rust::src_loc = "358:1"]
        pub fn close(__fd: std::ffi::c_int) -> std::ffi::c_int;
        #[c2rust::src_loc = "594:1"]
        pub fn execl(
            __path: *const std::ffi::c_char,
            __arg: *const std::ffi::c_char,
            _: ...
        ) -> std::ffi::c_int;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/clib/luakit.h:22"]
pub mod clib_luakit_h {
    use super::lua_h::lua_State;
    use super::gtypes_h::gint;
    extern "C" {
        #[c2rust::src_loc = "35:1"]
        pub fn luaH_luakit_time(L: *mut lua_State) -> gint;
        #[c2rust::src_loc = "36:1"]
        pub fn luaH_luakit_uri_encode(L: *mut lua_State) -> gint;
        #[c2rust::src_loc = "37:1"]
        pub fn luaH_luakit_uri_decode(L: *mut lua_State) -> gint;
        #[c2rust::src_loc = "38:1"]
        pub fn luaH_luakit_idle_add(L: *mut lua_State) -> gint;
        #[c2rust::src_loc = "39:1"]
        pub fn luaH_luakit_idle_remove(L: *mut lua_State) -> gint;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/luautil.h:23"]
pub mod luautil_h {
    use super::lua_h::lua_State;
    use super::gtypes_h::{gint, gchar};
    extern "C" {
        #[c2rust::src_loc = "26:1"]
        pub fn luaH_dofunction_on_error(L: *mut lua_State) -> gint;
        #[c2rust::src_loc = "29:1"]
        pub fn luaH_push_strv(L: *mut lua_State, strv: *const *const gchar) -> gint;
        #[c2rust::src_loc = "30:1"]
        pub fn luaH_checkstrv(L: *mut lua_State, idx: gint) -> *mut *const gchar;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/lualib.h:23"]
pub mod lualib_h {
    #[inline]
    #[c2rust::src_loc = "90:1"]
    pub unsafe extern "C" fn luaH_absindex(mut L: *mut lua_State, mut ud: gint) -> gint {
        return if ud >= 0 as std::ffi::c_int || ud <= -(10000 as std::ffi::c_int) {
            ud
        } else {
            lua_gettop(L) + ud + 1 as std::ffi::c_int
        };
    }
    #[inline]
    #[c2rust::src_loc = "101:1"]
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
        let mut error_func_pos: gint = lua_gettop(L) - nargs - 1 as std::ffi::c_int;
        if lua_pcall(L, nargs, nret, -nargs - 2 as std::ffi::c_int) != 0 {
            _log(
                LOG_LEVEL_error,
                b"./common/lualib.h\0" as *const u8 as *const std::ffi::c_char,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                lua_tolstring(L, -(1 as std::ffi::c_int), 0 as *mut size_t),
            );
            lua_settop(L, -(2 as std::ffi::c_int) - 1 as std::ffi::c_int);
            return 0 as std::ffi::c_int;
        }
        lua_remove(L, error_func_pos);
        return (0 as std::ffi::c_int == 0) as std::ffi::c_int;
    }
    use super::lua_h::{
        lua_State, lua_gettop, lua_insert, lua_pushcclosure, lua_pcall, lua_tolstring,
        lua_settop, lua_remove,
    };
    use super::gtypes_h::{gint, gboolean};
    use super::luautil_h::luaH_dofunction_on_error;
    use super::log_h::{_log, LOG_LEVEL_error, log_level_t};
    use super::__stddef_size_t_h::size_t;
}
#[c2rust::header_src = "/home/daana/git/luakit/common/luaobject.h:23"]
pub mod luaobject_h {
    #[inline]
    #[c2rust::src_loc = "88:1"]
    pub unsafe extern "C" fn luaH_object_registry_push(mut L: *mut lua_State) {
        lua_pushlstring(
            L,
            b"luakit.object.registry\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 23]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        lua_rawget(L, -(10000 as std::ffi::c_int));
    }
    #[inline]
    #[c2rust::src_loc = "99:1"]
    pub unsafe extern "C" fn luaH_object_ref(
        mut L: *mut lua_State,
        mut oud: gint,
    ) -> gpointer {
        luaH_object_registry_push(L);
        let mut p: gpointer = luaH_object_incref(
            L,
            -(1 as std::ffi::c_int),
            if oud < 0 as std::ffi::c_int { oud - 1 as std::ffi::c_int } else { oud },
        );
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        return p;
    }
    #[inline]
    #[c2rust::src_loc = "121:1"]
    pub unsafe extern "C" fn luaH_object_unref(mut L: *mut lua_State, mut p: gpointer) {
        luaH_object_registry_push(L);
        luaH_object_decref(L, -(1 as std::ffi::c_int), p);
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    }
    #[inline]
    #[c2rust::src_loc = "132:1"]
    pub unsafe extern "C" fn luaH_object_push(
        mut L: *mut lua_State,
        mut p: gpointer,
    ) -> gint {
        luaH_object_registry_push(L);
        lua_pushlightuserdata(L, p);
        lua_rawget(L, -(2 as std::ffi::c_int));
        lua_remove(L, -(2 as std::ffi::c_int));
        return 1 as std::ffi::c_int;
    }
    use super::lua_h::{
        lua_State, lua_pushlstring, lua_rawget, lua_settop, lua_pushlightuserdata,
        lua_remove,
    };
    use super::gtypes_h::{gint, gpointer, gchar};
    use super::common_signal_h::signal_t;
    extern "C" {
        #[c2rust::src_loc = "40:1"]
        pub fn luaH_object_incref(L: *mut lua_State, tud: gint, oud: gint) -> gpointer;
        #[c2rust::src_loc = "41:1"]
        pub fn luaH_object_decref(L: *mut lua_State, tud: gint, oud: gpointer);
        #[c2rust::src_loc = "158:1"]
        pub fn signal_object_emit(
            _: *mut lua_State,
            signals: *mut signal_t,
            name: *const gchar,
            nargs: gint,
            nret: gint,
        ) -> gint;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/luah.h:23"]
pub mod luah_h {
    #[inline]
    #[c2rust::src_loc = "39:1"]
    pub unsafe extern "C" fn luaH_checkboolean(
        mut L: *mut lua_State,
        mut n: gint,
    ) -> gboolean {
        if !(lua_type(L, n) == 1 as std::ffi::c_int) {
            luaL_typerror(L, n, b"boolean\0" as *const u8 as *const std::ffi::c_char);
        }
        return lua_toboolean(L, n);
    }
    use super::lua_h::{lua_State, lua_type, lua_toboolean};
    use super::gtypes_h::{gint, gboolean};
    use super::lauxlib_h::luaL_typerror;
}
#[c2rust::header_src = "/home/daana/git/luakit/luah.h:23"]
pub mod luakit_luah_h {
    use super::lua_h::lua_State;
    use super::gtypes_h::guint;
    extern "C" {
        #[c2rust::src_loc = "32:1"]
        pub fn luaH_keystr_push(_: *mut lua_State, _: guint);
    }
}
#[c2rust::header_src = "/usr/include/gtk-3.0/gdk/gdkkeys.h:23"]
pub mod gdkkeys_h {
    use super::gtypes_h::{gchar, guint};
    extern "C" {
        #[c2rust::src_loc = "140:1"]
        pub fn gdk_keyval_from_name(keyval_name: *const gchar) -> guint;
        #[c2rust::src_loc = "142:1"]
        pub fn gdk_keyval_convert_case(
            symbol: guint,
            lower: *mut guint,
            upper: *mut guint,
        );
    }
}
#[c2rust::header_src = "/usr/include/gtk-3.0/gtk/gtkclipboard.h:23"]
pub mod gtkclipboard_h {
    use super::gdktypes_h::{_GdkAtom, GdkAtom};
    use super::gtktypes_h::GtkClipboard;
    use super::gtypes_h::{gchar, gint};
    extern "C" {
        #[c2rust::src_loc = "189:1"]
        pub fn gtk_clipboard_get(selection: GdkAtom) -> *mut GtkClipboard;
        #[c2rust::src_loc = "215:1"]
        pub fn gtk_clipboard_clear(clipboard: *mut GtkClipboard);
        #[c2rust::src_loc = "217:1"]
        pub fn gtk_clipboard_set_text(
            clipboard: *mut GtkClipboard,
            text: *const gchar,
            len: gint,
        );
        #[c2rust::src_loc = "255:1"]
        pub fn gtk_clipboard_wait_for_text(clipboard: *mut GtkClipboard) -> *mut gchar;
    }
}
#[c2rust::header_src = "/usr/include/gtk-3.0/gtk/gtkfilechooserdialog.h:23"]
pub mod gtkfilechooserdialog_h {
    use super::gtypes_h::gchar;
    use super::gtktypes_h::{GtkWindow, GtkWidget};
    use super::gtkfilechooser_h::{GtkFileChooserAction, GTK_FILE_CHOOSER_ACTION_OPEN};
    extern "C" {
        #[c2rust::src_loc = "62:1"]
        pub fn gtk_file_chooser_dialog_new(
            title: *const gchar,
            parent: *mut GtkWindow,
            action: GtkFileChooserAction,
            first_button_text: *const gchar,
            _: ...
        ) -> *mut GtkWidget;
    }
}
#[c2rust::header_src = "/usr/include/gtk-3.0/gtk/gtkmain.h:23"]
pub mod gtkmain_h {
    use super::gtypes_h::guint;
    extern "C" {
        #[c2rust::src_loc = "155:1"]
        pub fn gtk_main_level() -> guint;
        #[c2rust::src_loc = "157:1"]
        pub fn gtk_main_quit();
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/luayield.h:25"]
pub mod luayield_h {
    use super::lua_h::lua_State;
    use super::gtypes_h::{gint, gboolean};
    extern "C" {
        #[c2rust::src_loc = "28:1"]
        pub fn luaH_yield_wrap_function(L: *mut lua_State);
        #[c2rust::src_loc = "29:1"]
        pub fn luaH_yield(L: *mut lua_State) -> std::ffi::c_int;
        #[c2rust::src_loc = "30:1"]
        pub fn luaH_resume(L: *mut lua_State, nret: gint) -> gboolean;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/resource.h:27"]
pub mod resource_h {
    use super::gtypes_h::gchar;
    extern "C" {
        #[c2rust::src_loc = "24:1"]
        pub fn resource_path_set(path: *const gchar);
        #[c2rust::src_loc = "25:1"]
        pub fn resource_path_get() -> *mut gchar;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/ipc.h:29"]
pub mod ipc_h {
    extern "C" {
        #[c2rust::src_loc = "26:1"]
        pub fn ipc_remove_socket_file();
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/log.h:31"]
pub mod luakit_log_h {
    use super::log_h::log_level_t;
    extern "C" {
        #[c2rust::src_loc = "29:1"]
        pub fn log_get_verbosity(group: *mut std::ffi::c_char) -> log_level_t;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/web_context.h:32"]
pub mod web_context_h {
    use super::WebKitWebContext_h::WebKitWebContext;
    use super::gtypes_h::{guint, gboolean};
    extern "C" {
        #[c2rust::src_loc = "28:1"]
        pub fn web_context_get() -> *mut WebKitWebContext;
        #[c2rust::src_loc = "30:1"]
        pub fn web_context_process_limit_get() -> guint;
        #[c2rust::src_loc = "31:1"]
        pub fn web_context_process_limit_set(limit: guint) -> gboolean;
    }
}
pub use self::__stddef_ptrdiff_t_h::ptrdiff_t;
pub use self::__stddef_size_t_h::size_t;
pub use self::glibconfig_h::{guint32, gint64, guint64, gssize, gsize, GPid};
pub use self::types_h::{__uint32_t, __uid_t, __pid_t, __clock_t};
pub use self::__sigval_t_h::{__sigval_t, sigval};
pub use self::gtypes_h::{
    gchar, gint, gboolean, gulong, guint, gdouble, gpointer, gconstpointer,
    GCompareDataFunc, GDestroyNotify,
};
pub use self::garray_h::{_GPtrArray, GPtrArray, g_ptr_array_free};
pub use self::gquark_h::GQuark;
pub use self::gerror_h::{_GError, GError, g_error_free, g_clear_error};
pub use self::__sigset_t_h::__sigset_t;
pub use self::sigset_t_h::sigset_t;
pub use self::siginfo_t_h::{
    siginfo_t, C2RustUnnamed, C2RustUnnamed_0, C2RustUnnamed_1, C2RustUnnamed_2,
    C2RustUnnamed_3, C2RustUnnamed_4, C2RustUnnamed_5, C2RustUnnamed_6, C2RustUnnamed_7,
    C2RustUnnamed_8,
};
pub use self::signal_h::{__sighandler_t, sigemptyset, sigaction};
pub use self::sigaction_h::{sigaction, C2RustUnnamed_9};
pub use self::gdatetime_h::GTimeSpan;
pub use self::gconvert_h::{GIConv, _GIConv};
pub use self::gdataset_h::{GData, _GData};
pub use self::glist_h::{_GList, GList, g_list_free, g_list_delete_link};
pub use self::ghash_h::{GHashTable, _GHashTable};
pub use self::gslist_h::{_GSList, GSList};
pub use self::gmain_h::{
    GIOCondition, G_IO_NVAL, G_IO_HUP, G_IO_ERR, G_IO_PRI, G_IO_OUT, G_IO_IN,
    GMainContext, _GSource, GSourcePrivate, GSource, GSourceFuncs, _GSourceFuncs,
    GSourceDummyMarshal, GSourceFunc, GSourceFuncsFinalizeFunc, GSourceFuncsDispatchFunc,
    GSourceFuncsCheckFunc, GSourceFuncsPrepareFunc, GSourceCallbackFuncs,
    _GSourceCallbackFuncs, GChildWatchFunc, _GMainContext, _GSourcePrivate,
    g_child_watch_add,
};
pub use self::gstring_h::{_GString, GString};
pub use self::giochannel_h::{
    _GIOChannel, GIOFuncs, _GIOFuncs, GIOChannel, GIOFlags, G_IO_FLAG_SET_MASK,
    G_IO_FLAG_GET_MASK, G_IO_FLAG_MASK, G_IO_FLAG_IS_SEEKABLE, G_IO_FLAG_IS_WRITEABLE,
    G_IO_FLAG_IS_WRITABLE, G_IO_FLAG_IS_READABLE, G_IO_FLAG_NONBLOCK, G_IO_FLAG_APPEND,
    G_IO_FLAG_NONE, GIOStatus, G_IO_STATUS_AGAIN, G_IO_STATUS_EOF, G_IO_STATUS_NORMAL,
    G_IO_STATUS_ERROR, GSeekType, G_SEEK_END, G_SEEK_SET, G_SEEK_CUR, g_io_channel_unref,
    g_io_channel_read_to_end, g_io_channel_unix_new,
};
pub use self::gregex_h::{
    GRegexCompileFlags, G_REGEX_JAVASCRIPT_COMPAT, G_REGEX_BSR_ANYCRLF,
    G_REGEX_NEWLINE_ANYCRLF, G_REGEX_NEWLINE_CRLF, G_REGEX_NEWLINE_LF,
    G_REGEX_NEWLINE_CR, G_REGEX_DUPNAMES, G_REGEX_FIRSTLINE, G_REGEX_OPTIMIZE,
    G_REGEX_NO_AUTO_CAPTURE, G_REGEX_RAW, G_REGEX_UNGREEDY, G_REGEX_DOLLAR_ENDONLY,
    G_REGEX_ANCHORED, G_REGEX_EXTENDED, G_REGEX_DOTALL, G_REGEX_MULTILINE,
    G_REGEX_CASELESS, G_REGEX_DEFAULT, GRegexMatchFlags, G_REGEX_MATCH_NOTEMPTY_ATSTART,
    G_REGEX_MATCH_PARTIAL_HARD, G_REGEX_MATCH_PARTIAL_SOFT, G_REGEX_MATCH_BSR_ANY,
    G_REGEX_MATCH_BSR_ANYCRLF, G_REGEX_MATCH_NEWLINE_ANYCRLF, G_REGEX_MATCH_NEWLINE_ANY,
    G_REGEX_MATCH_NEWLINE_CRLF, G_REGEX_MATCH_NEWLINE_LF, G_REGEX_MATCH_NEWLINE_CR,
    G_REGEX_MATCH_PARTIAL, G_REGEX_MATCH_NOTEMPTY, G_REGEX_MATCH_NOTEOL,
    G_REGEX_MATCH_NOTBOL, G_REGEX_MATCH_ANCHORED, G_REGEX_MATCH_DEFAULT,
    g_regex_match_simple,
};
pub use self::gspawn_h::{
    GSpawnChildSetupFunc, GSpawnFlags, G_SPAWN_STDIN_FROM_DEV_NULL,
    G_SPAWN_CHILD_INHERITS_STDERR, G_SPAWN_CHILD_INHERITS_STDOUT, G_SPAWN_CLOEXEC_PIPES,
    G_SPAWN_SEARCH_PATH_FROM_ENVP, G_SPAWN_FILE_AND_ARGV_ZERO,
    G_SPAWN_CHILD_INHERITS_STDIN, G_SPAWN_STDERR_TO_DEV_NULL, G_SPAWN_STDOUT_TO_DEV_NULL,
    G_SPAWN_SEARCH_PATH, G_SPAWN_DO_NOT_REAP_CHILD, G_SPAWN_LEAVE_DESCRIPTORS_OPEN,
    G_SPAWN_DEFAULT, g_spawn_async_with_pipes, g_spawn_command_line_sync,
    g_spawn_close_pid,
};
pub use self::gtree_h::{GTree, _GTree, g_tree_new_full};
pub use self::lua_h::{
    lua_CFunction, lua_Number, lua_Integer, lua_State, lua_gettop, lua_settop,
    lua_pushvalue, lua_remove, lua_insert, lua_isstring, lua_type, lua_typename,
    lua_tointeger, lua_toboolean, lua_tolstring, lua_objlen, lua_pushnil, lua_pushnumber,
    lua_pushinteger, lua_pushlstring, lua_pushstring, lua_pushfstring, lua_pushcclosure,
    lua_pushboolean, lua_pushlightuserdata, lua_rawget, lua_rawgeti, lua_createtable,
    lua_setfield, lua_rawset, lua_rawseti, lua_setmetatable, lua_pcall, lua_status,
    lua_error,
};
pub use self::log_h::{
    log_level_t, LOG_LEVEL_debug, LOG_LEVEL_verbose, LOG_LEVEL_info, LOG_LEVEL_warn,
    LOG_LEVEL_error, LOG_LEVEL_fatal, _log,
};
pub use self::common_signal_h::{signal_t, signal_cmp, signal_array_destroy, signal_new};
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
    L_TK_UNKNOWN, l_tokenize,
};
pub use self::lauxlib_h::{
    luaL_Reg, luaL_typerror, luaL_argerror, luaL_checklstring, luaL_optinteger,
    luaL_error,
};
pub use self::luaclass_h::{
    lua_class_property_array_t, lua_object_t, lua_class_allocator_t,
    lua_class_propfunc_t, lua_class_t, luaH_class_add_signal, luaH_class_remove_signal,
    luaH_class_emit_signal, luaH_openlib, luaH_usemetatable, luaH_checkudata,
};
pub use self::luakit_h::proc_callback_data_t;
pub use self::WebKitFaviconDatabase_h::{
    WebKitFaviconDatabase, _WebKitFaviconDatabase, WebKitFaviconDatabasePrivate,
    _WebKitFaviconDatabasePrivate, webkit_favicon_database_clear,
};
pub use self::gobject_h::{GObject, _GObject, GInitiallyUnowned, g_object_unref};
pub use self::gtype_h::{
    GTypeInstance, _GTypeInstance, GTypeClass, _GTypeClass, GType,
    g_type_check_instance_cast, g_type_check_instance_is_a,
};
pub use self::WebKitWebContext_h::{
    WebKitWebContext, _WebKitWebContext, WebKitWebContextPrivate,
    WebKitURISchemeRequestCallback, _WebKitWebContextPrivate,
    webkit_web_context_get_website_data_manager, webkit_web_context_get_favicon_database,
    webkit_web_context_register_uri_scheme,
    webkit_web_context_get_spell_checking_enabled,
    webkit_web_context_set_spell_checking_enabled,
    webkit_web_context_get_spell_checking_languages,
    webkit_web_context_set_spell_checking_languages,
    webkit_web_context_allow_tls_certificate_for_host,
};
pub use self::giotypes_h::{
    GTlsCertificate, GApplication, GAsyncReadyCallback, GAsyncResult, GCancellable,
    _GAsyncResult,
};
pub use self::gtlscertificate_h::{
    _GTlsCertificate, GTlsCertificatePrivate, _GTlsCertificatePrivate,
    g_tls_certificate_new_from_pem,
};
pub use self::WebKitURISchemeRequest_h::{
    WebKitURISchemeRequest, _WebKitURISchemeRequest, WebKitURISchemeRequestPrivate,
    _WebKitURISchemeRequestPrivate,
};
pub use self::common_h::{common_t, _common_t, common};
pub use self::gtktypes_h::{GtkWidget, GtkWindow, GtkClipboard, _GtkClipboard};
pub use self::gtkwidget_h::{
    _GtkWidget, GtkWidgetPrivate, _GtkWidgetPrivate, gtk_widget_destroy,
};
pub use self::gtkdialog_h::{
    GTK_RESPONSE_ACCEPT, GTK_RESPONSE_CANCEL, GtkDialog, _GtkDialog, GtkDialogPrivate,
    C2RustUnnamed_10, GTK_RESPONSE_HELP, GTK_RESPONSE_APPLY, GTK_RESPONSE_NO,
    GTK_RESPONSE_YES, GTK_RESPONSE_CLOSE, GTK_RESPONSE_OK, GTK_RESPONSE_DELETE_EVENT,
    GTK_RESPONSE_REJECT, GTK_RESPONSE_NONE, _GtkDialogPrivate, gtk_dialog_get_type,
    gtk_dialog_run,
};
pub use self::gtkfilechooser_h::{
    GtkFileChooserAction, GTK_FILE_CHOOSER_ACTION_CREATE_FOLDER,
    GTK_FILE_CHOOSER_ACTION_SELECT_FOLDER, GTK_FILE_CHOOSER_ACTION_SAVE,
    GTK_FILE_CHOOSER_ACTION_OPEN, GtkFileChooser, _GtkFileChooser,
    gtk_file_chooser_get_type, gtk_file_chooser_set_do_overwrite_confirmation,
    gtk_file_chooser_set_current_name, gtk_file_chooser_get_filename,
    gtk_file_chooser_set_current_folder,
};
pub use self::gtkwindow_h::{
    _GtkWindow, GtkWindowPrivate, _GtkWindowPrivate, gtk_window_get_type,
};
pub use self::gtkbin_h::{GtkBin, _GtkBin, GtkBinPrivate, _GtkBinPrivate};
pub use self::gtkcontainer_h::{
    GtkContainer, _GtkContainer, GtkContainerPrivate, _GtkContainerPrivate,
};
pub use self::widget_h::{
    widget_t, widget_destructor_t, widget_info_t, widget_constructor_t, widget_class,
};
pub use self::gtkcssprovider_h::{
    GtkCssProvider, _GtkCssProvider, GtkCssProviderPrivate, _GtkCssProviderPrivate,
};
pub use self::gdktypes_h::{GdkAtom, _GdkAtom};
pub use self::globalconf_h::{globalconf_t, globalconf};
pub use self::gtkapplication_h::{
    GtkApplication, _GtkApplication, GtkApplicationPrivate, _GtkApplicationPrivate,
};
pub use self::gapplication_h::{_GApplication, GApplicationPrivate, _GApplicationPrivate};
pub use self::WebKitWebsiteDataManager_h::{
    WebKitWebsiteDataManager, _WebKitWebsiteDataManager, WebKitWebsiteDataManagerPrivate,
    _WebKitWebsiteDataManagerPrivate, webkit_website_data_manager_fetch,
    webkit_website_data_manager_fetch_finish, webkit_website_data_manager_remove,
    webkit_website_data_manager_remove_finish, webkit_website_data_manager_clear,
    webkit_website_data_manager_clear_finish,
};
pub use self::gcancellable_h::{_GCancellable, GCancellablePrivate, _GCancellablePrivate};
pub use self::WebKitWebsiteData_h::{
    WebKitWebsiteDataTypes, WEBKIT_WEBSITE_DATA_ALL, WEBKIT_WEBSITE_DATA_DOM_CACHE,
    WEBKIT_WEBSITE_DATA_SERVICE_WORKER_REGISTRATIONS, WEBKIT_WEBSITE_DATA_ITP,
    WEBKIT_WEBSITE_DATA_HSTS_CACHE, WEBKIT_WEBSITE_DATA_DEVICE_ID_HASH_SALT,
    WEBKIT_WEBSITE_DATA_COOKIES, WEBKIT_WEBSITE_DATA_PLUGIN_DATA,
    WEBKIT_WEBSITE_DATA_INDEXEDDB_DATABASES, WEBKIT_WEBSITE_DATA_WEBSQL_DATABASES,
    WEBKIT_WEBSITE_DATA_LOCAL_STORAGE, WEBKIT_WEBSITE_DATA_SESSION_STORAGE,
    WEBKIT_WEBSITE_DATA_OFFLINE_APPLICATION_CACHE, WEBKIT_WEBSITE_DATA_DISK_CACHE,
    WEBKIT_WEBSITE_DATA_MEMORY_CACHE, WebKitWebsiteData, _WebKitWebsiteData,
    webkit_website_data_unref, webkit_website_data_get_name,
    webkit_website_data_get_types, webkit_website_data_get_size,
};
use self::string_h::{memcpy, strcmp, strlen};
use self::stdlib_h::exit;
use self::genviron_h::g_getenv;
use self::gmem_h::{g_free, g_malloc, g_malloc0_n};
pub use self::gstrfuncs_h::{g_strdup_inline, g_strdup, g_strfreev, g_strv_contains};
use self::gshell_h::g_shell_parse_argv;
use self::gslice_h::{g_slice_alloc0, g_slice_free1};
use self::gtestutils_h::{g_strcmp0, g_assertion_message_cmpint};
use self::unistd_h::{close, execl};
use self::clib_luakit_h::{
    luaH_luakit_time, luaH_luakit_uri_encode, luaH_luakit_uri_decode,
    luaH_luakit_idle_add, luaH_luakit_idle_remove,
};
use self::luautil_h::{luaH_dofunction_on_error, luaH_push_strv, luaH_checkstrv};
pub use self::lualib_h::{luaH_absindex, luaH_dofunction};
pub use self::luaobject_h::{
    luaH_object_registry_push, luaH_object_ref, luaH_object_unref, luaH_object_push,
    luaH_object_incref, luaH_object_decref, signal_object_emit,
};
pub use self::luah_h::luaH_checkboolean;
use self::luakit_luah_h::luaH_keystr_push;
use self::gdkkeys_h::{gdk_keyval_from_name, gdk_keyval_convert_case};
use self::gtkclipboard_h::{
    gtk_clipboard_get, gtk_clipboard_clear, gtk_clipboard_set_text,
    gtk_clipboard_wait_for_text,
};
use self::gtkfilechooserdialog_h::gtk_file_chooser_dialog_new;
use self::gtkmain_h::{gtk_main_level, gtk_main_quit};
use self::luayield_h::{luaH_yield_wrap_function, luaH_yield, luaH_resume};
use self::resource_h::{resource_path_set, resource_path_get};
use self::ipc_h::ipc_remove_socket_file;
use self::luakit_log_h::log_get_verbosity;
use self::web_context_h::{
    web_context_get, web_context_process_limit_get, web_context_process_limit_set,
};
extern "C" {
    #[c2rust::src_loc = "951:1"]
    pub fn luakit_uri_scheme_request_cb(_: *mut WebKitURISchemeRequest, _: gpointer);
}
#[c2rust::src_loc = "583:1"]
pub type website_data_remove_task_t = _website_data_remove_task_t;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "583:16"]
pub struct _website_data_remove_task_t {
    pub L: *mut lua_State,
    pub data_types: WebKitWebsiteDataTypes,
    pub domain: *mut std::ffi::c_char,
}
#[c2rust::src_loc = "43:20"]
static mut luakit_class: lua_class_t = lua_class_t {
    name: 0 as *const gchar,
    signals: 0 as *const signal_t as *mut signal_t,
    allocator: None,
    properties: 0 as *const lua_class_property_array_t
        as *mut lua_class_property_array_t,
    index_miss_property: None,
    newindex_miss_property: None,
};
#[inline]
#[c2rust::src_loc = "44:1"]
unsafe extern "C" fn luaH_luakit_class_add_signal(mut L: *mut lua_State) -> gint {
    luaH_class_add_signal(
        L,
        &mut luakit_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[inline]
#[c2rust::src_loc = "44:1"]
unsafe extern "C" fn luaH_luakit_class_remove_signal(mut L: *mut lua_State) -> gint {
    luaH_class_remove_signal(
        L,
        &mut luakit_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[inline]
#[c2rust::src_loc = "44:1"]
unsafe extern "C" fn luaH_luakit_class_emit_signal(mut L: *mut lua_State) -> gint {
    return luaH_class_emit_signal(
        L,
        &mut luakit_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t),
        lua_gettop(L) - 1 as std::ffi::c_int,
        -(1 as std::ffi::c_int),
    );
}
#[no_mangle]
#[c2rust::src_loc = "46:1"]
pub unsafe extern "C" fn luaH_clipboard_get(
    mut L: *mut lua_State,
    mut idx: gint,
) -> *mut GtkClipboard {
    match l_tokenize(luaL_checklstring(L, idx, 0 as *mut size_t)) as std::ffi::c_uint {
        169 => {
            return gtk_clipboard_get(
                1 as std::ffi::c_int as gulong as gpointer as GdkAtom,
            );
        }
        199 => {
            return gtk_clipboard_get(
                2 as std::ffi::c_int as gulong as gpointer as GdkAtom,
            );
        }
        30 => {
            return gtk_clipboard_get(
                69 as std::ffi::c_int as gulong as gpointer as GdkAtom,
            );
        }
        _ => {}
    }
    return 0 as *mut GtkClipboard;
}
#[c2rust::src_loc = "69:1"]
unsafe extern "C" fn luaH_luakit_selection_index(mut L: *mut lua_State) -> gint {
    let mut selection: *mut GtkClipboard = luaH_clipboard_get(L, 2 as std::ffi::c_int);
    if !selection.is_null() {
        let mut text: *mut gchar = gtk_clipboard_wait_for_text(selection);
        if !text.is_null() {
            lua_pushstring(L, text);
            g_free(text as gpointer);
            return 1 as std::ffi::c_int;
        }
    }
    return 0 as std::ffi::c_int;
}
#[c2rust::src_loc = "100:1"]
unsafe extern "C" fn luaH_luakit_selection_newindex(mut L: *mut lua_State) -> gint {
    let mut selection: *mut GtkClipboard = luaH_clipboard_get(L, 2 as std::ffi::c_int);
    if !selection.is_null() {
        let mut text: *const gchar = if !(lua_type(L, 3 as std::ffi::c_int)
            == 0 as std::ffi::c_int)
        {
            luaL_checklstring(L, 3 as std::ffi::c_int, 0 as *mut size_t)
        } else {
            0 as *const std::ffi::c_char
        };
        if !text.is_null() && *text as std::ffi::c_int != 0 {
            gtk_clipboard_set_text(selection, text, -(1 as std::ffi::c_int));
        } else {
            gtk_clipboard_clear(selection);
        }
    }
    return 0 as std::ffi::c_int;
}
#[c2rust::src_loc = "114:1"]
unsafe extern "C" fn luaH_luakit_selection_table_push(mut L: *mut lua_State) -> gint {
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_createtable(L, 0 as std::ffi::c_int, 2 as std::ffi::c_int);
    lua_pushlstring(
        L,
        b"__index\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 8]>() as std::ffi::c_ulong)
            .wrapping_div(
                ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
            )
            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
    );
    lua_pushcclosure(
        L,
        Some(
            luaH_luakit_selection_index as unsafe extern "C" fn(*mut lua_State) -> gint,
        ),
        0 as std::ffi::c_int,
    );
    lua_rawset(L, -(3 as std::ffi::c_int));
    lua_pushlstring(
        L,
        b"__newindex\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 11]>() as std::ffi::c_ulong)
            .wrapping_div(
                ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
            )
            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
    );
    lua_pushcclosure(
        L,
        Some(
            luaH_luakit_selection_newindex
                as unsafe extern "C" fn(*mut lua_State) -> gint,
        ),
        0 as std::ffi::c_int,
    );
    lua_rawset(L, -(3 as std::ffi::c_int));
    lua_setmetatable(L, -(2 as std::ffi::c_int));
    return 1 as std::ffi::c_int;
}
#[c2rust::src_loc = "145:1"]
unsafe extern "C" fn luaH_luakit_save_file(mut L: *mut lua_State) -> gint {
    let mut title: *const gchar = luaL_checklstring(
        L,
        1 as std::ffi::c_int,
        0 as *mut size_t,
    );
    let mut parent_window: *mut GtkWindow = 0 as *mut GtkWindow;
    if !(lua_type(L, 2 as std::ffi::c_int) == 0 as std::ffi::c_int) {
        let mut parent: *mut widget_t = luaH_checkudata(
            L,
            2 as std::ffi::c_int,
            &mut widget_class,
        ) as *mut widget_t;
        if ({
            let mut __inst: *mut GTypeInstance = (*parent).widget as *mut GTypeInstance;
            let mut __t: GType = gtk_window_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as std::ffi::c_int;
            } else if !((*__inst).g_class).is_null()
                && (*(*__inst).g_class).g_type == __t
            {
                __r = (0 as std::ffi::c_int == 0) as std::ffi::c_int;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) == 0
        {
            luaL_argerror(
                L,
                2 as std::ffi::c_int,
                b"window widget\0" as *const u8 as *const std::ffi::c_char,
            );
        }
        parent_window = g_type_check_instance_cast(
            (*parent).widget as *mut GTypeInstance,
            gtk_window_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkWindow;
    }
    let mut default_folder: *const gchar = luaL_checklstring(
        L,
        3 as std::ffi::c_int,
        0 as *mut size_t,
    );
    let mut default_name: *const gchar = luaL_checklstring(
        L,
        4 as std::ffi::c_int,
        0 as *mut size_t,
    );
    let mut dialog: *mut GtkWidget = gtk_file_chooser_dialog_new(
        title,
        parent_window,
        GTK_FILE_CHOOSER_ACTION_SAVE,
        b"_Cancel\0" as *const u8 as *const std::ffi::c_char,
        GTK_RESPONSE_CANCEL as std::ffi::c_int,
        b"_Save\0" as *const u8 as *const std::ffi::c_char,
        GTK_RESPONSE_ACCEPT as std::ffi::c_int,
        0 as *mut std::ffi::c_void,
    );
    gtk_file_chooser_set_current_folder(
        g_type_check_instance_cast(
            dialog as *mut GTypeInstance,
            gtk_file_chooser_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkFileChooser,
        default_folder,
    );
    gtk_file_chooser_set_current_name(
        g_type_check_instance_cast(
            dialog as *mut GTypeInstance,
            gtk_file_chooser_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkFileChooser,
        default_name,
    );
    gtk_file_chooser_set_do_overwrite_confirmation(
        g_type_check_instance_cast(
            dialog as *mut GTypeInstance,
            gtk_file_chooser_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkFileChooser,
        (0 as std::ffi::c_int == 0) as std::ffi::c_int,
    );
    if gtk_dialog_run(
        g_type_check_instance_cast(dialog as *mut GTypeInstance, gtk_dialog_get_type())
            as *mut std::ffi::c_void as *mut GtkDialog,
    ) == GTK_RESPONSE_ACCEPT as std::ffi::c_int
    {
        let mut filename: *mut gchar = gtk_file_chooser_get_filename(
            g_type_check_instance_cast(
                dialog as *mut GTypeInstance,
                gtk_file_chooser_get_type(),
            ) as *mut std::ffi::c_void as *mut GtkFileChooser,
        );
        lua_pushstring(L, filename);
        g_free(filename as gpointer);
    } else {
        lua_pushnil(L);
    }
    gtk_widget_destroy(dialog);
    return 1 as std::ffi::c_int;
}
#[c2rust::src_loc = "208:1"]
unsafe extern "C" fn luaH_luakit_spawn_sync(mut L: *mut lua_State) -> gint {
    let mut e: *mut GError = 0 as *mut GError;
    let mut _stdout: *mut gchar = 0 as *mut gchar;
    let mut _stderr: *mut gchar = 0 as *mut gchar;
    let mut rv: gint = 0;
    let mut sigact: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_9 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut oldact: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_9 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut command: *const gchar = luaL_checklstring(
        L,
        1 as std::ffi::c_int,
        0 as *mut size_t,
    );
    sigact.__sigaction_handler.sa_handler = None;
    sigemptyset(&mut sigact.sa_mask);
    sigact.sa_flags = 0 as std::ffi::c_int;
    if sigaction(17 as std::ffi::c_int, &mut sigact, &mut oldact) != 0 {
        _log(
            LOG_LEVEL_fatal,
            b"clib/luakit.c\0" as *const u8 as *const std::ffi::c_char,
            b"Can't clear SIGCHLD handler\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    g_spawn_command_line_sync(command, &mut _stdout, &mut _stderr, &mut rv, &mut e);
    if sigaction(17 as std::ffi::c_int, &mut oldact, 0 as *mut sigaction) != 0 {
        _log(
            LOG_LEVEL_fatal,
            b"clib/luakit.c\0" as *const u8 as *const std::ffi::c_char,
            b"Can't restore SIGCHLD handler\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    if !e.is_null() {
        lua_pushstring(L, (*e).message);
        g_clear_error(&mut e);
        lua_error(L);
    }
    lua_pushinteger(
        L,
        ((rv & 0xff00 as std::ffi::c_int) >> 8 as std::ffi::c_int) as lua_Integer,
    );
    lua_pushstring(L, _stdout);
    lua_pushstring(L, _stderr);
    g_free(_stdout as gpointer);
    g_free(_stderr as gpointer);
    return 3 as std::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "261:1"]
pub unsafe extern "C" fn read_proc_output(
    mut fd: std::ffi::c_int,
    mut L: *mut lua_State,
    mut ptr_out: *mut *mut gchar,
    mut len_out: *mut gsize,
) -> bool {
    let mut g_out: *mut GIOChannel = g_io_channel_unix_new(fd);
    let mut e: *mut GError = 0 as *mut GError;
    g_io_channel_read_to_end(g_out, ptr_out, len_out, &mut e);
    if !e.is_null() {
        lua_pushstring(L, (*e).message);
        g_clear_error(&mut e);
        g_free(*ptr_out as gpointer);
        lua_error(L);
        g_io_channel_unref(g_out);
        close(fd);
        return 0 as std::ffi::c_int != 0;
    }
    g_io_channel_unref(g_out);
    close(fd);
    return 1 as std::ffi::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "286:1"]
pub unsafe extern "C" fn async_callback_handler(
    mut pid: GPid,
    mut status: gint,
    mut data: gpointer,
) {
    if data.is_null() {
        g_spawn_close_pid(pid);
        return;
    }
    let mut L: *mut lua_State = common.L;
    let mut cb: *mut proc_callback_data_t = data as *mut proc_callback_data_t;
    let mut cb_ref: gpointer = (*cb).cb_ref;
    let mut stdout_fd: std::ffi::c_int = (*cb).stdout_fd;
    let mut stderr_fd: std::ffi::c_int = (*cb).stderr_fd;
    let mut str_stdout: *mut gchar = 0 as *mut gchar;
    let mut len_stdout: gsize = 0;
    if !read_proc_output(stdout_fd, L, &mut str_stdout, &mut len_stdout) {
        g_spawn_close_pid(pid);
        return;
    }
    let mut str_stderr: *mut gchar = 0 as *mut gchar;
    let mut len_stderr: gsize = 0;
    if !read_proc_output(stderr_fd, L, &mut str_stderr, &mut len_stderr) {
        g_free(str_stdout as gpointer);
        g_spawn_close_pid(pid);
        return;
    }
    g_spawn_close_pid(pid);
    if status & 0x7f as std::ffi::c_int == 0 as std::ffi::c_int {
        lua_pushlstring(
            L,
            b"exit\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 5]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        lua_pushinteger(
            L,
            ((status & 0xff00 as std::ffi::c_int) >> 8 as std::ffi::c_int) as lua_Integer,
        );
        lua_pushlstring(L, str_stdout, len_stdout);
        lua_pushlstring(L, str_stderr, len_stderr);
    } else if ((status & 0x7f as std::ffi::c_int) + 1 as std::ffi::c_int)
        as std::ffi::c_schar as std::ffi::c_int >> 1 as std::ffi::c_int
        > 0 as std::ffi::c_int
    {
        lua_pushlstring(
            L,
            b"signal\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 7]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        lua_pushinteger(L, (status & 0x7f as std::ffi::c_int) as lua_Integer);
        lua_pushlstring(L, str_stdout, len_stdout);
        lua_pushlstring(L, str_stderr, len_stderr);
    } else {
        lua_pushlstring(
            L,
            b"unknown\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 8]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        lua_pushinteger(L, -(1 as std::ffi::c_int) as lua_Integer);
    }
    luaH_object_push(L, cb_ref);
    luaH_dofunction(L, 4 as std::ffi::c_int, 0 as std::ffi::c_int);
    g_free(str_stdout as gpointer);
    g_free(str_stderr as gpointer);
    luaH_object_unref(L, cb_ref);
}
#[c2rust::src_loc = "392:1"]
unsafe extern "C" fn luaH_luakit_spawn(mut L: *mut lua_State) -> gint {
    let mut e: *mut GError = 0 as *mut GError;
    let mut pid: GPid = 0 as std::ffi::c_int;
    let mut command: *const gchar = luaL_checklstring(
        L,
        1 as std::ffi::c_int,
        0 as *mut size_t,
    );
    let mut argc: gint = 0 as std::ffi::c_int;
    let mut argv: *mut *mut gchar = 0 as *mut *mut gchar;
    let mut cb: *mut proc_callback_data_t = g_malloc0_n(
        1 as std::ffi::c_int as gsize,
        ::core::mem::size_of::<proc_callback_data_t>() as std::ffi::c_ulong,
    ) as *mut proc_callback_data_t;
    if lua_gettop(L) > 1 as std::ffi::c_int
        && !(lua_type(L, 2 as std::ffi::c_int) == 0 as std::ffi::c_int)
    {
        if lua_type(L, 2 as std::ffi::c_int) == 6 as std::ffi::c_int {
            (*cb).cb_ref = luaH_object_ref(L, 2 as std::ffi::c_int);
        } else if lua_type(L, 4 as std::ffi::c_int) == 6 as std::ffi::c_int {
            (*cb).cb_ref = luaH_object_ref(L, 4 as std::ffi::c_int);
        } else {
            luaL_typerror(
                L,
                2 as std::ffi::c_int,
                lua_typename(L, 6 as std::ffi::c_int),
            );
        }
    }
    if !(g_shell_parse_argv(command, &mut argc, &mut argv, &mut e) == 0) {
        if !(g_spawn_async_with_pipes(
            0 as *const gchar,
            argv,
            0 as *mut *mut gchar,
            (G_SPAWN_DO_NOT_REAP_CHILD as std::ffi::c_int
                | G_SPAWN_SEARCH_PATH as std::ffi::c_int) as GSpawnFlags,
            None,
            0 as *mut std::ffi::c_void,
            &mut pid,
            0 as *mut gint,
            &mut (*cb).stdout_fd,
            &mut (*cb).stderr_fd,
            &mut e,
        ) == 0)
        {
            g_child_watch_add(
                pid,
                Some(
                    async_callback_handler
                        as unsafe extern "C" fn(GPid, gint, gpointer) -> (),
                ),
                cb as gpointer,
            );
            g_strfreev(argv);
            lua_pushnumber(L, pid as lua_Number);
            return 1 as std::ffi::c_int;
        }
    }
    luaH_object_unref(L, (*cb).cb_ref);
    lua_pushstring(L, (*e).message);
    g_clear_error(&mut e);
    g_strfreev(argv);
    lua_error(L);
    return 0 as std::ffi::c_int;
}
#[c2rust::src_loc = "449:1"]
unsafe extern "C" fn luaH_luakit_exec(mut L: *mut lua_State) -> gint {
    static mut shell: *const gchar = 0 as *const gchar;
    if shell.is_null()
        && {
            shell = g_getenv(b"SHELL\0" as *const u8 as *const std::ffi::c_char);
            shell.is_null()
        }
    {
        shell = b"/bin/sh\0" as *const u8 as *const std::ffi::c_char;
    }
    ipc_remove_socket_file();
    execl(
        shell,
        shell,
        b"-c\0" as *const u8 as *const std::ffi::c_char,
        luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t),
        0 as *mut std::ffi::c_void,
    );
    return 0 as std::ffi::c_int;
}
#[c2rust::src_loc = "465:1"]
unsafe extern "C" fn luaH_luakit_push_options_table(mut L: *mut lua_State) -> gint {
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    let mut i: guint = 0 as std::ffi::c_int as guint;
    while i < (*globalconf.argv).len {
        lua_pushstring(
            L,
            *((*globalconf.argv).pdata).offset(i as isize) as *const std::ffi::c_char,
        );
        lua_rawseti(
            L,
            -(2 as std::ffi::c_int),
            i.wrapping_add(1 as std::ffi::c_int as guint) as std::ffi::c_int,
        );
        i = i.wrapping_add(1);
        i;
    }
    return 1 as std::ffi::c_int;
}
#[c2rust::src_loc = "476:1"]
unsafe extern "C" fn luaH_parse_website_data_types_table(
    mut L: *mut lua_State,
    mut idx: gint,
) -> WebKitWebsiteDataTypes {
    let mut types: WebKitWebsiteDataTypes = 0 as WebKitWebsiteDataTypes;
    idx = luaH_absindex(L, idx);
    if !(lua_type(L, idx) == 5 as std::ffi::c_int) {
        luaL_typerror(L, idx, b"table\0" as *const u8 as *const std::ffi::c_char);
    }
    let mut len: size_t = lua_objlen(L, idx);
    let mut i: size_t = 1 as std::ffi::c_int as size_t;
    while i <= len {
        lua_rawgeti(L, idx, i as std::ffi::c_int);
        if lua_isstring(L, -(1 as std::ffi::c_int)) == 0 {
            luaL_error(
                L,
                b"website data types must be strings\0" as *const u8
                    as *const std::ffi::c_char,
            );
        }
        let mut type_0: *const std::ffi::c_char = lua_tolstring(
            L,
            -(1 as std::ffi::c_int),
            0 as *mut size_t,
        );
        if strcmp(type_0, b"memory_cache\0" as *const u8 as *const std::ffi::c_char)
            == 0 as std::ffi::c_int
        {
            types = ::core::mem::transmute::<
                std::ffi::c_uint,
                WebKitWebsiteDataTypes,
            >(
                types as std::ffi::c_uint
                    | WEBKIT_WEBSITE_DATA_MEMORY_CACHE as std::ffi::c_int
                        as std::ffi::c_uint,
            );
        }
        if strcmp(type_0, b"disk_cache\0" as *const u8 as *const std::ffi::c_char)
            == 0 as std::ffi::c_int
        {
            types = ::core::mem::transmute::<
                std::ffi::c_uint,
                WebKitWebsiteDataTypes,
            >(
                types as std::ffi::c_uint
                    | WEBKIT_WEBSITE_DATA_DISK_CACHE as std::ffi::c_int
                        as std::ffi::c_uint,
            );
        }
        if strcmp(
            type_0,
            b"offline_application_cache\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            types = ::core::mem::transmute::<
                std::ffi::c_uint,
                WebKitWebsiteDataTypes,
            >(
                types as std::ffi::c_uint
                    | WEBKIT_WEBSITE_DATA_OFFLINE_APPLICATION_CACHE as std::ffi::c_int
                        as std::ffi::c_uint,
            );
        }
        if strcmp(type_0, b"session_storage\0" as *const u8 as *const std::ffi::c_char)
            == 0 as std::ffi::c_int
        {
            types = ::core::mem::transmute::<
                std::ffi::c_uint,
                WebKitWebsiteDataTypes,
            >(
                types as std::ffi::c_uint
                    | WEBKIT_WEBSITE_DATA_SESSION_STORAGE as std::ffi::c_int
                        as std::ffi::c_uint,
            );
        }
        if strcmp(type_0, b"local_storage\0" as *const u8 as *const std::ffi::c_char)
            == 0 as std::ffi::c_int
        {
            types = ::core::mem::transmute::<
                std::ffi::c_uint,
                WebKitWebsiteDataTypes,
            >(
                types as std::ffi::c_uint
                    | WEBKIT_WEBSITE_DATA_LOCAL_STORAGE as std::ffi::c_int
                        as std::ffi::c_uint,
            );
        }
        if strcmp(
            type_0,
            b"indexeddb_databases\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            types = ::core::mem::transmute::<
                std::ffi::c_uint,
                WebKitWebsiteDataTypes,
            >(
                types as std::ffi::c_uint
                    | WEBKIT_WEBSITE_DATA_INDEXEDDB_DATABASES as std::ffi::c_int
                        as std::ffi::c_uint,
            );
        }
        if strcmp(type_0, b"plugin_data\0" as *const u8 as *const std::ffi::c_char)
            == 0 as std::ffi::c_int
        {
            types = ::core::mem::transmute::<
                std::ffi::c_uint,
                WebKitWebsiteDataTypes,
            >(
                types as std::ffi::c_uint
                    | WEBKIT_WEBSITE_DATA_PLUGIN_DATA as std::ffi::c_int
                        as std::ffi::c_uint,
            );
        }
        if strcmp(type_0, b"cookies\0" as *const u8 as *const std::ffi::c_char)
            == 0 as std::ffi::c_int
        {
            types = ::core::mem::transmute::<
                std::ffi::c_uint,
                WebKitWebsiteDataTypes,
            >(
                types as std::ffi::c_uint
                    | WEBKIT_WEBSITE_DATA_COOKIES as std::ffi::c_int as std::ffi::c_uint,
            );
        }
        if strcmp(
            type_0,
            b"device_id_hash_salt\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            types = ::core::mem::transmute::<
                std::ffi::c_uint,
                WebKitWebsiteDataTypes,
            >(
                types as std::ffi::c_uint
                    | WEBKIT_WEBSITE_DATA_DEVICE_ID_HASH_SALT as std::ffi::c_int
                        as std::ffi::c_uint,
            );
        }
        if strcmp(type_0, b"hsts_cache\0" as *const u8 as *const std::ffi::c_char)
            == 0 as std::ffi::c_int
        {
            types = ::core::mem::transmute::<
                std::ffi::c_uint,
                WebKitWebsiteDataTypes,
            >(
                types as std::ffi::c_uint
                    | WEBKIT_WEBSITE_DATA_HSTS_CACHE as std::ffi::c_int
                        as std::ffi::c_uint,
            );
        }
        if strcmp(type_0, b"all\0" as *const u8 as *const std::ffi::c_char)
            == 0 as std::ffi::c_int
        {
            types = ::core::mem::transmute::<
                std::ffi::c_uint,
                WebKitWebsiteDataTypes,
            >(
                types as std::ffi::c_uint
                    | WEBKIT_WEBSITE_DATA_ALL as std::ffi::c_int as std::ffi::c_uint,
            );
        }
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        i = i.wrapping_add(1);
        i;
    }
    return types;
}
#[c2rust::src_loc = "514:1"]
unsafe extern "C" fn website_data_fetch_finish(
    mut manager: *mut WebKitWebsiteDataManager,
    mut result: *mut GAsyncResult,
    mut L: *mut lua_State,
) {
    let mut __n1: gint64 = lua_status(L) as gint64;
    let mut __n2: gint64 = 1 as std::ffi::c_int as gint64;
    if !(__n1 == __n2) {
        g_assertion_message_cmpint(
            0 as *mut gchar,
            b"clib/luakit.c\0" as *const u8 as *const std::ffi::c_char,
            517 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 26],
                &[std::ffi::c_char; 26],
            >(b"website_data_fetch_finish\0"))
                .as_ptr(),
            b"lua_status(L) == LUA_YIELD\0" as *const u8 as *const std::ffi::c_char,
            __n1 as guint64,
            b"==\0" as *const u8 as *const std::ffi::c_char,
            __n2 as guint64,
            'i' as i32 as std::ffi::c_char,
        );
    }
    let mut error: *mut GError = 0 as *mut GError;
    let mut items: *mut GList = webkit_website_data_manager_fetch_finish(
        manager,
        result,
        &mut error,
    );
    if !error.is_null() {
        lua_pushnil(L);
        lua_pushstring(L, (*error).message);
        g_error_free(error);
    } else {
        lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
        let mut item: *mut GList = items;
        while !item.is_null() {
            let mut website_data: *mut WebKitWebsiteData = (*item).data
                as *mut WebKitWebsiteData;
            let mut present: WebKitWebsiteDataTypes = webkit_website_data_get_types(
                website_data,
            );
            lua_pushstring(L, webkit_website_data_get_name(website_data));
            lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
            if present as std::ffi::c_uint
                & WEBKIT_WEBSITE_DATA_MEMORY_CACHE as std::ffi::c_int as std::ffi::c_uint
                != 0
            {
                lua_pushstring(
                    L,
                    b"memory_cache\0" as *const u8 as *const std::ffi::c_char,
                );
                lua_pushinteger(
                    L,
                    webkit_website_data_get_size(
                        website_data,
                        WEBKIT_WEBSITE_DATA_MEMORY_CACHE,
                    ) as lua_Integer,
                );
                lua_rawset(L, -(3 as std::ffi::c_int));
            }
            if present as std::ffi::c_uint
                & WEBKIT_WEBSITE_DATA_DISK_CACHE as std::ffi::c_int as std::ffi::c_uint
                != 0
            {
                lua_pushstring(
                    L,
                    b"disk_cache\0" as *const u8 as *const std::ffi::c_char,
                );
                lua_pushinteger(
                    L,
                    webkit_website_data_get_size(
                        website_data,
                        WEBKIT_WEBSITE_DATA_DISK_CACHE,
                    ) as lua_Integer,
                );
                lua_rawset(L, -(3 as std::ffi::c_int));
            }
            if present as std::ffi::c_uint
                & WEBKIT_WEBSITE_DATA_OFFLINE_APPLICATION_CACHE as std::ffi::c_int
                    as std::ffi::c_uint != 0
            {
                lua_pushstring(
                    L,
                    b"offline_application_cache\0" as *const u8
                        as *const std::ffi::c_char,
                );
                lua_pushinteger(
                    L,
                    webkit_website_data_get_size(
                        website_data,
                        WEBKIT_WEBSITE_DATA_OFFLINE_APPLICATION_CACHE,
                    ) as lua_Integer,
                );
                lua_rawset(L, -(3 as std::ffi::c_int));
            }
            if present as std::ffi::c_uint
                & WEBKIT_WEBSITE_DATA_SESSION_STORAGE as std::ffi::c_int
                    as std::ffi::c_uint != 0
            {
                lua_pushstring(
                    L,
                    b"session_storage\0" as *const u8 as *const std::ffi::c_char,
                );
                lua_pushinteger(
                    L,
                    webkit_website_data_get_size(
                        website_data,
                        WEBKIT_WEBSITE_DATA_SESSION_STORAGE,
                    ) as lua_Integer,
                );
                lua_rawset(L, -(3 as std::ffi::c_int));
            }
            if present as std::ffi::c_uint
                & WEBKIT_WEBSITE_DATA_LOCAL_STORAGE as std::ffi::c_int
                    as std::ffi::c_uint != 0
            {
                lua_pushstring(
                    L,
                    b"local_storage\0" as *const u8 as *const std::ffi::c_char,
                );
                lua_pushinteger(
                    L,
                    webkit_website_data_get_size(
                        website_data,
                        WEBKIT_WEBSITE_DATA_LOCAL_STORAGE,
                    ) as lua_Integer,
                );
                lua_rawset(L, -(3 as std::ffi::c_int));
            }
            if present as std::ffi::c_uint
                & WEBKIT_WEBSITE_DATA_INDEXEDDB_DATABASES as std::ffi::c_int
                    as std::ffi::c_uint != 0
            {
                lua_pushstring(
                    L,
                    b"indexeddb_databases\0" as *const u8 as *const std::ffi::c_char,
                );
                lua_pushinteger(
                    L,
                    webkit_website_data_get_size(
                        website_data,
                        WEBKIT_WEBSITE_DATA_INDEXEDDB_DATABASES,
                    ) as lua_Integer,
                );
                lua_rawset(L, -(3 as std::ffi::c_int));
            }
            if present as std::ffi::c_uint
                & WEBKIT_WEBSITE_DATA_PLUGIN_DATA as std::ffi::c_int as std::ffi::c_uint
                != 0
            {
                lua_pushstring(
                    L,
                    b"plugin_data\0" as *const u8 as *const std::ffi::c_char,
                );
                lua_pushinteger(
                    L,
                    webkit_website_data_get_size(
                        website_data,
                        WEBKIT_WEBSITE_DATA_PLUGIN_DATA,
                    ) as lua_Integer,
                );
                lua_rawset(L, -(3 as std::ffi::c_int));
            }
            if present as std::ffi::c_uint
                & WEBKIT_WEBSITE_DATA_COOKIES as std::ffi::c_int as std::ffi::c_uint != 0
            {
                lua_pushstring(L, b"cookies\0" as *const u8 as *const std::ffi::c_char);
                lua_pushinteger(
                    L,
                    webkit_website_data_get_size(
                        website_data,
                        WEBKIT_WEBSITE_DATA_COOKIES,
                    ) as lua_Integer,
                );
                lua_rawset(L, -(3 as std::ffi::c_int));
            }
            if present as std::ffi::c_uint
                & WEBKIT_WEBSITE_DATA_DEVICE_ID_HASH_SALT as std::ffi::c_int
                    as std::ffi::c_uint != 0
            {
                lua_pushstring(
                    L,
                    b"device_id_hash_salt\0" as *const u8 as *const std::ffi::c_char,
                );
                lua_pushinteger(
                    L,
                    webkit_website_data_get_size(
                        website_data,
                        WEBKIT_WEBSITE_DATA_DEVICE_ID_HASH_SALT,
                    ) as lua_Integer,
                );
                lua_rawset(L, -(3 as std::ffi::c_int));
            }
            if present as std::ffi::c_uint
                & WEBKIT_WEBSITE_DATA_HSTS_CACHE as std::ffi::c_int as std::ffi::c_uint
                != 0
            {
                lua_pushstring(
                    L,
                    b"hsts_cache\0" as *const u8 as *const std::ffi::c_char,
                );
                lua_pushinteger(
                    L,
                    webkit_website_data_get_size(
                        website_data,
                        WEBKIT_WEBSITE_DATA_HSTS_CACHE,
                    ) as lua_Integer,
                );
                lua_rawset(L, -(3 as std::ffi::c_int));
            }
            lua_rawset(L, -(3 as std::ffi::c_int));
            webkit_website_data_unref(website_data);
            item = (*item).next;
        }
    }
    g_list_free(items);
    luaH_resume(L, lua_gettop(L));
}
#[c2rust::src_loc = "567:1"]
unsafe extern "C" fn luaH_luakit_website_data_fetch(mut L: *mut lua_State) -> gint {
    let mut data_types: WebKitWebsiteDataTypes = luaH_parse_website_data_types_table(
        L,
        1 as std::ffi::c_int,
    );
    if data_types as std::ffi::c_uint == 0 as std::ffi::c_int as std::ffi::c_uint {
        return luaL_error(
            L,
            b"no website data types specified\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    let mut web_context: *mut WebKitWebContext = web_context_get();
    let mut data_manager: *mut WebKitWebsiteDataManager = webkit_web_context_get_website_data_manager(
        web_context,
    );
    webkit_website_data_manager_fetch(
        data_manager,
        data_types,
        0 as *mut GCancellable,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut WebKitWebsiteDataManager,
                    *mut GAsyncResult,
                    *mut lua_State,
                ) -> (),
            >,
            GAsyncReadyCallback,
        >(
            Some(
                website_data_fetch_finish
                    as unsafe extern "C" fn(
                        *mut WebKitWebsiteDataManager,
                        *mut GAsyncResult,
                        *mut lua_State,
                    ) -> (),
            ),
        ),
        L as gpointer,
    );
    return luaH_yield(L);
}
#[c2rust::src_loc = "589:1"]
unsafe extern "C" fn website_data_remove_finish(
    mut manager: *mut WebKitWebsiteDataManager,
    mut result: *mut GAsyncResult,
    mut wdrt: *mut website_data_remove_task_t,
) {
    let mut L: *mut lua_State = (*wdrt).L;
    let mut __n1: gint64 = lua_status(L) as gint64;
    let mut __n2: gint64 = 1 as std::ffi::c_int as gint64;
    if !(__n1 == __n2) {
        g_assertion_message_cmpint(
            0 as *mut gchar,
            b"clib/luakit.c\0" as *const u8 as *const std::ffi::c_char,
            593 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[std::ffi::c_char; 27],
            >(b"website_data_remove_finish\0"))
                .as_ptr(),
            b"lua_status(L) == LUA_YIELD\0" as *const u8 as *const std::ffi::c_char,
            __n1 as guint64,
            b"==\0" as *const u8 as *const std::ffi::c_char,
            __n2 as guint64,
            'i' as i32 as std::ffi::c_char,
        );
    }
    let mut error: *mut GError = 0 as *mut GError;
    webkit_website_data_manager_remove_finish(manager, result, &mut error);
    if !error.is_null() {
        lua_pushnil(L);
        lua_pushstring(L, (*error).message);
        g_error_free(error);
    } else {
        lua_pushboolean(L, (0 as std::ffi::c_int == 0) as std::ffi::c_int);
    }
    g_free((*wdrt).domain as gpointer);
    g_slice_free1(
        ::core::mem::size_of::<website_data_remove_task_t>() as std::ffi::c_ulong,
        wdrt as gpointer,
    );
    luaH_resume(L, lua_gettop(L));
}
#[c2rust::src_loc = "609:1"]
unsafe extern "C" fn luaH_luakit_website_data_remove_cont(
    mut manager: *mut WebKitWebsiteDataManager,
    mut result: *mut GAsyncResult,
    mut wdrt: *mut website_data_remove_task_t,
) {
    let mut L: *mut lua_State = (*wdrt).L;
    let mut __n1: gint64 = lua_status(L) as gint64;
    let mut __n2: gint64 = 1 as std::ffi::c_int as gint64;
    if !(__n1 == __n2) {
        g_assertion_message_cmpint(
            0 as *mut gchar,
            b"clib/luakit.c\0" as *const u8 as *const std::ffi::c_char,
            613 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 37],
                &[std::ffi::c_char; 37],
            >(b"luaH_luakit_website_data_remove_cont\0"))
                .as_ptr(),
            b"lua_status(L) == LUA_YIELD\0" as *const u8 as *const std::ffi::c_char,
            __n1 as guint64,
            b"==\0" as *const u8 as *const std::ffi::c_char,
            __n2 as guint64,
            'i' as i32 as std::ffi::c_char,
        );
    }
    let mut error: *mut GError = 0 as *mut GError;
    let mut items: *mut GList = webkit_website_data_manager_fetch_finish(
        manager,
        result,
        &mut error,
    );
    if !error.is_null() {
        lua_pushstring(L, (*error).message);
        g_error_free(error);
        g_free((*wdrt).domain as gpointer);
        g_slice_free1(
            ::core::mem::size_of::<website_data_remove_task_t>() as std::ffi::c_ulong,
            wdrt as gpointer,
        );
        luaL_error(L, lua_tolstring(L, -(1 as std::ffi::c_int), 0 as *mut size_t));
    }
    let mut item: *mut GList = items;
    while !item.is_null() {
        let mut website_data: *mut WebKitWebsiteData = (*item).data
            as *mut WebKitWebsiteData;
        let mut next: *mut GList = (*item).next;
        if !(strcmp(
            webkit_website_data_get_name(website_data),
            (*wdrt).domain as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int)
        {
            webkit_website_data_unref(website_data);
            items = g_list_delete_link(items, item);
        }
        item = next;
    }
    if items.is_null() {
        g_free((*wdrt).domain as gpointer);
        g_slice_free1(
            ::core::mem::size_of::<website_data_remove_task_t>() as std::ffi::c_ulong,
            wdrt as gpointer,
        );
        lua_pushboolean(L, (0 as std::ffi::c_int == 0) as std::ffi::c_int);
        luaH_resume(L, 1 as std::ffi::c_int);
        return;
    }
    let mut web_context: *mut WebKitWebContext = web_context_get();
    let mut data_manager: *mut WebKitWebsiteDataManager = webkit_web_context_get_website_data_manager(
        web_context,
    );
    webkit_website_data_manager_remove(
        data_manager,
        (*wdrt).data_types,
        items,
        0 as *mut GCancellable,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut WebKitWebsiteDataManager,
                    *mut GAsyncResult,
                    *mut website_data_remove_task_t,
                ) -> (),
            >,
            GAsyncReadyCallback,
        >(
            Some(
                website_data_remove_finish
                    as unsafe extern "C" fn(
                        *mut WebKitWebsiteDataManager,
                        *mut GAsyncResult,
                        *mut website_data_remove_task_t,
                    ) -> (),
            ),
        ),
        wdrt as gpointer,
    );
    item = items;
    while !item.is_null() {
        webkit_website_data_unref((*item).data as *mut WebKitWebsiteData);
        item = (*item).next;
    }
    g_list_free(items);
}
#[c2rust::src_loc = "657:1"]
unsafe extern "C" fn luaH_luakit_website_data_remove(mut L: *mut lua_State) -> gint {
    let mut data_types: WebKitWebsiteDataTypes = luaH_parse_website_data_types_table(
        L,
        1 as std::ffi::c_int,
    );
    if data_types as std::ffi::c_uint == 0 as std::ffi::c_int as std::ffi::c_uint {
        return luaL_error(
            L,
            b"no website data types specified\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    let mut domain: *const std::ffi::c_char = luaL_checklstring(
        L,
        2 as std::ffi::c_int,
        0 as *mut size_t,
    );
    let mut wdrt: *mut website_data_remove_task_t = g_slice_alloc0(
        ::core::mem::size_of::<website_data_remove_task_t>() as std::ffi::c_ulong,
    ) as *mut website_data_remove_task_t;
    (*wdrt).L = L;
    (*wdrt).domain = g_strdup_inline(domain);
    (*wdrt).data_types = data_types;
    let mut web_context: *mut WebKitWebContext = web_context_get();
    let mut data_manager: *mut WebKitWebsiteDataManager = webkit_web_context_get_website_data_manager(
        web_context,
    );
    webkit_website_data_manager_fetch(
        data_manager,
        data_types,
        0 as *mut GCancellable,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut WebKitWebsiteDataManager,
                    *mut GAsyncResult,
                    *mut website_data_remove_task_t,
                ) -> (),
            >,
            GAsyncReadyCallback,
        >(
            Some(
                luaH_luakit_website_data_remove_cont
                    as unsafe extern "C" fn(
                        *mut WebKitWebsiteDataManager,
                        *mut GAsyncResult,
                        *mut website_data_remove_task_t,
                    ) -> (),
            ),
        ),
        wdrt as gpointer,
    );
    return luaH_yield(L);
}
#[c2rust::src_loc = "678:1"]
unsafe extern "C" fn website_data_clear_finish(
    mut manager: *mut WebKitWebsiteDataManager,
    mut result: *mut GAsyncResult,
    mut L: *mut lua_State,
) {
    let mut __n1: gint64 = lua_status(L) as gint64;
    let mut __n2: gint64 = 1 as std::ffi::c_int as gint64;
    if !(__n1 == __n2) {
        g_assertion_message_cmpint(
            0 as *mut gchar,
            b"clib/luakit.c\0" as *const u8 as *const std::ffi::c_char,
            681 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 26],
                &[std::ffi::c_char; 26],
            >(b"website_data_clear_finish\0"))
                .as_ptr(),
            b"lua_status(L) == LUA_YIELD\0" as *const u8 as *const std::ffi::c_char,
            __n1 as guint64,
            b"==\0" as *const u8 as *const std::ffi::c_char,
            __n2 as guint64,
            'i' as i32 as std::ffi::c_char,
        );
    }
    let mut error: *mut GError = 0 as *mut GError;
    webkit_website_data_manager_clear_finish(manager, result, &mut error);
    if !error.is_null() {
        lua_pushnil(L);
        lua_pushstring(L, (*error).message);
        g_error_free(error);
    } else {
        lua_pushboolean(L, (0 as std::ffi::c_int == 0) as std::ffi::c_int);
    }
    luaH_resume(L, lua_gettop(L));
}
#[c2rust::src_loc = "695:1"]
unsafe extern "C" fn luaH_luakit_website_data_clear(mut L: *mut lua_State) -> gint {
    let mut data_types: WebKitWebsiteDataTypes = luaH_parse_website_data_types_table(
        L,
        1 as std::ffi::c_int,
    );
    if data_types as std::ffi::c_uint == 0 as std::ffi::c_int as std::ffi::c_uint {
        return luaL_error(
            L,
            b"no website data types specified\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    let mut timespan: GTimeSpan = luaL_optinteger(
        L,
        2 as std::ffi::c_int,
        0 as std::ffi::c_int as lua_Integer,
    );
    let mut web_context: *mut WebKitWebContext = web_context_get();
    let mut data_manager: *mut WebKitWebsiteDataManager = webkit_web_context_get_website_data_manager(
        web_context,
    );
    webkit_website_data_manager_clear(
        data_manager,
        data_types,
        timespan,
        0 as *mut GCancellable,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut WebKitWebsiteDataManager,
                    *mut GAsyncResult,
                    *mut lua_State,
                ) -> (),
            >,
            GAsyncReadyCallback,
        >(
            Some(
                website_data_clear_finish
                    as unsafe extern "C" fn(
                        *mut WebKitWebsiteDataManager,
                        *mut GAsyncResult,
                        *mut lua_State,
                    ) -> (),
            ),
        ),
        L as gpointer,
    );
    return luaH_yield(L);
}
#[c2rust::src_loc = "711:1"]
unsafe extern "C" fn luaH_luakit_website_data_index(mut L: *mut lua_State) -> gint {
    let mut prop: *const gchar = luaL_checklstring(
        L,
        2 as std::ffi::c_int,
        0 as *mut size_t,
    );
    let mut token: luakit_token_t = l_tokenize(prop);
    match token as std::ffi::c_uint {
        93 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_luakit_website_data_fetch
                        as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            luaH_yield_wrap_function(L);
            return 1 as std::ffi::c_int;
        }
        180 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_luakit_website_data_remove
                        as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            luaH_yield_wrap_function(L);
            return 1 as std::ffi::c_int;
        }
        26 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_luakit_website_data_clear
                        as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            luaH_yield_wrap_function(L);
            return 1 as std::ffi::c_int;
        }
        _ => return 0 as std::ffi::c_int,
    };
}
#[c2rust::src_loc = "733:1"]
unsafe extern "C" fn luaH_luakit_push_website_data_table(mut L: *mut lua_State) -> gint {
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_createtable(L, 0 as std::ffi::c_int, 2 as std::ffi::c_int);
    lua_pushlstring(
        L,
        b"__index\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 8]>() as std::ffi::c_ulong)
            .wrapping_div(
                ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
            )
            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
    );
    lua_pushvalue(L, 1 as std::ffi::c_int);
    lua_pushcclosure(
        L,
        Some(
            luaH_luakit_website_data_index
                as unsafe extern "C" fn(*mut lua_State) -> gint,
        ),
        1 as std::ffi::c_int,
    );
    lua_rawset(L, -(3 as std::ffi::c_int));
    lua_setmetatable(L, -(2 as std::ffi::c_int));
    return 1 as std::ffi::c_int;
}
#[c2rust::src_loc = "748:1"]
unsafe extern "C" fn luaH_string_wch_convert_case(
    mut L: *mut lua_State,
    mut key: *const std::ffi::c_char,
    mut upper: gboolean,
) -> gint {
    let mut kval: guint = gdk_keyval_from_name(key);
    if kval == 0xffffff as std::ffi::c_int as guint {
        _log(
            LOG_LEVEL_debug,
            b"clib/luakit.c\0" as *const u8 as *const std::ffi::c_char,
            b"unrecognized key symbol '%s'\0" as *const u8 as *const std::ffi::c_char,
            key,
        );
        lua_pushstring(L, key);
        return 1 as std::ffi::c_int;
    }
    let mut cased: guint = 0;
    gdk_keyval_convert_case(
        kval,
        if upper != 0 { 0 as *mut guint } else { &mut cased },
        if upper != 0 { &mut cased } else { 0 as *mut guint },
    );
    luaH_keystr_push(L, cased);
    return 1 as std::ffi::c_int;
}
#[c2rust::src_loc = "763:1"]
unsafe extern "C" fn luaH_luakit_wch_lower(mut L: *mut lua_State) -> gint {
    return luaH_string_wch_convert_case(
        L,
        luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t),
        0 as std::ffi::c_int,
    );
}
#[c2rust::src_loc = "769:1"]
unsafe extern "C" fn luaH_luakit_wch_upper(mut L: *mut lua_State) -> gint {
    return luaH_string_wch_convert_case(
        L,
        luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t),
        (0 as std::ffi::c_int == 0) as std::ffi::c_int,
    );
}
#[c2rust::src_loc = "775:1"]
unsafe extern "C" fn luaH_luakit_clear_favicon_database(
    mut UNUSED_L: *mut lua_State,
) -> gint {
    let mut ctx: *mut WebKitWebContext = web_context_get();
    let mut fdb: *mut WebKitFaviconDatabase = webkit_web_context_get_favicon_database(
        ctx,
    );
    webkit_favicon_database_clear(fdb);
    return 0 as std::ffi::c_int;
}
#[c2rust::src_loc = "784:1"]
unsafe extern "C" fn luaH_luakit_push_install_paths_table(
    mut L: *mut lua_State,
) -> gint {
    lua_createtable(L, 0 as std::ffi::c_int, 6 as std::ffi::c_int);
    lua_pushlstring(
        L,
        b"/usr/local/share/luakit\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 24]>() as std::ffi::c_ulong)
            .wrapping_div(
                ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
            )
            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
    );
    lua_setfield(
        L,
        -(2 as std::ffi::c_int),
        b"install_dir\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_pushlstring(
        L,
        b"/etc/xdg\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 9]>() as std::ffi::c_ulong)
            .wrapping_div(
                ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
            )
            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
    );
    lua_setfield(
        L,
        -(2 as std::ffi::c_int),
        b"config_dir\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_pushlstring(
        L,
        b"/usr/local/share/luakit/doc\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 28]>() as std::ffi::c_ulong)
            .wrapping_div(
                ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
            )
            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
    );
    lua_setfield(
        L,
        -(2 as std::ffi::c_int),
        b"doc_dir\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_pushlstring(
        L,
        b"/usr/local/share/man\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 21]>() as std::ffi::c_ulong)
            .wrapping_div(
                ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
            )
            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
    );
    lua_setfield(
        L,
        -(2 as std::ffi::c_int),
        b"man_dir\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_pushlstring(
        L,
        b"/usr/local/share/pixmaps\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 25]>() as std::ffi::c_ulong)
            .wrapping_div(
                ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
            )
            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
    );
    lua_setfield(
        L,
        -(2 as std::ffi::c_int),
        b"pixmap_dir\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_pushlstring(
        L,
        b"/usr/local/share/applications\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 30]>() as std::ffi::c_ulong)
            .wrapping_div(
                ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
            )
            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
    );
    lua_setfield(
        L,
        -(2 as std::ffi::c_int),
        b"app_dir\0" as *const u8 as *const std::ffi::c_char,
    );
    return 1 as std::ffi::c_int;
}
#[c2rust::src_loc = "808:1"]
unsafe extern "C" fn luaH_luakit_index(mut L: *mut lua_State) -> gint {
    if luaH_usemetatable(L, 1 as std::ffi::c_int, 2 as std::ffi::c_int) != 0 {
        return 1 as std::ffi::c_int;
    }
    let mut w: *mut widget_t = 0 as *mut widget_t;
    let mut prop: *const gchar = luaL_checklstring(
        L,
        2 as std::ffi::c_int,
        0 as *mut size_t,
    );
    let mut token: luakit_token_t = l_tokenize(prop);
    match token as std::ffi::c_uint {
        16 => {
            lua_pushstring(L, globalconf.cache_dir);
            return 1 as std::ffi::c_int;
        }
        32 => {
            lua_pushstring(L, globalconf.config_dir);
            return 1 as std::ffi::c_int;
        }
        42 => {
            lua_pushstring(L, globalconf.data_dir);
            return 1 as std::ffi::c_int;
        }
        91 => {
            lua_pushstring(L, globalconf.execpath);
            return 1 as std::ffi::c_int;
        }
        33 => {
            lua_pushstring(L, globalconf.confpath);
            return 1 as std::ffi::c_int;
        }
        184 => {
            lua_pushstring(L, resource_path_get());
            return 1 as std::ffi::c_int;
        }
        250 => {
            lua_pushboolean(
                L,
                (log_get_verbosity(
                    b"all\0" as *const u8 as *const std::ffi::c_char
                        as *mut std::ffi::c_char,
                ) as std::ffi::c_uint
                    >= LOG_LEVEL_verbose as std::ffi::c_int as std::ffi::c_uint)
                    as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        155 => {
            lua_pushboolean(L, globalconf.nounique);
            return 1 as std::ffi::c_int;
        }
        80 => {
            lua_pushboolean(
                L,
                webkit_web_context_get_spell_checking_enabled(web_context_get()),
            );
            return 1 as std::ffi::c_int;
        }
        172 => {
            lua_pushinteger(L, web_context_process_limit_get() as lua_Integer);
            return 1 as std::ffi::c_int;
        }
        156 => return luaH_luakit_push_options_table(L),
        259 => return luaH_luakit_push_website_data_table(L),
        256 => {
            lua_pushboolean(L, 1 as std::ffi::c_int);
            return 1 as std::ffi::c_int;
        }
        264 => {
            lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
            let mut i: guint = 0 as std::ffi::c_int as guint;
            while i < (*globalconf.windows).len {
                w = *((*globalconf.windows).pdata).offset(i as isize) as *mut widget_t;
                luaH_object_push(L, (*w).ref_0);
                lua_rawseti(
                    L,
                    -(2 as std::ffi::c_int),
                    i.wrapping_add(1 as std::ffi::c_int as guint) as std::ffi::c_int,
                );
                i = i.wrapping_add(1);
                i;
            }
            return 1 as std::ffi::c_int;
        }
        258 => {
            lua_pushfstring(
                L,
                b"%d.%d.%d\0" as *const u8 as *const std::ffi::c_char,
                2 as std::ffi::c_int,
                48 as std::ffi::c_int,
                3 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        257 => {
            lua_pushfstring(
                L,
                b"%d.%d\0" as *const u8 as *const std::ffi::c_char,
                2 as std::ffi::c_int,
                48 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        201 => return luaH_luakit_selection_table_push(L),
        125 => {
            _log(
                LOG_LEVEL_warn,
                b"clib/luakit.c\0" as *const u8 as *const std::ffi::c_char,
                b"luakit.install_path is deprecated: use luakit.install_paths.install_dir instead\0"
                    as *const u8 as *const std::ffi::c_char,
            );
            lua_pushlstring(
                L,
                b"/usr/local/share/luakit\0" as *const u8 as *const std::ffi::c_char,
                (::core::mem::size_of::<[std::ffi::c_char; 24]>() as std::ffi::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                    )
                    .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
            );
            return 1 as std::ffi::c_int;
        }
        126 => return luaH_luakit_push_install_paths_table(L),
        251 => {
            lua_pushlstring(
                L,
                b"64175ca2\0" as *const u8 as *const std::ffi::c_char,
                (::core::mem::size_of::<[std::ffi::c_char; 9]>() as std::ffi::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                    )
                    .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
            );
            return 1 as std::ffi::c_int;
        }
        51 => {
            lua_pushboolean(L, 0 as std::ffi::c_int);
            return 1 as std::ffi::c_int;
        }
        219 => {
            luaH_push_strv(
                L,
                webkit_web_context_get_spell_checking_languages(web_context_get()),
            );
            return 1 as std::ffi::c_int;
        }
        _ => {}
    }
    return 0 as std::ffi::c_int;
}
#[c2rust::src_loc = "897:1"]
unsafe extern "C" fn luaH_luakit_newindex(mut L: *mut lua_State) -> gint {
    if lua_isstring(L, 2 as std::ffi::c_int) == 0 {
        return 0 as std::ffi::c_int;
    }
    let mut token: luakit_token_t = l_tokenize(
        lua_tolstring(L, 2 as std::ffi::c_int, 0 as *mut size_t),
    );
    match token as std::ffi::c_uint {
        172 => {
            if web_context_process_limit_set(
                lua_tointeger(L, 3 as std::ffi::c_int) as guint,
            ) == 0
            {
                return luaL_error(
                    L,
                    b"Too late to set WebKit process limit\0" as *const u8
                        as *const std::ffi::c_char,
                );
            }
        }
        80 => {
            webkit_web_context_set_spell_checking_enabled(
                web_context_get(),
                luaH_checkboolean(L, 3 as std::ffi::c_int),
            );
        }
        219 => {
            let mut langs: *mut *const gchar = luaH_checkstrv(L, 3 as std::ffi::c_int);
            let mut ctx: *mut WebKitWebContext = web_context_get();
            webkit_web_context_set_spell_checking_languages(ctx, langs);
            let mut accepted: *const *const gchar = webkit_web_context_get_spell_checking_languages(
                ctx,
            );
            let mut lang: *mut *const gchar = langs;
            while !(*lang).is_null() {
                if g_strv_contains(accepted, *lang) == 0 {
                    _log(
                        LOG_LEVEL_warn,
                        b"clib/luakit.c\0" as *const u8 as *const std::ffi::c_char,
                        b"unrecognized language code '%s'\0" as *const u8
                            as *const std::ffi::c_char,
                        *lang,
                    );
                }
                lang = lang.offset(1);
                lang;
            }
            g_free(langs as gpointer);
        }
        184 => {
            resource_path_set(
                luaL_checklstring(L, 3 as std::ffi::c_int, 0 as *mut size_t),
            );
        }
        _ => return 0 as std::ffi::c_int,
    }
    return 0 as std::ffi::c_int;
}
#[c2rust::src_loc = "940:1"]
unsafe extern "C" fn luaH_luakit_quit(mut UNUSED_L: *mut lua_State) -> gint {
    if gtk_main_level() != 0 {
        gtk_main_quit();
    } else {
        exit(0 as std::ffi::c_int);
    }
    return 0 as std::ffi::c_int;
}
#[c2rust::src_loc = "953:1"]
unsafe extern "C" fn luaH_luakit_register_scheme(mut L: *mut lua_State) -> gint {
    let mut scheme: *const gchar = luaL_checklstring(
        L,
        1 as std::ffi::c_int,
        0 as *mut size_t,
    );
    if strcmp(
        scheme as *const std::ffi::c_char,
        b"\0" as *const u8 as *const std::ffi::c_char,
    ) == 0 as std::ffi::c_int
    {
        return luaL_error(
            L,
            b"scheme cannot be empty\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    if strcmp(
        scheme as *const std::ffi::c_char,
        b"http\0" as *const u8 as *const std::ffi::c_char,
    ) == 0 as std::ffi::c_int
        || strcmp(
            scheme as *const std::ffi::c_char,
            b"https\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
    {
        return luaL_error(
            L,
            b"scheme cannot be 'http' or 'https'\0" as *const u8
                as *const std::ffi::c_char,
        );
    }
    if g_regex_match_simple(
        b"^[a-z][a-z0-9\\+\\-\\.]*$\0" as *const u8 as *const std::ffi::c_char,
        scheme,
        G_REGEX_DEFAULT,
        G_REGEX_MATCH_DEFAULT,
    ) == 0
    {
        return luaL_error(
            L,
            b"scheme must match [a-z][a-z0-9\\+\\-\\.]*\0" as *const u8
                as *const std::ffi::c_char,
        );
    }
    webkit_web_context_register_uri_scheme(
        web_context_get(),
        scheme,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut WebKitURISchemeRequest, gpointer) -> ()>,
            WebKitURISchemeRequestCallback,
        >(
            Some(
                luakit_uri_scheme_request_cb
                    as unsafe extern "C" fn(*mut WebKitURISchemeRequest, gpointer) -> (),
            ),
        ),
        g_strdup_inline(scheme) as gpointer,
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
    return 0 as std::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "971:1"]
pub unsafe extern "C" fn luaH_luakit_allow_certificate(mut L: *mut lua_State) -> gint {
    let mut host: *const gchar = luaL_checklstring(
        L,
        1 as std::ffi::c_int,
        0 as *mut size_t,
    );
    let mut len: size_t = 0;
    let mut cert_pem: *const gchar = luaL_checklstring(
        L,
        2 as std::ffi::c_int,
        &mut len,
    );
    let mut err: *mut GError = 0 as *mut GError;
    let mut cert: *mut GTlsCertificate = g_tls_certificate_new_from_pem(
        cert_pem,
        len as gssize,
        &mut err,
    );
    if !err.is_null() {
        lua_pushnil(L);
        lua_pushstring(L, (*err).message);
        return 2 as std::ffi::c_int;
    }
    let mut ctx: *mut WebKitWebContext = web_context_get();
    webkit_web_context_allow_tls_certificate_for_host(ctx, cert, host);
    g_object_unref(
        g_type_check_instance_cast(
            cert as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject as gpointer,
    );
    lua_pushboolean(L, (0 as std::ffi::c_int == 0) as std::ffi::c_int);
    return 1 as std::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "995:1"]
pub unsafe extern "C" fn luaH_class_index_miss_property(
    mut L: *mut lua_State,
    mut UNUSED_obj: *mut lua_object_t,
) -> gint {
    signal_object_emit(
        L,
        luakit_class.signals,
        b"debug::index::miss\0" as *const u8 as *const std::ffi::c_char,
        2 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1002:1"]
pub unsafe extern "C" fn luaH_class_newindex_miss_property(
    mut L: *mut lua_State,
    mut UNUSED_obj: *mut lua_object_t,
) -> gint {
    signal_object_emit(
        L,
        luakit_class.signals,
        b"debug::newindex::miss\0" as *const u8 as *const std::ffi::c_char,
        3 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1013:1"]
pub unsafe extern "C" fn luakit_lib_setup(mut L: *mut lua_State) {
    static mut luakit_lib: [luaL_Reg; 21] = unsafe {
        [
            {
                let mut init = luaL_Reg {
                    name: b"add_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_luakit_class_add_signal
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"remove_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_luakit_class_remove_signal
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"emit_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_luakit_class_emit_signal
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"time\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_luakit_time as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"uri_encode\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_luakit_uri_encode
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"uri_decode\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_luakit_uri_decode
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"idle_add\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_luakit_idle_add
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"idle_remove\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_luakit_idle_remove
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__index\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_luakit_index as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__newindex\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_luakit_newindex
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"exec\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_luakit_exec as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"quit\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_luakit_quit as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"save_file\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_luakit_save_file
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"spawn\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_luakit_spawn as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"spawn_sync\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_luakit_spawn_sync
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"register_scheme\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_luakit_register_scheme
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"allow_certificate\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_luakit_allow_certificate
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"wch_lower\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_luakit_wch_lower
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"wch_upper\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_luakit_wch_upper
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"clear_favicon_database\0" as *const u8
                        as *const std::ffi::c_char,
                    func: Some(
                        luaH_luakit_clear_favicon_database
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: 0 as *const std::ffi::c_char,
                    func: None,
                };
                init
            },
        ]
    };
    luakit_class.signals = signal_new();
    luaH_openlib(
        L,
        b"luakit\0" as *const u8 as *const std::ffi::c_char,
        luakit_lib.as_ptr(),
        luakit_lib.as_ptr(),
    );
}
#[no_mangle]
#[c2rust::src_loc = "1042:1"]
pub unsafe extern "C" fn luakit_lib_get_luakit_class() -> *mut lua_class_t {
    return &mut luakit_class;
}
