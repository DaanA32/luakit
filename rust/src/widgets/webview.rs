#![feature(extern_types)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(path_statements)]
#![allow(static_mut_refs)]

use ::libc;

pub mod __stddef_ptrdiff_t_h {
    pub type ptrdiff_t = std::ffi::c_long;
}
pub mod __stddef_size_t_h {
    pub type size_t = std::ffi::c_ulong;
}
pub mod glibconfig_h {
    pub type gint8 = std::ffi::c_schar;
    pub type guint8 = std::ffi::c_uchar;
    pub type gint16 = std::ffi::c_short;
    pub type guint16 = std::ffi::c_ushort;
    pub type guint32 = std::ffi::c_uint;
    pub type gint64 = std::ffi::c_long;
    pub type guint64 = std::ffi::c_ulong;
    pub type gsize = std::ffi::c_ulong;
    pub const G_MAXUINT: std::ffi::c_uint = UINT_MAX;
    use super::internal::__INT_MAX__;
    use super::limits_h::UINT_MAX;
}
pub mod types_h {
    pub type __pid_t = std::ffi::c_int;
}
pub mod time_h {
    pub type pid_t = __pid_t;
    use super::types_h::__pid_t;
}
pub mod gtypes_h {
    pub type gchar = std::ffi::c_char;
    pub type gshort = std::ffi::c_short;
    pub type glong = std::ffi::c_long;
    pub type gint = std::ffi::c_int;
    pub type gboolean = gint;
    pub type guchar = std::ffi::c_uchar;
    pub type gulong = std::ffi::c_ulong;
    pub type guint = std::ffi::c_uint;
    pub type gfloat = std::ffi::c_float;
    pub type gdouble = std::ffi::c_double;
    pub type gpointer = *mut std::ffi::c_void;
    pub type gconstpointer = *const std::ffi::c_void;
    pub type GEqualFunc = Option::<
        unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean,
    >;
    pub type GDestroyNotify = Option::<unsafe extern "C" fn(gpointer) -> ()>;
    pub type GHashFunc = Option::<unsafe extern "C" fn(gconstpointer) -> guint>;
}
pub mod garray_h {
    pub type GBytes = _GBytes;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GPtrArray {
        pub pdata: *mut gpointer,
        pub len: guint,
    }
    pub type GPtrArray = _GPtrArray;
    use super::gtypes_h::{gpointer, guint, gboolean};
    unsafe extern "C" {
        pub type _GBytes;
        pub fn g_ptr_array_new() -> *mut GPtrArray;
        pub fn g_ptr_array_remove(array: *mut GPtrArray, data: gpointer) -> gboolean;
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
    use super::gtypes_h::{gint, gchar};
    unsafe extern "C" {
        pub fn g_error_new_literal(
            domain: GQuark,
            code: gint,
            message: *const gchar,
        ) -> *mut GError;
        pub fn g_error_free(error: *mut GError);
    }
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
    use super::gtypes_h::{gpointer, gconstpointer, guint};
    unsafe extern "C" {
        pub fn g_list_prepend(list: *mut GList, data: gpointer) -> *mut GList;
        pub fn g_list_remove_link(list: *mut GList, llink: *mut GList) -> *mut GList;
        pub fn g_list_find(list: *mut GList, data: gconstpointer) -> *mut GList;
        pub fn g_list_length(list: *mut GList) -> guint;
    }
}
pub mod ghash_h {
    pub type GHashTable = _GHashTable;
    use super::gtypes_h::{
        GHashFunc, GEqualFunc, gpointer, gboolean, gconstpointer, guint,
    };
    unsafe extern "C" {
        pub type _GHashTable;
        pub fn g_hash_table_new(
            hash_func: GHashFunc,
            key_equal_func: GEqualFunc,
        ) -> *mut GHashTable;
        pub fn g_hash_table_insert(
            hash_table: *mut GHashTable,
            key: gpointer,
            value: gpointer,
        ) -> gboolean;
        pub fn g_hash_table_lookup(
            hash_table: *mut GHashTable,
            key: gconstpointer,
        ) -> gpointer;
        pub fn g_str_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
        pub fn g_str_hash(v: gconstpointer) -> guint;
    }
}
pub mod gslist_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GSList {
        pub data: gpointer,
        pub next: *mut GSList,
    }
    pub type GSList = _GSList;
    use super::gtypes_h::{gpointer, GDestroyNotify};
    unsafe extern "C" {
        pub fn g_slist_free(list: *mut GSList);
        pub fn g_slist_free_full(list: *mut GSList, free_func: GDestroyNotify);
        pub fn g_slist_prepend(list: *mut GSList, data: gpointer) -> *mut GSList;
    }
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
        pub fn g_idle_remove_by_data(data: gpointer) -> gboolean;
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
    use c2rust_bitfields::BitfieldStruct;
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
    unsafe extern "C" {
        pub type _GTree;
    }
}
pub mod gtype_h {
    use super::gvalue_h::_GValue;
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
    pub const G_TYPE_FUNDAMENTAL_SHIFT: std::ffi::c_int = 2 as std::ffi::c_int;
    pub const G_TYPE_INT: std::ffi::c_int = (6 as std::ffi::c_int)
        << G_TYPE_FUNDAMENTAL_SHIFT;
    pub const G_TYPE_ENUM: std::ffi::c_int = (12 as std::ffi::c_int)
        << G_TYPE_FUNDAMENTAL_SHIFT;
    use super::glibconfig_h::gsize;
    use super::gtypes_h::gboolean;
    unsafe extern "C" {
        pub fn g_type_check_instance_cast(
            instance: *mut GTypeInstance,
            iface_type: GType,
        ) -> *mut GTypeInstance;
        pub fn g_type_check_instance_is_a(
            instance: *mut GTypeInstance,
            iface_type: GType,
        ) -> gboolean;
    }
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
    use super::gtype_h::{GType, GValue};
    use super::gtypes_h::{gint, guint, glong, gulong, gfloat, gdouble, gpointer};
    use super::glibconfig_h::{gint64, guint64};
    unsafe extern "C" {
        pub fn g_value_init(value: *mut GValue, g_type: GType) -> *mut GValue;
    }
}
pub mod gparam_h {
    pub type GParamFlags = std::ffi::c_int;
    pub const G_PARAM_DEPRECATED: GParamFlags = -2147483648;
    pub const G_PARAM_EXPLICIT_NOTIFY: GParamFlags = 1073741824;
    pub const G_PARAM_STATIC_BLURB: GParamFlags = 128;
    pub const G_PARAM_STATIC_NICK: GParamFlags = 64;
    pub const G_PARAM_PRIVATE: GParamFlags = 32;
    pub const G_PARAM_STATIC_NAME: GParamFlags = 32;
    pub const G_PARAM_LAX_VALIDATION: GParamFlags = 16;
    pub const G_PARAM_CONSTRUCT_ONLY: GParamFlags = 8;
    pub const G_PARAM_CONSTRUCT: GParamFlags = 4;
    pub const G_PARAM_READWRITE: GParamFlags = 3;
    pub const G_PARAM_WRITABLE: GParamFlags = 2;
    pub const G_PARAM_READABLE: GParamFlags = 1;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GParamSpec {
        pub g_type_instance: GTypeInstance,
        pub name: *const gchar,
        pub flags: GParamFlags,
        pub value_type: GType,
        pub owner_type: GType,
        pub _nick: *mut gchar,
        pub _blurb: *mut gchar,
        pub qdata: *mut GData,
        pub ref_count: guint,
        pub param_id: guint,
    }
    pub type GParamSpec = _GParamSpec;
    use super::gtype_h::{GTypeInstance, GType};
    use super::gtypes_h::{gchar, guint};
    use super::gdataset_h::GData;
}
pub mod gclosure_h {
    use c2rust_bitfields::BitfieldStruct;
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
    pub type GSignalMatchType = std::ffi::c_uint;
    pub const G_SIGNAL_MATCH_UNBLOCKED: GSignalMatchType = 32;
    pub const G_SIGNAL_MATCH_DATA: GSignalMatchType = 16;
    pub const G_SIGNAL_MATCH_FUNC: GSignalMatchType = 8;
    pub const G_SIGNAL_MATCH_CLOSURE: GSignalMatchType = 4;
    pub const G_SIGNAL_MATCH_DETAIL: GSignalMatchType = 2;
    pub const G_SIGNAL_MATCH_ID: GSignalMatchType = 1;
    use super::gtypes_h::{gpointer, gchar, gulong, guint};
    use super::gclosure_h::{GCallback, GClosureNotify, GClosure};
    use super::gquark_h::GQuark;
    unsafe extern "C" {
        pub fn g_signal_connect_data(
            instance: gpointer,
            detailed_signal: *const gchar,
            c_handler: GCallback,
            data: gpointer,
            destroy_data: GClosureNotify,
            connect_flags: GConnectFlags,
        ) -> gulong;
        pub fn g_signal_handlers_disconnect_matched(
            instance: gpointer,
            mask: GSignalMatchType,
            signal_id: guint,
            detail: GQuark,
            closure: *mut GClosure,
            func: gpointer,
            data: gpointer,
        ) -> guint;
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
    pub type GInitiallyUnowned = _GObject;
    use super::gtype_h::{GTypeInstance, GType, GValue};
    use super::gtypes_h::{guint, gchar, gpointer};
    use super::gdataset_h::GData;
    unsafe extern "C" {
        pub fn g_object_new(
            object_type: GType,
            first_property_name: *const gchar,
            _: ...
        ) -> gpointer;
        pub fn g_object_set(object: gpointer, first_property_name: *const gchar, _: ...);
        pub fn g_object_get(object: gpointer, first_property_name: *const gchar, _: ...);
        pub fn g_object_connect(
            object: gpointer,
            signal_spec: *const gchar,
            _: ...
        ) -> gpointer;
        pub fn g_object_set_property(
            object: *mut GObject,
            property_name: *const gchar,
            value: *const GValue,
        );
        pub fn g_object_freeze_notify(object: *mut GObject);
        pub fn g_object_thaw_notify(object: *mut GObject);
        pub fn g_object_ref(object: gpointer) -> gpointer;
        pub fn g_object_unref(object: gpointer);
        pub fn g_object_get_data(object: *mut GObject, key: *const gchar) -> gpointer;
        pub fn g_object_set_data(
            object: *mut GObject,
            key: *const gchar,
            data: gpointer,
        );
    }
}
pub mod gioenums_h {
    pub type GTlsCertificateFlags = std::ffi::c_uint;
    pub const G_TLS_CERTIFICATE_VALIDATE_ALL: GTlsCertificateFlags = 127;
    pub const G_TLS_CERTIFICATE_GENERIC_ERROR: GTlsCertificateFlags = 64;
    pub const G_TLS_CERTIFICATE_INSECURE: GTlsCertificateFlags = 32;
    pub const G_TLS_CERTIFICATE_REVOKED: GTlsCertificateFlags = 16;
    pub const G_TLS_CERTIFICATE_EXPIRED: GTlsCertificateFlags = 8;
    pub const G_TLS_CERTIFICATE_NOT_ACTIVATED: GTlsCertificateFlags = 4;
    pub const G_TLS_CERTIFICATE_BAD_IDENTITY: GTlsCertificateFlags = 2;
    pub const G_TLS_CERTIFICATE_UNKNOWN_CA: GTlsCertificateFlags = 1;
    pub const G_TLS_CERTIFICATE_NO_FLAGS: GTlsCertificateFlags = 0;
}
pub mod giotypes_h {
    pub type GAsyncResult = _GAsyncResult;
    pub type GCancellable = _GCancellable;
    pub type GApplication = _GApplication;
    pub type GFile = _GFile;
    pub type GTlsCertificate = _GTlsCertificate;
    pub type GAsyncReadyCallback = Option::<
        unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
    >;
    use super::gcancellable_h::_GCancellable;
    use super::gapplication_h::_GApplication;
    use super::gtlscertificate_h::_GTlsCertificate;
    use super::gobject_h::GObject;
    use super::gtypes_h::gpointer;
    unsafe extern "C" {
        pub type _GAsyncResult;
        pub type _GFile;
    }
}
pub mod gcancellable_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GCancellable {
        pub parent_instance: GObject,
        pub priv_0: *mut GCancellablePrivate,
    }
    pub type GCancellablePrivate = _GCancellablePrivate;
    use super::gobject_h::GObject;
    unsafe extern "C" {
        pub type _GCancellablePrivate;
    }
}
pub mod gapplication_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GApplication {
        pub parent_instance: GObject,
        pub priv_0: *mut GApplicationPrivate,
    }
    pub type GApplicationPrivate = _GApplicationPrivate;
    use super::gobject_h::GObject;
    unsafe extern "C" {
        pub type _GApplicationPrivate;
    }
}
pub mod gtlscertificate_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GTlsCertificate {
        pub parent_instance: GObject,
        pub priv_0: *mut GTlsCertificatePrivate,
    }
    pub type GTlsCertificatePrivate = _GTlsCertificatePrivate;
    use super::gobject_h::GObject;
    unsafe extern "C" {
        pub type _GTlsCertificatePrivate;
    }
}
pub mod WebKitCredential_h {
    pub type WebKitCredential = _WebKitCredential;
    pub type WebKitCredentialPersistence = std::ffi::c_uint;
    pub const WEBKIT_CREDENTIAL_PERSISTENCE_PERMANENT: WebKitCredentialPersistence = 2;
    pub const WEBKIT_CREDENTIAL_PERSISTENCE_FOR_SESSION: WebKitCredentialPersistence = 1;
    pub const WEBKIT_CREDENTIAL_PERSISTENCE_NONE: WebKitCredentialPersistence = 0;
    use super::gtypes_h::gchar;
    unsafe extern "C" {
        pub type _WebKitCredential;
        pub fn webkit_credential_new(
            username: *const gchar,
            password: *const gchar,
            persistence: WebKitCredentialPersistence,
        ) -> *mut WebKitCredential;
        pub fn webkit_credential_free(credential: *mut WebKitCredential);
    }
}
pub mod cairo_h {
    pub type cairo_t = _cairo;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _cairo_rectangle_int {
        pub x: std::ffi::c_int,
        pub y: std::ffi::c_int,
        pub width: std::ffi::c_int,
        pub height: std::ffi::c_int,
    }
    pub type cairo_rectangle_int_t = _cairo_rectangle_int;
    pub type cairo_region_t = _cairo_region;
    unsafe extern "C" {
        pub type _cairo;
        pub type _cairo_region;
    }
}
pub mod gdktypes_h {
    pub type GdkRectangle = cairo_rectangle_int_t;
    pub type GdkAtom = *mut _GdkAtom;
    pub type GdkDevice = _GdkDevice;
    pub type GdkDragContext = _GdkDragContext;
    pub type GdkWindow = _GdkWindow;
    use super::cairo_h::cairo_rectangle_int_t;
    unsafe extern "C" {
        pub type _GdkAtom;
        pub type _GdkDevice;
        pub type _GdkDragContext;
        pub type _GdkWindow;
    }
}
pub mod gdkevents_h {
    use c2rust_bitfields::BitfieldStruct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GdkEventAny {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
    }
    pub type GdkEventType = std::ffi::c_int;
    pub const GDK_EVENT_LAST: GdkEventType = 48;
    pub const GDK_PAD_GROUP_MODE: GdkEventType = 47;
    pub const GDK_PAD_STRIP: GdkEventType = 46;
    pub const GDK_PAD_RING: GdkEventType = 45;
    pub const GDK_PAD_BUTTON_RELEASE: GdkEventType = 44;
    pub const GDK_PAD_BUTTON_PRESS: GdkEventType = 43;
    pub const GDK_TOUCHPAD_PINCH: GdkEventType = 42;
    pub const GDK_TOUCHPAD_SWIPE: GdkEventType = 41;
    pub const GDK_TOUCH_CANCEL: GdkEventType = 40;
    pub const GDK_TOUCH_END: GdkEventType = 39;
    pub const GDK_TOUCH_UPDATE: GdkEventType = 38;
    pub const GDK_TOUCH_BEGIN: GdkEventType = 37;
    pub const GDK_DAMAGE: GdkEventType = 36;
    pub const GDK_GRAB_BROKEN: GdkEventType = 35;
    pub const GDK_OWNER_CHANGE: GdkEventType = 34;
    pub const GDK_SETTING: GdkEventType = 33;
    pub const GDK_WINDOW_STATE: GdkEventType = 32;
    pub const GDK_SCROLL: GdkEventType = 31;
    pub const GDK_VISIBILITY_NOTIFY: GdkEventType = 29;
    pub const GDK_CLIENT_EVENT: GdkEventType = 28;
    pub const GDK_DROP_FINISHED: GdkEventType = 27;
    pub const GDK_DROP_START: GdkEventType = 26;
    pub const GDK_DRAG_STATUS: GdkEventType = 25;
    pub const GDK_DRAG_MOTION: GdkEventType = 24;
    pub const GDK_DRAG_LEAVE: GdkEventType = 23;
    pub const GDK_DRAG_ENTER: GdkEventType = 22;
    pub const GDK_PROXIMITY_OUT: GdkEventType = 21;
    pub const GDK_PROXIMITY_IN: GdkEventType = 20;
    pub const GDK_SELECTION_NOTIFY: GdkEventType = 19;
    pub const GDK_SELECTION_REQUEST: GdkEventType = 18;
    pub const GDK_SELECTION_CLEAR: GdkEventType = 17;
    pub const GDK_PROPERTY_NOTIFY: GdkEventType = 16;
    pub const GDK_UNMAP: GdkEventType = 15;
    pub const GDK_MAP: GdkEventType = 14;
    pub const GDK_CONFIGURE: GdkEventType = 13;
    pub const GDK_FOCUS_CHANGE: GdkEventType = 12;
    pub const GDK_LEAVE_NOTIFY: GdkEventType = 11;
    pub const GDK_ENTER_NOTIFY: GdkEventType = 10;
    pub const GDK_KEY_RELEASE: GdkEventType = 9;
    pub const GDK_KEY_PRESS: GdkEventType = 8;
    pub const GDK_BUTTON_RELEASE: GdkEventType = 7;
    pub const GDK_TRIPLE_BUTTON_PRESS: GdkEventType = 6;
    pub const GDK_3BUTTON_PRESS: GdkEventType = 6;
    pub const GDK_DOUBLE_BUTTON_PRESS: GdkEventType = 5;
    pub const GDK_2BUTTON_PRESS: GdkEventType = 5;
    pub const GDK_BUTTON_PRESS: GdkEventType = 4;
    pub const GDK_MOTION_NOTIFY: GdkEventType = 3;
    pub const GDK_EXPOSE: GdkEventType = 2;
    pub const GDK_DESTROY: GdkEventType = 1;
    pub const GDK_DELETE: GdkEventType = 0;
    pub const GDK_NOTHING: GdkEventType = -1;
    pub type GdkEventAny = _GdkEventAny;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GdkEventExpose {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub area: GdkRectangle,
        pub region: *mut cairo_region_t,
        pub count: gint,
    }
    pub type GdkEventExpose = _GdkEventExpose;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GdkEventVisibility {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub state: GdkVisibilityState,
    }
    pub type GdkVisibilityState = std::ffi::c_uint;
    pub const GDK_VISIBILITY_FULLY_OBSCURED: GdkVisibilityState = 2;
    pub const GDK_VISIBILITY_PARTIAL: GdkVisibilityState = 1;
    pub const GDK_VISIBILITY_UNOBSCURED: GdkVisibilityState = 0;
    pub type GdkEventVisibility = _GdkEventVisibility;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GdkEventMotion {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub time: guint32,
        pub x: gdouble,
        pub y: gdouble,
        pub axes: *mut gdouble,
        pub state: guint,
        pub is_hint: gint16,
        pub device: *mut GdkDevice,
        pub x_root: gdouble,
        pub y_root: gdouble,
    }
    pub type GdkEventMotion = _GdkEventMotion;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GdkEventButton {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub time: guint32,
        pub x: gdouble,
        pub y: gdouble,
        pub axes: *mut gdouble,
        pub state: guint,
        pub button: guint,
        pub device: *mut GdkDevice,
        pub x_root: gdouble,
        pub y_root: gdouble,
    }
    pub type GdkEventButton = _GdkEventButton;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GdkEventTouch {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub time: guint32,
        pub x: gdouble,
        pub y: gdouble,
        pub axes: *mut gdouble,
        pub state: guint,
        pub sequence: *mut GdkEventSequence,
        pub emulating_pointer: gboolean,
        pub device: *mut GdkDevice,
        pub x_root: gdouble,
        pub y_root: gdouble,
    }
    pub type GdkEventSequence = _GdkEventSequence;
    pub type GdkEventTouch = _GdkEventTouch;
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    pub struct _GdkEventScroll {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub time: guint32,
        pub x: gdouble,
        pub y: gdouble,
        pub state: guint,
        pub direction: GdkScrollDirection,
        pub device: *mut GdkDevice,
        pub x_root: gdouble,
        pub y_root: gdouble,
        pub delta_x: gdouble,
        pub delta_y: gdouble,
        #[bitfield(name = "is_stop", ty = "guint", bits = "0..=0")]
        pub is_stop: [u8; 1],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 7],
    }
    pub type GdkScrollDirection = std::ffi::c_uint;
    pub const GDK_SCROLL_SMOOTH: GdkScrollDirection = 4;
    pub const GDK_SCROLL_RIGHT: GdkScrollDirection = 3;
    pub const GDK_SCROLL_LEFT: GdkScrollDirection = 2;
    pub const GDK_SCROLL_DOWN: GdkScrollDirection = 1;
    pub const GDK_SCROLL_UP: GdkScrollDirection = 0;
    pub type GdkEventScroll = _GdkEventScroll;
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    pub struct _GdkEventKey {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub time: guint32,
        pub state: guint,
        pub keyval: guint,
        pub length: gint,
        pub string: *mut gchar,
        pub hardware_keycode: guint16,
        pub group: guint8,
        #[bitfield(name = "is_modifier", ty = "guint", bits = "0..=0")]
        pub is_modifier: [u8; 1],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 4],
    }
    pub type GdkEventKey = _GdkEventKey;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GdkEventFocus {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub in_0: gint16,
    }
    pub type GdkEventFocus = _GdkEventFocus;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GdkEventCrossing {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub subwindow: *mut GdkWindow,
        pub time: guint32,
        pub x: gdouble,
        pub y: gdouble,
        pub x_root: gdouble,
        pub y_root: gdouble,
        pub mode: GdkCrossingMode,
        pub detail: GdkNotifyType,
        pub focus: gboolean,
        pub state: guint,
    }
    pub type GdkNotifyType = std::ffi::c_uint;
    pub const GDK_NOTIFY_UNKNOWN: GdkNotifyType = 5;
    pub const GDK_NOTIFY_NONLINEAR_VIRTUAL: GdkNotifyType = 4;
    pub const GDK_NOTIFY_NONLINEAR: GdkNotifyType = 3;
    pub const GDK_NOTIFY_INFERIOR: GdkNotifyType = 2;
    pub const GDK_NOTIFY_VIRTUAL: GdkNotifyType = 1;
    pub const GDK_NOTIFY_ANCESTOR: GdkNotifyType = 0;
    pub type GdkCrossingMode = std::ffi::c_uint;
    pub const GDK_CROSSING_DEVICE_SWITCH: GdkCrossingMode = 8;
    pub const GDK_CROSSING_TOUCH_END: GdkCrossingMode = 7;
    pub const GDK_CROSSING_TOUCH_BEGIN: GdkCrossingMode = 6;
    pub const GDK_CROSSING_STATE_CHANGED: GdkCrossingMode = 5;
    pub const GDK_CROSSING_GTK_UNGRAB: GdkCrossingMode = 4;
    pub const GDK_CROSSING_GTK_GRAB: GdkCrossingMode = 3;
    pub const GDK_CROSSING_UNGRAB: GdkCrossingMode = 2;
    pub const GDK_CROSSING_GRAB: GdkCrossingMode = 1;
    pub const GDK_CROSSING_NORMAL: GdkCrossingMode = 0;
    pub type GdkEventCrossing = _GdkEventCrossing;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GdkEventConfigure {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub x: gint,
        pub y: gint,
        pub width: gint,
        pub height: gint,
    }
    pub type GdkEventConfigure = _GdkEventConfigure;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GdkEventProperty {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub atom: GdkAtom,
        pub time: guint32,
        pub state: guint,
    }
    pub type GdkEventProperty = _GdkEventProperty;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GdkEventSelection {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub selection: GdkAtom,
        pub target: GdkAtom,
        pub property: GdkAtom,
        pub time: guint32,
        pub requestor: *mut GdkWindow,
    }
    pub type GdkEventSelection = _GdkEventSelection;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GdkEventOwnerChange {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub owner: *mut GdkWindow,
        pub reason: GdkOwnerChange,
        pub selection: GdkAtom,
        pub time: guint32,
        pub selection_time: guint32,
    }
    pub type GdkOwnerChange = std::ffi::c_uint;
    pub const GDK_OWNER_CHANGE_CLOSE: GdkOwnerChange = 2;
    pub const GDK_OWNER_CHANGE_DESTROY: GdkOwnerChange = 1;
    pub const GDK_OWNER_CHANGE_NEW_OWNER: GdkOwnerChange = 0;
    pub type GdkEventOwnerChange = _GdkEventOwnerChange;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GdkEventProximity {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub time: guint32,
        pub device: *mut GdkDevice,
    }
    pub type GdkEventProximity = _GdkEventProximity;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GdkEventDND {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub context: *mut GdkDragContext,
        pub time: guint32,
        pub x_root: gshort,
        pub y_root: gshort,
    }
    pub type GdkEventDND = _GdkEventDND;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GdkEventWindowState {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub changed_mask: GdkWindowState,
        pub new_window_state: GdkWindowState,
    }
    pub type GdkWindowState = std::ffi::c_uint;
    pub const GDK_WINDOW_STATE_LEFT_RESIZABLE: GdkWindowState = 65536;
    pub const GDK_WINDOW_STATE_LEFT_TILED: GdkWindowState = 32768;
    pub const GDK_WINDOW_STATE_BOTTOM_RESIZABLE: GdkWindowState = 16384;
    pub const GDK_WINDOW_STATE_BOTTOM_TILED: GdkWindowState = 8192;
    pub const GDK_WINDOW_STATE_RIGHT_RESIZABLE: GdkWindowState = 4096;
    pub const GDK_WINDOW_STATE_RIGHT_TILED: GdkWindowState = 2048;
    pub const GDK_WINDOW_STATE_TOP_RESIZABLE: GdkWindowState = 1024;
    pub const GDK_WINDOW_STATE_TOP_TILED: GdkWindowState = 512;
    pub const GDK_WINDOW_STATE_TILED: GdkWindowState = 256;
    pub const GDK_WINDOW_STATE_FOCUSED: GdkWindowState = 128;
    pub const GDK_WINDOW_STATE_BELOW: GdkWindowState = 64;
    pub const GDK_WINDOW_STATE_ABOVE: GdkWindowState = 32;
    pub const GDK_WINDOW_STATE_FULLSCREEN: GdkWindowState = 16;
    pub const GDK_WINDOW_STATE_STICKY: GdkWindowState = 8;
    pub const GDK_WINDOW_STATE_MAXIMIZED: GdkWindowState = 4;
    pub const GDK_WINDOW_STATE_ICONIFIED: GdkWindowState = 2;
    pub const GDK_WINDOW_STATE_WITHDRAWN: GdkWindowState = 1;
    pub type GdkEventWindowState = _GdkEventWindowState;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GdkEventSetting {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub action: GdkSettingAction,
        pub name: *mut std::ffi::c_char,
    }
    pub type GdkSettingAction = std::ffi::c_uint;
    pub const GDK_SETTING_ACTION_DELETED: GdkSettingAction = 2;
    pub const GDK_SETTING_ACTION_CHANGED: GdkSettingAction = 1;
    pub const GDK_SETTING_ACTION_NEW: GdkSettingAction = 0;
    pub type GdkEventSetting = _GdkEventSetting;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GdkEventGrabBroken {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub keyboard: gboolean,
        pub implicit: gboolean,
        pub grab_window: *mut GdkWindow,
    }
    pub type GdkEventGrabBroken = _GdkEventGrabBroken;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GdkEventTouchpadSwipe {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub phase: gint8,
        pub n_fingers: gint8,
        pub time: guint32,
        pub x: gdouble,
        pub y: gdouble,
        pub dx: gdouble,
        pub dy: gdouble,
        pub x_root: gdouble,
        pub y_root: gdouble,
        pub state: guint,
    }
    pub type GdkEventTouchpadSwipe = _GdkEventTouchpadSwipe;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GdkEventTouchpadPinch {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub phase: gint8,
        pub n_fingers: gint8,
        pub time: guint32,
        pub x: gdouble,
        pub y: gdouble,
        pub dx: gdouble,
        pub dy: gdouble,
        pub angle_delta: gdouble,
        pub scale: gdouble,
        pub x_root: gdouble,
        pub y_root: gdouble,
        pub state: guint,
    }
    pub type GdkEventTouchpadPinch = _GdkEventTouchpadPinch;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GdkEventPadButton {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub time: guint32,
        pub group: guint,
        pub button: guint,
        pub mode: guint,
    }
    pub type GdkEventPadButton = _GdkEventPadButton;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GdkEventPadAxis {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub time: guint32,
        pub group: guint,
        pub index: guint,
        pub mode: guint,
        pub value: gdouble,
    }
    pub type GdkEventPadAxis = _GdkEventPadAxis;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GdkEventPadGroupMode {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub time: guint32,
        pub group: guint,
        pub mode: guint,
    }
    pub type GdkEventPadGroupMode = _GdkEventPadGroupMode;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union _GdkEvent {
        pub type_0: GdkEventType,
        pub any: GdkEventAny,
        pub expose: GdkEventExpose,
        pub visibility: GdkEventVisibility,
        pub motion: GdkEventMotion,
        pub button: GdkEventButton,
        pub touch: GdkEventTouch,
        pub scroll: GdkEventScroll,
        pub key: GdkEventKey,
        pub crossing: GdkEventCrossing,
        pub focus_change: GdkEventFocus,
        pub configure: GdkEventConfigure,
        pub property: GdkEventProperty,
        pub selection: GdkEventSelection,
        pub owner_change: GdkEventOwnerChange,
        pub proximity: GdkEventProximity,
        pub dnd: GdkEventDND,
        pub window_state: GdkEventWindowState,
        pub setting: GdkEventSetting,
        pub grab_broken: GdkEventGrabBroken,
        pub touchpad_swipe: GdkEventTouchpadSwipe,
        pub touchpad_pinch: GdkEventTouchpadPinch,
        pub pad_button: GdkEventPadButton,
        pub pad_axis: GdkEventPadAxis,
        pub pad_group_mode: GdkEventPadGroupMode,
    }
    pub type GdkEvent = _GdkEvent;
    use super::gdktypes_h::{GdkWindow, GdkRectangle, GdkDevice, GdkAtom, GdkDragContext};
    use super::glibconfig_h::{gint8, guint32, gint16, guint16, guint8};
    use super::cairo_h::cairo_region_t;
    use super::gtypes_h::{gint, gdouble, guint, gboolean, gchar, gshort};
    unsafe extern "C" {
        pub type _GdkEventSequence;
        pub fn gdk_event_get_scroll_deltas(
            event: *const GdkEvent,
            delta_x: *mut gdouble,
            delta_y: *mut gdouble,
        ) -> gboolean;
    }
}
pub mod gtkenums_h {
    pub type GtkAlign = std::ffi::c_uint;
    pub const GTK_ALIGN_BASELINE: GtkAlign = 4;
    pub const GTK_ALIGN_CENTER: GtkAlign = 3;
    pub const GTK_ALIGN_END: GtkAlign = 2;
    pub const GTK_ALIGN_START: GtkAlign = 1;
    pub const GTK_ALIGN_FILL: GtkAlign = 0;
    pub type GtkIconSize = std::ffi::c_uint;
    pub const GTK_ICON_SIZE_DIALOG: GtkIconSize = 6;
    pub const GTK_ICON_SIZE_DND: GtkIconSize = 5;
    pub const GTK_ICON_SIZE_BUTTON: GtkIconSize = 4;
    pub const GTK_ICON_SIZE_LARGE_TOOLBAR: GtkIconSize = 3;
    pub const GTK_ICON_SIZE_SMALL_TOOLBAR: GtkIconSize = 2;
    pub const GTK_ICON_SIZE_MENU: GtkIconSize = 1;
    pub const GTK_ICON_SIZE_INVALID: GtkIconSize = 0;
    pub type GtkPositionType = std::ffi::c_uint;
    pub const GTK_POS_BOTTOM: GtkPositionType = 3;
    pub const GTK_POS_TOP: GtkPositionType = 2;
    pub const GTK_POS_RIGHT: GtkPositionType = 1;
    pub const GTK_POS_LEFT: GtkPositionType = 0;
}
pub mod gtkwidget_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GtkWidget {
        pub parent_instance: GInitiallyUnowned,
        pub priv_0: *mut GtkWidgetPrivate,
    }
    pub type GtkWidgetPrivate = _GtkWidgetPrivate;
    use super::gobject_h::GInitiallyUnowned;
    use super::gtype_h::GType;
    use super::gtktypes_h::GtkWidget;
    use super::gtypes_h::gboolean;
    use super::gtkenums_h::{GtkAlign, GTK_ALIGN_FILL};
    unsafe extern "C" {
        pub type _GtkWidgetPrivate;
        pub fn gtk_widget_get_type() -> GType;
        pub fn gtk_widget_destroy(widget: *mut GtkWidget);
        pub fn gtk_widget_show(widget: *mut GtkWidget);
        pub fn gtk_widget_show_all(widget: *mut GtkWidget);
        pub fn gtk_widget_set_hexpand(widget: *mut GtkWidget, expand: gboolean);
        pub fn gtk_widget_set_vexpand(widget: *mut GtkWidget, expand: gboolean);
        pub fn gtk_widget_set_halign(widget: *mut GtkWidget, align: GtkAlign);
        pub fn gtk_widget_set_valign(widget: *mut GtkWidget, align: GtkAlign);
    }
}
pub mod gtktypes_h {
    pub type GtkWidget = _GtkWidget;
    pub type GtkWindow = _GtkWindow;
    use super::gtkwidget_h::_GtkWidget;
    use super::gtkwindow_h::_GtkWindow;
}
pub mod gtkwindow_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GtkWindow {
        pub bin: GtkBin,
        pub priv_0: *mut GtkWindowPrivate,
    }
    pub type GtkWindowPrivate = _GtkWindowPrivate;
    use super::gtkbin_h::GtkBin;
    use super::gtype_h::GType;
    use super::gtktypes_h::GtkWindow;
    use super::gtypes_h::{gchar, gboolean};
    unsafe extern "C" {
        pub type _GtkWindowPrivate;
        pub fn gtk_window_get_type() -> GType;
        pub fn gtk_window_set_title(window: *mut GtkWindow, title: *const gchar);
        pub fn gtk_window_set_resizable(window: *mut GtkWindow, resizable: gboolean);
        pub fn gtk_window_set_icon_name(window: *mut GtkWindow, name: *const gchar);
    }
}
pub mod gtkbin_h {
    pub type GtkBin = _GtkBin;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GtkBin {
        pub container: GtkContainer,
        pub priv_0: *mut GtkBinPrivate,
    }
    pub type GtkBinPrivate = _GtkBinPrivate;
    use super::gtkcontainer_h::GtkContainer;
    use super::gtype_h::GType;
    use super::gtktypes_h::GtkWidget;
    unsafe extern "C" {
        pub type _GtkBinPrivate;
        pub fn gtk_bin_get_type() -> GType;
        pub fn gtk_bin_get_child(bin: *mut GtkBin) -> *mut GtkWidget;
    }
}
pub mod gtkcontainer_h {
    pub type GtkContainer = _GtkContainer;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GtkContainer {
        pub widget: GtkWidget,
        pub priv_0: *mut GtkContainerPrivate,
    }
    pub type GtkContainerPrivate = _GtkContainerPrivate;
    use super::gtktypes_h::GtkWidget;
    use super::gtype_h::GType;
    use super::gtypes_h::guint;
    unsafe extern "C" {
        pub type _GtkContainerPrivate;
        pub fn gtk_container_get_type() -> GType;
        pub fn gtk_container_set_border_width(
            container: *mut GtkContainer,
            border_width: guint,
        );
    }
}
pub mod gtkapplication_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GtkApplication {
        pub parent: GApplication,
        pub priv_0: *mut GtkApplicationPrivate,
    }
    pub type GtkApplicationPrivate = _GtkApplicationPrivate;
    pub type GtkApplication = _GtkApplication;
    use super::giotypes_h::GApplication;
    unsafe extern "C" {
        pub type _GtkApplicationPrivate;
    }
}
pub mod gtkdialog_h {
    pub type C2RustUnnamed_0 = std::ffi::c_int;
    pub const GTK_RESPONSE_HELP: C2RustUnnamed_0 = -11;
    pub const GTK_RESPONSE_APPLY: C2RustUnnamed_0 = -10;
    pub const GTK_RESPONSE_NO: C2RustUnnamed_0 = -9;
    pub const GTK_RESPONSE_YES: C2RustUnnamed_0 = -8;
    pub const GTK_RESPONSE_CLOSE: C2RustUnnamed_0 = -7;
    pub const GTK_RESPONSE_CANCEL: C2RustUnnamed_0 = -6;
    pub const GTK_RESPONSE_OK: C2RustUnnamed_0 = -5;
    pub const GTK_RESPONSE_DELETE_EVENT: C2RustUnnamed_0 = -4;
    pub const GTK_RESPONSE_ACCEPT: C2RustUnnamed_0 = -3;
    pub const GTK_RESPONSE_REJECT: C2RustUnnamed_0 = -2;
    pub const GTK_RESPONSE_NONE: C2RustUnnamed_0 = -1;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GtkDialog {
        pub window: GtkWindow,
        pub priv_0: *mut GtkDialogPrivate,
    }
    pub type GtkDialogPrivate = _GtkDialogPrivate;
    pub type GtkDialog = _GtkDialog;
    use super::gtktypes_h::{GtkWindow, GtkWidget};
    use super::gtype_h::GType;
    use super::gtypes_h::{gchar, gint};
    unsafe extern "C" {
        pub type _GtkDialogPrivate;
        pub fn gtk_dialog_get_type() -> GType;
        pub fn gtk_dialog_new() -> *mut GtkWidget;
        pub fn gtk_dialog_add_buttons(
            dialog: *mut GtkDialog,
            first_button_text: *const gchar,
            _: ...
        );
        pub fn gtk_dialog_set_default_response(
            dialog: *mut GtkDialog,
            response_id: gint,
        );
        pub fn gtk_dialog_get_content_area(dialog: *mut GtkDialog) -> *mut GtkWidget;
    }
}
pub mod gtkmisc_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GtkMisc {
        pub widget: GtkWidget,
        pub priv_0: *mut GtkMiscPrivate,
    }
    pub type GtkMiscPrivate = _GtkMiscPrivate;
    pub type GtkMisc = _GtkMisc;
    use super::gtktypes_h::GtkWidget;
    unsafe extern "C" {
        pub type _GtkMiscPrivate;
    }
}
pub mod gtklabel_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GtkLabel {
        pub misc: GtkMisc,
        pub priv_0: *mut GtkLabelPrivate,
    }
    pub type GtkLabelPrivate = _GtkLabelPrivate;
    pub type GtkLabel = _GtkLabel;
    use super::gtkmisc_h::GtkMisc;
    use super::gtype_h::GType;
    use super::gtypes_h::{gchar, gboolean};
    use super::gtktypes_h::GtkWidget;
    unsafe extern "C" {
        pub type _GtkLabelPrivate;
        pub fn gtk_label_get_type() -> GType;
        pub fn gtk_label_new(str: *const gchar) -> *mut GtkWidget;
        pub fn gtk_label_set_line_wrap(label: *mut GtkLabel, wrap: gboolean);
    }
}
pub mod gtkbox_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GtkBox {
        pub container: GtkContainer,
        pub priv_0: *mut GtkBoxPrivate,
    }
    pub type GtkBoxPrivate = _GtkBoxPrivate;
    pub type GtkBox = _GtkBox;
    use super::gtkcontainer_h::GtkContainer;
    use super::gtype_h::GType;
    use super::gtktypes_h::GtkWidget;
    use super::gtypes_h::{gboolean, guint};
    unsafe extern "C" {
        pub type _GtkBoxPrivate;
        pub fn gtk_box_get_type() -> GType;
        pub fn gtk_box_pack_start(
            box_0: *mut GtkBox,
            child: *mut GtkWidget,
            expand: gboolean,
            fill: gboolean,
            padding: guint,
        );
    }
}
pub mod gtkentry_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GtkEntry {
        pub parent_instance: GtkWidget,
        pub priv_0: *mut GtkEntryPrivate,
    }
    pub type GtkEntryPrivate = _GtkEntryPrivate;
    pub type GtkEntry = _GtkEntry;
    use super::gtktypes_h::GtkWidget;
    use super::gtype_h::GType;
    use super::gtypes_h::{gboolean, gchar};
    unsafe extern "C" {
        pub type _GtkEntryPrivate;
        pub fn gtk_entry_get_type() -> GType;
        pub fn gtk_entry_new() -> *mut GtkWidget;
        pub fn gtk_entry_set_visibility(entry: *mut GtkEntry, visible: gboolean);
        pub fn gtk_entry_set_activates_default(entry: *mut GtkEntry, setting: gboolean);
        pub fn gtk_entry_set_text(entry: *mut GtkEntry, text: *const gchar);
        pub fn gtk_entry_get_text(entry: *mut GtkEntry) -> *const gchar;
    }
}
pub mod gtkbutton_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GtkButton {
        pub bin: GtkBin,
        pub priv_0: *mut GtkButtonPrivate,
    }
    pub type GtkButtonPrivate = _GtkButtonPrivate;
    pub type GtkButton = _GtkButton;
    use super::gtkbin_h::GtkBin;
    unsafe extern "C" {
        pub type _GtkButtonPrivate;
    }
}
pub mod gtktogglebutton_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GtkToggleButton {
        pub button: GtkButton,
        pub priv_0: *mut GtkToggleButtonPrivate,
    }
    pub type GtkToggleButtonPrivate = _GtkToggleButtonPrivate;
    pub type GtkToggleButton = _GtkToggleButton;
    use super::gtkbutton_h::GtkButton;
    use super::gtype_h::GType;
    use super::gtypes_h::gboolean;
    unsafe extern "C" {
        pub type _GtkToggleButtonPrivate;
        pub fn gtk_toggle_button_get_type() -> GType;
        pub fn gtk_toggle_button_get_active(
            toggle_button: *mut GtkToggleButton,
        ) -> gboolean;
    }
}
pub mod gtkcssprovider_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GtkCssProvider {
        pub parent_instance: GObject,
        pub priv_0: *mut GtkCssProviderPrivate,
    }
    pub type GtkCssProviderPrivate = _GtkCssProviderPrivate;
    pub type GtkCssProvider = _GtkCssProvider;
    use super::gobject_h::GObject;
    unsafe extern "C" {
        pub type _GtkCssProviderPrivate;
    }
}
pub mod gtkgrid_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GtkGrid {
        pub container: GtkContainer,
        pub priv_0: *mut GtkGridPrivate,
    }
    pub type GtkGridPrivate = _GtkGridPrivate;
    pub type GtkGrid = _GtkGrid;
    use super::gtkcontainer_h::GtkContainer;
    use super::gtype_h::GType;
    use super::gtktypes_h::GtkWidget;
    use super::gtypes_h::{gint, gboolean, guint};
    use super::gtkenums_h::{GtkPositionType, GTK_POS_LEFT};
    unsafe extern "C" {
        pub type _GtkGridPrivate;
        pub fn gtk_grid_get_type() -> GType;
        pub fn gtk_grid_new() -> *mut GtkWidget;
        pub fn gtk_grid_attach(
            grid: *mut GtkGrid,
            child: *mut GtkWidget,
            left: gint,
            top: gint,
            width: gint,
            height: gint,
        );
        pub fn gtk_grid_attach_next_to(
            grid: *mut GtkGrid,
            child: *mut GtkWidget,
            sibling: *mut GtkWidget,
            side: GtkPositionType,
            width: gint,
            height: gint,
        );
        pub fn gtk_grid_set_row_homogeneous(grid: *mut GtkGrid, homogeneous: gboolean);
        pub fn gtk_grid_set_row_spacing(grid: *mut GtkGrid, spacing: guint);
        pub fn gtk_grid_set_column_homogeneous(
            grid: *mut GtkGrid,
            homogeneous: gboolean,
        );
        pub fn gtk_grid_set_column_spacing(grid: *mut GtkGrid, spacing: guint);
    }
}
pub mod gtkaction_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GtkAction {
        pub object: GObject,
        pub private_data: *mut GtkActionPrivate,
    }
    pub type GtkActionPrivate = _GtkActionPrivate;
    pub type GtkAction = _GtkAction;
    use super::gobject_h::GObject;
    use super::gtypes_h::gchar;
    unsafe extern "C" {
        pub type _GtkActionPrivate;
        pub fn gtk_action_new(
            name: *const gchar,
            label: *const gchar,
            tooltip: *const gchar,
            stock_id: *const gchar,
        ) -> *mut GtkAction;
        pub fn gtk_action_get_label(action: *mut GtkAction) -> *const gchar;
    }
}
pub mod WebKitAuthenticationRequest_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _WebKitAuthenticationRequest {
        pub parent: GObject,
        pub priv_0: *mut WebKitAuthenticationRequestPrivate,
    }
    pub type WebKitAuthenticationRequestPrivate = _WebKitAuthenticationRequestPrivate;
    pub type WebKitAuthenticationRequest = _WebKitAuthenticationRequest;
    use super::gobject_h::GObject;
    use super::gtypes_h::gchar;
    use super::WebKitCredential_h::WebKitCredential;
    unsafe extern "C" {
        pub type _WebKitAuthenticationRequestPrivate;
        pub fn webkit_authentication_request_get_host(
            request: *mut WebKitAuthenticationRequest,
        ) -> *const gchar;
        pub fn webkit_authentication_request_authenticate(
            request: *mut WebKitAuthenticationRequest,
            credential: *mut WebKitCredential,
        );
    }
}
pub mod WebKitBackForwardListItem_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _WebKitBackForwardListItem {
        pub parent: GInitiallyUnowned,
        pub priv_0: *mut WebKitBackForwardListItemPrivate,
    }
    pub type WebKitBackForwardListItemPrivate = _WebKitBackForwardListItemPrivate;
    pub type WebKitBackForwardListItem = _WebKitBackForwardListItem;
    use super::gobject_h::GInitiallyUnowned;
    use super::gtypes_h::gchar;
    unsafe extern "C" {
        pub type _WebKitBackForwardListItemPrivate;
        pub fn webkit_back_forward_list_item_get_uri(
            list_item: *mut WebKitBackForwardListItem,
        ) -> *const gchar;
        pub fn webkit_back_forward_list_item_get_title(
            list_item: *mut WebKitBackForwardListItem,
        ) -> *const gchar;
    }
}
pub mod WebKitBackForwardList_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _WebKitBackForwardList {
        pub parent: GObject,
        pub priv_0: *mut WebKitBackForwardListPrivate,
    }
    pub type WebKitBackForwardListPrivate = _WebKitBackForwardListPrivate;
    pub type WebKitBackForwardList = _WebKitBackForwardList;
    use super::gobject_h::GObject;
    use super::WebKitBackForwardListItem_h::WebKitBackForwardListItem;
    use super::gtypes_h::gint;
    use super::glist_h::GList;
    unsafe extern "C" {
        pub type _WebKitBackForwardListPrivate;
        pub fn webkit_back_forward_list_get_current_item(
            back_forward_list: *mut WebKitBackForwardList,
        ) -> *mut WebKitBackForwardListItem;
        pub fn webkit_back_forward_list_get_nth_item(
            back_forward_list: *mut WebKitBackForwardList,
            index: gint,
        ) -> *mut WebKitBackForwardListItem;
        pub fn webkit_back_forward_list_get_back_list(
            back_forward_list: *mut WebKitBackForwardList,
        ) -> *mut GList;
        pub fn webkit_back_forward_list_get_forward_list(
            back_forward_list: *mut WebKitBackForwardList,
        ) -> *mut GList;
    }
}
pub mod WebKitContextMenu_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _WebKitContextMenu {
        pub parent: GObject,
        pub priv_0: *mut WebKitContextMenuPrivate,
    }
    pub type WebKitContextMenuPrivate = _WebKitContextMenuPrivate;
    pub type WebKitContextMenu = _WebKitContextMenu;
    pub type WebKitContextMenuItem = _WebKitContextMenuItem;
    use super::gobject_h::GObject;
    use super::WebKitContextMenuItem_h::_WebKitContextMenuItem;
    use super::gtypes_h::guint;
    unsafe extern "C" {
        pub type _WebKitContextMenuPrivate;
        pub fn webkit_context_menu_new() -> *mut WebKitContextMenu;
        pub fn webkit_context_menu_append(
            menu: *mut WebKitContextMenu,
            item: *mut WebKitContextMenuItem,
        );
        pub fn webkit_context_menu_get_n_items(menu: *mut WebKitContextMenu) -> guint;
        pub fn webkit_context_menu_get_item_at_position(
            menu: *mut WebKitContextMenu,
            position: guint,
        ) -> *mut WebKitContextMenuItem;
        pub fn webkit_context_menu_remove_all(menu: *mut WebKitContextMenu);
    }
}
pub mod WebKitContextMenuItem_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _WebKitContextMenuItem {
        pub parent: GInitiallyUnowned,
        pub priv_0: *mut WebKitContextMenuItemPrivate,
    }
    pub type WebKitContextMenuItemPrivate = _WebKitContextMenuItemPrivate;
    use super::gobject_h::GInitiallyUnowned;
    use super::gtkaction_h::GtkAction;
    use super::WebKitContextMenu_h::{WebKitContextMenuItem, WebKitContextMenu};
    use super::WebKitContextMenuActions_h::{
        WebKitContextMenuAction, WEBKIT_CONTEXT_MENU_ACTION_NO_ACTION,
    };
    use super::gtypes_h::{gchar, gboolean};
    unsafe extern "C" {
        pub type _WebKitContextMenuItemPrivate;
        pub fn webkit_context_menu_item_new(
            action: *mut GtkAction,
        ) -> *mut WebKitContextMenuItem;
        pub fn webkit_context_menu_item_new_from_stock_action_with_label(
            action: WebKitContextMenuAction,
            label: *const gchar,
        ) -> *mut WebKitContextMenuItem;
        pub fn webkit_context_menu_item_new_with_submenu(
            label: *const gchar,
            submenu: *mut WebKitContextMenu,
        ) -> *mut WebKitContextMenuItem;
        pub fn webkit_context_menu_item_new_separator() -> *mut WebKitContextMenuItem;
        pub fn webkit_context_menu_item_get_action(
            item: *mut WebKitContextMenuItem,
        ) -> *mut GtkAction;
        pub fn webkit_context_menu_item_get_stock_action(
            item: *mut WebKitContextMenuItem,
        ) -> WebKitContextMenuAction;
        pub fn webkit_context_menu_item_is_separator(
            item: *mut WebKitContextMenuItem,
        ) -> gboolean;
        pub fn webkit_context_menu_item_get_submenu(
            item: *mut WebKitContextMenuItem,
        ) -> *mut WebKitContextMenu;
    }
}
pub mod WebKitContextMenuActions_h {
    pub type WebKitContextMenuAction = std::ffi::c_uint;
    pub const WEBKIT_CONTEXT_MENU_ACTION_CUSTOM: WebKitContextMenuAction = 10000;
    pub const WEBKIT_CONTEXT_MENU_ACTION_PASTE_AS_PLAIN_TEXT: WebKitContextMenuAction = 45;
    pub const WEBKIT_CONTEXT_MENU_ACTION_INSERT_EMOJI: WebKitContextMenuAction = 44;
    pub const WEBKIT_CONTEXT_MENU_ACTION_DOWNLOAD_AUDIO_TO_DISK: WebKitContextMenuAction = 43;
    pub const WEBKIT_CONTEXT_MENU_ACTION_DOWNLOAD_VIDEO_TO_DISK: WebKitContextMenuAction = 42;
    pub const WEBKIT_CONTEXT_MENU_ACTION_MEDIA_MUTE: WebKitContextMenuAction = 41;
    pub const WEBKIT_CONTEXT_MENU_ACTION_MEDIA_PAUSE: WebKitContextMenuAction = 40;
    pub const WEBKIT_CONTEXT_MENU_ACTION_MEDIA_PLAY: WebKitContextMenuAction = 39;
    pub const WEBKIT_CONTEXT_MENU_ACTION_ENTER_VIDEO_FULLSCREEN: WebKitContextMenuAction = 38;
    pub const WEBKIT_CONTEXT_MENU_ACTION_TOGGLE_MEDIA_LOOP: WebKitContextMenuAction = 37;
    pub const WEBKIT_CONTEXT_MENU_ACTION_TOGGLE_MEDIA_CONTROLS: WebKitContextMenuAction = 36;
    pub const WEBKIT_CONTEXT_MENU_ACTION_COPY_AUDIO_LINK_TO_CLIPBOARD: WebKitContextMenuAction = 35;
    pub const WEBKIT_CONTEXT_MENU_ACTION_COPY_VIDEO_LINK_TO_CLIPBOARD: WebKitContextMenuAction = 34;
    pub const WEBKIT_CONTEXT_MENU_ACTION_OPEN_AUDIO_IN_NEW_WINDOW: WebKitContextMenuAction = 33;
    pub const WEBKIT_CONTEXT_MENU_ACTION_OPEN_VIDEO_IN_NEW_WINDOW: WebKitContextMenuAction = 32;
    pub const WEBKIT_CONTEXT_MENU_ACTION_INSPECT_ELEMENT: WebKitContextMenuAction = 31;
    pub const WEBKIT_CONTEXT_MENU_ACTION_OUTLINE: WebKitContextMenuAction = 30;
    pub const WEBKIT_CONTEXT_MENU_ACTION_UNDERLINE: WebKitContextMenuAction = 29;
    pub const WEBKIT_CONTEXT_MENU_ACTION_ITALIC: WebKitContextMenuAction = 28;
    pub const WEBKIT_CONTEXT_MENU_ACTION_BOLD: WebKitContextMenuAction = 27;
    pub const WEBKIT_CONTEXT_MENU_ACTION_FONT_MENU: WebKitContextMenuAction = 26;
    pub const WEBKIT_CONTEXT_MENU_ACTION_IGNORE_GRAMMAR: WebKitContextMenuAction = 25;
    pub const WEBKIT_CONTEXT_MENU_ACTION_LEARN_SPELLING: WebKitContextMenuAction = 24;
    pub const WEBKIT_CONTEXT_MENU_ACTION_IGNORE_SPELLING: WebKitContextMenuAction = 23;
    pub const WEBKIT_CONTEXT_MENU_ACTION_NO_GUESSES_FOUND: WebKitContextMenuAction = 22;
    pub const WEBKIT_CONTEXT_MENU_ACTION_SPELLING_GUESS: WebKitContextMenuAction = 21;
    pub const WEBKIT_CONTEXT_MENU_ACTION_UNICODE: WebKitContextMenuAction = 20;
    pub const WEBKIT_CONTEXT_MENU_ACTION_INPUT_METHODS: WebKitContextMenuAction = 19;
    pub const WEBKIT_CONTEXT_MENU_ACTION_SELECT_ALL: WebKitContextMenuAction = 18;
    pub const WEBKIT_CONTEXT_MENU_ACTION_DELETE: WebKitContextMenuAction = 17;
    pub const WEBKIT_CONTEXT_MENU_ACTION_PASTE: WebKitContextMenuAction = 16;
    pub const WEBKIT_CONTEXT_MENU_ACTION_CUT: WebKitContextMenuAction = 15;
    pub const WEBKIT_CONTEXT_MENU_ACTION_COPY: WebKitContextMenuAction = 14;
    pub const WEBKIT_CONTEXT_MENU_ACTION_RELOAD: WebKitContextMenuAction = 13;
    pub const WEBKIT_CONTEXT_MENU_ACTION_STOP: WebKitContextMenuAction = 12;
    pub const WEBKIT_CONTEXT_MENU_ACTION_GO_FORWARD: WebKitContextMenuAction = 11;
    pub const WEBKIT_CONTEXT_MENU_ACTION_GO_BACK: WebKitContextMenuAction = 10;
    pub const WEBKIT_CONTEXT_MENU_ACTION_OPEN_FRAME_IN_NEW_WINDOW: WebKitContextMenuAction = 9;
    pub const WEBKIT_CONTEXT_MENU_ACTION_COPY_IMAGE_URL_TO_CLIPBOARD: WebKitContextMenuAction = 8;
    pub const WEBKIT_CONTEXT_MENU_ACTION_COPY_IMAGE_TO_CLIPBOARD: WebKitContextMenuAction = 7;
    pub const WEBKIT_CONTEXT_MENU_ACTION_DOWNLOAD_IMAGE_TO_DISK: WebKitContextMenuAction = 6;
    pub const WEBKIT_CONTEXT_MENU_ACTION_OPEN_IMAGE_IN_NEW_WINDOW: WebKitContextMenuAction = 5;
    pub const WEBKIT_CONTEXT_MENU_ACTION_COPY_LINK_TO_CLIPBOARD: WebKitContextMenuAction = 4;
    pub const WEBKIT_CONTEXT_MENU_ACTION_DOWNLOAD_LINK_TO_DISK: WebKitContextMenuAction = 3;
    pub const WEBKIT_CONTEXT_MENU_ACTION_OPEN_LINK_IN_NEW_WINDOW: WebKitContextMenuAction = 2;
    pub const WEBKIT_CONTEXT_MENU_ACTION_OPEN_LINK: WebKitContextMenuAction = 1;
    pub const WEBKIT_CONTEXT_MENU_ACTION_NO_ACTION: WebKitContextMenuAction = 0;
}
pub mod WebKitURIRequest_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _WebKitURIRequest {
        pub parent: GObject,
        pub priv_0: *mut WebKitURIRequestPrivate,
    }
    pub type WebKitURIRequestPrivate = _WebKitURIRequestPrivate;
    pub type WebKitURIRequest = _WebKitURIRequest;
    use super::gobject_h::GObject;
    use super::gtypes_h::gchar;
    unsafe extern "C" {
        pub type _WebKitURIRequestPrivate;
        pub fn webkit_uri_request_get_uri(
            request: *mut WebKitURIRequest,
        ) -> *const gchar;
    }
}
pub mod WebKitURIResponse_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _WebKitURIResponse {
        pub parent: GObject,
        pub priv_0: *mut WebKitURIResponsePrivate,
    }
    pub type WebKitURIResponsePrivate = _WebKitURIResponsePrivate;
    pub type WebKitURIResponse = _WebKitURIResponse;
    use super::gobject_h::GObject;
    use super::gtypes_h::{gchar, guint};
    unsafe extern "C" {
        pub type _WebKitURIResponsePrivate;
        pub fn webkit_uri_response_get_uri(
            response: *mut WebKitURIResponse,
        ) -> *const gchar;
        pub fn webkit_uri_response_get_status_code(
            response: *mut WebKitURIResponse,
        ) -> guint;
        pub fn webkit_uri_response_get_mime_type(
            response: *mut WebKitURIResponse,
        ) -> *const gchar;
    }
}
pub mod WebKitDownload_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _WebKitDownload {
        pub parent: GObject,
        pub priv_0: *mut WebKitDownloadPrivate,
    }
    pub type WebKitDownloadPrivate = _WebKitDownloadPrivate;
    pub type WebKitDownload = _WebKitDownload;
    pub type WebKitWebView = _WebKitWebView;
    use super::gobject_h::GObject;
    use super::WebKitWebView_h::_WebKitWebView;
    unsafe extern "C" {
        pub type _WebKitDownloadPrivate;
        pub fn webkit_download_get_web_view(
            download: *mut WebKitDownload,
        ) -> *mut WebKitWebView;
    }
}
pub mod WebKitWebView_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _WebKitWebView {
        pub parent: WebKitWebViewBase,
        pub priv_0: *mut WebKitWebViewPrivate,
    }
    pub type WebKitWebViewPrivate = _WebKitWebViewPrivate;
    pub type WebKitPolicyDecisionType = std::ffi::c_uint;
    pub const WEBKIT_POLICY_DECISION_TYPE_RESPONSE: WebKitPolicyDecisionType = 2;
    pub const WEBKIT_POLICY_DECISION_TYPE_NEW_WINDOW_ACTION: WebKitPolicyDecisionType = 1;
    pub const WEBKIT_POLICY_DECISION_TYPE_NAVIGATION_ACTION: WebKitPolicyDecisionType = 0;
    pub type WebKitLoadEvent = std::ffi::c_uint;
    pub const WEBKIT_LOAD_FINISHED: WebKitLoadEvent = 3;
    pub const WEBKIT_LOAD_COMMITTED: WebKitLoadEvent = 2;
    pub const WEBKIT_LOAD_REDIRECTED: WebKitLoadEvent = 1;
    pub const WEBKIT_LOAD_STARTED: WebKitLoadEvent = 0;
    pub type WebKitSaveMode = std::ffi::c_uint;
    pub const WEBKIT_SAVE_MODE_MHTML: WebKitSaveMode = 0;
    // pub const WEBKIT_TYPE_WEB_VIEW: GType = webkit_web_view_get_type();
    use super::WebKitWebViewBase_h::WebKitWebViewBase;
    use super::gtype_h::GType;
    use super::WebKitDownload_h::WebKitWebView;
    use super::gtypes_h::{gchar, gboolean, gpointer};
    use super::glibconfig_h::guint64;
    use super::WebKitBackForwardList_h::WebKitBackForwardList;
    use super::WebKitBackForwardListItem_h::WebKitBackForwardListItem;
    use super::WebKitSettings_h::WebKitSettings;
    use super::WebKitFindController_h::WebKitFindController;
    use super::WebKitWebResource_h::WebKitWebResource;
    use super::WebKitWebInspector_h::WebKitWebInspector;
    use super::giotypes_h::{
        GFile, GCancellable, GAsyncReadyCallback, GAsyncResult, GTlsCertificate,
    };
    use super::gerror_h::GError;
    use super::gioenums_h::GTlsCertificateFlags;
    use super::WebKitWebViewSessionState_h::WebKitWebViewSessionState;
    unsafe extern "C" {
        pub type _WebKitWebViewPrivate;
        pub fn webkit_web_view_get_type() -> GType;
        pub fn webkit_web_view_load_uri(web_view: *mut WebKitWebView, uri: *const gchar);
        pub fn webkit_web_view_load_alternate_html(
            web_view: *mut WebKitWebView,
            content: *const gchar,
            content_uri: *const gchar,
            base_uri: *const gchar,
        );
        pub fn webkit_web_view_stop_loading(web_view: *mut WebKitWebView);
        pub fn webkit_web_view_get_page_id(web_view: *mut WebKitWebView) -> guint64;
        pub fn webkit_web_view_reload(web_view: *mut WebKitWebView);
        pub fn webkit_web_view_reload_bypass_cache(web_view: *mut WebKitWebView);
        pub fn webkit_web_view_can_go_back(web_view: *mut WebKitWebView) -> gboolean;
        pub fn webkit_web_view_can_go_forward(web_view: *mut WebKitWebView) -> gboolean;
        pub fn webkit_web_view_get_back_forward_list(
            web_view: *mut WebKitWebView,
        ) -> *mut WebKitBackForwardList;
        pub fn webkit_web_view_go_to_back_forward_list_item(
            web_view: *mut WebKitWebView,
            list_item: *mut WebKitBackForwardListItem,
        );
        pub fn webkit_web_view_get_uri(web_view: *mut WebKitWebView) -> *const gchar;
        pub fn webkit_web_view_get_settings(
            web_view: *mut WebKitWebView,
        ) -> *mut WebKitSettings;
        pub fn webkit_web_view_get_find_controller(
            web_view: *mut WebKitWebView,
        ) -> *mut WebKitFindController;
        pub fn webkit_web_view_get_main_resource(
            web_view: *mut WebKitWebView,
        ) -> *mut WebKitWebResource;
        pub fn webkit_web_view_get_inspector(
            web_view: *mut WebKitWebView,
        ) -> *mut WebKitWebInspector;
        pub fn webkit_web_view_save_to_file(
            web_view: *mut WebKitWebView,
            file: *mut GFile,
            save_mode: WebKitSaveMode,
            cancellable: *mut GCancellable,
            callback: GAsyncReadyCallback,
            user_data: gpointer,
        );
        pub fn webkit_web_view_save_to_file_finish(
            web_view: *mut WebKitWebView,
            result: *mut GAsyncResult,
            error: *mut *mut GError,
        ) -> gboolean;
        pub fn webkit_web_view_get_tls_info(
            web_view: *mut WebKitWebView,
            certificate: *mut *mut GTlsCertificate,
            errors: *mut GTlsCertificateFlags,
        ) -> gboolean;
        pub fn webkit_web_view_get_session_state(
            web_view: *mut WebKitWebView,
        ) -> *mut WebKitWebViewSessionState;
        pub fn webkit_web_view_restore_session_state(
            web_view: *mut WebKitWebView,
            state: *mut WebKitWebViewSessionState,
        );
    }
}
pub mod WebKitWebViewBase_h {
    pub type WebKitWebViewBase = _WebKitWebViewBase;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _WebKitWebViewBase {
        pub parent: GtkContainer,
        pub priv_0: *mut WebKitWebViewBasePrivate,
    }
    pub type WebKitWebViewBasePrivate = _WebKitWebViewBasePrivate;
    use super::gtkcontainer_h::GtkContainer;
    unsafe extern "C" {
        pub type _WebKitWebViewBasePrivate;
    }
}
pub mod WebKitFeature_h {
    pub type WebKitFeature = _WebKitFeature;
    pub type WebKitFeatureList = _WebKitFeatureList;
    pub type WebKitFeatureList_autoptr = *mut WebKitFeatureList;
    use super::glibconfig_h::gsize;
    unsafe extern "C" {
        pub type _WebKitFeature;
        pub type _WebKitFeatureList;
        pub fn webkit_feature_get_identifier(
            feature: *mut WebKitFeature,
        ) -> *const std::ffi::c_char;
        pub fn webkit_feature_list_get_length(
            feature_list: *mut WebKitFeatureList,
        ) -> gsize;
        pub fn webkit_feature_list_get(
            feature_list: *mut WebKitFeatureList,
            index: gsize,
        ) -> *mut WebKitFeature;
    }
}
pub mod WebKitFindController_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _WebKitFindController {
        pub parent: GObject,
        pub priv_0: *mut WebKitFindControllerPrivate,
    }
    pub type WebKitFindControllerPrivate = _WebKitFindControllerPrivate;
    pub type WebKitFindController = _WebKitFindController;
    pub type C2RustUnnamed_1 = std::ffi::c_uint;
    pub const WEBKIT_FIND_OPTIONS_WRAP_AROUND: C2RustUnnamed_1 = 16;
    pub const WEBKIT_FIND_OPTIONS_BACKWARDS: C2RustUnnamed_1 = 8;
    pub const WEBKIT_FIND_OPTIONS_TREAT_MEDIAL_CAPITAL_AS_WORD_START: C2RustUnnamed_1 = 4;
    pub const WEBKIT_FIND_OPTIONS_AT_WORD_STARTS: C2RustUnnamed_1 = 2;
    pub const WEBKIT_FIND_OPTIONS_CASE_INSENSITIVE: C2RustUnnamed_1 = 1;
    pub const WEBKIT_FIND_OPTIONS_NONE: C2RustUnnamed_1 = 0;
    use super::gobject_h::GObject;
    use super::gtypes_h::{gchar, guint};
    use super::glibconfig_h::guint32;
    unsafe extern "C" {
        pub type _WebKitFindControllerPrivate;
        pub fn webkit_find_controller_search(
            find_controller: *mut WebKitFindController,
            search_text: *const gchar,
            find_options: guint32,
            max_match_count: guint,
        );
        pub fn webkit_find_controller_search_finish(
            find_controller: *mut WebKitFindController,
        );
        pub fn webkit_find_controller_search_next(
            find_controller: *mut WebKitFindController,
        );
        pub fn webkit_find_controller_search_previous(
            find_controller: *mut WebKitFindController,
        );
    }
}
pub mod WebKitHitTestResult_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _WebKitHitTestResult {
        pub parent: GObject,
        pub priv_0: *mut WebKitHitTestResultPrivate,
    }
    pub type WebKitHitTestResultPrivate = _WebKitHitTestResultPrivate;
    pub type WebKitHitTestResult = _WebKitHitTestResult;
    pub type C2RustUnnamed_2 = std::ffi::c_uint;
    pub const WEBKIT_HIT_TEST_RESULT_CONTEXT_SELECTION: C2RustUnnamed_2 = 128;
    pub const WEBKIT_HIT_TEST_RESULT_CONTEXT_SCROLLBAR: C2RustUnnamed_2 = 64;
    pub const WEBKIT_HIT_TEST_RESULT_CONTEXT_EDITABLE: C2RustUnnamed_2 = 32;
    pub const WEBKIT_HIT_TEST_RESULT_CONTEXT_MEDIA: C2RustUnnamed_2 = 16;
    pub const WEBKIT_HIT_TEST_RESULT_CONTEXT_IMAGE: C2RustUnnamed_2 = 8;
    pub const WEBKIT_HIT_TEST_RESULT_CONTEXT_LINK: C2RustUnnamed_2 = 4;
    pub const WEBKIT_HIT_TEST_RESULT_CONTEXT_DOCUMENT: C2RustUnnamed_2 = 2;
    use super::gobject_h::GObject;
    use super::gtypes_h::{guint, gboolean, gchar};
    unsafe extern "C" {
        pub type _WebKitHitTestResultPrivate;
        pub fn webkit_hit_test_result_get_context(
            hit_test_result: *mut WebKitHitTestResult,
        ) -> guint;
        pub fn webkit_hit_test_result_context_is_link(
            hit_test_result: *mut WebKitHitTestResult,
        ) -> gboolean;
        pub fn webkit_hit_test_result_get_link_uri(
            hit_test_result: *mut WebKitHitTestResult,
        ) -> *const gchar;
    }
}
pub mod WebKitInstallMissingMediaPluginsPermissionRequest_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _WebKitInstallMissingMediaPluginsPermissionRequest {
        pub parent: GObject,
        pub priv_0: *mut WebKitInstallMissingMediaPluginsPermissionRequestPrivate,
    }
    pub type WebKitInstallMissingMediaPluginsPermissionRequestPrivate = _WebKitInstallMissingMediaPluginsPermissionRequestPrivate;
    pub type WebKitInstallMissingMediaPluginsPermissionRequest = _WebKitInstallMissingMediaPluginsPermissionRequest;
    use super::gobject_h::GObject;
    use super::gtype_h::GType;
    use super::gtypes_h::gchar;
    unsafe extern "C" {
        pub type _WebKitInstallMissingMediaPluginsPermissionRequestPrivate;
        pub fn webkit_install_missing_media_plugins_permission_request_get_type() -> GType;
        pub fn webkit_install_missing_media_plugins_permission_request_get_description(
            request: *mut WebKitInstallMissingMediaPluginsPermissionRequest,
        ) -> *const gchar;
    }
}
pub mod WebKitNavigationAction_h {
    pub type WebKitNavigationType = std::ffi::c_uint;
    pub const WEBKIT_NAVIGATION_TYPE_OTHER: WebKitNavigationType = 5;
    pub const WEBKIT_NAVIGATION_TYPE_FORM_RESUBMITTED: WebKitNavigationType = 4;
    pub const WEBKIT_NAVIGATION_TYPE_RELOAD: WebKitNavigationType = 3;
    pub const WEBKIT_NAVIGATION_TYPE_BACK_FORWARD: WebKitNavigationType = 2;
    pub const WEBKIT_NAVIGATION_TYPE_FORM_SUBMITTED: WebKitNavigationType = 1;
    pub const WEBKIT_NAVIGATION_TYPE_LINK_CLICKED: WebKitNavigationType = 0;
    pub type WebKitNavigationAction = _WebKitNavigationAction;
    use super::WebKitURIRequest_h::WebKitURIRequest;
    unsafe extern "C" {
        pub type _WebKitNavigationAction;
        pub fn webkit_navigation_action_get_navigation_type(
            navigation: *mut WebKitNavigationAction,
        ) -> WebKitNavigationType;
        pub fn webkit_navigation_action_get_request(
            navigation: *mut WebKitNavigationAction,
        ) -> *mut WebKitURIRequest;
    }
}
pub mod WebKitPolicyDecision_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _WebKitPolicyDecision {
        pub parent: GObject,
        pub priv_0: *mut WebKitPolicyDecisionPrivate,
    }
    pub type WebKitPolicyDecisionPrivate = _WebKitPolicyDecisionPrivate;
    pub type WebKitPolicyDecision = _WebKitPolicyDecision;
    use super::gobject_h::GObject;
    unsafe extern "C" {
        pub type _WebKitPolicyDecisionPrivate;
        pub fn webkit_policy_decision_use(decision: *mut WebKitPolicyDecision);
        pub fn webkit_policy_decision_ignore(decision: *mut WebKitPolicyDecision);
        pub fn webkit_policy_decision_download(decision: *mut WebKitPolicyDecision);
    }
}
pub mod WebKitNavigationPolicyDecision_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _WebKitNavigationPolicyDecision {
        pub parent: WebKitPolicyDecision,
        pub priv_0: *mut WebKitNavigationPolicyDecisionPrivate,
    }
    pub type WebKitNavigationPolicyDecisionPrivate = _WebKitNavigationPolicyDecisionPrivate;
    pub type WebKitNavigationPolicyDecision = _WebKitNavigationPolicyDecision;
    use super::WebKitPolicyDecision_h::WebKitPolicyDecision;
    use super::gtype_h::GType;
    use super::WebKitNavigationAction_h::WebKitNavigationAction;
    unsafe extern "C" {
        pub type _WebKitNavigationPolicyDecisionPrivate;
        pub fn webkit_navigation_policy_decision_get_type() -> GType;
        pub fn webkit_navigation_policy_decision_get_navigation_action(
            decision: *mut WebKitNavigationPolicyDecision,
        ) -> *mut WebKitNavigationAction;
    }
}
pub mod WebKitPermissionRequest_h {
    pub type WebKitPermissionRequest = _WebKitPermissionRequest;
    unsafe extern "C" {
        pub type _WebKitPermissionRequest;
        pub fn webkit_permission_request_allow(request: *mut WebKitPermissionRequest);
        pub fn webkit_permission_request_deny(request: *mut WebKitPermissionRequest);
    }
}
pub mod WebKitSettings_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _WebKitSettings {
        pub parent: GObject,
        pub priv_0: *mut WebKitSettingsPrivate,
    }
    pub type WebKitSettingsPrivate = _WebKitSettingsPrivate;
    pub type WebKitSettings = _WebKitSettings;
    pub type WebKitHardwareAccelerationPolicy = std::ffi::c_uint;
    pub const WEBKIT_HARDWARE_ACCELERATION_POLICY_NEVER: WebKitHardwareAccelerationPolicy = 2;
    pub const WEBKIT_HARDWARE_ACCELERATION_POLICY_ALWAYS: WebKitHardwareAccelerationPolicy = 1;
    pub const WEBKIT_HARDWARE_ACCELERATION_POLICY_ON_DEMAND: WebKitHardwareAccelerationPolicy = 0;
    use super::gobject_h::GObject;
    use super::WebKitFeature_h::{WebKitFeature, WebKitFeatureList};
    use super::gtypes_h::gboolean;
    unsafe extern "C" {
        pub type _WebKitSettingsPrivate;
        pub fn webkit_settings_get_hardware_acceleration_policy(
            settings: *mut WebKitSettings,
        ) -> WebKitHardwareAccelerationPolicy;
        pub fn webkit_settings_set_hardware_acceleration_policy(
            settings: *mut WebKitSettings,
            policy: WebKitHardwareAccelerationPolicy,
        );
        pub fn webkit_settings_set_feature_enabled(
            settings: *mut WebKitSettings,
            feature: *mut WebKitFeature,
            enabled: gboolean,
        );
        pub fn webkit_settings_get_all_features() -> *mut WebKitFeatureList;
    }
}
pub mod WebKitUserContent_h {
    pub type WebKitUserStyleSheet = _WebKitUserStyleSheet;
    unsafe extern "C" {
        pub type _WebKitUserStyleSheet;
    }
}
pub mod WebKitUserContentManager_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _WebKitUserContentManager {
        pub parent: GObject,
        pub priv_0: *mut WebKitUserContentManagerPrivate,
    }
    pub type WebKitUserContentManagerPrivate = _WebKitUserContentManagerPrivate;
    pub type WebKitUserContentManager = _WebKitUserContentManager;
    use super::gobject_h::GObject;
    use super::WebKitUserContent_h::WebKitUserStyleSheet;
    unsafe extern "C" {
        pub type _WebKitUserContentManagerPrivate;
        pub fn webkit_user_content_manager_new() -> *mut WebKitUserContentManager;
        pub fn webkit_user_content_manager_add_style_sheet(
            manager: *mut WebKitUserContentManager,
            stylesheet: *mut WebKitUserStyleSheet,
        );
        pub fn webkit_user_content_manager_remove_all_style_sheets(
            manager: *mut WebKitUserContentManager,
        );
    }
}
pub mod WebKitURISchemeRequest_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _WebKitURISchemeRequest {
        pub parent: GObject,
        pub priv_0: *mut WebKitURISchemeRequestPrivate,
    }
    pub type WebKitURISchemeRequestPrivate = _WebKitURISchemeRequestPrivate;
    pub type WebKitURISchemeRequest = _WebKitURISchemeRequest;
    use super::gobject_h::GObject;
    use super::gtypes_h::gchar;
    use super::WebKitDownload_h::WebKitWebView;
    unsafe extern "C" {
        pub type _WebKitURISchemeRequestPrivate;
        pub fn webkit_uri_scheme_request_get_uri(
            request: *mut WebKitURISchemeRequest,
        ) -> *const gchar;
        pub fn webkit_uri_scheme_request_get_web_view(
            request: *mut WebKitURISchemeRequest,
        ) -> *mut WebKitWebView;
    }
}
pub mod WebKitWebContext_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _WebKitWebContext {
        pub parent: GObject,
        pub priv_0: *mut WebKitWebContextPrivate,
    }
    pub type WebKitWebContextPrivate = _WebKitWebContextPrivate;
    pub type WebKitWebContext = _WebKitWebContext;
    use super::gobject_h::GObject;
    unsafe extern "C" {
        pub type _WebKitWebContextPrivate;
    }
}
pub mod WebKitWebResource_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _WebKitWebResource {
        pub parent: GObject,
        pub priv_0: *mut WebKitWebResourcePrivate,
    }
    pub type WebKitWebResourcePrivate = _WebKitWebResourcePrivate;
    pub type WebKitWebResource = _WebKitWebResource;
    use super::gobject_h::GObject;
    use super::giotypes_h::{GCancellable, GAsyncReadyCallback, GAsyncResult};
    use super::gtypes_h::{gpointer, guchar};
    use super::glibconfig_h::gsize;
    use super::gerror_h::GError;
    unsafe extern "C" {
        pub type _WebKitWebResourcePrivate;
        pub fn webkit_web_resource_get_data(
            resource: *mut WebKitWebResource,
            cancellable: *mut GCancellable,
            callback: GAsyncReadyCallback,
            user_data: gpointer,
        );
        pub fn webkit_web_resource_get_data_finish(
            resource: *mut WebKitWebResource,
            result: *mut GAsyncResult,
            length: *mut gsize,
            error: *mut *mut GError,
        ) -> *mut guchar;
    }
}
pub mod WebKitWebViewSessionState_h {
    pub type WebKitWebViewSessionState = _WebKitWebViewSessionState;
    use super::garray_h::GBytes;
    unsafe extern "C" {
        pub type _WebKitWebViewSessionState;
        pub fn webkit_web_view_session_state_new(
            data: *mut GBytes,
        ) -> *mut WebKitWebViewSessionState;
        pub fn webkit_web_view_session_state_unref(
            state: *mut WebKitWebViewSessionState,
        );
        pub fn webkit_web_view_session_state_serialize(
            state: *mut WebKitWebViewSessionState,
        ) -> *mut GBytes;
    }
}
pub mod WebKitWebInspector_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _WebKitWebInspector {
        pub parent: GObject,
        pub priv_0: *mut WebKitWebInspectorPrivate,
    }
    pub type WebKitWebInspectorPrivate = _WebKitWebInspectorPrivate;
    pub type WebKitWebInspector = _WebKitWebInspector;
    use super::gobject_h::GObject;
    unsafe extern "C" {
        pub type _WebKitWebInspectorPrivate;
        pub fn webkit_web_inspector_show(inspector: *mut WebKitWebInspector);
        pub fn webkit_web_inspector_close(inspector: *mut WebKitWebInspector);
    }
}
pub mod WebKitResponsePolicyDecision_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _WebKitResponsePolicyDecision {
        pub parent: WebKitPolicyDecision,
        pub priv_0: *mut WebKitResponsePolicyDecisionPrivate,
    }
    pub type WebKitResponsePolicyDecisionPrivate = _WebKitResponsePolicyDecisionPrivate;
    pub type WebKitResponsePolicyDecision = _WebKitResponsePolicyDecision;
    use super::WebKitPolicyDecision_h::WebKitPolicyDecision;
    use super::gtype_h::GType;
    use super::WebKitURIResponse_h::WebKitURIResponse;
    use super::gtypes_h::gboolean;
    unsafe extern "C" {
        pub type _WebKitResponsePolicyDecisionPrivate;
        pub fn webkit_response_policy_decision_get_type() -> GType;
        pub fn webkit_response_policy_decision_get_response(
            decision: *mut WebKitResponsePolicyDecision,
        ) -> *mut WebKitURIResponse;
        pub fn webkit_response_policy_decision_is_mime_type_supported(
            decision: *mut WebKitResponsePolicyDecision,
        ) -> gboolean;
    }
}
pub mod WebKitUserMediaPermissionRequest_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _WebKitUserMediaPermissionRequest {
        pub parent: GObject,
        pub priv_0: *mut WebKitUserMediaPermissionRequestPrivate,
    }
    pub type WebKitUserMediaPermissionRequestPrivate = _WebKitUserMediaPermissionRequestPrivate;
    pub type WebKitUserMediaPermissionRequest = _WebKitUserMediaPermissionRequest;
    use super::gobject_h::GObject;
    use super::gtype_h::GType;
    use super::gtypes_h::gboolean;
    unsafe extern "C" {
        pub type _WebKitUserMediaPermissionRequestPrivate;
        pub fn webkit_user_media_permission_request_get_type() -> GType;
        pub fn webkit_user_media_permission_is_for_audio_device(
            request: *mut WebKitUserMediaPermissionRequest,
        ) -> gboolean;
        pub fn webkit_user_media_permission_is_for_video_device(
            request: *mut WebKitUserMediaPermissionRequest,
        ) -> gboolean;
    }
}
pub mod lua_h {
    pub type lua_CFunction = Option::<
        unsafe extern "C" fn(*mut lua_State) -> std::ffi::c_int,
    >;
    pub type lua_Number = std::ffi::c_double;
    pub type lua_Integer = ptrdiff_t;
    pub const LUA_MULTRET: std::ffi::c_int = -(1 as std::ffi::c_int);
    pub const LUA_REGISTRYINDEX: std::ffi::c_int = -(10000 as std::ffi::c_int);
    pub const LUA_TNIL: std::ffi::c_int = 0 as std::ffi::c_int;
    pub const LUA_TBOOLEAN: std::ffi::c_int = 1 as std::ffi::c_int;
    pub const LUA_TLIGHTUSERDATA: std::ffi::c_int = 2 as std::ffi::c_int;
    pub const LUA_TNUMBER: std::ffi::c_int = 3 as std::ffi::c_int;
    pub const LUA_TTABLE: std::ffi::c_int = 5 as std::ffi::c_int;
    pub const LUA_TFUNCTION: std::ffi::c_int = 6 as std::ffi::c_int;
    use super::__stddef_ptrdiff_t_h::ptrdiff_t;
    use super::__stddef_size_t_h::size_t;
    unsafe extern "C" {
        pub type lua_State;
        pub fn lua_gettop(L: *mut lua_State) -> std::ffi::c_int;
        pub fn lua_settop(L: *mut lua_State, idx: std::ffi::c_int);
        pub fn lua_pushvalue(L: *mut lua_State, idx: std::ffi::c_int);
        pub fn lua_remove(L: *mut lua_State, idx: std::ffi::c_int);
        pub fn lua_insert(L: *mut lua_State, idx: std::ffi::c_int);
        pub fn lua_isnumber(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;
        pub fn lua_isstring(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;
        pub fn lua_type(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;
        pub fn lua_typename(
            L: *mut lua_State,
            tp: std::ffi::c_int,
        ) -> *const std::ffi::c_char;
        pub fn lua_tonumber(L: *mut lua_State, idx: std::ffi::c_int) -> lua_Number;
        pub fn lua_tointeger(L: *mut lua_State, idx: std::ffi::c_int) -> lua_Integer;
        pub fn lua_toboolean(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;
        pub fn lua_tolstring(
            L: *mut lua_State,
            idx: std::ffi::c_int,
            len: *mut size_t,
        ) -> *const std::ffi::c_char;
        pub fn lua_objlen(L: *mut lua_State, idx: std::ffi::c_int) -> size_t;
        pub fn lua_touserdata(
            L: *mut lua_State,
            idx: std::ffi::c_int,
        ) -> *mut std::ffi::c_void;
        pub fn lua_topointer(
            L: *mut lua_State,
            idx: std::ffi::c_int,
        ) -> *const std::ffi::c_void;
        pub fn lua_pushnil(L: *mut lua_State);
        pub fn lua_pushnumber(L: *mut lua_State, n: lua_Number);
        pub fn lua_pushinteger(L: *mut lua_State, n: lua_Integer);
        pub fn lua_pushlstring(L: *mut lua_State, s: *const std::ffi::c_char, l: size_t);
        pub fn lua_pushstring(L: *mut lua_State, s: *const std::ffi::c_char);
        pub fn lua_pushcclosure(
            L: *mut lua_State,
            fn_0: lua_CFunction,
            n: std::ffi::c_int,
        );
        pub fn lua_pushboolean(L: *mut lua_State, b: std::ffi::c_int);
        pub fn lua_pushlightuserdata(L: *mut lua_State, p: *mut std::ffi::c_void);
        pub fn lua_rawget(L: *mut lua_State, idx: std::ffi::c_int);
        pub fn lua_rawgeti(L: *mut lua_State, idx: std::ffi::c_int, n: std::ffi::c_int);
        pub fn lua_createtable(
            L: *mut lua_State,
            narr: std::ffi::c_int,
            nrec: std::ffi::c_int,
        );
        pub fn lua_setfield(
            L: *mut lua_State,
            idx: std::ffi::c_int,
            k: *const std::ffi::c_char,
        );
        pub fn lua_rawset(L: *mut lua_State, idx: std::ffi::c_int);
        pub fn lua_rawseti(L: *mut lua_State, idx: std::ffi::c_int, n: std::ffi::c_int);
        pub fn lua_setmetatable(
            L: *mut lua_State,
            objindex: std::ffi::c_int,
        ) -> std::ffi::c_int;
        pub fn lua_pcall(
            L: *mut lua_State,
            nargs: std::ffi::c_int,
            nresults: std::ffi::c_int,
            errfunc: std::ffi::c_int,
        ) -> std::ffi::c_int;
    }
}
pub mod common_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _common_t {
        pub L: *mut lua_State,
    }
    pub type common_t = _common_t;
    use super::lua_h::lua_State;
    unsafe extern "C" {
        pub static mut common: common_t;
    }
}
pub mod widget_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
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
    pub type widget_destructor_t = unsafe extern "C" fn(*mut widget_t) -> ();
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct widget_info_t {
        pub tok: luakit_token_t,
        pub name: *const gchar,
        pub wc: Option::<widget_constructor_t>,
    }
    pub type widget_constructor_t = unsafe extern "C" fn(
        *mut lua_State,
        *mut widget_t,
        luakit_token_t,
    ) -> *mut widget_t;
    pub const GOBJECT_LUAKIT_WIDGET_DATA_KEY: [std::ffi::c_char; 19] = unsafe {
        *::core::mem::transmute::<
            &[u8; 19],
            &[std::ffi::c_char; 19],
        >(b"luakit_widget_data\0")
    };
    #[inline]
    pub unsafe extern "C" fn luaH_checkwidget(
        L: *mut lua_State,
        udx: gint,
    ) -> *mut widget_t {
        let w = luaH_checkudata(L, udx, &mut widget_class) as *mut widget_t;
        if ((*w).widget).is_null() {
            luaL_error(
                L,
                b"widget %p (%s) has been destroyed\0" as *const u8
                    as *const std::ffi::c_char,
                w,
                (*(*w).info).name,
            );
        }
        if ({
            let mut __inst = (*w).widget as *mut GTypeInstance;
            let mut __t = gtk_widget_get_type();
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
        }) != 0
        {} else {
            g_assertion_message_expr(
                G_LOG_DOMAIN as *const std::ffi::c_char,
                b"./clib/widget.h\0" as *const u8 as *const std::ffi::c_char,
                101 as std::ffi::c_int,
                (*::core::mem::transmute::<
                    &[u8; 17],
                    &[std::ffi::c_char; 17],
                >(b"luaH_checkwidget\0"))
                    .as_ptr(),
                b"GTK_IS_WIDGET(w->widget)\0" as *const u8 as *const std::ffi::c_char,
            );
        }
        return w;
    }
    use super::signal_h::signal_t;
    use super::gtypes_h::{gint, gpointer, gchar, gboolean};
    use super::lua_h::lua_State;
    use super::tokenize_h::luakit_token_t;
    use super::gtktypes_h::GtkWidget;
    use super::gtkcssprovider_h::GtkCssProvider;
    use super::luaclass_h::{
        lua_class_t, lua_class_allocator_t, lua_class_property_array_t,
        lua_class_propfunc_t, luaH_checkudata,
    };
    use super::lauxlib_h::luaL_error;
    use super::gtype_h::{GTypeInstance, GType, g_type_check_instance_is_a};
    use super::gtkwidget_h::gtk_widget_get_type;
    use super::gtestutils_h::g_assertion_message_expr;
    use super::gmessages_h::G_LOG_DOMAIN;
    unsafe extern "C" {
        pub static mut widget_class: lua_class_t;
    }
}
pub mod tokenize_h {
    pub type luakit_token_t = std::ffi::c_uint;
    pub const L_TK_ZOOM_TEXT_ONLY: luakit_token_t = 274;
    pub const L_TK_ZOOM_LEVEL: luakit_token_t = 273;
    pub const L_TK_YPAGE_SIZE: luakit_token_t = 272;
    pub const L_TK_YMAX: luakit_token_t = 271;
    pub const L_TK_Y: luakit_token_t = 270;
    pub const L_TK_XPAGE_SIZE: luakit_token_t = 269;
    pub const L_TK_XMAX: luakit_token_t = 268;
    pub const L_TK_X: luakit_token_t = 267;
    pub const L_TK_WRAP_JS: luakit_token_t = 266;
    pub const L_TK_WIN_XID: luakit_token_t = 265;
    pub const L_TK_WINDOWS: luakit_token_t = 264;
    pub const L_TK_WINDOW: luakit_token_t = 263;
    pub const L_TK_WIDTH: luakit_token_t = 262;
    pub const L_TK_WEB_PROCESS_ID: luakit_token_t = 261;
    pub const L_TK_WEBVIEW: luakit_token_t = 260;
    pub const L_TK_WEBSITE_DATA: luakit_token_t = 259;
    pub const L_TK_WEBKIT_VERSION: luakit_token_t = 258;
    pub const L_TK_WEBKIT_USER_AGENT_VERSION: luakit_token_t = 257;
    pub const L_TK_WEBKIT2: luakit_token_t = 256;
    pub const L_TK_VPANED: luakit_token_t = 255;
    pub const L_TK_VISIBLE_CHILD: luakit_token_t = 254;
    pub const L_TK_VISIBLE: luakit_token_t = 253;
    pub const L_TK_VIDEOS_DIR: luakit_token_t = 252;
    pub const L_TK_VERSION: luakit_token_t = 251;
    pub const L_TK_VERBOSE: luakit_token_t = 250;
    pub const L_TK_VBOX: luakit_token_t = 249;
    pub const L_TK_VALUE: luakit_token_t = 248;
    pub const L_TK_USER_AGENT: luakit_token_t = 247;
    pub const L_TK_URI: luakit_token_t = 246;
    pub const L_TK_URGENCY_HINT: luakit_token_t = 245;
    pub const L_TK_TYPE: luakit_token_t = 244;
    pub const L_TK_TOTAL_SIZE: luakit_token_t = 243;
    pub const L_TK_TOP: luakit_token_t = 242;
    pub const L_TK_TOOLTIP: luakit_token_t = 241;
    pub const L_TK_TITLE: luakit_token_t = 240;
    pub const L_TK_TEXT_CONTENT: luakit_token_t = 239;
    pub const L_TK_TEXTWIDTH: luakit_token_t = 238;
    pub const L_TK_TEXT: luakit_token_t = 237;
    pub const L_TK_TEMPLATES_DIR: luakit_token_t = 236;
    pub const L_TK_TAG_NAME: luakit_token_t = 235;
    pub const L_TK_SYSTEM_DATA_DIRS: luakit_token_t = 234;
    pub const L_TK_SYSTEM_CONFIG_DIRS: luakit_token_t = 233;
    pub const L_TK_SWITCH: luakit_token_t = 232;
    pub const L_TK_SUGGESTED_FILENAME: luakit_token_t = 231;
    pub const L_TK_SUBMIT: luakit_token_t = 230;
    pub const L_TK_STYLESHEETS: luakit_token_t = 229;
    pub const L_TK_STYLE: luakit_token_t = 228;
    pub const L_TK_STOP: luakit_token_t = 227;
    pub const L_TK_STATUS: luakit_token_t = 226;
    pub const L_TK_STARTED: luakit_token_t = 225;
    pub const L_TK_START: luakit_token_t = 224;
    pub const L_TK_STACK: luakit_token_t = 223;
    pub const L_TK_SSL_TRUSTED: luakit_token_t = 222;
    pub const L_TK_SRC: luakit_token_t = 221;
    pub const L_TK_SPINNER: luakit_token_t = 220;
    pub const L_TK_SPELL_CHECKING_LANGUAGES: luakit_token_t = 219;
    pub const L_TK_SPACING: luakit_token_t = 218;
    pub const L_TK_SOURCE: luakit_token_t = 217;
    pub const L_TK_SOCKET: luakit_token_t = 216;
    pub const L_TK_SHOW_TABS: luakit_token_t = 215;
    pub const L_TK_SHOW_INSPECTOR: luakit_token_t = 214;
    pub const L_TK_SHOW_FRAME: luakit_token_t = 213;
    pub const L_TK_SHOW_BORDER: luakit_token_t = 212;
    pub const L_TK_SHOW: luakit_token_t = 211;
    pub const L_TK_SET_TITLE: luakit_token_t = 210;
    pub const L_TK_SET_PDFJS: luakit_token_t = 209;
    pub const L_TK_SET_FAVICON_FOR_URI: luakit_token_t = 208;
    pub const L_TK_SET_DEFAULT_SIZE: luakit_token_t = 207;
    pub const L_TK_SET_DARK_MODE: luakit_token_t = 206;
    pub const L_TK_SESSION_STATE: luakit_token_t = 205;
    pub const L_TK_SERIF_FONT_FAMILY: luakit_token_t = 204;
    pub const L_TK_SEND_KEY: luakit_token_t = 203;
    pub const L_TK_SELECT_REGION: luakit_token_t = 202;
    pub const L_TK_SELECTION: luakit_token_t = 201;
    pub const L_TK_SELECTABLE: luakit_token_t = 200;
    pub const L_TK_SECONDARY: luakit_token_t = 199;
    pub const L_TK_SEARCH_PREVIOUS: luakit_token_t = 198;
    pub const L_TK_SEARCH_NEXT: luakit_token_t = 197;
    pub const L_TK_SEARCH: luakit_token_t = 196;
    pub const L_TK_SCROLL_Y: luakit_token_t = 195;
    pub const L_TK_SCROLL_X: luakit_token_t = 194;
    pub const L_TK_SCROLLED: luakit_token_t = 193;
    pub const L_TK_SCROLLBARS: luakit_token_t = 192;
    pub const L_TK_SCROLL: luakit_token_t = 191;
    pub const L_TK_SCREEN: luakit_token_t = 190;
    pub const L_TK_SCALE: luakit_token_t = 189;
    pub const L_TK_SAVE: luakit_token_t = 188;
    pub const L_TK_SANS_SERIF_FONT_FAMILY: luakit_token_t = 187;
    pub const L_TK_ROOT_WIN_XID: luakit_token_t = 186;
    pub const L_TK_RIGHT: luakit_token_t = 185;
    pub const L_TK_RESOURCE_PATH: luakit_token_t = 184;
    pub const L_TK_REPLACE: luakit_token_t = 183;
    pub const L_TK_REORDER: luakit_token_t = 182;
    pub const L_TK_REMOVE_EVENT_LISTENER: luakit_token_t = 181;
    pub const L_TK_REMOVE: luakit_token_t = 180;
    pub const L_TK_RELOAD_BYPASS_CACHE: luakit_token_t = 179;
    pub const L_TK_RELOAD: luakit_token_t = 178;
    pub const L_TK_RECT: luakit_token_t = 177;
    pub const L_TK_QUERY: luakit_token_t = 176;
    pub const L_TK_PUBLIC_SHARE_DIR: luakit_token_t = 175;
    pub const L_TK_PROXY_URI: luakit_token_t = 174;
    pub const L_TK_PROGRESS: luakit_token_t = 173;
    pub const L_TK_PROCESS_LIMIT: luakit_token_t = 172;
    pub const L_TK_PRIVATE: luakit_token_t = 171;
    pub const L_TK_PRINT_BACKGROUNDS: luakit_token_t = 170;
    pub const L_TK_PRIMARY: luakit_token_t = 169;
    pub const L_TK_PREV_SIBLING: luakit_token_t = 168;
    pub const L_TK_POSITION: luakit_token_t = 167;
    pub const L_TK_PLUGGED: luakit_token_t = 166;
    pub const L_TK_PICTURES_DIR: luakit_token_t = 165;
    pub const L_TK_PICTOGRAPH_FONT_FAMILY: luakit_token_t = 164;
    pub const L_TK_PATTERN: luakit_token_t = 163;
    pub const L_TK_PARENT: luakit_token_t = 162;
    pub const L_TK_PACK2: luakit_token_t = 161;
    pub const L_TK_PACK1: luakit_token_t = 160;
    pub const L_TK_PACK: luakit_token_t = 159;
    pub const L_TK_OWNER_DOCUMENT: luakit_token_t = 158;
    pub const L_TK_OVERLAY: luakit_token_t = 157;
    pub const L_TK_OPTIONS: luakit_token_t = 156;
    pub const L_TK_NOUNIQUE: luakit_token_t = 155;
    pub const L_TK_NOTEBOOK: luakit_token_t = 154;
    pub const L_TK_NEXT_SIBLING: luakit_token_t = 153;
    pub const L_TK_NAME: luakit_token_t = 152;
    pub const L_TK_MUSIC_DIR: luakit_token_t = 151;
    pub const L_TK_MONOSPACE_FONT_FAMILY: luakit_token_t = 150;
    pub const L_TK_MIN_SIZE: luakit_token_t = 149;
    pub const L_TK_MINIMUM_FONT_SIZE: luakit_token_t = 148;
    pub const L_TK_MIME_TYPE: luakit_token_t = 147;
    pub const L_TK_MEDIA_PLAYBACK_REQUIRES_GESTURE: luakit_token_t = 146;
    pub const L_TK_MEDIA_PLAYBACK_ALLOWS_INLINE: luakit_token_t = 145;
    pub const L_TK_MAXIMIZED: luakit_token_t = 144;
    pub const L_TK_MARGIN_TOP: luakit_token_t = 143;
    pub const L_TK_MARGIN_RIGHT: luakit_token_t = 142;
    pub const L_TK_MARGIN_LEFT: luakit_token_t = 141;
    pub const L_TK_MARGIN_BOTTOM: luakit_token_t = 140;
    pub const L_TK_MARGIN: luakit_token_t = 139;
    pub const L_TK_LOAD_STRING: luakit_token_t = 138;
    pub const L_TK_LOADING: luakit_token_t = 137;
    pub const L_TK_LEFT: luakit_token_t = 136;
    pub const L_TK_LAST_CHILD: luakit_token_t = 135;
    pub const L_TK_LABEL: luakit_token_t = 134;
    pub const L_TK_JAVASCRIPT_CAN_OPEN_WINDOWS_AUTOMATICALLY: luakit_token_t = 133;
    pub const L_TK_JAVASCRIPT_CAN_ACCESS_CLIPBOARD: luakit_token_t = 132;
    pub const L_TK_IS_PLAYING_AUDIO: luakit_token_t = 131;
    pub const L_TK_IS_LOADING: luakit_token_t = 130;
    pub const L_TK_IS_ALIVE: luakit_token_t = 129;
    pub const L_TK_INVALIDATE: luakit_token_t = 128;
    pub const L_TK_INTERVAL: luakit_token_t = 127;
    pub const L_TK_INSTALL_PATHS: luakit_token_t = 126;
    pub const L_TK_INSTALL_PATH: luakit_token_t = 125;
    pub const L_TK_INSPECTOR: luakit_token_t = 124;
    pub const L_TK_INSERT: luakit_token_t = 123;
    pub const L_TK_INNER_WIDTH: luakit_token_t = 122;
    pub const L_TK_INNER_HTML: luakit_token_t = 121;
    pub const L_TK_INNER_HEIGHT: luakit_token_t = 120;
    pub const L_TK_INDEXOF: luakit_token_t = 119;
    pub const L_TK_IMAGE: luakit_token_t = 118;
    pub const L_TK_ID: luakit_token_t = 117;
    pub const L_TK_ICON: luakit_token_t = 116;
    pub const L_TK_HREF: luakit_token_t = 115;
    pub const L_TK_HPANED: luakit_token_t = 114;
    pub const L_TK_HOVERED_URI: luakit_token_t = 113;
    pub const L_TK_HOMOGENEOUS: luakit_token_t = 112;
    pub const L_TK_HISTORY: luakit_token_t = 111;
    pub const L_TK_HIDE: luakit_token_t = 110;
    pub const L_TK_HEIGHT: luakit_token_t = 109;
    pub const L_TK_HBOX: luakit_token_t = 108;
    pub const L_TK_HARDWARE_ACCELERATION_POLICY: luakit_token_t = 107;
    pub const L_TK_GO_FORWARD: luakit_token_t = 106;
    pub const L_TK_GO_BACK: luakit_token_t = 105;
    pub const L_TK_GET_TITLE: luakit_token_t = 104;
    pub const L_TK_GET_SOURCE: luakit_token_t = 103;
    pub const L_TK_FULLSCREEN: luakit_token_t = 102;
    pub const L_TK_FONT: luakit_token_t = 101;
    pub const L_TK_FOCUSED: luakit_token_t = 100;
    pub const L_TK_FOCUS: luakit_token_t = 99;
    pub const L_TK_FIRST_CHILD: luakit_token_t = 98;
    pub const L_TK_FINISHED: luakit_token_t = 97;
    pub const L_TK_FILL: luakit_token_t = 96;
    pub const L_TK_FILENAME: luakit_token_t = 95;
    pub const L_TK_FG: luakit_token_t = 94;
    pub const L_TK_FETCH: luakit_token_t = 93;
    pub const L_TK_FANTASY_FONT_FAMILY: luakit_token_t = 92;
    pub const L_TK_EXECPATH: luakit_token_t = 91;
    pub const L_TK_EVENTBOX: luakit_token_t = 90;
    pub const L_TK_EVAL_JS: luakit_token_t = 89;
    pub const L_TK_ERROR: luakit_token_t = 88;
    pub const L_TK_ENTRY: luakit_token_t = 87;
    pub const L_TK_END: luakit_token_t = 86;
    pub const L_TK_ENABLE_XSS_AUDITOR: luakit_token_t = 85;
    pub const L_TK_ENABLE_WRITE_CONSOLE_MESSAGES_TO_STDOUT: luakit_token_t = 84;
    pub const L_TK_ENABLE_WEBGL: luakit_token_t = 83;
    pub const L_TK_ENABLE_WEBAUDIO: luakit_token_t = 82;
    pub const L_TK_ENABLE_TABS_TO_LINKS: luakit_token_t = 81;
    pub const L_TK_ENABLE_SPELL_CHECKING: luakit_token_t = 80;
    pub const L_TK_ENABLE_SPATIAL_NAVIGATION: luakit_token_t = 79;
    pub const L_TK_ENABLE_SMOOTH_SCROLLING: luakit_token_t = 78;
    pub const L_TK_ENABLE_SITE_SPECIFIC_QUIRKS: luakit_token_t = 77;
    pub const L_TK_ENABLE_SCRIPTS: luakit_token_t = 76;
    pub const L_TK_ENABLE_RESIZABLE_TEXT_AREAS: luakit_token_t = 75;
    pub const L_TK_ENABLE_PLUGINS: luakit_token_t = 74;
    pub const L_TK_ENABLE_PAGE_CACHE: luakit_token_t = 73;
    pub const L_TK_ENABLE_MEDIA_STREAM: luakit_token_t = 72;
    pub const L_TK_ENABLE_MEDIASOURCE: luakit_token_t = 71;
    pub const L_TK_ENABLE_JAVASCRIPT: luakit_token_t = 70;
    pub const L_TK_ENABLE_JAVA: luakit_token_t = 69;
    pub const L_TK_ENABLE_HYPERLINK_AUDITING: luakit_token_t = 68;
    pub const L_TK_ENABLE_HTML5_LOCAL_STORAGE: luakit_token_t = 67;
    pub const L_TK_ENABLE_HTML5_DATABASE: luakit_token_t = 66;
    pub const L_TK_ENABLE_FULLSCREEN: luakit_token_t = 65;
    pub const L_TK_ENABLE_FRAME_FLATTENING: luakit_token_t = 64;
    pub const L_TK_ENABLE_DNS_PREFETCHING: luakit_token_t = 63;
    pub const L_TK_ENABLE_DEVELOPER_EXTRAS: luakit_token_t = 62;
    pub const L_TK_ENABLE_CARET_BROWSING: luakit_token_t = 61;
    pub const L_TK_ENABLE_ACCELERATED_2D_CANVAS: luakit_token_t = 60;
    pub const L_TK_ELEMENT_FROM_POINT: luakit_token_t = 59;
    pub const L_TK_ELAPSED_TIME: luakit_token_t = 58;
    pub const L_TK_EDITABLE: luakit_token_t = 57;
    pub const L_TK_DRAW_COMPOSITING_INDICATORS: luakit_token_t = 56;
    pub const L_TK_DRAWING_AREA: luakit_token_t = 55;
    pub const L_TK_DOWNLOAD_DIR: luakit_token_t = 54;
    pub const L_TK_DOCUMENTS_DIR: luakit_token_t = 53;
    pub const L_TK_DOCUMENT: luakit_token_t = 52;
    pub const L_TK_DEV_PATHS: luakit_token_t = 51;
    pub const L_TK_DESTROY: luakit_token_t = 50;
    pub const L_TK_DESTINATION: luakit_token_t = 49;
    pub const L_TK_DESKTOP_DIR: luakit_token_t = 48;
    pub const L_TK_DEFAULT_MONOSPACE_FONT_SIZE: luakit_token_t = 47;
    pub const L_TK_DEFAULT_FONT_SIZE: luakit_token_t = 46;
    pub const L_TK_DEFAULT_FONT_FAMILY: luakit_token_t = 45;
    pub const L_TK_DEFAULT_CHARSET: luakit_token_t = 44;
    pub const L_TK_DECORATED: luakit_token_t = 43;
    pub const L_TK_DATA_DIR: luakit_token_t = 42;
    pub const L_TK_CURSIVE_FONT_FAMILY: luakit_token_t = 41;
    pub const L_TK_CURRENT_SIZE: luakit_token_t = 40;
    pub const L_TK_CURRENT: luakit_token_t = 39;
    pub const L_TK_CSS: luakit_token_t = 38;
    pub const L_TK_CREATE_ELEMENT: luakit_token_t = 37;
    pub const L_TK_CRASH: luakit_token_t = 36;
    pub const L_TK_COUNT: luakit_token_t = 35;
    pub const L_TK_COOKIES_STORAGE: luakit_token_t = 34;
    pub const L_TK_CONFPATH: luakit_token_t = 33;
    pub const L_TK_CONFIG_DIR: luakit_token_t = 32;
    pub const L_TK_CLOSE_INSPECTOR: luakit_token_t = 31;
    pub const L_TK_CLIPBOARD: luakit_token_t = 30;
    pub const L_TK_CLIENT_RECTS: luakit_token_t = 29;
    pub const L_TK_CLICK: luakit_token_t = 28;
    pub const L_TK_CLEAR_SEARCH: luakit_token_t = 27;
    pub const L_TK_CLEAR: luakit_token_t = 26;
    pub const L_TK_CHILD_COUNT: luakit_token_t = 25;
    pub const L_TK_CHILDREN: luakit_token_t = 24;
    pub const L_TK_CHILD: luakit_token_t = 23;
    pub const L_TK_CHECKED: luakit_token_t = 22;
    pub const L_TK_CERTIFICATE: luakit_token_t = 21;
    pub const L_TK_CENTER: luakit_token_t = 20;
    pub const L_TK_CAN_GO_FORWARD: luakit_token_t = 19;
    pub const L_TK_CAN_GO_BACK: luakit_token_t = 18;
    pub const L_TK_CAN_FOCUS: luakit_token_t = 17;
    pub const L_TK_CACHE_DIR: luakit_token_t = 16;
    pub const L_TK_BOTTOM: luakit_token_t = 15;
    pub const L_TK_BODY: luakit_token_t = 14;
    pub const L_TK_BG: luakit_token_t = 13;
    pub const L_TK_BASELINE: luakit_token_t = 12;
    pub const L_TK_AUTO_LOAD_IMAGES: luakit_token_t = 11;
    pub const L_TK_ATTR: luakit_token_t = 10;
    pub const L_TK_APPEND: luakit_token_t = 9;
    pub const L_TK_ALLOW_UNIVERSAL_ACCESS_FROM_FILE_URLS: luakit_token_t = 8;
    pub const L_TK_ALLOW_OVERWRITE: luakit_token_t = 7;
    pub const L_TK_ALLOW_MODAL_DIALOGS: luakit_token_t = 6;
    pub const L_TK_ALLOW_FILE_ACCESS_FROM_FILE_URLS: luakit_token_t = 5;
    pub const L_TK_ALLOW_CERTIFICATE: luakit_token_t = 4;
    pub const L_TK_ALIGN: luakit_token_t = 3;
    pub const L_TK_ADD_EVENT_LISTENER: luakit_token_t = 2;
    pub const L_TK_ACCEPT_POLICY: luakit_token_t = 1;
    pub const L_TK_UNKNOWN: luakit_token_t = 0;
    use super::gtypes_h::gchar;
    unsafe extern "C" {
        pub fn l_tokenize(_: *const gchar) -> luakit_token_t;
    }
}
pub mod signal_h {
    pub type signal_t = GTree;
    use super::gtree_h::GTree;
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
    unsafe extern "C" {
        pub fn _log(lvl: log_level_t, _: *const gchar, _: *const gchar, _: ...);
    }
}
pub mod util_h {
    pub type LuakitError = std::ffi::c_uint;
    pub const LUAKIT_ERROR_TLS: LuakitError = 0;
    // pub const LUAKIT_ERROR: GQuark = luakit_error_quark();
    use super::gtypes_h::{gchar, gboolean};
    use super::lua_h::lua_State;
    use super::gquark_h::GQuark;
    unsafe extern "C" {
        pub fn file_exists(_: *const gchar) -> gboolean;
        pub fn luaH_callerinfo(_: *mut lua_State) -> *mut gchar;
        pub fn luakit_error_quark() -> GQuark;
    }
}
pub mod luaclass_h {
    pub type lua_class_property_array_t = GHashTable;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct lua_object_t {
        pub signals: *mut signal_t,
    }
    pub type lua_class_allocator_t = Option::<
        unsafe extern "C" fn(*mut lua_State) -> *mut lua_object_t,
    >;
    pub type lua_class_propfunc_t = Option::<
        unsafe extern "C" fn(*mut lua_State, *mut lua_object_t) -> gint,
    >;
    #[derive(Copy, Clone)]
    #[repr(C)]
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
    unsafe extern "C" {
        pub fn luaH_class_emit_signal(
            _: *mut lua_State,
            _: *mut lua_class_t,
            name: *const gchar,
            nargs: gint,
            nret: gint,
        ) -> gint;
        pub fn luaH_checkudata(
            _: *mut lua_State,
            _: gint,
            _: *mut lua_class_t,
        ) -> gpointer;
        pub fn luaH_toudata(
            L: *mut lua_State,
            ud: gint,
            _: *mut lua_class_t,
        ) -> gpointer;
    }
}
pub mod ipc_h {
    pub type ipc_endpoint_t = _ipc_endpoint_t;
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
    pub type ipc_recv_state_t = _ipc_recv_state_t;
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
    pub type ipc_header_t = _ipc_header_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _ipc_header_t {
        pub length: guint,
        pub type_0: ipc_type_t,
    }
    pub type ipc_type_t = std::ffi::c_uint;
    pub const IPC_TYPE_crash: ipc_type_t = 128;
    pub const IPC_TYPE_page_created: ipc_type_t = 64;
    pub const IPC_TYPE_log: ipc_type_t = 32;
    pub const IPC_TYPE_eval_js: ipc_type_t = 16;
    pub const IPC_TYPE_extension_init: ipc_type_t = 8;
    pub const IPC_TYPE_scroll: ipc_type_t = 4;
    pub const IPC_TYPE_lua_ipc: ipc_type_t = 2;
    pub const IPC_TYPE_lua_require_module: ipc_type_t = 1;
    pub type ipc_endpoint_status_t = std::ffi::c_uint;
    pub const IPC_ENDPOINT_FREED: ipc_endpoint_status_t = 2;
    pub const IPC_ENDPOINT_CONNECTED: ipc_endpoint_status_t = 1;
    pub const IPC_ENDPOINT_DISCONNECTED: ipc_endpoint_status_t = 0;
    pub type ipc_scroll_subtype_t = std::ffi::c_uint;
    pub const IPC_SCROLL_TYPE_scroll: ipc_scroll_subtype_t = 2;
    pub const IPC_SCROLL_TYPE_winresize: ipc_scroll_subtype_t = 1;
    pub const IPC_SCROLL_TYPE_docresize: ipc_scroll_subtype_t = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _ipc_scroll_t {
        pub h: gint,
        pub v: gint,
        pub page_id: guint64,
        pub subtype: ipc_scroll_subtype_t,
    }
    pub type ipc_scroll_t = _ipc_scroll_t;
    use super::gtypes_h::{gchar, gint, gboolean, guint, gpointer};
    use super::giochannel_h::GIOChannel;
    use super::gqueue_h::GQueue;
    use super::garray_h::GPtrArray;
    use super::glibconfig_h::{gsize, guint64};
    use super::lua_h::lua_State;
    unsafe extern "C" {
        pub fn ipc_endpoint_new(name: *const gchar) -> *mut ipc_endpoint_t;
        pub fn ipc_endpoint_replace(
            orig: *mut ipc_endpoint_t,
            new: *mut ipc_endpoint_t,
        ) -> *mut ipc_endpoint_t;
        pub fn ipc_endpoint_decref(ipc: *mut ipc_endpoint_t);
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
pub mod auth_c {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct LuakitAuthData {
        pub request: *mut WebKitAuthenticationRequest,
        pub w: *mut widget_t,
        pub login_entry: *mut GtkWidget,
        pub password_entry: *mut GtkWidget,
        pub checkbutton: *mut GtkWidget,
    }
    pub unsafe extern "C" fn free_auth_data(auth_data: *mut LuakitAuthData) {
        g_object_unref((*auth_data).request as gpointer);
        g_slice_free1(
            ::core::mem::size_of::<LuakitAuthData>() as std::ffi::c_ulong,
            auth_data as gpointer,
        );
    }
    pub unsafe extern "C" fn luakit_store_password(
        auth_data: *mut LuakitAuthData,
        login: *const gchar,
        password: *const gchar,
    ) {
        let L = common.L;
        let uri = webkit_web_view_get_uri(
            g_type_check_instance_cast(
                (*(*auth_data).w).widget as *mut GTypeInstance,
                webkit_web_view_get_type(),
            ) as *mut std::ffi::c_void as *mut WebKitWebView,
        );
        luaH_object_push(L, (*(*auth_data).w).ref_0);
        lua_pushstring(L, uri);
        lua_pushstring(L, login);
        lua_pushstring(L, password);
        luaH_object_emit_signal(
            L,
            -(4 as std::ffi::c_int),
            b"store-password\0" as *const u8 as *const std::ffi::c_char,
            3 as std::ffi::c_int,
            0 as std::ffi::c_int,
        );
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    }
    pub unsafe extern "C" fn luakit_find_password(
        auth_data: *mut LuakitAuthData,
        login: *mut *const gchar,
        password: *mut *const gchar,
    ) {
        let L = common.L;
        let uri = webkit_web_view_get_uri(
            g_type_check_instance_cast(
                (*(*auth_data).w).widget as *mut GTypeInstance,
                webkit_web_view_get_type(),
            ) as *mut std::ffi::c_void as *mut WebKitWebView,
        );
        luaH_object_push(L, (*(*auth_data).w).ref_0);
        lua_pushstring(L, uri);
        let ret = luaH_object_emit_signal(
            L,
            -(2 as std::ffi::c_int),
            b"store-password\0" as *const u8 as *const std::ffi::c_char,
            1 as std::ffi::c_int,
            LUA_MULTRET,
        );
        if ret >= 2 as std::ffi::c_int {
            *password = luaL_checklstring(
                L,
                -(1 as std::ffi::c_int),
                NULL_0 as *mut size_t,
            );
            *login = luaL_checklstring(
                L,
                -(2 as std::ffi::c_int),
                NULL_0 as *mut size_t,
            );
        }
        lua_settop(L, -(1 as std::ffi::c_int + ret) - 1 as std::ffi::c_int);
    }
    pub unsafe extern "C" fn response_callback(
        mut dialog: *mut GtkDialog,
        mut response_id: gint,
        mut auth_data: *mut LuakitAuthData,
    ) {
        let mut login = 0 as *const gchar;
        let mut password = 0 as *const gchar;
        let mut store_password: gboolean = 0;
        let mut credential = 0 as *mut WebKitCredential;
        match response_id {
            -5 => {
                login = gtk_entry_get_text(
                    g_type_check_instance_cast(
                        (*auth_data).login_entry as *mut GTypeInstance,
                        gtk_entry_get_type(),
                    ) as *mut std::ffi::c_void as *mut GtkEntry,
                );
                password = gtk_entry_get_text(
                    g_type_check_instance_cast(
                        (*auth_data).password_entry as *mut GTypeInstance,
                        gtk_entry_get_type(),
                    ) as *mut std::ffi::c_void as *mut GtkEntry,
                );
                credential = webkit_credential_new(
                    login,
                    password,
                    WEBKIT_CREDENTIAL_PERSISTENCE_NONE,
                );
                webkit_authentication_request_authenticate(
                    (*auth_data).request,
                    credential,
                );
                webkit_credential_free(credential);
                store_password = gtk_toggle_button_get_active(
                    g_type_check_instance_cast(
                        (*auth_data).checkbutton as *mut GTypeInstance,
                        gtk_toggle_button_get_type(),
                    ) as *mut std::ffi::c_void as *mut GtkToggleButton,
                );
                if store_password != 0 {
                    luakit_store_password(auth_data, login, password);
                }
            }
            _ => {}
        }
        free_auth_data(auth_data);
        gtk_widget_destroy(
            g_type_check_instance_cast(
                dialog as *mut GTypeInstance,
                gtk_widget_get_type(),
            ) as *mut std::ffi::c_void as *mut GtkWidget,
        );
    }
    pub unsafe extern "C" fn table_add_entry(
        mut table: *mut GtkWidget,
        mut row: gint,
        mut label_text: *const gchar,
        mut value: *const gchar,
        mut UNUSED_user_data: gpointer,
    ) -> *mut GtkWidget {
        let mut label = gtk_label_new(label_text);
        let mut align = {
            let mut init = GValue {
                g_type: 0 as std::ffi::c_int as GType,
                data: [
                    C2RustUnnamed {
                        v_int: 0 as std::ffi::c_int,
                    },
                    C2RustUnnamed { v_int: 0 },
                ],
            };
            init
        };
        g_value_init(&mut align, G_TYPE_ENUM as GType);
        g_value_set_int(&mut align, GTK_ALIGN_CENTER as std::ffi::c_int);
        g_object_set_property(
            g_type_check_instance_cast(
                label as *mut GTypeInstance,
                ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
            ) as *mut std::ffi::c_void as *mut GObject,
            b"halign\0" as *const u8 as *const std::ffi::c_char,
            &mut align,
        );
        gtk_widget_set_vexpand(
            g_type_check_instance_cast(
                label as *mut GTypeInstance,
                gtk_widget_get_type(),
            ) as *mut std::ffi::c_void as *mut GtkWidget,
            TRUE,
        );
        let mut entry = gtk_entry_new();
        gtk_entry_set_activates_default(
            g_type_check_instance_cast(entry as *mut GTypeInstance, gtk_entry_get_type())
                as *mut std::ffi::c_void as *mut GtkEntry,
            TRUE,
        );
        if !value.is_null() {
            gtk_entry_set_text(
                g_type_check_instance_cast(
                    entry as *mut GTypeInstance,
                    gtk_entry_get_type(),
                ) as *mut std::ffi::c_void as *mut GtkEntry,
                value,
            );
        }
        gtk_grid_attach(
            g_type_check_instance_cast(table as *mut GTypeInstance, gtk_grid_get_type())
                as *mut std::ffi::c_void as *mut GtkGrid,
            label,
            0 as std::ffi::c_int,
            row,
            1 as std::ffi::c_int,
            1 as std::ffi::c_int,
        );
        gtk_grid_attach(
            g_type_check_instance_cast(table as *mut GTypeInstance, gtk_grid_get_type())
                as *mut std::ffi::c_void as *mut GtkGrid,
            entry,
            1 as std::ffi::c_int,
            row,
            1 as std::ffi::c_int,
            1 as std::ffi::c_int,
        );
        gtk_widget_set_halign(label, GTK_ALIGN_FILL);
        gtk_widget_set_valign(label, GTK_ALIGN_FILL);
        gtk_widget_set_halign(entry, GTK_ALIGN_FILL);
        gtk_widget_set_valign(entry, GTK_ALIGN_FILL);
        gtk_widget_set_vexpand(label, TRUE);
        gtk_widget_set_vexpand(entry, TRUE);
        return entry;
    }
    pub unsafe extern "C" fn show_auth_dialog(
        mut auth_data: *mut LuakitAuthData,
        mut login: *const std::ffi::c_char,
        mut password: *const std::ffi::c_char,
    ) {
        let mut widget = gtk_dialog_new();
        let mut window = g_type_check_instance_cast(
            widget as *mut GTypeInstance,
            gtk_window_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkWindow;
        let mut dialog = g_type_check_instance_cast(
            widget as *mut GTypeInstance,
            gtk_dialog_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkDialog;
        gtk_dialog_add_buttons(
            dialog,
            b"_Cancel\0" as *const u8 as *const std::ffi::c_char,
            GTK_RESPONSE_CANCEL as std::ffi::c_int,
            b"_OK\0" as *const u8 as *const std::ffi::c_char,
            GTK_RESPONSE_OK as std::ffi::c_int,
            NULL_0 as *mut std::ffi::c_void,
        );
        gtk_container_set_border_width(
            g_type_check_instance_cast(
                dialog as *mut GTypeInstance,
                gtk_container_get_type(),
            ) as *mut std::ffi::c_void as *mut GtkContainer,
            5 as std::ffi::c_int as guint,
        );
        let mut button_spacing = {
            let mut init = GValue {
                g_type: 0 as std::ffi::c_int as GType,
                data: [
                    C2RustUnnamed {
                        v_int: 0 as std::ffi::c_int,
                    },
                    C2RustUnnamed { v_int: 0 },
                ],
            };
            init
        };
        g_value_init(&mut button_spacing, G_TYPE_INT as GType);
        g_value_set_int(&mut button_spacing, 6 as std::ffi::c_int);
        g_object_set_property(
            g_type_check_instance_cast(
                dialog as *mut GTypeInstance,
                ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
            ) as *mut std::ffi::c_void as *mut GObject,
            b"button-spacing\0" as *const u8 as *const std::ffi::c_char,
            &mut button_spacing,
        );
        gtk_window_set_resizable(window, FALSE);
        gtk_window_set_title(window, b"\0" as *const u8 as *const std::ffi::c_char);
        gtk_window_set_icon_name(
            window,
            b"dialog-password\0" as *const u8 as *const std::ffi::c_char,
        );
        gtk_dialog_set_default_response(dialog, GTK_RESPONSE_OK as std::ffi::c_int);
        let mut hbox = gtk_grid_new();
        let mut margin = {
            let mut init = GValue {
                g_type: 0 as std::ffi::c_int as GType,
                data: [
                    C2RustUnnamed {
                        v_int: 0 as std::ffi::c_int,
                    },
                    C2RustUnnamed { v_int: 0 },
                ],
            };
            init
        };
        g_value_init(&mut margin, G_TYPE_INT as GType);
        g_value_set_int(&mut margin, 5 as std::ffi::c_int);
        g_object_set_property(
            g_type_check_instance_cast(
                hbox as *mut GTypeInstance,
                ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
            ) as *mut std::ffi::c_void as *mut GObject,
            b"margin\0" as *const u8 as *const std::ffi::c_char,
            &mut margin,
        );
        gtk_grid_set_column_spacing(
            g_type_check_instance_cast(hbox as *mut GTypeInstance, gtk_grid_get_type())
                as *mut std::ffi::c_void as *mut GtkGrid,
            12 as std::ffi::c_int as guint,
        );
        gtk_box_pack_start(
            g_type_check_instance_cast(
                gtk_dialog_get_content_area(dialog) as *mut GTypeInstance,
                gtk_box_get_type(),
            ) as *mut std::ffi::c_void as *mut GtkBox,
            hbox,
            TRUE,
            TRUE,
            0 as std::ffi::c_int as guint,
        );
        let mut icon = gtk_image_new_from_icon_name(
            b"dialog-password\0" as *const u8 as *const std::ffi::c_char,
            GTK_ICON_SIZE_DIALOG,
        );
        let mut align = {
            let mut init = GValue {
                g_type: 0 as std::ffi::c_int as GType,
                data: [
                    C2RustUnnamed {
                        v_int: 0 as std::ffi::c_int,
                    },
                    C2RustUnnamed { v_int: 0 },
                ],
            };
            init
        };
        g_value_init(&mut align, G_TYPE_ENUM as GType);
        g_value_set_int(&mut align, GTK_ALIGN_CENTER as std::ffi::c_int);
        g_object_set_property(
            g_type_check_instance_cast(
                hbox as *mut GTypeInstance,
                ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
            ) as *mut std::ffi::c_void as *mut GObject,
            b"halign\0" as *const u8 as *const std::ffi::c_char,
            &mut align,
        );
        gtk_grid_attach(
            g_type_check_instance_cast(hbox as *mut GTypeInstance, gtk_grid_get_type())
                as *mut std::ffi::c_void as *mut GtkGrid,
            icon,
            0 as std::ffi::c_int,
            0 as std::ffi::c_int,
            1 as std::ffi::c_int,
            2 as std::ffi::c_int,
        );
        gtk_grid_set_row_spacing(
            g_type_check_instance_cast(hbox as *mut GTypeInstance, gtk_grid_get_type())
                as *mut std::ffi::c_void as *mut GtkGrid,
            6 as std::ffi::c_int as guint,
        );
        let mut msg = g_strdup_printf(
            b"A username and password are being requested by the site %s\0" as *const u8
                as *const std::ffi::c_char,
            webkit_authentication_request_get_host((*auth_data).request),
        );
        let mut msg_label = gtk_label_new(msg);
        g_free(msg as gpointer);
        g_object_set_property(
            g_type_check_instance_cast(
                msg_label as *mut GTypeInstance,
                ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
            ) as *mut std::ffi::c_void as *mut GObject,
            b"halign\0" as *const u8 as *const std::ffi::c_char,
            &mut align,
        );
        gtk_label_set_line_wrap(
            g_type_check_instance_cast(
                msg_label as *mut GTypeInstance,
                gtk_label_get_type(),
            ) as *mut std::ffi::c_void as *mut GtkLabel,
            TRUE,
        );
        let mut max_width_chars = {
            let mut init = GValue {
                g_type: 0 as std::ffi::c_int as GType,
                data: [
                    C2RustUnnamed {
                        v_int: 0 as std::ffi::c_int,
                    },
                    C2RustUnnamed { v_int: 0 },
                ],
            };
            init
        };
        g_value_init(&mut max_width_chars, G_TYPE_INT as GType);
        g_value_set_int(&mut max_width_chars, 32 as std::ffi::c_int);
        g_object_set_property(
            g_type_check_instance_cast(
                msg_label as *mut GTypeInstance,
                ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
            ) as *mut std::ffi::c_void as *mut GObject,
            b"max-width-chars\0" as *const u8 as *const std::ffi::c_char,
            &mut max_width_chars,
        );
        gtk_grid_attach_next_to(
            g_type_check_instance_cast(hbox as *mut GTypeInstance, gtk_grid_get_type())
                as *mut std::ffi::c_void as *mut GtkGrid,
            g_type_check_instance_cast(
                msg_label as *mut GTypeInstance,
                gtk_widget_get_type(),
            ) as *mut std::ffi::c_void as *mut GtkWidget,
            icon,
            GTK_POS_RIGHT,
            1 as std::ffi::c_int,
            1 as std::ffi::c_int,
        );
        gtk_widget_set_hexpand(
            g_type_check_instance_cast(
                msg_label as *mut GTypeInstance,
                gtk_widget_get_type(),
            ) as *mut std::ffi::c_void as *mut GtkWidget,
            FALSE,
        );
        gtk_widget_set_vexpand(
            g_type_check_instance_cast(
                msg_label as *mut GTypeInstance,
                gtk_widget_get_type(),
            ) as *mut std::ffi::c_void as *mut GtkWidget,
            TRUE,
        );
        let mut table = gtk_grid_new();
        gtk_grid_attach_next_to(
            g_type_check_instance_cast(hbox as *mut GTypeInstance, gtk_grid_get_type())
                as *mut std::ffi::c_void as *mut GtkGrid,
            table,
            g_type_check_instance_cast(
                msg_label as *mut GTypeInstance,
                gtk_widget_get_type(),
            ) as *mut std::ffi::c_void as *mut GtkWidget,
            GTK_POS_BOTTOM,
            1 as std::ffi::c_int,
            1 as std::ffi::c_int,
        );
        gtk_grid_set_column_homogeneous(
            g_type_check_instance_cast(table as *mut GTypeInstance, gtk_grid_get_type())
                as *mut std::ffi::c_void as *mut GtkGrid,
            FALSE,
        );
        gtk_grid_set_row_homogeneous(
            g_type_check_instance_cast(table as *mut GTypeInstance, gtk_grid_get_type())
                as *mut std::ffi::c_void as *mut GtkGrid,
            FALSE,
        );
        gtk_grid_set_column_spacing(
            g_type_check_instance_cast(table as *mut GTypeInstance, gtk_grid_get_type())
                as *mut std::ffi::c_void as *mut GtkGrid,
            12 as std::ffi::c_int as guint,
        );
        gtk_grid_set_row_spacing(
            g_type_check_instance_cast(table as *mut GTypeInstance, gtk_grid_get_type())
                as *mut std::ffi::c_void as *mut GtkGrid,
            6 as std::ffi::c_int as guint,
        );
        (*auth_data)
            .login_entry = table_add_entry(
            table,
            0 as std::ffi::c_int,
            b"Username:\0" as *const u8 as *const std::ffi::c_char,
            login,
            NULL_0 as *mut std::ffi::c_void,
        );
        (*auth_data)
            .password_entry = table_add_entry(
            table,
            1 as std::ffi::c_int,
            b"Password:\0" as *const u8 as *const std::ffi::c_char,
            password,
            NULL_0 as *mut std::ffi::c_void,
        );
        gtk_entry_set_visibility(
            g_type_check_instance_cast(
                (*auth_data).password_entry as *mut GTypeInstance,
                gtk_entry_get_type(),
            ) as *mut std::ffi::c_void as *mut GtkEntry,
            FALSE,
        );
        let mut checkbutton = gtk_check_button_new_with_label(
            b"Store password\0" as *const u8 as *const std::ffi::c_char,
        );
        gtk_label_set_line_wrap(
            g_type_check_instance_cast(
                gtk_bin_get_child(
                    g_type_check_instance_cast(
                        checkbutton as *mut GTypeInstance,
                        gtk_bin_get_type(),
                    ) as *mut std::ffi::c_void as *mut GtkBin,
                ) as *mut GTypeInstance,
                gtk_label_get_type(),
            ) as *mut std::ffi::c_void as *mut GtkLabel,
            TRUE,
        );
        gtk_grid_attach_next_to(
            g_type_check_instance_cast(hbox as *mut GTypeInstance, gtk_grid_get_type())
                as *mut std::ffi::c_void as *mut GtkGrid,
            checkbutton,
            table,
            GTK_POS_BOTTOM,
            1 as std::ffi::c_int,
            1 as std::ffi::c_int,
        );
        (*auth_data).checkbutton = checkbutton;
        g_signal_connect_data(
            dialog as gpointer,
            b"response\0" as *const u8 as *const std::ffi::c_char,
            ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut GtkDialog, gint, *mut LuakitAuthData) -> (),
                >,
                GCallback,
            >(
                Some(
                    response_callback
                        as unsafe extern "C" fn(
                            *mut GtkDialog,
                            gint,
                            *mut LuakitAuthData,
                        ) -> (),
                ),
            ),
            auth_data as gpointer,
            ::core::mem::transmute::<
                libc::intptr_t,
                GClosureNotify,
            >(NULL_0 as libc::intptr_t),
            G_CONNECT_DEFAULT,
        );
        gtk_widget_show_all(widget);
    }
    pub unsafe extern "C" fn session_authenticate(
        mut UNUSED_web_view: *mut WebKitWebView,
        mut request: *mut WebKitAuthenticationRequest,
        mut w: *mut widget_t,
    ) -> gboolean {
        g_object_ref(request as gpointer);
        let mut auth_data = g_slice_alloc(
            ::core::mem::size_of::<LuakitAuthData>() as std::ffi::c_ulong,
        ) as *mut LuakitAuthData;
        (*auth_data).request = request;
        (*auth_data).w = w;
        let mut login = NULL_0 as *const gchar;
        let mut password = NULL_0 as *const gchar;
        luakit_find_password(auth_data, &mut login, &mut password);
        show_auth_dialog(auth_data, login, password);
        return TRUE;
    }
    use super::WebKitAuthenticationRequest_h::{
        WebKitAuthenticationRequest, webkit_authentication_request_authenticate,
        webkit_authentication_request_get_host,
    };
    use super::widget_h::widget_t;
    use super::gtktypes_h::{GtkWidget, GtkWindow};
    use super::gobject_h::{g_object_unref, g_object_set_property, GObject, g_object_ref};
    use super::gtypes_h::{gpointer, gchar, gint, gboolean, guint};
    use super::gslice_h::{g_slice_free1, g_slice_alloc};
    use super::common_h::common;
    use super::lua_h::{lua_State, lua_pushstring, lua_settop, LUA_MULTRET};
    use super::WebKitWebView_h::{webkit_web_view_get_uri, webkit_web_view_get_type};
    use super::gtype_h::{
        g_type_check_instance_cast, GTypeInstance, GType, GValue, G_TYPE_ENUM, G_TYPE_INT,
    };
    use super::WebKitDownload_h::WebKitWebView;
    use super::luaobject_h::{luaH_object_push, luaH_object_emit_signal};
    use super::lauxlib_h::luaL_checklstring;
    use super::__stddef_null_h::NULL_0;
    use super::__stddef_size_t_h::size_t;
    use super::gtkdialog_h::{
        GtkDialog, gtk_dialog_new, gtk_dialog_get_type, gtk_dialog_add_buttons,
        GTK_RESPONSE_CANCEL, GTK_RESPONSE_OK, gtk_dialog_set_default_response,
        gtk_dialog_get_content_area,
    };
    use super::WebKitCredential_h::{
        WebKitCredential, webkit_credential_new, WEBKIT_CREDENTIAL_PERSISTENCE_NONE,
        WebKitCredentialPersistence, webkit_credential_free,
    };
    use super::gtkentry_h::{
        gtk_entry_get_text, gtk_entry_get_type, GtkEntry, gtk_entry_new,
        gtk_entry_set_activates_default, gtk_entry_set_text, gtk_entry_set_visibility,
    };
    use super::gtktogglebutton_h::{
        gtk_toggle_button_get_active, gtk_toggle_button_get_type, GtkToggleButton,
    };
    use super::gtkwidget_h::{
        gtk_widget_destroy, gtk_widget_get_type, gtk_widget_set_vexpand,
        gtk_widget_set_halign, gtk_widget_set_valign, gtk_widget_set_hexpand,
        gtk_widget_show_all,
    };
    use super::gtklabel_h::{
        gtk_label_new, gtk_label_set_line_wrap, gtk_label_get_type, GtkLabel,
    };
    use super::gvalue_h::{C2RustUnnamed, g_value_init};
    use super::gvaluetypes_h::g_value_set_int;
    use super::gtkenums_h::{
        GTK_ALIGN_CENTER, GTK_ALIGN_FILL, GtkAlign, GTK_ICON_SIZE_DIALOG, GtkIconSize,
        GTK_POS_RIGHT, GtkPositionType, GTK_POS_BOTTOM,
    };
    use super::gmacros_h::{TRUE, FALSE};
    use super::gtkgrid_h::{
        gtk_grid_attach, gtk_grid_get_type, GtkGrid, gtk_grid_new,
        gtk_grid_set_column_spacing, gtk_grid_set_row_spacing, gtk_grid_attach_next_to,
        gtk_grid_set_column_homogeneous, gtk_grid_set_row_homogeneous,
    };
    use super::gtkwindow_h::{
        gtk_window_get_type, gtk_window_set_resizable, gtk_window_set_title,
        gtk_window_set_icon_name,
    };
    use super::gtkcontainer_h::{
        gtk_container_set_border_width, gtk_container_get_type, GtkContainer,
    };
    use super::gtkbox_h::{gtk_box_pack_start, gtk_box_get_type, GtkBox};
    use super::gtkimage_h::gtk_image_new_from_icon_name;
    use super::gstrfuncs_h::g_strdup_printf;
    use super::gmem_h::g_free;
    use super::gtkcheckbutton_h::gtk_check_button_new_with_label;
    use super::gtkbin_h::{gtk_bin_get_child, gtk_bin_get_type, GtkBin};
    use super::gsignal_h::{g_signal_connect_data, GConnectFlags, G_CONNECT_DEFAULT};
    use super::gclosure_h::{GCallback, GClosureNotify};
}
pub mod stylesheet_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct lstylesheet_t {
        pub signals: *mut signal_t,
        pub stylesheet: *mut WebKitUserStyleSheet,
        pub source: *mut gchar,
    }
    use super::signal_h::signal_t;
    use super::WebKitUserContent_h::WebKitUserStyleSheet;
    use super::gtypes_h::{gchar, gint, gpointer};
    use super::lua_h::lua_State;
    unsafe extern "C" {
        pub fn luaH_checkstylesheet(L: *mut lua_State, idx: gint) -> gpointer;
    }
}
pub mod property_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct property_t {
        pub tok: luakit_token_t,
        pub name: *const gchar,
        pub type_0: property_value_t,
        pub writable: gboolean,
    }
    pub type property_value_t = std::ffi::c_uint;
    pub const URI: property_value_t = 5;
    pub const INT: property_value_t = 4;
    pub const FLOAT: property_value_t = 3;
    pub const DOUBLE: property_value_t = 2;
    pub const CHAR: property_value_t = 1;
    pub const BOOL: property_value_t = 0;
    use super::tokenize_h::luakit_token_t;
    use super::gtypes_h::{gchar, gboolean, gint};
    use super::lua_h::lua_State;
    use super::gobject_h::GObject;
    unsafe extern "C" {
        pub fn luaH_gobject_index(
            _: *mut lua_State,
            _: *mut property_t,
            _: luakit_token_t,
            _: *mut GObject,
        ) -> gint;
        pub fn luaH_gobject_newindex(
            _: *mut lua_State,
            _: *mut property_t,
            _: luakit_token_t,
            _: gint,
            _: *mut GObject,
        ) -> gboolean;
    }
}
pub mod string_h {
    unsafe extern "C" {
        pub fn memcpy(
            _: *mut std::ffi::c_void,
            _: *const std::ffi::c_void,
            _: std::ffi::c_ulong,
        ) -> *mut std::ffi::c_void;
        pub fn strcmp(
            _: *const std::ffi::c_char,
            _: *const std::ffi::c_char,
        ) -> std::ffi::c_int;
        pub fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    }
}
pub mod gbytes_h {
    use super::gtypes_h::gconstpointer;
    use super::glibconfig_h::gsize;
    use super::garray_h::GBytes;
    unsafe extern "C" {
        pub fn g_bytes_new(data: gconstpointer, size: gsize) -> *mut GBytes;
        pub fn g_bytes_get_data(bytes: *mut GBytes, size: *mut gsize) -> gconstpointer;
        pub fn g_bytes_unref(bytes: *mut GBytes);
    }
}
pub mod gfileutils_h {
    use super::gtypes_h::{gchar, gboolean};
    unsafe extern "C" {
        pub fn g_build_filename(first_element: *const gchar, _: ...) -> *mut gchar;
        pub fn g_path_is_absolute(file_name: *const gchar) -> gboolean;
        pub fn g_get_current_dir() -> *mut gchar;
    }
}
pub mod gmem_h {
    use super::gtypes_h::gpointer;
    use super::glibconfig_h::gsize;
    unsafe extern "C" {
        pub fn g_free(mem: gpointer);
        pub fn g_malloc(n_bytes: gsize) -> gpointer;
        pub fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
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
            let len = (strlen(str))
                .wrapping_add(1 as std::ffi::c_int as std::ffi::c_ulong);
            let mut dup_str = g_malloc(len) as *mut std::ffi::c_char;
            return memcpy(
                dup_str as *mut std::ffi::c_void,
                str as *const std::ffi::c_void,
                len,
            ) as *mut std::ffi::c_char;
        }
        return g_strdup(str);
    }
    use super::gtypes_h::gchar;
    use super::__stddef_null_h::NULL;
    use super::string_h::{strlen, memcpy};
    use super::__stddef_size_t_h::size_t;
    use super::gmem_h::g_malloc;
    unsafe extern "C" {
        pub fn g_strrstr(haystack: *const gchar, needle: *const gchar) -> *mut gchar;
        pub fn g_strdup(str: *const gchar) -> *mut gchar;
        pub fn g_strdup_printf(format: *const gchar, _: ...) -> *mut gchar;
        pub fn g_strconcat(string1: *const gchar, _: ...) -> *mut gchar;
    }
}
pub mod gmessages_h {
    pub const G_LOG_DOMAIN: std::ffi::c_int = 0 as std::ffi::c_int;
}
pub mod gslice_h {
    use super::glibconfig_h::gsize;
    use super::gtypes_h::gpointer;
    unsafe extern "C" {
        pub fn g_slice_alloc(block_size: gsize) -> gpointer;
        pub fn g_slice_alloc0(block_size: gsize) -> gpointer;
        pub fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    }
}
pub mod gtestutils_h {
    use super::glibconfig_h::guint64;
    unsafe extern "C" {
        pub fn g_strcmp0(
            str1: *const std::ffi::c_char,
            str2: *const std::ffi::c_char,
        ) -> std::ffi::c_int;
        pub fn g_assertion_message_expr(
            domain: *const std::ffi::c_char,
            file: *const std::ffi::c_char,
            line: std::ffi::c_int,
            func: *const std::ffi::c_char,
            expr: *const std::ffi::c_char,
        ) -> !;
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
pub mod gvaluetypes_h {
    use super::gtype_h::GValue;
    use super::gtypes_h::gint;
    unsafe extern "C" {
        pub fn g_value_set_int(value: *mut GValue, v_int: gint);
    }
}
pub mod gfile_h {
    use super::giotypes_h::GFile;
    unsafe extern "C" {
        pub fn g_file_new_for_path(path: *const std::ffi::c_char) -> *mut GFile;
    }
}
pub mod gtkimage_h {
    use super::gtypes_h::gchar;
    use super::gtkenums_h::{GtkIconSize, GTK_ICON_SIZE_INVALID};
    use super::gtktypes_h::GtkWidget;
    unsafe extern "C" {
        pub fn gtk_image_new_from_icon_name(
            icon_name: *const gchar,
            size: GtkIconSize,
        ) -> *mut GtkWidget;
    }
}
pub mod gtkcheckbutton_h {
    use super::gtypes_h::gchar;
    use super::gtktypes_h::GtkWidget;
    unsafe extern "C" {
        pub fn gtk_check_button_new_with_label(label: *const gchar) -> *mut GtkWidget;
    }
}
pub mod WebKitGeolocationPermissionRequest_h {
    use super::gtype_h::GType;
    unsafe extern "C" {
        pub fn webkit_geolocation_permission_request_get_type() -> GType;
    }
}
pub mod WebKitNotificationPermissionRequest_h {
    use super::gtype_h::GType;
    unsafe extern "C" {
        pub fn webkit_notification_permission_request_get_type() -> GType;
    }
}
pub mod lauxlib_h {
    use super::lua_h::{lua_State, lua_Number};
    use super::__stddef_size_t_h::size_t;
    unsafe extern "C" {
        pub fn luaL_typerror(
            L: *mut lua_State,
            narg: std::ffi::c_int,
            tname: *const std::ffi::c_char,
        ) -> std::ffi::c_int;
        pub fn luaL_argerror(
            L: *mut lua_State,
            numarg: std::ffi::c_int,
            extramsg: *const std::ffi::c_char,
        ) -> std::ffi::c_int;
        pub fn luaL_checklstring(
            L: *mut lua_State,
            numArg: std::ffi::c_int,
            l: *mut size_t,
        ) -> *const std::ffi::c_char;
        pub fn luaL_checknumber(
            L: *mut lua_State,
            numArg: std::ffi::c_int,
        ) -> lua_Number;
        pub fn luaL_error(
            L: *mut lua_State,
            fmt: *const std::ffi::c_char,
            _: ...
        ) -> std::ffi::c_int;
    }
}
pub mod luautil_h {
    use super::lua_h::lua_State;
    use super::gtypes_h::gint;
    use super::gerror_h::GError;
    unsafe extern "C" {
        pub fn luaH_dofunction_on_error(L: *mut lua_State) -> gint;
        pub fn luaH_push_gerror(L: *mut lua_State, error: *mut GError) -> gint;
    }
}
pub mod lualib_h {
    #[inline]
    pub unsafe extern "C" fn luaH_absindex(mut L: *mut lua_State, mut ud: gint) -> gint {
        return if ud >= 0 as std::ffi::c_int || ud <= LUA_REGISTRYINDEX {
            ud
        } else {
            lua_gettop(L) + ud + 1 as std::ffi::c_int
        };
    }
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
                lua_tolstring(L, -(1 as std::ffi::c_int), NULL_0 as *mut size_t),
            );
            lua_settop(L, -(2 as std::ffi::c_int) - 1 as std::ffi::c_int);
            return FALSE;
        }
        lua_remove(L, error_func_pos);
        return TRUE;
    }
    use super::lua_h::{
        lua_State, LUA_REGISTRYINDEX, lua_gettop, lua_insert, lua_pushcclosure,
        lua_pcall, lua_tolstring, lua_settop, lua_remove,
    };
    use super::gtypes_h::{gint, gboolean};
    use super::luautil_h::luaH_dofunction_on_error;
    use super::log_h::{_log, LOG_LEVEL_error, log_level_t};
    use super::__stddef_null_h::NULL_0;
    use super::__stddef_size_t_h::size_t;
    use super::gmacros_h::{FALSE, TRUE};
}
pub mod luaobject_h {
    #[inline]
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
        lua_rawget(L, LUA_REGISTRYINDEX);
    }
    #[inline]
    pub unsafe extern "C" fn luaH_object_ref(
        mut L: *mut lua_State,
        mut oud: gint,
    ) -> gpointer {
        luaH_object_registry_push(L);
        let mut p = luaH_object_incref(
            L,
            -(1 as std::ffi::c_int),
            if oud < 0 as std::ffi::c_int { oud - 1 as std::ffi::c_int } else { oud },
        );
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        return p;
    }
    #[inline]
    pub unsafe extern "C" fn luaH_object_unref(mut L: *mut lua_State, mut p: gpointer) {
        luaH_object_registry_push(L);
        luaH_object_decref(L, -(1 as std::ffi::c_int), p);
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    }
    #[inline]
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
        lua_State, lua_pushlstring, lua_rawget, LUA_REGISTRYINDEX, lua_settop,
        lua_pushlightuserdata, lua_remove,
    };
    use super::gtypes_h::{gint, gpointer, gchar};
    use super::tokenize_h::{luakit_token_t, L_TK_UNKNOWN};
    unsafe extern "C" {
        pub fn luaH_object_incref(L: *mut lua_State, tud: gint, oud: gint) -> gpointer;
        pub fn luaH_object_decref(L: *mut lua_State, tud: gint, oud: gpointer);
        pub fn luaH_object_emit_signal(
            L: *mut lua_State,
            oud: gint,
            name: *const gchar,
            nargs: gint,
            nret: gint,
        ) -> gint;
        pub fn luaH_object_property_signal(
            _: *mut lua_State,
            _: gint,
            _: luakit_token_t,
        ) -> gint;
    }
}
pub mod luah_h {
    #[inline]
    pub unsafe extern "C" fn luaH_checkboolean(
        mut L: *mut lua_State,
        mut n: gint,
    ) -> gboolean {
        if !(lua_type(L, n) == LUA_TBOOLEAN) {
            luaL_typerror(L, n, b"boolean\0" as *const u8 as *const std::ffi::c_char);
        }
        return lua_toboolean(L, n);
    }
    #[inline]
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
        lua_State, lua_type, LUA_TBOOLEAN, lua_toboolean, lua_pushstring, lua_rawget,
        LUA_TNIL, lua_settop,
    };
    use super::gtypes_h::{gint, gboolean, gchar};
    use super::lauxlib_h::luaL_typerror;
}
pub mod luakit_luah_h {
    use super::lua_h::lua_State;
    use super::gtypes_h::guint;
    unsafe extern "C" {
        pub fn luaH_modifier_table_push(_: *mut lua_State, _: guint);
    }
}
pub mod widgets_common_h {
    use super::gtktypes_h::GtkWidget;
    use super::gdkevents_h::{GdkEventFocus, GdkEventKey};
    use super::widget_h::widget_t;
    use super::gtypes_h::{gboolean, gint};
    use super::lua_h::lua_State;
    use super::gdktypes_h::GdkRectangle;
    unsafe extern "C" {
        pub fn focus_cb(
            _: *mut GtkWidget,
            _: *mut GdkEventFocus,
            _: *mut widget_t,
        ) -> gboolean;
        pub fn key_press_cb(
            _: *mut GtkWidget,
            _: *mut GdkEventKey,
            _: *mut widget_t,
        ) -> gboolean;
        pub fn luaH_widget_destroy(_: *mut lua_State) -> gint;
        pub fn luaH_widget_focus(_: *mut lua_State) -> gint;
        pub fn luaH_widget_get_children(_: *mut lua_State, _: *mut widget_t) -> gint;
        pub fn luaH_widget_hide(_: *mut lua_State) -> gint;
        pub fn luaH_widget_show(_: *mut lua_State) -> gint;
        pub fn luaH_widget_replace(_: *mut lua_State) -> gint;
        pub fn luaH_widget_send_key(_: *mut lua_State) -> gint;
        pub fn luaH_widget_get_parent(L: *mut lua_State, w: *mut widget_t) -> gint;
        pub fn luaH_widget_get_focused(L: *mut lua_State, _: *mut widget_t) -> gint;
        pub fn luaH_widget_get_visible(L: *mut lua_State, _: *mut widget_t) -> gint;
        pub fn luaH_widget_get_width(L: *mut lua_State, _: *mut widget_t) -> gint;
        pub fn luaH_widget_get_height(L: *mut lua_State, _: *mut widget_t) -> gint;
        pub fn luaH_widget_set_visible(L: *mut lua_State, _: *mut widget_t) -> gint;
        pub fn luaH_widget_set_tooltip(L: *mut lua_State, w: *mut widget_t) -> gint;
        pub fn luaH_widget_get_tooltip(L: *mut lua_State, w: *mut widget_t) -> gint;
        pub fn luaH_widget_set_min_size(L: *mut lua_State, w: *mut widget_t) -> gint;
        pub fn luaH_widget_get_min_size(L: *mut lua_State, w: *mut widget_t) -> gint;
        pub fn luaH_widget_set_align(L: *mut lua_State, w: *mut widget_t) -> gint;
        pub fn luaH_widget_get_align(L: *mut lua_State, w: *mut widget_t) -> gint;
        pub fn parent_set_cb(_: *mut GtkWidget, _: *mut GtkWidget, _: *mut widget_t);
        pub fn resize_cb(_: *mut GtkWidget, _: *mut GdkRectangle, _: *mut widget_t);
        pub fn destroy_cb(UNUSED_win: *mut GtkWidget, w: *mut widget_t);
    }
}
pub mod request_h {
    use super::lua_h::lua_State;
    use super::WebKitURISchemeRequest_h::WebKitURISchemeRequest;
    use super::gtypes_h::gint;
    unsafe extern "C" {
        pub fn luaH_request_push_uri_scheme_request(
            _: *mut lua_State,
            _: *mut WebKitURISchemeRequest,
        ) -> gint;
    }
}
pub mod web_context_h {
    use super::WebKitWebContext_h::WebKitWebContext;
    unsafe extern "C" {
        pub fn web_context_init_finish();
        pub fn web_context_get() -> *mut WebKitWebContext;
    }
}
pub mod luayield_h {
    use super::lua_h::lua_State;
    use super::gtypes_h::{gint, gboolean};
    unsafe extern "C" {
        pub fn luaH_yield_wrap_function(L: *mut lua_State);
        pub fn luaH_yield(L: *mut lua_State) -> std::ffi::c_int;
        pub fn luaH_resume(L: *mut lua_State, nret: gint) -> gboolean;
    }
}
pub mod gmacros_h {
    pub const FALSE: std::ffi::c_int = 0 as std::ffi::c_int;
    pub const TRUE: std::ffi::c_int = (FALSE == 0) as std::ffi::c_int;
}
pub mod __stddef_null_h {
    pub const NULL: std::ffi::c_int = 0 as std::ffi::c_int;
    pub const NULL_0: std::ffi::c_int = 0 as std::ffi::c_int;
}
pub mod internal {
    pub const __INT_MAX__: std::ffi::c_int = 2147483647 as std::ffi::c_int;
}
pub mod luaserialize_h {
    use super::lua_h::lua_State;
    use super::glibconfig_h::guint8;
    use super::gtypes_h::guint;
    unsafe extern "C" {
        pub fn lua_deserialize_range(
            L: *mut lua_State,
            in_0: *const guint8,
            length: guint,
        ) -> std::ffi::c_int;
    }
}
pub mod javascript_c {
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn run_javascript_finished(
        mut msg: *const guint8,
        mut length: guint,
    ) {
        let mut L = common.L;
        let mut top = lua_gettop(L);
        let mut n = lua_deserialize_range(L, msg, length);
        let mut __n1 = n as gint64;
        let mut __n2 = 2 as std::ffi::c_int as gint64;
        if !(__n1 >= __n2) {
            g_assertion_message_cmpint(
                G_LOG_DOMAIN as *const std::ffi::c_char,
                b"./widgets/webview/javascript.c\0" as *const u8
                    as *const std::ffi::c_char,
                33 as std::ffi::c_int,
                (*::core::mem::transmute::<
                    &[u8; 24],
                    &[std::ffi::c_char; 24],
                >(b"run_javascript_finished\0"))
                    .as_ptr(),
                b"n >= 2\0" as *const u8 as *const std::ffi::c_char,
                __n1 as guint64,
                b">=\0" as *const u8 as *const std::ffi::c_char,
                __n2 as guint64,
                'i' as i32 as std::ffi::c_char,
            );
        }
        let mut __n1_0 = n as gint64;
        let mut __n2_0 = 4 as std::ffi::c_int as gint64;
        if !(__n1_0 <= __n2_0) {
            g_assertion_message_cmpint(
                G_LOG_DOMAIN as *const std::ffi::c_char,
                b"./widgets/webview/javascript.c\0" as *const u8
                    as *const std::ffi::c_char,
                34 as std::ffi::c_int,
                (*::core::mem::transmute::<
                    &[u8; 24],
                    &[std::ffi::c_char; 24],
                >(b"run_javascript_finished\0"))
                    .as_ptr(),
                b"n <= 4\0" as *const u8 as *const std::ffi::c_char,
                __n1_0 as guint64,
                b"<=\0" as *const u8 as *const std::ffi::c_char,
                __n2_0 as guint64,
                'i' as i32 as std::ffi::c_char,
            );
        }
        let mut w = webview_get_by_id(lua_tointeger(L, -n) as guint64);
        lua_remove(L, -n);
        n -= 1;
        n;
        let mut cb = lua_touserdata(L, -n);
        if cb.is_null() {
            _log(
                LOG_LEVEL_warn,
                b"./widgets/webview/javascript.c\0" as *const u8
                    as *const std::ffi::c_char,
                b"javascript finshed called on non object\0" as *const u8
                    as *const std::ffi::c_char,
            );
            return;
        }
        lua_remove(L, -n);
        n -= 1;
        n;
        if n == 2 as std::ffi::c_int {
            if lua_type(L, -(2 as std::ffi::c_int)) == 0 as std::ffi::c_int {} else {
                g_assertion_message_expr(
                    G_LOG_DOMAIN as *const std::ffi::c_char,
                    b"./widgets/webview/javascript.c\0" as *const u8
                        as *const std::ffi::c_char,
                    49 as std::ffi::c_int,
                    (*::core::mem::transmute::<
                        &[u8; 24],
                        &[std::ffi::c_char; 24],
                    >(b"run_javascript_finished\0"))
                        .as_ptr(),
                    b"lua_isnil(L, -2)\0" as *const u8 as *const std::ffi::c_char,
                );
            }
            if lua_isstring(L, -(1 as std::ffi::c_int)) != 0 {} else {
                g_assertion_message_expr(
                    G_LOG_DOMAIN as *const std::ffi::c_char,
                    b"./widgets/webview/javascript.c\0" as *const u8
                        as *const std::ffi::c_char,
                    50 as std::ffi::c_int,
                    (*::core::mem::transmute::<
                        &[u8; 24],
                        &[std::ffi::c_char; 24],
                    >(b"run_javascript_finished\0"))
                        .as_ptr(),
                    b"lua_isstring(L, -1)\0" as *const u8 as *const std::ffi::c_char,
                );
            }
        }
        if n >= 1 as std::ffi::c_int && !cb.is_null() && !w.is_null() {
            luaH_object_push(L, cb);
            luaH_dofunction(L, n, 0 as std::ffi::c_int);
        }
        if !w.is_null() && !cb.is_null() {
            g_signal_handlers_disconnect_matched(
                (*w).widget as gpointer,
                G_SIGNAL_MATCH_DATA,
                0 as std::ffi::c_int as guint,
                0 as std::ffi::c_int as GQuark,
                NULL_0 as *mut GClosure,
                NULL_0 as *mut std::ffi::c_void,
                cb,
            );
            luaH_object_unref(L, cb);
        }
        lua_settop(L, top);
    }
    pub unsafe extern "C" fn run_javascript_webview_closed(
        mut UNUSED_view: *mut WebKitWebView,
        mut cb: gpointer,
    ) {
        luaH_object_unref(common.L, cb);
    }
    pub unsafe extern "C" fn luaH_webview_eval_js(mut L: *mut lua_State) -> gint {
        let mut cb = NULL_0 as *mut std::ffi::c_void;
        let mut d = (*luaH_checkwebview(L, 1 as std::ffi::c_int)).data
            as *mut webview_data_t;
        let mut script = luaL_checklstring(
            L,
            2 as std::ffi::c_int,
            NULL_0 as *mut size_t,
        );
        let mut usr_source = NULL_0 as *const gchar;
        let mut source = NULL_0 as *mut gchar;
        let mut no_return = false_0 != 0;
        if !(lua_type(L, 3 as std::ffi::c_int) == LUA_TTABLE) {
            luaL_typerror(
                L,
                3 as std::ffi::c_int,
                b"table\0" as *const u8 as *const std::ffi::c_char,
            );
        }
        let mut top = lua_gettop(L);
        if luaH_rawfield(
            L,
            3 as std::ffi::c_int,
            b"source\0" as *const u8 as *const std::ffi::c_char,
        ) != 0 && lua_isstring(L, -(1 as std::ffi::c_int)) != 0
        {
            usr_source = lua_tolstring(
                L,
                -(1 as std::ffi::c_int),
                NULL_0 as *mut size_t,
            );
        }
        if luaH_rawfield(
            L,
            3 as std::ffi::c_int,
            b"no_return\0" as *const u8 as *const std::ffi::c_char,
        ) != 0
        {
            no_return = lua_toboolean(L, -(1 as std::ffi::c_int)) != 0;
        }
        if luaH_rawfield(
            L,
            3 as std::ffi::c_int,
            b"callback\0" as *const u8 as *const std::ffi::c_char,
        ) != 0
        {
            if !(lua_type(L, -(1 as std::ffi::c_int)) == LUA_TFUNCTION) {
                luaL_typerror(
                    L,
                    -(1 as std::ffi::c_int),
                    b"function\0" as *const u8 as *const std::ffi::c_char,
                );
            }
            cb = luaH_object_ref(L, -(1 as std::ffi::c_int));
        }
        lua_settop(L, top);
        if usr_source.is_null() {
            source = luaH_callerinfo(L);
        }
        lua_pushboolean(L, no_return as std::ffi::c_int);
        lua_pushstring(L, script);
        lua_pushstring(
            L,
            if !usr_source.is_null() { g_strdup_inline(usr_source) } else { source },
        );
        lua_pushinteger(L, webkit_web_view_get_page_id((*d).view) as lua_Integer);
        lua_pushlightuserdata(L, cb);
        ipc_send_lua(
            (*d).ipc,
            IPC_TYPE_eval_js,
            L,
            -(5 as std::ffi::c_int),
            -(1 as std::ffi::c_int),
        );
        lua_settop(L, -(5 as std::ffi::c_int) - 1 as std::ffi::c_int);
        if !cb.is_null() {
            g_signal_connect_data(
                (*d).view as gpointer,
                b"destroy\0" as *const u8 as *const std::ffi::c_char,
                ::core::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut WebKitWebView, gpointer) -> ()>,
                    GCallback,
                >(
                    Some(
                        run_javascript_webview_closed
                            as unsafe extern "C" fn(*mut WebKitWebView, gpointer) -> (),
                    ),
                ),
                cb,
                ::core::mem::transmute::<
                    libc::intptr_t,
                    GClosureNotify,
                >(NULL_0 as libc::intptr_t),
                G_CONNECT_DEFAULT,
            );
        }
        return FALSE;
    }
    use super::glibconfig_h::{guint8, gint64, guint64};
    use super::gtypes_h::{guint, gint, gpointer, gchar};
    use super::common_h::common;
    use super::lua_h::{
        lua_State, lua_gettop, lua_tointeger, lua_Integer, lua_remove, lua_touserdata,
        lua_type, lua_isstring, lua_settop, LUA_TTABLE, lua_tolstring, lua_toboolean,
        LUA_TFUNCTION, lua_pushboolean, lua_pushstring, lua_pushinteger,
        lua_pushlightuserdata,
    };
    use super::luaserialize_h::lua_deserialize_range;
    use super::gtestutils_h::{g_assertion_message_cmpint, g_assertion_message_expr};
    use super::gmessages_h::G_LOG_DOMAIN;
    use super::{webview_get_by_id, luaH_checkwebview, webview_data_t};
    use super::widget_h::widget_t;
    use super::log_h::{_log, LOG_LEVEL_warn, log_level_t};
    use super::luaobject_h::{luaH_object_push, luaH_object_unref, luaH_object_ref};
    use super::lualib_h::luaH_dofunction;
    use super::gsignal_h::{
        g_signal_handlers_disconnect_matched, G_SIGNAL_MATCH_DATA, GSignalMatchType,
        g_signal_connect_data, GConnectFlags, G_CONNECT_DEFAULT,
    };
    use super::gquark_h::GQuark;
    use super::__stddef_null_h::NULL_0;
    use super::gclosure_h::{GClosure, GCallback, GClosureNotify};
    use super::WebKitDownload_h::WebKitWebView;
    use super::lauxlib_h::{luaL_checklstring, luaL_typerror};
    use super::__stddef_size_t_h::size_t;
    use super::stdbool_h::false_0;
    use super::luah_h::luaH_rawfield;
    use super::util_h::luaH_callerinfo;
    use super::gstrfuncs_h::g_strdup_inline;
    use super::WebKitWebView_h::webkit_web_view_get_page_id;
    use super::ipc_h::{ipc_send_lua, IPC_TYPE_eval_js, ipc_type_t};
    use super::gmacros_h::FALSE;
}
pub mod download_h {
    use super::lua_h::lua_State;
    use super::WebKitDownload_h::WebKitDownload;
    use super::gtypes_h::gint;
    unsafe extern "C" {
        pub fn luaH_download_push(_: *mut lua_State, _: *mut WebKitDownload) -> gint;
    }
}
pub mod luakit_h {
    use super::luaclass_h::lua_class_t;
    use super::lua_h::lua_State;
    use super::gtypes_h::gint;
    unsafe extern "C" {
        pub fn luakit_lib_get_luakit_class() -> *mut lua_class_t;
        pub fn luaH_luakit_allow_certificate(L: *mut lua_State) -> gint;
    }
}
pub mod downloads_c {
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn download_start_cb(
        mut UNUSED_c: *mut WebKitWebContext,
        mut dl: *mut WebKitDownload,
        mut UNUSED_user_data: gpointer,
    ) -> gboolean {
        let mut dl_view = webkit_download_get_web_view(dl);
        let mut w = if !dl_view.is_null() {
            g_object_get_data(
                g_type_check_instance_cast(
                    dl_view as *mut GTypeInstance,
                    ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
                ) as *mut std::ffi::c_void as *mut GObject,
                GOBJECT_LUAKIT_WIDGET_DATA_KEY.as_ptr(),
            ) as *mut widget_t
        } else {
            NULL_0 as *mut widget_t
        };
        let mut L = common.L;
        let mut top = lua_gettop(L);
        luaH_download_push(L, dl);
        if !w.is_null() {
            luaH_object_push(L, (*w).ref_0);
        } else {
            lua_pushnil(L);
        }
        let mut luakit_class = luakit_lib_get_luakit_class();
        let mut ret = luaH_class_emit_signal(
            L,
            luakit_class,
            b"download-start\0" as *const u8 as *const std::ffi::c_char,
            2 as std::ffi::c_int,
            1 as std::ffi::c_int,
        );
        let mut handled = (ret != 0 && lua_toboolean(L, 2 as std::ffi::c_int) != 0)
            as std::ffi::c_int;
        lua_settop(L, top);
        return handled;
    }
    use super::WebKitWebContext_h::WebKitWebContext;
    use super::WebKitDownload_h::{
        WebKitDownload, webkit_download_get_web_view, WebKitWebView,
    };
    use super::gtypes_h::{gpointer, gboolean, gint};
    use super::gobject_h::{g_object_get_data, GObject};
    use super::gtype_h::{g_type_check_instance_cast, GTypeInstance, GType};
    use super::widget_h::{GOBJECT_LUAKIT_WIDGET_DATA_KEY, widget_t};
    use super::__stddef_null_h::NULL_0;
    use super::common_h::common;
    use super::lua_h::{lua_State, lua_gettop, lua_pushnil, lua_toboolean, lua_settop};
    use super::download_h::luaH_download_push;
    use super::luaobject_h::luaH_object_push;
    use super::luakit_h::luakit_lib_get_luakit_class;
    use super::luaclass_h::{lua_class_t, luaH_class_emit_signal};
}
pub mod history_c {
    pub unsafe extern "C" fn luaH_webview_push_history(
        mut L: *mut lua_State,
        mut view: *mut WebKitWebView,
    ) -> gint {
        let mut bflist = webkit_web_view_get_back_forward_list(view);
        let mut item = 0 as *mut WebKitBackForwardListItem;
        let mut backlen = g_list_length(webkit_back_forward_list_get_back_list(bflist))
            as gint;
        let mut forwardlen = g_list_length(
            webkit_back_forward_list_get_forward_list(bflist),
        ) as gint;
        lua_createtable(L, 0 as std::ffi::c_int, 2 as std::ffi::c_int);
        lua_pushlstring(
            L,
            b"index\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 6]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        lua_pushnumber(L, (backlen + 1 as std::ffi::c_int) as lua_Number);
        lua_rawset(L, -(3 as std::ffi::c_int));
        lua_createtable(
            L,
            backlen + forwardlen + 1 as std::ffi::c_int,
            0 as std::ffi::c_int,
        );
        let mut i = -backlen;
        while i <= forwardlen {
            item = webkit_back_forward_list_get_nth_item(bflist, i);
            lua_createtable(L, 0 as std::ffi::c_int, 2 as std::ffi::c_int);
            lua_pushlstring(
                L,
                b"uri\0" as *const u8 as *const std::ffi::c_char,
                (::core::mem::size_of::<[std::ffi::c_char; 4]>() as std::ffi::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                    )
                    .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
            );
            lua_pushstring(
                L,
                if !item.is_null() {
                    webkit_back_forward_list_item_get_uri(item)
                } else {
                    b"about:blank\0" as *const u8 as *const std::ffi::c_char
                },
            );
            lua_rawset(L, -(3 as std::ffi::c_int));
            lua_pushlstring(
                L,
                b"title\0" as *const u8 as *const std::ffi::c_char,
                (::core::mem::size_of::<[std::ffi::c_char; 6]>() as std::ffi::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                    )
                    .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
            );
            lua_pushstring(
                L,
                if !item.is_null() {
                    webkit_back_forward_list_item_get_title(item)
                } else {
                    b"\0" as *const u8 as *const std::ffi::c_char
                },
            );
            lua_rawset(L, -(3 as std::ffi::c_int));
            lua_rawseti(L, -(2 as std::ffi::c_int), backlen + i + 1 as std::ffi::c_int);
            i += 1;
            i;
        }
        lua_pushlstring(
            L,
            b"items\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 6]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        lua_insert(L, lua_gettop(L) - 1 as std::ffi::c_int);
        lua_rawset(L, -(3 as std::ffi::c_int));
        return 1 as std::ffi::c_int;
    }
    pub unsafe extern "C" fn luaH_webview_can_go_back(mut L: *mut lua_State) -> gint {
        let mut d = (*luaH_checkwebview(L, 1 as std::ffi::c_int)).data
            as *mut webview_data_t;
        lua_pushboolean(L, webkit_web_view_can_go_back((*d).view));
        return 1 as std::ffi::c_int;
    }
    pub unsafe extern "C" fn luaH_webview_can_go_forward(mut L: *mut lua_State) -> gint {
        let mut d = (*luaH_checkwebview(L, 1 as std::ffi::c_int)).data
            as *mut webview_data_t;
        lua_pushboolean(L, webkit_web_view_can_go_forward((*d).view));
        return 1 as std::ffi::c_int;
    }
    pub unsafe extern "C" fn webview_history_go(
        mut L: *mut lua_State,
        mut direction: gint,
    ) -> gint {
        let mut d = (*luaH_checkwebview(L, 1 as std::ffi::c_int)).data
            as *mut webview_data_t;
        let mut steps = luaL_checknumber(L, 2 as std::ffi::c_int) as gint * direction;
        let mut item = webkit_back_forward_list_get_nth_item(
            webkit_web_view_get_back_forward_list((*d).view),
            steps,
        );
        if !item.is_null() {
            webkit_web_view_go_to_back_forward_list_item((*d).view, item);
        }
        lua_pushboolean(
            L,
            (item != NULL_0 as *mut WebKitBackForwardListItem) as std::ffi::c_int,
        );
        return 1 as std::ffi::c_int;
    }
    pub unsafe extern "C" fn luaH_webview_go_back(mut L: *mut lua_State) -> gint {
        return webview_history_go(L, -(1 as std::ffi::c_int));
    }
    pub unsafe extern "C" fn luaH_webview_go_forward(mut L: *mut lua_State) -> gint {
        return webview_history_go(L, 1 as std::ffi::c_int);
    }
    pub unsafe extern "C" fn luaH_webview_set_session_state(
        mut L: *mut lua_State,
        mut d: *mut webview_data_t,
    ) {
        let mut len: size_t = 0;
        let mut str = lua_tolstring(L, 3 as std::ffi::c_int, &mut len);
        let mut bytes = g_bytes_new(str as gconstpointer, len);
        let mut state = webkit_web_view_session_state_new(bytes);
        g_bytes_unref(bytes);
        if state.is_null() {
            luaL_error(
                L,
                b"Invalid session state\0" as *const u8 as *const std::ffi::c_char,
            );
        }
        webkit_web_view_restore_session_state((*d).view, state);
        webkit_web_view_session_state_unref(state);
        let mut bfl = webkit_web_view_get_back_forward_list((*d).view);
        let mut item = webkit_back_forward_list_get_current_item(bfl);
        if !item.is_null() {
            webkit_web_view_go_to_back_forward_list_item((*d).view, item);
            update_uri((*d).widget, webkit_back_forward_list_item_get_uri(item));
        }
    }
    pub unsafe extern "C" fn luaH_webview_push_session_state(
        mut L: *mut lua_State,
        mut d: *mut webview_data_t,
    ) -> std::ffi::c_int {
        let mut state = webkit_web_view_get_session_state((*d).view);
        let mut bytes = webkit_web_view_session_state_serialize(state);
        let mut len: gsize = 0;
        let mut str = g_bytes_get_data(bytes, &mut len) as *const gchar;
        lua_pushlstring(L, str, len);
        g_bytes_unref(bytes);
        webkit_web_view_session_state_unref(state);
        return 1 as std::ffi::c_int;
    }
    use super::lua_h::{
        lua_State, lua_createtable, lua_pushlstring, lua_pushnumber, lua_Number,
        lua_rawset, lua_pushstring, lua_rawseti, lua_insert, lua_gettop, lua_pushboolean,
        lua_tolstring,
    };
    use super::WebKitDownload_h::WebKitWebView;
    use super::gtypes_h::{gint, guint, gchar, gconstpointer};
    use super::WebKitWebView_h::{
        webkit_web_view_get_back_forward_list, webkit_web_view_can_go_back,
        webkit_web_view_can_go_forward, webkit_web_view_go_to_back_forward_list_item,
        webkit_web_view_restore_session_state, webkit_web_view_get_session_state,
    };
    use super::WebKitBackForwardList_h::{
        WebKitBackForwardList, webkit_back_forward_list_get_back_list,
        webkit_back_forward_list_get_forward_list, webkit_back_forward_list_get_nth_item,
        webkit_back_forward_list_get_current_item,
    };
    use super::WebKitBackForwardListItem_h::{
        WebKitBackForwardListItem, webkit_back_forward_list_item_get_uri,
        webkit_back_forward_list_item_get_title,
    };
    use super::glist_h::g_list_length;
    use super::{luaH_checkwebview, webview_data_t, update_uri};
    use super::lauxlib_h::{luaL_checknumber, luaL_error};
    use super::__stddef_null_h::NULL_0;
    use super::__stddef_size_t_h::size_t;
    use super::gbytes_h::{g_bytes_new, g_bytes_unref, g_bytes_get_data};
    use super::garray_h::GBytes;
    use super::WebKitWebViewSessionState_h::{
        webkit_web_view_session_state_new, WebKitWebViewSessionState,
        webkit_web_view_session_state_unref, webkit_web_view_session_state_serialize,
    };
    use super::glibconfig_h::gsize;
}
pub mod scroll_c {
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn webview_scroll_recv(
        mut w: *mut widget_t,
        mut msg: *const ipc_scroll_t,
    ) {
        let mut d = (*w).data as *mut webview_data_t;
        if webkit_web_view_get_page_id((*d).view) != (*msg).page_id {
            return;
        }
        match (*msg).subtype as std::ffi::c_uint {
            0 => {
                (*d).doc_w = (*msg).h;
                (*d).doc_h = (*msg).v;
            }
            1 => {
                (*d).win_w = (*msg).h;
                (*d).win_h = (*msg).v;
            }
            2 => {
                (*d).scroll_x = (*msg).h;
                (*d).scroll_y = (*msg).v;
            }
            _ => {}
        };
    }
    pub unsafe extern "C" fn luaH_webview_scroll_newindex(
        mut L: *mut lua_State,
    ) -> gint {
        let mut d = (*luaH_checkwebview(
            L,
            -(10002 as std::ffi::c_int) - 1 as std::ffi::c_int,
        ))
            .data as *mut webview_data_t;
        let mut prop = luaL_checklstring(L, 2 as std::ffi::c_int, NULL_0 as *mut size_t);
        let mut t = l_tokenize(prop);
        if t as std::ffi::c_uint == L_TK_X as std::ffi::c_int as std::ffi::c_uint {
            (*d).scroll_x = luaL_checknumber(L, 3 as std::ffi::c_int) as gint;
        } else if t as std::ffi::c_uint == L_TK_Y as std::ffi::c_int as std::ffi::c_uint
        {
            (*d).scroll_y = luaL_checknumber(L, 3 as std::ffi::c_int) as gint;
        } else {
            return 0 as std::ffi::c_int
        }
        lua_pushinteger(L, webkit_web_view_get_page_id((*d).view) as lua_Integer);
        lua_pushinteger(L, (*d).scroll_x as lua_Integer);
        lua_pushinteger(L, (*d).scroll_y as lua_Integer);
        ipc_send_lua(
            (*d).ipc,
            IPC_TYPE_scroll,
            L,
            4 as std::ffi::c_int,
            6 as std::ffi::c_int,
        );
        return 0 as std::ffi::c_int;
    }
    pub unsafe extern "C" fn luaH_webview_scroll_index(mut L: *mut lua_State) -> gint {
        let mut d = (*luaH_checkwebview(
            L,
            -(10002 as std::ffi::c_int) - 1 as std::ffi::c_int,
        ))
            .data as *mut webview_data_t;
        let mut prop = luaL_checklstring(L, 2 as std::ffi::c_int, NULL_0 as *mut size_t);
        let mut t = l_tokenize(prop);
        match t as std::ffi::c_uint {
            267 => {
                lua_pushnumber(L, (*d).scroll_x as lua_Number);
                return 1 as std::ffi::c_int;
            }
            270 => {
                lua_pushnumber(L, (*d).scroll_y as lua_Number);
                return 1 as std::ffi::c_int;
            }
            268 => {
                lua_pushnumber(L, ((*d).doc_w - (*d).win_w) as lua_Number);
                return 1 as std::ffi::c_int;
            }
            271 => {
                lua_pushnumber(L, ((*d).doc_h - (*d).win_h) as lua_Number);
                return 1 as std::ffi::c_int;
            }
            269 => {
                lua_pushnumber(L, (*d).win_w as lua_Number);
                return 1 as std::ffi::c_int;
            }
            272 => {
                lua_pushnumber(L, (*d).win_h as lua_Number);
                return 1 as std::ffi::c_int;
            }
            _ => return 0 as std::ffi::c_int,
        };
    }
    pub unsafe extern "C" fn luaH_webview_push_scroll_table(
        mut L: *mut lua_State,
    ) -> gint {
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
                luaH_webview_scroll_index as unsafe extern "C" fn(*mut lua_State) -> gint,
            ),
            1 as std::ffi::c_int,
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
        lua_pushvalue(L, 1 as std::ffi::c_int);
        lua_pushcclosure(
            L,
            Some(
                luaH_webview_scroll_newindex
                    as unsafe extern "C" fn(*mut lua_State) -> gint,
            ),
            1 as std::ffi::c_int,
        );
        lua_rawset(L, -(3 as std::ffi::c_int));
        lua_setmetatable(L, -(2 as std::ffi::c_int));
        return 1 as std::ffi::c_int;
    }
    use super::widget_h::widget_t;
    use super::ipc_h::{
        ipc_scroll_t, ipc_scroll_subtype_t, ipc_send_lua, IPC_TYPE_scroll, ipc_type_t,
    };
    use super::{webview_data_t, luaH_checkwebview};
    use super::WebKitWebView_h::webkit_web_view_get_page_id;
    use super::gtypes_h::{gint, gchar};
    use super::lua_h::{
        lua_State, lua_Number, lua_pushinteger, lua_Integer, lua_pushnumber,
        lua_createtable, lua_pushlstring, lua_pushvalue, lua_pushcclosure, lua_rawset,
        lua_setmetatable,
    };
    use super::lauxlib_h::{luaL_checklstring, luaL_checknumber};
    use super::__stddef_null_h::NULL_0;
    use super::__stddef_size_t_h::size_t;
    use super::tokenize_h::{l_tokenize, luakit_token_t, L_TK_X, L_TK_Y};
    use super::glibconfig_h::guint64;
}
pub mod inspector_c {
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn inspector_open_window_cb(
        mut UNUSED_inspector: *mut WebKitWebInspector,
        mut w: *mut widget_t,
    ) -> gboolean {
        let mut L = common.L;
        luaH_object_push(L, (*w).ref_0);
        let mut nret = luaH_object_emit_signal(
            L,
            -(1 as std::ffi::c_int),
            b"create-inspector-window\0" as *const u8 as *const std::ffi::c_char,
            0 as std::ffi::c_int,
            1 as std::ffi::c_int,
        );
        let mut ret = (nret != 0 && lua_toboolean(L, -(1 as std::ffi::c_int)) != 0)
            as std::ffi::c_int;
        lua_settop(L, -(1 as std::ffi::c_int + nret) - 1 as std::ffi::c_int);
        return ret;
    }
    pub unsafe extern "C" fn inspector_show_window_cb(
        mut UNUSED_inspector: *mut WebKitWebInspector,
        mut w: *mut widget_t,
    ) -> gboolean {
        let mut L = common.L;
        luaH_object_push(L, (*w).ref_0);
        let mut nret = luaH_object_emit_signal(
            L,
            -(1 as std::ffi::c_int),
            b"show-inspector\0" as *const u8 as *const std::ffi::c_char,
            0 as std::ffi::c_int,
            1 as std::ffi::c_int,
        );
        let mut ret = (nret != 0 && lua_toboolean(L, -(1 as std::ffi::c_int)) != 0)
            as std::ffi::c_int;
        lua_settop(L, -(1 as std::ffi::c_int + nret) - 1 as std::ffi::c_int);
        return ret;
    }
    pub unsafe extern "C" fn inspector_close_window_cb(
        mut UNUSED_inspector: *mut WebKitWebInspector,
        mut w: *mut widget_t,
    ) -> gboolean {
        let mut L = common.L;
        luaH_object_push(L, (*w).ref_0);
        let mut d = (*w).data as *mut webview_data_t;
        lua_pushnil(L);
        (*d).inspector_open = FALSE;
        let mut nret = luaH_object_emit_signal(
            L,
            -(2 as std::ffi::c_int),
            b"close-inspector\0" as *const u8 as *const std::ffi::c_char,
            1 as std::ffi::c_int,
            0 as std::ffi::c_int,
        );
        let mut ret = (nret != 0 && lua_toboolean(L, -(1 as std::ffi::c_int)) != 0)
            as std::ffi::c_int;
        lua_settop(L, -(1 as std::ffi::c_int + nret) - 1 as std::ffi::c_int);
        return ret;
    }
    pub unsafe extern "C" fn inspector_attach_window_cb(
        mut UNUSED_inspector: *mut WebKitWebInspector,
        mut w: *mut widget_t,
    ) -> gboolean {
        let mut L = common.L;
        let mut d = (*w).data as *mut webview_data_t;
        (*d).inspector_open = TRUE;
        luaH_object_push(L, (*w).ref_0);
        let mut nret = luaH_object_emit_signal(
            L,
            -(1 as std::ffi::c_int),
            b"attach-inspector\0" as *const u8 as *const std::ffi::c_char,
            0 as std::ffi::c_int,
            0 as std::ffi::c_int,
        );
        let mut ret = (nret != 0 && lua_toboolean(L, -(1 as std::ffi::c_int)) != 0)
            as std::ffi::c_int;
        lua_settop(L, -(1 as std::ffi::c_int + nret) - 1 as std::ffi::c_int);
        return ret;
    }
    pub unsafe extern "C" fn inspector_detach_window_cb(
        mut UNUSED_inspector: *mut WebKitWebInspector,
        mut w: *mut widget_t,
    ) -> gboolean {
        let mut L = common.L;
        luaH_object_push(L, (*w).ref_0);
        let mut nret = luaH_object_emit_signal(
            L,
            -(1 as std::ffi::c_int),
            b"detach-inspector\0" as *const u8 as *const std::ffi::c_char,
            0 as std::ffi::c_int,
            0 as std::ffi::c_int,
        );
        let mut ret = (nret != 0 && lua_toboolean(L, -(1 as std::ffi::c_int)) != 0)
            as std::ffi::c_int;
        lua_settop(L, -(1 as std::ffi::c_int + nret) - 1 as std::ffi::c_int);
        return ret;
    }
    pub unsafe extern "C" fn luaH_webview_show_inspector(mut L: *mut lua_State) -> gint {
        let mut d = (*luaH_checkwebview(L, 1 as std::ffi::c_int)).data
            as *mut webview_data_t;
        webkit_web_inspector_show((*d).inspector);
        return 0 as std::ffi::c_int;
    }
    pub unsafe extern "C" fn luaH_webview_close_inspector(
        mut L: *mut lua_State,
    ) -> gint {
        let mut d = (*luaH_checkwebview(L, 1 as std::ffi::c_int)).data
            as *mut webview_data_t;
        webkit_web_inspector_close((*d).inspector);
        return 0 as std::ffi::c_int;
    }
    use super::WebKitWebInspector_h::{
        WebKitWebInspector, webkit_web_inspector_show, webkit_web_inspector_close,
    };
    use super::widget_h::widget_t;
    use super::gtypes_h::{gboolean, gint};
    use super::common_h::common;
    use super::lua_h::{lua_State, lua_toboolean, lua_settop, lua_pushnil};
    use super::luaobject_h::{luaH_object_push, luaH_object_emit_signal};
    use super::{webview_data_t, luaH_checkwebview};
    use super::gmacros_h::{FALSE, TRUE};
}
pub mod find_controller_c {
    pub unsafe extern "C" fn found_text_cb(
        mut UNUSED_find_controller: *mut WebKitFindController,
        mut match_count: guint,
        mut w: *mut widget_t,
    ) {
        let mut L = common.L;
        luaH_object_push(L, (*w).ref_0);
        lua_pushinteger(L, match_count as lua_Integer);
        luaH_object_emit_signal(
            L,
            -(2 as std::ffi::c_int),
            b"found-text\0" as *const u8 as *const std::ffi::c_char,
            1 as std::ffi::c_int,
            0 as std::ffi::c_int,
        );
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    }
    pub unsafe extern "C" fn failed_to_find_text_cb(
        mut UNUSED_find_controller: *mut WebKitFindController,
        mut w: *mut widget_t,
    ) {
        let mut L = common.L;
        luaH_object_push(L, (*w).ref_0);
        luaH_object_emit_signal(
            L,
            -(1 as std::ffi::c_int),
            b"failed-to-find-text\0" as *const u8 as *const std::ffi::c_char,
            0 as std::ffi::c_int,
            0 as std::ffi::c_int,
        );
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    }
    use super::WebKitFindController_h::WebKitFindController;
    use super::gtypes_h::guint;
    use super::widget_h::widget_t;
    use super::common_h::common;
    use super::lua_h::{lua_State, lua_pushinteger, lua_Integer, lua_settop};
    use super::luaobject_h::{luaH_object_push, luaH_object_emit_signal};
}
pub mod stylesheets_c {
    pub static mut inside_stylesheet_cb: gboolean = FALSE;
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn webview_stylesheets_regenerate_stylesheet(
        mut w: *mut widget_t,
        mut stylesheet: *mut lstylesheet_t,
    ) {
        let mut d = (*w).data as *mut webview_data_t;
        if !(g_list_find((*d).stylesheets, stylesheet as gconstpointer)).is_null() {
            (*d).stylesheet_refreshed = TRUE;
        }
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn webview_stylesheets_regenerate(mut w: *mut widget_t) {
        let mut d = (*w).data as *mut webview_data_t;
        if (*d).stylesheet_added != 0 || (*d).stylesheet_removed != 0
            || (*d).stylesheet_refreshed != 0
        {
            webkit_user_content_manager_remove_all_style_sheets((*d).user_content);
            let mut l = 0 as *mut GList;
            l = (*d).stylesheets;
            while !l.is_null() {
                let mut stylesheet = (*l).data as *mut lstylesheet_t;
                webkit_user_content_manager_add_style_sheet(
                    (*d).user_content,
                    (*stylesheet).stylesheet,
                );
                l = (*l).next;
            }
            (*d).stylesheet_added = FALSE;
            (*d).stylesheet_removed = FALSE;
        }
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn webview_stylesheet_set_enabled(
        mut w: *mut widget_t,
        mut stylesheet: *mut lstylesheet_t,
        mut enable: gboolean,
    ) -> std::ffi::c_int {
        let mut d = (*w).data as *mut webview_data_t;
        let mut item = g_list_find((*d).stylesheets, stylesheet as gconstpointer);
        if enable == (item != NULL_0 as *mut GList) as std::ffi::c_int {
            return 0 as std::ffi::c_int;
        }
        if enable != 0 {
            (*d).stylesheets = g_list_prepend((*d).stylesheets, stylesheet as gpointer);
            (*d).stylesheet_added = TRUE;
        } else {
            (*d).stylesheets = g_list_remove_link((*d).stylesheets, item);
            (*d).stylesheet_removed = TRUE;
        }
        if inside_stylesheet_cb == 0 {
            webview_stylesheets_regenerate(w);
        }
        return 0 as std::ffi::c_int;
    }
    pub unsafe extern "C" fn luaH_webview_stylesheets_index(
        mut L: *mut lua_State,
    ) -> gint {
        let mut d = (*luaH_checkwebview(
            L,
            -(10002 as std::ffi::c_int) - 1 as std::ffi::c_int,
        ))
            .data as *mut webview_data_t;
        let mut stylesheet = luaH_checkstylesheet(L, 2 as std::ffi::c_int)
            as *mut lstylesheet_t;
        let mut enabled = (g_list_find((*d).stylesheets, stylesheet as gconstpointer)
            != NULL_0 as *mut GList) as std::ffi::c_int;
        lua_pushboolean(L, enabled);
        return 1 as std::ffi::c_int;
    }
    pub unsafe extern "C" fn luaH_webview_stylesheets_newindex(
        mut L: *mut lua_State,
    ) -> gint {
        let mut d = (*luaH_checkwebview(
            L,
            -(10002 as std::ffi::c_int) - 1 as std::ffi::c_int,
        ))
            .data as *mut webview_data_t;
        let mut stylesheet = luaH_checkstylesheet(L, 2 as std::ffi::c_int)
            as *mut lstylesheet_t;
        let mut enable = lua_toboolean(L, 3 as std::ffi::c_int);
        webview_stylesheet_set_enabled((*d).widget, stylesheet, enable);
        return 0 as std::ffi::c_int;
    }
    pub unsafe extern "C" fn luaH_webview_push_stylesheets_table(
        mut L: *mut lua_State,
    ) -> gint {
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
                luaH_webview_stylesheets_index
                    as unsafe extern "C" fn(*mut lua_State) -> gint,
            ),
            1 as std::ffi::c_int,
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
        lua_pushvalue(L, 1 as std::ffi::c_int);
        lua_pushcclosure(
            L,
            Some(
                luaH_webview_stylesheets_newindex
                    as unsafe extern "C" fn(*mut lua_State) -> gint,
            ),
            1 as std::ffi::c_int,
        );
        lua_rawset(L, -(3 as std::ffi::c_int));
        lua_setmetatable(L, -(2 as std::ffi::c_int));
        return 1 as std::ffi::c_int;
    }
    pub unsafe extern "C" fn webview_update_stylesheets(
        mut L: *mut lua_State,
        mut w: *mut widget_t,
    ) {
        let mut d = (*w).data as *mut webview_data_t;
        (*d).stylesheet_added = FALSE;
        (*d).stylesheet_removed = FALSE;
        inside_stylesheet_cb = TRUE;
        luaH_object_push(L, (*w).ref_0);
        luaH_object_emit_signal(
            L,
            -(1 as std::ffi::c_int),
            b"stylesheet\0" as *const u8 as *const std::ffi::c_char,
            0 as std::ffi::c_int,
            0 as std::ffi::c_int,
        );
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        inside_stylesheet_cb = FALSE;
        webview_stylesheets_regenerate(w);
    }
    use super::gmacros_h::{FALSE, TRUE};
    use super::gtypes_h::{gboolean, gconstpointer, gpointer, gint};
    use super::widget_h::widget_t;
    use super::stylesheet_h::{lstylesheet_t, luaH_checkstylesheet};
    use super::{webview_data_t, luaH_checkwebview};
    use super::glist_h::{g_list_find, GList, g_list_prepend, g_list_remove_link};
    use super::WebKitUserContentManager_h::{
        webkit_user_content_manager_remove_all_style_sheets,
        webkit_user_content_manager_add_style_sheet,
    };
    use super::__stddef_null_h::NULL_0;
    use super::lua_h::{
        lua_State, lua_pushboolean, lua_toboolean, lua_createtable, lua_pushlstring,
        lua_pushvalue, lua_pushcclosure, lua_rawset, lua_setmetatable, lua_settop,
    };
    use super::luaobject_h::{luaH_object_push, luaH_object_emit_signal};
}
pub mod limits_h {
    pub const UINT_MAX: std::ffi::c_uint = (__INT_MAX__ as std::ffi::c_uint)
        .wrapping_mul(2 as std::ffi::c_uint)
        .wrapping_add(1 as std::ffi::c_uint);
    use super::internal::__INT_MAX__;
}
pub mod stdbool_h {
    pub const false_0: std::ffi::c_int = 0 as std::ffi::c_int;
}
pub use self::__stddef_ptrdiff_t_h::ptrdiff_t;
pub use self::__stddef_size_t_h::size_t;
pub use self::glibconfig_h::{
    gint8, guint8, gint16, guint16, guint32, gint64, guint64, gsize, G_MAXUINT,
};
pub use self::types_h::__pid_t;
pub use self::time_h::pid_t;
pub use self::gtypes_h::{
    gchar, gshort, glong, gint, gboolean, guchar, gulong, guint, gfloat, gdouble,
    gpointer, gconstpointer, GEqualFunc, GDestroyNotify, GHashFunc,
};
pub use self::garray_h::{
    GBytes, _GPtrArray, GPtrArray, _GBytes, g_ptr_array_new, g_ptr_array_remove,
    g_ptr_array_add,
};
pub use self::gquark_h::GQuark;
pub use self::gerror_h::{_GError, GError, g_error_new_literal, g_error_free};
pub use self::gconvert_h::{GIConv, _GIConv};
pub use self::gdataset_h::{GData, _GData};
pub use self::glist_h::{
    _GList, GList, g_list_prepend, g_list_remove_link, g_list_find, g_list_length,
};
pub use self::ghash_h::{
    GHashTable, _GHashTable, g_hash_table_new, g_hash_table_insert, g_hash_table_lookup,
    g_str_equal, g_str_hash,
};
pub use self::gslist_h::{
    _GSList, GSList, g_slist_free, g_slist_free_full, g_slist_prepend,
};
pub use self::gmain_h::{
    GIOCondition, G_IO_NVAL, G_IO_HUP, G_IO_ERR, G_IO_PRI, G_IO_OUT, G_IO_IN,
    GMainContext, _GSource, GSourcePrivate, GSource, GSourceFuncs, _GSourceFuncs,
    GSourceDummyMarshal, GSourceFunc, GSourceFuncsFinalizeFunc, GSourceFuncsDispatchFunc,
    GSourceFuncsCheckFunc, GSourceFuncsPrepareFunc, GSourceCallbackFuncs,
    _GSourceCallbackFuncs, _GMainContext, _GSourcePrivate, g_idle_remove_by_data,
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
pub use self::gtype_h::{
    GType, GValue, _GTypeClass, GTypeClass, _GTypeInstance, GTypeInstance,
    G_TYPE_FUNDAMENTAL_SHIFT, G_TYPE_INT, G_TYPE_ENUM, g_type_check_instance_cast,
    g_type_check_instance_is_a,
};
pub use self::gvalue_h::{_GValue, C2RustUnnamed, g_value_init};
pub use self::gparam_h::{
    GParamFlags, G_PARAM_DEPRECATED, G_PARAM_EXPLICIT_NOTIFY, G_PARAM_STATIC_BLURB,
    G_PARAM_STATIC_NICK, G_PARAM_PRIVATE, G_PARAM_STATIC_NAME, G_PARAM_LAX_VALIDATION,
    G_PARAM_CONSTRUCT_ONLY, G_PARAM_CONSTRUCT, G_PARAM_READWRITE, G_PARAM_WRITABLE,
    G_PARAM_READABLE, _GParamSpec, GParamSpec,
};
pub use self::gclosure_h::{
    _GClosure, GClosureNotifyData, _GClosureNotifyData, GClosureNotify, GClosure,
    GCallback,
};
pub use self::gsignal_h::{
    GConnectFlags, G_CONNECT_SWAPPED, G_CONNECT_AFTER, G_CONNECT_DEFAULT,
    GSignalMatchType, G_SIGNAL_MATCH_UNBLOCKED, G_SIGNAL_MATCH_DATA, G_SIGNAL_MATCH_FUNC,
    G_SIGNAL_MATCH_CLOSURE, G_SIGNAL_MATCH_DETAIL, G_SIGNAL_MATCH_ID,
    g_signal_connect_data, g_signal_handlers_disconnect_matched,
};
pub use self::gobject_h::{
    _GObject, GObject, GInitiallyUnowned, g_object_new, g_object_set, g_object_get,
    g_object_connect, g_object_set_property, g_object_freeze_notify,
    g_object_thaw_notify, g_object_ref, g_object_unref, g_object_get_data,
    g_object_set_data,
};
pub use self::gioenums_h::{
    GTlsCertificateFlags, G_TLS_CERTIFICATE_VALIDATE_ALL,
    G_TLS_CERTIFICATE_GENERIC_ERROR, G_TLS_CERTIFICATE_INSECURE,
    G_TLS_CERTIFICATE_REVOKED, G_TLS_CERTIFICATE_EXPIRED,
    G_TLS_CERTIFICATE_NOT_ACTIVATED, G_TLS_CERTIFICATE_BAD_IDENTITY,
    G_TLS_CERTIFICATE_UNKNOWN_CA, G_TLS_CERTIFICATE_NO_FLAGS,
};
pub use self::giotypes_h::{
    GAsyncResult, GCancellable, GApplication, GFile, GTlsCertificate,
    GAsyncReadyCallback, _GAsyncResult, _GFile,
};
pub use self::gcancellable_h::{_GCancellable, GCancellablePrivate, _GCancellablePrivate};
pub use self::gapplication_h::{_GApplication, GApplicationPrivate, _GApplicationPrivate};
pub use self::gtlscertificate_h::{
    _GTlsCertificate, GTlsCertificatePrivate, _GTlsCertificatePrivate,
};
pub use self::WebKitCredential_h::{
    WebKitCredential, WebKitCredentialPersistence,
    WEBKIT_CREDENTIAL_PERSISTENCE_PERMANENT, WEBKIT_CREDENTIAL_PERSISTENCE_FOR_SESSION,
    WEBKIT_CREDENTIAL_PERSISTENCE_NONE, _WebKitCredential, webkit_credential_new,
    webkit_credential_free,
};
pub use self::cairo_h::{
    cairo_t, _cairo_rectangle_int, cairo_rectangle_int_t, cairo_region_t, _cairo,
    _cairo_region,
};
pub use self::gdktypes_h::{
    GdkRectangle, GdkAtom, GdkDevice, GdkDragContext, GdkWindow, _GdkAtom, _GdkDevice,
    _GdkDragContext, _GdkWindow,
};
pub use self::gdkevents_h::{
    _GdkEventAny, GdkEventType, GDK_EVENT_LAST, GDK_PAD_GROUP_MODE, GDK_PAD_STRIP,
    GDK_PAD_RING, GDK_PAD_BUTTON_RELEASE, GDK_PAD_BUTTON_PRESS, GDK_TOUCHPAD_PINCH,
    GDK_TOUCHPAD_SWIPE, GDK_TOUCH_CANCEL, GDK_TOUCH_END, GDK_TOUCH_UPDATE,
    GDK_TOUCH_BEGIN, GDK_DAMAGE, GDK_GRAB_BROKEN, GDK_OWNER_CHANGE, GDK_SETTING,
    GDK_WINDOW_STATE, GDK_SCROLL, GDK_VISIBILITY_NOTIFY, GDK_CLIENT_EVENT,
    GDK_DROP_FINISHED, GDK_DROP_START, GDK_DRAG_STATUS, GDK_DRAG_MOTION, GDK_DRAG_LEAVE,
    GDK_DRAG_ENTER, GDK_PROXIMITY_OUT, GDK_PROXIMITY_IN, GDK_SELECTION_NOTIFY,
    GDK_SELECTION_REQUEST, GDK_SELECTION_CLEAR, GDK_PROPERTY_NOTIFY, GDK_UNMAP, GDK_MAP,
    GDK_CONFIGURE, GDK_FOCUS_CHANGE, GDK_LEAVE_NOTIFY, GDK_ENTER_NOTIFY, GDK_KEY_RELEASE,
    GDK_KEY_PRESS, GDK_BUTTON_RELEASE, GDK_TRIPLE_BUTTON_PRESS, GDK_3BUTTON_PRESS,
    GDK_DOUBLE_BUTTON_PRESS, GDK_2BUTTON_PRESS, GDK_BUTTON_PRESS, GDK_MOTION_NOTIFY,
    GDK_EXPOSE, GDK_DESTROY, GDK_DELETE, GDK_NOTHING, GdkEventAny, _GdkEventExpose,
    GdkEventExpose, _GdkEventVisibility, GdkVisibilityState,
    GDK_VISIBILITY_FULLY_OBSCURED, GDK_VISIBILITY_PARTIAL, GDK_VISIBILITY_UNOBSCURED,
    GdkEventVisibility, _GdkEventMotion, GdkEventMotion, _GdkEventButton, GdkEventButton,
    _GdkEventTouch, GdkEventSequence, GdkEventTouch, _GdkEventScroll, GdkScrollDirection,
    GDK_SCROLL_SMOOTH, GDK_SCROLL_RIGHT, GDK_SCROLL_LEFT, GDK_SCROLL_DOWN, GDK_SCROLL_UP,
    GdkEventScroll, _GdkEventKey, GdkEventKey, _GdkEventFocus, GdkEventFocus,
    _GdkEventCrossing, GdkNotifyType, GDK_NOTIFY_UNKNOWN, GDK_NOTIFY_NONLINEAR_VIRTUAL,
    GDK_NOTIFY_NONLINEAR, GDK_NOTIFY_INFERIOR, GDK_NOTIFY_VIRTUAL, GDK_NOTIFY_ANCESTOR,
    GdkCrossingMode, GDK_CROSSING_DEVICE_SWITCH, GDK_CROSSING_TOUCH_END,
    GDK_CROSSING_TOUCH_BEGIN, GDK_CROSSING_STATE_CHANGED, GDK_CROSSING_GTK_UNGRAB,
    GDK_CROSSING_GTK_GRAB, GDK_CROSSING_UNGRAB, GDK_CROSSING_GRAB, GDK_CROSSING_NORMAL,
    GdkEventCrossing, _GdkEventConfigure, GdkEventConfigure, _GdkEventProperty,
    GdkEventProperty, _GdkEventSelection, GdkEventSelection, _GdkEventOwnerChange,
    GdkOwnerChange, GDK_OWNER_CHANGE_CLOSE, GDK_OWNER_CHANGE_DESTROY,
    GDK_OWNER_CHANGE_NEW_OWNER, GdkEventOwnerChange, _GdkEventProximity,
    GdkEventProximity, _GdkEventDND, GdkEventDND, _GdkEventWindowState, GdkWindowState,
    GDK_WINDOW_STATE_LEFT_RESIZABLE, GDK_WINDOW_STATE_LEFT_TILED,
    GDK_WINDOW_STATE_BOTTOM_RESIZABLE, GDK_WINDOW_STATE_BOTTOM_TILED,
    GDK_WINDOW_STATE_RIGHT_RESIZABLE, GDK_WINDOW_STATE_RIGHT_TILED,
    GDK_WINDOW_STATE_TOP_RESIZABLE, GDK_WINDOW_STATE_TOP_TILED, GDK_WINDOW_STATE_TILED,
    GDK_WINDOW_STATE_FOCUSED, GDK_WINDOW_STATE_BELOW, GDK_WINDOW_STATE_ABOVE,
    GDK_WINDOW_STATE_FULLSCREEN, GDK_WINDOW_STATE_STICKY, GDK_WINDOW_STATE_MAXIMIZED,
    GDK_WINDOW_STATE_ICONIFIED, GDK_WINDOW_STATE_WITHDRAWN, GdkEventWindowState,
    _GdkEventSetting, GdkSettingAction, GDK_SETTING_ACTION_DELETED,
    GDK_SETTING_ACTION_CHANGED, GDK_SETTING_ACTION_NEW, GdkEventSetting,
    _GdkEventGrabBroken, GdkEventGrabBroken, _GdkEventTouchpadSwipe,
    GdkEventTouchpadSwipe, _GdkEventTouchpadPinch, GdkEventTouchpadPinch,
    _GdkEventPadButton, GdkEventPadButton, _GdkEventPadAxis, GdkEventPadAxis,
    _GdkEventPadGroupMode, GdkEventPadGroupMode, _GdkEvent, GdkEvent, _GdkEventSequence,
    gdk_event_get_scroll_deltas,
};
pub use self::gtkenums_h::{
    GtkAlign, GTK_ALIGN_BASELINE, GTK_ALIGN_CENTER, GTK_ALIGN_END, GTK_ALIGN_START,
    GTK_ALIGN_FILL, GtkIconSize, GTK_ICON_SIZE_DIALOG, GTK_ICON_SIZE_DND,
    GTK_ICON_SIZE_BUTTON, GTK_ICON_SIZE_LARGE_TOOLBAR, GTK_ICON_SIZE_SMALL_TOOLBAR,
    GTK_ICON_SIZE_MENU, GTK_ICON_SIZE_INVALID, GtkPositionType, GTK_POS_BOTTOM,
    GTK_POS_TOP, GTK_POS_RIGHT, GTK_POS_LEFT,
};
pub use self::gtkwidget_h::{
    _GtkWidget, GtkWidgetPrivate, _GtkWidgetPrivate, gtk_widget_get_type,
    gtk_widget_destroy, gtk_widget_show, gtk_widget_show_all, gtk_widget_set_hexpand,
    gtk_widget_set_vexpand, gtk_widget_set_halign, gtk_widget_set_valign,
};
pub use self::gtktypes_h::{GtkWidget, GtkWindow};
pub use self::gtkwindow_h::{
    _GtkWindow, GtkWindowPrivate, _GtkWindowPrivate, gtk_window_get_type,
    gtk_window_set_title, gtk_window_set_resizable, gtk_window_set_icon_name,
};
pub use self::gtkbin_h::{
    GtkBin, _GtkBin, GtkBinPrivate, _GtkBinPrivate, gtk_bin_get_type, gtk_bin_get_child,
};
pub use self::gtkcontainer_h::{
    GtkContainer, _GtkContainer, GtkContainerPrivate, _GtkContainerPrivate,
    gtk_container_get_type, gtk_container_set_border_width,
};
pub use self::gtkapplication_h::{
    _GtkApplication, GtkApplicationPrivate, GtkApplication, _GtkApplicationPrivate,
};
pub use self::gtkdialog_h::{
    C2RustUnnamed_0, GTK_RESPONSE_HELP, GTK_RESPONSE_APPLY, GTK_RESPONSE_NO,
    GTK_RESPONSE_YES, GTK_RESPONSE_CLOSE, GTK_RESPONSE_CANCEL, GTK_RESPONSE_OK,
    GTK_RESPONSE_DELETE_EVENT, GTK_RESPONSE_ACCEPT, GTK_RESPONSE_REJECT,
    GTK_RESPONSE_NONE, _GtkDialog, GtkDialogPrivate, GtkDialog, _GtkDialogPrivate,
    gtk_dialog_get_type, gtk_dialog_new, gtk_dialog_add_buttons,
    gtk_dialog_set_default_response, gtk_dialog_get_content_area,
};
pub use self::gtkmisc_h::{_GtkMisc, GtkMiscPrivate, GtkMisc, _GtkMiscPrivate};
pub use self::gtklabel_h::{
    _GtkLabel, GtkLabelPrivate, GtkLabel, _GtkLabelPrivate, gtk_label_get_type,
    gtk_label_new, gtk_label_set_line_wrap,
};
pub use self::gtkbox_h::{
    _GtkBox, GtkBoxPrivate, GtkBox, _GtkBoxPrivate, gtk_box_get_type, gtk_box_pack_start,
};
pub use self::gtkentry_h::{
    _GtkEntry, GtkEntryPrivate, GtkEntry, _GtkEntryPrivate, gtk_entry_get_type,
    gtk_entry_new, gtk_entry_set_visibility, gtk_entry_set_activates_default,
    gtk_entry_set_text, gtk_entry_get_text,
};
pub use self::gtkbutton_h::{_GtkButton, GtkButtonPrivate, GtkButton, _GtkButtonPrivate};
pub use self::gtktogglebutton_h::{
    _GtkToggleButton, GtkToggleButtonPrivate, GtkToggleButton, _GtkToggleButtonPrivate,
    gtk_toggle_button_get_type, gtk_toggle_button_get_active,
};
pub use self::gtkcssprovider_h::{
    _GtkCssProvider, GtkCssProviderPrivate, GtkCssProvider, _GtkCssProviderPrivate,
};
pub use self::gtkgrid_h::{
    _GtkGrid, GtkGridPrivate, GtkGrid, _GtkGridPrivate, gtk_grid_get_type, gtk_grid_new,
    gtk_grid_attach, gtk_grid_attach_next_to, gtk_grid_set_row_homogeneous,
    gtk_grid_set_row_spacing, gtk_grid_set_column_homogeneous,
    gtk_grid_set_column_spacing,
};
pub use self::gtkaction_h::{
    _GtkAction, GtkActionPrivate, GtkAction, _GtkActionPrivate, gtk_action_new,
    gtk_action_get_label,
};
pub use self::WebKitAuthenticationRequest_h::{
    _WebKitAuthenticationRequest, WebKitAuthenticationRequestPrivate,
    WebKitAuthenticationRequest, _WebKitAuthenticationRequestPrivate,
    webkit_authentication_request_get_host, webkit_authentication_request_authenticate,
};
pub use self::WebKitBackForwardListItem_h::{
    _WebKitBackForwardListItem, WebKitBackForwardListItemPrivate,
    WebKitBackForwardListItem, _WebKitBackForwardListItemPrivate,
    webkit_back_forward_list_item_get_uri, webkit_back_forward_list_item_get_title,
};
pub use self::WebKitBackForwardList_h::{
    _WebKitBackForwardList, WebKitBackForwardListPrivate, WebKitBackForwardList,
    _WebKitBackForwardListPrivate, webkit_back_forward_list_get_current_item,
    webkit_back_forward_list_get_nth_item, webkit_back_forward_list_get_back_list,
    webkit_back_forward_list_get_forward_list,
};
pub use self::WebKitContextMenu_h::{
    _WebKitContextMenu, WebKitContextMenuPrivate, WebKitContextMenu,
    WebKitContextMenuItem, _WebKitContextMenuPrivate, webkit_context_menu_new,
    webkit_context_menu_append, webkit_context_menu_get_n_items,
    webkit_context_menu_get_item_at_position, webkit_context_menu_remove_all,
};
pub use self::WebKitContextMenuItem_h::{
    _WebKitContextMenuItem, WebKitContextMenuItemPrivate, _WebKitContextMenuItemPrivate,
    webkit_context_menu_item_new,
    webkit_context_menu_item_new_from_stock_action_with_label,
    webkit_context_menu_item_new_with_submenu, webkit_context_menu_item_new_separator,
    webkit_context_menu_item_get_action, webkit_context_menu_item_get_stock_action,
    webkit_context_menu_item_is_separator, webkit_context_menu_item_get_submenu,
};
pub use self::WebKitContextMenuActions_h::{
    WebKitContextMenuAction, WEBKIT_CONTEXT_MENU_ACTION_CUSTOM,
    WEBKIT_CONTEXT_MENU_ACTION_PASTE_AS_PLAIN_TEXT,
    WEBKIT_CONTEXT_MENU_ACTION_INSERT_EMOJI,
    WEBKIT_CONTEXT_MENU_ACTION_DOWNLOAD_AUDIO_TO_DISK,
    WEBKIT_CONTEXT_MENU_ACTION_DOWNLOAD_VIDEO_TO_DISK,
    WEBKIT_CONTEXT_MENU_ACTION_MEDIA_MUTE, WEBKIT_CONTEXT_MENU_ACTION_MEDIA_PAUSE,
    WEBKIT_CONTEXT_MENU_ACTION_MEDIA_PLAY,
    WEBKIT_CONTEXT_MENU_ACTION_ENTER_VIDEO_FULLSCREEN,
    WEBKIT_CONTEXT_MENU_ACTION_TOGGLE_MEDIA_LOOP,
    WEBKIT_CONTEXT_MENU_ACTION_TOGGLE_MEDIA_CONTROLS,
    WEBKIT_CONTEXT_MENU_ACTION_COPY_AUDIO_LINK_TO_CLIPBOARD,
    WEBKIT_CONTEXT_MENU_ACTION_COPY_VIDEO_LINK_TO_CLIPBOARD,
    WEBKIT_CONTEXT_MENU_ACTION_OPEN_AUDIO_IN_NEW_WINDOW,
    WEBKIT_CONTEXT_MENU_ACTION_OPEN_VIDEO_IN_NEW_WINDOW,
    WEBKIT_CONTEXT_MENU_ACTION_INSPECT_ELEMENT, WEBKIT_CONTEXT_MENU_ACTION_OUTLINE,
    WEBKIT_CONTEXT_MENU_ACTION_UNDERLINE, WEBKIT_CONTEXT_MENU_ACTION_ITALIC,
    WEBKIT_CONTEXT_MENU_ACTION_BOLD, WEBKIT_CONTEXT_MENU_ACTION_FONT_MENU,
    WEBKIT_CONTEXT_MENU_ACTION_IGNORE_GRAMMAR, WEBKIT_CONTEXT_MENU_ACTION_LEARN_SPELLING,
    WEBKIT_CONTEXT_MENU_ACTION_IGNORE_SPELLING,
    WEBKIT_CONTEXT_MENU_ACTION_NO_GUESSES_FOUND,
    WEBKIT_CONTEXT_MENU_ACTION_SPELLING_GUESS, WEBKIT_CONTEXT_MENU_ACTION_UNICODE,
    WEBKIT_CONTEXT_MENU_ACTION_INPUT_METHODS, WEBKIT_CONTEXT_MENU_ACTION_SELECT_ALL,
    WEBKIT_CONTEXT_MENU_ACTION_DELETE, WEBKIT_CONTEXT_MENU_ACTION_PASTE,
    WEBKIT_CONTEXT_MENU_ACTION_CUT, WEBKIT_CONTEXT_MENU_ACTION_COPY,
    WEBKIT_CONTEXT_MENU_ACTION_RELOAD, WEBKIT_CONTEXT_MENU_ACTION_STOP,
    WEBKIT_CONTEXT_MENU_ACTION_GO_FORWARD, WEBKIT_CONTEXT_MENU_ACTION_GO_BACK,
    WEBKIT_CONTEXT_MENU_ACTION_OPEN_FRAME_IN_NEW_WINDOW,
    WEBKIT_CONTEXT_MENU_ACTION_COPY_IMAGE_URL_TO_CLIPBOARD,
    WEBKIT_CONTEXT_MENU_ACTION_COPY_IMAGE_TO_CLIPBOARD,
    WEBKIT_CONTEXT_MENU_ACTION_DOWNLOAD_IMAGE_TO_DISK,
    WEBKIT_CONTEXT_MENU_ACTION_OPEN_IMAGE_IN_NEW_WINDOW,
    WEBKIT_CONTEXT_MENU_ACTION_COPY_LINK_TO_CLIPBOARD,
    WEBKIT_CONTEXT_MENU_ACTION_DOWNLOAD_LINK_TO_DISK,
    WEBKIT_CONTEXT_MENU_ACTION_OPEN_LINK_IN_NEW_WINDOW,
    WEBKIT_CONTEXT_MENU_ACTION_OPEN_LINK, WEBKIT_CONTEXT_MENU_ACTION_NO_ACTION,
};
pub use self::WebKitURIRequest_h::{
    _WebKitURIRequest, WebKitURIRequestPrivate, WebKitURIRequest,
    _WebKitURIRequestPrivate, webkit_uri_request_get_uri,
};
pub use self::WebKitURIResponse_h::{
    _WebKitURIResponse, WebKitURIResponsePrivate, WebKitURIResponse,
    _WebKitURIResponsePrivate, webkit_uri_response_get_uri,
    webkit_uri_response_get_status_code, webkit_uri_response_get_mime_type,
};
pub use self::WebKitDownload_h::{
    _WebKitDownload, WebKitDownloadPrivate, WebKitDownload, WebKitWebView,
    _WebKitDownloadPrivate, webkit_download_get_web_view,
};
pub use self::WebKitWebView_h::{
    _WebKitWebView, WebKitWebViewPrivate, WebKitPolicyDecisionType,
    WEBKIT_POLICY_DECISION_TYPE_RESPONSE, WEBKIT_POLICY_DECISION_TYPE_NEW_WINDOW_ACTION,
    WEBKIT_POLICY_DECISION_TYPE_NAVIGATION_ACTION, WebKitLoadEvent, WEBKIT_LOAD_FINISHED,
    WEBKIT_LOAD_COMMITTED, WEBKIT_LOAD_REDIRECTED, WEBKIT_LOAD_STARTED, WebKitSaveMode,
    WEBKIT_SAVE_MODE_MHTML, /*WEBKIT_TYPE_WEB_VIEW,*/ _WebKitWebViewPrivate,
    webkit_web_view_get_type, webkit_web_view_load_uri,
    webkit_web_view_load_alternate_html, webkit_web_view_stop_loading,
    webkit_web_view_get_page_id, webkit_web_view_reload,
    webkit_web_view_reload_bypass_cache, webkit_web_view_can_go_back,
    webkit_web_view_can_go_forward, webkit_web_view_get_back_forward_list,
    webkit_web_view_go_to_back_forward_list_item, webkit_web_view_get_uri,
    webkit_web_view_get_settings, webkit_web_view_get_find_controller,
    webkit_web_view_get_main_resource, webkit_web_view_get_inspector,
    webkit_web_view_save_to_file, webkit_web_view_save_to_file_finish,
    webkit_web_view_get_tls_info, webkit_web_view_get_session_state,
    webkit_web_view_restore_session_state,
};
pub use self::WebKitWebViewBase_h::{
    WebKitWebViewBase, _WebKitWebViewBase, WebKitWebViewBasePrivate,
    _WebKitWebViewBasePrivate,
};
pub use self::WebKitFeature_h::{
    WebKitFeature, WebKitFeatureList, WebKitFeatureList_autoptr, _WebKitFeature,
    _WebKitFeatureList, webkit_feature_get_identifier, webkit_feature_list_get_length,
    webkit_feature_list_get,
};
pub use self::WebKitFindController_h::{
    _WebKitFindController, WebKitFindControllerPrivate, WebKitFindController,
    C2RustUnnamed_1, WEBKIT_FIND_OPTIONS_WRAP_AROUND, WEBKIT_FIND_OPTIONS_BACKWARDS,
    WEBKIT_FIND_OPTIONS_TREAT_MEDIAL_CAPITAL_AS_WORD_START,
    WEBKIT_FIND_OPTIONS_AT_WORD_STARTS, WEBKIT_FIND_OPTIONS_CASE_INSENSITIVE,
    WEBKIT_FIND_OPTIONS_NONE, _WebKitFindControllerPrivate,
    webkit_find_controller_search, webkit_find_controller_search_finish,
    webkit_find_controller_search_next, webkit_find_controller_search_previous,
};
pub use self::WebKitHitTestResult_h::{
    _WebKitHitTestResult, WebKitHitTestResultPrivate, WebKitHitTestResult,
    C2RustUnnamed_2, WEBKIT_HIT_TEST_RESULT_CONTEXT_SELECTION,
    WEBKIT_HIT_TEST_RESULT_CONTEXT_SCROLLBAR, WEBKIT_HIT_TEST_RESULT_CONTEXT_EDITABLE,
    WEBKIT_HIT_TEST_RESULT_CONTEXT_MEDIA, WEBKIT_HIT_TEST_RESULT_CONTEXT_IMAGE,
    WEBKIT_HIT_TEST_RESULT_CONTEXT_LINK, WEBKIT_HIT_TEST_RESULT_CONTEXT_DOCUMENT,
    _WebKitHitTestResultPrivate, webkit_hit_test_result_get_context,
    webkit_hit_test_result_context_is_link, webkit_hit_test_result_get_link_uri,
};
pub use self::WebKitInstallMissingMediaPluginsPermissionRequest_h::{
    _WebKitInstallMissingMediaPluginsPermissionRequest,
    WebKitInstallMissingMediaPluginsPermissionRequestPrivate,
    WebKitInstallMissingMediaPluginsPermissionRequest,
    _WebKitInstallMissingMediaPluginsPermissionRequestPrivate,
    webkit_install_missing_media_plugins_permission_request_get_type,
    webkit_install_missing_media_plugins_permission_request_get_description,
};
pub use self::WebKitNavigationAction_h::{
    WebKitNavigationType, WEBKIT_NAVIGATION_TYPE_OTHER,
    WEBKIT_NAVIGATION_TYPE_FORM_RESUBMITTED, WEBKIT_NAVIGATION_TYPE_RELOAD,
    WEBKIT_NAVIGATION_TYPE_BACK_FORWARD, WEBKIT_NAVIGATION_TYPE_FORM_SUBMITTED,
    WEBKIT_NAVIGATION_TYPE_LINK_CLICKED, WebKitNavigationAction, _WebKitNavigationAction,
    webkit_navigation_action_get_navigation_type, webkit_navigation_action_get_request,
};
pub use self::WebKitPolicyDecision_h::{
    _WebKitPolicyDecision, WebKitPolicyDecisionPrivate, WebKitPolicyDecision,
    _WebKitPolicyDecisionPrivate, webkit_policy_decision_use,
    webkit_policy_decision_ignore, webkit_policy_decision_download,
};
pub use self::WebKitNavigationPolicyDecision_h::{
    _WebKitNavigationPolicyDecision, WebKitNavigationPolicyDecisionPrivate,
    WebKitNavigationPolicyDecision, _WebKitNavigationPolicyDecisionPrivate,
    webkit_navigation_policy_decision_get_type,
    webkit_navigation_policy_decision_get_navigation_action,
};
pub use self::WebKitPermissionRequest_h::{
    WebKitPermissionRequest, _WebKitPermissionRequest, webkit_permission_request_allow,
    webkit_permission_request_deny,
};
pub use self::WebKitSettings_h::{
    _WebKitSettings, WebKitSettingsPrivate, WebKitSettings,
    WebKitHardwareAccelerationPolicy, WEBKIT_HARDWARE_ACCELERATION_POLICY_NEVER,
    WEBKIT_HARDWARE_ACCELERATION_POLICY_ALWAYS,
    WEBKIT_HARDWARE_ACCELERATION_POLICY_ON_DEMAND, _WebKitSettingsPrivate,
    webkit_settings_get_hardware_acceleration_policy,
    webkit_settings_set_hardware_acceleration_policy,
    webkit_settings_set_feature_enabled, webkit_settings_get_all_features,
};
pub use self::WebKitUserContent_h::{WebKitUserStyleSheet, _WebKitUserStyleSheet};
pub use self::WebKitUserContentManager_h::{
    _WebKitUserContentManager, WebKitUserContentManagerPrivate, WebKitUserContentManager,
    _WebKitUserContentManagerPrivate, webkit_user_content_manager_new,
    webkit_user_content_manager_add_style_sheet,
    webkit_user_content_manager_remove_all_style_sheets,
};
pub use self::WebKitURISchemeRequest_h::{
    _WebKitURISchemeRequest, WebKitURISchemeRequestPrivate, WebKitURISchemeRequest,
    _WebKitURISchemeRequestPrivate, webkit_uri_scheme_request_get_uri,
    webkit_uri_scheme_request_get_web_view,
};
pub use self::WebKitWebContext_h::{
    _WebKitWebContext, WebKitWebContextPrivate, WebKitWebContext,
    _WebKitWebContextPrivate,
};
pub use self::WebKitWebResource_h::{
    _WebKitWebResource, WebKitWebResourcePrivate, WebKitWebResource,
    _WebKitWebResourcePrivate, webkit_web_resource_get_data,
    webkit_web_resource_get_data_finish,
};
pub use self::WebKitWebViewSessionState_h::{
    WebKitWebViewSessionState, _WebKitWebViewSessionState,
    webkit_web_view_session_state_new, webkit_web_view_session_state_unref,
    webkit_web_view_session_state_serialize,
};
pub use self::WebKitWebInspector_h::{
    _WebKitWebInspector, WebKitWebInspectorPrivate, WebKitWebInspector,
    _WebKitWebInspectorPrivate, webkit_web_inspector_show, webkit_web_inspector_close,
};
pub use self::WebKitResponsePolicyDecision_h::{
    _WebKitResponsePolicyDecision, WebKitResponsePolicyDecisionPrivate,
    WebKitResponsePolicyDecision, _WebKitResponsePolicyDecisionPrivate,
    webkit_response_policy_decision_get_type,
    webkit_response_policy_decision_get_response,
    webkit_response_policy_decision_is_mime_type_supported,
};
pub use self::WebKitUserMediaPermissionRequest_h::{
    _WebKitUserMediaPermissionRequest, WebKitUserMediaPermissionRequestPrivate,
    WebKitUserMediaPermissionRequest, _WebKitUserMediaPermissionRequestPrivate,
    webkit_user_media_permission_request_get_type,
    webkit_user_media_permission_is_for_audio_device,
    webkit_user_media_permission_is_for_video_device,
};
pub use self::lua_h::{
    lua_CFunction, lua_Number, lua_Integer, LUA_MULTRET, LUA_REGISTRYINDEX, LUA_TNIL,
    LUA_TBOOLEAN, LUA_TLIGHTUSERDATA, LUA_TNUMBER, LUA_TTABLE, LUA_TFUNCTION, lua_State,
    lua_gettop, lua_settop, lua_pushvalue, lua_remove, lua_insert, lua_isnumber,
    lua_isstring, lua_type, lua_typename, lua_tonumber, lua_tointeger, lua_toboolean,
    lua_tolstring, lua_objlen, lua_touserdata, lua_topointer, lua_pushnil,
    lua_pushnumber, lua_pushinteger, lua_pushlstring, lua_pushstring, lua_pushcclosure,
    lua_pushboolean, lua_pushlightuserdata, lua_rawget, lua_rawgeti, lua_createtable,
    lua_setfield, lua_rawset, lua_rawseti, lua_setmetatable, lua_pcall,
};
pub use self::common_h::{_common_t, common_t, common};
pub use self::globalconf_h::{globalconf_t, globalconf};
pub use self::widget_h::{
    widget_t, widget_destructor_t, widget_info_t, widget_constructor_t,
    GOBJECT_LUAKIT_WIDGET_DATA_KEY, luaH_checkwidget, widget_class,
};
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
pub use self::signal_h::signal_t;
pub use self::log_h::{
    log_level_t, LOG_LEVEL_debug, LOG_LEVEL_verbose, LOG_LEVEL_info, LOG_LEVEL_warn,
    LOG_LEVEL_error, LOG_LEVEL_fatal, _log,
};
pub use self::util_h::{
    LuakitError, LUAKIT_ERROR_TLS, /*LUAKIT_ERROR,*/ file_exists, luaH_callerinfo,
    luakit_error_quark,
};
pub use self::luaclass_h::{
    lua_class_property_array_t, lua_object_t, lua_class_allocator_t,
    lua_class_propfunc_t, lua_class_t, luaH_class_emit_signal, luaH_checkudata,
    luaH_toudata,
};
pub use self::ipc_h::{
    ipc_endpoint_t, _ipc_endpoint_t, ipc_recv_state_t, _ipc_recv_state_t, ipc_header_t,
    _ipc_header_t, ipc_type_t, IPC_TYPE_crash, IPC_TYPE_page_created, IPC_TYPE_log,
    IPC_TYPE_eval_js, IPC_TYPE_extension_init, IPC_TYPE_scroll, IPC_TYPE_lua_ipc,
    IPC_TYPE_lua_require_module, ipc_endpoint_status_t, IPC_ENDPOINT_FREED,
    IPC_ENDPOINT_CONNECTED, IPC_ENDPOINT_DISCONNECTED, ipc_scroll_subtype_t,
    IPC_SCROLL_TYPE_scroll, IPC_SCROLL_TYPE_winresize, IPC_SCROLL_TYPE_docresize,
    _ipc_scroll_t, ipc_scroll_t, ipc_endpoint_new, ipc_endpoint_replace,
    ipc_endpoint_decref, ipc_send_lua, ipc_send,
};
pub use self::auth_c::{
    LuakitAuthData, free_auth_data, luakit_store_password, luakit_find_password,
    response_callback, table_add_entry, show_auth_dialog, session_authenticate,
};
pub use self::stylesheet_h::{lstylesheet_t, luaH_checkstylesheet};
pub use self::property_h::{
    property_t, property_value_t, URI, INT, FLOAT, DOUBLE, CHAR, BOOL,
    luaH_gobject_index, luaH_gobject_newindex,
};
use self::string_h::{memcpy, strcmp, strlen};
use self::gbytes_h::{g_bytes_new, g_bytes_get_data, g_bytes_unref};
use self::gfileutils_h::{g_build_filename, g_path_is_absolute, g_get_current_dir};
use self::gmem_h::{g_free, g_malloc, g_malloc0_n};
pub use self::gstrfuncs_h::{
    g_strdup_inline, g_strrstr, g_strdup, g_strdup_printf, g_strconcat,
};
pub use self::gmessages_h::G_LOG_DOMAIN;
use self::gslice_h::{g_slice_alloc, g_slice_alloc0, g_slice_free1};
use self::gtestutils_h::{
    g_strcmp0, g_assertion_message_expr, g_assertion_message_cmpint,
};
use self::gvaluetypes_h::g_value_set_int;
use self::gfile_h::g_file_new_for_path;
use self::gtkimage_h::gtk_image_new_from_icon_name;
use self::gtkcheckbutton_h::gtk_check_button_new_with_label;
use self::WebKitGeolocationPermissionRequest_h::webkit_geolocation_permission_request_get_type;
use self::WebKitNotificationPermissionRequest_h::webkit_notification_permission_request_get_type;
use self::lauxlib_h::{
    luaL_typerror, luaL_argerror, luaL_checklstring, luaL_checknumber, luaL_error,
};
use self::luautil_h::{luaH_dofunction_on_error, luaH_push_gerror};
pub use self::lualib_h::{luaH_absindex, luaH_dofunction};
pub use self::luaobject_h::{
    luaH_object_registry_push, luaH_object_ref, luaH_object_unref, luaH_object_push,
    luaH_object_incref, luaH_object_decref, luaH_object_emit_signal,
    luaH_object_property_signal,
};
pub use self::luah_h::{luaH_checkboolean, luaH_rawfield};
use self::luakit_luah_h::luaH_modifier_table_push;
use self::widgets_common_h::{
    focus_cb, key_press_cb, luaH_widget_destroy, luaH_widget_focus,
    luaH_widget_get_children, luaH_widget_hide, luaH_widget_show, luaH_widget_replace,
    luaH_widget_send_key, luaH_widget_get_parent, luaH_widget_get_focused,
    luaH_widget_get_visible, luaH_widget_get_width, luaH_widget_get_height,
    luaH_widget_set_visible, luaH_widget_set_tooltip, luaH_widget_get_tooltip,
    luaH_widget_set_min_size, luaH_widget_get_min_size, luaH_widget_set_align,
    luaH_widget_get_align, parent_set_cb, resize_cb, destroy_cb,
};
use self::request_h::luaH_request_push_uri_scheme_request;
use self::web_context_h::{web_context_init_finish, web_context_get};
use self::luayield_h::{luaH_yield_wrap_function, luaH_yield, luaH_resume};
pub use self::gmacros_h::{FALSE, TRUE};
pub use self::__stddef_null_h::{NULL, NULL_0};
pub use self::internal::__INT_MAX__;
use self::luaserialize_h::lua_deserialize_range;
pub use self::javascript_c::{
    run_javascript_finished, run_javascript_webview_closed, luaH_webview_eval_js,
};
use self::download_h::luaH_download_push;
use self::luakit_h::{luakit_lib_get_luakit_class, luaH_luakit_allow_certificate};
pub use self::downloads_c::download_start_cb;
pub use self::history_c::{
    luaH_webview_push_history, luaH_webview_can_go_back, luaH_webview_can_go_forward,
    webview_history_go, luaH_webview_go_back, luaH_webview_go_forward,
    luaH_webview_set_session_state, luaH_webview_push_session_state,
};
pub use self::scroll_c::{
    webview_scroll_recv, luaH_webview_scroll_newindex, luaH_webview_scroll_index,
    luaH_webview_push_scroll_table,
};
pub use self::inspector_c::{
    inspector_open_window_cb, inspector_show_window_cb, inspector_close_window_cb,
    inspector_attach_window_cb, inspector_detach_window_cb, luaH_webview_show_inspector,
    luaH_webview_close_inspector,
};
pub use self::find_controller_c::{found_text_cb, failed_to_find_text_cb};
pub use self::stylesheets_c::{
    inside_stylesheet_cb, webview_stylesheets_regenerate_stylesheet,
    webview_stylesheets_regenerate, webview_stylesheet_set_enabled,
    luaH_webview_stylesheets_index, luaH_webview_stylesheets_newindex,
    luaH_webview_push_stylesheets_table, webview_update_stylesheets,
};
pub use self::limits_h::UINT_MAX;
pub use self::stdbool_h::false_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct webview_data_t {
    pub widget: *mut widget_t,
    pub view: *mut WebKitWebView,
    pub user_content: *mut WebKitUserContentManager,
    pub stylesheets: *mut GList,
    pub stylesheet_added: gboolean,
    pub stylesheet_removed: gboolean,
    pub stylesheet_refreshed: gboolean,
    pub uri: *mut gchar,
    pub hover: *mut gchar,
    pub inspector: *mut WebKitWebInspector,
    pub inspector_open: gboolean,
    pub htr_context: guint,
    pub is_committed: gboolean,
    pub is_failed: gboolean,
    pub private: gboolean,
    pub doc_w: gint,
    pub doc_h: gint,
    pub win_w: gint,
    pub win_h: gint,
    pub scroll_x: gint,
    pub scroll_y: gint,
    pub cert: *mut GTlsCertificate,
    pub ipc: *mut ipc_endpoint_t,
    pub web_process_id: pid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub refs: *mut GSList,
    pub old_refs: *mut GSList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct save_cb_s {
    pub filename: *const gchar,
    pub window: *mut widget_t,
}
static mut related_view: *mut WebKitWebView = 0 as *const WebKitWebView
    as *mut WebKitWebView;
static mut last_popup: C2RustUnnamed_3 = {
    let init = C2RustUnnamed_3 {
        refs: NULL_0 as *mut GSList,
        old_refs: NULL_0 as *mut GSList,
    };
    init
};
static mut webview_properties: [property_t; 7] = [
    {
        let init = property_t {
            tok: L_TK_EDITABLE,
            name: b"editable\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_PROGRESS,
            name: b"estimated-load-progress\0" as *const u8 as *const std::ffi::c_char,
            type_0: DOUBLE,
            writable: FALSE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_IS_LOADING,
            name: b"is-loading\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: FALSE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_IS_PLAYING_AUDIO,
            name: b"is-playing-audio\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: FALSE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_TITLE,
            name: b"title\0" as *const u8 as *const std::ffi::c_char,
            type_0: CHAR,
            writable: FALSE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ZOOM_LEVEL,
            name: b"zoom-level\0" as *const u8 as *const std::ffi::c_char,
            type_0: DOUBLE,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_UNKNOWN,
            name: NULL_0 as *const gchar,
            type_0: BOOL,
            writable: 0 as std::ffi::c_int,
        };
        init
    },
];
static mut webview_settings_properties: [property_t; 48] = [
    {
        let init = property_t {
            tok: L_TK_ALLOW_FILE_ACCESS_FROM_FILE_URLS,
            name: b"allow-file-access-from-file-urls\0" as *const u8
                as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ALLOW_MODAL_DIALOGS,
            name: b"allow-modal-dialogs\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ALLOW_UNIVERSAL_ACCESS_FROM_FILE_URLS,
            name: b"allow-universal-access-from-file-urls\0" as *const u8
                as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_AUTO_LOAD_IMAGES,
            name: b"auto-load-images\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_CURSIVE_FONT_FAMILY,
            name: b"cursive-font-family\0" as *const u8 as *const std::ffi::c_char,
            type_0: CHAR,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_DEFAULT_CHARSET,
            name: b"default-charset\0" as *const u8 as *const std::ffi::c_char,
            type_0: CHAR,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_DEFAULT_FONT_FAMILY,
            name: b"default-font-family\0" as *const u8 as *const std::ffi::c_char,
            type_0: CHAR,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_DEFAULT_FONT_SIZE,
            name: b"default-font-size\0" as *const u8 as *const std::ffi::c_char,
            type_0: INT,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_DEFAULT_MONOSPACE_FONT_SIZE,
            name: b"default-monospace-font-size\0" as *const u8
                as *const std::ffi::c_char,
            type_0: INT,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_DRAW_COMPOSITING_INDICATORS,
            name: b"draw-compositing-indicators\0" as *const u8
                as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_ACCELERATED_2D_CANVAS,
            name: b"enable-accelerated-2d-canvas\0" as *const u8
                as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_CARET_BROWSING,
            name: b"enable-caret-browsing\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_DEVELOPER_EXTRAS,
            name: b"enable-developer-extras\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_DNS_PREFETCHING,
            name: b"enable-dns-prefetching\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_FRAME_FLATTENING,
            name: b"enable-frame-flattening\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_FULLSCREEN,
            name: b"enable-fullscreen\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_HTML5_DATABASE,
            name: b"enable-html5-database\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_HTML5_LOCAL_STORAGE,
            name: b"enable-html5-local-storage\0" as *const u8
                as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_HYPERLINK_AUDITING,
            name: b"enable-hyperlink-auditing\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_JAVA,
            name: b"enable-java\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_JAVASCRIPT,
            name: b"enable-javascript\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_MEDIA_STREAM,
            name: b"enable-media-stream\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_MEDIASOURCE,
            name: b"enable-mediasource\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_PAGE_CACHE,
            name: b"enable-page-cache\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_PLUGINS,
            name: b"enable-plugins\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_RESIZABLE_TEXT_AREAS,
            name: b"enable-resizable-text-areas\0" as *const u8
                as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_SITE_SPECIFIC_QUIRKS,
            name: b"enable-site-specific-quirks\0" as *const u8
                as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_SMOOTH_SCROLLING,
            name: b"enable-smooth-scrolling\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_SPATIAL_NAVIGATION,
            name: b"enable-spatial-navigation\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_WEBGL,
            name: b"enable-webgl\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_TABS_TO_LINKS,
            name: b"enable-tabs-to-links\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_WEBAUDIO,
            name: b"enable-webaudio\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_WRITE_CONSOLE_MESSAGES_TO_STDOUT,
            name: b"enable-write-console-messages-to-stdout\0" as *const u8
                as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_XSS_AUDITOR,
            name: b"enable-xss-auditor\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_FANTASY_FONT_FAMILY,
            name: b"fantasy-font-family\0" as *const u8 as *const std::ffi::c_char,
            type_0: CHAR,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_JAVASCRIPT_CAN_ACCESS_CLIPBOARD,
            name: b"javascript-can-access-clipboard\0" as *const u8
                as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_JAVASCRIPT_CAN_OPEN_WINDOWS_AUTOMATICALLY,
            name: b"javascript-can-open-windows-automatically\0" as *const u8
                as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_MEDIA_PLAYBACK_ALLOWS_INLINE,
            name: b"media-playback-allows-inline\0" as *const u8
                as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_MEDIA_PLAYBACK_REQUIRES_GESTURE,
            name: b"media-playback-requires-user-gesture\0" as *const u8
                as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_MINIMUM_FONT_SIZE,
            name: b"minimum-font-size\0" as *const u8 as *const std::ffi::c_char,
            type_0: INT,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_MONOSPACE_FONT_FAMILY,
            name: b"monospace-font-family\0" as *const u8 as *const std::ffi::c_char,
            type_0: CHAR,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_PICTOGRAPH_FONT_FAMILY,
            name: b"pictograph-font-family\0" as *const u8 as *const std::ffi::c_char,
            type_0: CHAR,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_PRINT_BACKGROUNDS,
            name: b"print-backgrounds\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_SANS_SERIF_FONT_FAMILY,
            name: b"sans-serif-font-family\0" as *const u8 as *const std::ffi::c_char,
            type_0: CHAR,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_SERIF_FONT_FAMILY,
            name: b"serif-font-family\0" as *const u8 as *const std::ffi::c_char,
            type_0: CHAR,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_USER_AGENT,
            name: b"user-agent\0" as *const u8 as *const std::ffi::c_char,
            type_0: CHAR,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ZOOM_TEXT_ONLY,
            name: b"zoom-text-only\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_UNKNOWN,
            name: NULL_0 as *const gchar,
            type_0: BOOL,
            writable: 0 as std::ffi::c_int,
        };
        init
    },
];
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_checkwebview(
    mut L: *mut lua_State,
    mut udx: gint,
) -> *mut widget_t {
    let mut w = luaH_checkwidget(L, udx);
    if (*(*w).info).tok as std::ffi::c_uint
        != L_TK_WEBVIEW as std::ffi::c_int as std::ffi::c_uint
    {
        luaL_argerror(
            L,
            udx,
            b"incorrect widget type (expected webview)\0" as *const u8
                as *const std::ffi::c_char,
        );
    }
    return w;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn webview_get_by_id(mut view_id: guint64) -> *mut widget_t {
    let mut i = 0 as std::ffi::c_int as std::ffi::c_uint;
    while i < (*globalconf.webviews).len {
        let mut w = *((*globalconf.webviews).pdata).offset(i as isize) as *mut widget_t;
        if webkit_web_view_get_page_id(
            g_type_check_instance_cast(
                (*w).widget as *mut GTypeInstance,
                webkit_web_view_get_type(),
            ) as *mut std::ffi::c_void as *mut WebKitWebView,
        ) == view_id
        {
            return w;
        }
        i = i.wrapping_add(1);
        i;
    }
    return NULL_0 as *mut widget_t;
}
unsafe extern "C" fn luaH_webview_load_string(mut L: *mut lua_State) -> gint {
    let mut d = (*luaH_checkwebview(L, 1 as std::ffi::c_int)).data
        as *mut webview_data_t;
    let mut string = luaL_checklstring(L, 2 as std::ffi::c_int, NULL_0 as *mut size_t);
    let mut base_uri = luaL_checklstring(L, 3 as std::ffi::c_int, NULL_0 as *mut size_t);
    webkit_web_view_load_alternate_html(
        (*d).view,
        string,
        base_uri,
        NULL_0 as *const gchar,
    );
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn save_cb(
    mut o: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut view = o as *mut WebKitWebView;
    let mut scbs = user_data as *mut save_cb_s;
    let mut L = common.L;
    let mut err = NULL_0 as *mut GError;
    let mut result: gboolean = 0;
    result = webkit_web_view_save_to_file_finish(view, res, &mut err);
    luaH_object_push(L, (*(*scbs).window).ref_0);
    lua_pushstring(L, (*scbs).filename);
    if result != 0 {
        lua_pushnil(L);
    } else {
        lua_pushstring(L, (*err).message);
    }
    luaH_object_emit_signal(
        L,
        -(3 as std::ffi::c_int),
        b"save-finished\0" as *const u8 as *const std::ffi::c_char,
        2 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    g_free(scbs as gpointer);
}
unsafe extern "C" fn luaH_webview_save(mut L: *mut lua_State) -> gint {
    let mut scbs = g_malloc0_n(
        1 as std::ffi::c_int as gsize,
        ::core::mem::size_of::<save_cb_s>() as std::ffi::c_ulong,
    ) as *mut save_cb_s;
    let mut d = (*luaH_checkwebview(L, 1 as std::ffi::c_int)).data
        as *mut webview_data_t;
    (*scbs).filename = luaL_checklstring(L, 2 as std::ffi::c_int, NULL_0 as *mut size_t);
    (*scbs).window = (*d).widget;
    let mut fd = g_file_new_for_path((*scbs).filename);
    webkit_web_view_save_to_file(
        (*d).view,
        fd,
        WEBKIT_SAVE_MODE_MHTML,
        NULL_0 as *mut GCancellable,
        Some(
            save_cb
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        scbs as gpointer,
    );
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn notify_cb(
    mut UNUSED_v: *mut WebKitWebView,
    mut ps: *mut GParamSpec,
    mut w: *mut widget_t,
) {
    static mut wvprops: *mut GHashTable = NULL_0 as *mut GHashTable;
    let mut p = 0 as *mut property_t;
    if wvprops.is_null() {
        wvprops = g_hash_table_new(
            Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
            Some(
                g_str_equal
                    as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean,
            ),
        );
        p = webview_properties.as_mut_ptr();
        while !((*p).name).is_null() {
            g_hash_table_insert(wvprops, (*p).name as gpointer, p as gpointer);
            p = p.offset(1);
            p;
        }
    }
    p = g_hash_table_lookup(wvprops, (*ps).name as gconstpointer) as *mut property_t;
    if !p.is_null() {
        let mut L = common.L;
        luaH_object_push(L, (*w).ref_0);
        luaH_object_property_signal(L, -(1 as std::ffi::c_int), (*p).tok);
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    }
}
unsafe extern "C" fn update_uri(mut w: *mut widget_t, mut uri: *const gchar) {
    let mut d = (*w).data as *mut webview_data_t;
    if ((*w).destructor).is_none() {
        return;
    }
    if uri.is_null() {
        uri = webkit_web_view_get_uri((*d).view);
        if uri.is_null() || *uri.offset(0 as std::ffi::c_int as isize) == 0 {
            uri = b"about:blank\0" as *const u8 as *const std::ffi::c_char;
        }
    }
    if g_strcmp0((*d).uri, uri) != 0 {
        g_free((*d).uri as gpointer);
        (*d).uri = g_strdup_inline(uri);
        let mut L = common.L;
        luaH_object_push(L, (*w).ref_0);
        luaH_object_emit_signal(
            L,
            -(1 as std::ffi::c_int),
            b"property::uri\0" as *const u8 as *const std::ffi::c_char,
            0 as std::ffi::c_int,
            0 as std::ffi::c_int,
        );
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    }
}
unsafe extern "C" fn load_failed_cb(
    mut UNUSED_v: *mut WebKitWebView,
    mut UNUSED_e: WebKitLoadEvent,
    mut failing_uri: *mut gchar,
    mut error: *mut GError,
    mut w: *mut widget_t,
) -> gboolean {
    update_uri(w, failing_uri);
    let mut L = common.L;
    (*((*w).data as *mut webview_data_t)).is_failed = TRUE;
    luaH_object_push(L, (*w).ref_0);
    lua_pushstring(L, b"failed\0" as *const u8 as *const std::ffi::c_char);
    lua_pushstring(L, failing_uri);
    luaH_push_gerror(L, error);
    let mut ret = luaH_object_emit_signal(
        L,
        -(4 as std::ffi::c_int),
        b"load-status\0" as *const u8 as *const std::ffi::c_char,
        3 as std::ffi::c_int,
        1 as std::ffi::c_int,
    );
    let mut ignore = (ret != 0 && lua_toboolean(L, -(1 as std::ffi::c_int)) != 0)
        as std::ffi::c_int;
    lua_settop(L, -(ret + 1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    return ignore;
}
unsafe extern "C" fn luaH_webview_push_certificate_flags(
    mut L: *mut lua_State,
    mut errors: GTlsCertificateFlags,
) -> std::ffi::c_int {
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    let mut n = 1 as std::ffi::c_int;
    if errors as std::ffi::c_uint
        & G_TLS_CERTIFICATE_UNKNOWN_CA as std::ffi::c_int as std::ffi::c_uint != 0
    {
        lua_pushlstring(
            L,
            b"unknown-ca\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 11]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        let fresh0 = n;
        n = n + 1;
        lua_rawseti(L, -(2 as std::ffi::c_int), fresh0);
    }
    if errors as std::ffi::c_uint
        & G_TLS_CERTIFICATE_BAD_IDENTITY as std::ffi::c_int as std::ffi::c_uint != 0
    {
        lua_pushlstring(
            L,
            b"bad-identity\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 13]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        let fresh1 = n;
        n = n + 1;
        lua_rawseti(L, -(2 as std::ffi::c_int), fresh1);
    }
    if errors as std::ffi::c_uint
        & G_TLS_CERTIFICATE_NOT_ACTIVATED as std::ffi::c_int as std::ffi::c_uint != 0
    {
        lua_pushlstring(
            L,
            b"not-activated\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 14]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        let fresh2 = n;
        n = n + 1;
        lua_rawseti(L, -(2 as std::ffi::c_int), fresh2);
    }
    if errors as std::ffi::c_uint
        & G_TLS_CERTIFICATE_EXPIRED as std::ffi::c_int as std::ffi::c_uint != 0
    {
        lua_pushlstring(
            L,
            b"expired\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 8]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        let fresh3 = n;
        n = n + 1;
        lua_rawseti(L, -(2 as std::ffi::c_int), fresh3);
    }
    if errors as std::ffi::c_uint
        & G_TLS_CERTIFICATE_REVOKED as std::ffi::c_int as std::ffi::c_uint != 0
    {
        lua_pushlstring(
            L,
            b"revoked\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 8]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        let fresh4 = n;
        n = n + 1;
        lua_rawseti(L, -(2 as std::ffi::c_int), fresh4);
    }
    if errors as std::ffi::c_uint
        & G_TLS_CERTIFICATE_INSECURE as std::ffi::c_int as std::ffi::c_uint != 0
    {
        lua_pushlstring(
            L,
            b"insecure\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 9]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        let fresh5 = n;
        n = n + 1;
        lua_rawseti(L, -(2 as std::ffi::c_int), fresh5);
    }
    if errors as std::ffi::c_uint
        & G_TLS_CERTIFICATE_GENERIC_ERROR as std::ffi::c_int as std::ffi::c_uint != 0
    {
        lua_pushlstring(
            L,
            b"generic-error\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 14]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        let fresh6 = n;
        n = n + 1;
        lua_rawseti(L, -(2 as std::ffi::c_int), fresh6);
    }
    return 1 as std::ffi::c_int;
}
unsafe extern "C" fn load_failed_tls_cb(
    mut UNUSED_v: *mut WebKitWebView,
    mut failing_uri: *mut gchar,
    mut certificate: *mut GTlsCertificate,
    mut errors: GTlsCertificateFlags,
    mut w: *mut widget_t,
) -> gboolean {
    let mut L = common.L;
    let mut d = (*w).data as *mut webview_data_t;
    update_uri(w, failing_uri);
    (*((*w).data as *mut webview_data_t)).is_failed = TRUE;
    if !((*d).cert).is_null() {
        g_object_unref(
            g_type_check_instance_cast(
                (*d).cert as *mut GTypeInstance,
                ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
            ) as *mut std::ffi::c_void as *mut GObject as gpointer,
        );
        (*d).cert = NULL_0 as *mut GTlsCertificate;
    }
    (*d).cert = certificate;
    g_object_ref(
        g_type_check_instance_cast(
            (*d).cert as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject as gpointer,
    );
    luaH_object_push(L, (*w).ref_0);
    lua_pushlstring(
        L,
        b"failed\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 7]>() as std::ffi::c_ulong)
            .wrapping_div(
                ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
            )
            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
    );
    lua_pushstring(L, failing_uri);
    let mut error = g_error_new_literal(
        luakit_error_quark(),
        LUAKIT_ERROR_TLS as std::ffi::c_int,
        b"Unacceptable TLS certificate\0" as *const u8 as *const std::ffi::c_char,
    );
    luaH_push_gerror(L, error);
    g_error_free(error);
    luaH_webview_push_certificate_flags(L, errors);
    lua_setfield(
        L,
        -(2 as std::ffi::c_int),
        b"certificate_flags\0" as *const u8 as *const std::ffi::c_char,
    );
    luaH_object_emit_signal(
        L,
        -(4 as std::ffi::c_int),
        b"load-status\0" as *const u8 as *const std::ffi::c_char,
        3 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    return TRUE;
}
unsafe extern "C" fn webview_get_source_finished(
    mut main_resource: *mut WebKitWebResource,
    mut res: *mut GAsyncResult,
    mut L: *mut lua_State,
) {
    let mut length: gsize = 0;
    let mut source: *const gchar = webkit_web_resource_get_data_finish(
        main_resource,
        res,
        &mut length,
        NULL_0 as *mut *mut GError,
    ) as *mut gchar;
    g_object_unref(main_resource as gpointer);
    lua_pushlstring(L, source, length);
    luaH_resume(L, 1 as std::ffi::c_int);
}
unsafe extern "C" fn luaH_webview_push_source(mut L: *mut lua_State) -> gint {
    let mut d = (*luaH_checkwebview(L, 1 as std::ffi::c_int)).data
        as *mut webview_data_t;
    let mut main_resource = webkit_web_view_get_main_resource((*d).view);
    if main_resource.is_null() {
        return 0 as std::ffi::c_int;
    }
    g_object_ref(main_resource as gpointer);
    webkit_web_resource_get_data(
        main_resource,
        NULL_0 as *mut GCancellable,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut WebKitWebResource,
                    *mut GAsyncResult,
                    *mut lua_State,
                ) -> (),
            >,
            GAsyncReadyCallback,
        >(
            Some(
                webview_get_source_finished
                    as unsafe extern "C" fn(
                        *mut WebKitWebResource,
                        *mut GAsyncResult,
                        *mut lua_State,
                    ) -> (),
            ),
        ),
        L as gpointer,
    );
    return luaH_yield(L);
}
unsafe extern "C" fn load_changed_cb(
    mut UNUSED_v: *mut WebKitWebView,
    mut e: WebKitLoadEvent,
    mut w: *mut widget_t,
) {
    let mut d = (*w).data as *mut webview_data_t;
    let mut L = common.L;
    let mut name = NULL_0 as *mut gchar;
    match e as std::ffi::c_uint {
        0 => {
            name = b"provisional\0" as *const u8 as *const std::ffi::c_char
                as *mut gchar;
        }
        1 => {
            name = b"redirected\0" as *const u8 as *const std::ffi::c_char as *mut gchar;
        }
        2 => {
            name = b"committed\0" as *const u8 as *const std::ffi::c_char as *mut gchar;
        }
        3 => {
            name = b"finished\0" as *const u8 as *const std::ffi::c_char as *mut gchar;
        }
        _ => {
            _log(
                LOG_LEVEL_warn,
                b"widgets/webview.c\0" as *const u8 as *const std::ffi::c_char,
                b"programmer error, unable to get load status literal\0" as *const u8
                    as *const std::ffi::c_char,
            );
        }
    }
    update_uri(w, NULL_0 as *const gchar);
    if e as std::ffi::c_uint
        == WEBKIT_LOAD_STARTED as std::ffi::c_int as std::ffi::c_uint
    {
        (*((*w).data as *mut webview_data_t)).is_committed = FALSE;
    } else if e as std::ffi::c_uint
        == WEBKIT_LOAD_COMMITTED as std::ffi::c_int as std::ffi::c_uint
        || e as std::ffi::c_uint
            == WEBKIT_LOAD_FINISHED as std::ffi::c_int as std::ffi::c_uint
    {
        (*((*w).data as *mut webview_data_t)).is_committed = TRUE;
    }
    if e as std::ffi::c_uint
        == WEBKIT_LOAD_STARTED as std::ffi::c_int as std::ffi::c_uint
    {
        if !((*d).cert).is_null() {
            g_object_unref(
                g_type_check_instance_cast(
                    (*d).cert as *mut GTypeInstance,
                    ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
                ) as *mut std::ffi::c_void as *mut GObject as gpointer,
            );
            (*d).cert = NULL_0 as *mut GTlsCertificate;
        }
    } else if e as std::ffi::c_uint
        == WEBKIT_LOAD_COMMITTED as std::ffi::c_int as std::ffi::c_uint
    {
        if ((*d).cert).is_null() {} else {
            g_assertion_message_expr(
                G_LOG_DOMAIN as *const std::ffi::c_char,
                b"widgets/webview.c\0" as *const u8 as *const std::ffi::c_char,
                411 as std::ffi::c_int,
                (*::core::mem::transmute::<
                    &[u8; 16],
                    &[std::ffi::c_char; 16],
                >(b"load_changed_cb\0"))
                    .as_ptr(),
                b"!d->cert\0" as *const u8 as *const std::ffi::c_char,
            );
        }
        webkit_web_view_get_tls_info(
            (*d).view,
            &mut (*d).cert,
            NULL_0 as *mut GTlsCertificateFlags,
        );
        if !((*d).cert).is_null() {
            g_object_ref(
                g_type_check_instance_cast(
                    (*d).cert as *mut GTypeInstance,
                    ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
                ) as *mut std::ffi::c_void as *mut GObject as gpointer,
            );
        }
    }
    if e as std::ffi::c_uint
        == WEBKIT_LOAD_COMMITTED as std::ffi::c_int as std::ffi::c_uint
    {
        webview_update_stylesheets(L, w);
    }
    if e as std::ffi::c_uint
        == WEBKIT_LOAD_STARTED as std::ffi::c_int as std::ffi::c_uint
    {
        (*((*w).data as *mut webview_data_t)).is_failed = FALSE;
    }
    if e as std::ffi::c_uint
        == WEBKIT_LOAD_FINISHED as std::ffi::c_int as std::ffi::c_uint
        && (*((*w).data as *mut webview_data_t)).is_failed != 0
    {
        return;
    }
    luaH_object_push(L, (*w).ref_0);
    lua_pushstring(L, name);
    luaH_object_emit_signal(
        L,
        -(2 as std::ffi::c_int),
        b"load-status\0" as *const u8 as *const std::ffi::c_char,
        1 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
}
unsafe extern "C" fn create_cb(
    mut v: *mut WebKitWebView,
    mut UNUSED_a: *mut WebKitNavigationAction,
    mut w: *mut widget_t,
) -> *mut GtkWidget {
    let mut view = NULL_0 as *mut WebKitWebView;
    let mut new = 0 as *mut widget_t;
    if related_view.is_null() {} else {
        g_assertion_message_expr(
            G_LOG_DOMAIN as *const std::ffi::c_char,
            b"widgets/webview.c\0" as *const u8 as *const std::ffi::c_char,
            439 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 10],
                &[std::ffi::c_char; 10],
            >(b"create_cb\0"))
                .as_ptr(),
            b"!related_view\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    related_view = v;
    let mut L = common.L;
    let mut top = lua_gettop(L);
    luaH_object_push(L, (*w).ref_0);
    let mut ret = luaH_object_emit_signal(
        L,
        -(1 as std::ffi::c_int),
        b"create-web-view\0" as *const u8 as *const std::ffi::c_char,
        0 as std::ffi::c_int,
        1 as std::ffi::c_int,
    );
    related_view = NULL_0 as *mut WebKitWebView;
    if ret != 0 {
        new = luaH_toudata(L, -(1 as std::ffi::c_int), &mut widget_class)
            as *mut widget_t;
        if !new.is_null() {
            if (*(*new).info).tok as std::ffi::c_uint
                == L_TK_WEBVIEW as std::ffi::c_int as std::ffi::c_uint
            {
                view = g_type_check_instance_cast(
                    (*((*new).data as *mut webview_data_t)).view as *mut GTypeInstance,
                    webkit_web_view_get_type(),
                ) as *mut std::ffi::c_void as *mut WebKitWebView;
            } else {
                _log(
                    LOG_LEVEL_warn,
                    b"widgets/webview.c\0" as *const u8 as *const std::ffi::c_char,
                    b"invalid return widget type (expected webview, got %s)\0"
                        as *const u8 as *const std::ffi::c_char,
                    (*(*new).info).name,
                );
            }
        } else {
            _log(
                LOG_LEVEL_warn,
                b"widgets/webview.c\0" as *const u8 as *const std::ffi::c_char,
                b"invalid signal return object type (expected webview widget, got %s)\0"
                    as *const u8 as *const std::ffi::c_char,
                lua_typename(L, lua_type(L, -(1 as std::ffi::c_int))),
            );
        }
    }
    lua_settop(L, top);
    return g_type_check_instance_cast(view as *mut GTypeInstance, gtk_widget_get_type())
        as *mut std::ffi::c_void as *mut GtkWidget;
}
unsafe extern "C" fn decide_policy_cb(
    mut UNUSED_v: *mut WebKitWebView,
    mut p: *mut WebKitPolicyDecision,
    mut type_0: WebKitPolicyDecisionType,
    mut w: *mut widget_t,
) -> gboolean {
    let mut L = common.L;
    match type_0 as std::ffi::c_uint {
        0 | 1 => {
            let mut top = lua_gettop(L);
            let mut np = g_type_check_instance_cast(
                p as *mut GTypeInstance,
                webkit_navigation_policy_decision_get_type(),
            ) as *mut std::ffi::c_void as *mut WebKitNavigationPolicyDecision;
            let mut na = webkit_navigation_policy_decision_get_navigation_action(np);
            let mut signal_name = if type_0 as std::ffi::c_uint
                == WEBKIT_POLICY_DECISION_TYPE_NAVIGATION_ACTION as std::ffi::c_int
                    as std::ffi::c_uint
            {
                b"navigation-request\0" as *const u8 as *const std::ffi::c_char
            } else {
                b"new-window-decision\0" as *const u8 as *const std::ffi::c_char
            };
            let mut uri = webkit_uri_request_get_uri(
                webkit_navigation_action_get_request(na),
            );
            let mut reason = NULL_0 as *mut gchar;
            match webkit_navigation_action_get_navigation_type(na) as std::ffi::c_uint {
                0 => {
                    reason = b"link-clicked\0" as *const u8 as *const std::ffi::c_char
                        as *mut gchar;
                }
                1 => {
                    reason = b"form-submitted\0" as *const u8 as *const std::ffi::c_char
                        as *mut gchar;
                }
                2 => {
                    reason = b"back-forward\0" as *const u8 as *const std::ffi::c_char
                        as *mut gchar;
                }
                3 => {
                    reason = b"reload\0" as *const u8 as *const std::ffi::c_char
                        as *mut gchar;
                }
                4 => {
                    reason = b"form-resubmitted\0" as *const u8
                        as *const std::ffi::c_char as *mut gchar;
                }
                5 => {
                    reason = b"other\0" as *const u8 as *const std::ffi::c_char
                        as *mut gchar;
                }
                _ => {
                    _log(
                        LOG_LEVEL_warn,
                        b"widgets/webview.c\0" as *const u8 as *const std::ffi::c_char,
                        b"programmer error, unable to get web navigation reason literal\0"
                            as *const u8 as *const std::ffi::c_char,
                    );
                }
            }
            luaH_object_push(L, (*w).ref_0);
            lua_pushstring(L, uri);
            lua_pushstring(L, reason);
            let mut ret = luaH_object_emit_signal(
                L,
                -(3 as std::ffi::c_int),
                signal_name,
                2 as std::ffi::c_int,
                1 as std::ffi::c_int,
            );
            let mut ignore = (ret != 0 && lua_toboolean(L, -(1 as std::ffi::c_int)) == 0)
                as std::ffi::c_int;
            if ignore != 0 {
                webkit_policy_decision_ignore(p);
            }
            lua_settop(L, top);
            return ignore;
        }
        2 => {
            let mut rp = g_type_check_instance_cast(
                p as *mut GTypeInstance,
                webkit_response_policy_decision_get_type(),
            ) as *mut std::ffi::c_void as *mut WebKitResponsePolicyDecision;
            let mut r = webkit_response_policy_decision_get_response(rp);
            let mut uri_0 = webkit_uri_response_get_uri(r);
            let mut mime = webkit_uri_response_get_mime_type(r);
            if webkit_uri_response_get_status_code(r) != 0
                && !(webkit_uri_response_get_status_code(r)
                    >= 200 as std::ffi::c_int as guint
                    && webkit_uri_response_get_status_code(r)
                        < 300 as std::ffi::c_int as guint)
            {
                return FALSE;
            }
            luaH_object_push(L, (*w).ref_0);
            lua_pushstring(L, uri_0);
            lua_pushstring(L, mime);
            let mut ret_0 = luaH_object_emit_signal(
                L,
                -(3 as std::ffi::c_int),
                b"mime-type-decision\0" as *const u8 as *const std::ffi::c_char,
                2 as std::ffi::c_int,
                1 as std::ffi::c_int,
            );
            let mut ignore_0 = (ret_0 != 0
                && lua_toboolean(L, -(1 as std::ffi::c_int)) == 0) as std::ffi::c_int;
            if ignore_0 != 0 {
                webkit_policy_decision_ignore(p);
            } else if strcmp(
                mime as *const std::ffi::c_char,
                b"application/x-extension-html\0" as *const u8 as *const std::ffi::c_char,
            ) == 0 as std::ffi::c_int
            {
                webkit_policy_decision_use(p);
            } else if webkit_response_policy_decision_is_mime_type_supported(rp) != 0 {
                webkit_policy_decision_use(p);
            } else {
                webkit_policy_decision_download(p);
            }
            lua_settop(L, -(ret_0 + 1 as std::ffi::c_int) - 1 as std::ffi::c_int);
            return TRUE;
        }
        _ => {}
    }
    return FALSE;
}
unsafe extern "C" fn luaH_webview_reload(mut L: *mut lua_State) -> gint {
    let mut d = (*luaH_checkwebview(L, 1 as std::ffi::c_int)).data
        as *mut webview_data_t;
    webkit_web_view_reload((*d).view);
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn luaH_webview_reload_bypass_cache(mut L: *mut lua_State) -> gint {
    let mut d = (*luaH_checkwebview(L, 1 as std::ffi::c_int)).data
        as *mut webview_data_t;
    webkit_web_view_reload_bypass_cache((*d).view);
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn luaH_webview_search(mut L: *mut lua_State) -> gint {
    let mut d = (*luaH_checkwebview(L, 1 as std::ffi::c_int)).data
        as *mut webview_data_t;
    let mut text = luaL_checklstring(L, 2 as std::ffi::c_int, NULL_0 as *mut size_t);
    let mut case_sensitive = luaH_checkboolean(L, 3 as std::ffi::c_int);
    let mut forward = luaH_checkboolean(L, 4 as std::ffi::c_int);
    let mut wrap = luaH_checkboolean(L, 5 as std::ffi::c_int);
    let mut textlen = strlen(text);
    let mut max_match_count = if textlen < 5 as std::ffi::c_int as size_t {
        100 as std::ffi::c_int as std::ffi::c_uint
    } else {
        G_MAXUINT
    };
    let mut webkit_fc = webkit_web_view_get_find_controller((*d).view);
    webkit_find_controller_search_finish(webkit_fc);
    webkit_find_controller_search(
        webkit_fc,
        text,
        (WEBKIT_FIND_OPTIONS_CASE_INSENSITIVE as std::ffi::c_int
            * (case_sensitive == 0) as std::ffi::c_int
            | WEBKIT_FIND_OPTIONS_BACKWARDS as std::ffi::c_int
                * (forward == 0) as std::ffi::c_int
            | WEBKIT_FIND_OPTIONS_WRAP_AROUND as std::ffi::c_int * wrap) as guint32,
        max_match_count,
    );
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn luaH_webview_search_next(mut L: *mut lua_State) -> gint {
    let mut d = (*luaH_checkwebview(L, 1 as std::ffi::c_int)).data
        as *mut webview_data_t;
    let mut webkit_fc = webkit_web_view_get_find_controller((*d).view);
    webkit_find_controller_search_next(webkit_fc);
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn luaH_webview_search_previous(mut L: *mut lua_State) -> gint {
    let mut d = (*luaH_checkwebview(L, 1 as std::ffi::c_int)).data
        as *mut webview_data_t;
    let mut webkit_fc = webkit_web_view_get_find_controller((*d).view);
    webkit_find_controller_search_previous(webkit_fc);
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn luaH_webview_clear_search(mut L: *mut lua_State) -> gint {
    let mut d = (*luaH_checkwebview(L, 1 as std::ffi::c_int)).data
        as *mut webview_data_t;
    let mut webkit_fc = webkit_web_view_get_find_controller((*d).view);
    webkit_find_controller_search_finish(webkit_fc);
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn luaH_webview_loading(mut L: *mut lua_State) -> gint {
    let mut d = (*luaH_checkwebview(L, 1 as std::ffi::c_int)).data
        as *mut webview_data_t;
    luaH_gobject_index(
        L,
        webview_properties.as_mut_ptr(),
        L_TK_IS_LOADING,
        g_type_check_instance_cast(
            (*d).view as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject,
    );
    return 1 as std::ffi::c_int;
}
unsafe extern "C" fn luaH_webview_stop(mut L: *mut lua_State) -> gint {
    let mut d = (*luaH_checkwebview(L, 1 as std::ffi::c_int)).data
        as *mut webview_data_t;
    webkit_web_view_stop_loading((*d).view);
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn luaH_webview_crash(mut L: *mut lua_State) -> gint {
    let mut d = (*luaH_checkwebview(L, 1 as std::ffi::c_int)).data
        as *mut webview_data_t;
    let mut header = {
        let mut init = _ipc_header_t {
            length: 0 as std::ffi::c_int as guint,
            type_0: IPC_TYPE_crash,
        };
        init
    };
    ipc_send((*d).ipc, &mut header, NULL_0 as *const std::ffi::c_void);
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn luaH_webview_ssl_trusted(mut L: *mut lua_State) -> gint {
    let mut d = (*luaH_checkwebview(L, 1 as std::ffi::c_int)).data
        as *mut webview_data_t;
    let mut uri = webkit_web_view_get_uri((*d).view);
    let mut cert = 0 as *mut GTlsCertificate;
    let mut cert_errors = G_TLS_CERTIFICATE_NO_FLAGS;
    if !uri.is_null() && (*d).is_committed != 0
        && webkit_web_view_get_tls_info((*d).view, &mut cert, &mut cert_errors) != 0
    {
        let mut is_trusted = (cert_errors as std::ffi::c_uint
            == 0 as std::ffi::c_int as std::ffi::c_uint) as std::ffi::c_int;
        lua_pushboolean(L, is_trusted);
        return 1 as std::ffi::c_int;
    }
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn luaH_webview_allow_certificate(mut L: *mut lua_State) -> gint {
    _log(
        LOG_LEVEL_warn,
        b"widgets/webview.c\0" as *const u8 as *const std::ffi::c_char,
        b"webview:allow_certificate() is deprecated: use luakit.allow_certificate() instead\0"
            as *const u8 as *const std::ffi::c_char,
    );
    luaH_checkwebview(L, 1 as std::ffi::c_int);
    lua_remove(L, 1 as std::ffi::c_int);
    luaL_checklstring(L, 1 as std::ffi::c_int, NULL_0 as *mut size_t);
    luaL_checklstring(L, 2 as std::ffi::c_int, NULL_0 as *mut size_t);
    return luaH_luakit_allow_certificate(L);
}
unsafe extern "C" fn luaH_webview_push_certificate(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
) -> gint {
    let mut d = (*w).data as *mut webview_data_t;
    if ((*d).cert).is_null() {
        return 0 as std::ffi::c_int;
    }
    let mut cert_pem = 0 as *mut gchar;
    g_object_get(
        g_type_check_instance_cast(
            (*d).cert as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject as gpointer,
        b"certificate-pem\0" as *const u8 as *const std::ffi::c_char,
        &mut cert_pem as *mut *mut gchar,
        NULL_0 as *mut std::ffi::c_void,
    );
    lua_pushstring(L, cert_pem);
    g_free(cert_pem as gpointer);
    return 1 as std::ffi::c_int;
}
unsafe extern "C" fn webview_translate_old_token(
    mut token: luakit_token_t,
) -> luakit_token_t {
    match token as std::ffi::c_uint {
        76 => return L_TK_ENABLE_JAVASCRIPT,
        _ => return token,
    };
}
unsafe extern "C" fn favicon_cb(
    mut UNUSED_v: *mut WebKitWebView,
    mut UNUSED_param_spec: *mut GParamSpec,
    mut w: *mut widget_t,
) {
    let mut L = common.L;
    luaH_object_push(L, (*w).ref_0);
    luaH_object_emit_signal(
        L,
        -(1 as std::ffi::c_int),
        b"favicon\0" as *const u8 as *const std::ffi::c_char,
        0 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
}
unsafe extern "C" fn uri_cb(
    mut UNUSED_v: *mut WebKitWebView,
    mut UNUSED_param_spec: *mut GParamSpec,
    mut w: *mut widget_t,
) {
    update_uri(w, NULL_0 as *const gchar);
}
unsafe extern "C" fn permission_request_cb(
    mut UNUSED_v: *mut WebKitWebView,
    mut request: *mut WebKitPermissionRequest,
    mut w: *mut widget_t,
) -> gboolean {
    let mut L = common.L;
    let mut top = lua_gettop(L);
    if ({
        let mut __inst = request as *mut GTypeInstance;
        let mut __t = webkit_notification_permission_request_get_type();
        let mut __r: gboolean = 0;
        if __inst.is_null() {
            __r = FALSE;
        } else if !((*__inst).g_class).is_null() && (*(*__inst).g_class).g_type == __t {
            __r = TRUE;
        } else {
            __r = g_type_check_instance_is_a(__inst, __t);
        }
        __r
    }) != 0
    {
        lua_pushlstring(
            L,
            b"notification\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 13]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
    } else if ({
        let mut __inst = request as *mut GTypeInstance;
        let mut __t = webkit_geolocation_permission_request_get_type();
        let mut __r: gboolean = 0;
        if __inst.is_null() {
            __r = FALSE;
        } else if !((*__inst).g_class).is_null() && (*(*__inst).g_class).g_type == __t {
            __r = TRUE;
        } else {
            __r = g_type_check_instance_is_a(__inst, __t);
        }
        __r
    }) != 0
    {
        lua_pushlstring(
            L,
            b"geolocation\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 12]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
    } else if ({
        let mut __inst = request as *mut GTypeInstance;
        let mut __t = webkit_install_missing_media_plugins_permission_request_get_type();
        let mut __r: gboolean = 0;
        if __inst.is_null() {
            __r = FALSE;
        } else if !((*__inst).g_class).is_null() && (*(*__inst).g_class).g_type == __t {
            __r = TRUE;
        } else {
            __r = g_type_check_instance_is_a(__inst, __t);
        }
        __r
    }) != 0
    {
        lua_pushlstring(
            L,
            b"install-missing-media-plugins\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 30]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        let mut ummpr = request
            as *mut WebKitInstallMissingMediaPluginsPermissionRequest;
        lua_pushstring(
            L,
            webkit_install_missing_media_plugins_permission_request_get_description(
                ummpr,
            ),
        );
    } else if ({
        let mut __inst = request as *mut GTypeInstance;
        let mut __t = webkit_user_media_permission_request_get_type();
        let mut __r: gboolean = 0;
        if __inst.is_null() {
            __r = FALSE;
        } else if !((*__inst).g_class).is_null() && (*(*__inst).g_class).g_type == __t {
            __r = TRUE;
        } else {
            __r = g_type_check_instance_is_a(__inst, __t);
        }
        __r
    }) != 0
    {
        lua_pushlstring(
            L,
            b"user-media\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 11]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        lua_createtable(L, 0 as std::ffi::c_int, 2 as std::ffi::c_int);
        let mut umpr = request as *mut WebKitUserMediaPermissionRequest;
        lua_pushboolean(L, webkit_user_media_permission_is_for_audio_device(umpr));
        lua_setfield(
            L,
            -(2 as std::ffi::c_int),
            b"audio\0" as *const u8 as *const std::ffi::c_char,
        );
        lua_pushboolean(L, webkit_user_media_permission_is_for_video_device(umpr));
        lua_setfield(
            L,
            -(2 as std::ffi::c_int),
            b"video\0" as *const u8 as *const std::ffi::c_char,
        );
    } else {
        return FALSE
    }
    let mut argc = lua_gettop(L) - top;
    luaH_object_push(L, (*w).ref_0);
    lua_insert(L, top + 1 as std::ffi::c_int);
    let mut ret = luaH_object_emit_signal(
        L,
        top + 1 as std::ffi::c_int,
        b"permission-request\0" as *const u8 as *const std::ffi::c_char,
        argc,
        1 as std::ffi::c_int,
    );
    if ret != 0 {
        if lua_toboolean(L, -(1 as std::ffi::c_int)) != 0 {
            webkit_permission_request_allow(request);
        } else {
            webkit_permission_request_deny(request);
        }
    }
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    return (ret > 0 as std::ffi::c_int) as std::ffi::c_int;
}
unsafe extern "C" fn luaH_webview_set_pdfjs(mut L: *mut lua_State) -> gint {
    let mut d = (*luaH_checkwebview(L, 1 as std::ffi::c_int)).data
        as *mut webview_data_t;
    let mut enabled = luaH_checkboolean(L, 2 as std::ffi::c_int);
    let mut features = webkit_settings_get_all_features();
    let mut settings = webkit_web_view_get_settings((*d).view);
    let mut i = 0 as std::ffi::c_int as gsize;
    while i < webkit_feature_list_get_length(features) {
        let mut feature = webkit_feature_list_get(features, i);
        if strcmp(
            webkit_feature_get_identifier(feature),
            b"PdfJSViewer\0" as *const u8 as *const std::ffi::c_char,
        ) == 0
        {
            webkit_settings_set_feature_enabled(settings, feature, enabled);
            break;
        } else {
            i = i.wrapping_add(1);
            i;
        }
    }
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn luaH_webview_index(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
    mut token: luakit_token_t,
) -> gint {
    let mut d = (*w).data as *mut webview_data_t;
    let mut ret: gint = 0;
    token = webview_translate_old_token(token);
    match token as std::ffi::c_uint {
        162 => return luaH_widget_get_parent(L, w),
        100 => return luaH_widget_get_focused(L, w),
        253 => return luaH_widget_get_visible(L, w),
        241 => return luaH_widget_get_tooltip(L, w),
        262 => return luaH_widget_get_width(L, w),
        109 => return luaH_widget_get_height(L, w),
        149 => return luaH_widget_get_min_size(L, w),
        3 => return luaH_widget_get_align(L, w),
        24 => return luaH_widget_get_children(L, w),
        211 => {
            lua_pushcclosure(
                L,
                Some(luaH_widget_show as unsafe extern "C" fn(*mut lua_State) -> gint),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        110 => {
            lua_pushcclosure(
                L,
                Some(luaH_widget_hide as unsafe extern "C" fn(*mut lua_State) -> gint),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        99 => {
            lua_pushcclosure(
                L,
                Some(luaH_widget_focus as unsafe extern "C" fn(*mut lua_State) -> gint),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        50 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_widget_destroy as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        183 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_widget_replace as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        203 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_widget_send_key as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        124 => {
            lua_pushboolean(L, (*d).inspector_open);
            return 1 as std::ffi::c_int;
        }
        171 => {
            lua_pushboolean(L, (*d).private);
            return 1 as std::ffi::c_int;
        }
        27 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_webview_clear_search
                        as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        196 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_webview_search as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        197 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_webview_search_next
                        as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        198 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_webview_search_previous
                        as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        105 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_webview_go_back as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        106 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_webview_go_forward
                        as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        18 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_webview_can_go_back
                        as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        19 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_webview_can_go_forward
                        as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        89 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_webview_eval_js as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        138 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_webview_load_string
                        as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        188 => {
            lua_pushcclosure(
                L,
                Some(luaH_webview_save as unsafe extern "C" fn(*mut lua_State) -> gint),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        137 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_webview_loading as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        178 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_webview_reload as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        179 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_webview_reload_bypass_cache
                        as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        222 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_webview_ssl_trusted
                        as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        227 => {
            lua_pushcclosure(
                L,
                Some(luaH_webview_stop as unsafe extern "C" fn(*mut lua_State) -> gint),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        36 => {
            lua_pushcclosure(
                L,
                Some(luaH_webview_crash as unsafe extern "C" fn(*mut lua_State) -> gint),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        214 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_webview_show_inspector
                        as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        31 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_webview_close_inspector
                        as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        4 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_webview_allow_certificate
                        as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        209 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_webview_set_pdfjs
                        as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        113 => {
            lua_pushstring(L, (*d).hover);
            return 1 as std::ffi::c_int;
        }
        246 => {
            lua_pushstring(L, (*d).uri);
            return 1 as std::ffi::c_int;
        }
        261 => {
            lua_pushinteger(L, (*d).web_process_id as lua_Integer);
            return 1 as std::ffi::c_int;
        }
        217 => {
            return luaL_error(
                L,
                b"view.source has been removed; use view:get_source() instead\0"
                    as *const u8 as *const std::ffi::c_char,
            );
        }
        103 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_webview_push_source
                        as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            luaH_yield_wrap_function(L);
            return 1 as std::ffi::c_int;
        }
        205 => return luaH_webview_push_session_state(L, d),
        229 => return luaH_webview_push_stylesheets_table(L),
        117 => {
            lua_pushnumber(L, webkit_web_view_get_page_id((*d).view) as lua_Number);
            return 1 as std::ffi::c_int;
        }
        111 => return luaH_webview_push_history(L, (*d).view),
        191 => return luaH_webview_push_scroll_table(L),
        21 => return luaH_webview_push_certificate(L, w),
        _ => {}
    }
    ret = luaH_gobject_index(
        L,
        webview_properties.as_mut_ptr(),
        token,
        g_type_check_instance_cast(
            (*d).view as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject,
    );
    if ret != 0 {
        return ret;
    }
    if token as std::ffi::c_uint
        == L_TK_HARDWARE_ACCELERATION_POLICY as std::ffi::c_int as std::ffi::c_uint
    {
        match webkit_settings_get_hardware_acceleration_policy(
            webkit_web_view_get_settings((*d).view),
        ) as std::ffi::c_uint
        {
            0 => {
                lua_pushstring(
                    L,
                    b"on-demand\0" as *const u8 as *const std::ffi::c_char,
                );
                return 1 as std::ffi::c_int;
            }
            1 => {
                lua_pushstring(L, b"always\0" as *const u8 as *const std::ffi::c_char);
                return 1 as std::ffi::c_int;
            }
            2 => {
                lua_pushstring(L, b"never\0" as *const u8 as *const std::ffi::c_char);
                return 1 as std::ffi::c_int;
            }
            _ => {
                g_assertion_message_expr(
                    G_LOG_DOMAIN as *const std::ffi::c_char,
                    b"widgets/webview.c\0" as *const u8 as *const std::ffi::c_char,
                    867 as std::ffi::c_int,
                    (*::core::mem::transmute::<
                        &[u8; 19],
                        &[std::ffi::c_char; 19],
                    >(b"luaH_webview_index\0"))
                        .as_ptr(),
                    NULL_0 as *const std::ffi::c_char,
                );
            }
        }
    }
    ret = luaH_gobject_index(
        L,
        webview_settings_properties.as_mut_ptr(),
        token,
        g_type_check_instance_cast(
            webkit_web_view_get_settings((*d).view) as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject,
    );
    if ret != 0 {
        return ret;
    }
    return luaL_error(
        L,
        b"cannot get unknown webview property '%s'\0" as *const u8
            as *const std::ffi::c_char,
        lua_tolstring(L, 2 as std::ffi::c_int, NULL_0 as *mut size_t),
    );
}
unsafe extern "C" fn parse_uri(mut uri: *const gchar) -> *mut gchar {
    if uri.is_null() || *uri.offset(0 as std::ffi::c_int as isize) == 0
        || g_strcmp0(uri, b"about:blank\0" as *const u8 as *const std::ffi::c_char) == 0
    {
        return g_strdup_inline(b"about:blank\0" as *const u8 as *const std::ffi::c_char)
    } else if !(g_strrstr(uri, b"://\0" as *const u8 as *const std::ffi::c_char))
        .is_null()
    {
        return g_strdup_inline(uri)
    } else if file_exists(uri) != 0 {
        if g_path_is_absolute(uri) != 0 {
            return g_strdup_printf(
                b"file://%s\0" as *const u8 as *const std::ffi::c_char,
                uri,
            )
        } else {
            let mut cwd = g_get_current_dir();
            let mut path = g_build_filename(cwd, uri, NULL_0 as *mut std::ffi::c_void);
            let mut new = g_strdup_printf(
                b"file://%s\0" as *const u8 as *const std::ffi::c_char,
                path,
            );
            g_free(cwd as gpointer);
            g_free(path as gpointer);
            return new;
        }
    }
    return g_strdup_printf(b"http://%s\0" as *const u8 as *const std::ffi::c_char, uri);
}
unsafe extern "C" fn luaH_webview_newindex(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
    mut token: luakit_token_t,
) -> gint {
    let mut len: size_t = 0;
    let mut d = (*w).data as *mut webview_data_t;
    let mut uri = 0 as *mut gchar;
    token = webview_translate_old_token(token);
    match token as std::ffi::c_uint {
        253 => {
            luaH_widget_set_visible(L, w);
        }
        241 => {
            luaH_widget_set_tooltip(L, w);
        }
        149 => {
            luaH_widget_set_min_size(L, w);
        }
        3 => {
            luaH_widget_set_align(L, w);
        }
        246 => {
            uri = parse_uri(luaL_checklstring(L, 3 as std::ffi::c_int, &mut len));
            webkit_web_view_load_uri((*d).view, uri);
            update_uri(w, uri);
            g_free(uri as gpointer);
            return 0 as std::ffi::c_int;
        }
        205 => {
            luaH_webview_set_session_state(L, d);
            return 0 as std::ffi::c_int;
        }
        _ => {}
    }
    if token as std::ffi::c_uint
        == L_TK_ZOOM_LEVEL as std::ffi::c_int as std::ffi::c_uint
        && lua_isnumber(L, 3 as std::ffi::c_int) != 0
        && lua_tonumber(L, 3 as std::ffi::c_int) != 1.0f64
    {
        g_object_freeze_notify(
            g_type_check_instance_cast(
                (*d).view as *mut GTypeInstance,
                ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
            ) as *mut std::ffi::c_void as *mut GObject,
        );
        g_object_set(
            (*d).view as gpointer,
            b"zoom-level\0" as *const u8 as *const std::ffi::c_char,
            1.0f64,
            NULL_0 as *mut std::ffi::c_void,
        );
        g_object_thaw_notify(
            g_type_check_instance_cast(
                (*d).view as *mut GTypeInstance,
                ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
            ) as *mut std::ffi::c_void as *mut GObject,
        );
    }
    let mut emit = luaH_gobject_newindex(
        L,
        webview_properties.as_mut_ptr(),
        token,
        3 as std::ffi::c_int,
        g_type_check_instance_cast(
            (*d).view as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject,
    );
    if token as std::ffi::c_uint
        == L_TK_HARDWARE_ACCELERATION_POLICY as std::ffi::c_int as std::ffi::c_uint
    {
        let mut str = luaL_checklstring(L, 3 as std::ffi::c_int, NULL_0 as *mut size_t);
        let mut value = WEBKIT_HARDWARE_ACCELERATION_POLICY_ON_DEMAND;
        if strcmp(str, b"on-demand\0" as *const u8 as *const std::ffi::c_char)
            == 0 as std::ffi::c_int
        {
            value = WEBKIT_HARDWARE_ACCELERATION_POLICY_ON_DEMAND;
        } else if strcmp(str, b"always\0" as *const u8 as *const std::ffi::c_char)
            == 0 as std::ffi::c_int
        {
            value = WEBKIT_HARDWARE_ACCELERATION_POLICY_ALWAYS;
        } else if strcmp(str, b"never\0" as *const u8 as *const std::ffi::c_char)
            == 0 as std::ffi::c_int
        {
            value = WEBKIT_HARDWARE_ACCELERATION_POLICY_NEVER;
        } else {
            return luaL_error(
                L,
                b"invalid value (expected one of 'on-demand', 'always', 'never')\0"
                    as *const u8 as *const std::ffi::c_char,
            )
        }
        webkit_settings_set_hardware_acceleration_policy(
            webkit_web_view_get_settings((*d).view),
            value,
        );
        emit = TRUE;
    }
    if emit == 0 {
        emit = luaH_gobject_newindex(
            L,
            webview_settings_properties.as_mut_ptr(),
            token,
            3 as std::ffi::c_int,
            g_type_check_instance_cast(
                webkit_web_view_get_settings((*d).view) as *mut GTypeInstance,
                ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
            ) as *mut std::ffi::c_void as *mut GObject,
        );
    }
    if emit != 0 {
        return luaH_object_property_signal(L, 1 as std::ffi::c_int, token);
    }
    return luaL_error(
        L,
        b"cannot set unknown webview property '%s'\0" as *const u8
            as *const std::ffi::c_char,
        lua_tolstring(L, 2 as std::ffi::c_int, NULL_0 as *mut size_t),
    );
}
unsafe extern "C" fn expose_cb(
    mut UNUSED_widget: *mut GtkWidget,
    mut UNUSED_e: *mut cairo_t,
    mut w: *mut widget_t,
) -> gboolean {
    let mut L = common.L;
    luaH_object_push(L, (*w).ref_0);
    luaH_object_emit_signal(
        L,
        -(1 as std::ffi::c_int),
        b"expose\0" as *const u8 as *const std::ffi::c_char,
        0 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    return FALSE;
}
unsafe extern "C" fn mouse_target_changed_cb(
    mut UNUSED_v: *mut WebKitWebView,
    mut htr: *mut WebKitHitTestResult,
    mut UNUSED_modifiers: guint,
    mut w: *mut widget_t,
) {
    let mut L = common.L;
    let mut d = (*w).data as *mut webview_data_t;
    (*d).htr_context = webkit_hit_test_result_get_context(htr);
    let mut link = NULL_0 as *const std::ffi::c_char;
    if webkit_hit_test_result_context_is_link(htr) != 0 {
        link = webkit_hit_test_result_get_link_uri(htr);
    }
    if !((*d).hover).is_null() && !link.is_null() && strcmp((*d).hover, link) == 0 {
        return;
    }
    luaH_object_push(L, (*w).ref_0);
    if !((*d).hover).is_null() {
        lua_pushstring(L, (*d).hover);
        g_free((*d).hover as gpointer);
        luaH_object_emit_signal(
            L,
            -(2 as std::ffi::c_int),
            b"link-unhover\0" as *const u8 as *const std::ffi::c_char,
            1 as std::ffi::c_int,
            0 as std::ffi::c_int,
        );
    }
    if !link.is_null() {
        (*d).hover = g_strdup_inline(link);
        lua_pushstring(L, (*d).hover);
        luaH_object_emit_signal(
            L,
            -(2 as std::ffi::c_int),
            b"link-hover\0" as *const u8 as *const std::ffi::c_char,
            1 as std::ffi::c_int,
            0 as std::ffi::c_int,
        );
        luaH_object_emit_signal(
            L,
            -(1 as std::ffi::c_int),
            b"property::hovered_uri\0" as *const u8 as *const std::ffi::c_char,
            0 as std::ffi::c_int,
            0 as std::ffi::c_int,
        );
    } else {
        (*d).hover = NULL_0 as *mut gchar;
    }
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
}
unsafe extern "C" fn luaH_push_hit_test(
    mut L: *mut lua_State,
    mut UNUSED_v: *mut WebKitWebView,
    mut w: *mut widget_t,
) -> gint {
    let mut c = (*((*w).data as *mut webview_data_t)).htr_context;
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    let mut name = 0 as *const gchar;
    if c & WEBKIT_HIT_TEST_RESULT_CONTEXT_DOCUMENT as std::ffi::c_int as guint != 0 {
        name = b"document\0" as *const u8 as *const std::ffi::c_char;
        lua_pushstring(L, name);
        lua_pushboolean(L, TRUE);
        lua_rawset(L, -(3 as std::ffi::c_int));
    }
    if c & WEBKIT_HIT_TEST_RESULT_CONTEXT_LINK as std::ffi::c_int as guint != 0 {
        name = b"link\0" as *const u8 as *const std::ffi::c_char;
        lua_pushstring(L, name);
        lua_pushboolean(L, TRUE);
        lua_rawset(L, -(3 as std::ffi::c_int));
    }
    if c & WEBKIT_HIT_TEST_RESULT_CONTEXT_IMAGE as std::ffi::c_int as guint != 0 {
        name = b"image\0" as *const u8 as *const std::ffi::c_char;
        lua_pushstring(L, name);
        lua_pushboolean(L, TRUE);
        lua_rawset(L, -(3 as std::ffi::c_int));
    }
    if c & WEBKIT_HIT_TEST_RESULT_CONTEXT_MEDIA as std::ffi::c_int as guint != 0 {
        name = b"media\0" as *const u8 as *const std::ffi::c_char;
        lua_pushstring(L, name);
        lua_pushboolean(L, TRUE);
        lua_rawset(L, -(3 as std::ffi::c_int));
    }
    if c & WEBKIT_HIT_TEST_RESULT_CONTEXT_EDITABLE as std::ffi::c_int as guint != 0 {
        name = b"editable\0" as *const u8 as *const std::ffi::c_char;
        lua_pushstring(L, name);
        lua_pushboolean(L, TRUE);
        lua_rawset(L, -(3 as std::ffi::c_int));
    }
    if c & WEBKIT_HIT_TEST_RESULT_CONTEXT_SCROLLBAR as std::ffi::c_int as guint != 0 {
        name = b"scrollbar\0" as *const u8 as *const std::ffi::c_char;
        lua_pushstring(L, name);
        lua_pushboolean(L, TRUE);
        lua_rawset(L, -(3 as std::ffi::c_int));
    }
    return 1 as std::ffi::c_int;
}
unsafe extern "C" fn webview_button_cb(
    mut view: *mut GtkWidget,
    mut ev: *mut GdkEventButton,
    mut w: *mut widget_t,
) -> gboolean {
    let mut ret: gint = 0;
    let mut L = common.L;
    luaH_object_push(L, (*w).ref_0);
    luaH_modifier_table_push(L, (*ev).state);
    lua_pushinteger(L, (*ev).button as lua_Integer);
    luaH_push_hit_test(
        L,
        g_type_check_instance_cast(
            view as *mut GTypeInstance,
            webkit_web_view_get_type(),
        ) as *mut std::ffi::c_void as *mut WebKitWebView,
        w,
    );
    match (*ev).type_0 as std::ffi::c_int {
        5 => {
            ret = luaH_object_emit_signal(
                L,
                -(4 as std::ffi::c_int),
                b"button-double-click\0" as *const u8 as *const std::ffi::c_char,
                3 as std::ffi::c_int,
                1 as std::ffi::c_int,
            );
        }
        7 => {
            ret = luaH_object_emit_signal(
                L,
                -(4 as std::ffi::c_int),
                b"button-release\0" as *const u8 as *const std::ffi::c_char,
                3 as std::ffi::c_int,
                1 as std::ffi::c_int,
            );
        }
        _ => {
            ret = luaH_object_emit_signal(
                L,
                -(4 as std::ffi::c_int),
                b"button-press\0" as *const u8 as *const std::ffi::c_char,
                3 as std::ffi::c_int,
                1 as std::ffi::c_int,
            );
        }
    }
    if ret != 0 && lua_toboolean(L, -(1 as std::ffi::c_int)) != 0 {
        lua_settop(L, -(ret + 1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        return TRUE;
    }
    lua_settop(L, -(ret + 1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    return FALSE;
}
unsafe extern "C" fn webview_scroll_cb(
    mut view: *mut GtkWidget,
    mut ev: *mut GdkEventScroll,
    mut w: *mut widget_t,
) -> gboolean {
    let mut dx: std::ffi::c_double = 0.;
    let mut dy: std::ffi::c_double = 0.;
    match (*ev).direction as std::ffi::c_uint {
        0 => {
            dx = 0 as std::ffi::c_int as std::ffi::c_double;
            dy = -(1 as std::ffi::c_int) as std::ffi::c_double;
        }
        1 => {
            dx = 0 as std::ffi::c_int as std::ffi::c_double;
            dy = 1 as std::ffi::c_int as std::ffi::c_double;
        }
        2 => {
            dx = -(1 as std::ffi::c_int) as std::ffi::c_double;
            dy = 0 as std::ffi::c_int as std::ffi::c_double;
        }
        3 => {
            dx = 1 as std::ffi::c_int as std::ffi::c_double;
            dy = 0 as std::ffi::c_int as std::ffi::c_double;
        }
        4 => {
            gdk_event_get_scroll_deltas(ev as *mut GdkEvent, &mut dx, &mut dy);
        }
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN as *const std::ffi::c_char,
                b"widgets/webview.c\0" as *const u8 as *const std::ffi::c_char,
                1091 as std::ffi::c_int,
                (*::core::mem::transmute::<
                    &[u8; 18],
                    &[std::ffi::c_char; 18],
                >(b"webview_scroll_cb\0"))
                    .as_ptr(),
                NULL_0 as *const std::ffi::c_char,
            );
        }
    }
    let mut L = common.L;
    luaH_object_push(L, (*w).ref_0);
    luaH_modifier_table_push(L, (*ev).state);
    lua_pushnumber(L, dx);
    lua_pushnumber(L, dy);
    luaH_push_hit_test(
        L,
        g_type_check_instance_cast(
            view as *mut GTypeInstance,
            webkit_web_view_get_type(),
        ) as *mut std::ffi::c_void as *mut WebKitWebView,
        w,
    );
    let mut ret = luaH_object_emit_signal(
        L,
        -(5 as std::ffi::c_int),
        b"scroll\0" as *const u8 as *const std::ffi::c_char,
        4 as std::ffi::c_int,
        1 as std::ffi::c_int,
    );
    lua_settop(L, -(ret + 1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    return ret;
}
unsafe extern "C" fn menu_item_cb(mut action: *mut GtkAction, mut w: *mut widget_t) {
    let mut L = common.L;
    let mut ref_0 = g_object_get_data(
        g_type_check_instance_cast(
            action as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject,
        b"lua_callback\0" as *const u8 as *const std::ffi::c_char,
    );
    luaH_object_push(L, (*w).ref_0);
    luaH_object_push(L, ref_0);
    luaH_dofunction(L, 1 as std::ffi::c_int, 0 as std::ffi::c_int);
}
unsafe extern "C" fn hide_popup_cb(
    mut UNUSED_v: *mut WebKitWebView,
    mut UNUSED_w: *mut widget_t,
) {
    let mut iter = 0 as *mut GSList;
    let mut L = common.L;
    if !(last_popup.old_refs).is_null() {
        iter = last_popup.old_refs;
        while !iter.is_null() {
            luaH_object_unref(L, (*iter).data);
            iter = (*iter).next;
        }
        g_slist_free(last_popup.old_refs);
        last_popup.old_refs = NULL_0 as *mut GSList;
    }
}
static mut context_menu_actions: *mut GSList = 0 as *const GSList as *mut GSList;
unsafe extern "C" fn table_from_context_menu(
    mut L: *mut lua_State,
    mut menu: *mut WebKitContextMenu,
    mut w: *mut widget_t,
) -> std::ffi::c_int {
    let mut len = webkit_context_menu_get_n_items(menu);
    lua_createtable(L, len as std::ffi::c_int, 0 as std::ffi::c_int);
    let mut i = 1 as std::ffi::c_int as guint;
    while i <= len {
        let mut item = webkit_context_menu_get_item_at_position(
            menu,
            i.wrapping_sub(1 as std::ffi::c_int as guint),
        );
        if webkit_context_menu_item_is_separator(item) != 0 {
            lua_pushboolean(L, TRUE);
        } else {
            let mut action = webkit_context_menu_item_get_action(item);
            let mut stock_action = webkit_context_menu_item_get_stock_action(item);
            let mut submenu = webkit_context_menu_item_get_submenu(item);
            lua_createtable(L, 2 as std::ffi::c_int, 0 as std::ffi::c_int);
            lua_pushstring(L, gtk_action_get_label(action));
            lua_rawseti(L, -(2 as std::ffi::c_int), 1 as std::ffi::c_int);
            if !submenu.is_null() {
                table_from_context_menu(L, submenu, w);
            } else if stock_action as std::ffi::c_uint
                == WEBKIT_CONTEXT_MENU_ACTION_CUSTOM as std::ffi::c_int
                    as std::ffi::c_uint
            {
                context_menu_actions = g_slist_prepend(
                    context_menu_actions,
                    action as gpointer,
                );
                g_object_ref(
                    g_type_check_instance_cast(
                        action as *mut GTypeInstance,
                        ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
                    ) as *mut std::ffi::c_void as *mut GObject as gpointer,
                );
                lua_pushlightuserdata(L, action as *mut std::ffi::c_void);
            } else {
                lua_pushinteger(L, stock_action as lua_Integer);
            }
            lua_rawseti(L, -(2 as std::ffi::c_int), 2 as std::ffi::c_int);
        }
        lua_rawseti(L, -(2 as std::ffi::c_int), i as std::ffi::c_int);
        i = i.wrapping_add(1);
        i;
    }
    return 1 as std::ffi::c_int;
}
unsafe extern "C" fn context_menu_from_table(
    mut L: *mut lua_State,
    mut menu: *mut WebKitContextMenu,
    mut w: *mut widget_t,
) {
    let mut item = 0 as *mut WebKitContextMenuItem;
    let mut submenu = 0 as *mut WebKitContextMenu;
    let mut ref_0 = 0 as *mut std::ffi::c_void;
    let mut label = 0 as *const gchar;
    let mut i: gint = 0;
    let mut len = lua_objlen(L, -(1 as std::ffi::c_int)) as gint;
    i = 1 as std::ffi::c_int;
    while i <= len {
        lua_rawgeti(L, -(1 as std::ffi::c_int), i);
        if lua_type(L, -(1 as std::ffi::c_int)) == LUA_TTABLE
            && lua_objlen(L, -(1 as std::ffi::c_int)) >= 2 as std::ffi::c_int as size_t
        {
            lua_rawgeti(L, -(1 as std::ffi::c_int), 1 as std::ffi::c_int);
            label = lua_tolstring(L, -(1 as std::ffi::c_int), NULL_0 as *mut size_t);
            lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
            lua_rawgeti(L, -(1 as std::ffi::c_int), 2 as std::ffi::c_int);
            if lua_type(L, -(1 as std::ffi::c_int)) == LUA_TTABLE {
                submenu = webkit_context_menu_new();
                item = webkit_context_menu_item_new_with_submenu(label, submenu);
                webkit_context_menu_append(menu, item);
                context_menu_from_table(L, submenu, w);
                lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
            } else if lua_type(L, -(1 as std::ffi::c_int)) == LUA_TFUNCTION {
                let mut action = gtk_action_new(
                    label,
                    label,
                    NULL_0 as *const gchar,
                    NULL_0 as *const gchar,
                );
                item = webkit_context_menu_item_new(action);
                ref_0 = luaH_object_ref(L, -(1 as std::ffi::c_int));
                last_popup.refs = g_slist_prepend(last_popup.refs, ref_0);
                g_object_set_data(
                    g_type_check_instance_cast(
                        action as *mut GTypeInstance,
                        ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
                    ) as *mut std::ffi::c_void as *mut GObject,
                    b"lua_callback\0" as *const u8 as *const std::ffi::c_char,
                    ref_0,
                );
                webkit_context_menu_append(menu, item);
                g_signal_connect_data(
                    action as gpointer,
                    b"activate\0" as *const u8 as *const std::ffi::c_char,
                    ::core::mem::transmute::<
                        Option::<
                            unsafe extern "C" fn(*mut GtkAction, *mut widget_t) -> (),
                        >,
                        GCallback,
                    >(
                        Some(
                            menu_item_cb
                                as unsafe extern "C" fn(*mut GtkAction, *mut widget_t) -> (),
                        ),
                    ),
                    w as gpointer,
                    ::core::mem::transmute::<
                        libc::intptr_t,
                        GClosureNotify,
                    >(NULL_0 as libc::intptr_t),
                    G_CONNECT_DEFAULT,
                );
            } else if lua_type(L, -(1 as std::ffi::c_int)) == LUA_TNUMBER {
                let mut stock_action = lua_tointeger(L, -(1 as std::ffi::c_int))
                    as WebKitContextMenuAction;
                let mut __n1 = stock_action as gint64;
                let mut __n2 = WEBKIT_CONTEXT_MENU_ACTION_CUSTOM as std::ffi::c_int
                    as gint64;
                if !(__n1 != __n2) {
                    g_assertion_message_cmpint(
                        G_LOG_DOMAIN as *const std::ffi::c_char,
                        b"widgets/webview.c\0" as *const u8 as *const std::ffi::c_char,
                        1216 as std::ffi::c_int,
                        (*::core::mem::transmute::<
                            &[u8; 24],
                            &[std::ffi::c_char; 24],
                        >(b"context_menu_from_table\0"))
                            .as_ptr(),
                        b"stock_action != WEBKIT_CONTEXT_MENU_ACTION_CUSTOM\0"
                            as *const u8 as *const std::ffi::c_char,
                        __n1 as guint64,
                        b"!=\0" as *const u8 as *const std::ffi::c_char,
                        __n2 as guint64,
                        'i' as i32 as std::ffi::c_char,
                    );
                }
                item = webkit_context_menu_item_new_from_stock_action_with_label(
                    stock_action,
                    label,
                );
                webkit_context_menu_append(menu, item);
                lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
            } else if lua_type(L, -(1 as std::ffi::c_int)) == LUA_TLIGHTUSERDATA {
                let mut action_0 = lua_topointer(L, -(1 as std::ffi::c_int))
                    as *mut std::ffi::c_void as *mut GtkAction;
                item = webkit_context_menu_item_new(action_0);
                webkit_context_menu_append(menu, item);
                lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
            }
        } else if lua_type(L, -(1 as std::ffi::c_int)) == LUA_TBOOLEAN
            && lua_toboolean(L, -(1 as std::ffi::c_int)) != 0
        {
            item = webkit_context_menu_item_new_separator();
            webkit_context_menu_append(menu, item);
        }
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        i += 1;
        i;
    }
}
unsafe extern "C" fn context_menu_cb(
    mut UNUSED_v: *mut WebKitWebView,
    mut menu: *mut WebKitContextMenu,
    mut UNUSED_e: *mut GdkEvent,
    mut UNUSED_htr: *mut WebKitHitTestResult,
    mut w: *mut widget_t,
) -> gboolean {
    let mut L = common.L;
    if context_menu_actions.is_null() {} else {
        g_assertion_message_expr(
            G_LOG_DOMAIN as *const std::ffi::c_char,
            b"widgets/webview.c\0" as *const u8 as *const std::ffi::c_char,
            1244 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 16],
                &[std::ffi::c_char; 16],
            >(b"context_menu_cb\0"))
                .as_ptr(),
            b"!context_menu_actions\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    table_from_context_menu(L, menu, w);
    luaH_object_push(L, (*w).ref_0);
    lua_pushvalue(L, -(2 as std::ffi::c_int));
    luaH_object_emit_signal(
        L,
        -(2 as std::ffi::c_int),
        b"populate-popup\0" as *const u8 as *const std::ffi::c_char,
        1 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    last_popup.old_refs = last_popup.refs;
    last_popup.refs = NULL_0 as *mut GSList;
    webkit_context_menu_remove_all(menu);
    context_menu_from_table(L, menu, w);
    g_slist_free_full(
        context_menu_actions,
        Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
    );
    context_menu_actions = NULL_0 as *mut GSList;
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    return FALSE;
}
unsafe extern "C" fn webview_destructor(mut w: *mut widget_t) {
    let mut d = (*w).data as *mut webview_data_t;
    g_idle_remove_by_data(w as gpointer);
    if !((*d).ipc).is_null() {} else {
        g_assertion_message_expr(
            G_LOG_DOMAIN as *const std::ffi::c_char,
            b"widgets/webview.c\0" as *const u8 as *const std::ffi::c_char,
            1272 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 19],
                &[std::ffi::c_char; 19],
            >(b"webview_destructor\0"))
                .as_ptr(),
            b"d->ipc\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    ipc_endpoint_decref((*d).ipc);
    (*d).ipc = NULL_0 as *mut ipc_endpoint_t;
    g_ptr_array_remove(globalconf.webviews, w as gpointer);
    g_free((*d).uri as gpointer);
    g_free((*d).hover as gpointer);
    g_object_unref(
        g_type_check_instance_cast(
            (*d).user_content as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject as gpointer,
    );
    if !((*d).cert).is_null() {
        g_object_unref(
            g_type_check_instance_cast(
                (*d).cert as *mut GTypeInstance,
                ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
            ) as *mut std::ffi::c_void as *mut GObject as gpointer,
        );
    }
    g_slice_free1(
        ::core::mem::size_of::<webview_data_t>() as std::ffi::c_ulong,
        d as gpointer,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luakit_uri_scheme_request_cb(
    mut request: *mut WebKitURISchemeRequest,
    mut scheme: *const gchar,
) {
    let mut uri = webkit_uri_scheme_request_get_uri(request);
    let mut view = webkit_uri_scheme_request_get_web_view(request);
    if view.is_null() {
        return;
    }
    let mut w = g_object_get_data(
        g_type_check_instance_cast(
            view as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject,
        GOBJECT_LUAKIT_WIDGET_DATA_KEY.as_ptr(),
    ) as *mut widget_t;
    let mut L = common.L;
    if !scheme.is_null() {} else {
        g_assertion_message_expr(
            G_LOG_DOMAIN as *const std::ffi::c_char,
            b"widgets/webview.c\0" as *const u8 as *const std::ffi::c_char,
            1298 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[std::ffi::c_char; 29],
            >(b"luakit_uri_scheme_request_cb\0"))
                .as_ptr(),
            b"scheme\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    let mut sig = g_strconcat(
        b"scheme-request::\0" as *const u8 as *const std::ffi::c_char,
        scheme,
        NULL_0 as *mut std::ffi::c_void,
    );
    luaH_object_push(L, (*w).ref_0);
    lua_pushstring(L, uri);
    luaH_request_push_uri_scheme_request(L, request);
    luaH_object_emit_signal(
        L,
        -(3 as std::ffi::c_int),
        sig,
        2 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    g_free(sig as gpointer);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn webview_crashed_cb(
    mut UNUSED_view: *mut WebKitWebView,
    mut w: *mut widget_t,
) -> gboolean {
    let mut d = (*w).data as *mut webview_data_t;
    (*d).ipc = ipc_endpoint_new(b"UI\0" as *const u8 as *const std::ffi::c_char);
    let mut L = common.L;
    luaH_object_push(L, (*w).ref_0);
    luaH_object_emit_signal(
        L,
        -(1 as std::ffi::c_int),
        b"crashed\0" as *const u8 as *const std::ffi::c_char,
        0 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn webview_connect_to_endpoint(
    mut w: *mut widget_t,
    mut ipc: *mut ipc_endpoint_t,
) {
    if (*(*w).info).tok as std::ffi::c_uint
        == L_TK_WEBVIEW as std::ffi::c_int as std::ffi::c_uint
    {} else {
        g_assertion_message_expr(
            G_LOG_DOMAIN as *const std::ffi::c_char,
            b"widgets/webview.c\0" as *const u8 as *const std::ffi::c_char,
            1327 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[std::ffi::c_char; 28],
            >(b"webview_connect_to_endpoint\0"))
                .as_ptr(),
            b"w->info->tok == L_TK_WEBVIEW\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    if !ipc.is_null() {} else {
        g_assertion_message_expr(
            G_LOG_DOMAIN as *const std::ffi::c_char,
            b"widgets/webview.c\0" as *const u8 as *const std::ffi::c_char,
            1328 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[std::ffi::c_char; 28],
            >(b"webview_connect_to_endpoint\0"))
                .as_ptr(),
            b"ipc\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    let mut d = (*w).data as *mut webview_data_t;
    (*d).ipc = ipc_endpoint_replace((*d).ipc, ipc);
    let mut L = common.L;
    if (*ipc).creation_notified == 0 {
        (*ipc).creation_notified = TRUE;
        let mut top = lua_gettop(L);
        luaH_object_push(L, (*w).ref_0);
        let mut luakit_class = luakit_lib_get_luakit_class();
        luaH_class_emit_signal(
            L,
            luakit_class,
            b"web-extension-created\0" as *const u8 as *const std::ffi::c_char,
            1 as std::ffi::c_int,
            0 as std::ffi::c_int,
        );
        lua_settop(L, top);
    }
    luaH_object_push(L, (*w).ref_0);
    if !(lua_type(L, -(1 as std::ffi::c_int)) == LUA_TNIL) {
        luaH_object_emit_signal(
            L,
            -(1 as std::ffi::c_int),
            b"web-extension-loaded\0" as *const u8 as *const std::ffi::c_char,
            0 as std::ffi::c_int,
            0 as std::ffi::c_int,
        );
    }
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn webview_get_endpoint(
    mut w: *mut widget_t,
) -> *mut ipc_endpoint_t {
    if (*(*w).info).tok as std::ffi::c_uint
        == L_TK_WEBVIEW as std::ffi::c_int as std::ffi::c_uint
    {} else {
        g_assertion_message_expr(
            G_LOG_DOMAIN as *const std::ffi::c_char,
            b"widgets/webview.c\0" as *const u8 as *const std::ffi::c_char,
            1358 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 21],
                &[std::ffi::c_char; 21],
            >(b"webview_get_endpoint\0"))
                .as_ptr(),
            b"w->info->tok == L_TK_WEBVIEW\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    let mut d = (*w).data as *mut webview_data_t;
    return (*d).ipc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn webview_set_web_process_id(
    mut w: *mut widget_t,
    mut pid: pid_t,
) {
    let mut d = (*w).data as *mut webview_data_t;
    (*d).web_process_id = pid;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn widget_webview(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
    mut UNUSED_token: luakit_token_t,
) -> *mut widget_t {
    (*w)
        .index = Some(
        luaH_webview_index
            as unsafe extern "C" fn(
                *mut lua_State,
                *mut widget_t,
                luakit_token_t,
            ) -> gint,
    );
    (*w)
        .newindex = Some(
        luaH_webview_newindex
            as unsafe extern "C" fn(
                *mut lua_State,
                *mut widget_t,
                luakit_token_t,
            ) -> gint,
    );
    (*w)
        .destructor = Some(
        webview_destructor as unsafe extern "C" fn(*mut widget_t) -> (),
    );
    let mut d = g_slice_alloc0(
        ::core::mem::size_of::<webview_data_t>() as std::ffi::c_ulong,
    ) as *mut webview_data_t;
    (*d).widget = w;
    (*w).data = d as gpointer;
    let mut prop_tbl_idx = luaH_absindex(L, -(4 as std::ffi::c_int));
    if lua_type(L, prop_tbl_idx) == 5 as std::ffi::c_int {} else {
        g_assertion_message_expr(
            G_LOG_DOMAIN as *const std::ffi::c_char,
            b"widgets/webview.c\0" as *const u8 as *const std::ffi::c_char,
            1385 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 15],
                &[std::ffi::c_char; 15],
            >(b"widget_webview\0"))
                .as_ptr(),
            b"lua_istable(L, prop_tbl_idx)\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    lua_pushstring(L, b"private\0" as *const u8 as *const std::ffi::c_char);
    lua_rawget(L, prop_tbl_idx);
    let mut private = if lua_type(L, -(1 as std::ffi::c_int)) == LUA_TNIL {
        FALSE
    } else {
        lua_toboolean(L, -(1 as std::ffi::c_int))
    };
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    (*d).private = private;
    if (globalconf.webviews).is_null() {
        globalconf.webviews = g_ptr_array_new();
    }
    if (globalconf.stylesheets).is_null() {
        globalconf.stylesheets = g_ptr_array_new();
    }
    (*d).stylesheets = NULL_0 as *mut GList;
    web_context_init_finish();
    (*d).user_content = webkit_user_content_manager_new();
    (*d)
        .view = g_object_new(
        webkit_web_view_get_type(),
        b"web-context\0" as *const u8 as *const std::ffi::c_char,
        web_context_get(),
        b"is-ephemeral\0" as *const u8 as *const std::ffi::c_char,
        (*d).private,
        b"user-content-manager\0" as *const u8 as *const std::ffi::c_char,
        (*d).user_content,
        if !related_view.is_null() {
            b"related-view\0" as *const u8 as *const std::ffi::c_char
        } else {
            NULL_0 as *const std::ffi::c_char
        },
        related_view,
        NULL_0 as *mut std::ffi::c_void,
    ) as *mut WebKitWebView;
    (*d).inspector = webkit_web_view_get_inspector((*d).view);
    (*d).is_committed = FALSE;
    (*d).ipc = ipc_endpoint_new(b"UI\0" as *const u8 as *const std::ffi::c_char);
    (*w)
        .widget = g_type_check_instance_cast(
        (*d).view as *mut GTypeInstance,
        gtk_widget_get_type(),
    ) as *mut std::ffi::c_void as *mut GtkWidget;
    g_ptr_array_add(globalconf.webviews, w as gpointer);
    g_object_connect(
        g_type_check_instance_cast(
            (*d).view as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject as gpointer,
        b"signal::destroy\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkWidget, *mut widget_t) -> ()>,
            GCallback,
        >(Some(destroy_cb as unsafe extern "C" fn(*mut GtkWidget, *mut widget_t) -> ())),
        w,
        b"signal::size-allocate\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut GdkRectangle,
                    *mut widget_t,
                ) -> (),
            >,
            GCallback,
        >(
            Some(
                resize_cb
                    as unsafe extern "C" fn(
                        *mut GtkWidget,
                        *mut GdkRectangle,
                        *mut widget_t,
                    ) -> (),
            ),
        ),
        w,
        b"signal::focus-in-event\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut GdkEventFocus,
                    *mut widget_t,
                ) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                focus_cb
                    as unsafe extern "C" fn(
                        *mut GtkWidget,
                        *mut GdkEventFocus,
                        *mut widget_t,
                    ) -> gboolean,
            ),
        ),
        w,
        b"signal::focus-out-event\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut GdkEventFocus,
                    *mut widget_t,
                ) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                focus_cb
                    as unsafe extern "C" fn(
                        *mut GtkWidget,
                        *mut GdkEventFocus,
                        *mut widget_t,
                    ) -> gboolean,
            ),
        ),
        w,
        b"signal::parent-set\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut GtkWidget, *mut GtkWidget, *mut widget_t) -> (),
            >,
            GCallback,
        >(
            Some(
                parent_set_cb
                    as unsafe extern "C" fn(
                        *mut GtkWidget,
                        *mut GtkWidget,
                        *mut widget_t,
                    ) -> (),
            ),
        ),
        w,
        b"signal::button-press-event\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut GdkEventButton,
                    *mut widget_t,
                ) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                webview_button_cb
                    as unsafe extern "C" fn(
                        *mut GtkWidget,
                        *mut GdkEventButton,
                        *mut widget_t,
                    ) -> gboolean,
            ),
        ),
        w,
        b"signal::button-release-event\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut GdkEventButton,
                    *mut widget_t,
                ) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                webview_button_cb
                    as unsafe extern "C" fn(
                        *mut GtkWidget,
                        *mut GdkEventButton,
                        *mut widget_t,
                    ) -> gboolean,
            ),
        ),
        w,
        b"signal::scroll-event\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut GdkEventScroll,
                    *mut widget_t,
                ) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                webview_scroll_cb
                    as unsafe extern "C" fn(
                        *mut GtkWidget,
                        *mut GdkEventScroll,
                        *mut widget_t,
                    ) -> gboolean,
            ),
        ),
        w,
        b"signal::create\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut WebKitWebView,
                    *mut WebKitNavigationAction,
                    *mut widget_t,
                ) -> *mut GtkWidget,
            >,
            GCallback,
        >(
            Some(
                create_cb
                    as unsafe extern "C" fn(
                        *mut WebKitWebView,
                        *mut WebKitNavigationAction,
                        *mut widget_t,
                    ) -> *mut GtkWidget,
            ),
        ),
        w,
        b"signal::web-process-crashed\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut WebKitWebView, *mut widget_t) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                webview_crashed_cb
                    as unsafe extern "C" fn(
                        *mut WebKitWebView,
                        *mut widget_t,
                    ) -> gboolean,
            ),
        ),
        w,
        b"signal::draw\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut cairo_t,
                    *mut widget_t,
                ) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                expose_cb
                    as unsafe extern "C" fn(
                        *mut GtkWidget,
                        *mut cairo_t,
                        *mut widget_t,
                    ) -> gboolean,
            ),
        ),
        w,
        b"signal::mouse-target-changed\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut WebKitWebView,
                    *mut WebKitHitTestResult,
                    guint,
                    *mut widget_t,
                ) -> (),
            >,
            GCallback,
        >(
            Some(
                mouse_target_changed_cb
                    as unsafe extern "C" fn(
                        *mut WebKitWebView,
                        *mut WebKitHitTestResult,
                        guint,
                        *mut widget_t,
                    ) -> (),
            ),
        ),
        w,
        b"signal::key-press-event\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut GdkEventKey,
                    *mut widget_t,
                ) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                key_press_cb
                    as unsafe extern "C" fn(
                        *mut GtkWidget,
                        *mut GdkEventKey,
                        *mut widget_t,
                    ) -> gboolean,
            ),
        ),
        w,
        b"signal::decide-policy\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut WebKitWebView,
                    *mut WebKitPolicyDecision,
                    WebKitPolicyDecisionType,
                    *mut widget_t,
                ) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                decide_policy_cb
                    as unsafe extern "C" fn(
                        *mut WebKitWebView,
                        *mut WebKitPolicyDecision,
                        WebKitPolicyDecisionType,
                        *mut widget_t,
                    ) -> gboolean,
            ),
        ),
        w,
        b"signal::notify\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut WebKitWebView,
                    *mut GParamSpec,
                    *mut widget_t,
                ) -> (),
            >,
            GCallback,
        >(
            Some(
                notify_cb
                    as unsafe extern "C" fn(
                        *mut WebKitWebView,
                        *mut GParamSpec,
                        *mut widget_t,
                    ) -> (),
            ),
        ),
        w,
        b"signal::load-changed\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut WebKitWebView,
                    WebKitLoadEvent,
                    *mut widget_t,
                ) -> (),
            >,
            GCallback,
        >(
            Some(
                load_changed_cb
                    as unsafe extern "C" fn(
                        *mut WebKitWebView,
                        WebKitLoadEvent,
                        *mut widget_t,
                    ) -> (),
            ),
        ),
        w,
        b"signal::load-failed\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut WebKitWebView,
                    WebKitLoadEvent,
                    *mut gchar,
                    *mut GError,
                    *mut widget_t,
                ) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                load_failed_cb
                    as unsafe extern "C" fn(
                        *mut WebKitWebView,
                        WebKitLoadEvent,
                        *mut gchar,
                        *mut GError,
                        *mut widget_t,
                    ) -> gboolean,
            ),
        ),
        w,
        b"signal::load-failed-with-tls-errors\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut WebKitWebView,
                    *mut gchar,
                    *mut GTlsCertificate,
                    GTlsCertificateFlags,
                    *mut widget_t,
                ) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                load_failed_tls_cb
                    as unsafe extern "C" fn(
                        *mut WebKitWebView,
                        *mut gchar,
                        *mut GTlsCertificate,
                        GTlsCertificateFlags,
                        *mut widget_t,
                    ) -> gboolean,
            ),
        ),
        w,
        b"signal::context-menu\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut WebKitWebView,
                    *mut WebKitContextMenu,
                    *mut GdkEvent,
                    *mut WebKitHitTestResult,
                    *mut widget_t,
                ) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                context_menu_cb
                    as unsafe extern "C" fn(
                        *mut WebKitWebView,
                        *mut WebKitContextMenu,
                        *mut GdkEvent,
                        *mut WebKitHitTestResult,
                        *mut widget_t,
                    ) -> gboolean,
            ),
        ),
        w,
        b"signal::context-menu-dismissed\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut WebKitWebView, *mut widget_t) -> ()>,
            GCallback,
        >(
            Some(
                hide_popup_cb
                    as unsafe extern "C" fn(*mut WebKitWebView, *mut widget_t) -> (),
            ),
        ),
        w,
        b"signal::notify::favicon\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut WebKitWebView,
                    *mut GParamSpec,
                    *mut widget_t,
                ) -> (),
            >,
            GCallback,
        >(
            Some(
                favicon_cb
                    as unsafe extern "C" fn(
                        *mut WebKitWebView,
                        *mut GParamSpec,
                        *mut widget_t,
                    ) -> (),
            ),
        ),
        w,
        b"signal::notify::uri\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut WebKitWebView,
                    *mut GParamSpec,
                    *mut widget_t,
                ) -> (),
            >,
            GCallback,
        >(
            Some(
                uri_cb
                    as unsafe extern "C" fn(
                        *mut WebKitWebView,
                        *mut GParamSpec,
                        *mut widget_t,
                    ) -> (),
            ),
        ),
        w,
        b"signal::authenticate\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut WebKitWebView,
                    *mut WebKitAuthenticationRequest,
                    *mut widget_t,
                ) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                session_authenticate
                    as unsafe extern "C" fn(
                        *mut WebKitWebView,
                        *mut WebKitAuthenticationRequest,
                        *mut widget_t,
                    ) -> gboolean,
            ),
        ),
        w,
        b"signal::permission-request\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut WebKitWebView,
                    *mut WebKitPermissionRequest,
                    *mut widget_t,
                ) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                permission_request_cb
                    as unsafe extern "C" fn(
                        *mut WebKitWebView,
                        *mut WebKitPermissionRequest,
                        *mut widget_t,
                    ) -> gboolean,
            ),
        ),
        w,
        NULL_0 as *mut std::ffi::c_void,
    );
    g_object_connect(
        g_type_check_instance_cast(
            webkit_web_view_get_find_controller((*d).view) as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject as gpointer,
        b"signal::found-text\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut WebKitFindController,
                    guint,
                    *mut widget_t,
                ) -> (),
            >,
            GCallback,
        >(
            Some(
                found_text_cb
                    as unsafe extern "C" fn(
                        *mut WebKitFindController,
                        guint,
                        *mut widget_t,
                    ) -> (),
            ),
        ),
        w,
        b"signal::failed-to-find-text\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut WebKitFindController, *mut widget_t) -> (),
            >,
            GCallback,
        >(
            Some(
                failed_to_find_text_cb
                    as unsafe extern "C" fn(
                        *mut WebKitFindController,
                        *mut widget_t,
                    ) -> (),
            ),
        ),
        w,
        NULL_0 as *mut std::ffi::c_void,
    );
    g_object_connect(
        g_type_check_instance_cast(
            (*d).view as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject as gpointer,
        b"signal::parent-set\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut GtkWidget, *mut GtkWidget, *mut widget_t) -> (),
            >,
            GCallback,
        >(
            Some(
                parent_set_cb
                    as unsafe extern "C" fn(
                        *mut GtkWidget,
                        *mut GtkWidget,
                        *mut widget_t,
                    ) -> (),
            ),
        ),
        w,
        NULL_0 as *mut std::ffi::c_void,
    );
    g_object_connect(
        g_type_check_instance_cast(
            (*d).inspector as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject as gpointer,
        b"signal::attach\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut WebKitWebInspector, *mut widget_t) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                inspector_attach_window_cb
                    as unsafe extern "C" fn(
                        *mut WebKitWebInspector,
                        *mut widget_t,
                    ) -> gboolean,
            ),
        ),
        w,
        b"signal::bring-to-front\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut WebKitWebInspector, *mut widget_t) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                inspector_show_window_cb
                    as unsafe extern "C" fn(
                        *mut WebKitWebInspector,
                        *mut widget_t,
                    ) -> gboolean,
            ),
        ),
        w,
        b"signal::closed\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut WebKitWebInspector, *mut widget_t) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                inspector_close_window_cb
                    as unsafe extern "C" fn(
                        *mut WebKitWebInspector,
                        *mut widget_t,
                    ) -> gboolean,
            ),
        ),
        w,
        b"signal::detach\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut WebKitWebInspector, *mut widget_t) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                inspector_detach_window_cb
                    as unsafe extern "C" fn(
                        *mut WebKitWebInspector,
                        *mut widget_t,
                    ) -> gboolean,
            ),
        ),
        w,
        b"signal::open-window\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut WebKitWebInspector, *mut widget_t) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                inspector_open_window_cb
                    as unsafe extern "C" fn(
                        *mut WebKitWebInspector,
                        *mut widget_t,
                    ) -> gboolean,
            ),
        ),
        w,
        NULL_0 as *mut std::ffi::c_void,
    );
    gtk_widget_show(
        g_type_check_instance_cast(
            (*d).view as *mut GTypeInstance,
            gtk_widget_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkWidget,
    );
    return w;
}
