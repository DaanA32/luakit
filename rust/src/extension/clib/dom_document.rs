use ::libc;

pub mod __stddef_ptrdiff_t_h {

    pub type ptrdiff_t = std::ffi::c_long;
}

pub mod __stddef_size_t_h {

    pub type size_t = std::ffi::c_ulong;
}

pub mod glibconfig_h {

    pub type guint32 = std::ffi::c_uint;

    pub type gsize = std::ffi::c_ulong;
}

pub mod gtypes_h {

    pub type gchar = std::ffi::c_char;

    pub type glong = std::ffi::c_long;

    pub type gint = std::ffi::c_int;

    pub type gboolean = gint;

    pub type guint = std::ffi::c_uint;

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
    use super::gtypes_h::{gpointer, guint, gboolean};
    extern "C" {

        pub fn g_ptr_array_free(
            array: *mut GPtrArray,
            free_segment: gboolean,
        ) -> *mut gpointer;
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

pub mod gdataset_h {

    pub type GData = _GData;
    extern "C" {

        pub type _GData;
    }
}

pub mod ghash_h {

    pub type GHashTable = _GHashTable;
    extern "C" {

        pub type _GHashTable;
    }
}

pub mod gtree_h {

    pub type GTree = _GTree;
    use super::gtypes_h::{GCompareDataFunc, gpointer, GDestroyNotify};
    extern "C" {

        pub type _GTree;

        pub fn g_tree_new_full(
            key_compare_func: GCompareDataFunc,
            key_compare_data: gpointer,
            key_destroy_func: GDestroyNotify,
            value_destroy_func: GDestroyNotify,
        ) -> *mut GTree;
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
    use super::gtypes_h::{guint, gpointer};
    use super::gdataset_h::GData;
    extern "C" {

        pub fn g_object_weak_ref(
            object: *mut GObject,
            notify: GWeakNotify,
            data: gpointer,
        );
    }
}

pub mod webkitdomdefines_h {

    pub type WebKitDOMNode = _WebKitDOMNode;

    pub type WebKitDOMObject = _WebKitDOMObject;

    pub type WebKitDOMDOMWindow = _WebKitDOMDOMWindow;

    pub type WebKitDOMDocument = _WebKitDOMDocument;

    pub type WebKitDOMElement = _WebKitDOMElement;

    pub type WebKitDOMHTMLElement = _WebKitDOMHTMLElement;
    use super::WebKitDOMNode_h::_WebKitDOMNode;
    use super::WebKitDOMObject_h::_WebKitDOMObject;
    use super::WebKitDOMDOMWindow_h::_WebKitDOMDOMWindow;
    use super::WebKitDOMDocument_h::_WebKitDOMDocument;
    use super::WebKitDOMElement_h::_WebKitDOMElement;
    use super::WebKitDOMHTMLElement_h::_WebKitDOMHTMLElement;
}

pub mod WebKitDOMNode_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _WebKitDOMNode {
        pub parent_instance: WebKitDOMObject,
    }
    use super::webkitdomdefines_h::WebKitDOMObject;
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

pub mod WebKitDOMDOMWindow_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _WebKitDOMDOMWindow {
        pub parent_instance: WebKitDOMObject,
    }
    use super::webkitdomdefines_h::{WebKitDOMObject, WebKitDOMDOMWindow};
    use super::gtypes_h::glong;
    extern "C" {

        pub fn webkit_dom_dom_window_get_inner_height(
            self_0: *mut WebKitDOMDOMWindow,
        ) -> glong;

        pub fn webkit_dom_dom_window_get_inner_width(
            self_0: *mut WebKitDOMDOMWindow,
        ) -> glong;

        pub fn webkit_dom_dom_window_get_scroll_x(
            self_0: *mut WebKitDOMDOMWindow,
        ) -> glong;

        pub fn webkit_dom_dom_window_get_scroll_y(
            self_0: *mut WebKitDOMDOMWindow,
        ) -> glong;
    }
}

pub mod WebKitDOMDocument_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _WebKitDOMDocument {
        pub parent_instance: WebKitDOMNode,
    }
    use super::webkitdomdefines_h::{
        WebKitDOMNode, WebKitDOMDocument, WebKitDOMElement, WebKitDOMDOMWindow,
        WebKitDOMHTMLElement,
    };
    use super::gtype_h::GType;
    use super::gtypes_h::{gchar, glong};
    use super::gerror_h::GError;
    extern "C" {

        pub fn webkit_dom_document_get_type() -> GType;

        pub fn webkit_dom_document_create_element(
            self_0: *mut WebKitDOMDocument,
            tagName: *const gchar,
            error: *mut *mut GError,
        ) -> *mut WebKitDOMElement;

        pub fn webkit_dom_document_element_from_point(
            self_0: *mut WebKitDOMDocument,
            x: glong,
            y: glong,
        ) -> *mut WebKitDOMElement;

        pub fn webkit_dom_document_get_default_view(
            self_0: *mut WebKitDOMDocument,
        ) -> *mut WebKitDOMDOMWindow;

        pub fn webkit_dom_document_get_body(
            self_0: *mut WebKitDOMDocument,
        ) -> *mut WebKitDOMHTMLElement;
    }
}

pub mod WebKitDOMElement_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _WebKitDOMElement {
        pub parent_instance: WebKitDOMNode,
    }
    use super::webkitdomdefines_h::{WebKitDOMNode, WebKitDOMElement};
    use super::gtype_h::GType;
    use super::gtypes_h::gchar;
    use super::gerror_h::GError;
    extern "C" {

        pub fn webkit_dom_element_get_type() -> GType;

        pub fn webkit_dom_element_set_attribute(
            self_0: *mut WebKitDOMElement,
            name: *const gchar,
            value: *const gchar,
            error: *mut *mut GError,
        );
    }
}

