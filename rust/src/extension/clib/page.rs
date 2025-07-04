use ::libc;
use ::c2rust_bitfields;
#[c2rust::header_src = "/usr/lib/clang/20/include/__stddef_ptrdiff_t.h:19"]
pub mod __stddef_ptrdiff_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type ptrdiff_t = std::ffi::c_long;
}
#[c2rust::header_src = "/usr/lib/clang/20/include/__stddef_size_t.h:19"]
pub mod __stddef_size_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type size_t = std::ffi::c_ulong;
}
#[c2rust::header_src = "/usr/lib/glib-2.0/include/glibconfig.h:19"]
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
    #[c2rust::src_loc = "110:1"]
    pub type gconstpointer = *const std::ffi::c_void;
    #[c2rust::src_loc = "114:1"]
    pub type GCompareDataFunc = Option::<
        unsafe extern "C" fn(gconstpointer, gconstpointer, gpointer) -> gint,
    >;
    #[c2rust::src_loc = "140:1"]
    pub type GDestroyNotify = Option::<unsafe extern "C" fn(gpointer) -> ()>;
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
    use super::gtypes_h::{gpointer, guint, gboolean};
    extern "C" {
        #[c2rust::src_loc = "188:1"]
        pub fn g_ptr_array_free(
            array: *mut GPtrArray,
            free_segment: gboolean,
        ) -> *mut gpointer;
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
#[c2rust::header_src = "/usr/include/glib-2.0/glib/ghash.h:19"]
pub mod ghash_h {
    #[c2rust::src_loc = "40:1"]
    pub type GHashTable = _GHashTable;
    extern "C" {
        #[c2rust::src_loc = "40:16"]
        pub type _GHashTable;
    }
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
    #[c2rust::src_loc = "249:1"]
    pub type GWeakNotify = Option::<unsafe extern "C" fn(gpointer, *mut GObject) -> ()>;
    use super::gtype_h::GTypeInstance;
    use super::gtypes_h::{guint, gpointer};
    use super::gdataset_h::GData;
    extern "C" {
        #[c2rust::src_loc = "514:1"]
        pub fn g_object_unref(object: gpointer);
        #[c2rust::src_loc = "516:1"]
        pub fn g_object_weak_ref(
            object: *mut GObject,
            notify: GWeakNotify,
            data: gpointer,
        );
    }
}
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/jsc/JSCValue.h:19"]
pub mod JSCValue_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "48:1"]
    pub struct _JSCValue {
        pub parent: GObject,
        pub priv_0: *mut JSCValuePrivate,
    }
    #[c2rust::src_loc = "48:1"]
    pub type JSCValuePrivate = _JSCValuePrivate;
    #[c2rust::src_loc = "48:1"]
    pub type JSCValue = _JSCValue;
    #[c2rust::src_loc = "51:1"]
    pub type JSCContext = _JSCContext;
    use super::gobject_h::GObject;
    use super::JSCContext_h::_JSCContext;
    use super::gtypes_h::{gboolean, guint};
    extern "C" {
        #[c2rust::src_loc = "48:1"]
        pub type _JSCValuePrivate;
        #[c2rust::src_loc = "74:1"]
        pub fn jsc_value_get_context(value: *mut JSCValue) -> *mut JSCContext;
        #[c2rust::src_loc = "240:1"]
        pub fn jsc_value_is_function(value: *mut JSCValue) -> gboolean;
        #[c2rust::src_loc = "248:1"]
        pub fn jsc_value_function_callv(
            value: *mut JSCValue,
            n_parameters: guint,
            parameters: *mut *mut JSCValue,
        ) -> *mut JSCValue;
    }
}
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/jsc/JSCContext.h:19"]
pub mod JSCContext_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "53:1"]
    pub struct _JSCContext {
        pub parent: GObject,
        pub priv_0: *mut JSCContextPrivate,
    }
    #[c2rust::src_loc = "53:1"]
    pub type JSCContextPrivate = _JSCContextPrivate;
    use super::gobject_h::GObject;
    use super::JSCValue_h::{JSCContext, JSCValue};
    use super::JSCException_h::JSCException;
    use super::glibconfig_h::gssize;
    use super::gtypes_h::guint;
    extern "C" {
        #[c2rust::src_loc = "53:1"]
        pub type _JSCContextPrivate;
        #[c2rust::src_loc = "82:1"]
        pub fn jsc_context_get_exception(context: *mut JSCContext) -> *mut JSCException;
        #[c2rust::src_loc = "129:1"]
        pub fn jsc_context_evaluate_with_source_uri(
            context: *mut JSCContext,
            code: *const std::ffi::c_char,
            length: gssize,
            uri: *const std::ffi::c_char,
            line_number: guint,
        ) -> *mut JSCValue;
    }
}
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/jsc/JSCException.h:19"]
pub mod JSCException_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "48:1"]
    pub struct _JSCException {
        pub parent: GObject,
        pub priv_0: *mut JSCExceptionPrivate,
    }
    #[c2rust::src_loc = "48:1"]
    pub type JSCExceptionPrivate = _JSCExceptionPrivate;
    #[c2rust::src_loc = "48:1"]
    pub type JSCException = _JSCException;
    use super::gobject_h::GObject;
    extern "C" {
        #[c2rust::src_loc = "48:1"]
        pub type _JSCExceptionPrivate;
        #[c2rust::src_loc = "101:1"]
        pub fn jsc_exception_to_string(
            exception: *mut JSCException,
        ) -> *mut std::ffi::c_char;
    }
}
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/webkit/WebKitScriptWorld.h:21"]
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
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/webkitdom/webkitdomdefines.h:21"]
pub mod webkitdomdefines_h {
    #[c2rust::src_loc = "289:1"]
    pub type WebKitDOMNode = _WebKitDOMNode;
    #[c2rust::src_loc = "301:1"]
    pub type WebKitDOMObject = _WebKitDOMObject;
    #[c2rust::src_loc = "82:1"]
    pub type WebKitDOMDocument = _WebKitDOMDocument;
    #[c2rust::src_loc = "91:1"]
    pub type WebKitDOMElement = _WebKitDOMElement;
    use super::WebKitDOMNode_h::_WebKitDOMNode;
    use super::WebKitDOMObject_h::_WebKitDOMObject;
    use super::WebKitDOMDocument_h::_WebKitDOMDocument;
    use super::WebKitDOMElement_h::_WebKitDOMElement;
}
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/webkitdom/WebKitDOMNode.h:21"]
pub mod WebKitDOMNode_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "41:8"]
    pub struct _WebKitDOMNode {
        pub parent_instance: WebKitDOMObject,
    }
    use super::webkitdomdefines_h::WebKitDOMObject;
}
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/webkitdom/WebKitDOMObject.h:21"]
pub mod WebKitDOMObject_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "39:8"]
    pub struct _WebKitDOMObject {
        pub parentInstance: GObject,
        pub coreObject: gpointer,
    }
    use super::gobject_h::GObject;
    use super::gtypes_h::gpointer;
}
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/webkitdom/WebKitDOMDocument.h:21"]
pub mod WebKitDOMDocument_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "40:8"]
    pub struct _WebKitDOMDocument {
        pub parent_instance: WebKitDOMNode,
    }
    use super::webkitdomdefines_h::WebKitDOMNode;
}
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/webkitdom/WebKitDOMElement.h:21"]
pub mod WebKitDOMElement_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "40:8"]
    pub struct _WebKitDOMElement {
        pub parent_instance: WebKitDOMNode,
    }
    use super::webkitdomdefines_h::WebKitDOMNode;
}
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/webkit/WebKitFrame.h:21"]
pub mod WebKitFrame_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "52:1"]
    pub struct _WebKitFrame {
        pub parent: GObject,
        pub priv_0: *mut WebKitFramePrivate,
    }
    #[c2rust::src_loc = "46:1"]
    pub type WebKitFramePrivate = _WebKitFramePrivate;
    #[c2rust::src_loc = "44:1"]
    pub type WebKitFrame = _WebKitFrame;
    use super::gobject_h::GObject;
    use super::WebKitScriptWorld_h::WebKitScriptWorld;
    use super::JSCValue_h::JSCContext;
    extern "C" {
        #[c2rust::src_loc = "46:16"]
        pub type _WebKitFramePrivate;
        #[c2rust::src_loc = "73:1"]
        pub fn webkit_frame_get_js_context_for_script_world(
            frame: *mut WebKitFrame,
            world: *mut WebKitScriptWorld,
        ) -> *mut JSCContext;
    }
}
#[c2rust::header_src = "/usr/include/libsoup-3.0/libsoup/soup-message-headers.h:21"]
pub mod soup_message_headers_h {
    #[c2rust::src_loc = "12:1"]
    pub type SoupMessageHeaders = _SoupMessageHeaders;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "77:9"]
    pub struct SoupMessageHeadersIter {
        pub dummy: [gpointer; 3],
    }
    use super::gtypes_h::{gpointer, gboolean};
    extern "C" {
        #[c2rust::src_loc = "12:16"]
        pub type _SoupMessageHeaders;
        #[c2rust::src_loc = "36:1"]
        pub fn soup_message_headers_replace(
            hdrs: *mut SoupMessageHeaders,
            name: *const std::ffi::c_char,
            value: *const std::ffi::c_char,
        );
        #[c2rust::src_loc = "41:1"]
        pub fn soup_message_headers_remove(
            hdrs: *mut SoupMessageHeaders,
            name: *const std::ffi::c_char,
        );
        #[c2rust::src_loc = "82:1"]
        pub fn soup_message_headers_iter_init(
            iter: *mut SoupMessageHeadersIter,
            hdrs: *mut SoupMessageHeaders,
        );
        #[c2rust::src_loc = "85:1"]
        pub fn soup_message_headers_iter_next(
            iter: *mut SoupMessageHeadersIter,
            name: *mut *const std::ffi::c_char,
            value: *mut *const std::ffi::c_char,
        ) -> gboolean;
    }
}
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/webkit/WebKitURIRequest.h:21"]
pub mod WebKitURIRequest_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "50:1"]
    pub struct _WebKitURIRequest {
        pub parent: GObject,
        pub priv_0: *mut WebKitURIRequestPrivate,
    }
    #[c2rust::src_loc = "50:1"]
    pub type WebKitURIRequestPrivate = _WebKitURIRequestPrivate;
    #[c2rust::src_loc = "50:1"]
    pub type WebKitURIRequest = _WebKitURIRequest;
    use super::gobject_h::GObject;
    use super::gtypes_h::gchar;
    use super::soup_message_headers_h::SoupMessageHeaders;
    extern "C" {
        #[c2rust::src_loc = "50:1"]
        pub type _WebKitURIRequestPrivate;
        #[c2rust::src_loc = "55:1"]
        pub fn webkit_uri_request_get_uri(
            request: *mut WebKitURIRequest,
        ) -> *const gchar;
        #[c2rust::src_loc = "58:1"]
        pub fn webkit_uri_request_set_uri(
            request: *mut WebKitURIRequest,
            uri: *const gchar,
        );
        #[c2rust::src_loc = "65:1"]
        pub fn webkit_uri_request_get_http_headers(
            request: *mut WebKitURIRequest,
        ) -> *mut SoupMessageHeaders;
    }
}
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/webkit/WebKitURIResponse.h:21"]
pub mod WebKitURIResponse_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "50:1"]
    pub struct _WebKitURIResponse {
        pub parent: GObject,
        pub priv_0: *mut WebKitURIResponsePrivate,
    }
    #[c2rust::src_loc = "50:1"]
    pub type WebKitURIResponsePrivate = _WebKitURIResponsePrivate;
    #[c2rust::src_loc = "50:1"]
    pub type WebKitURIResponse = _WebKitURIResponse;
    use super::gobject_h::GObject;
    extern "C" {
        #[c2rust::src_loc = "50:1"]
        pub type _WebKitURIResponsePrivate;
    }
}
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/webkit/WebKitWebPage.h:21"]
pub mod WebKitWebPage_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "53:1"]
    pub struct _WebKitWebPage {
        pub parent: GObject,
        pub priv_0: *mut WebKitWebPagePrivate,
    }
    #[c2rust::src_loc = "47:1"]
    pub type WebKitWebPagePrivate = _WebKitWebPagePrivate;
    #[c2rust::src_loc = "45:1"]
    pub type WebKitWebPage = _WebKitWebPage;
    use super::gobject_h::GObject;
    use super::gtype_h::GType;
    use super::webkitdomdefines_h::WebKitDOMDocument;
    use super::glibconfig_h::guint64;
    use super::gtypes_h::gchar;
    use super::WebKitFrame_h::WebKitFrame;
    extern "C" {
        #[c2rust::src_loc = "47:16"]
        pub type _WebKitWebPagePrivate;
        #[c2rust::src_loc = "53:1"]
        pub fn webkit_web_page_get_type() -> GType;
        #[c2rust::src_loc = "77:1"]
        pub fn webkit_web_page_get_dom_document(
            web_page: *mut WebKitWebPage,
        ) -> *mut WebKitDOMDocument;
        #[c2rust::src_loc = "80:1"]
        pub fn webkit_web_page_get_id(web_page: *mut WebKitWebPage) -> guint64;
        #[c2rust::src_loc = "83:1"]
        pub fn webkit_web_page_get_uri(web_page: *mut WebKitWebPage) -> *const gchar;
        #[c2rust::src_loc = "86:1"]
        pub fn webkit_web_page_get_main_frame(
            web_page: *mut WebKitWebPage,
        ) -> *mut WebKitFrame;
    }
}
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/webkit/WebKitWebExtension.h:21"]
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
    use super::glibconfig_h::guint64;
    use super::WebKitWebPage_h::WebKitWebPage;
    extern "C" {
        #[c2rust::src_loc = "43:16"]
        pub type _WebKitWebExtensionPrivate;
        #[c2rust::src_loc = "75:1"]
        pub fn webkit_web_extension_get_page(
            extension_0: *mut WebKitWebExtension,
            page_id: guint64,
        ) -> *mut WebKitWebPage;
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
    #[c2rust::src_loc = "30:9"]
    pub const LUA_MULTRET: std::ffi::c_int = -(1 as std::ffi::c_int);
    #[c2rust::src_loc = "38:9"]
    pub const LUA_GLOBALSINDEX: std::ffi::c_int = -(10002 as std::ffi::c_int);
    #[c2rust::src_loc = "75:9"]
    pub const LUA_TNIL: std::ffi::c_int = 0 as std::ffi::c_int;
    #[c2rust::src_loc = "76:9"]
    pub const LUA_TBOOLEAN: std::ffi::c_int = 1 as std::ffi::c_int;
    #[c2rust::src_loc = "80:9"]
    pub const LUA_TTABLE: std::ffi::c_int = 5 as std::ffi::c_int;
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
        #[c2rust::src_loc = "155:1"]
        pub fn lua_topointer(
            L: *mut lua_State,
            idx: std::ffi::c_int,
        ) -> *const std::ffi::c_void;
        #[c2rust::src_loc = "161:1"]
        pub fn lua_pushnil(L: *mut lua_State);
        #[c2rust::src_loc = "163:1"]
        pub fn lua_pushinteger(L: *mut lua_State, n: lua_Integer);
        #[c2rust::src_loc = "165:1"]
        pub fn lua_pushstring(L: *mut lua_State, s: *const std::ffi::c_char);
        #[c2rust::src_loc = "169:1"]
        pub fn lua_pushcclosure(
            L: *mut lua_State,
            fn_0: lua_CFunction,
            n: std::ffi::c_int,
        );
        #[c2rust::src_loc = "171:1"]
        pub fn lua_pushlightuserdata(L: *mut lua_State, p: *mut std::ffi::c_void);
        #[c2rust::src_loc = "180:1"]
        pub fn lua_rawget(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "182:1"]
        pub fn lua_createtable(
            L: *mut lua_State,
            narr: std::ffi::c_int,
            nrec: std::ffi::c_int,
        );
        #[c2rust::src_loc = "183:1"]
        pub fn lua_newuserdata(L: *mut lua_State, sz: size_t) -> *mut std::ffi::c_void;
        #[c2rust::src_loc = "193:1"]
        pub fn lua_rawset(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "195:1"]
        pub fn lua_setmetatable(
            L: *mut lua_State,
            objindex: std::ffi::c_int,
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "196:1"]
        pub fn lua_setfenv(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;
        #[c2rust::src_loc = "241:1"]
        pub fn lua_next(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;
        #[c2rust::src_loc = "243:1"]
        pub fn lua_concat(L: *mut lua_State, n: std::ffi::c_int);
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
    use super::lua_h::{lua_CFunction, lua_State, lua_Number};
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
        #[c2rust::src_loc = "38:1"]
        pub fn luaL_checknumber(
            L: *mut lua_State,
            numArg: std::ffi::c_int,
        ) -> lua_Number;
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
#[c2rust::header_src = "/home/daana/git/luakit/common/ipc.h:21"]
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
}
#[c2rust::header_src = "/home/daana/git/luakit/common/common.h:21"]
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
#[c2rust::header_src = "/home/daana/git/luakit/extension/extension.h:21"]
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
    extern "C" {
        #[c2rust::src_loc = "44:20"]
        pub static mut extension: extension_t;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/signal.h:22"]
pub mod signal_h {
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
        g_ptr_array_free(sigfuncs as *mut GPtrArray, TRUE);
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
            NULL as *mut std::ffi::c_void,
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
    use super::gmacros_h::{FALSE, TRUE};
    use super::__stddef_null_h::NULL;
    use super::gmem_h::g_free;
}
#[c2rust::header_src = "/home/daana/git/luakit/common/tokenize.h:22"]
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
#[c2rust::header_src = "/home/daana/git/luakit/common/luaclass.h:22"]
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
    use super::signal_h::signal_t;
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
        #[c2rust::src_loc = "76:1"]
        pub fn luaH_class_setup(
            _: *mut lua_State,
            _: *mut lua_class_t,
            _: *const gchar,
            _: lua_class_allocator_t,
            _: lua_class_propfunc_t,
            _: lua_class_propfunc_t,
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
#[c2rust::header_src = "/home/daana/git/luakit/extension/clib/page.h:22"]
pub mod page_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "30:16"]
    pub struct _page_t {
        pub signals: *mut signal_t,
        pub page: *mut WebKitWebPage,
        pub ref_0: gpointer,
    }
    #[c2rust::src_loc = "30:1"]
    pub type page_t = _page_t;
    use super::signal_h::signal_t;
    use super::WebKitWebPage_h::WebKitWebPage;
    use super::gtypes_h::gpointer;
}
#[c2rust::header_src = "/home/daana/git/luakit/extension/clib/dom_element.h:24"]
pub mod dom_element_h {
    #[c2rust::src_loc = "49:1"]
    pub type dom_element_t = _dom_element_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:16"]
    pub struct _dom_element_t {
        pub signals: *mut signal_t,
        pub dom_events: *mut signal_t,
        pub element: *mut WebKitDOMElement,
    }
    use super::signal_h::signal_t;
    use super::webkitdomdefines_h::WebKitDOMElement;
    use super::page_h::page_t;
    use super::JSCValue_h::JSCValue;
    use super::lua_h::lua_State;
    use super::gtypes_h::gint;
    extern "C" {
        #[c2rust::src_loc = "57:1"]
        pub fn dom_element_js_ref(
            page: *mut page_t,
            element: *mut dom_element_t,
        ) -> *mut JSCValue;
        #[c2rust::src_loc = "58:1"]
        pub fn luaH_to_dom_element(L: *mut lua_State, idx: gint) -> *mut dom_element_t;
    }
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
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:19"]
pub mod stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "687:13"]
        pub fn free(_: *mut std::ffi::c_void);
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
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtestutils.h:19"]
pub mod gtestutils_h {
    extern "C" {
        #[c2rust::src_loc = "282:1"]
        pub fn g_strcmp0(
            str1: *const std::ffi::c_char,
            str2: *const std::ffi::c_char,
        ) -> std::ffi::c_int;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/util.h:21"]
pub mod util_h {
    use super::lua_h::lua_State;
    use super::gtypes_h::gchar;
    extern "C" {
        #[c2rust::src_loc = "73:1"]
        pub fn luaH_callerinfo(_: *mut lua_State) -> *mut gchar;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/luaobject.h:22"]
pub mod luaobject_h {
    use super::lua_h::lua_State;
    use super::luaclass_h::lua_class_t;
    use super::gtypes_h::{gint, gchar};
    extern "C" {
        #[c2rust::src_loc = "38:1"]
        pub fn luaH_settype(L: *mut lua_State, lua_class: *mut lua_class_t) -> gint;
        #[c2rust::src_loc = "164:1"]
        pub fn luaH_object_emit_signal(
            L: *mut lua_State,
            oud: gint,
            name: *const gchar,
            nargs: gint,
            nret: gint,
        ) -> gint;
        #[c2rust::src_loc = "167:1"]
        pub fn luaH_object_add_signal_simple(L: *mut lua_State) -> gint;
        #[c2rust::src_loc = "168:1"]
        pub fn luaH_object_remove_signal_simple(L: *mut lua_State) -> gint;
        #[c2rust::src_loc = "169:1"]
        pub fn luaH_object_remove_signals_simple(L: *mut lua_State) -> gint;
        #[c2rust::src_loc = "170:1"]
        pub fn luaH_object_emit_signal_simple(L: *mut lua_State) -> gint;
        #[c2rust::src_loc = "203:1"]
        pub fn luaH_object_tostring(_: *mut lua_State) -> gint;
        #[c2rust::src_loc = "204:1"]
        pub fn luaH_object_gc(_: *mut lua_State) -> gint;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/extension/clib/dom_document.h:23"]
pub mod dom_document_h {
    use super::lua_h::lua_State;
    use super::webkitdomdefines_h::WebKitDOMDocument;
    use super::gtypes_h::gint;
    extern "C" {
        #[c2rust::src_loc = "36:1"]
        pub fn luaH_dom_document_from_webkit_dom_document(
            L: *mut lua_State,
            doc: *mut WebKitDOMDocument,
        ) -> gint;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/luauniq.h:27"]
pub mod luauniq_h {
    use super::lua_h::lua_State;
    use super::gtypes_h::{gchar, gpointer};
    extern "C" {
        #[c2rust::src_loc = "31:1"]
        pub fn luaH_uniq_setup(L: *mut lua_State, reg: *const gchar, mode: *const gchar);
        #[c2rust::src_loc = "33:1"]
        pub fn luaH_uniq_add_ptr(
            L: *mut lua_State,
            reg: *const gchar,
            key: gpointer,
            oud: std::ffi::c_int,
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "35:1"]
        pub fn luaH_uniq_get_ptr(
            L: *mut lua_State,
            reg: *const gchar,
            key: gpointer,
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "37:1"]
        pub fn luaH_uniq_del_ptr(L: *mut lua_State, reg: *const gchar, key: gpointer);
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/luajs.h:28"]
pub mod luajs_h {
    use super::lua_h::lua_State;
    use super::JSCValue_h::{JSCValue, JSCContext};
    extern "C" {
        #[c2rust::src_loc = "28:1"]
        pub fn luajs_pushvalue(
            L: *mut lua_State,
            value: *mut JSCValue,
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "29:1"]
        pub fn luajs_tovalue(
            L: *mut lua_State,
            idx: std::ffi::c_int,
            ctx: *mut JSCContext,
        ) -> *mut JSCValue;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/luah.h:29"]
pub mod luah_h {
    #[inline]
    #[c2rust::src_loc = "150:1"]
    pub unsafe extern "C" fn luaH_rawfield(
        mut L: *mut lua_State,
        mut idx: gint,
        mut field: *const gchar,
    ) -> gint {
        lua_pushstring(L, field);
        lua_rawget(L, idx);
        let mut type_0 = lua_type(L, -(1 as std::ffi::c_int));
        if type_0 == LUA_TNIL {
            lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        }
        return type_0;
    }
    use super::lua_h::{
        lua_State, lua_pushstring, lua_rawget, lua_type, LUA_TNIL, lua_settop,
    };
    use super::gtypes_h::{gint, gchar};
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gmacros.h:19"]
pub mod gmacros_h {
    #[c2rust::src_loc = "931:9"]
    pub const FALSE: std::ffi::c_int = 0 as std::ffi::c_int;
    #[c2rust::src_loc = "935:9"]
    pub const TRUE: std::ffi::c_int = (FALSE == 0) as std::ffi::c_int;
}
#[c2rust::header_src = "/usr/lib/clang/20/include/__stddef_null.h:19"]
pub mod __stddef_null_h {
    #[c2rust::src_loc = "26:9"]
    pub const NULL: std::ffi::c_int = 0 as std::ffi::c_int;
}
pub use self::__stddef_ptrdiff_t_h::ptrdiff_t;
pub use self::__stddef_size_t_h::size_t;
pub use self::glibconfig_h::{guint32, gint64, guint64, gssize, gsize};
pub use self::gtypes_h::{
    gchar, glong, gint, gboolean, gulong, guint, gfloat, gdouble, gpointer,
    gconstpointer, GCompareDataFunc, GDestroyNotify,
};
pub use self::garray_h::{_GPtrArray, GPtrArray, g_ptr_array_free};
pub use self::gquark_h::GQuark;
pub use self::gerror_h::{_GError, GError};
pub use self::gconvert_h::{GIConv, _GIConv};
pub use self::gdataset_h::{GData, _GData};
pub use self::glist_h::{_GList, GList};
pub use self::ghash_h::{GHashTable, _GHashTable};
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
pub use self::gtree_h::{GTree, _GTree, g_tree_new_full};
pub use self::gtype_h::{
    GType, GValue, _GTypeClass, GTypeClass, _GTypeInstance, GTypeInstance,
    g_type_check_instance_cast, g_type_check_instance_is_a,
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
pub use self::gobject_h::{
    _GObject, GObject, GWeakNotify, g_object_unref, g_object_weak_ref,
};
pub use self::JSCValue_h::{
    _JSCValue, JSCValuePrivate, JSCValue, JSCContext, _JSCValuePrivate,
    jsc_value_get_context, jsc_value_is_function, jsc_value_function_callv,
};
pub use self::JSCContext_h::{
    _JSCContext, JSCContextPrivate, _JSCContextPrivate, jsc_context_get_exception,
    jsc_context_evaluate_with_source_uri,
};
pub use self::JSCException_h::{
    _JSCException, JSCExceptionPrivate, JSCException, _JSCExceptionPrivate,
    jsc_exception_to_string,
};
pub use self::WebKitScriptWorld_h::{
    _WebKitScriptWorld, WebKitScriptWorldPrivate, WebKitScriptWorld,
    _WebKitScriptWorldPrivate,
};
pub use self::webkitdomdefines_h::{
    WebKitDOMNode, WebKitDOMObject, WebKitDOMDocument, WebKitDOMElement,
};
pub use self::WebKitDOMNode_h::_WebKitDOMNode;
pub use self::WebKitDOMObject_h::_WebKitDOMObject;
pub use self::WebKitDOMDocument_h::_WebKitDOMDocument;
pub use self::WebKitDOMElement_h::_WebKitDOMElement;
pub use self::WebKitFrame_h::{
    _WebKitFrame, WebKitFramePrivate, WebKitFrame, _WebKitFramePrivate,
    webkit_frame_get_js_context_for_script_world,
};
pub use self::soup_message_headers_h::{
    SoupMessageHeaders, SoupMessageHeadersIter, _SoupMessageHeaders,
    soup_message_headers_replace, soup_message_headers_remove,
    soup_message_headers_iter_init, soup_message_headers_iter_next,
};
pub use self::WebKitURIRequest_h::{
    _WebKitURIRequest, WebKitURIRequestPrivate, WebKitURIRequest,
    _WebKitURIRequestPrivate, webkit_uri_request_get_uri, webkit_uri_request_set_uri,
    webkit_uri_request_get_http_headers,
};
pub use self::WebKitURIResponse_h::{
    _WebKitURIResponse, WebKitURIResponsePrivate, WebKitURIResponse,
    _WebKitURIResponsePrivate,
};
pub use self::WebKitWebPage_h::{
    _WebKitWebPage, WebKitWebPagePrivate, WebKitWebPage, _WebKitWebPagePrivate,
    webkit_web_page_get_type, webkit_web_page_get_dom_document, webkit_web_page_get_id,
    webkit_web_page_get_uri, webkit_web_page_get_main_frame,
};
pub use self::WebKitWebExtension_h::{
    _WebKitWebExtension, WebKitWebExtensionPrivate, WebKitWebExtension,
    _WebKitWebExtensionPrivate, webkit_web_extension_get_page,
};
pub use self::lua_h::{
    lua_CFunction, lua_Number, lua_Integer, LUA_MULTRET, LUA_GLOBALSINDEX, LUA_TNIL,
    LUA_TBOOLEAN, LUA_TTABLE, lua_State, lua_gettop, lua_settop, lua_pushvalue,
    lua_insert, lua_isstring, lua_type, lua_typename, lua_toboolean, lua_tolstring,
    lua_objlen, lua_topointer, lua_pushnil, lua_pushinteger, lua_pushstring,
    lua_pushcclosure, lua_pushlightuserdata, lua_rawget, lua_createtable,
    lua_newuserdata, lua_rawset, lua_setmetatable, lua_setfenv, lua_next, lua_concat,
};
pub use self::lauxlib_h::{
    luaL_Reg, luaL_typerror, luaL_argerror, luaL_checklstring, luaL_checknumber,
};
pub use self::log_h::{
    log_level_t, LOG_LEVEL_debug, LOG_LEVEL_verbose, LOG_LEVEL_info, LOG_LEVEL_warn,
    LOG_LEVEL_error, LOG_LEVEL_fatal, _log,
};
pub use self::ipc_h::{
    ipc_type_t, IPC_TYPE_crash, IPC_TYPE_page_created, IPC_TYPE_log, IPC_TYPE_eval_js,
    IPC_TYPE_extension_init, IPC_TYPE_scroll, IPC_TYPE_lua_ipc,
    IPC_TYPE_lua_require_module, _ipc_header_t, ipc_header_t, _ipc_recv_state_t,
    ipc_recv_state_t, ipc_endpoint_status_t, IPC_ENDPOINT_FREED, IPC_ENDPOINT_CONNECTED,
    IPC_ENDPOINT_DISCONNECTED, _ipc_endpoint_t, ipc_endpoint_t,
};
pub use self::common_h::{_common_t, common_t, common};
pub use self::extension_h::{_extension_t, extension_t, extension};
pub use self::signal_h::{signal_t, signal_cmp, signal_array_destroy, signal_new};
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
pub use self::luaclass_h::{
    lua_class_property_array_t, lua_object_t, lua_class_allocator_t,
    lua_class_propfunc_t, lua_class_t, luaH_class_add_signal, luaH_class_remove_signal,
    luaH_class_emit_signal, luaH_class_setup, luaH_usemetatable, luaH_checkudata,
};
pub use self::page_h::{_page_t, page_t};
pub use self::dom_element_h::{
    dom_element_t, _dom_element_t, dom_element_js_ref, luaH_to_dom_element,
};
use self::string_h::memset;
use self::stdlib_h::free;
use self::gmem_h::g_free;
use self::gtestutils_h::g_strcmp0;
use self::util_h::luaH_callerinfo;
use self::luaobject_h::{
    luaH_settype, luaH_object_emit_signal, luaH_object_add_signal_simple,
    luaH_object_remove_signal_simple, luaH_object_remove_signals_simple,
    luaH_object_emit_signal_simple, luaH_object_tostring, luaH_object_gc,
};
use self::dom_document_h::luaH_dom_document_from_webkit_dom_document;
use self::luauniq_h::{
    luaH_uniq_setup, luaH_uniq_add_ptr, luaH_uniq_get_ptr, luaH_uniq_del_ptr,
};
use self::luajs_h::{luajs_pushvalue, luajs_tovalue};
pub use self::luah_h::luaH_rawfield;
pub use self::gmacros_h::{FALSE, TRUE};
pub use self::__stddef_null_h::NULL;
#[c2rust::src_loc = "31:9"]
pub const REG_KEY: [std::ffi::c_char; 26] = unsafe {
    *::core::mem::transmute::<
        &[u8; 26],
        &[std::ffi::c_char; 26],
    >(b"luakit.uniq.registry.page\0")
};
#[c2rust::src_loc = "33:20"]
static mut page_class: lua_class_t = lua_class_t {
    name: 0 as *const gchar,
    signals: 0 as *const signal_t as *mut signal_t,
    allocator: None,
    properties: 0 as *const lua_class_property_array_t
        as *mut lua_class_property_array_t,
    index_miss_property: None,
    newindex_miss_property: None,
};
#[inline]
#[c2rust::src_loc = "35:1"]
unsafe extern "C" fn page_new(mut L: *mut lua_State) -> *mut page_t {
    let mut p = lua_newuserdata(L, ::core::mem::size_of::<page_t>() as std::ffi::c_ulong)
        as *mut page_t;
    memset(
        p as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        (::core::mem::size_of::<page_t>() as std::ffi::c_ulong)
            .wrapping_mul(1 as std::ffi::c_int as std::ffi::c_ulong),
    );
    (*p).signals = signal_new();
    luaH_settype(L, &mut page_class);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_setmetatable(L, -(2 as std::ffi::c_int));
    lua_setfenv(L, -(2 as std::ffi::c_int));
    lua_pushvalue(L, -(1 as std::ffi::c_int));
    luaH_class_emit_signal(
        L,
        &mut page_class,
        b"new\0" as *const u8 as *const std::ffi::c_char,
        1 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    return p;
}
#[inline]
#[c2rust::src_loc = "35:1"]
unsafe extern "C" fn luaH_page_class_emit_signal(mut L: *mut lua_State) -> gint {
    return luaH_class_emit_signal(
        L,
        &mut page_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, NULL as *mut size_t),
        lua_gettop(L) - 1 as std::ffi::c_int,
        LUA_MULTRET,
    );
}
#[inline]
#[c2rust::src_loc = "35:1"]
unsafe extern "C" fn luaH_page_class_remove_signal(mut L: *mut lua_State) -> gint {
    luaH_class_remove_signal(
        L,
        &mut page_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, NULL as *mut size_t),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[inline]
#[c2rust::src_loc = "35:1"]
unsafe extern "C" fn luaH_page_class_add_signal(mut L: *mut lua_State) -> gint {
    luaH_class_add_signal(
        L,
        &mut page_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, NULL as *mut size_t),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[c2rust::src_loc = "37:1"]
unsafe extern "C" fn luaH_check_page(
    mut L: *mut lua_State,
    mut udx: gint,
) -> *mut page_t {
    let mut page = luaH_checkudata(L, udx, &mut page_class) as *mut page_t;
    if ((*page).page).is_null()
        || ({
            let mut __inst = (*page).page as *mut GTypeInstance;
            let mut __t = webkit_web_page_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = FALSE;
            } else if !((*__inst).g_class).is_null()
                && (*(*__inst).g_class).g_type == __t
            {
                __r = TRUE;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) == 0
    {
        luaL_argerror(
            L,
            udx,
            b"page no longer valid\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    return page;
}
#[c2rust::src_loc = "46:1"]
unsafe extern "C" fn send_request_cb(
    mut web_page: *mut WebKitWebPage,
    mut request: *mut WebKitURIRequest,
    mut UNUSED_redirected_response: *mut WebKitURIResponse,
    mut UNUSED_page: *mut page_t,
) -> gboolean {
    let mut L = common.L;
    let mut uri = webkit_uri_request_get_uri(request);
    let mut hdrs = webkit_uri_request_get_http_headers(request);
    let mut top = lua_gettop(L);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    if !hdrs.is_null() {
        let mut iter = SoupMessageHeadersIter {
            dummy: [0 as *mut std::ffi::c_void; 3],
        };
        soup_message_headers_iter_init(&mut iter, hdrs);
        let mut name = 0 as *const std::ffi::c_char;
        let mut value = 0 as *const std::ffi::c_char;
        while soup_message_headers_iter_next(&mut iter, &mut name, &mut value) != 0 {
            lua_pushstring(L, name);
            lua_pushstring(L, value);
            lua_rawset(L, -(3 as std::ffi::c_int));
        }
    }
    luaH_page_from_web_page(L, web_page);
    lua_pushstring(L, uri);
    lua_pushvalue(L, -(3 as std::ffi::c_int));
    let mut ret = luaH_object_emit_signal(
        L,
        -(3 as std::ffi::c_int),
        b"send-request\0" as *const u8 as *const std::ffi::c_char,
        2 as std::ffi::c_int,
        1 as std::ffi::c_int,
    );
    if ret != 0 {
        if lua_isstring(L, -(1 as std::ffi::c_int)) != 0 {
            webkit_uri_request_set_uri(
                request,
                lua_tolstring(L, -(1 as std::ffi::c_int), NULL as *mut size_t),
            );
        } else {
            if !(lua_type(L, -(1 as std::ffi::c_int)) == LUA_TBOOLEAN)
                || lua_toboolean(L, -(1 as std::ffi::c_int)) != 0
            {
                _log(
                    LOG_LEVEL_warn,
                    b"extension/clib/page.c\0" as *const u8 as *const std::ffi::c_char,
                    b"\x1B[34msend-request\x1B[0m handler returned %s, should be a string or false\0"
                        as *const u8 as *const std::ffi::c_char,
                    lua_typename(L, lua_type(L, -(1 as std::ffi::c_int))),
                );
            }
            lua_settop(L, top);
            return TRUE;
        }
        lua_settop(L, -ret - 1 as std::ffi::c_int);
    }
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    if !hdrs.is_null() {
        lua_pushnil(L);
        while lua_next(L, -(2 as std::ffi::c_int)) != 0 {
            soup_message_headers_replace(
                hdrs,
                luaL_checklstring(L, -(2 as std::ffi::c_int), NULL as *mut size_t),
                luaL_checklstring(L, -(1 as std::ffi::c_int), NULL as *mut size_t),
            );
            lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        }
        let mut iter_0 = SoupMessageHeadersIter {
            dummy: [0 as *mut std::ffi::c_void; 3],
        };
        soup_message_headers_iter_init(&mut iter_0, hdrs);
        let mut name_0 = 0 as *const std::ffi::c_char;
        let mut value_0 = 0 as *const std::ffi::c_char;
        while soup_message_headers_iter_next(&mut iter_0, &mut name_0, &mut value_0) != 0
        {
            lua_pushstring(L, name_0);
            lua_rawget(L, -(2 as std::ffi::c_int));
            if lua_type(L, -(1 as std::ffi::c_int)) == LUA_TNIL {
                soup_message_headers_remove(hdrs, name_0);
            }
            lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        }
    }
    lua_settop(L, top);
    return FALSE;
}
#[c2rust::src_loc = "117:1"]
unsafe extern "C" fn document_loaded_cb(
    mut web_page: *mut WebKitWebPage,
    mut UNUSED_page: *mut page_t,
) {
    let mut L = common.L;
    luaH_page_from_web_page(L, web_page);
    luaH_object_emit_signal(
        L,
        -(1 as std::ffi::c_int),
        b"document-loaded\0" as *const u8 as *const std::ffi::c_char,
        0 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
}
#[c2rust::src_loc = "126:1"]
unsafe extern "C" fn luaH_page_js_func(mut L: *mut lua_State) -> gint {
    let mut func = lua_topointer(L, LUA_GLOBALSINDEX - 1 as std::ffi::c_int)
        as *mut JSCValue;
    let mut page = luaH_check_page(L, LUA_GLOBALSINDEX - 2 as std::ffi::c_int);
    let mut ctx = jsc_value_get_context(func);
    let mut argc = lua_gettop(L);
    let mut args = (if argc > 0 as std::ffi::c_int {
        let mut fresh0 = ::std::vec::from_elem(
            0,
            (::core::mem::size_of::<*mut JSCValue>() as std::ffi::c_ulong)
                .wrapping_mul(argc as std::ffi::c_ulong) as usize,
        );
        fresh0.as_mut_ptr()
    } else {
        NULL as *mut std::ffi::c_void
    }) as *mut *mut JSCValue;
    let mut i = 0 as std::ffi::c_int;
    while i < argc {
        let mut elem = luaH_to_dom_element(L, i + 1 as std::ffi::c_int);
        if !elem.is_null() {
            let ref mut fresh1 = *args.offset(i as isize);
            *fresh1 = dom_element_js_ref(page, elem);
        } else {
            let ref mut fresh2 = *args.offset(i as isize);
            *fresh2 = luajs_tovalue(L, i + 1 as std::ffi::c_int, ctx);
        }
        i += 1;
        i;
    }
    let mut ret = jsc_value_function_callv(func, argc as guint, args);
    return luajs_pushvalue(L, ret);
}
#[c2rust::src_loc = "151:1"]
unsafe extern "C" fn luaH_page_eval_js(mut L: *mut lua_State) -> gint {
    let mut page = luaH_check_page(L, 1 as std::ffi::c_int);
    let mut script = luaL_checklstring(L, 2 as std::ffi::c_int, NULL as *mut size_t);
    let mut source = NULL as *const gchar;
    let mut top = lua_gettop(L);
    if top >= 3 as std::ffi::c_int && !(lua_type(L, 3 as std::ffi::c_int) == LUA_TNIL) {
        if !(lua_type(L, 3 as std::ffi::c_int) == LUA_TTABLE) {
            luaL_typerror(
                L,
                3 as std::ffi::c_int,
                b"table\0" as *const u8 as *const std::ffi::c_char,
            );
        }
        if luaH_rawfield(
            L,
            3 as std::ffi::c_int,
            b"source\0" as *const u8 as *const std::ffi::c_char,
        ) != 0
        {
            source = luaL_checklstring(L, -(1 as std::ffi::c_int), NULL as *mut size_t);
        }
        lua_settop(L, top);
    }
    source = if !source.is_null() { source } else { luaH_callerinfo(L) as *const gchar };
    let mut frame = webkit_web_page_get_main_frame((*page).page);
    let mut world = extension.script_world;
    let mut ctx = webkit_frame_get_js_context_for_script_world(frame, world);
    let mut res = jsc_context_evaluate_with_source_uri(
        ctx,
        script,
        -(1 as std::ffi::c_int) as gssize,
        source,
        1 as std::ffi::c_int as guint,
    );
    let mut exception = jsc_context_get_exception(ctx);
    g_object_unref(ctx as gpointer);
    if !exception.is_null() {
        g_object_unref(res as gpointer);
        let mut e = jsc_exception_to_string(exception);
        lua_pushnil(L);
        lua_pushstring(L, e);
        free(e as *mut std::ffi::c_void);
        return 2 as std::ffi::c_int;
    }
    if jsc_value_is_function(res) != 0 {
        lua_pushlightuserdata(L, res as *mut std::ffi::c_void);
        lua_pushvalue(L, 1 as std::ffi::c_int);
        lua_pushcclosure(
            L,
            Some(luaH_page_js_func as unsafe extern "C" fn(*mut lua_State) -> gint),
            2 as std::ffi::c_int,
        );
        return 1 as std::ffi::c_int;
    }
    let mut ret = luajs_pushvalue(L, res);
    g_object_unref(res as gpointer);
    if ret == 0 {
        lua_pushnil(L);
        lua_pushstring(
            L,
            b"unable to push the result onto the Lua stack\0" as *const u8
                as *const std::ffi::c_char,
        );
        return 2 as std::ffi::c_int;
    }
    return ret;
}
#[c2rust::src_loc = "202:1"]
unsafe extern "C" fn luaH_page_wrap_js(mut L: *mut lua_State) -> gint {
    luaL_checklstring(L, 2 as std::ffi::c_int, NULL as *mut size_t);
    if !(lua_type(L, 3 as std::ffi::c_int) == LUA_TNIL) {
        if !(lua_type(L, 3 as std::ffi::c_int) == LUA_TTABLE) {
            luaL_typerror(
                L,
                3 as std::ffi::c_int,
                b"table\0" as *const u8 as *const std::ffi::c_char,
            );
        }
    }
    lua_pushstring(L, b"(function(\0" as *const u8 as *const std::ffi::c_char);
    let mut i = 1 as std::ffi::c_int as size_t;
    while i <= lua_objlen(L, 3 as std::ffi::c_int) {
        lua_pushinteger(L, i as lua_Integer);
        lua_rawget(L, 3 as std::ffi::c_int);
        lua_pushstring(L, b",\0" as *const u8 as *const std::ffi::c_char);
        i = i.wrapping_add(1);
        i;
    }
    lua_pushstring(L, b"){\0" as *const u8 as *const std::ffi::c_char);
    lua_concat(L, lua_gettop(L) - 3 as std::ffi::c_int);
    lua_insert(L, 2 as std::ffi::c_int);
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    lua_pushstring(L, b"})\0" as *const u8 as *const std::ffi::c_char);
    lua_concat(L, 3 as std::ffi::c_int);
    return luaH_page_eval_js(L);
}
#[c2rust::src_loc = "230:1"]
unsafe extern "C" fn webkit_web_page_destroy_cb(
    mut page: *mut page_t,
    mut web_page: *mut GObject,
) {
    let mut L = common.L;
    luaH_uniq_get_ptr(L, REG_KEY.as_ptr(), web_page as gpointer);
    luaH_object_emit_signal(
        L,
        -(1 as std::ffi::c_int),
        b"destroy\0" as *const u8 as *const std::ffi::c_char,
        0 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    (*page).page = NULL as *mut WebKitWebPage;
    luaH_uniq_del_ptr(common.L, REG_KEY.as_ptr(), web_page as gpointer);
}
#[no_mangle]
#[c2rust::src_loc = "242:1"]
pub unsafe extern "C" fn luaH_page_from_web_page(
    mut L: *mut lua_State,
    mut web_page: *mut WebKitWebPage,
) -> gint {
    if web_page.is_null() {
        lua_pushnil(L);
        return 1 as std::ffi::c_int;
    }
    if luaH_uniq_get_ptr(L, REG_KEY.as_ptr(), web_page as gpointer) != 0 {
        return 1 as std::ffi::c_int;
    }
    let mut page = page_new(L);
    (*page).page = web_page;
    g_signal_connect_data(
        (*page).page as gpointer,
        b"send-request\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut WebKitWebPage,
                    *mut WebKitURIRequest,
                    *mut WebKitURIResponse,
                    *mut page_t,
                ) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                send_request_cb
                    as unsafe extern "C" fn(
                        *mut WebKitWebPage,
                        *mut WebKitURIRequest,
                        *mut WebKitURIResponse,
                        *mut page_t,
                    ) -> gboolean,
            ),
        ),
        page as gpointer,
        ::core::mem::transmute::<libc::intptr_t, GClosureNotify>(NULL as libc::intptr_t),
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        (*page).page as gpointer,
        b"document-loaded\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut WebKitWebPage, *mut page_t) -> ()>,
            GCallback,
        >(
            Some(
                document_loaded_cb
                    as unsafe extern "C" fn(*mut WebKitWebPage, *mut page_t) -> (),
            ),
        ),
        page as gpointer,
        ::core::mem::transmute::<libc::intptr_t, GClosureNotify>(NULL as libc::intptr_t),
        G_CONNECT_DEFAULT,
    );
    luaH_uniq_add_ptr(
        L,
        REG_KEY.as_ptr(),
        web_page as gpointer,
        -(1 as std::ffi::c_int),
    );
    g_object_weak_ref(
        g_type_check_instance_cast(
            web_page as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut page_t, *mut GObject) -> ()>,
            GWeakNotify,
        >(
            Some(
                webkit_web_page_destroy_cb
                    as unsafe extern "C" fn(*mut page_t, *mut GObject) -> (),
            ),
        ),
        page as gpointer,
    );
    return 1 as std::ffi::c_int;
}
#[c2rust::src_loc = "265:1"]
unsafe extern "C" fn luaH_page_new(mut L: *mut lua_State) -> std::ffi::c_int {
    let mut page_id = luaL_checknumber(L, -(1 as std::ffi::c_int)) as guint64;
    let mut page = webkit_web_extension_get_page(extension.ext, page_id);
    return luaH_page_from_web_page(L, page);
}
#[c2rust::src_loc = "273:1"]
unsafe extern "C" fn luaH_page_push_document(
    mut L: *mut lua_State,
    mut page: *mut page_t,
) -> gint {
    let mut doc = webkit_web_page_get_dom_document((*page).page);
    return luaH_dom_document_from_webkit_dom_document(L, doc);
}
#[c2rust::src_loc = "280:1"]
unsafe extern "C" fn luaH_page_index(mut L: *mut lua_State) -> gint {
    let mut prop = luaL_checklstring(L, 2 as std::ffi::c_int, NULL as *mut size_t);
    if luaH_usemetatable(L, 1 as std::ffi::c_int, 2 as std::ffi::c_int) != 0 {
        return 1 as std::ffi::c_int;
    }
    let mut page = luaH_check_page(L, 1 as std::ffi::c_int);
    let mut token = l_tokenize(prop);
    match token as std::ffi::c_uint {
        246 => {
            lua_pushstring(L, webkit_web_page_get_uri((*page).page));
            return 1 as std::ffi::c_int;
        }
        117 => {
            lua_pushinteger(L, webkit_web_page_get_id((*page).page) as lua_Integer);
            return 1 as std::ffi::c_int;
        }
        89 => {
            lua_pushcclosure(
                L,
                Some(luaH_page_eval_js as unsafe extern "C" fn(*mut lua_State) -> gint),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        266 => {
            lua_pushcclosure(
                L,
                Some(luaH_page_wrap_js as unsafe extern "C" fn(*mut lua_State) -> gint),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        52 => return luaH_page_push_document(L, page),
        _ => return 0 as std::ffi::c_int,
    };
}
#[no_mangle]
#[c2rust::src_loc = "303:1"]
pub unsafe extern "C" fn page_class_setup(mut L: *mut lua_State) {
    static mut page_methods: [luaL_Reg; 5] = unsafe {
        [
            {
                let mut init = luaL_Reg {
                    name: b"add_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_page_class_add_signal
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"remove_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_page_class_remove_signal
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"emit_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_page_class_emit_signal
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__call\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_page_new
                            as unsafe extern "C" fn(*mut lua_State) -> std::ffi::c_int,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: NULL as *const std::ffi::c_char,
                    func: ::core::mem::transmute::<
                        libc::intptr_t,
                        lua_CFunction,
                    >(NULL as libc::intptr_t),
                };
                init
            },
        ]
    };
    static mut page_meta: [luaL_Reg; 8] = unsafe {
        [
            {
                let mut init = luaL_Reg {
                    name: b"__tostring\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_object_tostring
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"add_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_object_add_signal_simple
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"remove_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_object_remove_signal_simple
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"remove_signals\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_object_remove_signals_simple
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"emit_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_object_emit_signal_simple
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__index\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_page_index as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__gc\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_object_gc as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: NULL as *const std::ffi::c_char,
                    func: ::core::mem::transmute::<
                        libc::intptr_t,
                        lua_CFunction,
                    >(NULL as libc::intptr_t),
                };
                init
            },
        ]
    };
    luaH_class_setup(
        L,
        &mut page_class,
        b"page\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut lua_State) -> *mut page_t>,
            lua_class_allocator_t,
        >(Some(page_new as unsafe extern "C" fn(*mut lua_State) -> *mut page_t)),
        ::core::mem::transmute::<
            libc::intptr_t,
            lua_class_propfunc_t,
        >(NULL as libc::intptr_t),
        ::core::mem::transmute::<
            libc::intptr_t,
            lua_class_propfunc_t,
        >(NULL as libc::intptr_t),
        page_methods.as_ptr(),
        page_meta.as_ptr(),
    );
    luaH_uniq_setup(L, REG_KEY.as_ptr(), b"\0" as *const u8 as *const std::ffi::c_char);
}
