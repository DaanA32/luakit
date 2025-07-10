use ::libc;
use ::c2rust_bitfields;

pub mod __stddef_ptrdiff_t_h {

    pub type ptrdiff_t = std::ffi::c_long;
}

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

    pub type glong = std::ffi::c_long;

    pub type gint = std::ffi::c_int;

    pub type gboolean = gint;

    pub type gushort = std::ffi::c_ushort;

    pub type gulong = std::ffi::c_ulong;

    pub type guint = std::ffi::c_uint;

    pub type gfloat = std::ffi::c_float;

    pub type gdouble = std::ffi::c_double;

    pub type gpointer = *mut std::ffi::c_void;

    pub type gconstpointer = *const std::ffi::c_void;

    pub type GCompareDataFunc = Option::<
        unsafe extern "C" fn(gconstpointer, gconstpointer, gpointer) -> gint,
    >;

    pub type GDestroyNotify = Option::<unsafe extern "C" fn(gpointer) -> ()>;
}

pub mod garray_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GPtrArray {
        pub pdata: *mut gpointer,
        pub len: guint,
    }

    pub type GPtrArray = _GPtrArray;
    use super::gtypes_h::{gpointer, guint, GDestroyNotify, gboolean};
    extern "C" {

        pub fn g_ptr_array_new() -> *mut GPtrArray;

        pub fn g_ptr_array_new_full(
            reserved_size: guint,
            element_free_func: GDestroyNotify,
        ) -> *mut GPtrArray;

        pub fn g_ptr_array_free(
            array: *mut GPtrArray,
            free_segment: gboolean,
        ) -> *mut gpointer;

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

pub mod ghash_h {

    pub type GHashTable = _GHashTable;
    extern "C" {

        pub type _GHashTable;
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

    pub type GTraverseFunc = Option::<
        unsafe extern "C" fn(gpointer, gpointer, gpointer) -> gboolean,
    >;
    use super::gtypes_h::{
        gboolean, gpointer, GCompareDataFunc, GDestroyNotify, gconstpointer,
    };
    extern "C" {

        pub type _GTree;

        pub fn g_tree_new_full(
            key_compare_func: GCompareDataFunc,
            key_compare_data: gpointer,
            key_destroy_func: GDestroyNotify,
            value_destroy_func: GDestroyNotify,
        ) -> *mut GTree;

        pub fn g_tree_destroy(tree: *mut GTree);

        pub fn g_tree_insert(tree: *mut GTree, key: gpointer, value: gpointer);

        pub fn g_tree_remove(tree: *mut GTree, key: gconstpointer) -> gboolean;

        pub fn g_tree_lookup(tree: *mut GTree, key: gconstpointer) -> gpointer;

        pub fn g_tree_foreach(
            tree: *mut GTree,
            func: GTraverseFunc,
            user_data: gpointer,
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

    pub const G_TYPE_FUNDAMENTAL_SHIFT: std::ffi::c_int = 2 as std::ffi::c_int;

    pub const G_TYPE_NONE: std::ffi::c_int = (1 as std::ffi::c_int)
        << G_TYPE_FUNDAMENTAL_SHIFT;

    pub const G_TYPE_STRING: std::ffi::c_int = (16 as std::ffi::c_int)
        << G_TYPE_FUNDAMENTAL_SHIFT;
    use super::glibconfig_h::gsize;
    use super::gtypes_h::gboolean;
    extern "C" {

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

pub mod gclosure_h {

    pub type GCallback = Option::<unsafe extern "C" fn() -> ()>;
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

    pub type GWeakNotify = Option::<unsafe extern "C" fn(gpointer, *mut GObject) -> ()>;
    use super::gtype_h::GTypeInstance;
    use super::gtypes_h::{guint, gpointer, gchar};
    use super::gdataset_h::GData;
    extern "C" {

        pub fn g_object_get(object: gpointer, first_property_name: *const gchar, _: ...);

        pub fn g_object_unref(object: gpointer);

        pub fn g_object_weak_ref(
            object: *mut GObject,
            notify: GWeakNotify,
            data: gpointer,
        );
    }
}

pub mod JSCValue_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _JSCValue {
        pub parent: GObject,
        pub priv_0: *mut JSCValuePrivate,
    }

    pub type JSCValuePrivate = _JSCValuePrivate;

    pub type JSCValue = _JSCValue;

    pub type JSCContext = _JSCContext;
    use super::gobject_h::GObject;
    use super::JSCContext_h::_JSCContext;
    use super::gtype_h::GType;
    extern "C" {

        pub type _JSCValuePrivate;

        pub fn jsc_value_object_get_property(
            value: *mut JSCValue,
            name: *const std::ffi::c_char,
        ) -> *mut JSCValue;

        pub fn jsc_value_object_invoke_method(
            value: *mut JSCValue,
            name: *const std::ffi::c_char,
            first_parameter_type: GType,
            _: ...
        ) -> *mut JSCValue;
    }
}

pub mod JSCContext_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _JSCContext {
        pub parent: GObject,
        pub priv_0: *mut JSCContextPrivate,
    }

    pub type JSCContextPrivate = _JSCContextPrivate;
    use super::gobject_h::GObject;
    use super::JSCValue_h::{JSCContext, JSCValue};
    extern "C" {

        pub type _JSCContextPrivate;

        pub fn jsc_context_get_global_object(context: *mut JSCContext) -> *mut JSCValue;
    }
}

pub mod webkitdomdefines_h {

    pub type WebKitDOMNode = _WebKitDOMNode;

    pub type WebKitDOMObject = _WebKitDOMObject;

    pub type WebKitDOMCSSStyleDeclaration = _WebKitDOMCSSStyleDeclaration;

    pub type WebKitDOMStyleSheet = _WebKitDOMStyleSheet;

    pub type WebKitDOMClientRect = _WebKitDOMClientRect;

    pub type WebKitDOMClientRectList = _WebKitDOMClientRectList;

    pub type WebKitDOMDOMWindow = _WebKitDOMDOMWindow;

    pub type WebKitDOMDocument = _WebKitDOMDocument;

    pub type WebKitDOMElement = _WebKitDOMElement;

    pub type WebKitDOMEvent = _WebKitDOMEvent;

    pub type WebKitDOMEventTarget = _WebKitDOMEventTarget;

    pub type WebKitDOMHTMLElement = _WebKitDOMHTMLElement;

    pub type WebKitDOMHTMLAnchorElement = _WebKitDOMHTMLAnchorElement;

    pub type WebKitDOMHTMLAreaElement = _WebKitDOMHTMLAreaElement;

    pub type WebKitDOMHTMLButtonElement = _WebKitDOMHTMLButtonElement;

    pub type WebKitDOMHTMLEmbedElement = _WebKitDOMHTMLEmbedElement;

    pub type WebKitDOMHTMLFormElement = _WebKitDOMHTMLFormElement;

    pub type WebKitDOMHTMLFrameElement = _WebKitDOMHTMLFrameElement;

    pub type WebKitDOMHTMLIFrameElement = _WebKitDOMHTMLIFrameElement;

    pub type WebKitDOMHTMLImageElement = _WebKitDOMHTMLImageElement;

    pub type WebKitDOMHTMLInputElement = _WebKitDOMHTMLInputElement;

    pub type WebKitDOMHTMLLIElement = _WebKitDOMHTMLLIElement;

    pub type WebKitDOMHTMLLinkElement = _WebKitDOMHTMLLinkElement;

    pub type WebKitDOMHTMLOptionElement = _WebKitDOMHTMLOptionElement;

    pub type WebKitDOMHTMLParamElement = _WebKitDOMHTMLParamElement;

    pub type WebKitDOMHTMLScriptElement = _WebKitDOMHTMLScriptElement;

    pub type WebKitDOMHTMLSelectElement = _WebKitDOMHTMLSelectElement;

    pub type WebKitDOMHTMLTextAreaElement = _WebKitDOMHTMLTextAreaElement;

    pub type WebKitDOMUIEvent = _WebKitDOMUIEvent;

    pub type WebKitDOMKeyboardEvent = _WebKitDOMKeyboardEvent;

    pub type WebKitDOMMouseEvent = _WebKitDOMMouseEvent;

    pub type WebKitDOMNodeList = _WebKitDOMNodeList;
    use super::WebKitDOMNode_h::_WebKitDOMNode;
    use super::WebKitDOMObject_h::_WebKitDOMObject;
    use super::WebKitDOMCSSStyleDeclaration_h::_WebKitDOMCSSStyleDeclaration;
    use super::WebKitDOMStyleSheet_h::_WebKitDOMStyleSheet;
    use super::WebKitDOMClientRect_h::_WebKitDOMClientRect;
    use super::WebKitDOMClientRectList_h::_WebKitDOMClientRectList;
    use super::WebKitDOMDOMWindow_h::_WebKitDOMDOMWindow;
    use super::WebKitDOMDocument_h::_WebKitDOMDocument;
    use super::WebKitDOMElement_h::_WebKitDOMElement;
    use super::WebKitDOMEvent_h::_WebKitDOMEvent;
    use super::WebKitDOMHTMLElement_h::_WebKitDOMHTMLElement;
    use super::WebKitDOMHTMLAnchorElement_h::_WebKitDOMHTMLAnchorElement;
    use super::WebKitDOMHTMLAreaElement_h::_WebKitDOMHTMLAreaElement;
    use super::WebKitDOMHTMLButtonElement_h::_WebKitDOMHTMLButtonElement;
    use super::WebKitDOMHTMLEmbedElement_h::_WebKitDOMHTMLEmbedElement;
    use super::WebKitDOMHTMLFormElement_h::_WebKitDOMHTMLFormElement;
    use super::WebKitDOMHTMLFrameElement_h::_WebKitDOMHTMLFrameElement;
    use super::WebKitDOMHTMLIFrameElement_h::_WebKitDOMHTMLIFrameElement;
    use super::WebKitDOMHTMLImageElement_h::_WebKitDOMHTMLImageElement;
    use super::WebKitDOMHTMLInputElement_h::_WebKitDOMHTMLInputElement;
    use super::WebKitDOMHTMLLIElement_h::_WebKitDOMHTMLLIElement;
    use super::WebKitDOMHTMLLinkElement_h::_WebKitDOMHTMLLinkElement;
    use super::WebKitDOMHTMLOptionElement_h::_WebKitDOMHTMLOptionElement;
    use super::WebKitDOMHTMLParamElement_h::_WebKitDOMHTMLParamElement;
    use super::WebKitDOMHTMLScriptElement_h::_WebKitDOMHTMLScriptElement;
    use super::WebKitDOMHTMLSelectElement_h::_WebKitDOMHTMLSelectElement;
    use super::WebKitDOMHTMLTextAreaElement_h::_WebKitDOMHTMLTextAreaElement;
    use super::WebKitDOMUIEvent_h::_WebKitDOMUIEvent;
    use super::WebKitDOMKeyboardEvent_h::_WebKitDOMKeyboardEvent;
    use super::WebKitDOMMouseEvent_h::_WebKitDOMMouseEvent;
    use super::WebKitDOMNodeList_h::_WebKitDOMNodeList;
    extern "C" {

        pub type _WebKitDOMEventTarget;
    }
}

pub mod WebKitDOMNode_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _WebKitDOMNode {
        pub parent_instance: WebKitDOMObject,
    }
    use super::webkitdomdefines_h::{WebKitDOMObject, WebKitDOMNode, WebKitDOMDocument};
    use super::gtype_h::GType;
    use super::gerror_h::GError;
    use super::gtypes_h::gchar;
    extern "C" {

        pub fn webkit_dom_node_get_type() -> GType;

        pub fn webkit_dom_node_append_child(
            self_0: *mut WebKitDOMNode,
            newChild: *mut WebKitDOMNode,
            error: *mut *mut GError,
        ) -> *mut WebKitDOMNode;

        pub fn webkit_dom_node_get_parent_node(
            self_0: *mut WebKitDOMNode,
        ) -> *mut WebKitDOMNode;

        pub fn webkit_dom_node_get_owner_document(
            self_0: *mut WebKitDOMNode,
        ) -> *mut WebKitDOMDocument;

        pub fn webkit_dom_node_get_text_content(
            self_0: *mut WebKitDOMNode,
        ) -> *mut gchar;
    }
}

pub mod WebKitDOMObject_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _WebKitDOMObject {
        pub parentInstance: GObject,
        pub coreObject: gpointer,
    }
    use super::gobject_h::GObject;
    use super::gtypes_h::gpointer;
}

pub mod WebKitDOMCSSStyleDeclaration_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _WebKitDOMCSSStyleDeclaration {
        pub parent_instance: WebKitDOMObject,
    }
    use super::webkitdomdefines_h::{WebKitDOMObject, WebKitDOMCSSStyleDeclaration};
    use super::gtypes_h::gchar;
    extern "C" {

        pub fn webkit_dom_css_style_declaration_get_property_value(
            self_0: *mut WebKitDOMCSSStyleDeclaration,
            propertyName: *const gchar,
        ) -> *mut gchar;
    }
}

pub mod WebKitDOMStyleSheet_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _WebKitDOMStyleSheet {
        pub parent_instance: WebKitDOMObject,
    }
    use super::webkitdomdefines_h::{WebKitDOMObject, WebKitDOMStyleSheet};
    use super::gtype_h::GType;
    use super::gtypes_h::gchar;
    extern "C" {

        pub fn webkit_dom_style_sheet_get_type() -> GType;

        pub fn webkit_dom_style_sheet_get_href(
            self_0: *mut WebKitDOMStyleSheet,
        ) -> *mut gchar;
    }
}

pub mod WebKitDOMClientRect_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _WebKitDOMClientRect {
        pub parent_instance: WebKitDOMObject,
    }
    use super::webkitdomdefines_h::{WebKitDOMObject, WebKitDOMClientRect};
    use super::gtypes_h::gfloat;
    extern "C" {

        pub fn webkit_dom_client_rect_get_top(
            self_0: *mut WebKitDOMClientRect,
        ) -> gfloat;

        pub fn webkit_dom_client_rect_get_right(
            self_0: *mut WebKitDOMClientRect,
        ) -> gfloat;

        pub fn webkit_dom_client_rect_get_bottom(
            self_0: *mut WebKitDOMClientRect,
        ) -> gfloat;

        pub fn webkit_dom_client_rect_get_left(
            self_0: *mut WebKitDOMClientRect,
        ) -> gfloat;

        pub fn webkit_dom_client_rect_get_width(
            self_0: *mut WebKitDOMClientRect,
        ) -> gfloat;

        pub fn webkit_dom_client_rect_get_height(
            self_0: *mut WebKitDOMClientRect,
        ) -> gfloat;
    }
}

pub mod WebKitDOMClientRectList_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _WebKitDOMClientRectList {
        pub parent_instance: WebKitDOMObject,
    }
    use super::webkitdomdefines_h::{
        WebKitDOMObject, WebKitDOMClientRectList, WebKitDOMClientRect,
    };
    use super::gtypes_h::gulong;
    extern "C" {

        pub fn webkit_dom_client_rect_list_get_length(
            self_0: *mut WebKitDOMClientRectList,
        ) -> gulong;

        pub fn webkit_dom_client_rect_list_item(
            self_0: *mut WebKitDOMClientRectList,
            index: gulong,
        ) -> *mut WebKitDOMClientRect;
    }
}

pub mod WebKitDOMDOMWindow_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _WebKitDOMDOMWindow {
        pub parent_instance: WebKitDOMObject,
    }
    use super::webkitdomdefines_h::{
        WebKitDOMObject, WebKitDOMDOMWindow, WebKitDOMElement,
        WebKitDOMCSSStyleDeclaration,
    };
    use super::gtypes_h::gchar;
    extern "C" {

        pub fn webkit_dom_dom_window_get_computed_style(
            self_0: *mut WebKitDOMDOMWindow,
            element: *mut WebKitDOMElement,
            pseudoElement: *const gchar,
        ) -> *mut WebKitDOMCSSStyleDeclaration;
    }
}

pub mod WebKitDOMDocument_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _WebKitDOMDocument {
        pub parent_instance: WebKitDOMNode,
    }
    use super::webkitdomdefines_h::{
        WebKitDOMNode, WebKitDOMDocument, WebKitDOMEvent, WebKitDOMDOMWindow,
    };
    use super::gtypes_h::gchar;
    use super::gerror_h::GError;
    extern "C" {

        pub fn webkit_dom_document_create_event(
            self_0: *mut WebKitDOMDocument,
            eventType: *const gchar,
            error: *mut *mut GError,
        ) -> *mut WebKitDOMEvent;

        pub fn webkit_dom_document_get_default_view(
            self_0: *mut WebKitDOMDocument,
        ) -> *mut WebKitDOMDOMWindow;
    }
}

