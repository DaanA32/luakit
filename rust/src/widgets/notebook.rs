use ::libc;


pub mod __stddef_size_t_h {

    pub type size_t = std::ffi::c_ulong;
}

pub mod glibconfig_h {

    pub type gint8 = std::ffi::c_schar;

    pub type guint8 = std::ffi::c_uchar;

    pub type gint16 = std::ffi::c_short;

    pub type guint16 = std::ffi::c_ushort;

    pub type guint32 = std::ffi::c_uint;

    pub type gsize = std::ffi::c_ulong;
}

pub mod gtypes_h {

    pub type gchar = std::ffi::c_char;

    pub type gint = std::ffi::c_int;

    pub type gboolean = gint;

    pub type guint = std::ffi::c_uint;

    pub type gpointer = *mut std::ffi::c_void;
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
    unsafe extern "C" {

        pub type _GTree;
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

    pub type GInitiallyUnowned = _GObject;
    use super::gtype_h::GTypeInstance;
    use super::gtypes_h::{guint, gpointer, gchar};
    use super::gdataset_h::GData;
    unsafe extern "C" {

        pub fn g_object_connect(
            object: gpointer,
            signal_spec: *const gchar,
            _: ...
        ) -> gpointer;

        pub fn g_object_get_data(object: *mut GObject, key: *const gchar) -> gpointer;
    }
}

pub mod lua_h {

    pub type lua_CFunction = Option::<
        unsafe extern "C" fn(*mut lua_State) -> std::ffi::c_int,
    >;

    pub type lua_Number = std::ffi::c_double;

    pub const LUA_REGISTRYINDEX: std::ffi::c_int = -(10000 as std::ffi::c_int);