pub mod WebKitDOMHTMLElement_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _WebKitDOMHTMLElement {
        pub parent_instance: WebKitDOMElement,
    }
    use super::webkitdomdefines_h::{WebKitDOMElement, WebKitDOMHTMLElement};
    use super::gtype_h::GType;
    use super::gtypes_h::gchar;
    use super::gerror_h::GError;
    extern "C" {

        pub fn webkit_dom_html_element_get_type() -> GType;

        pub fn webkit_dom_html_element_set_inner_text(
            self_0: *mut WebKitDOMHTMLElement,
            value: *const gchar,
            error: *mut *mut GError,
        );
    }
}

pub mod lua_h {

    pub type lua_CFunction = Option::<
        unsafe extern "C" fn(*mut lua_State) -> std::ffi::c_int,
    >;

    pub type lua_Number = std::ffi::c_double;

    pub type lua_Integer = ptrdiff_t;

    pub const LUA_MULTRET: std::ffi::c_int = -(1 as std::ffi::c_int);

    pub const LUA_GLOBALSINDEX: std::ffi::c_int = -(10002 as std::ffi::c_int);

    pub const LUA_TTABLE: std::ffi::c_int = 5 as std::ffi::c_int;
    use super::__stddef_ptrdiff_t_h::ptrdiff_t;
    use super::__stddef_size_t_h::size_t;
    extern "C" {

        pub type lua_State;

        pub fn lua_gettop(L: *mut lua_State) -> std::ffi::c_int;

        pub fn lua_settop(L: *mut lua_State, idx: std::ffi::c_int);

        pub fn lua_pushvalue(L: *mut lua_State, idx: std::ffi::c_int);

        pub fn lua_isstring(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;

        pub fn lua_type(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;

        pub fn lua_tolstring(
            L: *mut lua_State,
            idx: std::ffi::c_int,
            len: *mut size_t,
        ) -> *const std::ffi::c_char;

        pub fn lua_pushnil(L: *mut lua_State);

        pub fn lua_pushinteger(L: *mut lua_State, n: lua_Integer);

        pub fn lua_pushlstring(L: *mut lua_State, s: *const std::ffi::c_char, l: size_t);

        pub fn lua_pushcclosure(
            L: *mut lua_State,
            fn_0: lua_CFunction,
            n: std::ffi::c_int,
        );

        pub fn lua_createtable(
            L: *mut lua_State,
            narr: std::ffi::c_int,
            nrec: std::ffi::c_int,
        );

        pub fn lua_newuserdata(L: *mut lua_State, sz: size_t) -> *mut std::ffi::c_void;

        pub fn lua_rawset(L: *mut lua_State, idx: std::ffi::c_int);

        pub fn lua_setmetatable(
            L: *mut lua_State,
            objindex: std::ffi::c_int,
        ) -> std::ffi::c_int;

        pub fn lua_setfenv(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;

        pub fn lua_next(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;
    }
}

pub mod lauxlib_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct luaL_Reg {
        pub name: *const std::ffi::c_char,
        pub func: lua_CFunction,
    }
    use super::lua_h::{lua_CFunction, lua_State, lua_Number};
    use super::__stddef_size_t_h::size_t;
    extern "C" {

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

pub mod signal_h {

    pub type signal_t = GTree;
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
    }
}

pub mod dom_document_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _dom_document_t {
        pub signals: *mut signal_t,
        pub document: *mut WebKitDOMDocument,
    }

    pub type dom_document_t = _dom_document_t;
    use super::signal_h::signal_t;
    use super::webkitdomdefines_h::WebKitDOMDocument;
}

pub mod string_h {
    extern "C" {

        pub fn memset(
            _: *mut std::ffi::c_void,
            _: std::ffi::c_int,
            _: std::ffi::c_ulong,
        ) -> *mut std::ffi::c_void;
    }
}

pub mod gmem_h {
    use super::gtypes_h::gpointer;
    extern "C" {

        pub fn g_free(mem: gpointer);
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

pub mod luaobject_h {
    use super::lua_h::lua_State;
    use super::luaclass_h::lua_class_t;
    use super::gtypes_h::{gint, gchar};
    extern "C" {

        pub fn luaH_settype(L: *mut lua_State, lua_class: *mut lua_class_t) -> gint;

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

        pub fn luaH_object_tostring(_: *mut lua_State) -> gint;

        pub fn luaH_object_gc(_: *mut lua_State) -> gint;
    }
}

pub mod dom_element_h {
    use super::lua_h::lua_State;
    use super::webkitdomdefines_h::WebKitDOMElement;
    use super::gtypes_h::gint;
    extern "C" {

        pub fn luaH_dom_element_from_node(
            L: *mut lua_State,
            node: *mut WebKitDOMElement,
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
}
pub use self::__stddef_ptrdiff_t_h::ptrdiff_t;
pub use self::__stddef_size_t_h::size_t;
pub use self::glibconfig_h::{guint32, gsize};
pub use self::gtypes_h::{
    gchar, glong, gint, gboolean, guint, gpointer, gconstpointer, GCompareDataFunc,
    GDestroyNotify,
};
pub use self::garray_h::{_GPtrArray, GPtrArray, g_ptr_array_free};
pub use self::gquark_h::GQuark;
pub use self::gerror_h::{_GError, GError};
pub use self::gdataset_h::{GData, _GData};
pub use self::ghash_h::{GHashTable, _GHashTable};
pub use self::gtree_h::{GTree, _GTree, g_tree_new_full};
pub use self::gtype_h::{
    GType, _GTypeClass, GTypeClass, _GTypeInstance, GTypeInstance,
    g_type_check_instance_cast, g_type_check_instance_is_a,
};
pub use self::gobject_h::{_GObject, GObject, GWeakNotify, g_object_weak_ref};
pub use self::webkitdomdefines_h::{
    WebKitDOMNode, WebKitDOMObject, WebKitDOMDOMWindow, WebKitDOMDocument,
    WebKitDOMElement, WebKitDOMHTMLElement,
};
pub use self::WebKitDOMNode_h::_WebKitDOMNode;
pub use self::WebKitDOMObject_h::_WebKitDOMObject;
pub use self::WebKitDOMDOMWindow_h::{
    _WebKitDOMDOMWindow, webkit_dom_dom_window_get_inner_height,
    webkit_dom_dom_window_get_inner_width, webkit_dom_dom_window_get_scroll_x,
    webkit_dom_dom_window_get_scroll_y,
};
pub use self::WebKitDOMDocument_h::{
    _WebKitDOMDocument, webkit_dom_document_get_type, webkit_dom_document_create_element,
    webkit_dom_document_element_from_point, webkit_dom_document_get_default_view,
    webkit_dom_document_get_body,
};
pub use self::WebKitDOMElement_h::{
    _WebKitDOMElement, webkit_dom_element_get_type, webkit_dom_element_set_attribute,
};
pub use self::WebKitDOMHTMLElement_h::{
    _WebKitDOMHTMLElement, webkit_dom_html_element_get_type,
    webkit_dom_html_element_set_inner_text,
};
pub use self::lua_h::{
    lua_CFunction, lua_Number, lua_Integer, LUA_MULTRET, LUA_GLOBALSINDEX, LUA_TTABLE,
    lua_State, lua_gettop, lua_settop, lua_pushvalue, lua_isstring, lua_type,
    lua_tolstring, lua_pushnil, lua_pushinteger, lua_pushlstring, lua_pushcclosure,
    lua_createtable, lua_newuserdata, lua_rawset, lua_setmetatable, lua_setfenv, lua_next,
};
pub use self::lauxlib_h::{
    luaL_Reg, luaL_argerror, luaL_checklstring, luaL_checknumber, luaL_error,
};
pub use self::common_h::{_common_t, common_t, common};
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
pub use self::dom_document_h::{_dom_document_t, dom_document_t};
use self::string_h::memset;
use self::gmem_h::g_free;
use self::gtestutils_h::g_strcmp0;
use self::luaobject_h::{
    luaH_settype, luaH_object_emit_signal, luaH_object_add_signal_simple,
    luaH_object_remove_signal_simple, luaH_object_remove_signals_simple,
    luaH_object_emit_signal_simple, luaH_object_tostring, luaH_object_gc,
};
use self::dom_element_h::luaH_dom_element_from_node;
use self::luauniq_h::{
    luaH_uniq_setup, luaH_uniq_add_ptr, luaH_uniq_get_ptr, luaH_uniq_del_ptr,
};
pub use self::gmacros_h::{FALSE, TRUE};
pub use self::__stddef_null_h::NULL;

pub const REG_KEY: [std::ffi::c_char; 34] = unsafe {
    *::core::mem::transmute::<
        &[u8; 34],
        &[std::ffi::c_char; 34],
    >(b"luakit.uniq.registry.dom_document\0")
};

static mut dom_document_class: lua_class_t = lua_class_t {
    name: 0 as *const gchar,
    signals: 0 as *const signal_t as *mut signal_t,
    allocator: None,
    properties: 0 as *const lua_class_property_array_t
        as *mut lua_class_property_array_t,
    index_miss_property: None,
    newindex_miss_property: None,
};
#[inline]

unsafe extern "C" fn luaH_dom_document_class_emit_signal(mut L: *mut lua_State) -> gint {
    return luaH_class_emit_signal(
        L,
        &mut dom_document_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, NULL as *mut size_t),
        lua_gettop(L) - 1 as std::ffi::c_int,
        LUA_MULTRET,
    );
}
#[inline]

unsafe extern "C" fn luaH_dom_document_class_remove_signal(
    mut L: *mut lua_State,
) -> gint {
    luaH_class_remove_signal(
        L,
        &mut dom_document_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, NULL as *mut size_t),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[inline]

unsafe extern "C" fn luaH_dom_document_class_add_signal(mut L: *mut lua_State) -> gint {
    luaH_class_add_signal(
        L,
        &mut dom_document_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, NULL as *mut size_t),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[inline]

unsafe extern "C" fn dom_document_new(mut L: *mut lua_State) -> *mut dom_document_t {
    let mut p = lua_newuserdata(
        L,
        ::core::mem::size_of::<dom_document_t>() as std::ffi::c_ulong,
    ) as *mut dom_document_t;
    memset(
        p as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        (::core::mem::size_of::<dom_document_t>() as std::ffi::c_ulong)
            .wrapping_mul(1 as std::ffi::c_int as std::ffi::c_ulong),
    );
    (*p).signals = signal_new();
    luaH_settype(L, &mut dom_document_class);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_setmetatable(L, -(2 as std::ffi::c_int));
    lua_setfenv(L, -(2 as std::ffi::c_int));
    lua_pushvalue(L, -(1 as std::ffi::c_int));
    luaH_class_emit_signal(
        L,
        &mut dom_document_class,
        b"new\0" as *const u8 as *const std::ffi::c_char,
        1 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    return p;
}

unsafe extern "C" fn luaH_check_dom_document(
    mut L: *mut lua_State,
    mut udx: gint,
) -> *mut dom_document_t {
    let mut document = luaH_checkudata(L, udx, &mut dom_document_class)
        as *mut dom_document_t;
    if ((*document).document).is_null()
        || ({
            let mut __inst = (*document).document as *mut GTypeInstance;
            let mut __t = webkit_dom_document_get_type();
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
            b"DOM document no longer valid\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    return document;
}

unsafe extern "C" fn webkit_dom_document_destroy_cb(
    mut document: *mut dom_document_t,
    mut doc: *mut GObject,
) {
    let mut L = common.L;
    luaH_uniq_get_ptr(L, REG_KEY.as_ptr(), doc as gpointer);
    luaH_object_emit_signal(
        L,
        -(1 as std::ffi::c_int),
        b"destroy\0" as *const u8 as *const std::ffi::c_char,
        0 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    (*document).document = NULL as *mut WebKitDOMDocument;
    luaH_uniq_del_ptr(common.L, REG_KEY.as_ptr(), doc as gpointer);
}
#[no_mangle]

pub unsafe extern "C" fn luaH_dom_document_from_webkit_dom_document(
    mut L: *mut lua_State,
    mut doc: *mut WebKitDOMDocument,
) -> gint {
    if luaH_uniq_get_ptr(L, REG_KEY.as_ptr(), doc as gpointer) != 0 {
        return 1 as std::ffi::c_int;
    }
    let mut document = dom_document_new(L);
    (*document).document = doc;
    luaH_uniq_add_ptr(L, REG_KEY.as_ptr(), doc as gpointer, -(1 as std::ffi::c_int));
    g_object_weak_ref(
        g_type_check_instance_cast(
            doc as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut dom_document_t, *mut GObject) -> ()>,
            GWeakNotify,
        >(
            Some(
                webkit_dom_document_destroy_cb
                    as unsafe extern "C" fn(*mut dom_document_t, *mut GObject) -> (),
            ),
        ),
        document as gpointer,
    );
    return 1 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_dom_document_gc(mut L: *mut lua_State) -> gint {
    return luaH_object_gc(L);
}

unsafe extern "C" fn luaH_dom_document_push_body(
    mut L: *mut lua_State,
    mut document: *mut dom_document_t,
) -> gint {
    let mut node = webkit_dom_document_get_body((*document).document);
    return luaH_dom_element_from_node(
        L,
        g_type_check_instance_cast(
            node as *mut GTypeInstance,
            webkit_dom_element_get_type(),
        ) as *mut std::ffi::c_void as *mut WebKitDOMElement,
    );
}

unsafe extern "C" fn luaH_dom_document_window_index(mut L: *mut lua_State) -> gint {
    let mut document = luaH_check_dom_document(
        L,
        LUA_GLOBALSINDEX - 1 as std::ffi::c_int,
    );
    let mut prop = luaL_checklstring(L, 2 as std::ffi::c_int, NULL as *mut size_t);
    let mut token = l_tokenize(prop);
    let mut window = webkit_dom_document_get_default_view((*document).document);
    match token as std::ffi::c_uint {
        194 => {
            lua_pushinteger(L, webkit_dom_dom_window_get_scroll_x(window));
            return 1 as std::ffi::c_int;
        }
        195 => {
            lua_pushinteger(L, webkit_dom_dom_window_get_scroll_y(window));
            return 1 as std::ffi::c_int;
        }
        122 => {
            lua_pushinteger(L, webkit_dom_dom_window_get_inner_width(window));
            return 1 as std::ffi::c_int;
        }
        120 => {
            lua_pushinteger(L, webkit_dom_dom_window_get_inner_height(window));
            return 1 as std::ffi::c_int;
        }
        _ => return 0 as std::ffi::c_int,
    };
}

unsafe extern "C" fn luaH_dom_document_push_window_table(mut L: *mut lua_State) -> gint {
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
            luaH_dom_document_window_index
                as unsafe extern "C" fn(*mut lua_State) -> gint,
        ),
        1 as std::ffi::c_int,
    );
    lua_rawset(L, -(3 as std::ffi::c_int));
    lua_setmetatable(L, -(2 as std::ffi::c_int));
    return 1 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_dom_document_create_element(mut L: *mut lua_State) -> gint {
    let mut document = luaH_check_dom_document(L, 1 as std::ffi::c_int);
    let mut tagname = luaL_checklstring(L, 2 as std::ffi::c_int, NULL as *mut size_t);
    let mut error = NULL as *mut GError;
    let mut elem = webkit_dom_document_create_element(
        (*document).document,
        tagname,
        &mut error,
    );
    if !error.is_null() {
        return luaL_error(
            L,
            b"create element error: %s\0" as *const u8 as *const std::ffi::c_char,
            (*error).message,
        );
    }
    if lua_type(L, 3 as std::ffi::c_int) == LUA_TTABLE {
        lua_pushnil(L);
        while lua_next(L, 3 as std::ffi::c_int) != 0 as std::ffi::c_int {
            let mut name = luaL_checklstring(
                L,
                -(2 as std::ffi::c_int),
                NULL as *mut size_t,
            );
            let mut value = luaL_checklstring(
                L,
                -(1 as std::ffi::c_int),
                NULL as *mut size_t,
            );
            webkit_dom_element_set_attribute(elem, name, value, &mut error);
            lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
            if !error.is_null() {
                return luaL_error(
                    L,
                    b"set new element attribute error: %s\0" as *const u8
                        as *const std::ffi::c_char,
                    (*error).message,
                );
            }
        }
    }
    if lua_isstring(L, 4 as std::ffi::c_int) != 0 {
        let mut inner_text = lua_tolstring(L, 4 as std::ffi::c_int, NULL as *mut size_t);
        webkit_dom_html_element_set_inner_text(
            g_type_check_instance_cast(
                elem as *mut GTypeInstance,
                webkit_dom_html_element_get_type(),
            ) as *mut std::ffi::c_void as *mut WebKitDOMHTMLElement,
            inner_text,
            NULL as *mut *mut GError,
        );
    }
    return luaH_dom_element_from_node(L, elem);
}

unsafe extern "C" fn luaH_dom_document_element_from_point(
    mut L: *mut lua_State,
) -> gint {
    let mut document = luaH_check_dom_document(L, 1 as std::ffi::c_int);
    let mut x = luaL_checknumber(L, 2 as std::ffi::c_int) as glong;
    let mut y = luaL_checknumber(L, 3 as std::ffi::c_int) as glong;
    let mut elem = webkit_dom_document_element_from_point((*document).document, x, y);
    return luaH_dom_element_from_node(L, elem);
}

unsafe extern "C" fn luaH_dom_document_index(mut L: *mut lua_State) -> gint {
    if luaH_usemetatable(L, 1 as std::ffi::c_int, 2 as std::ffi::c_int) != 0 {
        return 1 as std::ffi::c_int;
    }
    let mut document = luaH_check_dom_document(L, 1 as std::ffi::c_int);
    let mut prop = luaL_checklstring(L, 2 as std::ffi::c_int, NULL as *mut size_t);
    let mut token = l_tokenize(prop);
    match token as std::ffi::c_uint {
        37 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_dom_document_create_element
                        as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        59 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_dom_document_element_from_point
                        as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        14 => return luaH_dom_document_push_body(L, document),
        263 => return luaH_dom_document_push_window_table(L),
        _ => return 0 as std::ffi::c_int,
    };
}
#[no_mangle]

pub unsafe extern "C" fn dom_document_class_setup(mut L: *mut lua_State) {
    static mut dom_document_methods: [luaL_Reg; 4] = unsafe {
        [
            {
                let mut init = luaL_Reg {
                    name: b"add_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_dom_document_class_add_signal
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"remove_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_dom_document_class_remove_signal
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"emit_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_dom_document_class_emit_signal
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
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
    static mut dom_document_meta: [luaL_Reg; 8] = unsafe {
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
                        luaH_dom_document_index
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__gc\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_dom_document_gc
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
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
        &mut dom_document_class,
        b"dom_document\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut lua_State) -> *mut dom_document_t>,
            lua_class_allocator_t,
        >(
            Some(
                dom_document_new
                    as unsafe extern "C" fn(*mut lua_State) -> *mut dom_document_t,
            ),
        ),
        ::core::mem::transmute::<
            libc::intptr_t,
            lua_class_propfunc_t,
        >(NULL as libc::intptr_t),
        ::core::mem::transmute::<
            libc::intptr_t,
            lua_class_propfunc_t,
        >(NULL as libc::intptr_t),
        dom_document_methods.as_ptr(),
        dom_document_meta.as_ptr(),
    );
    luaH_uniq_setup(L, REG_KEY.as_ptr(), b"\0" as *const u8 as *const std::ffi::c_char);
}