pub mod WebKitDOMElement_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _WebKitDOMElement {
        pub parent_instance: WebKitDOMNode,
    }
    use super::webkitdomdefines_h::{
        WebKitDOMNode, WebKitDOMElement, WebKitDOMNodeList, WebKitDOMClientRectList,
    };
    use super::gtype_h::GType;
    use super::gtypes_h::{gchar, gdouble, glong, gulong};
    use super::gerror_h::GError;
    extern "C" {

        pub fn webkit_dom_element_get_type() -> GType;

        pub fn webkit_dom_element_get_attribute(
            self_0: *mut WebKitDOMElement,
            name: *const gchar,
        ) -> *mut gchar;

        pub fn webkit_dom_element_set_attribute(
            self_0: *mut WebKitDOMElement,
            name: *const gchar,
            value: *const gchar,
            error: *mut *mut GError,
        );

        pub fn webkit_dom_element_focus(self_0: *mut WebKitDOMElement);

        pub fn webkit_dom_element_query_selector_all(
            self_0: *mut WebKitDOMElement,
            selectors: *const gchar,
            error: *mut *mut GError,
        ) -> *mut WebKitDOMNodeList;

        pub fn webkit_dom_element_get_tag_name(
            self_0: *mut WebKitDOMElement,
        ) -> *mut gchar;

        pub fn webkit_dom_element_get_offset_left(
            self_0: *mut WebKitDOMElement,
        ) -> gdouble;

        pub fn webkit_dom_element_get_offset_top(
            self_0: *mut WebKitDOMElement,
        ) -> gdouble;

        pub fn webkit_dom_element_get_offset_width(
            self_0: *mut WebKitDOMElement,
        ) -> gdouble;

        pub fn webkit_dom_element_get_offset_height(
            self_0: *mut WebKitDOMElement,
        ) -> gdouble;

        pub fn webkit_dom_element_get_scroll_left(
            self_0: *mut WebKitDOMElement,
        ) -> glong;

        pub fn webkit_dom_element_get_scroll_top(self_0: *mut WebKitDOMElement) -> glong;

        pub fn webkit_dom_element_get_client_rects(
            self_0: *mut WebKitDOMElement,
        ) -> *mut WebKitDOMClientRectList;

        pub fn webkit_dom_element_get_offset_parent(
            self_0: *mut WebKitDOMElement,
        ) -> *mut WebKitDOMElement;

        pub fn webkit_dom_element_get_inner_html(
            self_0: *mut WebKitDOMElement,
        ) -> *mut gchar;

        pub fn webkit_dom_element_set_inner_html(
            self_0: *mut WebKitDOMElement,
            value: *const gchar,
            error: *mut *mut GError,
        );

        pub fn webkit_dom_element_get_previous_element_sibling(
            self_0: *mut WebKitDOMElement,
        ) -> *mut WebKitDOMElement;

        pub fn webkit_dom_element_get_next_element_sibling(
            self_0: *mut WebKitDOMElement,
        ) -> *mut WebKitDOMElement;

        pub fn webkit_dom_element_get_first_element_child(
            self_0: *mut WebKitDOMElement,
        ) -> *mut WebKitDOMElement;

        pub fn webkit_dom_element_get_last_element_child(
            self_0: *mut WebKitDOMElement,
        ) -> *mut WebKitDOMElement;

        pub fn webkit_dom_element_get_child_element_count(
            self_0: *mut WebKitDOMElement,
        ) -> gulong;

        pub fn webkit_dom_element_remove(
            self_0: *mut WebKitDOMElement,
            error: *mut *mut GError,
        );
    }
}

pub mod WebKitDOMEvent_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _WebKitDOMEvent {
        pub parent_instance: WebKitDOMObject,
    }
    use super::webkitdomdefines_h::{
        WebKitDOMObject, WebKitDOMEvent, WebKitDOMEventTarget,
    };
    use super::gtypes_h::{gchar, gboolean, gushort};
    extern "C" {

        pub fn webkit_dom_event_stop_propagation(self_0: *mut WebKitDOMEvent);

        pub fn webkit_dom_event_prevent_default(self_0: *mut WebKitDOMEvent);

        pub fn webkit_dom_event_init_event(
            self_0: *mut WebKitDOMEvent,
            eventTypeArg: *const gchar,
            canBubbleArg: gboolean,
            cancelableArg: gboolean,
        );

        pub fn webkit_dom_event_get_event_type(
            self_0: *mut WebKitDOMEvent,
        ) -> *mut gchar;

        pub fn webkit_dom_event_get_event_phase(self_0: *mut WebKitDOMEvent) -> gushort;

        pub fn webkit_dom_event_get_src_element(
            self_0: *mut WebKitDOMEvent,
        ) -> *mut WebKitDOMEventTarget;
    }
}

pub mod WebKitDOMHTMLAnchorElement_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _WebKitDOMHTMLAnchorElement {
        pub parent_instance: WebKitDOMHTMLElement,
    }
    use super::webkitdomdefines_h::{WebKitDOMHTMLElement, WebKitDOMHTMLAnchorElement};
    use super::gtype_h::GType;
    use super::gtypes_h::gchar;
    extern "C" {

        pub fn webkit_dom_html_anchor_element_get_type() -> GType;

        pub fn webkit_dom_html_anchor_element_get_href(
            self_0: *mut WebKitDOMHTMLAnchorElement,
        ) -> *mut gchar;
    }
}

pub mod WebKitDOMHTMLElement_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _WebKitDOMHTMLElement {
        pub parent_instance: WebKitDOMElement,
    }
    use super::webkitdomdefines_h::WebKitDOMElement;
    use super::gtype_h::GType;
    extern "C" {

        pub fn webkit_dom_html_element_get_type() -> GType;
    }
}

pub mod WebKitDOMHTMLAreaElement_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _WebKitDOMHTMLAreaElement {
        pub parent_instance: WebKitDOMHTMLElement,
    }
    use super::webkitdomdefines_h::{WebKitDOMHTMLElement, WebKitDOMHTMLAreaElement};
    use super::gtype_h::GType;
    use super::gtypes_h::gchar;
    extern "C" {

        pub fn webkit_dom_html_area_element_get_type() -> GType;

        pub fn webkit_dom_html_area_element_get_href(
            self_0: *mut WebKitDOMHTMLAreaElement,
        ) -> *mut gchar;
    }
}

pub mod WebKitDOMHTMLButtonElement_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _WebKitDOMHTMLButtonElement {
        pub parent_instance: WebKitDOMHTMLElement,
    }
    use super::webkitdomdefines_h::{WebKitDOMHTMLElement, WebKitDOMHTMLButtonElement};
    use super::gtype_h::GType;
    use super::gtypes_h::gchar;
    extern "C" {

        pub fn webkit_dom_html_button_element_get_type() -> GType;

        pub fn webkit_dom_html_button_element_get_value(
            self_0: *mut WebKitDOMHTMLButtonElement,
        ) -> *mut gchar;

        pub fn webkit_dom_html_button_element_set_value(
            self_0: *mut WebKitDOMHTMLButtonElement,
            value: *const gchar,
        );
    }
}

pub mod WebKitDOMHTMLEmbedElement_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _WebKitDOMHTMLEmbedElement {
        pub parent_instance: WebKitDOMHTMLElement,
    }
    use super::webkitdomdefines_h::{WebKitDOMHTMLElement, WebKitDOMHTMLEmbedElement};
    use super::gtype_h::GType;
    use super::gtypes_h::gchar;
    extern "C" {

        pub fn webkit_dom_html_embed_element_get_type() -> GType;

        pub fn webkit_dom_html_embed_element_get_src(
            self_0: *mut WebKitDOMHTMLEmbedElement,
        ) -> *mut gchar;
    }
}

pub mod WebKitDOMHTMLFormElement_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _WebKitDOMHTMLFormElement {
        pub parent_instance: WebKitDOMHTMLElement,
    }
    use super::webkitdomdefines_h::{WebKitDOMHTMLElement, WebKitDOMHTMLFormElement};
    use super::gtype_h::GType;
    extern "C" {

        pub fn webkit_dom_html_form_element_get_type() -> GType;

        pub fn webkit_dom_html_form_element_submit(
            self_0: *mut WebKitDOMHTMLFormElement,
        );
    }
}

pub mod WebKitDOMHTMLFrameElement_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _WebKitDOMHTMLFrameElement {
        pub parent_instance: WebKitDOMHTMLElement,
    }
    use super::webkitdomdefines_h::{
        WebKitDOMHTMLElement, WebKitDOMHTMLFrameElement, WebKitDOMDocument,
    };
    use super::gtype_h::GType;
    use super::gtypes_h::gchar;
    extern "C" {

        pub fn webkit_dom_html_frame_element_get_type() -> GType;

        pub fn webkit_dom_html_frame_element_get_src(
            self_0: *mut WebKitDOMHTMLFrameElement,
        ) -> *mut gchar;

        pub fn webkit_dom_html_frame_element_get_content_document(
            self_0: *mut WebKitDOMHTMLFrameElement,
        ) -> *mut WebKitDOMDocument;
    }
}

pub mod WebKitDOMHTMLIFrameElement_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _WebKitDOMHTMLIFrameElement {
        pub parent_instance: WebKitDOMHTMLElement,
    }
    use super::webkitdomdefines_h::{
        WebKitDOMHTMLElement, WebKitDOMHTMLIFrameElement, WebKitDOMDocument,
    };
    use super::gtype_h::GType;
    use super::gtypes_h::gchar;
    extern "C" {

        pub fn webkit_dom_html_iframe_element_get_type() -> GType;

        pub fn webkit_dom_html_iframe_element_get_src(
            self_0: *mut WebKitDOMHTMLIFrameElement,
        ) -> *mut gchar;

        pub fn webkit_dom_html_iframe_element_get_content_document(
            self_0: *mut WebKitDOMHTMLIFrameElement,
        ) -> *mut WebKitDOMDocument;
    }
}

pub mod WebKitDOMHTMLImageElement_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _WebKitDOMHTMLImageElement {
        pub parent_instance: WebKitDOMHTMLElement,
    }
    use super::webkitdomdefines_h::{WebKitDOMHTMLElement, WebKitDOMHTMLImageElement};
    use super::gtype_h::GType;
    use super::gtypes_h::gchar;
    extern "C" {

        pub fn webkit_dom_html_image_element_get_type() -> GType;

        pub fn webkit_dom_html_image_element_get_src(
            self_0: *mut WebKitDOMHTMLImageElement,
        ) -> *mut gchar;
    }
}

pub mod WebKitDOMHTMLInputElement_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _WebKitDOMHTMLInputElement {
        pub parent_instance: WebKitDOMHTMLElement,
    }
    use super::webkitdomdefines_h::{WebKitDOMHTMLElement, WebKitDOMHTMLInputElement};
    use super::gtype_h::GType;
    use super::gtypes_h::{gboolean, gchar};
    extern "C" {

        pub fn webkit_dom_html_input_element_get_type() -> GType;

        pub fn webkit_dom_html_input_element_get_checked(
            self_0: *mut WebKitDOMHTMLInputElement,
        ) -> gboolean;

        pub fn webkit_dom_html_input_element_set_checked(
            self_0: *mut WebKitDOMHTMLInputElement,
            value: gboolean,
        );

        pub fn webkit_dom_html_input_element_get_src(
            self_0: *mut WebKitDOMHTMLInputElement,
        ) -> *mut gchar;

        pub fn webkit_dom_html_input_element_get_value(
            self_0: *mut WebKitDOMHTMLInputElement,
        ) -> *mut gchar;

        pub fn webkit_dom_html_input_element_set_value(
            self_0: *mut WebKitDOMHTMLInputElement,
            value: *const gchar,
        );
    }
}

pub mod WebKitDOMHTMLLIElement_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _WebKitDOMHTMLLIElement {
        pub parent_instance: WebKitDOMHTMLElement,
    }
    use super::webkitdomdefines_h::{WebKitDOMHTMLElement, WebKitDOMHTMLLIElement};
    use super::gtype_h::GType;
    use super::gtypes_h::glong;
    extern "C" {

        pub fn webkit_dom_html_li_element_get_type() -> GType;

        pub fn webkit_dom_html_li_element_get_value(
            self_0: *mut WebKitDOMHTMLLIElement,
        ) -> glong;

        pub fn webkit_dom_html_li_element_set_value(
            self_0: *mut WebKitDOMHTMLLIElement,
            value: glong,
        );
    }
}

pub mod WebKitDOMHTMLLinkElement_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _WebKitDOMHTMLLinkElement {
        pub parent_instance: WebKitDOMHTMLElement,
    }
    use super::webkitdomdefines_h::{WebKitDOMHTMLElement, WebKitDOMHTMLLinkElement};
    use super::gtype_h::GType;
    use super::gtypes_h::gchar;
    extern "C" {

        pub fn webkit_dom_html_link_element_get_type() -> GType;

        pub fn webkit_dom_html_link_element_get_href(
            self_0: *mut WebKitDOMHTMLLinkElement,
        ) -> *mut gchar;
    }
}

pub mod WebKitDOMHTMLOptionElement_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _WebKitDOMHTMLOptionElement {
        pub parent_instance: WebKitDOMHTMLElement,
    }
    use super::webkitdomdefines_h::{WebKitDOMHTMLElement, WebKitDOMHTMLOptionElement};
    use super::gtype_h::GType;
    use super::gtypes_h::gchar;
    extern "C" {

        pub fn webkit_dom_html_option_element_get_type() -> GType;

        pub fn webkit_dom_html_option_element_get_value(
            self_0: *mut WebKitDOMHTMLOptionElement,
        ) -> *mut gchar;

        pub fn webkit_dom_html_option_element_set_value(
            self_0: *mut WebKitDOMHTMLOptionElement,
            value: *const gchar,
        );
    }
}

pub mod WebKitDOMHTMLParamElement_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _WebKitDOMHTMLParamElement {
        pub parent_instance: WebKitDOMHTMLElement,
    }
    use super::webkitdomdefines_h::{WebKitDOMHTMLElement, WebKitDOMHTMLParamElement};
    use super::gtype_h::GType;
    use super::gtypes_h::gchar;
    extern "C" {

        pub fn webkit_dom_html_param_element_get_type() -> GType;

        pub fn webkit_dom_html_param_element_get_value(
            self_0: *mut WebKitDOMHTMLParamElement,
        ) -> *mut gchar;

        pub fn webkit_dom_html_param_element_set_value(
            self_0: *mut WebKitDOMHTMLParamElement,
            value: *const gchar,
        );
    }
}

pub mod WebKitDOMHTMLScriptElement_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _WebKitDOMHTMLScriptElement {
        pub parent_instance: WebKitDOMHTMLElement,
    }
    use super::webkitdomdefines_h::{WebKitDOMHTMLElement, WebKitDOMHTMLScriptElement};
    use super::gtype_h::GType;
    use super::gtypes_h::gchar;
    extern "C" {

        pub fn webkit_dom_html_script_element_get_type() -> GType;

        pub fn webkit_dom_html_script_element_get_src(
            self_0: *mut WebKitDOMHTMLScriptElement,
        ) -> *mut gchar;
    }
}

pub mod WebKitDOMHTMLSelectElement_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _WebKitDOMHTMLSelectElement {
        pub parent_instance: WebKitDOMHTMLElement,
    }
    use super::webkitdomdefines_h::{WebKitDOMHTMLElement, WebKitDOMHTMLSelectElement};
    use super::gtype_h::GType;
    use super::gtypes_h::gchar;
    extern "C" {

        pub fn webkit_dom_html_select_element_get_type() -> GType;

        pub fn webkit_dom_html_select_element_get_value(
            self_0: *mut WebKitDOMHTMLSelectElement,
        ) -> *mut gchar;

        pub fn webkit_dom_html_select_element_set_value(
            self_0: *mut WebKitDOMHTMLSelectElement,
            value: *const gchar,
        );
    }
}

pub mod WebKitDOMHTMLTextAreaElement_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _WebKitDOMHTMLTextAreaElement {
        pub parent_instance: WebKitDOMHTMLElement,
    }
    use super::webkitdomdefines_h::{WebKitDOMHTMLElement, WebKitDOMHTMLTextAreaElement};
    use super::gtype_h::GType;
    use super::gtypes_h::gchar;
    extern "C" {

        pub fn webkit_dom_html_text_area_element_get_type() -> GType;

        pub fn webkit_dom_html_text_area_element_get_value(
            self_0: *mut WebKitDOMHTMLTextAreaElement,
        ) -> *mut gchar;

        pub fn webkit_dom_html_text_area_element_set_value(
            self_0: *mut WebKitDOMHTMLTextAreaElement,
            value: *const gchar,
        );
    }
}

pub mod WebKitDOMKeyboardEvent_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _WebKitDOMKeyboardEvent {
        pub parent_instance: WebKitDOMUIEvent,
    }
    use super::webkitdomdefines_h::{WebKitDOMUIEvent, WebKitDOMKeyboardEvent};
    use super::gtype_h::GType;
    use super::gtypes_h::{gchar, gboolean};
    extern "C" {

        pub fn webkit_dom_keyboard_event_get_type() -> GType;

        pub fn webkit_dom_keyboard_event_get_key_identifier(
            self_0: *mut WebKitDOMKeyboardEvent,
        ) -> *mut gchar;

        pub fn webkit_dom_keyboard_event_get_ctrl_key(
            self_0: *mut WebKitDOMKeyboardEvent,
        ) -> gboolean;

        pub fn webkit_dom_keyboard_event_get_shift_key(
            self_0: *mut WebKitDOMKeyboardEvent,
        ) -> gboolean;

        pub fn webkit_dom_keyboard_event_get_alt_key(
            self_0: *mut WebKitDOMKeyboardEvent,
        ) -> gboolean;

        pub fn webkit_dom_keyboard_event_get_meta_key(
            self_0: *mut WebKitDOMKeyboardEvent,
        ) -> gboolean;
    }
}