    pub const LUA_TBOOLEAN: std::ffi::c_int = 1 as std::ffi::c_int;
    use super::__stddef_size_t_h::size_t;
    unsafe extern "C" {

        pub type lua_State;

        pub fn lua_gettop(L: *mut lua_State) -> std::ffi::c_int;

        pub fn lua_settop(L: *mut lua_State, idx: std::ffi::c_int);

        pub fn lua_remove(L: *mut lua_State, idx: std::ffi::c_int);

        pub fn lua_isnumber(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;

        pub fn lua_type(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;

        pub fn lua_toboolean(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;

        pub fn lua_pushnumber(L: *mut lua_State, n: lua_Number);

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
    }
}

pub mod signal_h {

    pub type signal_t = GTree;
    use super::gtree_h::GTree;
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

        pub fn luaH_checkudata(
            _: *mut lua_State,
            _: gint,
            _: *mut lua_class_t,
        ) -> gpointer;
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
        mut L: *mut lua_State,
        mut udx: gint,
    ) -> *mut widget_t {
        let mut w = luaH_checkudata(L, udx, &mut widget_class) as *mut widget_t;
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
    use super::gtype_h::GType;
    use super::gtktypes_h::GtkWidget;
    unsafe extern "C" {

        pub type _GtkWidgetPrivate;

        pub fn gtk_widget_get_type() -> GType;

        pub fn gtk_widget_show(widget: *mut GtkWidget);
    }
}

pub mod pango_layout_h {

    pub type PangoEllipsizeMode = std::ffi::c_uint;

    pub const PANGO_ELLIPSIZE_END: PangoEllipsizeMode = 3;

    pub const PANGO_ELLIPSIZE_MIDDLE: PangoEllipsizeMode = 2;

    pub const PANGO_ELLIPSIZE_START: PangoEllipsizeMode = 1;

    pub const PANGO_ELLIPSIZE_NONE: PangoEllipsizeMode = 0;
}

pub mod cairo_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _cairo_rectangle_int {
        pub x: std::ffi::c_int,
        pub y: std::ffi::c_int,
        pub width: std::ffi::c_int,
        pub height: std::ffi::c_int,
    }

    pub type cairo_rectangle_int_t = _cairo_rectangle_int;
}

pub mod gdktypes_h {

    pub type GdkRectangle = cairo_rectangle_int_t;

    pub type GdkWindow = _GdkWindow;
    use super::cairo_h::cairo_rectangle_int_t;
    unsafe extern "C" {

        pub type _GdkWindow;
    }
}

pub mod gdkevents_h {

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
    #[derive(Copy, Clone)]
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

        pub is_modifier: [u8; 1],

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
    use super::gdktypes_h::GdkWindow;
    use super::glibconfig_h::{gint8, guint32, guint16, guint8, gint16};
    use super::gtypes_h::{guint, gint, gchar};
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
    use super::gtypes_h::gchar;
    unsafe extern "C" {

        pub type _GtkContainerPrivate;

        pub fn gtk_container_get_type() -> GType;

        pub fn gtk_container_child_set(
            container: *mut GtkContainer,
            child: *mut GtkWidget,
            first_prop_name: *const gchar,
            _: ...
        );
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
    use super::gtypes_h::gchar;
    use super::gtktypes_h::GtkWidget;
    use super::pango_layout_h::{PangoEllipsizeMode, PANGO_ELLIPSIZE_NONE};
    unsafe extern "C" {

        pub type _GtkLabelPrivate;

        pub fn gtk_label_get_type() -> GType;

        pub fn gtk_label_new(str: *const gchar) -> *mut GtkWidget;

        pub fn gtk_label_set_ellipsize(label: *mut GtkLabel, mode: PangoEllipsizeMode);
    }
}

pub mod gtknotebook_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GtkNotebook {
        pub container: GtkContainer,
        pub priv_0: *mut GtkNotebookPrivate,
    }

    pub type GtkNotebookPrivate = _GtkNotebookPrivate;

    pub type GtkNotebook = _GtkNotebook;
    use super::gtkcontainer_h::GtkContainer;
    use super::gtype_h::GType;
    use super::gtktypes_h::GtkWidget;
    use super::gtypes_h::{gint, gboolean, gchar};
    unsafe extern "C" {

        pub type _GtkNotebookPrivate;

        pub fn gtk_notebook_get_type() -> GType;

        pub fn gtk_notebook_new() -> *mut GtkWidget;

        pub fn gtk_notebook_insert_page(
            notebook: *mut GtkNotebook,
            child: *mut GtkWidget,
            tab_label: *mut GtkWidget,
            position: gint,
        ) -> gint;

        pub fn gtk_notebook_get_current_page(notebook: *mut GtkNotebook) -> gint;

        pub fn gtk_notebook_get_nth_page(
            notebook: *mut GtkNotebook,
            page_num: gint,
        ) -> *mut GtkWidget;

        pub fn gtk_notebook_get_n_pages(notebook: *mut GtkNotebook) -> gint;

        pub fn gtk_notebook_page_num(
            notebook: *mut GtkNotebook,
            child: *mut GtkWidget,
        ) -> gint;

        pub fn gtk_notebook_set_current_page(notebook: *mut GtkNotebook, page_num: gint);

        pub fn gtk_notebook_set_show_border(
            notebook: *mut GtkNotebook,
            show_border: gboolean,
        );

        pub fn gtk_notebook_get_show_border(notebook: *mut GtkNotebook) -> gboolean;

        pub fn gtk_notebook_set_show_tabs(
            notebook: *mut GtkNotebook,
            show_tabs: gboolean,
        );

        pub fn gtk_notebook_get_show_tabs(notebook: *mut GtkNotebook) -> gboolean;

        pub fn gtk_notebook_set_scrollable(
            notebook: *mut GtkNotebook,
            scrollable: gboolean,
        );

        pub fn gtk_notebook_set_tab_label(
            notebook: *mut GtkNotebook,
            child: *mut GtkWidget,
            tab_label: *mut GtkWidget,
        );

        pub fn gtk_notebook_get_tab_label_text(
            notebook: *mut GtkNotebook,
            child: *mut GtkWidget,
        ) -> *const gchar;

        pub fn gtk_notebook_reorder_child(
            notebook: *mut GtkNotebook,
            child: *mut GtkWidget,
            position: gint,
        );
    }
}

pub mod gmessages_h {

    pub const G_LOG_DOMAIN: std::ffi::c_int = 0 as std::ffi::c_int;
}

pub mod gtestutils_h {
    unsafe extern "C" {

        pub fn g_assertion_message_expr(
            domain: *const std::ffi::c_char,
            file: *const std::ffi::c_char,
            line: std::ffi::c_int,
            func: *const std::ffi::c_char,
            expr: *const std::ffi::c_char,
        ) -> !;
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
        lua_State, lua_pushlstring, lua_rawget, LUA_REGISTRYINDEX, lua_pushlightuserdata,
        lua_remove,
    };
    use super::gtypes_h::{gpointer, gint, gchar};
    use super::tokenize_h::{luakit_token_t, L_TK_UNKNOWN};
    unsafe extern "C" {

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
    use super::lua_h::{lua_State, lua_type, LUA_TBOOLEAN, lua_toboolean};
    use super::gtypes_h::{gint, gboolean};
    use super::lauxlib_h::luaL_typerror;
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

        pub fn luaH_widget_remove(_: *mut lua_State) -> gint;

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

pub mod gmacros_h {

    pub const FALSE: std::ffi::c_int = 0 as std::ffi::c_int;

    pub const TRUE: std::ffi::c_int = (FALSE == 0) as std::ffi::c_int;
}

pub mod __stddef_null_h {

    pub const NULL: std::ffi::c_int = 0 as std::ffi::c_int;
}
pub use self::__stddef_size_t_h::size_t;
pub use self::glibconfig_h::{gint8, guint8, gint16, guint16, guint32, gsize};
pub use self::gtypes_h::{gchar, gint, gboolean, guint, gpointer};
pub use self::gdataset_h::{GData, _GData};
pub use self::ghash_h::{GHashTable, _GHashTable};
pub use self::gtree_h::{GTree, _GTree};
pub use self::gtype_h::{
    GType, _GTypeClass, GTypeClass, _GTypeInstance, GTypeInstance,
    g_type_check_instance_cast, g_type_check_instance_is_a,
};
pub use self::gclosure_h::GCallback;
pub use self::gobject_h::{
    _GObject, GObject, GInitiallyUnowned, g_object_connect, g_object_get_data,
};
pub use self::lua_h::{
    lua_CFunction, lua_Number, LUA_REGISTRYINDEX, LUA_TBOOLEAN, lua_State, lua_gettop,
    lua_settop, lua_remove, lua_isnumber, lua_type, lua_toboolean, lua_pushnumber,
    lua_pushlstring, lua_pushstring, lua_pushcclosure, lua_pushboolean,
    lua_pushlightuserdata, lua_rawget,
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
pub use self::luaclass_h::{
    lua_class_property_array_t, lua_object_t, lua_class_allocator_t,
    lua_class_propfunc_t, lua_class_t, luaH_checkudata,
};
pub use self::common_h::{_common_t, common_t, common};
pub use self::widget_h::{
    widget_t, widget_destructor_t, widget_info_t, widget_constructor_t,
    GOBJECT_LUAKIT_WIDGET_DATA_KEY, luaH_checkwidget, widget_class,
};
pub use self::gtkcssprovider_h::{
    GtkCssProvider, _GtkCssProvider, GtkCssProviderPrivate, _GtkCssProviderPrivate,
};
pub use self::gtktypes_h::GtkWidget;
pub use self::gtkwidget_h::{
    _GtkWidget, GtkWidgetPrivate, _GtkWidgetPrivate, gtk_widget_get_type, gtk_widget_show,
};
pub use self::pango_layout_h::{
    PangoEllipsizeMode, PANGO_ELLIPSIZE_END, PANGO_ELLIPSIZE_MIDDLE,
    PANGO_ELLIPSIZE_START, PANGO_ELLIPSIZE_NONE,
};
pub use self::cairo_h::{_cairo_rectangle_int, cairo_rectangle_int_t};
pub use self::gdktypes_h::{GdkRectangle, GdkWindow, _GdkWindow};
pub use self::gdkevents_h::{
    GdkEventType, GDK_EVENT_LAST, GDK_PAD_GROUP_MODE, GDK_PAD_STRIP, GDK_PAD_RING,
    GDK_PAD_BUTTON_RELEASE, GDK_PAD_BUTTON_PRESS, GDK_TOUCHPAD_PINCH, GDK_TOUCHPAD_SWIPE,
    GDK_TOUCH_CANCEL, GDK_TOUCH_END, GDK_TOUCH_UPDATE, GDK_TOUCH_BEGIN, GDK_DAMAGE,
    GDK_GRAB_BROKEN, GDK_OWNER_CHANGE, GDK_SETTING, GDK_WINDOW_STATE, GDK_SCROLL,
    GDK_VISIBILITY_NOTIFY, GDK_CLIENT_EVENT, GDK_DROP_FINISHED, GDK_DROP_START,
    GDK_DRAG_STATUS, GDK_DRAG_MOTION, GDK_DRAG_LEAVE, GDK_DRAG_ENTER, GDK_PROXIMITY_OUT,
    GDK_PROXIMITY_IN, GDK_SELECTION_NOTIFY, GDK_SELECTION_REQUEST, GDK_SELECTION_CLEAR,
    GDK_PROPERTY_NOTIFY, GDK_UNMAP, GDK_MAP, GDK_CONFIGURE, GDK_FOCUS_CHANGE,
    GDK_LEAVE_NOTIFY, GDK_ENTER_NOTIFY, GDK_KEY_RELEASE, GDK_KEY_PRESS,
    GDK_BUTTON_RELEASE, GDK_TRIPLE_BUTTON_PRESS, GDK_3BUTTON_PRESS,
    GDK_DOUBLE_BUTTON_PRESS, GDK_2BUTTON_PRESS, GDK_BUTTON_PRESS, GDK_MOTION_NOTIFY,
    GDK_EXPOSE, GDK_DESTROY, GDK_DELETE, GDK_NOTHING, _GdkEventKey, GdkEventKey,
    _GdkEventFocus, GdkEventFocus,
};
pub use self::gtkcontainer_h::{
    GtkContainer, _GtkContainer, GtkContainerPrivate, _GtkContainerPrivate,
    gtk_container_get_type, gtk_container_child_set,
};
pub use self::gtkmisc_h::{_GtkMisc, GtkMiscPrivate, GtkMisc, _GtkMiscPrivate};
pub use self::gtklabel_h::{
    _GtkLabel, GtkLabelPrivate, GtkLabel, _GtkLabelPrivate, gtk_label_get_type,
    gtk_label_new, gtk_label_set_ellipsize,
};
pub use self::gtknotebook_h::{
    _GtkNotebook, GtkNotebookPrivate, GtkNotebook, _GtkNotebookPrivate,
    gtk_notebook_get_type, gtk_notebook_new, gtk_notebook_insert_page,
    gtk_notebook_get_current_page, gtk_notebook_get_nth_page, gtk_notebook_get_n_pages,
    gtk_notebook_page_num, gtk_notebook_set_current_page, gtk_notebook_set_show_border,
    gtk_notebook_get_show_border, gtk_notebook_set_show_tabs, gtk_notebook_get_show_tabs,
    gtk_notebook_set_scrollable, gtk_notebook_set_tab_label,
    gtk_notebook_get_tab_label_text, gtk_notebook_reorder_child,
};
pub use self::gmessages_h::G_LOG_DOMAIN;
use self::gtestutils_h::g_assertion_message_expr;
use self::lauxlib_h::{luaL_typerror, luaL_checklstring, luaL_checknumber, luaL_error};
pub use self::luaobject_h::{
    luaH_object_registry_push, luaH_object_push, luaH_object_emit_signal,
    luaH_object_property_signal,
};
pub use self::luah_h::luaH_checkboolean;
use self::widgets_common_h::{
    focus_cb, key_press_cb, luaH_widget_destroy, luaH_widget_focus,
    luaH_widget_get_children, luaH_widget_hide, luaH_widget_remove, luaH_widget_show,
    luaH_widget_replace, luaH_widget_send_key, luaH_widget_get_parent,
    luaH_widget_get_focused, luaH_widget_get_visible, luaH_widget_get_width,
    luaH_widget_get_height, luaH_widget_set_visible, luaH_widget_set_tooltip,
    luaH_widget_get_tooltip, luaH_widget_set_min_size, luaH_widget_get_min_size,
    luaH_widget_set_align, luaH_widget_get_align, parent_set_cb, resize_cb, destroy_cb,
};
pub use self::gmacros_h::{FALSE, TRUE};
pub use self::__stddef_null_h::NULL;

unsafe extern "C" fn luaH_notebook_current(mut L: *mut lua_State) -> gint {
    let mut w = luaH_checkwidget(L, 1 as std::ffi::c_int);
    let mut n = gtk_notebook_get_n_pages(
        g_type_check_instance_cast(
            (*w).widget as *mut GTypeInstance,
            gtk_notebook_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkNotebook,
    );
    if n == 1 as std::ffi::c_int {
        lua_pushnumber(L, 1 as std::ffi::c_int as lua_Number);
    } else {
        lua_pushnumber(
            L,
            (gtk_notebook_get_current_page(
                g_type_check_instance_cast(
                    (*w).widget as *mut GTypeInstance,
                    gtk_notebook_get_type(),
                ) as *mut std::ffi::c_void as *mut GtkNotebook,
            ) + 1 as std::ffi::c_int) as lua_Number,
        );
    }
    return 1 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_notebook_atindex(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
    mut idx: gint,
) -> gint {
    if idx != -(1 as std::ffi::c_int) {
        idx -= 1;
        idx;
    }
    let mut widget = gtk_notebook_get_nth_page(
        g_type_check_instance_cast(
            (*w).widget as *mut GTypeInstance,
            gtk_notebook_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkNotebook,
        idx,
    );
    if widget.is_null() {
        return 0 as std::ffi::c_int;
    }
    let mut child = g_object_get_data(
        g_type_check_instance_cast(
            widget as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject,
        GOBJECT_LUAKIT_WIDGET_DATA_KEY.as_ptr(),
    ) as *mut widget_t;
    luaH_object_push(L, (*child).ref_0);
    return 1 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_notebook_indexof(mut L: *mut lua_State) -> gint {
    let mut w = luaH_checkwidget(L, 1 as std::ffi::c_int);
    let mut child = luaH_checkwidget(L, 2 as std::ffi::c_int);
    let mut i = gtk_notebook_page_num(
        g_type_check_instance_cast(
            (*w).widget as *mut GTypeInstance,
            gtk_notebook_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkNotebook,
        (*child).widget,
    );
    i += 1;
    if i == 0 {
        return 0 as std::ffi::c_int;
    }
    lua_pushnumber(L, i as lua_Number);
    return 1 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_notebook_insert(mut L: *mut lua_State) -> gint {
    let mut w = luaH_checkwidget(L, 1 as std::ffi::c_int);
    let mut pos = -(1 as std::ffi::c_int);
    let mut idx = 2 as std::ffi::c_int;
    if lua_gettop(L) > 2 as std::ffi::c_int {
        let fresh0 = idx;
        idx = idx + 1;
        pos = luaL_checknumber(L, fresh0) as gint;
        if pos > 0 as std::ffi::c_int {
            pos -= 1;
            pos;
        }
    }
    pos = gtk_notebook_insert_page(
        g_type_check_instance_cast(
            (*w).widget as *mut GTypeInstance,
            gtk_notebook_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkNotebook,
        g_type_check_instance_cast(
            (*luaH_checkwidget(L, idx)).widget as *mut GTypeInstance,
            gtk_widget_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkWidget,
        NULL as *mut GtkWidget,
        pos,
    );
    if pos == -(1 as std::ffi::c_int) {
        return 0 as std::ffi::c_int;
    }
    pos += 1;
    lua_pushnumber(L, pos as lua_Number);
    return 1 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_notebook_count(mut L: *mut lua_State) -> gint {
    let mut w = luaH_checkwidget(L, 1 as std::ffi::c_int);
    lua_pushnumber(
        L,
        gtk_notebook_get_n_pages(
            g_type_check_instance_cast(
                (*w).widget as *mut GTypeInstance,
                gtk_notebook_get_type(),
            ) as *mut std::ffi::c_void as *mut GtkNotebook,
        ) as lua_Number,
    );
    return 1 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_notebook_set_title(mut L: *mut lua_State) -> gint {
    let mut len: size_t = 0;
    let mut w = luaH_checkwidget(L, 1 as std::ffi::c_int);
    let mut child = luaH_checkwidget(L, 2 as std::ffi::c_int);
    let mut title = luaL_checklstring(L, 3 as std::ffi::c_int, &mut len);
    let mut label = gtk_label_new(title);
    gtk_label_set_ellipsize(
        g_type_check_instance_cast(label as *mut GTypeInstance, gtk_label_get_type())
            as *mut std::ffi::c_void as *mut GtkLabel,
        PANGO_ELLIPSIZE_MIDDLE,
    );
    gtk_notebook_set_tab_label(
        g_type_check_instance_cast(
            (*w).widget as *mut GTypeInstance,
            gtk_notebook_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkNotebook,
        (*child).widget,
        label,
    );
    gtk_container_child_set(
        g_type_check_instance_cast(
            (*w).widget as *mut GTypeInstance,
            gtk_container_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkContainer,
        label,
        b"tab-expand\0" as *const u8 as *const std::ffi::c_char,
        TRUE,
        b"tab-fill\0" as *const u8 as *const std::ffi::c_char,
        TRUE,
        NULL as *mut std::ffi::c_void,
    );
    return 0 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_notebook_get_title(mut L: *mut lua_State) -> gint {
    let mut w = luaH_checkwidget(L, 1 as std::ffi::c_int);
    let mut child = luaH_checkwidget(L, 2 as std::ffi::c_int);
    lua_pushstring(
        L,
        gtk_notebook_get_tab_label_text(
            g_type_check_instance_cast(
                (*w).widget as *mut GTypeInstance,
                gtk_notebook_get_type(),
            ) as *mut std::ffi::c_void as *mut GtkNotebook,
            (*child).widget,
        ),
    );
    return 1 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_notebook_switch(mut L: *mut lua_State) -> gint {
    let mut w = luaH_checkwidget(L, 1 as std::ffi::c_int);
    let mut i = luaL_checknumber(L, 2 as std::ffi::c_int) as gint;
    if i != -(1 as std::ffi::c_int) {
        i -= 1;
        i;
    }
    gtk_notebook_set_current_page(
        g_type_check_instance_cast(
            (*w).widget as *mut GTypeInstance,
            gtk_notebook_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkNotebook,
        i,
    );
    lua_pushnumber(
        L,
        gtk_notebook_get_current_page(
            g_type_check_instance_cast(
                (*w).widget as *mut GTypeInstance,
                gtk_notebook_get_type(),
            ) as *mut std::ffi::c_void as *mut GtkNotebook,
        ) as lua_Number,
    );
    return 1 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_notebook_reorder(mut L: *mut lua_State) -> gint {
    let mut w = luaH_checkwidget(L, 1 as std::ffi::c_int);
    let mut child = luaH_checkwidget(L, 2 as std::ffi::c_int);
    let mut i = luaL_checknumber(L, 3 as std::ffi::c_int) as gint;
    if i != -(1 as std::ffi::c_int) {
        i -= 1;
        i;
    }
    gtk_notebook_reorder_child(
        g_type_check_instance_cast(
            (*w).widget as *mut GTypeInstance,
            gtk_notebook_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkNotebook,
        (*child).widget,
        i,
    );
    lua_pushnumber(
        L,
        gtk_notebook_page_num(
            g_type_check_instance_cast(
                (*w).widget as *mut GTypeInstance,
                gtk_notebook_get_type(),
            ) as *mut std::ffi::c_void as *mut GtkNotebook,
            (*child).widget,
        ) as lua_Number,
    );
    return 1 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_notebook_index(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
    mut token: luakit_token_t,
) -> gint {
    if token as std::ffi::c_uint == L_TK_UNKNOWN as std::ffi::c_int as std::ffi::c_uint
        && lua_isnumber(L, 2 as std::ffi::c_int) != 0
    {
        return luaH_notebook_atindex(
            L,
            w,
            luaL_checknumber(L, 2 as std::ffi::c_int) as gint,
        );
    }
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
        180 => {
            lua_pushcclosure(
                L,
                Some(luaH_widget_remove as unsafe extern "C" fn(*mut lua_State) -> gint),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        35 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_notebook_count as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        39 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_notebook_current as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        104 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_notebook_get_title
                        as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        119 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_notebook_indexof as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        123 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_notebook_insert as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        210 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_notebook_set_title
                        as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        232 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_notebook_switch as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        182 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_notebook_reorder as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        215 => {
            lua_pushboolean(
                L,
                gtk_notebook_get_show_tabs(
                    g_type_check_instance_cast(
                        (*w).widget as *mut GTypeInstance,
                        gtk_notebook_get_type(),
                    ) as *mut std::ffi::c_void as *mut GtkNotebook,
                ),
            );
            return 1 as std::ffi::c_int;
        }
        212 => {
            lua_pushboolean(
                L,
                gtk_notebook_get_show_border(
                    g_type_check_instance_cast(
                        (*w).widget as *mut GTypeInstance,
                        gtk_notebook_get_type(),
                    ) as *mut std::ffi::c_void as *mut GtkNotebook,
                ),
            );
            return 1 as std::ffi::c_int;
        }
        _ => {}
    }
    return 0 as std::ffi::c_int;
}

unsafe extern "C" fn luaH_notebook_newindex(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
    mut token: luakit_token_t,
) -> gint {
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
        215 => {
            gtk_notebook_set_show_tabs(
                g_type_check_instance_cast(
                    (*w).widget as *mut GTypeInstance,
                    gtk_notebook_get_type(),
                ) as *mut std::ffi::c_void as *mut GtkNotebook,
                luaH_checkboolean(L, 3 as std::ffi::c_int),
            );
        }
        212 => {
            gtk_notebook_set_show_border(
                g_type_check_instance_cast(
                    (*w).widget as *mut GTypeInstance,
                    gtk_notebook_get_type(),
                ) as *mut std::ffi::c_void as *mut GtkNotebook,
                luaH_checkboolean(L, 3 as std::ffi::c_int),
            );
        }
        _ => return 0 as std::ffi::c_int,
    }
    return luaH_object_property_signal(L, 1 as std::ffi::c_int, token);
}

unsafe extern "C" fn page_added_cb(
    mut UNUSED_n: *mut GtkNotebook,
    mut widget: *mut GtkWidget,
    mut i: guint,
    mut w: *mut widget_t,
) {
    let mut child = g_object_get_data(
        g_type_check_instance_cast(
            widget as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject,
        GOBJECT_LUAKIT_WIDGET_DATA_KEY.as_ptr(),
    ) as *mut widget_t;
    let mut L = common.L;
    luaH_object_push(L, (*w).ref_0);
    luaH_object_push(L, (*child).ref_0);
    lua_pushnumber(L, i.wrapping_add(1 as std::ffi::c_int as guint) as lua_Number);
    luaH_object_emit_signal(
        L,
        -(3 as std::ffi::c_int),
        b"page-added\0" as *const u8 as *const std::ffi::c_char,
        2 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
}

unsafe extern "C" fn page_removed_cb(
    mut UNUSED_n: *mut GtkNotebook,
    mut widget: *mut GtkWidget,
    mut UNUSED_i: guint,
    mut w: *mut widget_t,
) {
    let mut child = g_object_get_data(
        g_type_check_instance_cast(
            widget as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject,
        GOBJECT_LUAKIT_WIDGET_DATA_KEY.as_ptr(),
    ) as *mut widget_t;
    let mut L = common.L;
    luaH_object_push(L, (*w).ref_0);
    luaH_object_push(L, (*child).ref_0);
    luaH_object_emit_signal(
        L,
        -(2 as std::ffi::c_int),
        b"page-removed\0" as *const u8 as *const std::ffi::c_char,
        1 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
}

unsafe extern "C" fn switch_cb(
    mut n: *mut GtkNotebook,
    mut UNUSED_p: *mut GtkWidget,
    mut i: guint,
    mut w: *mut widget_t,
) {
    let mut widget = gtk_notebook_get_nth_page(
        g_type_check_instance_cast(n as *mut GTypeInstance, gtk_notebook_get_type())
            as *mut std::ffi::c_void as *mut GtkNotebook,
        i as gint,
    );
    let mut child = g_object_get_data(
        g_type_check_instance_cast(
            widget as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject,
        GOBJECT_LUAKIT_WIDGET_DATA_KEY.as_ptr(),
    ) as *mut widget_t;
    let mut L = common.L;
    luaH_object_push(L, (*w).ref_0);
    luaH_object_push(L, (*child).ref_0);
    lua_pushnumber(L, i.wrapping_add(1 as std::ffi::c_int as guint) as lua_Number);
    luaH_object_emit_signal(
        L,
        -(3 as std::ffi::c_int),
        b"switch-page\0" as *const u8 as *const std::ffi::c_char,
        2 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
}

unsafe extern "C" fn reorder_cb(
    mut UNUSED_n: *mut GtkNotebook,
    mut widget: *mut GtkWidget,
    mut i: guint,
    mut w: *mut widget_t,
) {
    let mut child = g_object_get_data(
        g_type_check_instance_cast(
            widget as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject,
        GOBJECT_LUAKIT_WIDGET_DATA_KEY.as_ptr(),
    ) as *mut widget_t;
    let mut L = common.L;
    luaH_object_push(L, (*w).ref_0);
    luaH_object_push(L, (*child).ref_0);
    lua_pushnumber(L, i.wrapping_add(1 as std::ffi::c_int as guint) as lua_Number);
    luaH_object_emit_signal(
        L,
        -(3 as std::ffi::c_int),
        b"page-reordered\0" as *const u8 as *const std::ffi::c_char,
        2 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn widget_notebook(
    mut UNUSED_L: *mut lua_State,
    mut w: *mut widget_t,
    mut UNUSED_token: luakit_token_t,
) -> *mut widget_t {
    (*w)
        .index = Some(
        luaH_notebook_index
            as unsafe extern "C" fn(
                *mut lua_State,
                *mut widget_t,
                luakit_token_t,
            ) -> gint,
    );
    (*w)
        .newindex = Some(
        luaH_notebook_newindex
            as unsafe extern "C" fn(
                *mut lua_State,
                *mut widget_t,
                luakit_token_t,
            ) -> gint,
    );
    (*w).widget = gtk_notebook_new();
    gtk_notebook_set_show_border(
        g_type_check_instance_cast(
            (*w).widget as *mut GTypeInstance,
            gtk_notebook_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkNotebook,
        FALSE,
    );
    gtk_notebook_set_scrollable(
        g_type_check_instance_cast(
            (*w).widget as *mut GTypeInstance,
            gtk_notebook_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkNotebook,
        TRUE,
    );
    g_object_connect(
        g_type_check_instance_cast(
            (*w).widget as *mut GTypeInstance,
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
        b"signal::page-added\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut GtkNotebook,
                    *mut GtkWidget,
                    guint,
                    *mut widget_t,
                ) -> (),
            >,
            GCallback,
        >(
            Some(
                page_added_cb
                    as unsafe extern "C" fn(
                        *mut GtkNotebook,
                        *mut GtkWidget,
                        guint,
                        *mut widget_t,
                    ) -> (),
            ),
        ),
        w,
        b"signal::page-removed\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut GtkNotebook,
                    *mut GtkWidget,
                    guint,
                    *mut widget_t,
                ) -> (),
            >,
            GCallback,
        >(
            Some(
                page_removed_cb
                    as unsafe extern "C" fn(
                        *mut GtkNotebook,
                        *mut GtkWidget,
                        guint,
                        *mut widget_t,
                    ) -> (),
            ),
        ),
        w,
        b"signal::page-reordered\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut GtkNotebook,
                    *mut GtkWidget,
                    guint,
                    *mut widget_t,
                ) -> (),
            >,
            GCallback,
        >(
            Some(
                reorder_cb
                    as unsafe extern "C" fn(
                        *mut GtkNotebook,
                        *mut GtkWidget,
                        guint,
                        *mut widget_t,
                    ) -> (),
            ),
        ),
        w,
        b"signal::switch-page\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut GtkNotebook,
                    *mut GtkWidget,
                    guint,
                    *mut widget_t,
                ) -> (),
            >,
            GCallback,
        >(
            Some(
                switch_cb
                    as unsafe extern "C" fn(
                        *mut GtkNotebook,
                        *mut GtkWidget,
                        guint,
                        *mut widget_t,
                    ) -> (),
            ),
        ),
        w,
        NULL as *mut std::ffi::c_void,
    );
    gtk_widget_show((*w).widget);
    return w;
}
