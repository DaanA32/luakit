use ::libc;
use lua::ffi::{luaL_Reg, luaL_checklstring, lua_State, lua_createtable, lua_gettop, lua_newuserdata, lua_pushstring, lua_pushvalue, lua_setmetatable, LUA_MULTRET};
use lua::Function;
pub mod __stddef_size_t_h {
    pub type size_t = std::ffi::c_ulong;
}
pub mod glibconfig_h {
    pub type gsize = std::ffi::c_ulong;
}
pub mod gtypes_h {
    pub type gchar = std::ffi::c_char;
    pub type gint = std::ffi::c_int;
    pub type gboolean = gint;
    pub type guint = std::ffi::c_uint;
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
    use super::gtypes_h::{gpointer, guint, gboolean};
    unsafe extern "C" {
        pub fn g_ptr_array_free(
            array: *mut GPtrArray,
            free_segment: gboolean,
        ) -> *mut gpointer;
    }
}
pub mod gdataset_h {
    pub type GData = _GData;
    unsafe extern "C" {
        pub type _GData;
    }
}
pub mod ghash_h {
    pub type GHashTable = _GHashTable;
    unsafe extern "C" {
        pub type _GHashTable;
    }
}
pub mod gtree_h {
    pub type GTree = _GTree;
    use super::gtypes_h::{GCompareDataFunc, gpointer, GDestroyNotify};
    unsafe extern "C" {
        pub type _GTree;
        pub fn g_tree_new_full(
            key_compare_func: GCompareDataFunc,
            key_compare_data: gpointer,
            key_destroy_func: GDestroyNotify,
            value_destroy_func: GDestroyNotify,
        ) -> *mut GTree;
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
    use lua::ffi::lua_State;

    use crate::common::luaclass::signal_t;

    use super::gtypes_h::{gint, gpointer, gchar};
    use super::tokenize_h::luakit_token_t;
    use super::gtktypes_h::GtkWidget;
    use super::gtkcssprovider_h::GtkCssProvider;
}
pub mod gtkcssprovider_h {
    pub type GtkCssProvider = _GtkCssProvider;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GtkCssProvider {
        pub parent_instance: GObject,
        pub priv_0: *mut GtkCssProviderPrivate,
    }
    pub type GtkCssProviderPrivate = _GtkCssProviderPrivate;
    use super::gobject_h::GObject;
    unsafe extern "C" {
        pub type _GtkCssProviderPrivate;
    }
}
pub mod gobject_h {
    pub type GObject = _GObject;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GObject {
        pub g_type_instance: GTypeInstance,
        pub ref_count: guint,
        pub qdata: *mut GData,
    }
    pub type GInitiallyUnowned = _GObject;
    use super::gtype_h::GTypeInstance;
    use super::gtypes_h::guint;
    use super::gdataset_h::GData;
}
pub mod gtype_h {
    pub type GTypeInstance = _GTypeInstance;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GTypeInstance {
        pub g_class: *mut GTypeClass,
    }
    pub type GTypeClass = _GTypeClass;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GTypeClass {
        pub g_type: GType,
    }
    pub type GType = gsize;
    use super::glibconfig_h::gsize;
}
pub mod gtktypes_h {
    pub type GtkWidget = _GtkWidget;
    use super::gtkwidget_h::_GtkWidget;
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
    unsafe extern "C" {
        pub type _GtkWidgetPrivate;
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
pub mod giotypes_h {
    pub type GApplication = _GApplication;
    use super::gapplication_h::_GApplication;
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
pub mod globalconf_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
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
    unsafe extern "C" {
        pub static mut globalconf: globalconf_t;
    }
}
pub mod WebKitUserContent_h {
    pub type WebKitUserContentInjectedFrames = std::ffi::c_uint;
    pub const WEBKIT_USER_CONTENT_INJECT_TOP_FRAME: WebKitUserContentInjectedFrames = 1;
    pub const WEBKIT_USER_CONTENT_INJECT_ALL_FRAMES: WebKitUserContentInjectedFrames = 0;
    pub type WebKitUserStyleLevel = std::ffi::c_uint;
    pub const WEBKIT_USER_STYLE_LEVEL_AUTHOR: WebKitUserStyleLevel = 1;
    pub const WEBKIT_USER_STYLE_LEVEL_USER: WebKitUserStyleLevel = 0;
    pub type WebKitUserStyleSheet = _WebKitUserStyleSheet;
    use super::gtypes_h::gchar;
    unsafe extern "C" {
        pub type _WebKitUserStyleSheet;
        pub fn webkit_user_style_sheet_unref(
            user_style_sheet: *mut WebKitUserStyleSheet,
        );
        pub fn webkit_user_style_sheet_new(
            source: *const gchar,
            injected_frames: WebKitUserContentInjectedFrames,
            level: WebKitUserStyleLevel,
            allow_list: *const *const gchar,
            block_list: *const *const gchar,
        ) -> *mut WebKitUserStyleSheet;
    }
}
pub mod stylesheet_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct lstylesheet_t {
        pub signals: *mut signal_t,
        pub stylesheet: *mut WebKitUserStyleSheet,
        pub source: *mut gchar,
    }
    use crate::common::luaclass::signal_t;

    use super::WebKitUserContent_h::WebKitUserStyleSheet;
    use super::gtypes_h::gchar;
    use super::widget_h::widget_t;
    unsafe extern "C" {
        pub fn webview_stylesheets_regenerate_stylesheet(
            w: *mut widget_t,
            stylesheet: *mut lstylesheet_t,
        );
    }
}
pub mod string_h {
    unsafe extern "C" {
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
        pub fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    }
}
pub mod gmem_h {
    use super::gtypes_h::gpointer;
    use super::glibconfig_h::gsize;
    unsafe extern "C" {
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
    use super::gtypes_h::gchar;
    use super::__stddef_null_h::NULL;
    use super::string_h::{strlen, memcpy};
    use super::__stddef_size_t_h::size_t;
    use super::gmem_h::g_malloc;
    unsafe extern "C" {
        pub fn g_strdup(str: *const gchar) -> *mut gchar;
    }
}
pub mod gtestutils_h {
    unsafe extern "C" {
        pub fn g_strcmp0(
            str1: *const std::ffi::c_char,
            str2: *const std::ffi::c_char,
        ) -> std::ffi::c_int;
    }
}
pub mod gmacros_h {
    pub const FALSE: std::ffi::c_int = 0 as std::ffi::c_int;
    pub const TRUE: std::ffi::c_int = (FALSE == 0) as std::ffi::c_int;
}
pub mod __stddef_null_h {
    pub const NULL: std::ffi::c_int = 0 as std::ffi::c_int;
    pub const NULL_0: std::ffi::c_int = 0 as std::ffi::c_int;
    pub const NULL_1: std::ffi::c_int = 0 as std::ffi::c_int;
}
use crate::common::luaclass::{luaH_class_add_property, luaH_class_add_signal, luaH_class_emit_signal, luaH_class_index, luaH_class_newindex, luaH_class_remove_signal, luaH_class_setup, lua_class_allocator_t, lua_class_property_array_t, lua_class_propfunc_t, lua_class_t};

pub use self::__stddef_size_t_h::size_t;
pub use self::glibconfig_h::gsize;
pub use self::gtypes_h::{
    gchar, gint, gboolean, guint, gdouble, gpointer, gconstpointer, GCompareDataFunc,
    GDestroyNotify,
};
pub use self::garray_h::{_GPtrArray, GPtrArray, g_ptr_array_free};
pub use self::gdataset_h::{GData, _GData};
pub use self::ghash_h::{GHashTable, _GHashTable};
pub use self::gtree_h::{GTree, _GTree, g_tree_new_full};
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
pub use self::gtkcssprovider_h::{
    GtkCssProvider, _GtkCssProvider, GtkCssProviderPrivate, _GtkCssProviderPrivate,
};
pub use self::gobject_h::{GObject, _GObject, GInitiallyUnowned};
pub use self::gtype_h::{GTypeInstance, _GTypeInstance, GTypeClass, _GTypeClass, GType};
pub use self::gtktypes_h::GtkWidget;
pub use self::gtkwidget_h::{_GtkWidget, GtkWidgetPrivate, _GtkWidgetPrivate};
pub use self::gapplication_h::{_GApplication, GApplicationPrivate, _GApplicationPrivate};
pub use self::giotypes_h::GApplication;
pub use self::gtkapplication_h::{
    _GtkApplication, GtkApplicationPrivate, GtkApplication, _GtkApplicationPrivate,
};
pub use self::globalconf_h::{globalconf_t, globalconf};
pub use self::WebKitUserContent_h::{
    WebKitUserContentInjectedFrames, WEBKIT_USER_CONTENT_INJECT_TOP_FRAME,
    WEBKIT_USER_CONTENT_INJECT_ALL_FRAMES, WebKitUserStyleLevel,
    WEBKIT_USER_STYLE_LEVEL_AUTHOR, WEBKIT_USER_STYLE_LEVEL_USER, WebKitUserStyleSheet,
    _WebKitUserStyleSheet, webkit_user_style_sheet_unref, webkit_user_style_sheet_new,
};
pub use self::stylesheet_h::{lstylesheet_t, webview_stylesheets_regenerate_stylesheet};
use self::string_h::{memcpy, memset, strlen};
use self::gmem_h::{g_free, g_malloc};
pub use self::gstrfuncs_h::{g_strdup_inline, g_strdup};
use self::gtestutils_h::g_strcmp0;
pub use self::gmacros_h::{FALSE, TRUE};
pub use self::__stddef_null_h::{NULL, NULL_0, NULL_1};
unsafe extern "C" {
    pub fn webview_stylesheet_set_enabled(
        w: *mut widget_t,
        stylesheet: *mut lstylesheet_t,
        enable: gboolean,
    ) -> std::ffi::c_int;
}
static mut stylesheet_class: lua_class_t = lua_class_t {
    name: 0 as *const gchar,
    signals: 0 as *const signal_t as *mut signal_t,
    allocator: None,
    properties: 0 as *const lua_class_property_array_t
        as *mut lua_class_property_array_t,
    index_miss_property: None,
    newindex_miss_property: None,
};
#[inline]
unsafe extern "C" fn luaH_stylesheet_class_emit_signal(mut L: *mut lua_State) -> gint {
    return luaH_class_emit_signal(
        L,
        &mut stylesheet_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, NULL_1 as *mut usize),
        lua_gettop(L) - 1 as std::ffi::c_int,
        LUA_MULTRET,
    );
}
#[inline]
unsafe extern "C" fn luaH_stylesheet_class_remove_signal(mut L: *mut lua_State) -> gint {
    luaH_class_remove_signal(
        L,
        &mut stylesheet_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, NULL_1 as *mut usize),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[inline]
unsafe extern "C" fn luaH_stylesheet_class_add_signal(mut L: *mut lua_State) -> gint {
    luaH_class_add_signal(
        L,
        &mut stylesheet_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, NULL_1 as *mut usize),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[inline]
unsafe extern "C" fn stylesheet_new(mut L: *mut lua_State) -> *mut lstylesheet_t {
    let mut p = lua_newuserdata(
        L,
        ::core::mem::size_of::<lstylesheet_t>(),
    ) as *mut lstylesheet_t;
    memset(
        p as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        (::core::mem::size_of::<lstylesheet_t>() as std::ffi::c_ulong)
            .wrapping_mul(1 as std::ffi::c_int as std::ffi::c_ulong),
    );
    (*p).signals = signal_new();
    luaH_settype(L, &mut stylesheet_class);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_setmetatable(L, -(2 as std::ffi::c_int));
    println!("lua_setfenv(L, -(2 as std::ffi::c_int));");
    lua_pushvalue(L, -(1 as std::ffi::c_int));
    luaH_class_emit_signal(
        L,
        &mut stylesheet_class,
        b"new\0" as *const u8 as *const std::ffi::c_char,
        1 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    return p;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_checkstylesheet(
    mut L: *mut lua_State,
    mut idx: gint,
) -> gpointer {
    return luaH_checkudata(L, idx, &mut stylesheet_class);
}
unsafe extern "C" fn luaH_stylesheet_gc(mut L: *mut lua_State) -> gint {
    let mut stylesheet = luaH_checkstylesheet(L, 1 as std::ffi::c_int)
        as *mut lstylesheet_t;
    if !((*stylesheet).stylesheet).is_null() && !(globalconf.webviews).is_null() {
        let mut i = 0 as std::ffi::c_int as std::ffi::c_uint;
        while i < (*globalconf.webviews).len {
            let mut w = *((*globalconf.webviews).pdata).offset(i as isize)
                as *mut widget_t;
            webview_stylesheet_set_enabled(w, stylesheet, FALSE);
            i = i.wrapping_add(1);
            i;
        }
        webkit_user_style_sheet_unref((*stylesheet).stylesheet);
    }
    g_free((*stylesheet).source as gpointer);
    return luaH_object_gc(L);
}
unsafe extern "C" fn luaH_stylesheet_new(mut L: *mut lua_State) -> gint {
    luaH_class_new(L, &mut stylesheet_class);
    return 1 as std::ffi::c_int;
}
unsafe extern "C" fn regenerate_stylesheet(mut stylesheet: *mut lstylesheet_t) {
    let mut old = (*stylesheet).stylesheet;
    if !old.is_null() {
        webkit_user_style_sheet_unref(old);
    }
    (*stylesheet)
        .stylesheet = webkit_user_style_sheet_new(
        (*stylesheet).source,
        WEBKIT_USER_CONTENT_INJECT_ALL_FRAMES,
        WEBKIT_USER_STYLE_LEVEL_USER,
        NULL_1 as *const *const gchar,
        NULL_1 as *const *const gchar,
    );
    if !old.is_null() && !(globalconf.webviews).is_null() {
        let mut i = 0 as std::ffi::c_int as std::ffi::c_uint;
        while i < (*globalconf.webviews).len {
            let mut w = *((*globalconf.webviews).pdata).offset(i as isize)
                as *mut widget_t;
            webview_stylesheets_regenerate_stylesheet(w, stylesheet);
            i = i.wrapping_add(1);
            i;
        }
    }
}
unsafe extern "C" fn luaH_stylesheet_set_source(
    mut L: *mut lua_State,
    mut stylesheet: *mut lstylesheet_t,
) -> std::ffi::c_int {
    let mut new_source = luaL_checklstring(
        L,
        -(1 as std::ffi::c_int),
        NULL_1 as *mut usize,
    );
    g_free((*stylesheet).source as gpointer);
    (*stylesheet).source = g_strdup_inline(new_source);
    regenerate_stylesheet(stylesheet);
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn luaH_stylesheet_get_source(
    mut L: *mut lua_State,
    mut stylesheet: *mut lstylesheet_t,
) -> std::ffi::c_int {
    lua_pushstring(L, (*stylesheet).source);
    return 1 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stylesheet_class_setup(mut L: *mut lua_State) {
    static mut stylesheet_methods: [luaL_Reg; 5] = unsafe {
        [
            {
                let mut init = luaL_Reg {
                    name: b"add_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_stylesheet_class_add_signal
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"remove_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_stylesheet_class_remove_signal
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"emit_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_stylesheet_class_emit_signal
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__call\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_stylesheet_new
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: NULL_1 as *const std::ffi::c_char,
                    func: ::core::mem::transmute::<
                        libc::intptr_t,
                        Function,
                    >(NULL_1 as libc::intptr_t),
                };
                init
            },
        ]
    };
    static mut stylesheet_meta: [luaL_Reg; 9] = unsafe {
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
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__index\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_class_index,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__newindex\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_class_newindex
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__gc\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_stylesheet_gc
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: NULL_1 as *const std::ffi::c_char,
                    func: ::core::mem::transmute::<
                        libc::intptr_t,
                        Function,
                    >(NULL_1 as libc::intptr_t),
                };
                init
            },
        ]
    };
    luaH_class_setup(
        L,
        &mut stylesheet_class,
        b"stylesheet\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut lua_State) -> *mut lstylesheet_t>,
            lua_class_allocator_t,
        >(
            Some(
                stylesheet_new
                    as unsafe extern "C" fn(*mut lua_State) -> *mut lstylesheet_t,
            ),
        ),
        Some(
            luaH_class_index_miss_property
        ),
        Some(
            luaH_class_newindex_miss_property
        ),
        stylesheet_methods.as_ptr(),
        stylesheet_meta.as_ptr(),
    );
    luaH_class_add_property(
        &mut stylesheet_class,
        L_TK_SOURCE,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut lua_State,
                    *mut lstylesheet_t,
                ) -> std::ffi::c_int,
            >,
            lua_class_propfunc_t,
        >(
            Some(
                luaH_stylesheet_set_source
                    as unsafe extern "C" fn(
                        *mut lua_State,
                        *mut lstylesheet_t,
                    ) -> std::ffi::c_int,
            ),
        ),
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut lua_State,
                    *mut lstylesheet_t,
                ) -> std::ffi::c_int,
            >,
            lua_class_propfunc_t,
        >(
            Some(
                luaH_stylesheet_get_source
                    as unsafe extern "C" fn(
                        *mut lua_State,
                        *mut lstylesheet_t,
                    ) -> std::ffi::c_int,
            ),
        ),
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut lua_State,
                    *mut lstylesheet_t,
                ) -> std::ffi::c_int,
            >,
            lua_class_propfunc_t,
        >(
            Some(
                luaH_stylesheet_set_source
                    as unsafe extern "C" fn(
                        *mut lua_State,
                        *mut lstylesheet_t,
                    ) -> std::ffi::c_int,
            ),
        ),
    );
}