pub mod WebKitDOMUIEvent_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _WebKitDOMUIEvent {
        pub parent_instance: WebKitDOMEvent,
    }
    use super::webkitdomdefines_h::{WebKitDOMEvent, WebKitDOMUIEvent};
    use super::gtype_h::GType;
    use super::gtypes_h::glong;
    extern "C" {

        pub fn webkit_dom_ui_event_get_type() -> GType;

        pub fn webkit_dom_ui_event_get_char_code(self_0: *mut WebKitDOMUIEvent) -> glong;
    }
}

pub mod WebKitDOMMouseEvent_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _WebKitDOMMouseEvent {
        pub parent_instance: WebKitDOMUIEvent,
    }
    use super::webkitdomdefines_h::{WebKitDOMUIEvent, WebKitDOMMouseEvent};
    use super::gtype_h::GType;
    use super::gtypes_h::gushort;
    extern "C" {

        pub fn webkit_dom_mouse_event_get_type() -> GType;

        pub fn webkit_dom_mouse_event_get_button(
            self_0: *mut WebKitDOMMouseEvent,
        ) -> gushort;
    }
}

pub mod WebKitDOMNodeList_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _WebKitDOMNodeList {
        pub parent_instance: WebKitDOMObject,
    }
    use super::webkitdomdefines_h::{WebKitDOMObject, WebKitDOMNodeList, WebKitDOMNode};
    use super::gtypes_h::gulong;
    extern "C" {

        pub fn webkit_dom_node_list_item(
            self_0: *mut WebKitDOMNodeList,
            index: gulong,
        ) -> *mut WebKitDOMNode;

        pub fn webkit_dom_node_list_get_length(self_0: *mut WebKitDOMNodeList) -> gulong;
    }
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

