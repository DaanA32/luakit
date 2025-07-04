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
    use super::gtypes_h::{gpointer, guint};
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
    #[c2rust::src_loc = "48:9"]
    pub const G_TYPE_FUNDAMENTAL_SHIFT: std::ffi::c_int = 2 as std::ffi::c_int;
    #[c2rust::src_loc = "72:9"]
    pub const G_TYPE_NONE: std::ffi::c_int = (1 as std::ffi::c_int)
        << G_TYPE_FUNDAMENTAL_SHIFT;
    use super::glibconfig_h::gsize;
    use super::gvalue_h::_GValue;
    use super::gtypes_h::gboolean;
    extern "C" {
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
    use super::gtype_h::GTypeInstance;
    use super::gtypes_h::{guint, gpointer};
    use super::gdataset_h::GData;
    extern "C" {
        #[c2rust::src_loc = "512:1"]
        pub fn g_object_ref(object: gpointer) -> gpointer;
        #[c2rust::src_loc = "514:1"]
        pub fn g_object_unref(object: gpointer);
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
    #[c2rust::src_loc = "32:9"]
    pub const JSC_TYPE_VALUE: GType = jsc_value_get_type();
    use super::gobject_h::GObject;
    use super::JSCContext_h::_JSCContext;
    use super::gtype_h::GType;
    use super::gclosure_h::GCallback;
    use super::gtypes_h::{gpointer, GDestroyNotify, guint, gboolean};
    extern "C" {
        #[c2rust::src_loc = "48:1"]
        pub type _JSCValuePrivate;
        #[c2rust::src_loc = "48:1"]
        pub fn jsc_value_get_type() -> GType;
        #[c2rust::src_loc = "77:1"]
        pub fn jsc_value_new_undefined(context: *mut JSCContext) -> *mut JSCValue;
        #[c2rust::src_loc = "212:1"]
        pub fn jsc_value_new_function(
            context: *mut JSCContext,
            name: *const std::ffi::c_char,
            callback: GCallback,
            user_data: gpointer,
            destroy_notify: GDestroyNotify,
            return_type: GType,
            n_params: guint,
            _: ...
        ) -> *mut JSCValue;
        #[c2rust::src_loc = "232:1"]
        pub fn jsc_value_new_function_variadic(
            context: *mut JSCContext,
            name: *const std::ffi::c_char,
            callback: GCallback,
            user_data: gpointer,
            destroy_notify: GDestroyNotify,
            return_type: GType,
        ) -> *mut JSCValue;
        #[c2rust::src_loc = "240:1"]
        pub fn jsc_value_is_function(value: *mut JSCValue) -> gboolean;
        #[c2rust::src_loc = "243:1"]
        pub fn jsc_value_function_call(
            value: *mut JSCValue,
            first_parameter_type: GType,
            _: ...
        ) -> *mut JSCValue;
        #[c2rust::src_loc = "302:1"]
        pub fn jsc_value_is_constructor(value: *mut JSCValue) -> gboolean;
        #[c2rust::src_loc = "305:1"]
        pub fn jsc_value_constructor_call(
            value: *mut JSCValue,
            first_parameter_type: GType,
            _: ...
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
    extern "C" {
        #[c2rust::src_loc = "53:1"]
        pub type _JSCContextPrivate;
        #[c2rust::src_loc = "105:1"]
        pub fn jsc_context_throw_exception(
            context: *mut JSCContext,
            exception: *mut JSCException,
        );
        #[c2rust::src_loc = "158:1"]
        pub fn jsc_context_set_value(
            context: *mut JSCContext,
            name: *const std::ffi::c_char,
            value: *mut JSCValue,
        );
        #[c2rust::src_loc = "163:1"]
        pub fn jsc_context_get_value(
            context: *mut JSCContext,
            name: *const std::ffi::c_char,
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
    use super::JSCValue_h::JSCContext;
    extern "C" {
        #[c2rust::src_loc = "48:1"]
        pub type _JSCExceptionPrivate;
        #[c2rust::src_loc = "56:1"]
        pub fn jsc_exception_new_printf(
            context: *mut JSCContext,
            format: *const std::ffi::c_char,
            _: ...
        ) -> *mut JSCException;
    }
}
#[c2rust::header_src = "/usr/include/luajit-2.1/lua.h:24"]
pub mod lua_h {
    #[c2rust::src_loc = "53:1"]
    pub type lua_CFunction = Option::<
        unsafe extern "C" fn(*mut lua_State) -> std::ffi::c_int,
    >;
    #[c2rust::src_loc = "104:1"]
    pub type lua_Integer = ptrdiff_t;
    #[c2rust::src_loc = "36:9"]
    pub const LUA_REGISTRYINDEX: std::ffi::c_int = -(10000 as std::ffi::c_int);
    #[c2rust::src_loc = "38:9"]
    pub const LUA_GLOBALSINDEX: std::ffi::c_int = -(10002 as std::ffi::c_int);
    #[c2rust::src_loc = "75:9"]
    pub const LUA_TNIL: std::ffi::c_int = 0 as std::ffi::c_int;
    #[c2rust::src_loc = "81:9"]
    pub const LUA_TFUNCTION: std::ffi::c_int = 6 as std::ffi::c_int;
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
        #[c2rust::src_loc = "126:1"]
        pub fn lua_replace(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "137:1"]
        pub fn lua_isstring(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;
        #[c2rust::src_loc = "140:1"]
        pub fn lua_type(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;
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
        #[c2rust::src_loc = "155:1"]
        pub fn lua_topointer(
            L: *mut lua_State,
            idx: std::ffi::c_int,
        ) -> *const std::ffi::c_void;
        #[c2rust::src_loc = "161:1"]
        pub fn lua_pushnil(L: *mut lua_State);
        #[c2rust::src_loc = "163:1"]
        pub fn lua_pushinteger(L: *mut lua_State, n: lua_Integer);
        #[c2rust::src_loc = "164:1"]
        pub fn lua_pushlstring(L: *mut lua_State, s: *const std::ffi::c_char, l: size_t);
        #[c2rust::src_loc = "165:1"]
        pub fn lua_pushstring(L: *mut lua_State, s: *const std::ffi::c_char);
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
        #[c2rust::src_loc = "179:1"]
        pub fn lua_getfield(
            L: *mut lua_State,
            idx: std::ffi::c_int,
            k: *const std::ffi::c_char,
        );
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
        #[c2rust::src_loc = "193:1"]
        pub fn lua_rawset(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "203:1"]
        pub fn lua_pcall(
            L: *mut lua_State,
            nargs: std::ffi::c_int,
            nresults: std::ffi::c_int,
            errfunc: std::ffi::c_int,
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "241:1"]
        pub fn lua_next(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/log.h:24"]
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
    extern "C" {
        #[c2rust::src_loc = "29:17"]
        pub static mut common: common_t;
    }
}
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/webkit/WebKitScriptWorld.h:25"]
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
        #[c2rust::src_loc = "54:1"]
        pub fn webkit_script_world_get_default() -> *mut WebKitScriptWorld;
    }
}
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/webkit/WebKitFrame.h:25"]
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
    use super::gtypes_h::gboolean;
    use super::JSCValue_h::JSCContext;
    use super::WebKitScriptWorld_h::WebKitScriptWorld;
    extern "C" {
        #[c2rust::src_loc = "46:16"]
        pub type _WebKitFramePrivate;
        #[c2rust::src_loc = "57:1"]
        pub fn webkit_frame_is_main_frame(frame: *mut WebKitFrame) -> gboolean;
        #[c2rust::src_loc = "70:1"]
        pub fn webkit_frame_get_js_context(frame: *mut WebKitFrame) -> *mut JSCContext;
        #[c2rust::src_loc = "73:1"]
        pub fn webkit_frame_get_js_context_for_script_world(
            frame: *mut WebKitFrame,
            world: *mut WebKitScriptWorld,
        ) -> *mut JSCContext;
    }
}
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/webkit/WebKitWebPage.h:25"]
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
    use super::glibconfig_h::guint64;
    use super::gtypes_h::gchar;
    use super::WebKitFrame_h::WebKitFrame;
    extern "C" {
        #[c2rust::src_loc = "47:16"]
        pub type _WebKitWebPagePrivate;
        #[c2rust::src_loc = "53:1"]
        pub fn webkit_web_page_get_type() -> GType;
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
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/webkit/WebKitWebExtension.h:25"]
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
#[c2rust::header_src = "/home/daana/git/luakit/common/ipc.h:25"]
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
#[c2rust::header_src = "/home/daana/git/luakit/extension/extension.h:25"]
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
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gmessages.h:19"]
pub mod gmessages_h {
    #[c2rust::src_loc = "320:9"]
    pub const G_LOG_DOMAIN: std::ffi::c_int = 0 as std::ffi::c_int;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gslice.h:19"]
pub mod gslice_h {
    use super::glibconfig_h::gsize;
    use super::gtypes_h::gpointer;
    extern "C" {
        #[c2rust::src_loc = "34:1"]
        pub fn g_slice_alloc(block_size: gsize) -> gpointer;
        #[c2rust::src_loc = "41:1"]
        pub fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtestutils.h:19"]
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
#[c2rust::header_src = "/usr/include/luajit-2.1/lauxlib.h:24"]
pub mod lauxlib_h {
    #[c2rust::src_loc = "60:9"]
    pub const LUA_REFNIL: std::ffi::c_int = -(1 as std::ffi::c_int);
    use super::lua_h::lua_State;
    extern "C" {
        #[c2rust::src_loc = "32:1"]
        pub fn luaL_typerror(
            L: *mut lua_State,
            narg: std::ffi::c_int,
            tname: *const std::ffi::c_char,
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "53:1"]
        pub fn luaL_error(
            L: *mut lua_State,
            fmt: *const std::ffi::c_char,
            _: ...
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "62:1"]
        pub fn luaL_ref(L: *mut lua_State, t: std::ffi::c_int) -> std::ffi::c_int;
        #[c2rust::src_loc = "63:1"]
        pub fn luaL_unref(L: *mut lua_State, t: std::ffi::c_int, ref_0: std::ffi::c_int);
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/luautil.h:24"]
pub mod luautil_h {
    use super::lua_h::lua_State;
    use super::gtypes_h::gint;
    extern "C" {
        #[c2rust::src_loc = "26:1"]
        pub fn luaH_dofunction_on_error(L: *mut lua_State) -> gint;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/lualib.h:24"]
pub mod lualib_h {
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
#[c2rust::header_src = "/home/daana/git/luakit/common/luaobject.h:24"]
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
        lua_rawget(L, LUA_REGISTRYINDEX);
    }
    #[inline]
    #[c2rust::src_loc = "99:1"]
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
        lua_State, lua_pushlstring, lua_rawget, LUA_REGISTRYINDEX, lua_settop,
        lua_pushlightuserdata, lua_remove,
    };
    use super::gtypes_h::{gint, gpointer};
    extern "C" {
        #[c2rust::src_loc = "40:1"]
        pub fn luaH_object_incref(L: *mut lua_State, tud: gint, oud: gint) -> gpointer;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/luah.h:24"]
pub mod luah_h {
    #[inline]
    #[c2rust::src_loc = "84:1"]
    pub unsafe extern "C" fn luaH_register(
        mut L: *mut lua_State,
        mut idx: gint,
        mut ref_0: *mut gint,
    ) -> gint {
        lua_pushvalue(L, idx);
        if *ref_0 != LUA_REFNIL {
            luaL_unref(L, LUA_REGISTRYINDEX, *ref_0);
        }
        *ref_0 = luaL_ref(L, LUA_REGISTRYINDEX);
        return 0 as std::ffi::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "111:1"]
    pub unsafe extern "C" fn luaH_registerfct(
        mut L: *mut lua_State,
        mut idx: gint,
        mut fct: *mut gint,
    ) -> gint {
        if !(lua_type(L, idx) == LUA_TFUNCTION) {
            luaL_typerror(L, idx, b"function\0" as *const u8 as *const std::ffi::c_char);
        }
        return luaH_register(L, idx, fct);
    }
    #[inline]
    #[c2rust::src_loc = "124:1"]
    pub unsafe extern "C" fn luaH_dofunction_from_registry(
        mut L: *mut lua_State,
        mut ref_0: gint,
        mut nargs: gint,
        mut nret: gint,
    ) -> gboolean {
        lua_rawgeti(L, LUA_REGISTRYINDEX, ref_0);
        return luaH_dofunction(L, nargs, nret);
    }
    use super::lua_h::{
        lua_State, lua_pushvalue, LUA_REGISTRYINDEX, lua_type, LUA_TFUNCTION, lua_rawgeti,
    };
    use super::gtypes_h::{gint, gboolean};
    use super::lauxlib_h::{LUA_REFNIL, luaL_unref, luaL_ref, luaL_typerror};
    use super::lualib_h::luaH_dofunction;
}
#[c2rust::header_src = "/home/daana/git/luakit/extension/clib/page.h:27"]
pub mod page_h {
    use super::lua_h::lua_State;
    use super::WebKitWebPage_h::WebKitWebPage;
    use super::gtypes_h::gint;
    extern "C" {
        #[c2rust::src_loc = "38:1"]
        pub fn luaH_page_from_web_page(
            L: *mut lua_State,
            web_page: *mut WebKitWebPage,
        ) -> gint;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/luajs.h:31"]
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
pub use self::glibconfig_h::{guint32, gint64, guint64, gsize};
pub use self::gtypes_h::{
    gchar, glong, gint, gboolean, gulong, guint, gfloat, gdouble, gpointer,
    GDestroyNotify,
};
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
pub use self::gtype_h::{
    GType, GValue, _GTypeClass, GTypeClass, _GTypeInstance, GTypeInstance,
    G_TYPE_FUNDAMENTAL_SHIFT, G_TYPE_NONE, g_type_check_instance_is_a,
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
pub use self::gobject_h::{_GObject, GObject, g_object_ref, g_object_unref};
pub use self::JSCValue_h::{
    _JSCValue, JSCValuePrivate, JSCValue, JSCContext, JSC_TYPE_VALUE, _JSCValuePrivate,
    jsc_value_get_type, jsc_value_new_undefined, jsc_value_new_function,
    jsc_value_new_function_variadic, jsc_value_is_function, jsc_value_function_call,
    jsc_value_is_constructor, jsc_value_constructor_call,
};
pub use self::JSCContext_h::{
    _JSCContext, JSCContextPrivate, _JSCContextPrivate, jsc_context_throw_exception,
    jsc_context_set_value, jsc_context_get_value,
};
pub use self::JSCException_h::{
    _JSCException, JSCExceptionPrivate, JSCException, _JSCExceptionPrivate,
    jsc_exception_new_printf,
};
pub use self::lua_h::{
    lua_CFunction, lua_Integer, LUA_REGISTRYINDEX, LUA_GLOBALSINDEX, LUA_TNIL,
    LUA_TFUNCTION, lua_State, lua_gettop, lua_settop, lua_pushvalue, lua_remove,
    lua_insert, lua_replace, lua_isstring, lua_type, lua_tointeger, lua_toboolean,
    lua_tolstring, lua_topointer, lua_pushnil, lua_pushinteger, lua_pushlstring,
    lua_pushstring, lua_pushcclosure, lua_pushboolean, lua_pushlightuserdata,
    lua_getfield, lua_rawget, lua_rawgeti, lua_createtable, lua_rawset, lua_pcall,
    lua_next,
};
pub use self::log_h::{
    log_level_t, LOG_LEVEL_debug, LOG_LEVEL_verbose, LOG_LEVEL_info, LOG_LEVEL_warn,
    LOG_LEVEL_error, LOG_LEVEL_fatal, _log,
};
pub use self::common_h::{_common_t, common_t, common};
pub use self::WebKitScriptWorld_h::{
    _WebKitScriptWorld, WebKitScriptWorldPrivate, WebKitScriptWorld,
    _WebKitScriptWorldPrivate, webkit_script_world_get_default,
};
pub use self::WebKitFrame_h::{
    _WebKitFrame, WebKitFramePrivate, WebKitFrame, _WebKitFramePrivate,
    webkit_frame_is_main_frame, webkit_frame_get_js_context,
    webkit_frame_get_js_context_for_script_world,
};
pub use self::WebKitWebPage_h::{
    _WebKitWebPage, WebKitWebPagePrivate, WebKitWebPage, _WebKitWebPagePrivate,
    webkit_web_page_get_type, webkit_web_page_get_id, webkit_web_page_get_uri,
    webkit_web_page_get_main_frame,
};
pub use self::WebKitWebExtension_h::{
    _WebKitWebExtension, WebKitWebExtensionPrivate, WebKitWebExtension,
    _WebKitWebExtensionPrivate, webkit_web_extension_get_page,
};
pub use self::ipc_h::{
    ipc_type_t, IPC_TYPE_crash, IPC_TYPE_page_created, IPC_TYPE_log, IPC_TYPE_eval_js,
    IPC_TYPE_extension_init, IPC_TYPE_scroll, IPC_TYPE_lua_ipc,
    IPC_TYPE_lua_require_module, _ipc_header_t, ipc_header_t, _ipc_recv_state_t,
    ipc_recv_state_t, ipc_endpoint_status_t, IPC_ENDPOINT_FREED, IPC_ENDPOINT_CONNECTED,
    IPC_ENDPOINT_DISCONNECTED, _ipc_endpoint_t, ipc_endpoint_t,
};
pub use self::extension_h::{_extension_t, extension_t, extension};
pub use self::gmessages_h::G_LOG_DOMAIN;
use self::gslice_h::{g_slice_alloc, g_slice_free1};
use self::gtestutils_h::g_assertion_message_expr;
pub use self::lauxlib_h::{LUA_REFNIL, luaL_typerror, luaL_error, luaL_ref, luaL_unref};
use self::luautil_h::luaH_dofunction_on_error;
pub use self::lualib_h::luaH_dofunction;
pub use self::luaobject_h::{
    luaH_object_registry_push, luaH_object_ref, luaH_object_push, luaH_object_incref,
};
pub use self::luah_h::{luaH_register, luaH_registerfct, luaH_dofunction_from_registry};
use self::page_h::luaH_page_from_web_page;
use self::luajs_h::{luajs_pushvalue, luajs_tovalue};
pub use self::gmacros_h::{FALSE, TRUE};
pub use self::__stddef_null_h::NULL;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "46:8"]
pub struct cb_data {
    pub ctx: *mut luajs_func_ctx_t,
    pub context: *mut JSCContext,
}
#[c2rust::src_loc = "33:1"]
pub type luajs_func_ctx_t = _luajs_func_ctx_t;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "33:16"]
pub struct _luajs_func_ctx_t {
    pub ref_0: gpointer,
    pub page_id: guint64,
}
#[c2rust::src_loc = "40:1"]
pub type js_promise_t = _js_promise_t;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "40:16"]
pub struct _js_promise_t {
    pub promise: *mut JSCValue,
    pub resolve: *mut JSCValue,
    pub reject: *mut JSCValue,
}
#[c2rust::src_loc = "38:13"]
static mut lua_string_find_ref: gint = LUA_REFNIL;
#[c2rust::src_loc = "51:1"]
unsafe extern "C" fn promise_executor_cb(
    mut resolve: *mut JSCValue,
    mut reject: *mut JSCValue,
    mut promise: *mut js_promise_t,
) {
    if jsc_value_is_function(resolve) != 0 {} else {
        g_assertion_message_expr(
            G_LOG_DOMAIN as *const std::ffi::c_char,
            b"extension/luajs.c\0" as *const u8 as *const std::ffi::c_char,
            54 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 20],
                &[std::ffi::c_char; 20],
            >(b"promise_executor_cb\0"))
                .as_ptr(),
            b"jsc_value_is_function(resolve)\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    if jsc_value_is_function(reject) != 0 {} else {
        g_assertion_message_expr(
            G_LOG_DOMAIN as *const std::ffi::c_char,
            b"extension/luajs.c\0" as *const u8 as *const std::ffi::c_char,
            55 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 20],
                &[std::ffi::c_char; 20],
            >(b"promise_executor_cb\0"))
                .as_ptr(),
            b"jsc_value_is_function(reject)\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    g_object_ref(resolve as gpointer);
    g_object_ref(reject as gpointer);
    (*promise).resolve = resolve;
    (*promise).reject = reject;
}
#[c2rust::src_loc = "63:1"]
unsafe extern "C" fn new_promise(
    mut context: *mut JSCContext,
    mut promise: *mut js_promise_t,
) {
    let mut promise_ctor = jsc_context_get_value(
        context,
        b"Promise\0" as *const u8 as *const std::ffi::c_char,
    );
    if jsc_value_is_constructor(promise_ctor) != 0 {} else {
        g_assertion_message_expr(
            G_LOG_DOMAIN as *const std::ffi::c_char,
            b"extension/luajs.c\0" as *const u8 as *const std::ffi::c_char,
            68 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 12],
                &[std::ffi::c_char; 12],
            >(b"new_promise\0"))
                .as_ptr(),
            b"jsc_value_is_constructor(promise_ctor)\0" as *const u8
                as *const std::ffi::c_char,
        );
    }
    let mut func = jsc_value_new_function(
        context,
        NULL as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut JSCValue,
                    *mut JSCValue,
                    *mut js_promise_t,
                ) -> (),
            >,
            GCallback,
        >(
            Some(
                promise_executor_cb
                    as unsafe extern "C" fn(
                        *mut JSCValue,
                        *mut JSCValue,
                        *mut js_promise_t,
                    ) -> (),
            ),
        ),
        promise as gpointer,
        ::core::mem::transmute::<libc::intptr_t, GDestroyNotify>(NULL as libc::intptr_t),
        G_TYPE_NONE as GType,
        2 as std::ffi::c_int as guint,
        JSC_TYPE_VALUE,
        JSC_TYPE_VALUE,
    );
    (*promise)
        .promise = jsc_value_constructor_call(
        promise_ctor,
        JSC_TYPE_VALUE,
        func,
        G_TYPE_NONE as GType,
    );
    g_object_unref(func as gpointer);
    g_object_unref(promise_ctor as gpointer);
}
#[c2rust::src_loc = "77:1"]
unsafe extern "C" fn luaJS_promise_resolve_reject(
    mut L: *mut lua_State,
) -> std::ffi::c_int {
    let mut page_id = lua_tointeger(L, LUA_GLOBALSINDEX - 1 as std::ffi::c_int)
        as guint64;
    let mut page = webkit_web_extension_get_page(extension.ext, page_id);
    if page.is_null()
        || ({
            let mut __inst = page as *mut GTypeInstance;
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
        return luaL_error(
            L,
            b"promise no longer valid (associated page closed)\0" as *const u8
                as *const std::ffi::c_char,
        );
    }
    let mut context = webkit_frame_get_js_context(webkit_web_page_get_main_frame(page));
    let mut promise = lua_topointer(L, LUA_GLOBALSINDEX - 2 as std::ffi::c_int)
        as *mut js_promise_t;
    let mut cb = if lua_toboolean(L, LUA_GLOBALSINDEX - 3 as std::ffi::c_int) != 0 {
        (*promise).resolve
    } else {
        (*promise).reject
    };
    let mut ret = luajs_tovalue(L, 1 as std::ffi::c_int, context);
    let mut undefined = jsc_value_function_call(
        cb,
        JSC_TYPE_VALUE,
        ret,
        G_TYPE_NONE as GType,
    );
    g_object_unref(undefined as gpointer);
    g_object_unref((*promise).reject as gpointer);
    g_object_unref((*promise).resolve as gpointer);
    g_slice_free1(
        ::core::mem::size_of::<js_promise_t>() as std::ffi::c_ulong,
        promise as gpointer,
    );
    g_object_unref(ret as gpointer);
    g_object_unref(context as gpointer);
    return 0 as std::ffi::c_int;
}
#[c2rust::src_loc = "104:1"]
unsafe extern "C" fn luaJS_registered_function_callback(
    mut args: *mut GPtrArray,
    mut user_data: *mut cb_data,
) -> *mut JSCValue {
    let mut L = common.L;
    let mut top = lua_gettop(L);
    let mut ctx = (*user_data).ctx;
    let mut context = (*user_data).context;
    let mut argc = (*args).len;
    let mut argv = (*args).pdata as *mut *mut JSCValue;
    let mut promise = g_slice_alloc(
        ::core::mem::size_of::<js_promise_t>() as std::ffi::c_ulong,
    ) as *mut js_promise_t;
    new_promise(context, promise);
    luaH_page_from_web_page(
        L,
        webkit_web_extension_get_page(extension.ext, (*ctx).page_id),
    );
    lua_pushinteger(L, (*ctx).page_id as lua_Integer);
    lua_pushlightuserdata(L, promise as *mut std::ffi::c_void);
    lua_pushboolean(L, TRUE);
    lua_pushcclosure(
        L,
        Some(
            luaJS_promise_resolve_reject
                as unsafe extern "C" fn(*mut lua_State) -> std::ffi::c_int,
        ),
        3 as std::ffi::c_int,
    );
    lua_pushinteger(L, (*ctx).page_id as lua_Integer);
    lua_pushlightuserdata(L, promise as *mut std::ffi::c_void);
    lua_pushboolean(L, FALSE);
    lua_pushcclosure(
        L,
        Some(
            luaJS_promise_resolve_reject
                as unsafe extern "C" fn(*mut lua_State) -> std::ffi::c_int,
        ),
        3 as std::ffi::c_int,
    );
    let mut i = 0 as std::ffi::c_int as guint;
    while i < argc {
        if luajs_pushvalue(L, *argv.offset(i as isize)) != 0 {
            i = i.wrapping_add(1);
            i;
        } else {
            jsc_context_throw_exception(
                context,
                jsc_exception_new_printf(
                    context,
                    b"bad argument #%d to Lua function\0" as *const u8
                        as *const std::ffi::c_char,
                    i,
                ),
            );
            lua_settop(L, top);
            return jsc_value_new_undefined(context);
        }
    }
    luaH_object_push(L, (*ctx).ref_0);
    luaH_dofunction(
        L,
        argc.wrapping_add(3 as std::ffi::c_int as guint) as gint,
        0 as std::ffi::c_int,
    );
    lua_settop(L, top);
    return (*promise).promise;
}
#[c2rust::src_loc = "151:1"]
unsafe extern "C" fn luaJS_registered_function_destroy(
    mut user_data: *mut std::ffi::c_void,
) {
    let mut cb_data = user_data as *mut cb_data;
    g_object_unref((*cb_data).context as gpointer);
    g_slice_free1(
        ::core::mem::size_of::<luajs_func_ctx_t>() as std::ffi::c_ulong,
        (*cb_data).ctx as gpointer,
    );
    g_slice_free1(
        ::core::mem::size_of::<cb_data>() as std::ffi::c_ulong,
        cb_data as gpointer,
    );
}
#[no_mangle]
#[c2rust::src_loc = "160:1"]
pub unsafe extern "C" fn luaJS_register_function(mut L: *mut lua_State) {
    if lua_isstring(L, -(3 as std::ffi::c_int)) != 0 {} else {
        g_assertion_message_expr(
            G_LOG_DOMAIN as *const std::ffi::c_char,
            b"extension/luajs.c\0" as *const u8 as *const std::ffi::c_char,
            163 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 24],
                &[std::ffi::c_char; 24],
            >(b"luaJS_register_function\0"))
                .as_ptr(),
            b"lua_isstring(L, -3)\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    if lua_isstring(L, -(2 as std::ffi::c_int)) != 0 {} else {
        g_assertion_message_expr(
            G_LOG_DOMAIN as *const std::ffi::c_char,
            b"extension/luajs.c\0" as *const u8 as *const std::ffi::c_char,
            164 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 24],
                &[std::ffi::c_char; 24],
            >(b"luaJS_register_function\0"))
                .as_ptr(),
            b"lua_isstring(L, -2)\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    if lua_type(L, -(1 as std::ffi::c_int)) == 6 as std::ffi::c_int {} else {
        g_assertion_message_expr(
            G_LOG_DOMAIN as *const std::ffi::c_char,
            b"extension/luajs.c\0" as *const u8 as *const std::ffi::c_char,
            165 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 24],
                &[std::ffi::c_char; 24],
            >(b"luaJS_register_function\0"))
                .as_ptr(),
            b"lua_isfunction(L, -1)\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    lua_pushlstring(
        L,
        b"luakit.luajs.registry\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 22]>() as std::ffi::c_ulong)
            .wrapping_div(
                ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
            )
            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
    );
    lua_rawget(L, LUA_REGISTRYINDEX);
    lua_pushvalue(L, -(4 as std::ffi::c_int));
    lua_rawget(L, -(2 as std::ffi::c_int));
    if lua_type(L, -(1 as std::ffi::c_int)) == LUA_TNIL {
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        lua_pushvalue(L, -(4 as std::ffi::c_int));
        lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
        lua_rawset(L, -(3 as std::ffi::c_int));
        lua_pushvalue(L, -(4 as std::ffi::c_int));
        lua_rawget(L, -(2 as std::ffi::c_int));
    }
    lua_replace(L, -(2 as std::ffi::c_int));
    lua_insert(L, -(3 as std::ffi::c_int));
    lua_rawset(L, -(3 as std::ffi::c_int));
    lua_settop(L, -(2 as std::ffi::c_int) - 1 as std::ffi::c_int);
}
#[c2rust::src_loc = "193:1"]
unsafe extern "C" fn register_func(
    mut world: *mut WebKitScriptWorld,
    mut web_page: *mut WebKitWebPage,
    mut frame: *mut WebKitFrame,
    mut name: *const gchar,
    mut ref_0: gpointer,
) {
    let mut context = webkit_frame_get_js_context_for_script_world(frame, world);
    let mut ctx = g_slice_alloc(
        ::core::mem::size_of::<luajs_func_ctx_t>() as std::ffi::c_ulong,
    ) as *mut luajs_func_ctx_t;
    (*ctx).page_id = webkit_web_page_get_id(web_page);
    (*ctx).ref_0 = ref_0;
    let mut user_data = g_slice_alloc(
        ::core::mem::size_of::<cb_data>() as std::ffi::c_ulong,
    ) as *mut cb_data;
    (*user_data).ctx = ctx;
    (*user_data).context = context;
    let mut fun = jsc_value_new_function_variadic(
        context,
        name,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut GPtrArray, *mut cb_data) -> *mut JSCValue,
            >,
            GCallback,
        >(
            Some(
                luaJS_registered_function_callback
                    as unsafe extern "C" fn(
                        *mut GPtrArray,
                        *mut cb_data,
                    ) -> *mut JSCValue,
            ),
        ),
        user_data as gpointer,
        Some(
            luaJS_registered_function_destroy
                as unsafe extern "C" fn(*mut std::ffi::c_void) -> (),
        ),
        JSC_TYPE_VALUE,
    );
    jsc_context_set_value(context, name, fun);
    g_object_unref(fun as gpointer);
}
#[c2rust::src_loc = "210:1"]
unsafe extern "C" fn window_object_cleared_cb(
    mut world: *mut WebKitScriptWorld,
    mut web_page: *mut WebKitWebPage,
    mut frame: *mut WebKitFrame,
    mut UNUSED_user_data: gpointer,
) {
    if webkit_frame_is_main_frame(frame) == 0 {
        return;
    }
    let mut L = common.L;
    let ref mut fresh0 = webkit_web_page_get_uri(web_page);
    let mut uri = if !(*fresh0).is_null() {
        *fresh0
    } else {
        b"about:blank\0" as *const u8 as *const std::ffi::c_char
    };
    lua_pushlstring(
        L,
        b"luakit.luajs.registry\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 22]>() as std::ffi::c_ulong)
            .wrapping_div(
                ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
            )
            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
    );
    lua_rawget(L, LUA_REGISTRYINDEX);
    lua_pushnil(L);
    while lua_next(L, -(2 as std::ffi::c_int)) != 0 as std::ffi::c_int {
        if lua_isstring(L, -(2 as std::ffi::c_int)) != 0 {} else {
            g_assertion_message_expr(
                G_LOG_DOMAIN as *const std::ffi::c_char,
                b"extension/luajs.c\0" as *const u8 as *const std::ffi::c_char,
                227 as std::ffi::c_int,
                (*::core::mem::transmute::<
                    &[u8; 25],
                    &[std::ffi::c_char; 25],
                >(b"window_object_cleared_cb\0"))
                    .as_ptr(),
                b"lua_isstring(L, -2)\0" as *const u8 as *const std::ffi::c_char,
            );
        }
        if lua_type(L, -(1 as std::ffi::c_int)) == 5 as std::ffi::c_int {} else {
            g_assertion_message_expr(
                G_LOG_DOMAIN as *const std::ffi::c_char,
                b"extension/luajs.c\0" as *const u8 as *const std::ffi::c_char,
                228 as std::ffi::c_int,
                (*::core::mem::transmute::<
                    &[u8; 25],
                    &[std::ffi::c_char; 25],
                >(b"window_object_cleared_cb\0"))
                    .as_ptr(),
                b"lua_istable(L, -1)\0" as *const u8 as *const std::ffi::c_char,
            );
        }
        lua_pushstring(L, uri);
        lua_pushvalue(L, -(3 as std::ffi::c_int));
        luaH_dofunction_from_registry(
            L,
            lua_string_find_ref,
            2 as std::ffi::c_int,
            1 as std::ffi::c_int,
        );
        if !(lua_type(L, -(1 as std::ffi::c_int)) == LUA_TNIL) {
            lua_pushnil(L);
            while lua_next(L, -(3 as std::ffi::c_int)) != 0 as std::ffi::c_int {
                if lua_isstring(L, -(2 as std::ffi::c_int)) != 0 {} else {
                    g_assertion_message_expr(
                        G_LOG_DOMAIN as *const std::ffi::c_char,
                        b"extension/luajs.c\0" as *const u8 as *const std::ffi::c_char,
                        240 as std::ffi::c_int,
                        (*::core::mem::transmute::<
                            &[u8; 25],
                            &[std::ffi::c_char; 25],
                        >(b"window_object_cleared_cb\0"))
                            .as_ptr(),
                        b"lua_isstring(L, -2)\0" as *const u8 as *const std::ffi::c_char,
                    );
                }
                if lua_type(L, -(1 as std::ffi::c_int)) == 6 as std::ffi::c_int {} else {
                    g_assertion_message_expr(
                        G_LOG_DOMAIN as *const std::ffi::c_char,
                        b"extension/luajs.c\0" as *const u8 as *const std::ffi::c_char,
                        241 as std::ffi::c_int,
                        (*::core::mem::transmute::<
                            &[u8; 25],
                            &[std::ffi::c_char; 25],
                        >(b"window_object_cleared_cb\0"))
                            .as_ptr(),
                        b"lua_isfunction(L, -1)\0" as *const u8
                            as *const std::ffi::c_char,
                    );
                }
                let mut ref_0 = luaH_object_ref(L, -(1 as std::ffi::c_int));
                register_func(
                    world,
                    web_page,
                    frame,
                    lua_tolstring(L, -(1 as std::ffi::c_int), NULL as *mut size_t),
                    ref_0,
                );
            }
        }
        lua_settop(L, -(2 as std::ffi::c_int) - 1 as std::ffi::c_int);
    }
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "255:1"]
pub unsafe extern "C" fn web_luajs_init() {
    g_signal_connect_data(
        webkit_script_world_get_default() as gpointer,
        b"window-object-cleared\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut WebKitScriptWorld,
                    *mut WebKitWebPage,
                    *mut WebKitFrame,
                    gpointer,
                ) -> (),
            >,
            GCallback,
        >(
            Some(
                window_object_cleared_cb
                    as unsafe extern "C" fn(
                        *mut WebKitScriptWorld,
                        *mut WebKitWebPage,
                        *mut WebKitFrame,
                        gpointer,
                    ) -> (),
            ),
        ),
        0 as *mut std::ffi::c_void,
        ::core::mem::transmute::<libc::intptr_t, GClosureNotify>(NULL as libc::intptr_t),
        G_CONNECT_DEFAULT,
    );
    let mut L = common.L;
    lua_pushlstring(
        L,
        b"luakit.luajs.registry\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 22]>() as std::ffi::c_ulong)
            .wrapping_div(
                ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
            )
            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
    );
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_rawset(L, LUA_REGISTRYINDEX);
    lua_getfield(
        L,
        LUA_GLOBALSINDEX,
        b"string\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_getfield(
        L,
        -(1 as std::ffi::c_int),
        b"find\0" as *const u8 as *const std::ffi::c_char,
    );
    luaH_registerfct(L, -(1 as std::ffi::c_int), &mut lua_string_find_ref);
    lua_settop(L, -(2 as std::ffi::c_int) - 1 as std::ffi::c_int);
}
