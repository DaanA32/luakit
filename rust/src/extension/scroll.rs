use ::libc;
use ::c2rust_bitfields;
#[c2rust::header_src = "/usr/lib/glib-2.0/include/glibconfig.h:20"]
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
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtypes.h:20"]
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
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/garray.h:20"]
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
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gquark.h:20"]
pub mod gquark_h {
    #[c2rust::src_loc = "38:1"]
    pub type GQuark = guint32;
    use super::glibconfig_h::guint32;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gerror.h:20"]
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
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gconvert.h:20"]
pub mod gconvert_h {
    #[c2rust::src_loc = "85:1"]
    pub type GIConv = *mut _GIConv;
    extern "C" {
        #[c2rust::src_loc = "85:16"]
        pub type _GIConv;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gdataset.h:20"]
pub mod gdataset_h {
    #[c2rust::src_loc = "38:1"]
    pub type GData = _GData;
    extern "C" {
        #[c2rust::src_loc = "38:16"]
        pub type _GData;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/glist.h:20"]
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
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gslist.h:20"]
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
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gmain.h:20"]
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
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gstring.h:20"]
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
#[c2rust::header_src = "/usr/include/glib-2.0/glib/giochannel.h:20"]
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
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gqueue.h:20"]
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
#[c2rust::header_src = "/usr/include/glib-2.0/gobject/gtype.h:20"]
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
    extern "C" {
        #[c2rust::src_loc = "2626:1"]
        pub fn g_type_check_instance_cast(
            instance: *mut GTypeInstance,
            iface_type: GType,
        ) -> *mut GTypeInstance;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/gobject/gvalue.h:20"]
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
#[c2rust::header_src = "/usr/include/glib-2.0/gobject/gclosure.h:20"]
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
#[c2rust::header_src = "/usr/include/glib-2.0/gobject/gsignal.h:20"]
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
#[c2rust::header_src = "/usr/include/glib-2.0/gobject/gobject.h:20"]
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
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/webkitdom/webkitdomdefines.h:20"]
pub mod webkitdomdefines_h {
    #[c2rust::src_loc = "289:1"]
    pub type WebKitDOMNode = _WebKitDOMNode;
    #[c2rust::src_loc = "301:1"]
    pub type WebKitDOMObject = _WebKitDOMObject;
    #[c2rust::src_loc = "79:1"]
    pub type WebKitDOMDOMWindow = _WebKitDOMDOMWindow;
    #[c2rust::src_loc = "82:1"]
    pub type WebKitDOMDocument = _WebKitDOMDocument;
    #[c2rust::src_loc = "91:1"]
    pub type WebKitDOMElement = _WebKitDOMElement;
    #[c2rust::src_loc = "94:1"]
    pub type WebKitDOMEvent = _WebKitDOMEvent;
    #[c2rust::src_loc = "97:1"]
    pub type WebKitDOMEventTarget = _WebKitDOMEventTarget;
    use super::WebKitDOMNode_h::_WebKitDOMNode;
    use super::WebKitDOMObject_h::_WebKitDOMObject;
    use super::WebKitDOMDOMWindow_h::_WebKitDOMDOMWindow;
    use super::WebKitDOMDocument_h::_WebKitDOMDocument;
    use super::WebKitDOMElement_h::_WebKitDOMElement;
    use super::WebKitDOMEvent_h::_WebKitDOMEvent;
    extern "C" {
        #[c2rust::src_loc = "97:16"]
        pub type _WebKitDOMEventTarget;
    }
}
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/webkitdom/WebKitDOMNode.h:22"]
pub mod WebKitDOMNode_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "41:8"]
    pub struct _WebKitDOMNode {
        pub parent_instance: WebKitDOMObject,
    }
    use super::webkitdomdefines_h::WebKitDOMObject;
}
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/webkitdom/WebKitDOMObject.h:20"]
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
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/webkitdom/WebKitDOMDOMWindow.h:20"]
pub mod WebKitDOMDOMWindow_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "40:8"]
    pub struct _WebKitDOMDOMWindow {
        pub parent_instance: WebKitDOMObject,
    }
    use super::webkitdomdefines_h::{WebKitDOMObject, WebKitDOMDOMWindow};
    use super::gtypes_h::{gdouble, glong};
    extern "C" {
        #[c2rust::src_loc = "203:1"]
        pub fn webkit_dom_dom_window_scroll_to(
            self_0: *mut WebKitDOMDOMWindow,
            x: gdouble,
            y: gdouble,
        );
        #[c2rust::src_loc = "357:1"]
        pub fn webkit_dom_dom_window_get_inner_height(
            self_0: *mut WebKitDOMDOMWindow,
        ) -> glong;
        #[c2rust::src_loc = "370:1"]
        pub fn webkit_dom_dom_window_get_inner_width(
            self_0: *mut WebKitDOMDOMWindow,
        ) -> glong;
        #[c2rust::src_loc = "435:1"]
        pub fn webkit_dom_dom_window_get_scroll_x(
            self_0: *mut WebKitDOMDOMWindow,
        ) -> glong;
        #[c2rust::src_loc = "448:1"]
        pub fn webkit_dom_dom_window_get_scroll_y(
            self_0: *mut WebKitDOMDOMWindow,
        ) -> glong;
    }
}
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/webkitdom/WebKitDOMDocument.h:22"]
pub mod WebKitDOMDocument_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "40:8"]
    pub struct _WebKitDOMDocument {
        pub parent_instance: WebKitDOMNode,
    }
    use super::webkitdomdefines_h::{
        WebKitDOMNode, WebKitDOMDocument, WebKitDOMElement, WebKitDOMDOMWindow,
    };
    extern "C" {
        #[c2rust::src_loc = "538:1"]
        pub fn webkit_dom_document_get_document_element(
            self_0: *mut WebKitDOMDocument,
        ) -> *mut WebKitDOMElement;
        #[c2rust::src_loc = "639:1"]
        pub fn webkit_dom_document_get_default_view(
            self_0: *mut WebKitDOMDocument,
        ) -> *mut WebKitDOMDOMWindow;
    }
}
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/webkitdom/WebKitDOMElement.h:22"]
pub mod WebKitDOMElement_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "40:8"]
    pub struct _WebKitDOMElement {
        pub parent_instance: WebKitDOMNode,
    }
    use super::webkitdomdefines_h::{WebKitDOMNode, WebKitDOMElement};
    use super::gtypes_h::glong;
    extern "C" {
        #[c2rust::src_loc = "617:1"]
        pub fn webkit_dom_element_get_scroll_width(
            self_0: *mut WebKitDOMElement,
        ) -> glong;
        #[c2rust::src_loc = "628:1"]
        pub fn webkit_dom_element_get_scroll_height(
            self_0: *mut WebKitDOMElement,
        ) -> glong;
    }
}
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/webkitdom/WebKitDOMEvent.h:22"]
pub mod WebKitDOMEvent_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "184:8"]
    pub struct _WebKitDOMEvent {
        pub parent_instance: WebKitDOMObject,
    }
    use super::webkitdomdefines_h::WebKitDOMObject;
}
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/webkit/WebKitScriptWorld.h:22"]
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
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/webkit/WebKitWebPage.h:22"]
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
    use super::webkitdomdefines_h::WebKitDOMDocument;
    use super::glibconfig_h::guint64;
    extern "C" {
        #[c2rust::src_loc = "47:16"]
        pub type _WebKitWebPagePrivate;
        #[c2rust::src_loc = "77:1"]
        pub fn webkit_web_page_get_dom_document(
            web_page: *mut WebKitWebPage,
        ) -> *mut WebKitDOMDocument;
        #[c2rust::src_loc = "80:1"]
        pub fn webkit_web_page_get_id(web_page: *mut WebKitWebPage) -> guint64;
    }
}
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/webkit/WebKitWebExtension.h:22"]
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
#[c2rust::header_src = "/home/daana/git/luakit/common/ipc.h:22"]
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
    use super::gtypes_h::{guint, gint, gpointer, gboolean, gchar};
    use super::glibconfig_h::{guint64, gsize};
    use super::garray_h::GPtrArray;
    use super::giochannel_h::GIOChannel;
    use super::gqueue_h::GQueue;
    extern "C" {
        #[c2rust::src_loc = "138:1"]
        pub fn ipc_send(
            ipc: *mut ipc_endpoint_t,
            header: *const ipc_header_t,
            data: *const std::ffi::c_void,
        );
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/extension/extension.h:22"]
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
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/webkitdom/WebKitDOMEventTarget.h:22"]
pub mod WebKitDOMEventTarget_h {
    use super::gtype_h::GType;
    use super::webkitdomdefines_h::WebKitDOMEventTarget;
    use super::gclosure_h::GCallback;
    use super::gtypes_h::{gboolean, gpointer};
    extern "C" {
        #[c2rust::src_loc = "57:1"]
        pub fn webkit_dom_event_target_get_type() -> GType;
        #[c2rust::src_loc = "85:1"]
        pub fn webkit_dom_event_target_add_event_listener(
            target: *mut WebKitDOMEventTarget,
            event_name: *const std::ffi::c_char,
            handler: GCallback,
            use_capture: gboolean,
            user_data: gpointer,
        ) -> gboolean;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gmacros.h:20"]
pub mod gmacros_h {
    #[c2rust::src_loc = "931:9"]
    pub const FALSE: std::ffi::c_int = 0 as std::ffi::c_int;
}
#[c2rust::header_src = "/usr/lib/clang/20/include/__stddef_null.h:20"]
pub mod __stddef_null_h {
    #[c2rust::src_loc = "26:9"]
    pub const NULL: std::ffi::c_int = 0 as std::ffi::c_int;
}
pub use self::glibconfig_h::{guint32, gint64, guint64, gsize};
pub use self::gtypes_h::{
    gchar, glong, gint, gboolean, gulong, guint, gfloat, gdouble, gpointer,
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
    g_type_check_instance_cast,
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
pub use self::gobject_h::{_GObject, GObject};
pub use self::webkitdomdefines_h::{
    WebKitDOMNode, WebKitDOMObject, WebKitDOMDOMWindow, WebKitDOMDocument,
    WebKitDOMElement, WebKitDOMEvent, WebKitDOMEventTarget, _WebKitDOMEventTarget,
};
pub use self::WebKitDOMNode_h::_WebKitDOMNode;
pub use self::WebKitDOMObject_h::_WebKitDOMObject;
pub use self::WebKitDOMDOMWindow_h::{
    _WebKitDOMDOMWindow, webkit_dom_dom_window_scroll_to,
    webkit_dom_dom_window_get_inner_height, webkit_dom_dom_window_get_inner_width,
    webkit_dom_dom_window_get_scroll_x, webkit_dom_dom_window_get_scroll_y,
};
pub use self::WebKitDOMDocument_h::{
    _WebKitDOMDocument, webkit_dom_document_get_document_element,
    webkit_dom_document_get_default_view,
};
pub use self::WebKitDOMElement_h::{
    _WebKitDOMElement, webkit_dom_element_get_scroll_width,
    webkit_dom_element_get_scroll_height,
};
pub use self::WebKitDOMEvent_h::_WebKitDOMEvent;
pub use self::WebKitScriptWorld_h::{
    _WebKitScriptWorld, WebKitScriptWorldPrivate, WebKitScriptWorld,
    _WebKitScriptWorldPrivate,
};
pub use self::WebKitWebPage_h::{
    _WebKitWebPage, WebKitWebPagePrivate, WebKitWebPage, _WebKitWebPagePrivate,
    webkit_web_page_get_dom_document, webkit_web_page_get_id,
};
pub use self::WebKitWebExtension_h::{
    _WebKitWebExtension, WebKitWebExtensionPrivate, WebKitWebExtension,
    _WebKitWebExtensionPrivate, webkit_web_extension_get_page,
};
pub use self::ipc_h::{
    ipc_type_t, IPC_TYPE_crash, IPC_TYPE_page_created, IPC_TYPE_log, IPC_TYPE_eval_js,
    IPC_TYPE_extension_init, IPC_TYPE_scroll, IPC_TYPE_lua_ipc,
    IPC_TYPE_lua_require_module, _ipc_header_t, ipc_header_t, ipc_scroll_subtype_t,
    IPC_SCROLL_TYPE_scroll, IPC_SCROLL_TYPE_winresize, IPC_SCROLL_TYPE_docresize,
    _ipc_scroll_t, ipc_scroll_t, _ipc_recv_state_t, ipc_recv_state_t,
    ipc_endpoint_status_t, IPC_ENDPOINT_FREED, IPC_ENDPOINT_CONNECTED,
    IPC_ENDPOINT_DISCONNECTED, _ipc_endpoint_t, ipc_endpoint_t, ipc_send,
};
pub use self::extension_h::{_extension_t, extension_t, extension};
use self::WebKitDOMEventTarget_h::{
    webkit_dom_event_target_get_type, webkit_dom_event_target_add_event_listener,
};
pub use self::gmacros_h::FALSE;
pub use self::__stddef_null_h::NULL;
#[c2rust::src_loc = "26:1"]
unsafe extern "C" fn send_scroll_msg(
    mut h: gint,
    mut v: gint,
    mut web_page: *mut WebKitWebPage,
    mut subtype: ipc_scroll_subtype_t,
) {
    let data = {
        let mut init = _ipc_scroll_t {
            h: h,
            v: v,
            page_id: webkit_web_page_get_id(web_page),
            subtype: subtype,
        };
        init
    };
    let mut header = {
        let mut init = _ipc_header_t {
            length: ::core::mem::size_of::<ipc_scroll_t>() as std::ffi::c_ulong as guint,
            type_0: IPC_TYPE_scroll,
        };
        init
    };
    ipc_send(
        extension.ipc,
        &mut header,
        &data as *const ipc_scroll_t as *const std::ffi::c_void,
    );
}
#[c2rust::src_loc = "41:1"]
unsafe extern "C" fn window_scroll_cb(
    mut window: *mut WebKitDOMDOMWindow,
    mut UNUSED_event: *mut WebKitDOMEvent,
    mut web_page: *mut WebKitWebPage,
) {
    let mut h = webkit_dom_dom_window_get_scroll_x(window) as gint;
    let mut v = webkit_dom_dom_window_get_scroll_y(window) as gint;
    send_scroll_msg(h, v, web_page, IPC_SCROLL_TYPE_scroll);
}
#[c2rust::src_loc = "49:1"]
unsafe extern "C" fn window_resize_cb(
    mut window: *mut WebKitDOMDOMWindow,
    mut UNUSED_event: *mut WebKitDOMEvent,
    mut web_page: *mut WebKitWebPage,
) {
    let mut h = webkit_dom_dom_window_get_inner_width(window) as gint;
    let mut v = webkit_dom_dom_window_get_inner_height(window) as gint;
    send_scroll_msg(h, v, web_page, IPC_SCROLL_TYPE_winresize);
}
#[c2rust::src_loc = "57:13"]
static mut scroll_width_prev: gint = -(1 as std::ffi::c_int);
#[c2rust::src_loc = "57:37"]
static mut scroll_height_prev: gint = -(1 as std::ffi::c_int);
#[c2rust::src_loc = "59:1"]
unsafe extern "C" fn document_resize_cb(
    mut html: *mut WebKitDOMElement,
    mut UNUSED_event: *mut WebKitDOMEvent,
    mut web_page: *mut WebKitWebPage,
) {
    let mut h = webkit_dom_element_get_scroll_width(html) as gint;
    let mut v = webkit_dom_element_get_scroll_height(html) as gint;
    if h == scroll_width_prev && v == scroll_height_prev {
        return;
    }
    scroll_width_prev = h;
    scroll_height_prev = v;
    send_scroll_msg(h, v, web_page, IPC_SCROLL_TYPE_docresize);
}
#[c2rust::src_loc = "75:1"]
unsafe extern "C" fn web_page_document_loaded_cb(
    mut web_page: *mut WebKitWebPage,
    mut UNUSED_user_data: gpointer,
) {
    let mut document = webkit_web_page_get_dom_document(web_page);
    let mut html = webkit_dom_document_get_document_element(document);
    let mut window = webkit_dom_document_get_default_view(document);
    webkit_dom_event_target_add_event_listener(
        g_type_check_instance_cast(
            window as *mut GTypeInstance,
            webkit_dom_event_target_get_type(),
        ) as *mut std::ffi::c_void as *mut WebKitDOMEventTarget,
        b"scroll\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut WebKitDOMDOMWindow,
                    *mut WebKitDOMEvent,
                    *mut WebKitWebPage,
                ) -> (),
            >,
            GCallback,
        >(
            Some(
                window_scroll_cb
                    as unsafe extern "C" fn(
                        *mut WebKitDOMDOMWindow,
                        *mut WebKitDOMEvent,
                        *mut WebKitWebPage,
                    ) -> (),
            ),
        ),
        FALSE,
        web_page as gpointer,
    );
    webkit_dom_event_target_add_event_listener(
        g_type_check_instance_cast(
            window as *mut GTypeInstance,
            webkit_dom_event_target_get_type(),
        ) as *mut std::ffi::c_void as *mut WebKitDOMEventTarget,
        b"resize\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut WebKitDOMDOMWindow,
                    *mut WebKitDOMEvent,
                    *mut WebKitWebPage,
                ) -> (),
            >,
            GCallback,
        >(
            Some(
                window_resize_cb
                    as unsafe extern "C" fn(
                        *mut WebKitDOMDOMWindow,
                        *mut WebKitDOMEvent,
                        *mut WebKitWebPage,
                    ) -> (),
            ),
        ),
        FALSE,
        web_page as gpointer,
    );
    webkit_dom_event_target_add_event_listener(
        g_type_check_instance_cast(
            html as *mut GTypeInstance,
            webkit_dom_event_target_get_type(),
        ) as *mut std::ffi::c_void as *mut WebKitDOMEventTarget,
        b"DOMSubtreeModified\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut WebKitDOMElement,
                    *mut WebKitDOMEvent,
                    *mut WebKitWebPage,
                ) -> (),
            >,
            GCallback,
        >(
            Some(
                document_resize_cb
                    as unsafe extern "C" fn(
                        *mut WebKitDOMElement,
                        *mut WebKitDOMEvent,
                        *mut WebKitWebPage,
                    ) -> (),
            ),
        ),
        FALSE,
        web_page as gpointer,
    );
    window_scroll_cb(window, NULL as *mut WebKitDOMEvent, web_page);
    window_resize_cb(window, NULL as *mut WebKitDOMEvent, web_page);
    document_resize_cb(html, NULL as *mut WebKitDOMEvent, web_page);
}
#[c2rust::src_loc = "98:1"]
unsafe extern "C" fn web_page_created_cb(
    mut UNUSED_ext: *mut WebKitWebExtension,
    mut web_page: *mut WebKitWebPage,
    mut UNUSED_user_data: gpointer,
) {
    g_signal_connect_data(
        web_page as gpointer,
        b"document-loaded\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut WebKitWebPage, gpointer) -> ()>,
            GCallback,
        >(
            Some(
                web_page_document_loaded_cb
                    as unsafe extern "C" fn(*mut WebKitWebPage, gpointer) -> (),
            ),
        ),
        0 as *mut std::ffi::c_void,
        ::core::mem::transmute::<libc::intptr_t, GClosureNotify>(NULL as libc::intptr_t),
        G_CONNECT_DEFAULT,
    );
}
#[no_mangle]
#[c2rust::src_loc = "104:1"]
pub unsafe extern "C" fn web_scroll_to(
    mut page_id: guint64,
    mut scroll_x: gint,
    mut scroll_y: gint,
) {
    let mut page = webkit_web_extension_get_page(extension.ext, page_id);
    let mut document = webkit_web_page_get_dom_document(page);
    let mut window = webkit_dom_document_get_default_view(document);
    webkit_dom_dom_window_scroll_to(window, scroll_x as gdouble, scroll_y as gdouble);
    window_scroll_cb(window, NULL as *mut WebKitDOMEvent, page);
}
#[no_mangle]
#[c2rust::src_loc = "116:1"]
pub unsafe extern "C" fn web_scroll_init() {
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
        ::core::mem::transmute::<libc::intptr_t, GClosureNotify>(NULL as libc::intptr_t),
        G_CONNECT_DEFAULT,
    );
}