pub mod WebKitFrame_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _WebKitFrame {
        pub parent: GObject,
        pub priv_0: *mut WebKitFramePrivate,
    }

    pub type WebKitFramePrivate = _WebKitFramePrivate;

    pub type WebKitFrame = _WebKitFrame;
    use super::gobject_h::GObject;
    use super::WebKitScriptWorld_h::WebKitScriptWorld;
    use super::JSCValue_h::JSCContext;
    extern "C" {

        pub type _WebKitFramePrivate;

        pub fn webkit_frame_get_js_context_for_script_world(
            frame: *mut WebKitFrame,
            world: *mut WebKitScriptWorld,
        ) -> *mut JSCContext;
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
    use super::WebKitFrame_h::WebKitFrame;
    extern "C" {

        pub type _WebKitWebPagePrivate;

        pub fn webkit_web_page_get_main_frame(
            web_page: *mut WebKitWebPage,
        ) -> *mut WebKitFrame;
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
    extern "C" {

        pub type _WebKitWebExtensionPrivate;
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

    pub const LUA_GLOBALSINDEX: std::ffi::c_int = -(10002 as std::ffi::c_int);

    pub const LUA_TFUNCTION: std::ffi::c_int = 6 as std::ffi::c_int;
    use super::__stddef_ptrdiff_t_h::ptrdiff_t;
    use super::__stddef_size_t_h::size_t;
    extern "C" {

        pub type lua_State;

        pub fn lua_gettop(L: *mut lua_State) -> std::ffi::c_int;

        pub fn lua_settop(L: *mut lua_State, idx: std::ffi::c_int);

        pub fn lua_pushvalue(L: *mut lua_State, idx: std::ffi::c_int);

        pub fn lua_remove(L: *mut lua_State, idx: std::ffi::c_int);

        pub fn lua_insert(L: *mut lua_State, idx: std::ffi::c_int);

        pub fn lua_type(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;

        pub fn lua_toboolean(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;

        pub fn lua_tolstring(
            L: *mut lua_State,
            idx: std::ffi::c_int,
            len: *mut size_t,
        ) -> *const std::ffi::c_char;

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

        pub fn lua_createtable(
            L: *mut lua_State,
            narr: std::ffi::c_int,
            nrec: std::ffi::c_int,
        );

        pub fn lua_newuserdata(L: *mut lua_State, sz: size_t) -> *mut std::ffi::c_void;

        pub fn lua_getfenv(L: *mut lua_State, idx: std::ffi::c_int);

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

        pub fn lua_setfenv(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;

        pub fn lua_pcall(
            L: *mut lua_State,
            nargs: std::ffi::c_int,
            nresults: std::ffi::c_int,
            errfunc: std::ffi::c_int,
        ) -> std::ffi::c_int;
    }
}

pub mod lauxlib_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct luaL_Reg {
        pub name: *const std::ffi::c_char,
        pub func: lua_CFunction,
    }
    use super::lua_h::{lua_CFunction, lua_State, lua_Integer};
    use super::__stddef_size_t_h::size_t;
    extern "C" {

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

        pub fn luaL_checkinteger(
            L: *mut lua_State,
            numArg: std::ffi::c_int,
        ) -> lua_Integer;

        pub fn luaL_checkstack(
            L: *mut lua_State,
            sz: std::ffi::c_int,
            msg: *const std::ffi::c_char,
        );

        pub fn luaL_error(
            L: *mut lua_State,
            fmt: *const std::ffi::c_char,
            _: ...
        ) -> std::ffi::c_int;
    }
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
    extern "C" {

        pub fn _log(lvl: log_level_t, _: *const gchar, _: *const gchar, _: ...);
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

pub mod common_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _common_t {
        pub L: *mut lua_State,
    }

    pub type common_t = _common_t;
    use super::lua_h::lua_State;
    extern "C" {

        pub static mut common: common_t;
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

    pub type signal_array_t = GPtrArray;
    #[inline]

    pub unsafe extern "C" fn signal_cmp(
        mut a: gconstpointer,
        mut b: gconstpointer,
        mut UNUSED_p: gpointer,
    ) -> gint {
        return g_strcmp0(a as *const std::ffi::c_char, b as *const std::ffi::c_char);
    }
    #[inline]

    pub unsafe extern "C" fn signal_array_destroy(mut sigfuncs: *mut gpointer) {
        g_ptr_array_free(sigfuncs as *mut GPtrArray, TRUE);
    }
    #[inline]

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
            NULL_0 as *mut std::ffi::c_void,
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut gpointer) -> ()>,
                GDestroyNotify,
            >(Some(signal_array_destroy as unsafe extern "C" fn(*mut gpointer) -> ())),
        ) as *mut signal_t;
    }
    #[inline]

    pub unsafe extern "C" fn signal_destroy(mut signals: *mut signal_t) {
        g_tree_destroy(signals as *mut GTree);
    }
    #[inline]

    pub unsafe extern "C" fn signal_lookup(
        mut signals: *mut signal_t,
        mut name: *const gchar,
    ) -> *mut signal_array_t {
        return g_tree_lookup(signals as *mut GTree, name as gpointer as gconstpointer)
            as *mut signal_array_t;
    }
    #[inline]

    pub unsafe extern "C" fn signal_add(
        mut signals: *mut signal_t,
        mut name: *const gchar,
        mut func: gpointer,
    ) {
        let mut sigfuncs = signal_lookup(signals, name);
        if sigfuncs.is_null() {
            sigfuncs = g_ptr_array_new() as *mut signal_array_t;
            g_tree_insert(
                signals as *mut GTree,
                g_strdup_inline(name) as gpointer,
                sigfuncs as gpointer,
            );
        }
        g_ptr_array_add(sigfuncs as *mut GPtrArray, func);
    }
    #[inline]

    pub unsafe extern "C" fn signal_remove(
        mut signals: *mut signal_t,
        mut name: *const gchar,
        mut func: gpointer,
    ) {
        let mut sigfuncs = signal_lookup(signals, name);
        if !sigfuncs.is_null() {
            g_ptr_array_remove(sigfuncs as *mut GPtrArray, func);
            if (*sigfuncs).len == 0 {
                g_tree_remove(signals as *mut GTree, name as gpointer as gconstpointer);
            }
        }
    }
    use super::gtree_h::{
        GTree, g_tree_new_full, g_tree_destroy, g_tree_lookup, g_tree_insert,
        g_tree_remove,
    };
    use super::garray_h::{
        GPtrArray, g_ptr_array_free, g_ptr_array_new, g_ptr_array_add, g_ptr_array_remove,
    };
    use super::gtypes_h::{
        gconstpointer, gpointer, gint, GCompareDataFunc, GDestroyNotify, gchar,
    };
    use super::gtestutils_h::g_strcmp0;
    use super::gmacros_h::{FALSE, TRUE};
    use super::__stddef_null_h::NULL_0;
    use super::gmem_h::g_free;
    use super::gstrfuncs_h::g_strdup_inline;
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
    extern "C" {

        pub fn l_tokenize(_: *const gchar) -> luakit_token_t;
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
    use super::lauxlib_h::luaL_Reg;
    extern "C" {

        pub fn luaH_class_add_signal(
            _: *mut lua_State,
            _: *mut lua_class_t,
            name: *const gchar,
            ud: gint,
        );

        pub fn luaH_class_remove_signal(
            _: *mut lua_State,
            _: *mut lua_class_t,
            name: *const gchar,
            ud: gint,
        );

        pub fn luaH_class_emit_signal(
            _: *mut lua_State,
            _: *mut lua_class_t,
            name: *const gchar,
            nargs: gint,
            nret: gint,
        ) -> gint;

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

        pub fn luaH_usemetatable(_: *mut lua_State, _: gint, _: gint) -> gint;

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

pub mod page_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _page_t {
        pub signals: *mut signal_t,
        pub page: *mut WebKitWebPage,
        pub ref_0: gpointer,
    }

    pub type page_t = _page_t;
    use super::signal_h::signal_t;
    use super::WebKitWebPage_h::WebKitWebPage;
    use super::gtypes_h::gpointer;
}

pub mod dom_element_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _dom_element_t {
        pub signals: *mut signal_t,
        pub dom_events: *mut signal_t,
        pub element: *mut WebKitDOMElement,
    }

    pub type dom_element_t = _dom_element_t;
    use super::signal_h::signal_t;
    use super::webkitdomdefines_h::WebKitDOMElement;
}

pub mod string_h {
    extern "C" {

        pub fn memcpy(
            _: *mut std::ffi::c_void,
            _: *const std::ffi::c_void,
            _: std::ffi::c_ulong,
        ) -> *mut std::ffi::c_void;

        pub fn memset(
            _: *mut std::ffi::c_void,
            _: std::ffi::c_int,
            _: std::ffi::c_ulong,
        ) -> *mut std::ffi::c_void;

        pub fn memcmp(
            _: *const std::ffi::c_void,
            _: *const std::ffi::c_void,
            _: std::ffi::c_ulong,
        ) -> std::ffi::c_int;

        pub fn strcmp(
            _: *const std::ffi::c_char,
            _: *const std::ffi::c_char,
        ) -> std::ffi::c_int;

        pub fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    }
}

pub mod gmem_h {
    use super::gtypes_h::gpointer;
    use super::glibconfig_h::gsize;
    extern "C" {

        pub fn g_free(mem: gpointer);

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
    use super::gtypes_h::{gchar, gboolean};
    use super::__stddef_null_h::NULL;
    use super::string_h::{strlen, memcpy};
    use super::__stddef_size_t_h::size_t;
    use super::gmem_h::g_malloc;
    extern "C" {

        pub fn g_str_has_suffix(str: *const gchar, suffix: *const gchar) -> gboolean;

        pub fn g_strdup(str: *const gchar) -> *mut gchar;

        pub fn g_strdup_printf(format: *const gchar, _: ...) -> *mut gchar;

        pub fn g_strjoin(separator: *const gchar, _: ...) -> *mut gchar;

        pub fn g_strjoinv(
            separator: *const gchar,
            str_array: *mut *mut gchar,
        ) -> *mut gchar;
    }
}

pub mod gtestutils_h {
    extern "C" {

        pub fn g_strcmp0(
            str1: *const std::ffi::c_char,
            str2: *const std::ffi::c_char,
        ) -> std::ffi::c_int;
    }
}

pub mod WebKitDOMEventTarget_h {
    use super::gtype_h::GType;
    use super::webkitdomdefines_h::{WebKitDOMEventTarget, WebKitDOMEvent};
    use super::gerror_h::GError;
    use super::gtypes_h::{gboolean, gpointer};
    use super::gclosure_h::GCallback;
    extern "C" {

        pub fn webkit_dom_event_target_get_type() -> GType;

        pub fn webkit_dom_event_target_dispatch_event(
            target: *mut WebKitDOMEventTarget,
            event: *mut WebKitDOMEvent,
            error: *mut *mut GError,
        ) -> gboolean;

        pub fn webkit_dom_event_target_add_event_listener(
            target: *mut WebKitDOMEventTarget,
            event_name: *const std::ffi::c_char,
            handler: GCallback,
            use_capture: gboolean,
            user_data: gpointer,
        ) -> gboolean;

        pub fn webkit_dom_event_target_remove_event_listener(
            target: *mut WebKitDOMEventTarget,
            event_name: *const std::ffi::c_char,
            handler: GCallback,
            use_capture: gboolean,
        ) -> gboolean;
    }
}

pub mod util_h {
    use super::lua_h::lua_State;
    use super::gtypes_h::gchar;
    extern "C" {

        pub fn luaH_callerinfo(_: *mut lua_State) -> *mut gchar;
    }
}

pub mod luautil_h {
    use super::lua_h::lua_State;
    use super::gtypes_h::gint;
    extern "C" {

        pub fn luaH_dofunction_on_error(L: *mut lua_State) -> gint;
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

    pub unsafe extern "C" fn luaH_object_ref_item(
        mut L: *mut lua_State,
        mut ud: gint,
        mut iud: gint,
    ) -> gpointer {
        lua_getfenv(L, ud);
        let mut p = luaH_object_incref(
            L,
            -(1 as std::ffi::c_int),
            if iud < 0 as std::ffi::c_int { iud - 1 as std::ffi::c_int } else { iud },
        );
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        return p;
    }
    #[inline]

    pub unsafe extern "C" fn luaH_object_unref_item(
        mut L: *mut lua_State,
        mut ud: gint,
        mut p: gpointer,
    ) {
        lua_getfenv(L, ud);
        luaH_object_decref(L, -(1 as std::ffi::c_int), p);
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    }
    #[inline]

    pub unsafe extern "C" fn luaH_object_push_item(
        mut L: *mut lua_State,
        mut ud: gint,
        mut p: gpointer,
    ) -> gint {
        lua_getfenv(L, ud);
        lua_pushlightuserdata(L, p);
        lua_rawget(L, -(2 as std::ffi::c_int));
        lua_remove(L, -(2 as std::ffi::c_int));
        return 1 as std::ffi::c_int;
    }
    use super::lua_h::{
        lua_State, lua_getfenv, lua_settop, lua_pushlightuserdata, lua_rawget, lua_remove,
    };
    use super::luaclass_h::lua_class_t;
    use super::gtypes_h::{gint, gpointer, gchar};
    use super::tokenize_h::{luakit_token_t, L_TK_UNKNOWN};
    extern "C" {

        pub fn luaH_settype(L: *mut lua_State, lua_class: *mut lua_class_t) -> gint;

        pub fn luaH_object_incref(L: *mut lua_State, tud: gint, oud: gint) -> gpointer;

        pub fn luaH_object_decref(L: *mut lua_State, tud: gint, oud: gpointer);

        pub fn luaH_object_emit_signal(
            L: *mut lua_State,
            oud: gint,
            name: *const gchar,
            nargs: gint,
            nret: gint,
        ) -> gint;

        pub fn luaH_object_add_signal_simple(L: *mut lua_State) -> gint;

        pub fn luaH_object_remove_signal_simple(L: *mut lua_State) -> gint;

        pub fn luaH_object_remove_signals_simple(L: *mut lua_State) -> gint;

        pub fn luaH_object_emit_signal_simple(L: *mut lua_State) -> gint;

        pub fn luaH_object_property_signal(
            _: *mut lua_State,
            _: gint,
            _: luakit_token_t,
        ) -> gint;

        pub fn luaH_object_tostring(_: *mut lua_State) -> gint;

        pub fn luaH_object_gc(_: *mut lua_State) -> gint;
    }
}

pub mod dom_document_h {
    use super::lua_h::lua_State;
    use super::webkitdomdefines_h::WebKitDOMDocument;
    use super::gtypes_h::gint;
    extern "C" {

        pub fn luaH_dom_document_from_webkit_dom_document(
            L: *mut lua_State,
            doc: *mut WebKitDOMDocument,
        ) -> gint;
    }
}

pub mod luauniq_h {
    use super::lua_h::lua_State;
    use super::gtypes_h::{gchar, gpointer};
    extern "C" {

        pub fn luaH_uniq_setup(L: *mut lua_State, reg: *const gchar, mode: *const gchar);

        pub fn luaH_uniq_add_ptr(
            L: *mut lua_State,
            reg: *const gchar,
            key: gpointer,
            oud: std::ffi::c_int,
        ) -> std::ffi::c_int;

        pub fn luaH_uniq_get_ptr(
            L: *mut lua_State,
            reg: *const gchar,
            key: gpointer,
        ) -> std::ffi::c_int;

        pub fn luaH_uniq_del_ptr(L: *mut lua_State, reg: *const gchar, key: gpointer);
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

pub mod stdbool_h {

    pub const true_0: std::ffi::c_int = 1 as std::ffi::c_int;

    pub const false_0: std::ffi::c_int = 0 as std::ffi::c_int;
}
pub use self::__stddef_ptrdiff_t_h::ptrdiff_t;
pub use self::__stddef_size_t_h::size_t;
pub use self::glibconfig_h::{guint32, gint64, gsize};
pub use self::gtypes_h::{
    gchar, glong, gint, gboolean, gushort, gulong, guint, gfloat, gdouble, gpointer,
    gconstpointer, GCompareDataFunc, GDestroyNotify,
};
pub use self::garray_h::{
    _GPtrArray, GPtrArray, g_ptr_array_new, g_ptr_array_new_full, g_ptr_array_free,
    g_ptr_array_remove, g_ptr_array_add,
};
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
pub use self::gtree_h::{
    GTree, GTraverseFunc, _GTree, g_tree_new_full, g_tree_destroy, g_tree_insert,
    g_tree_remove, g_tree_lookup, g_tree_foreach,
};
pub use self::gtype_h::{
    GType, _GTypeClass, GTypeClass, _GTypeInstance, GTypeInstance,
    G_TYPE_FUNDAMENTAL_SHIFT, G_TYPE_NONE, G_TYPE_STRING, g_type_check_instance_cast,
    g_type_check_instance_is_a,
};
pub use self::gclosure_h::GCallback;
pub use self::gobject_h::{
    _GObject, GObject, GWeakNotify, g_object_get, g_object_unref, g_object_weak_ref,
};
pub use self::JSCValue_h::{
    _JSCValue, JSCValuePrivate, JSCValue, JSCContext, _JSCValuePrivate,
    jsc_value_object_get_property, jsc_value_object_invoke_method,
};
pub use self::JSCContext_h::{
    _JSCContext, JSCContextPrivate, _JSCContextPrivate, jsc_context_get_global_object,
};
pub use self::webkitdomdefines_h::{
    WebKitDOMNode, WebKitDOMObject, WebKitDOMCSSStyleDeclaration, WebKitDOMStyleSheet,
    WebKitDOMClientRect, WebKitDOMClientRectList, WebKitDOMDOMWindow, WebKitDOMDocument,
    WebKitDOMElement, WebKitDOMEvent, WebKitDOMEventTarget, WebKitDOMHTMLElement,
    WebKitDOMHTMLAnchorElement, WebKitDOMHTMLAreaElement, WebKitDOMHTMLButtonElement,
    WebKitDOMHTMLEmbedElement, WebKitDOMHTMLFormElement, WebKitDOMHTMLFrameElement,
    WebKitDOMHTMLIFrameElement, WebKitDOMHTMLImageElement, WebKitDOMHTMLInputElement,
    WebKitDOMHTMLLIElement, WebKitDOMHTMLLinkElement, WebKitDOMHTMLOptionElement,
    WebKitDOMHTMLParamElement, WebKitDOMHTMLScriptElement, WebKitDOMHTMLSelectElement,
    WebKitDOMHTMLTextAreaElement, WebKitDOMUIEvent, WebKitDOMKeyboardEvent,
    WebKitDOMMouseEvent, WebKitDOMNodeList, _WebKitDOMEventTarget,
};
pub use self::WebKitDOMNode_h::{
    _WebKitDOMNode, webkit_dom_node_get_type, webkit_dom_node_append_child,
    webkit_dom_node_get_parent_node, webkit_dom_node_get_owner_document,
    webkit_dom_node_get_text_content,
};
pub use self::WebKitDOMObject_h::_WebKitDOMObject;
pub use self::WebKitDOMCSSStyleDeclaration_h::{
    _WebKitDOMCSSStyleDeclaration, webkit_dom_css_style_declaration_get_property_value,
};
pub use self::WebKitDOMStyleSheet_h::{
    _WebKitDOMStyleSheet, webkit_dom_style_sheet_get_type,
    webkit_dom_style_sheet_get_href,
};
pub use self::WebKitDOMClientRect_h::{
    _WebKitDOMClientRect, webkit_dom_client_rect_get_top,
    webkit_dom_client_rect_get_right, webkit_dom_client_rect_get_bottom,
    webkit_dom_client_rect_get_left, webkit_dom_client_rect_get_width,
    webkit_dom_client_rect_get_height,
};
pub use self::WebKitDOMClientRectList_h::{
    _WebKitDOMClientRectList, webkit_dom_client_rect_list_get_length,
    webkit_dom_client_rect_list_item,
};
pub use self::WebKitDOMDOMWindow_h::{
    _WebKitDOMDOMWindow, webkit_dom_dom_window_get_computed_style,
};
pub use self::WebKitDOMDocument_h::{
    _WebKitDOMDocument, webkit_dom_document_create_event,
    webkit_dom_document_get_default_view,
};
pub use self::WebKitDOMElement_h::{
    _WebKitDOMElement, webkit_dom_element_get_type, webkit_dom_element_get_attribute,
    webkit_dom_element_set_attribute, webkit_dom_element_focus,
    webkit_dom_element_query_selector_all, webkit_dom_element_get_tag_name,
    webkit_dom_element_get_offset_left, webkit_dom_element_get_offset_top,
    webkit_dom_element_get_offset_width, webkit_dom_element_get_offset_height,
    webkit_dom_element_get_scroll_left, webkit_dom_element_get_scroll_top,
    webkit_dom_element_get_client_rects, webkit_dom_element_get_offset_parent,
    webkit_dom_element_get_inner_html, webkit_dom_element_set_inner_html,
    webkit_dom_element_get_previous_element_sibling,
    webkit_dom_element_get_next_element_sibling,
    webkit_dom_element_get_first_element_child,
    webkit_dom_element_get_last_element_child,
    webkit_dom_element_get_child_element_count, webkit_dom_element_remove,
};
pub use self::WebKitDOMEvent_h::{
    _WebKitDOMEvent, webkit_dom_event_stop_propagation, webkit_dom_event_prevent_default,
    webkit_dom_event_init_event, webkit_dom_event_get_event_type,
    webkit_dom_event_get_event_phase, webkit_dom_event_get_src_element,
};
pub use self::WebKitDOMHTMLAnchorElement_h::{
    _WebKitDOMHTMLAnchorElement, webkit_dom_html_anchor_element_get_type,
    webkit_dom_html_anchor_element_get_href,
};
pub use self::WebKitDOMHTMLElement_h::{
    _WebKitDOMHTMLElement, webkit_dom_html_element_get_type,
};
pub use self::WebKitDOMHTMLAreaElement_h::{
    _WebKitDOMHTMLAreaElement, webkit_dom_html_area_element_get_type,
    webkit_dom_html_area_element_get_href,
};
pub use self::WebKitDOMHTMLButtonElement_h::{
    _WebKitDOMHTMLButtonElement, webkit_dom_html_button_element_get_type,
    webkit_dom_html_button_element_get_value, webkit_dom_html_button_element_set_value,
};
pub use self::WebKitDOMHTMLEmbedElement_h::{
    _WebKitDOMHTMLEmbedElement, webkit_dom_html_embed_element_get_type,
    webkit_dom_html_embed_element_get_src,
};
pub use self::WebKitDOMHTMLFormElement_h::{
    _WebKitDOMHTMLFormElement, webkit_dom_html_form_element_get_type,
    webkit_dom_html_form_element_submit,
};
pub use self::WebKitDOMHTMLFrameElement_h::{
    _WebKitDOMHTMLFrameElement, webkit_dom_html_frame_element_get_type,
    webkit_dom_html_frame_element_get_src,
    webkit_dom_html_frame_element_get_content_document,
};
pub use self::WebKitDOMHTMLIFrameElement_h::{
    _WebKitDOMHTMLIFrameElement, webkit_dom_html_iframe_element_get_type,
    webkit_dom_html_iframe_element_get_src,
    webkit_dom_html_iframe_element_get_content_document,
};
pub use self::WebKitDOMHTMLImageElement_h::{
    _WebKitDOMHTMLImageElement, webkit_dom_html_image_element_get_type,
    webkit_dom_html_image_element_get_src,
};
pub use self::WebKitDOMHTMLInputElement_h::{
    _WebKitDOMHTMLInputElement, webkit_dom_html_input_element_get_type,
    webkit_dom_html_input_element_get_checked, webkit_dom_html_input_element_set_checked,
    webkit_dom_html_input_element_get_src, webkit_dom_html_input_element_get_value,
    webkit_dom_html_input_element_set_value,
};
pub use self::WebKitDOMHTMLLIElement_h::{
    _WebKitDOMHTMLLIElement, webkit_dom_html_li_element_get_type,
    webkit_dom_html_li_element_get_value, webkit_dom_html_li_element_set_value,
};
pub use self::WebKitDOMHTMLLinkElement_h::{
    _WebKitDOMHTMLLinkElement, webkit_dom_html_link_element_get_type,
    webkit_dom_html_link_element_get_href,
};
pub use self::WebKitDOMHTMLOptionElement_h::{
    _WebKitDOMHTMLOptionElement, webkit_dom_html_option_element_get_type,
    webkit_dom_html_option_element_get_value, webkit_dom_html_option_element_set_value,
};
pub use self::WebKitDOMHTMLParamElement_h::{
    _WebKitDOMHTMLParamElement, webkit_dom_html_param_element_get_type,
    webkit_dom_html_param_element_get_value, webkit_dom_html_param_element_set_value,
};
pub use self::WebKitDOMHTMLScriptElement_h::{
    _WebKitDOMHTMLScriptElement, webkit_dom_html_script_element_get_type,
    webkit_dom_html_script_element_get_src,
};
pub use self::WebKitDOMHTMLSelectElement_h::{
    _WebKitDOMHTMLSelectElement, webkit_dom_html_select_element_get_type,
    webkit_dom_html_select_element_get_value, webkit_dom_html_select_element_set_value,
};
pub use self::WebKitDOMHTMLTextAreaElement_h::{
    _WebKitDOMHTMLTextAreaElement, webkit_dom_html_text_area_element_get_type,
    webkit_dom_html_text_area_element_get_value,
    webkit_dom_html_text_area_element_set_value,
};
pub use self::WebKitDOMKeyboardEvent_h::{
    _WebKitDOMKeyboardEvent, webkit_dom_keyboard_event_get_type,
    webkit_dom_keyboard_event_get_key_identifier, webkit_dom_keyboard_event_get_ctrl_key,
    webkit_dom_keyboard_event_get_shift_key, webkit_dom_keyboard_event_get_alt_key,
    webkit_dom_keyboard_event_get_meta_key,
};
pub use self::WebKitDOMUIEvent_h::{
    _WebKitDOMUIEvent, webkit_dom_ui_event_get_type, webkit_dom_ui_event_get_char_code,
};
pub use self::WebKitDOMMouseEvent_h::{
    _WebKitDOMMouseEvent, webkit_dom_mouse_event_get_type,
    webkit_dom_mouse_event_get_button,
};
pub use self::WebKitDOMNodeList_h::{
    _WebKitDOMNodeList, webkit_dom_node_list_item, webkit_dom_node_list_get_length,
};
pub use self::WebKitScriptWorld_h::{
    _WebKitScriptWorld, WebKitScriptWorldPrivate, WebKitScriptWorld,
    _WebKitScriptWorldPrivate,
};
pub use self::WebKitFrame_h::{
    _WebKitFrame, WebKitFramePrivate, WebKitFrame, _WebKitFramePrivate,
    webkit_frame_get_js_context_for_script_world,
};
pub use self::WebKitWebPage_h::{
    _WebKitWebPage, WebKitWebPagePrivate, WebKitWebPage, _WebKitWebPagePrivate,
    webkit_web_page_get_main_frame,
};
pub use self::WebKitWebExtension_h::{
    _WebKitWebExtension, WebKitWebExtensionPrivate, WebKitWebExtension,
    _WebKitWebExtensionPrivate,
};
pub use self::lua_h::{
    lua_CFunction, lua_Number, lua_Integer, LUA_MULTRET, LUA_REGISTRYINDEX,
    LUA_GLOBALSINDEX, LUA_TFUNCTION, lua_State, lua_gettop, lua_settop, lua_pushvalue,
    lua_remove, lua_insert, lua_type, lua_toboolean, lua_tolstring, lua_topointer,
    lua_pushnil, lua_pushnumber, lua_pushinteger, lua_pushlstring, lua_pushstring,
    lua_pushcclosure, lua_pushboolean, lua_pushlightuserdata, lua_rawget,
    lua_createtable, lua_newuserdata, lua_getfenv, lua_setfield, lua_rawset, lua_rawseti,
    lua_setmetatable, lua_setfenv, lua_pcall,
};
pub use self::lauxlib_h::{
    luaL_Reg, luaL_typerror, luaL_argerror, luaL_checklstring, luaL_checkinteger,
    luaL_checkstack, luaL_error,
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
pub use self::signal_h::{
    signal_t, signal_array_t, signal_cmp, signal_array_destroy, signal_new,
    signal_destroy, signal_lookup, signal_add, signal_remove,
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
pub use self::luaclass_h::{
    lua_class_property_array_t, lua_object_t, lua_class_allocator_t,
    lua_class_propfunc_t, lua_class_t, luaH_class_add_signal, luaH_class_remove_signal,
    luaH_class_emit_signal, luaH_class_setup, luaH_usemetatable, luaH_checkudata,
    luaH_toudata,
};
pub use self::page_h::{_page_t, page_t};
pub use self::dom_element_h::{_dom_element_t, dom_element_t};
use self::string_h::{memcpy, memset, memcmp, strcmp, strlen};
use self::gmem_h::{g_free, g_malloc};
pub use self::gstrfuncs_h::{
    g_strdup_inline, g_str_has_suffix, g_strdup, g_strdup_printf, g_strjoin, g_strjoinv,
};
use self::gtestutils_h::g_strcmp0;
use self::WebKitDOMEventTarget_h::{
    webkit_dom_event_target_get_type, webkit_dom_event_target_dispatch_event,
    webkit_dom_event_target_add_event_listener,
    webkit_dom_event_target_remove_event_listener,
};
use self::util_h::luaH_callerinfo;
use self::luautil_h::luaH_dofunction_on_error;
pub use self::lualib_h::{luaH_absindex, luaH_dofunction};
pub use self::luaobject_h::{
    luaH_object_ref_item, luaH_object_unref_item, luaH_object_push_item, luaH_settype,
    luaH_object_incref, luaH_object_decref, luaH_object_emit_signal,
    luaH_object_add_signal_simple, luaH_object_remove_signal_simple,
    luaH_object_remove_signals_simple, luaH_object_emit_signal_simple,
    luaH_object_property_signal, luaH_object_tostring, luaH_object_gc,
};
use self::dom_document_h::luaH_dom_document_from_webkit_dom_document;
use self::luauniq_h::{
    luaH_uniq_setup, luaH_uniq_add_ptr, luaH_uniq_get_ptr, luaH_uniq_del_ptr,
};
pub use self::gmacros_h::{FALSE, TRUE};
pub use self::__stddef_null_h::{NULL, NULL_0};
pub use self::stdbool_h::{true_0, false_0};

pub const REG_KEY: [std::ffi::c_char; 33] = unsafe {
    *::core::mem::transmute::<
        &[u8; 33],
        &[std::ffi::c_char; 33],
    >(b"luakit.uniq.registry.dom_element\0")
};

static mut dom_element_class: lua_class_t = lua_class_t {
    name: 0 as *const gchar,
    signals: 0 as *const signal_t as *mut signal_t,
    allocator: None,
    properties: 0 as *const lua_class_property_array_t
        as *mut lua_class_property_array_t,
    index_miss_property: None,
    newindex_miss_property: None,
};
#[inline]

unsafe extern "C" fn dom_element_new(mut L: *mut lua_State) -> *mut dom_element_t {
    let mut p = lua_newuserdata(
        L,
        ::core::mem::size_of::<dom_element_t>() as std::ffi::c_ulong,
    ) as *mut dom_element_t;
    memset(
        p as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        (::core::mem::size_of::<dom_element_t>() as std::ffi::c_ulong)
            .wrapping_mul(1 as std::ffi::c_int as std::ffi::c_ulong),
    );
    (*p).signals = signal_new();
    (*p).dom_events = signal_new();
    luaH_settype(L, &mut dom_element_class);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_setmetatable(L, -(2 as std::ffi::c_int));
    lua_setfenv(L, -(2 as std::ffi::c_int));
    lua_pushvalue(L, -(1 as std::ffi::c_int));
    luaH_class_emit_signal(
        L,
        &mut dom_element_class,
        b"new\0" as *const u8 as *const std::ffi::c_char,
        1 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    return p;
}
#[inline]

unsafe extern "C" fn luaH_dom_element_class_emit_signal(mut L: *mut lua_State) -> gint {
    return luaH_class_emit_signal(
        L,
        &mut dom_element_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, NULL_0 as *mut size_t),
        lua_gettop(L) - 1 as std::ffi::c_int,
        LUA_MULTRET,
    );
}
#[inline]

unsafe extern "C" fn luaH_dom_element_class_remove_signal(
    mut L: *mut lua_State,
) -> gint {
    luaH_class_remove_signal(
        L,
        &mut dom_element_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, NULL_0 as *mut size_t),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[inline]

unsafe extern "C" fn luaH_dom_element_class_add_signal(mut L: *mut lua_State) -> gint {
    luaH_class_add_signal(
        L,
        &mut dom_element_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, NULL_0 as *mut size_t),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_check_dom_element(
    mut L: *mut lua_State,
    mut udx: gint,
) -> *mut dom_element_t {
    let mut element = luaH_checkudata(L, udx, &mut dom_element_class)
        as *mut dom_element_t;
    if ((*element).element).is_null()
        || ({
            let mut __inst = (*element).element as *mut GTypeInstance;
            let mut __t = webkit_dom_element_get_type();
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
            b"DOM element no longer valid\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    return element;
}

unsafe extern "C" fn dom_element_collect_event_keys(
    mut key: gpointer,
    mut UNUSED_value: gpointer,
    mut keys: *mut GPtrArray,
) -> gboolean {
    g_ptr_array_add(keys, key);
    return FALSE;
}

unsafe extern "C" fn dom_element_unregister_webkit_event_listeners(
    mut element: *mut dom_element_t,
) {
    if !element.is_null() && !((*element).element).is_null()
        && !((*element).dom_events).is_null()
    {
        let mut target = g_type_check_instance_cast(
            (*element).element as *mut GTypeInstance,
            webkit_dom_event_target_get_type(),
        ) as *mut std::ffi::c_void as *mut WebKitDOMEventTarget;
        if !target.is_null() {
            let mut i: guint = 0;
            let mut keys = g_ptr_array_new();
            g_tree_foreach(
                (*element).dom_events,
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            gpointer,
                            gpointer,
                            *mut GPtrArray,
                        ) -> gboolean,
                    >,
                    GTraverseFunc,
                >(
                    Some(
                        dom_element_collect_event_keys
                            as unsafe extern "C" fn(
                                gpointer,
                                gpointer,
                                *mut GPtrArray,
                            ) -> gboolean,
                    ),
                ),
                keys as gpointer,
            );
            i = 0 as std::ffi::c_int as guint;
            while i < (*keys).len {
                let mut type_0 = *((*keys).pdata).offset(i as isize)
                    as *mut std::ffi::c_char;
                if if 0 != 0 {
                    ({
                        let __str: *const std::ffi::c_char = type_0;
                        let __suffix = b"::capture\0" as *const u8
                            as *const std::ffi::c_char;
                        let mut __result = FALSE;
                        if __str.is_null() || __suffix.is_null() {
                            __result = g_str_has_suffix(__str, __suffix);
                        } else {
                            let __str_len = strlen(
                                __str.offset(__str.is_null() as std::ffi::c_int as isize),
                            );
                            let __suffix_len = strlen(
                                __suffix
                                    .offset(__suffix.is_null() as std::ffi::c_int as isize),
                            );
                            if __str_len >= __suffix_len {
                                __result = (memcmp(
                                    __str
                                        .offset(__str_len as isize)
                                        .offset(-(__suffix_len as isize))
                                        as *const std::ffi::c_void,
                                    __suffix
                                        .offset(__suffix.is_null() as std::ffi::c_int as isize)
                                        as *const std::ffi::c_void,
                                    __suffix_len,
                                ) == 0 as std::ffi::c_int) as std::ffi::c_int;
                            }
                        }
                        __result
                    })
                } else {
                    g_str_has_suffix(
                        type_0,
                        b"::capture\0" as *const u8 as *const std::ffi::c_char,
                    )
                } != 0
                {
                    webkit_dom_event_target_remove_event_listener(
                        target,
                        type_0,
                        ::core::mem::transmute::<
                            Option::<
                                unsafe extern "C" fn(
                                    *mut WebKitDOMElement,
                                    *mut WebKitDOMEvent,
                                    *mut dom_element_t,
                                ) -> (),
                            >,
                            GCallback,
                        >(
                            Some(
                                event_listener_capture_cb
                                    as unsafe extern "C" fn(
                                        *mut WebKitDOMElement,
                                        *mut WebKitDOMEvent,
                                        *mut dom_element_t,
                                    ) -> (),
                            ),
                        ),
                        TRUE,
                    );
                } else {
                    webkit_dom_event_target_remove_event_listener(
                        target,
                        type_0,
                        ::core::mem::transmute::<
                            Option::<
                                unsafe extern "C" fn(
                                    *mut WebKitDOMElement,
                                    *mut WebKitDOMEvent,
                                    *mut dom_element_t,
                                ) -> (),
                            >,
                            GCallback,
                        >(
                            Some(
                                event_listener_bubble_cb
                                    as unsafe extern "C" fn(
                                        *mut WebKitDOMElement,
                                        *mut WebKitDOMEvent,
                                        *mut dom_element_t,
                                    ) -> (),
                            ),
                        ),
                        FALSE,
                    );
                }
                i = i.wrapping_add(1);
                i;
            }
            g_ptr_array_free(keys, FALSE);
        }
    }
}

unsafe extern "C" fn webkit_web_page_destroy_cb(
    mut element: *mut dom_element_t,
    mut node: *mut GObject,
) {
    let mut L = common.L;
    luaH_uniq_get_ptr(L, REG_KEY.as_ptr(), node as gpointer);
    luaH_object_emit_signal(
        L,
        -(1 as std::ffi::c_int),
        b"destroy\0" as *const u8 as *const std::ffi::c_char,
        0 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    dom_element_unregister_webkit_event_listeners(element);
    (*element).element = NULL_0 as *mut WebKitDOMElement;
    luaH_uniq_del_ptr(common.L, REG_KEY.as_ptr(), node as gpointer);
}

unsafe extern "C" fn luaH_dom_element_gc(mut L: *mut lua_State) -> gint {
    let mut element = luaH_checkudata(L, 1 as std::ffi::c_int, &mut dom_element_class)
        as *mut dom_element_t;
    if !element.is_null() {
        dom_element_unregister_webkit_event_listeners(element);
        if !((*element).dom_events).is_null() {
            signal_destroy((*element).dom_events);
        }
    }
    return luaH_object_gc(L);
}
#[no_mangle]

pub unsafe extern "C" fn luaH_dom_element_from_node(
    mut L: *mut lua_State,
    mut node: *mut WebKitDOMElement,
) -> gint {
    if node.is_null() {
        lua_pushnil(L);
        return 1 as std::ffi::c_int;
    }
    if luaH_uniq_get_ptr(L, REG_KEY.as_ptr(), node as gpointer) != 0 {
        return 1 as std::ffi::c_int;
    }
    let mut element = dom_element_new(L);
    (*element).element = node;
    luaH_uniq_add_ptr(L, REG_KEY.as_ptr(), node as gpointer, -(1 as std::ffi::c_int));
    g_object_weak_ref(
        g_type_check_instance_cast(
            node as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut dom_element_t, *mut GObject) -> ()>,
            GWeakNotify,
        >(
            Some(
                webkit_web_page_destroy_cb
                    as unsafe extern "C" fn(*mut dom_element_t, *mut GObject) -> (),
            ),
        ),
        element as gpointer,
    );
    return 1 as std::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn luaH_to_dom_element(
    mut L: *mut lua_State,
    mut idx: gint,
) -> *mut dom_element_t {
    return luaH_toudata(L, idx, &mut dom_element_class) as *mut dom_element_t;
}

unsafe extern "C" fn dom_element_selector(
    mut element: *mut dom_element_t,
) -> *mut std::ffi::c_char {
    let mut elem = g_type_check_instance_cast(
        (*element).element as *mut GTypeInstance,
        webkit_dom_node_get_type(),
    ) as *mut std::ffi::c_void as *mut WebKitDOMNode;
    let mut parent = 0 as *mut WebKitDOMNode;
    let mut parts = g_ptr_array_new_full(
        10 as std::ffi::c_int as guint,
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
    loop {
        parent = webkit_dom_node_get_parent_node(elem);
        if parent.is_null() {
            break;
        }
        let mut tag = webkit_dom_element_get_tag_name(
            g_type_check_instance_cast(
                elem as *mut GTypeInstance,
                webkit_dom_element_get_type(),
            ) as *mut std::ffi::c_void as *mut WebKitDOMElement,
        );
        if strcmp(tag, b"BODY\0" as *const u8 as *const std::ffi::c_char) == 0
            || strcmp(tag, b"HEAD\0" as *const u8 as *const std::ffi::c_char) == 0
        {
            g_ptr_array_add(parts, g_strdup_inline(tag) as gpointer);
            break;
        } else {
            let mut c = 1 as std::ffi::c_int;
            let mut e = g_type_check_instance_cast(
                elem as *mut GTypeInstance,
                webkit_dom_element_get_type(),
            ) as *mut std::ffi::c_void as *mut WebKitDOMElement;
            let mut ps = 0 as *mut WebKitDOMElement;
            loop {
                ps = webkit_dom_element_get_previous_element_sibling(e);
                if ps.is_null() {
                    break;
                }
                e = ps;
                c += 1;
                c;
            }
            g_ptr_array_add(
                parts,
                g_strdup_printf(
                    b"%s:nth-child(%d)\0" as *const u8 as *const std::ffi::c_char,
                    tag,
                    c,
                ) as gpointer,
            );
            elem = parent;
        }
    }
    let mut i = 0 as std::ffi::c_int as guint;
    let mut j = ((*parts).len).wrapping_sub(1 as std::ffi::c_int as guint);
    while i < j {
        let mut tmp = *((*parts).pdata).offset(i as isize) as *mut std::ffi::c_char;
        let ref mut fresh0 = *((*parts).pdata).offset(i as isize);
        *fresh0 = *((*parts).pdata).offset(j as isize);
        let ref mut fresh1 = *((*parts).pdata).offset(j as isize);
        *fresh1 = tmp as gpointer;
        i = i.wrapping_add(1);
        i;
        j = j.wrapping_sub(1);
        j;
    }
    g_ptr_array_add(parts, NULL_0 as *mut std::ffi::c_void);
    let mut sel = g_strjoinv(
        b" > \0" as *const u8 as *const std::ffi::c_char,
        (*parts).pdata as *mut *mut std::ffi::c_char,
    );
    g_ptr_array_free(parts, TRUE);
    return sel;
}
#[no_mangle]

pub unsafe extern "C" fn dom_element_js_ref(
    mut page: *mut page_t,
    mut element: *mut dom_element_t,
) -> *mut JSCValue {
    let mut sel = dom_element_selector(element);
    let mut frame = webkit_web_page_get_main_frame((*page).page);
    let mut world = extension.script_world;
    let mut ctx = webkit_frame_get_js_context_for_script_world(frame, world);
    let mut js_global = jsc_context_get_global_object(ctx);
    let mut js_doc = jsc_value_object_get_property(
        js_global,
        b"document\0" as *const u8 as *const std::ffi::c_char,
    );
    let mut ret = jsc_value_object_invoke_method(
        js_doc,
        b"querySelector\0" as *const u8 as *const std::ffi::c_char,
        G_TYPE_STRING as GType,
        sel,
        G_TYPE_NONE as GType,
    );
    g_object_unref(js_doc as gpointer);
    g_object_unref(js_global as gpointer);
    g_object_unref(ctx as gpointer);
    g_free(sel as gpointer);
    return ret;
}

unsafe extern "C" fn luaH_dom_element_query(mut L: *mut lua_State) -> gint {
    let mut element = luaH_check_dom_element(L, 1 as std::ffi::c_int);
    let mut elem = (*element).element;
    let mut query = luaL_checklstring(L, 2 as std::ffi::c_int, NULL_0 as *mut size_t);
    let mut error = NULL_0 as *mut GError;
    let mut nodes = webkit_dom_element_query_selector_all(elem, query, &mut error);
    if !error.is_null() {
        return luaL_error(
            L,
            b"query error: %s\0" as *const u8 as *const std::ffi::c_char,
            (*error).message,
        );
    }
    let mut n = webkit_dom_node_list_get_length(nodes);
    lua_createtable(L, n as std::ffi::c_int, 0 as std::ffi::c_int);
    let mut i = 0 as std::ffi::c_int as gulong;
    while i < n {
        let mut node = webkit_dom_node_list_item(nodes, i);
        luaH_dom_element_from_node(
            L,
            g_type_check_instance_cast(
                node as *mut GTypeInstance,
                webkit_dom_element_get_type(),
            ) as *mut std::ffi::c_void as *mut WebKitDOMElement,
        );
        lua_rawseti(
            L,
            3 as std::ffi::c_int,
            i.wrapping_add(1 as std::ffi::c_int as gulong) as std::ffi::c_int,
        );
        i = i.wrapping_add(1);
        i;
    }
    return 1 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_dom_element_append(mut L: *mut lua_State) -> gint {
    let mut parent = luaH_check_dom_element(L, 1 as std::ffi::c_int);
    let mut child = luaH_check_dom_element(L, 2 as std::ffi::c_int);
    let mut p = g_type_check_instance_cast(
        (*parent).element as *mut GTypeInstance,
        webkit_dom_node_get_type(),
    ) as *mut std::ffi::c_void as *mut WebKitDOMNode;
    let mut c = g_type_check_instance_cast(
        (*child).element as *mut GTypeInstance,
        webkit_dom_node_get_type(),
    ) as *mut std::ffi::c_void as *mut WebKitDOMNode;
    let mut error = NULL_0 as *mut GError;
    webkit_dom_node_append_child(p, c, &mut error);
    return if !error.is_null() {
        luaL_error(
            L,
            b"append element error: %s\0" as *const u8 as *const std::ffi::c_char,
            (*error).message,
        )
    } else {
        0 as std::ffi::c_int
    };
}

unsafe extern "C" fn luaH_dom_element_remove(mut L: *mut lua_State) -> gint {
    let mut element = luaH_checkudata(L, 1 as std::ffi::c_int, &mut dom_element_class)
        as *mut dom_element_t;
    if ({
        let mut __inst = (*element).element as *mut GTypeInstance;
        let mut __t = webkit_dom_element_get_type();
        let mut __r: gboolean = 0;
        if __inst.is_null() {
            __r = FALSE;
        } else if !((*__inst).g_class).is_null() && (*(*__inst).g_class).g_type == __t {
            __r = TRUE;
        } else {
            __r = g_type_check_instance_is_a(__inst, __t);
        }
        __r
    }) == 0
    {
        return 0 as std::ffi::c_int;
    }
    let mut error = NULL_0 as *mut GError;
    webkit_dom_element_remove((*element).element, &mut error);
    return if !error.is_null() {
        luaL_error(
            L,
            b"remove element error: %s\0" as *const u8 as *const std::ffi::c_char,
            (*error).message,
        )
    } else {
        0 as std::ffi::c_int
    };
}

unsafe extern "C" fn dom_element_get_left_and_top(
    mut elem: *mut WebKitDOMElement,
    mut l: *mut glong,
    mut t: *mut glong,
) {
    if elem.is_null() {
        *l = 0 as std::ffi::c_int as glong;
        *t = 0 as std::ffi::c_int as glong;
    } else {
        dom_element_get_left_and_top(webkit_dom_element_get_offset_parent(elem), l, t);
        *l = (*l as gdouble + webkit_dom_element_get_offset_left(elem)) as glong;
        *l -= webkit_dom_element_get_scroll_left(elem);
        *t = (*t as gdouble + webkit_dom_element_get_offset_top(elem)) as glong;
        *t -= webkit_dom_element_get_scroll_top(elem);
    };
}

unsafe extern "C" fn luaH_dom_element_rect_index(mut L: *mut lua_State) -> gint {
    let mut element = luaH_check_dom_element(L, LUA_GLOBALSINDEX - 1 as std::ffi::c_int);
    let mut prop = luaL_checklstring(L, 2 as std::ffi::c_int, NULL_0 as *mut size_t);
    let mut token = l_tokenize(prop);
    let mut elem = (*element).element;
    let mut left: glong = 0;
    let mut top: glong = 0;
    match token as std::ffi::c_uint {
        262 => {
            lua_pushinteger(L, webkit_dom_element_get_offset_width(elem) as lua_Integer);
            return 1 as std::ffi::c_int;
        }
        109 => {
            lua_pushinteger(
                L,
                webkit_dom_element_get_offset_height(elem) as lua_Integer,
            );
            return 1 as std::ffi::c_int;
        }
        136 | 242 => {
            dom_element_get_left_and_top(elem, &mut left, &mut top);
            lua_pushinteger(
                L,
                if token as std::ffi::c_uint
                    == L_TK_LEFT as std::ffi::c_int as std::ffi::c_uint
                {
                    left
                } else {
                    top
                },
            );
            return 1 as std::ffi::c_int;
        }
        _ => return 0 as std::ffi::c_int,
    };
}

unsafe extern "C" fn luaH_dom_element_push_rect_table(mut L: *mut lua_State) -> gint {
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
            luaH_dom_element_rect_index as unsafe extern "C" fn(*mut lua_State) -> gint,
        ),
        1 as std::ffi::c_int,
    );
    lua_rawset(L, -(3 as std::ffi::c_int));
    lua_setmetatable(L, -(2 as std::ffi::c_int));
    return 1 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_dom_element_attribute_index(mut L: *mut lua_State) -> gint {
    let mut element = luaH_check_dom_element(L, LUA_GLOBALSINDEX - 1 as std::ffi::c_int);
    let mut name = luaL_checklstring(L, 2 as std::ffi::c_int, NULL_0 as *mut size_t);
    let mut attr: *const gchar = webkit_dom_element_get_attribute(
        (*element).element,
        name,
    );
    lua_pushstring(L, attr);
    return 1 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_dom_element_attribute_newindex(mut L: *mut lua_State) -> gint {
    let mut element = luaH_check_dom_element(L, LUA_GLOBALSINDEX - 1 as std::ffi::c_int);
    let mut attr = luaL_checklstring(L, 2 as std::ffi::c_int, NULL_0 as *mut size_t);
    let mut value = luaL_checklstring(L, 3 as std::ffi::c_int, NULL_0 as *mut size_t);
    let mut error = NULL_0 as *mut GError;
    webkit_dom_element_set_attribute((*element).element, attr, value, &mut error);
    return if !error.is_null() {
        luaL_error(
            L,
            b"attribute error: %s\0" as *const u8 as *const std::ffi::c_char,
            (*error).message,
        )
    } else {
        0 as std::ffi::c_int
    };
}

unsafe extern "C" fn luaH_dom_element_push_attribute_table(
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
            luaH_dom_element_attribute_index
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
            luaH_dom_element_attribute_newindex
                as unsafe extern "C" fn(*mut lua_State) -> gint,
        ),
        1 as std::ffi::c_int,
    );
    lua_rawset(L, -(3 as std::ffi::c_int));
    lua_setmetatable(L, -(2 as std::ffi::c_int));
    return 1 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_dom_element_style_index(mut L: *mut lua_State) -> gint {
    let mut element = luaH_check_dom_element(L, LUA_GLOBALSINDEX - 1 as std::ffi::c_int);
    let mut document = webkit_dom_node_get_owner_document(
        g_type_check_instance_cast(
            (*element).element as *mut GTypeInstance,
            webkit_dom_node_get_type(),
        ) as *mut std::ffi::c_void as *mut WebKitDOMNode,
    );
    let mut window = webkit_dom_document_get_default_view(document);
    let mut style = webkit_dom_dom_window_get_computed_style(
        window,
        (*element).element,
        b"\0" as *const u8 as *const std::ffi::c_char,
    );
    let mut name = luaL_checklstring(L, 2 as std::ffi::c_int, NULL_0 as *mut size_t);
    let mut value: *const gchar = webkit_dom_css_style_declaration_get_property_value(
        style,
        name,
    );
    lua_pushstring(L, value);
    return 1 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_dom_element_push_style_table(mut L: *mut lua_State) -> gint {
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
            luaH_dom_element_style_index as unsafe extern "C" fn(*mut lua_State) -> gint,
        ),
        1 as std::ffi::c_int,
    );
    lua_rawset(L, -(3 as std::ffi::c_int));
    lua_setmetatable(L, -(2 as std::ffi::c_int));
    return 1 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_dom_element_click(mut L: *mut lua_State) -> gint {
    let mut element = luaH_check_dom_element(L, 1 as std::ffi::c_int);
    let mut elem = (*element).element;
    let mut doc = webkit_dom_node_get_owner_document(
        g_type_check_instance_cast(
            elem as *mut GTypeInstance,
            webkit_dom_node_get_type(),
        ) as *mut std::ffi::c_void as *mut WebKitDOMNode,
    );
    let mut target = g_type_check_instance_cast(
        (*element).element as *mut GTypeInstance,
        webkit_dom_event_target_get_type(),
    ) as *mut std::ffi::c_void as *mut WebKitDOMEventTarget;
    let mut error = NULL_0 as *mut GError;
    let mut event = webkit_dom_document_create_event(
        doc,
        b"MouseEvent\0" as *const u8 as *const std::ffi::c_char,
        &mut error,
    );
    if !error.is_null() {
        return luaL_error(
            L,
            b"create event error: %s\0" as *const u8 as *const std::ffi::c_char,
            (*error).message,
        );
    }
    webkit_dom_event_init_event(
        event,
        b"click\0" as *const u8 as *const std::ffi::c_char,
        TRUE,
        TRUE,
    );
    webkit_dom_event_target_dispatch_event(target, event, &mut error);
    if !error.is_null() {
        return luaL_error(
            L,
            b"dispatch event error: %s\0" as *const u8 as *const std::ffi::c_char,
            (*error).message,
        );
    }
    return 0 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_dom_element_focus(mut L: *mut lua_State) -> gint {
    let mut element = luaH_check_dom_element(L, 1 as std::ffi::c_int);
    webkit_dom_element_focus((*element).element);
    return 0 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_dom_element_submit(mut L: *mut lua_State) -> gint {
    let mut element = luaH_check_dom_element(L, 1 as std::ffi::c_int);
    webkit_dom_html_form_element_submit(
        g_type_check_instance_cast(
            (*element).element as *mut GTypeInstance,
            webkit_dom_html_form_element_get_type(),
        ) as *mut std::ffi::c_void as *mut WebKitDOMHTMLFormElement,
    );
    return 0 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_dom_element_emit_dom_event(
    mut L: *mut lua_State,
    mut event: *mut WebKitDOMEvent,
    mut oud: gint,
    mut name: *const gchar,
) -> gint {
    let mut nargs = 1 as std::ffi::c_int;
    let mut nret = 0 as std::ffi::c_int;
    let mut ret: gint = 0;
    let mut top: gint = 0;
    let mut bot = lua_gettop(L) - nargs + 1 as std::ffi::c_int;
    let mut oud_abs = luaH_absindex(L, oud);
    let mut obj = luaH_check_dom_element(L, oud);
    let mut origin = luaH_callerinfo(L);
    _log(
        LOG_LEVEL_debug,
        b"extension/clib/dom_element.c\0" as *const u8 as *const std::ffi::c_char,
        b"emit dom event \x1B[34m\"%s\"\x1B[0m on %p from \x1B[32m%s\x1B[0m (%d args, %d nret)\0"
            as *const u8 as *const std::ffi::c_char,
        name,
        obj,
        if !origin.is_null() {
            origin as *const gchar
        } else {
            b"<GTK>\0" as *const u8 as *const std::ffi::c_char
        },
        nargs,
        nret,
    );
    g_free(origin as gpointer);
    if obj.is_null() {
        return luaL_error(
            L,
            b"trying to emit dom event \x1B[34m\"%s\"\x1B[0m on non-object\0"
                as *const u8 as *const std::ffi::c_char,
            name,
        );
    }
    let mut sigfuncs = signal_lookup((*obj).dom_events, name);
    if !sigfuncs.is_null() {
        let mut nbfunc = (*sigfuncs).len;
        luaL_checkstack(
            L,
            (lua_gettop(L) as guint)
                .wrapping_add(nbfunc)
                .wrapping_add(nargs as guint)
                .wrapping_add(2 as std::ffi::c_int as guint) as std::ffi::c_int,
            b"too many signal handlers; need a new implementation!\0" as *const u8
                as *const std::ffi::c_char,
        );
        let mut i = 0 as std::ffi::c_int as guint;
        while i < nbfunc {
            luaH_object_push_item(L, oud_abs, *((*sigfuncs).pdata).offset(i as isize));
            i = i.wrapping_add(1);
            i;
        }
        let mut cancel = false_0;
        let mut i_0 = 0 as std::ffi::c_int as guint;
        while i_0 < nbfunc {
            lua_pushvalue(L, oud_abs);
            lua_pushvalue(
                L,
                (-nargs as guint)
                    .wrapping_sub(nbfunc)
                    .wrapping_sub(1 as std::ffi::c_int as guint)
                    .wrapping_add(i_0) as std::ffi::c_int,
            );
            lua_pushvalue(
                L,
                (-nargs as guint)
                    .wrapping_sub(nbfunc)
                    .wrapping_sub(1 as std::ffi::c_int as guint)
                    .wrapping_add(i_0) as std::ffi::c_int,
            );
            lua_remove(
                L,
                (-nargs as guint)
                    .wrapping_sub(nbfunc)
                    .wrapping_sub(2 as std::ffi::c_int as guint)
                    .wrapping_add(i_0) as std::ffi::c_int,
            );
            top = lua_gettop(L) - 2 as std::ffi::c_int - nargs;
            luaH_dofunction(L, nargs + 1 as std::ffi::c_int, LUA_MULTRET);
            ret = lua_gettop(L) - top;
            lua_settop(L, -ret - 1 as std::ffi::c_int);
            lua_pushvalue(
                L,
                (-nargs as guint)
                    .wrapping_sub(nbfunc)
                    .wrapping_add(1 as std::ffi::c_int as guint)
                    .wrapping_add(i_0) as std::ffi::c_int,
            );
            lua_pushlstring(
                L,
                b"prevent_default\0" as *const u8 as *const std::ffi::c_char,
                (::core::mem::size_of::<[std::ffi::c_char; 16]>() as std::ffi::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                    )
                    .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
            );
            lua_rawget(L, -(2 as std::ffi::c_int));
            if lua_toboolean(L, -(1 as std::ffi::c_int)) != 0 {
                webkit_dom_event_prevent_default(event);
            }
            lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
            lua_pushlstring(
                L,
                b"cancel\0" as *const u8 as *const std::ffi::c_char,
                (::core::mem::size_of::<[std::ffi::c_char; 7]>() as std::ffi::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                    )
                    .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
            );
            lua_rawget(L, -(2 as std::ffi::c_int));
            if lua_toboolean(L, -(1 as std::ffi::c_int)) != 0 {
                webkit_dom_event_stop_propagation(event);
                cancel = true_0;
            }
            lua_settop(L, -(2 as std::ffi::c_int) - 1 as std::ffi::c_int);
            if cancel != 0 {
                let mut i_1 = bot;
                while i_1 < top {
                    lua_remove(L, bot);
                    i_1 += 1;
                    i_1;
                }
                break;
            } else {
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        }
    }
    lua_settop(L, -nargs - 1 as std::ffi::c_int);
    return 0 as std::ffi::c_int;
}

unsafe extern "C" fn event_listener_cb(
    mut UNUSED_elem: *mut WebKitDOMElement,
    mut event: *mut WebKitDOMEvent,
    mut capture: gboolean,
    mut element: *mut dom_element_t,
) {
    let mut L = common.L;
    luaH_uniq_get_ptr(L, REG_KEY.as_ptr(), (*element).element as gpointer);
    lua_createtable(L, 0 as std::ffi::c_int, 1 as std::ffi::c_int);
    lua_pushlstring(
        L,
        b"target\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 7]>() as std::ffi::c_ulong)
            .wrapping_div(
                ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
            )
            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
    );
    let mut target = webkit_dom_event_get_src_element(event);
    luaH_dom_element_from_node(
        L,
        g_type_check_instance_cast(
            target as *mut GTypeInstance,
            webkit_dom_element_get_type(),
        ) as *mut std::ffi::c_void as *mut WebKitDOMElement,
    );
    lua_rawset(L, -(3 as std::ffi::c_int));
    lua_pushlstring(
        L,
        b"type\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 5]>() as std::ffi::c_ulong)
            .wrapping_div(
                ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
            )
            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
    );
    let mut type_0 = webkit_dom_event_get_event_type(event);
    lua_pushstring(L, type_0);
    lua_rawset(L, -(3 as std::ffi::c_int));
    let mut staged_type = g_strjoin(
        b"::\0" as *const u8 as *const std::ffi::c_char,
        type_0,
        if capture != 0 {
            b"capture\0" as *const u8 as *const std::ffi::c_char
        } else {
            b"bubble\0" as *const u8 as *const std::ffi::c_char
        },
        NULL_0 as *mut std::ffi::c_void,
    );
    lua_pushlstring(
        L,
        b"phase\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 6]>() as std::ffi::c_ulong)
            .wrapping_div(
                ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
            )
            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
    );
    let mut phase = webkit_dom_event_get_event_phase(event);
    lua_pushinteger(L, phase as lua_Integer);
    lua_rawset(L, -(3 as std::ffi::c_int));
    if ({
        let mut __inst = event as *mut GTypeInstance;
        let mut __t = webkit_dom_mouse_event_get_type();
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
            b"button\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 7]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        let mut button = webkit_dom_mouse_event_get_button(
            g_type_check_instance_cast(
                event as *mut GTypeInstance,
                webkit_dom_mouse_event_get_type(),
            ) as *mut std::ffi::c_void as *mut WebKitDOMMouseEvent,
        );
        lua_pushinteger(L, button as lua_Integer);
        lua_rawset(L, -(3 as std::ffi::c_int));
    }
    if ({
        let mut __inst = event as *mut GTypeInstance;
        let mut __t = webkit_dom_keyboard_event_get_type();
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
            b"key\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 4]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        let mut key = webkit_dom_keyboard_event_get_key_identifier(
            g_type_check_instance_cast(
                event as *mut GTypeInstance,
                webkit_dom_keyboard_event_get_type(),
            ) as *mut std::ffi::c_void as *mut WebKitDOMKeyboardEvent,
        );
        lua_pushstring(L, key);
        lua_rawset(L, -(3 as std::ffi::c_int));
        lua_pushlstring(
            L,
            b"code\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 5]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        let mut code = webkit_dom_ui_event_get_char_code(
            g_type_check_instance_cast(
                event as *mut GTypeInstance,
                webkit_dom_ui_event_get_type(),
            ) as *mut std::ffi::c_void as *mut WebKitDOMUIEvent,
        );
        lua_pushinteger(L, code);
        lua_rawset(L, -(3 as std::ffi::c_int));
        lua_pushlstring(
            L,
            b"ctrl_key\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 9]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        let mut ctrl = webkit_dom_keyboard_event_get_ctrl_key(
            g_type_check_instance_cast(
                event as *mut GTypeInstance,
                webkit_dom_keyboard_event_get_type(),
            ) as *mut std::ffi::c_void as *mut WebKitDOMKeyboardEvent,
        );
        lua_pushboolean(L, ctrl);
        lua_rawset(L, -(3 as std::ffi::c_int));
        lua_pushlstring(
            L,
            b"alt_key\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 8]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        let mut alt = webkit_dom_keyboard_event_get_alt_key(
            g_type_check_instance_cast(
                event as *mut GTypeInstance,
                webkit_dom_keyboard_event_get_type(),
            ) as *mut std::ffi::c_void as *mut WebKitDOMKeyboardEvent,
        );
        lua_pushboolean(L, alt);
        lua_rawset(L, -(3 as std::ffi::c_int));
        lua_pushlstring(
            L,
            b"shift_key\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 10]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        let mut shift = webkit_dom_keyboard_event_get_shift_key(
            g_type_check_instance_cast(
                event as *mut GTypeInstance,
                webkit_dom_keyboard_event_get_type(),
            ) as *mut std::ffi::c_void as *mut WebKitDOMKeyboardEvent,
        );
        lua_pushboolean(L, shift);
        lua_rawset(L, -(3 as std::ffi::c_int));
        lua_pushlstring(
            L,
            b"meta_key\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 9]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        let mut meta = webkit_dom_keyboard_event_get_meta_key(
            g_type_check_instance_cast(
                event as *mut GTypeInstance,
                webkit_dom_keyboard_event_get_type(),
            ) as *mut std::ffi::c_void as *mut WebKitDOMKeyboardEvent,
        );
        lua_pushboolean(L, meta);
        lua_rawset(L, -(3 as std::ffi::c_int));
    }
    luaH_dom_element_emit_dom_event(L, event, -(2 as std::ffi::c_int), staged_type);
    g_free(staged_type as gpointer);
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
}

unsafe extern "C" fn event_listener_capture_cb(
    mut elem: *mut WebKitDOMElement,
    mut event: *mut WebKitDOMEvent,
    mut element: *mut dom_element_t,
) {
    return event_listener_cb(elem, event, TRUE, element);
}

unsafe extern "C" fn event_listener_bubble_cb(
    mut elem: *mut WebKitDOMElement,
    mut event: *mut WebKitDOMEvent,
    mut element: *mut dom_element_t,
) {
    return event_listener_cb(elem, event, FALSE, element);
}
#[no_mangle]

pub unsafe extern "C" fn luaH_dom_element_add_dom_event(
    mut L: *mut lua_State,
    mut oud: gint,
    mut name: *const gchar,
    mut ud: gint,
) {
    if !(lua_type(L, ud) == LUA_TFUNCTION) {
        luaL_typerror(L, ud, b"function\0" as *const u8 as *const std::ffi::c_char);
    }
    let mut obj = luaH_check_dom_element(L, oud);
    let mut origin = luaH_callerinfo(L);
    _log(
        LOG_LEVEL_debug,
        b"extension/clib/dom_element.c\0" as *const u8 as *const std::ffi::c_char,
        b"add dom event \x1B[34m\"%s\"\x1B[0m on %p from \x1B[32m%s\x1B[0m\0"
            as *const u8 as *const std::ffi::c_char,
        name,
        obj,
        origin,
    );
    g_free(origin as gpointer);
    signal_add((*obj).dom_events, name, luaH_object_ref_item(L, oud, ud));
}
#[no_mangle]

pub unsafe extern "C" fn luaH_dom_element_remove_dom_event(
    mut L: *mut lua_State,
    mut oud: gint,
    mut name: *const gchar,
    mut ud: gint,
) {
    if !(lua_type(L, ud) == LUA_TFUNCTION) {
        luaL_typerror(L, ud, b"function\0" as *const u8 as *const std::ffi::c_char);
    }
    let mut obj = luaH_check_dom_element(L, oud);
    let mut ref_0 = lua_topointer(L, ud) as gpointer;
    let mut origin = luaH_callerinfo(L);
    _log(
        LOG_LEVEL_debug,
        b"extension/clib/dom_element.c\0" as *const u8 as *const std::ffi::c_char,
        b"remove dom event \x1B[34m\"%s\"\x1B[0m on %p from \x1B[32m%s\x1B[0m\0"
            as *const u8 as *const std::ffi::c_char,
        name,
        obj,
        origin,
    );
    g_free(origin as gpointer);
    signal_remove((*obj).dom_events, name, ref_0);
    luaH_object_unref_item(L, oud, ref_0);
    lua_remove(L, ud);
}

unsafe extern "C" fn luaH_dom_element_add_event_listener(mut L: *mut lua_State) -> gint {
    let mut element = luaH_check_dom_element(L, 1 as std::ffi::c_int);
    let mut type_0 = luaL_checklstring(L, 2 as std::ffi::c_int, NULL_0 as *mut size_t);
    let mut capture = lua_toboolean(L, 3 as std::ffi::c_int);
    if !(lua_type(L, 4 as std::ffi::c_int) == LUA_TFUNCTION) {
        luaL_typerror(
            L,
            4 as std::ffi::c_int,
            b"function\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    let mut ret = true_0;
    let mut target = g_type_check_instance_cast(
        (*element).element as *mut GTypeInstance,
        webkit_dom_event_target_get_type(),
    ) as *mut std::ffi::c_void as *mut WebKitDOMEventTarget;
    let mut staged_type = g_strjoin(
        b"::\0" as *const u8 as *const std::ffi::c_char,
        type_0,
        if capture != 0 {
            b"capture\0" as *const u8 as *const std::ffi::c_char
        } else {
            b"bubble\0" as *const u8 as *const std::ffi::c_char
        },
        NULL_0 as *mut std::ffi::c_void,
    );
    let mut signals = signal_lookup((*element).dom_events, staged_type);
    if signals.is_null()
        || !signals.is_null() && (*signals).len == 0 as std::ffi::c_int as guint
    {
        if capture != 0 {
            ret = webkit_dom_event_target_add_event_listener(
                target,
                type_0,
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut WebKitDOMElement,
                            *mut WebKitDOMEvent,
                            *mut dom_element_t,
                        ) -> (),
                    >,
                    GCallback,
                >(
                    Some(
                        event_listener_capture_cb
                            as unsafe extern "C" fn(
                                *mut WebKitDOMElement,
                                *mut WebKitDOMEvent,
                                *mut dom_element_t,
                            ) -> (),
                    ),
                ),
                capture,
                element as gpointer,
            );
        } else {
            ret = webkit_dom_event_target_add_event_listener(
                target,
                type_0,
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut WebKitDOMElement,
                            *mut WebKitDOMEvent,
                            *mut dom_element_t,
                        ) -> (),
                    >,
                    GCallback,
                >(
                    Some(
                        event_listener_bubble_cb
                            as unsafe extern "C" fn(
                                *mut WebKitDOMElement,
                                *mut WebKitDOMEvent,
                                *mut dom_element_t,
                            ) -> (),
                    ),
                ),
                capture,
                element as gpointer,
            );
        }
    }
    luaH_dom_element_add_dom_event(
        L,
        1 as std::ffi::c_int,
        staged_type,
        4 as std::ffi::c_int,
    );
    g_free(staged_type as gpointer);
    lua_settop(L, -(3 as std::ffi::c_int) - 1 as std::ffi::c_int);
    lua_pushboolean(L, ret);
    return 1 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_dom_element_remove_event_listener(
    mut L: *mut lua_State,
) -> gint {
    let mut element = luaH_check_dom_element(L, 1 as std::ffi::c_int);
    let mut type_0 = luaL_checklstring(L, 2 as std::ffi::c_int, NULL_0 as *mut size_t);
    let mut capture = lua_toboolean(L, 3 as std::ffi::c_int);
    if !(lua_type(L, 4 as std::ffi::c_int) == LUA_TFUNCTION) {
        luaL_typerror(
            L,
            4 as std::ffi::c_int,
            b"function\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    let mut ret = true_0;
    let mut staged_type = g_strjoin(
        b"::\0" as *const u8 as *const std::ffi::c_char,
        type_0,
        if capture != 0 {
            b"capture\0" as *const u8 as *const std::ffi::c_char
        } else {
            b"bubble\0" as *const u8 as *const std::ffi::c_char
        },
        NULL_0 as *mut std::ffi::c_void,
    );
    luaH_dom_element_remove_dom_event(
        L,
        1 as std::ffi::c_int,
        staged_type,
        4 as std::ffi::c_int,
    );
    let mut signals = signal_lookup((*element).dom_events, staged_type);
    g_free(staged_type as gpointer);
    if signals.is_null()
        || !signals.is_null() && (*signals).len == 0 as std::ffi::c_int as guint
    {
        let mut target = g_type_check_instance_cast(
            (*element).element as *mut GTypeInstance,
            webkit_dom_event_target_get_type(),
        ) as *mut std::ffi::c_void as *mut WebKitDOMEventTarget;
        if capture != 0 {
            ret = webkit_dom_event_target_remove_event_listener(
                target,
                type_0,
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut WebKitDOMElement,
                            *mut WebKitDOMEvent,
                            *mut dom_element_t,
                        ) -> (),
                    >,
                    GCallback,
                >(
                    Some(
                        event_listener_capture_cb
                            as unsafe extern "C" fn(
                                *mut WebKitDOMElement,
                                *mut WebKitDOMEvent,
                                *mut dom_element_t,
                            ) -> (),
                    ),
                ),
                capture,
            );
        } else {
            ret = webkit_dom_event_target_remove_event_listener(
                target,
                type_0,
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut WebKitDOMElement,
                            *mut WebKitDOMEvent,
                            *mut dom_element_t,
                        ) -> (),
                    >,
                    GCallback,
                >(
                    Some(
                        event_listener_bubble_cb
                            as unsafe extern "C" fn(
                                *mut WebKitDOMElement,
                                *mut WebKitDOMEvent,
                                *mut dom_element_t,
                            ) -> (),
                    ),
                ),
                capture,
            );
        }
    }
    lua_pushboolean(L, ret);
    return 1 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_dom_element_client_rects(mut L: *mut lua_State) -> gint {
    let mut element = luaH_check_dom_element(L, 1 as std::ffi::c_int);
    let mut rects = webkit_dom_element_get_client_rects((*element).element);
    let mut num_rects = webkit_dom_client_rect_list_get_length(rects) as std::ffi::c_int;
    lua_createtable(L, num_rects, 0 as std::ffi::c_int);
    let mut i = 0 as std::ffi::c_int;
    while i < num_rects {
        let mut rect = webkit_dom_client_rect_list_item(rects, i as gulong);
        lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
        lua_pushnumber(L, webkit_dom_client_rect_get_top(rect) as lua_Number);
        lua_setfield(
            L,
            -(2 as std::ffi::c_int),
            b"top\0" as *const u8 as *const std::ffi::c_char,
        );
        lua_pushnumber(L, webkit_dom_client_rect_get_right(rect) as lua_Number);
        lua_setfield(
            L,
            -(2 as std::ffi::c_int),
            b"right\0" as *const u8 as *const std::ffi::c_char,
        );
        lua_pushnumber(L, webkit_dom_client_rect_get_bottom(rect) as lua_Number);
        lua_setfield(
            L,
            -(2 as std::ffi::c_int),
            b"bottom\0" as *const u8 as *const std::ffi::c_char,
        );
        lua_pushnumber(L, webkit_dom_client_rect_get_left(rect) as lua_Number);
        lua_setfield(
            L,
            -(2 as std::ffi::c_int),
            b"left\0" as *const u8 as *const std::ffi::c_char,
        );
        lua_pushnumber(L, webkit_dom_client_rect_get_width(rect) as lua_Number);
        lua_setfield(
            L,
            -(2 as std::ffi::c_int),
            b"width\0" as *const u8 as *const std::ffi::c_char,
        );
        lua_pushnumber(L, webkit_dom_client_rect_get_height(rect) as lua_Number);
        lua_setfield(
            L,
            -(2 as std::ffi::c_int),
            b"height\0" as *const u8 as *const std::ffi::c_char,
        );
        lua_rawseti(L, -(2 as std::ffi::c_int), i + 1 as std::ffi::c_int);
        i += 1;
        i;
    }
    return 1 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_dom_element_push_src(mut L: *mut lua_State) -> gint {
    let mut element = luaH_check_dom_element(L, 1 as std::ffi::c_int);
    if ({
        let mut __inst = (*element).element as *mut GTypeInstance;
        let mut __t = webkit_dom_html_input_element_get_type();
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
        lua_pushstring(
            L,
            webkit_dom_html_input_element_get_src(
                g_type_check_instance_cast(
                    (*element).element as *mut GTypeInstance,
                    webkit_dom_html_input_element_get_type(),
                ) as *mut std::ffi::c_void as *mut WebKitDOMHTMLInputElement,
            ),
        );
        return 1 as std::ffi::c_int;
    }
    if ({
        let mut __inst = (*element).element as *mut GTypeInstance;
        let mut __t = webkit_dom_html_frame_element_get_type();
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
        lua_pushstring(
            L,
            webkit_dom_html_frame_element_get_src(
                g_type_check_instance_cast(
                    (*element).element as *mut GTypeInstance,
                    webkit_dom_html_frame_element_get_type(),
                ) as *mut std::ffi::c_void as *mut WebKitDOMHTMLFrameElement,
            ),
        );
        return 1 as std::ffi::c_int;
    }
    if ({
        let mut __inst = (*element).element as *mut GTypeInstance;
        let mut __t = webkit_dom_html_iframe_element_get_type();
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
        lua_pushstring(
            L,
            webkit_dom_html_iframe_element_get_src(
                g_type_check_instance_cast(
                    (*element).element as *mut GTypeInstance,
                    webkit_dom_html_iframe_element_get_type(),
                ) as *mut std::ffi::c_void as *mut WebKitDOMHTMLIFrameElement,
            ),
        );
        return 1 as std::ffi::c_int;
    }
    if ({
        let mut __inst = (*element).element as *mut GTypeInstance;
        let mut __t = webkit_dom_html_embed_element_get_type();
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
        lua_pushstring(
            L,
            webkit_dom_html_embed_element_get_src(
                g_type_check_instance_cast(
                    (*element).element as *mut GTypeInstance,
                    webkit_dom_html_embed_element_get_type(),
                ) as *mut std::ffi::c_void as *mut WebKitDOMHTMLEmbedElement,
            ),
        );
        return 1 as std::ffi::c_int;
    }
    if ({
        let mut __inst = (*element).element as *mut GTypeInstance;
        let mut __t = webkit_dom_html_image_element_get_type();
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
        lua_pushstring(
            L,
            webkit_dom_html_image_element_get_src(
                g_type_check_instance_cast(
                    (*element).element as *mut GTypeInstance,
                    webkit_dom_html_image_element_get_type(),
                ) as *mut std::ffi::c_void as *mut WebKitDOMHTMLImageElement,
            ),
        );
        return 1 as std::ffi::c_int;
    }
    if ({
        let mut __inst = (*element).element as *mut GTypeInstance;
        let mut __t = webkit_dom_html_script_element_get_type();
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
        lua_pushstring(
            L,
            webkit_dom_html_script_element_get_src(
                g_type_check_instance_cast(
                    (*element).element as *mut GTypeInstance,
                    webkit_dom_html_script_element_get_type(),
                ) as *mut std::ffi::c_void as *mut WebKitDOMHTMLScriptElement,
            ),
        );
        return 1 as std::ffi::c_int;
    }
    return 0 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_dom_element_push_href(mut L: *mut lua_State) -> gint {
    let mut element = luaH_check_dom_element(L, 1 as std::ffi::c_int);
    if ({
        let mut __inst = (*element).element as *mut GTypeInstance;
        let mut __t = webkit_dom_html_anchor_element_get_type();
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
        lua_pushstring(
            L,
            webkit_dom_html_anchor_element_get_href(
                g_type_check_instance_cast(
                    (*element).element as *mut GTypeInstance,
                    webkit_dom_html_anchor_element_get_type(),
                ) as *mut std::ffi::c_void as *mut WebKitDOMHTMLAnchorElement,
            ),
        );
        return 1 as std::ffi::c_int;
    }
    if ({
        let mut __inst = (*element).element as *mut GTypeInstance;
        let mut __t = webkit_dom_html_area_element_get_type();
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
        lua_pushstring(
            L,
            webkit_dom_html_area_element_get_href(
                g_type_check_instance_cast(
                    (*element).element as *mut GTypeInstance,
                    webkit_dom_html_area_element_get_type(),
                ) as *mut std::ffi::c_void as *mut WebKitDOMHTMLAreaElement,
            ),
        );
        return 1 as std::ffi::c_int;
    }
    if ({
        let mut __inst = (*element).element as *mut GTypeInstance;
        let mut __t = webkit_dom_html_link_element_get_type();
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
        lua_pushstring(
            L,
            webkit_dom_html_link_element_get_href(
                g_type_check_instance_cast(
                    (*element).element as *mut GTypeInstance,
                    webkit_dom_html_link_element_get_type(),
                ) as *mut std::ffi::c_void as *mut WebKitDOMHTMLLinkElement,
            ),
        );
        return 1 as std::ffi::c_int;
    }
    if ({
        let mut __inst = (*element).element as *mut GTypeInstance;
        let mut __t = webkit_dom_style_sheet_get_type();
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
        lua_pushstring(
            L,
            webkit_dom_style_sheet_get_href(
                g_type_check_instance_cast(
                    (*element).element as *mut GTypeInstance,
                    webkit_dom_style_sheet_get_type(),
                ) as *mut std::ffi::c_void as *mut WebKitDOMStyleSheet,
            ),
        );
        return 1 as std::ffi::c_int;
    }
    return 0 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_dom_element_push_value(mut L: *mut lua_State) -> gint {
    let mut element = luaH_check_dom_element(L, 1 as std::ffi::c_int);
    if ({
        let mut __inst = (*element).element as *mut GTypeInstance;
        let mut __t = webkit_dom_html_text_area_element_get_type();
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
        lua_pushstring(
            L,
            webkit_dom_html_text_area_element_get_value(
                g_type_check_instance_cast(
                    (*element).element as *mut GTypeInstance,
                    webkit_dom_html_text_area_element_get_type(),
                ) as *mut std::ffi::c_void as *mut WebKitDOMHTMLTextAreaElement,
            ),
        );
        return 1 as std::ffi::c_int;
    }
    if ({
        let mut __inst = (*element).element as *mut GTypeInstance;
        let mut __t = webkit_dom_html_input_element_get_type();
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
        lua_pushstring(
            L,
            webkit_dom_html_input_element_get_value(
                g_type_check_instance_cast(
                    (*element).element as *mut GTypeInstance,
                    webkit_dom_html_input_element_get_type(),
                ) as *mut std::ffi::c_void as *mut WebKitDOMHTMLInputElement,
            ),
        );
        return 1 as std::ffi::c_int;
    }
    if ({
        let mut __inst = (*element).element as *mut GTypeInstance;
        let mut __t = webkit_dom_html_option_element_get_type();
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
        lua_pushstring(
            L,
            webkit_dom_html_option_element_get_value(
                g_type_check_instance_cast(
                    (*element).element as *mut GTypeInstance,
                    webkit_dom_html_option_element_get_type(),
                ) as *mut std::ffi::c_void as *mut WebKitDOMHTMLOptionElement,
            ),
        );
        return 1 as std::ffi::c_int;
    }
    if ({
        let mut __inst = (*element).element as *mut GTypeInstance;
        let mut __t = webkit_dom_html_param_element_get_type();
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
        lua_pushstring(
            L,
            webkit_dom_html_param_element_get_value(
                g_type_check_instance_cast(
                    (*element).element as *mut GTypeInstance,
                    webkit_dom_html_param_element_get_type(),
                ) as *mut std::ffi::c_void as *mut WebKitDOMHTMLParamElement,
            ),
        );
        return 1 as std::ffi::c_int;
    }
    if ({
        let mut __inst = (*element).element as *mut GTypeInstance;
        let mut __t = webkit_dom_html_li_element_get_type();
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
        lua_pushinteger(
            L,
            webkit_dom_html_li_element_get_value(
                g_type_check_instance_cast(
                    (*element).element as *mut GTypeInstance,
                    webkit_dom_html_li_element_get_type(),
                ) as *mut std::ffi::c_void as *mut WebKitDOMHTMLLIElement,
            ),
        );
        return 1 as std::ffi::c_int;
    }
    if ({
        let mut __inst = (*element).element as *mut GTypeInstance;
        let mut __t = webkit_dom_html_button_element_get_type();
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
        lua_pushstring(
            L,
            webkit_dom_html_button_element_get_value(
                g_type_check_instance_cast(
                    (*element).element as *mut GTypeInstance,
                    webkit_dom_html_button_element_get_type(),
                ) as *mut std::ffi::c_void as *mut WebKitDOMHTMLButtonElement,
            ),
        );
        return 1 as std::ffi::c_int;
    }
    if ({
        let mut __inst = (*element).element as *mut GTypeInstance;
        let mut __t = webkit_dom_html_select_element_get_type();
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
        lua_pushstring(
            L,
            webkit_dom_html_select_element_get_value(
                g_type_check_instance_cast(
                    (*element).element as *mut GTypeInstance,
                    webkit_dom_html_select_element_get_type(),
                ) as *mut std::ffi::c_void as *mut WebKitDOMHTMLSelectElement,
            ),
        );
        return 1 as std::ffi::c_int;
    }
    return 0 as std::ffi::c_int;
}

unsafe extern "C" fn dom_html_element_set_value(
    mut L: *mut lua_State,
    mut element: *mut WebKitDOMHTMLElement,
) -> gint {
    if ({
        let mut __inst = element as *mut GTypeInstance;
        let mut __t = webkit_dom_html_text_area_element_get_type();
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
        webkit_dom_html_text_area_element_set_value(
            g_type_check_instance_cast(
                element as *mut GTypeInstance,
                webkit_dom_html_text_area_element_get_type(),
            ) as *mut std::ffi::c_void as *mut WebKitDOMHTMLTextAreaElement,
            luaL_checklstring(L, 3 as std::ffi::c_int, NULL_0 as *mut size_t),
        );
        return 1 as std::ffi::c_int;
    }
    if ({
        let mut __inst = element as *mut GTypeInstance;
        let mut __t = webkit_dom_html_input_element_get_type();
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
        webkit_dom_html_input_element_set_value(
            g_type_check_instance_cast(
                element as *mut GTypeInstance,
                webkit_dom_html_input_element_get_type(),
            ) as *mut std::ffi::c_void as *mut WebKitDOMHTMLInputElement,
            luaL_checklstring(L, 3 as std::ffi::c_int, NULL_0 as *mut size_t),
        );
        return 1 as std::ffi::c_int;
    }
    if ({
        let mut __inst = element as *mut GTypeInstance;
        let mut __t = webkit_dom_html_option_element_get_type();
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
        webkit_dom_html_option_element_set_value(
            g_type_check_instance_cast(
                element as *mut GTypeInstance,
                webkit_dom_html_option_element_get_type(),
            ) as *mut std::ffi::c_void as *mut WebKitDOMHTMLOptionElement,
            luaL_checklstring(L, 3 as std::ffi::c_int, NULL_0 as *mut size_t),
        );
        return 1 as std::ffi::c_int;
    }
    if ({
        let mut __inst = element as *mut GTypeInstance;
        let mut __t = webkit_dom_html_param_element_get_type();
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
        webkit_dom_html_param_element_set_value(
            g_type_check_instance_cast(
                element as *mut GTypeInstance,
                webkit_dom_html_param_element_get_type(),
            ) as *mut std::ffi::c_void as *mut WebKitDOMHTMLParamElement,
            luaL_checklstring(L, 3 as std::ffi::c_int, NULL_0 as *mut size_t),
        );
        return 1 as std::ffi::c_int;
    }
    if ({
        let mut __inst = element as *mut GTypeInstance;
        let mut __t = webkit_dom_html_li_element_get_type();
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
        webkit_dom_html_li_element_set_value(
            g_type_check_instance_cast(
                element as *mut GTypeInstance,
                webkit_dom_html_li_element_get_type(),
            ) as *mut std::ffi::c_void as *mut WebKitDOMHTMLLIElement,
            luaL_checkinteger(L, 3 as std::ffi::c_int),
        );
        return 1 as std::ffi::c_int;
    }
    if ({
        let mut __inst = element as *mut GTypeInstance;
        let mut __t = webkit_dom_html_button_element_get_type();
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
        webkit_dom_html_button_element_set_value(
            g_type_check_instance_cast(
                element as *mut GTypeInstance,
                webkit_dom_html_button_element_get_type(),
            ) as *mut std::ffi::c_void as *mut WebKitDOMHTMLButtonElement,
            luaL_checklstring(L, 3 as std::ffi::c_int, NULL_0 as *mut size_t),
        );
        return 1 as std::ffi::c_int;
    }
    if ({
        let mut __inst = element as *mut GTypeInstance;
        let mut __t = webkit_dom_html_select_element_get_type();
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
        webkit_dom_html_select_element_set_value(
            g_type_check_instance_cast(
                element as *mut GTypeInstance,
                webkit_dom_html_select_element_get_type(),
            ) as *mut std::ffi::c_void as *mut WebKitDOMHTMLSelectElement,
            luaL_checklstring(L, 3 as std::ffi::c_int, NULL_0 as *mut size_t),
        );
        return 1 as std::ffi::c_int;
    }
    return 0 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_dom_element_push_parent(mut L: *mut lua_State) -> gint {
    let mut element = luaH_check_dom_element(L, 1 as std::ffi::c_int);
    let mut parent = webkit_dom_node_get_parent_node(
        g_type_check_instance_cast(
            (*element).element as *mut GTypeInstance,
            webkit_dom_node_get_type(),
        ) as *mut std::ffi::c_void as *mut WebKitDOMNode,
    );
    return luaH_dom_element_from_node(
        L,
        g_type_check_instance_cast(
            parent as *mut GTypeInstance,
            webkit_dom_element_get_type(),
        ) as *mut std::ffi::c_void as *mut WebKitDOMElement,
    );
}

unsafe extern "C" fn luaH_dom_element_push_first_child(mut L: *mut lua_State) -> gint {
    let mut element = luaH_check_dom_element(L, 1 as std::ffi::c_int);
    let mut elem = (*element).element;
    let mut child = webkit_dom_element_get_first_element_child(elem);
    return luaH_dom_element_from_node(L, child);
}

unsafe extern "C" fn luaH_dom_element_push_last_child(mut L: *mut lua_State) -> gint {
    let mut element = luaH_check_dom_element(L, 1 as std::ffi::c_int);
    let mut elem = (*element).element;
    let mut child = webkit_dom_element_get_last_element_child(elem);
    return luaH_dom_element_from_node(L, child);
}

unsafe extern "C" fn luaH_dom_element_push_prev_sibling(mut L: *mut lua_State) -> gint {
    let mut element = luaH_check_dom_element(L, 1 as std::ffi::c_int);
    let mut elem = (*element).element;
    let mut child = webkit_dom_element_get_previous_element_sibling(elem);
    return luaH_dom_element_from_node(L, child);
}

unsafe extern "C" fn luaH_dom_element_push_next_sibling(mut L: *mut lua_State) -> gint {
    let mut element = luaH_check_dom_element(L, 1 as std::ffi::c_int);
    let mut elem = (*element).element;
    let mut child = webkit_dom_element_get_next_element_sibling(elem);
    return luaH_dom_element_from_node(L, child);
}

unsafe extern "C" fn luaH_dom_element_push_document(mut L: *mut lua_State) -> gint {
    let mut element = luaH_check_dom_element(L, 1 as std::ffi::c_int);
    let mut doc = 0 as *mut WebKitDOMDocument;
    if ({
        let mut __inst = (*element).element as *mut GTypeInstance;
        let mut __t = webkit_dom_html_frame_element_get_type();
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
        doc = webkit_dom_html_frame_element_get_content_document(
            g_type_check_instance_cast(
                (*element).element as *mut GTypeInstance,
                webkit_dom_html_frame_element_get_type(),
            ) as *mut std::ffi::c_void as *mut WebKitDOMHTMLFrameElement,
        );
    } else if ({
        let mut __inst = (*element).element as *mut GTypeInstance;
        let mut __t = webkit_dom_html_iframe_element_get_type();
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
        doc = webkit_dom_html_iframe_element_get_content_document(
            g_type_check_instance_cast(
                (*element).element as *mut GTypeInstance,
                webkit_dom_html_iframe_element_get_type(),
            ) as *mut std::ffi::c_void as *mut WebKitDOMHTMLIFrameElement,
        );
    } else {
        doc = webkit_dom_node_get_owner_document(
            g_type_check_instance_cast(
                (*element).element as *mut GTypeInstance,
                webkit_dom_node_get_type(),
            ) as *mut std::ffi::c_void as *mut WebKitDOMNode,
        );
    }
    return luaH_dom_document_from_webkit_dom_document(L, doc);
}

unsafe extern "C" fn luaH_dom_element_push_owner_document(
    mut L: *mut lua_State,
) -> gint {
    let mut element = luaH_check_dom_element(L, 1 as std::ffi::c_int);
    let mut doc = webkit_dom_node_get_owner_document(
        g_type_check_instance_cast(
            (*element).element as *mut GTypeInstance,
            webkit_dom_node_get_type(),
        ) as *mut std::ffi::c_void as *mut WebKitDOMNode,
    );
    return luaH_dom_document_from_webkit_dom_document(L, doc);
}

unsafe extern "C" fn luaH_dom_element_index(mut L: *mut lua_State) -> gint {
    if luaH_usemetatable(L, 1 as std::ffi::c_int, 2 as std::ffi::c_int) != 0 {
        return 1 as std::ffi::c_int;
    }
    let mut element = luaH_check_dom_element(L, 1 as std::ffi::c_int);
    let mut prop = luaL_checklstring(L, 2 as std::ffi::c_int, NULL_0 as *mut size_t);
    let mut token = l_tokenize(prop);
    let mut elem = (*element).element;
    match token as std::ffi::c_uint {
        235 => {
            lua_pushstring(L, webkit_dom_element_get_tag_name(elem));
            return 1 as std::ffi::c_int;
        }
        239 => {
            lua_pushstring(
                L,
                webkit_dom_node_get_text_content(
                    g_type_check_instance_cast(
                        elem as *mut GTypeInstance,
                        webkit_dom_node_get_type(),
                    ) as *mut std::ffi::c_void as *mut WebKitDOMNode,
                ),
            );
            return 1 as std::ffi::c_int;
        }
        121 => {
            lua_pushstring(L, webkit_dom_element_get_inner_html(elem));
            return 1 as std::ffi::c_int;
        }
        176 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_dom_element_query
                        as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        9 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_dom_element_append
                        as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        180 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_dom_element_remove
                        as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        28 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_dom_element_click
                        as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        99 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_dom_element_focus
                        as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        230 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_dom_element_submit
                        as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        2 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_dom_element_add_event_listener
                        as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        181 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_dom_element_remove_event_listener
                        as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        29 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_dom_element_client_rects
                        as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        25 => {
            lua_pushinteger(
                L,
                webkit_dom_element_get_child_element_count(elem) as lua_Integer,
            );
            return 1 as std::ffi::c_int;
        }
        221 => return luaH_dom_element_push_src(L),
        115 => return luaH_dom_element_push_href(L),
        248 => return luaH_dom_element_push_value(L),
        22 => {
            return webkit_dom_html_input_element_get_checked(
                g_type_check_instance_cast(
                    elem as *mut GTypeInstance,
                    webkit_dom_html_input_element_get_type(),
                ) as *mut std::ffi::c_void as *mut WebKitDOMHTMLInputElement,
            );
        }
        244 => {
            let mut type_0 = 0 as *mut gchar;
            g_object_get(
                (*element).element as gpointer,
                b"type\0" as *const u8 as *const std::ffi::c_char,
                &mut type_0 as *mut *mut gchar,
                NULL_0 as *mut std::ffi::c_void,
            );
            lua_pushstring(L, type_0);
            return 1 as std::ffi::c_int;
        }
        162 => return luaH_dom_element_push_parent(L),
        98 => return luaH_dom_element_push_first_child(L),
        135 => return luaH_dom_element_push_last_child(L),
        168 => return luaH_dom_element_push_prev_sibling(L),
        153 => return luaH_dom_element_push_next_sibling(L),
        177 => return luaH_dom_element_push_rect_table(L),
        10 => return luaH_dom_element_push_attribute_table(L),
        228 => return luaH_dom_element_push_style_table(L),
        52 => return luaH_dom_element_push_document(L),
        158 => return luaH_dom_element_push_owner_document(L),
        _ => return 0 as std::ffi::c_int,
    };
}

unsafe extern "C" fn luaH_dom_element_newindex(mut L: *mut lua_State) -> gint {
    let mut element = luaH_check_dom_element(L, 1 as std::ffi::c_int);
    let mut prop = luaL_checklstring(L, 2 as std::ffi::c_int, NULL_0 as *mut size_t);
    let mut token = l_tokenize(prop);
    let mut error = NULL_0 as *mut GError;
    match token as std::ffi::c_uint {
        121 => {
            webkit_dom_element_set_inner_html(
                (*element).element,
                luaL_checklstring(L, 3 as std::ffi::c_int, NULL_0 as *mut size_t),
                &mut error,
            );
            if !error.is_null() {
                return luaL_error(
                    L,
                    b"set inner html error: %s\0" as *const u8
                        as *const std::ffi::c_char,
                    (*error).message,
                );
            }
        }
        248 => {
            if dom_html_element_set_value(
                L,
                g_type_check_instance_cast(
                    (*element).element as *mut GTypeInstance,
                    webkit_dom_html_element_get_type(),
                ) as *mut std::ffi::c_void as *mut WebKitDOMHTMLElement,
            ) == 0
            {
                return luaL_error(
                    L,
                    b"set value error: wrong element type\0" as *const u8
                        as *const std::ffi::c_char,
                );
            }
        }
        22 => {
            webkit_dom_html_input_element_set_checked(
                g_type_check_instance_cast(
                    (*element).element as *mut GTypeInstance,
                    webkit_dom_html_input_element_get_type(),
                ) as *mut std::ffi::c_void as *mut WebKitDOMHTMLInputElement,
                lua_toboolean(L, 3 as std::ffi::c_int),
            );
        }
        _ => return 0 as std::ffi::c_int,
    }
    return luaH_object_property_signal(L, 1 as std::ffi::c_int, token);
}
#[no_mangle]

pub unsafe extern "C" fn dom_element_class_setup(mut L: *mut lua_State) {
    static mut dom_element_methods: [luaL_Reg; 4] = unsafe {
        [
            {
                let mut init = luaL_Reg {
                    name: b"add_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_dom_element_class_add_signal
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"remove_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_dom_element_class_remove_signal
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"emit_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_dom_element_class_emit_signal
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: NULL_0 as *const std::ffi::c_char,
                    func: ::core::mem::transmute::<
                        libc::intptr_t,
                        lua_CFunction,
                    >(NULL_0 as libc::intptr_t),
                };
                init
            },
        ]
    };
    static mut dom_element_meta: [luaL_Reg; 9] = unsafe {
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
                        luaH_dom_element_index
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__newindex\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_dom_element_newindex
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__gc\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_dom_element_gc
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: NULL_0 as *const std::ffi::c_char,
                    func: ::core::mem::transmute::<
                        libc::intptr_t,
                        lua_CFunction,
                    >(NULL_0 as libc::intptr_t),
                };
                init
            },
        ]
    };
    luaH_class_setup(
        L,
        &mut dom_element_class,
        b"dom_element\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut lua_State) -> *mut dom_element_t>,
            lua_class_allocator_t,
        >(
            Some(
                dom_element_new
                    as unsafe extern "C" fn(*mut lua_State) -> *mut dom_element_t,
            ),
        ),
        ::core::mem::transmute::<
            libc::intptr_t,
            lua_class_propfunc_t,
        >(NULL_0 as libc::intptr_t),
        ::core::mem::transmute::<
            libc::intptr_t,
            lua_class_propfunc_t,
        >(NULL_0 as libc::intptr_t),
        dom_element_methods.as_ptr(),
        dom_element_meta.as_ptr(),
    );
    luaH_uniq_setup(L, REG_KEY.as_ptr(), b"\0" as *const u8 as *const std::ffi::c_char);
}
