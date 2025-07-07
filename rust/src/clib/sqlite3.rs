use ::libc;
use lua::ffi::{luaL_Reg, lua_State, lua_createtable, lua_error, lua_gettop, lua_newuserdata, lua_next, lua_pushfstring, lua_pushlstring, lua_pushnil, lua_pushnumber, lua_pushstring, lua_pushvalue, lua_rawset, lua_rawseti, lua_setmetatable, lua_settop, lua_toboolean, lua_tointeger, lua_tolstring, lua_tonumber, lua_type, lua_typename, LUA_MULTRET, LUA_TNUMBER, LUA_TSTRING, LUA_TTABLE};
use lua::{Function, Number};
pub mod __stddef_ptrdiff_t_h {
    pub type ptrdiff_t = std::ffi::c_long;
}
pub mod __stddef_size_t_h {
    pub type size_t = std::ffi::c_ulong;
}
pub mod gtypes_h {
    pub type gint = std::ffi::c_int;
    pub type gpointer = *mut std::ffi::c_void;
    pub type gchar = std::ffi::c_char;
    pub type GDestroyNotify = Option::<unsafe extern "C" fn(gpointer) -> ()>;
    pub type guint = std::ffi::c_uint;
    pub type gboolean = gint;
    pub type GCompareDataFunc = Option::<
        unsafe extern "C" fn(gconstpointer, gconstpointer, gpointer) -> gint,
    >;
    pub type gconstpointer = *const std::ffi::c_void;
}
pub mod sqlite3_h {
    pub type sqlite3_destructor_type = Option::<
        unsafe extern "C" fn(*mut std::ffi::c_void) -> (),
    >;
    pub const SQLITE_OK: std::ffi::c_int = 0 as std::ffi::c_int;
    pub const SQLITE_RANGE: std::ffi::c_int = 25 as std::ffi::c_int;
    pub const SQLITE_ROW: std::ffi::c_int = 100 as std::ffi::c_int;
    pub const SQLITE_DONE: std::ffi::c_int = 101 as std::ffi::c_int;
    pub const SQLITE_INTEGER: std::ffi::c_int = 1;
    pub const SQLITE_FLOAT: std::ffi::c_int = 2;
    pub const SQLITE_BLOB: std::ffi::c_int = 4;
    pub const SQLITE_NULL: std::ffi::c_int = 5;
    pub const SQLITE_TEXT: std::ffi::c_int = 3;
    pub const SQLITE_TRANSIENT: std::ffi::c_int = -(1 as std::ffi::c_int);
    unsafe extern "C" {
        pub type sqlite3_stmt;
        pub type sqlite3;
        pub fn sqlite3_close(_: *mut sqlite3) -> std::ffi::c_int;
        pub fn sqlite3_changes(_: *mut sqlite3) -> std::ffi::c_int;
        pub fn sqlite3_open(
            filename: *const std::ffi::c_char,
            ppDb: *mut *mut sqlite3,
        ) -> std::ffi::c_int;
        pub fn sqlite3_errmsg(_: *mut sqlite3) -> *const std::ffi::c_char;
        pub fn sqlite3_prepare_v2(
            db: *mut sqlite3,
            zSql: *const std::ffi::c_char,
            nByte: std::ffi::c_int,
            ppStmt: *mut *mut sqlite3_stmt,
            pzTail: *mut *const std::ffi::c_char,
        ) -> std::ffi::c_int;
        pub fn sqlite3_bind_double(
            _: *mut sqlite3_stmt,
            _: std::ffi::c_int,
            _: std::ffi::c_double,
        ) -> std::ffi::c_int;
        pub fn sqlite3_bind_int(
            _: *mut sqlite3_stmt,
            _: std::ffi::c_int,
            _: std::ffi::c_int,
        ) -> std::ffi::c_int;
        pub fn sqlite3_bind_text(
            _: *mut sqlite3_stmt,
            _: std::ffi::c_int,
            _: *const std::ffi::c_char,
            _: std::ffi::c_int,
            _: Option::<unsafe extern "C" fn(*mut std::ffi::c_void) -> ()>,
        ) -> std::ffi::c_int;
        pub fn sqlite3_bind_parameter_index(
            _: *mut sqlite3_stmt,
            zName: *const std::ffi::c_char,
        ) -> std::ffi::c_int;
        pub fn sqlite3_clear_bindings(_: *mut sqlite3_stmt) -> std::ffi::c_int;
        pub fn sqlite3_column_count(pStmt: *mut sqlite3_stmt) -> std::ffi::c_int;
        pub fn sqlite3_column_name(
            _: *mut sqlite3_stmt,
            N: std::ffi::c_int,
        ) -> *const std::ffi::c_char;
        pub fn sqlite3_step(_: *mut sqlite3_stmt) -> std::ffi::c_int;
        pub fn sqlite3_column_blob(
            _: *mut sqlite3_stmt,
            iCol: std::ffi::c_int,
        ) -> *const std::ffi::c_void;
        pub fn sqlite3_column_double(
            _: *mut sqlite3_stmt,
            iCol: std::ffi::c_int,
        ) -> std::ffi::c_double;
        pub fn sqlite3_column_type(
            _: *mut sqlite3_stmt,
            iCol: std::ffi::c_int,
        ) -> std::ffi::c_int;
        pub fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> std::ffi::c_int;
        pub fn sqlite3_reset(pStmt: *mut sqlite3_stmt) -> std::ffi::c_int;
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
            NULL_1 as *mut std::ffi::c_void,
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut gpointer) -> ()>,
                GDestroyNotify,
            >(Some(signal_array_destroy as unsafe extern "C" fn(*mut gpointer) -> ())),
        ) as *mut signal_t;
    }
    use glib_sys::{g_tree_new_full, GTree};

    use super::gtypes_h::{
        gconstpointer, gpointer, gint, GCompareDataFunc, GDestroyNotify,
    };
    use super::gtestutils_h::g_strcmp0;
    use super::garray_h::{g_ptr_array_free, GPtrArray};
    use super::gmacros_h::{FALSE, TRUE};
    use super::__stddef_null_h::NULL_1;
    use super::gmem_h::g_free;
}
pub mod ghash_h {
    pub type GHashTable = _GHashTable;
    unsafe extern "C" {
        pub type _GHashTable;
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
    unsafe extern "C" {
        pub fn _log(lvl: log_level_t, _: *const gchar, _: *const gchar, _: ...);
    }
}
pub mod glibconfig_h {
    pub type gsize = std::ffi::c_ulong;
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
pub mod garray_h {
    pub type GPtrArray = _GPtrArray;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GPtrArray {
        pub pdata: *mut gpointer,
        pub len: guint,
    }
    use super::gtypes_h::{gpointer, guint, gboolean};
    unsafe extern "C" {
        pub fn g_ptr_array_free(
            array: *mut GPtrArray,
            free_segment: gboolean,
        ) -> *mut gpointer;
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
            return NULL_0 as *mut std::ffi::c_char;
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
    use super::__stddef_null_h::NULL_0;
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
pub mod luaobject_h {
    #[inline]
    pub unsafe extern "C" fn luaH_object_registry_push(mut L: *mut lua_State) {
        lua_pushlstring(
            L,
            b"luakit.object.registry\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 23]>())
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>(),
                )
                .wrapping_sub(1),
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
    use lua::ffi::{lua_State, lua_pushlstring, lua_rawget, lua_settop, LUA_REGISTRYINDEX};

    use crate::common::luaclass::lua_class_t;

    use super::gtypes_h::{gint, gpointer};
    unsafe extern "C" {
        pub fn luaH_settype(L: *mut lua_State, lua_class: *mut lua_class_t) -> gint;
        pub fn luaH_object_incref(L: *mut lua_State, tud: gint, oud: gint) -> gpointer;
        pub fn luaH_object_decref(L: *mut lua_State, tud: gint, oud: gpointer);
        pub fn luaH_object_add_signal_simple(L: *mut lua_State) -> gint;
        pub fn luaH_object_remove_signal_simple(L: *mut lua_State) -> gint;
        pub fn luaH_object_remove_signals_simple(L: *mut lua_State) -> gint;
        pub fn luaH_object_emit_signal_simple(L: *mut lua_State) -> gint;
        pub fn luaH_object_tostring(_: *mut lua_State) -> gint;
        pub fn luaH_object_gc(_: *mut lua_State) -> gint;
    }
}
pub mod gmacros_h {
    pub const FALSE: std::ffi::c_int = 0 as std::ffi::c_int;
    pub const TRUE: std::ffi::c_int = (FALSE == 0) as std::ffi::c_int;
}
pub mod __stddef_null_h {
    pub const NULL_0: std::ffi::c_int = 0 as std::ffi::c_int;
    pub const NULL_1: std::ffi::c_int = 0 as std::ffi::c_int;
    pub const NULL: std::ffi::c_int = 0 as std::ffi::c_int;
}
use crate::common::luaclass::{luaH_checkudata, luaH_class_add_property, luaH_class_emit_signal, luaH_class_index, luaH_class_new, luaH_class_newindex, luaH_class_setup, lua_class_allocator_t, lua_class_property_array_t, lua_class_propfunc_t, lua_class_t};

pub use self::__stddef_ptrdiff_t_h::ptrdiff_t;
pub use self::__stddef_size_t_h::size_t;
pub use self::gtypes_h::{
    gint, gpointer, gchar, GDestroyNotify, guint, gboolean, GCompareDataFunc,
    gconstpointer,
};
pub use self::sqlite3_h::{
    sqlite3_destructor_type, SQLITE_OK, SQLITE_RANGE, SQLITE_ROW, SQLITE_DONE,
    SQLITE_INTEGER, SQLITE_FLOAT, SQLITE_BLOB, SQLITE_NULL, SQLITE_TEXT,
    SQLITE_TRANSIENT, sqlite3_stmt, sqlite3, sqlite3_close, sqlite3_changes,
    sqlite3_open, sqlite3_errmsg, sqlite3_prepare_v2, sqlite3_bind_double,
    sqlite3_bind_int, sqlite3_bind_text, sqlite3_bind_parameter_index,
    sqlite3_clear_bindings, sqlite3_column_count, sqlite3_column_name, sqlite3_step,
    sqlite3_column_blob, sqlite3_column_double, sqlite3_column_type, sqlite3_finalize,
    sqlite3_reset,
};
pub use self::signal_h::{signal_t, signal_cmp, signal_array_destroy, signal_new};
pub use self::ghash_h::{GHashTable, _GHashTable};
pub use self::log_h::{
    log_level_t, LOG_LEVEL_debug, LOG_LEVEL_verbose, LOG_LEVEL_info, LOG_LEVEL_warn,
    LOG_LEVEL_error, LOG_LEVEL_fatal, _log,
};
pub use self::glibconfig_h::gsize;
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
pub use self::garray_h::{GPtrArray, _GPtrArray, g_ptr_array_free};
use self::string_h::{memcpy, memset, strlen};
use self::gmem_h::{g_free, g_malloc};
pub use self::gstrfuncs_h::{g_strdup_inline, g_strdup};
use self::gtestutils_h::g_strcmp0;
pub use self::luaobject_h::{
    luaH_object_registry_push, luaH_object_ref, luaH_object_unref, luaH_settype,
    luaH_object_incref, luaH_object_decref, luaH_object_add_signal_simple,
    luaH_object_remove_signal_simple, luaH_object_remove_signals_simple,
    luaH_object_emit_signal_simple, luaH_object_tostring, luaH_object_gc,
};
pub use self::gmacros_h::{FALSE, TRUE};
pub use self::__stddef_null_h::{NULL_0, NULL_1, NULL};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_stmt_t {
    pub sqlite: *mut sqlite3_t,
    pub stmt: *mut sqlite3_stmt,
    pub parent_ref: gpointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_t {
    pub signals: *mut signal_t,
    pub filename: *mut std::ffi::c_char,
    pub db: *mut sqlite3,
}
static mut sqlite3_class: lua_class_t = lua_class_t {
    name: 0 as *const gchar,
    signals: 0 as *const signal_t as *mut signal_t,
    allocator: None,
    properties: 0 as *const lua_class_property_array_t
        as *mut lua_class_property_array_t,
    index_miss_property: None,
    newindex_miss_property: None,
};
static mut sqlite3_stmt_class: lua_class_t = lua_class_t {
    name: 0 as *const gchar,
    signals: 0 as *const signal_t as *mut signal_t,
    allocator: None,
    properties: 0 as *const lua_class_property_array_t
        as *mut lua_class_property_array_t,
    index_miss_property: None,
    newindex_miss_property: None,
};
#[inline]
unsafe extern "C" fn luaH_sqlite3_class_emit_signal(mut L: *mut lua_State) -> gint {
    return luaH_class_emit_signal(
        L,
        &mut sqlite3_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, NULL as *mut size_t),
        lua_gettop(L) - 1 as std::ffi::c_int,
        LUA_MULTRET,
    );
}
#[inline]
unsafe extern "C" fn luaH_sqlite3_class_remove_signal(mut L: *mut lua_State) -> gint {
    luaH_class_remove_signal(
        L,
        &mut sqlite3_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, NULL as *mut size_t),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[inline]
unsafe extern "C" fn luaH_sqlite3_class_add_signal(mut L: *mut lua_State) -> gint {
    luaH_class_add_signal(
        L,
        &mut sqlite3_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, NULL as *mut size_t),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[inline]
unsafe extern "C" fn sqlite3_new(mut L: *mut lua_State) -> *mut sqlite3_t {
    let mut p = lua_newuserdata(
        L,
        ::core::mem::size_of::<sqlite3_t>(),
    ) as *mut sqlite3_t;
    memset(
        p as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        (::core::mem::size_of::<sqlite3_t>() as std::ffi::c_ulong)
            .wrapping_mul(1 as std::ffi::c_int as std::ffi::c_ulong),
    );
    (*p).signals = signal_new();
    luaH_settype(L, &mut sqlite3_class);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_setmetatable(L, -(2 as std::ffi::c_int));
    println!("lua_setfenv(L, -(2 as std::ffi::c_int));");
    lua_pushvalue(L, -(1 as std::ffi::c_int));
    luaH_class_emit_signal(
        L,
        &mut sqlite3_class,
        b"new\0" as *const u8 as *const std::ffi::c_char,
        1 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    return p;
}
#[inline]
unsafe extern "C" fn luaH_sqlite3_checkopen(
    mut L: *mut lua_State,
    mut sqlite: *mut sqlite3_t,
) {
    if ((*sqlite).db).is_null() {
        lua_pushlstring(
            L,
            b"sqlite3: database handle closed\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 32]>())
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>(),
                )
                .wrapping_sub(1),
        );
        lua_error(L);
    }
}
unsafe extern "C" fn luaH_sqlite3_stmt_gc(mut L: *mut lua_State) -> gint {
    let mut stmt = luaH_checkudata(L, 1 as std::ffi::c_int, &mut sqlite3_stmt_class)
        as *mut sqlite3_stmt_t;
    luaH_object_unref(L, (*stmt).parent_ref);
    sqlite3_finalize((*stmt).stmt);
    return 0 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_stmt_new(mut L: *mut lua_State) -> *mut sqlite3_stmt_t {
    let mut p = lua_newuserdata(
        L,
        ::core::mem::size_of::<sqlite3_stmt_t>(),
    ) as *mut sqlite3_stmt_t;
    memset(
        p as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        (::core::mem::size_of::<sqlite3_stmt_t>() as std::ffi::c_ulong)
            .wrapping_mul(1 as std::ffi::c_int as std::ffi::c_ulong),
    );
    luaH_settype(L, &mut sqlite3_stmt_class);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_setmetatable(L, -(2 as std::ffi::c_int));
    println!("lua_setfenv(L, -(2 as std::ffi::c_int));");
    return p;
}
unsafe extern "C" fn luaH_sqlite3_compile(mut L: *mut lua_State) -> gint {
    let mut sqlite = luaH_checkudata(L, 1 as std::ffi::c_int, &mut sqlite3_class)
        as *mut sqlite3_t;
    luaH_sqlite3_checkopen(L, sqlite);
    let mut sql = luaL_checklstring(L, 2 as std::ffi::c_int, NULL as *mut size_t);
    let mut tail = 0 as *const gchar;
    let mut stmt = 0 as *mut sqlite3_stmt;
    if sqlite3_prepare_v2(
        (*sqlite).db,
        sql,
        -(1 as std::ffi::c_int),
        &mut stmt,
        &mut tail,
    ) != 0
    {
        lua_pushfstring(
            L,
            b"sqlite3: statement compilation failed (%s)\0" as *const u8
                as *const std::ffi::c_char,
            sqlite3_errmsg((*sqlite).db),
        );
        sqlite3_finalize(stmt);
        lua_error(L);
    } else if stmt.is_null() {
        lua_pushfstring(
            L,
            b"sqlite3: no SQL found in string: \"%s\"\0" as *const u8
                as *const std::ffi::c_char,
            sql,
        );
        lua_error(L);
    }
    let mut p = sqlite3_stmt_new(L);
    (*p).sqlite = sqlite;
    (*p).stmt = stmt;
    (*p).parent_ref = luaH_object_ref(L, 1 as std::ffi::c_int);
    if !tail.is_null() && *tail as std::ffi::c_int != 0 {
        lua_pushstring(L, tail);
        return 2 as std::ffi::c_int;
    }
    return 1 as std::ffi::c_int;
}
unsafe extern "C" fn luaH_sqlite3_close(mut L: *mut lua_State) -> gint {
    let mut sqlite = luaH_checkudata(L, 1 as std::ffi::c_int, &mut sqlite3_class)
        as *mut sqlite3_t;
    if !((*sqlite).filename).is_null() {
        g_free((*sqlite).filename as gpointer);
        (*sqlite).filename = NULL as *mut std::ffi::c_char;
    }
    if !((*sqlite).db).is_null() {
        sqlite3_close((*sqlite).db);
        (*sqlite).db = NULL as *mut sqlite3;
    }
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn luaH_sqlite3_gc(mut L: *mut lua_State) -> gint {
    luaH_sqlite3_close(L);
    return luaH_object_gc(L);
}
unsafe extern "C" fn luaH_sqlite3_set_filename(
    mut L: *mut lua_State,
    mut sqlite: *mut sqlite3_t,
) -> gint {
    let mut filename = luaL_checklstring(
        L,
        -(1 as std::ffi::c_int),
        NULL as *mut size_t,
    );
    if sqlite3_open(filename, &mut (*sqlite).db) != 0 {
        lua_pushfstring(
            L,
            b"sqlite3: failed to open \"%s\" (%s)\0" as *const u8
                as *const std::ffi::c_char,
            filename,
            sqlite3_errmsg((*sqlite).db),
        );
        sqlite3_close((*sqlite).db);
        lua_error(L);
    }
    (*sqlite).filename = g_strdup_inline(filename);
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn luaH_sqlite3_get_filename(
    mut L: *mut lua_State,
    mut sqlite: *mut sqlite3_t,
) -> gint {
    lua_pushstring(L, (*sqlite).filename);
    return 1 as std::ffi::c_int;
}
unsafe extern "C" fn luaH_sqlite3_changes(mut L: *mut lua_State) -> gint {
    let mut sqlite = luaH_checkudata(L, 1 as std::ffi::c_int, &mut sqlite3_class)
        as *mut sqlite3_t;
    luaH_sqlite3_checkopen(L, sqlite);
    lua_pushnumber(L, sqlite3_changes((*sqlite).db) as Number);
    return 1 as std::ffi::c_int;
}
unsafe extern "C" fn luaH_param_index(
    mut L: *mut lua_State,
    mut stmt: *mut sqlite3_stmt,
    mut idx: gint,
) -> gint {
    let mut type_0 = lua_type(L, idx);
    if type_0 == LUA_TNUMBER {
        return lua_tointeger(L, idx) as gint
    } else if type_0 == LUA_TSTRING {
        return sqlite3_bind_parameter_index(
            stmt,
            lua_tolstring(L, idx, NULL as *mut usize),
        )
    }
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn luaH_bind_value(
    mut L: *mut lua_State,
    mut stmt: *mut sqlite3_stmt,
    mut bidx: gint,
    mut idx: gint,
) -> gint {
    match lua_type(L, idx) {
        LUA_TNUMBER => return sqlite3_bind_double(stmt, bidx, lua_tonumber(L, idx)),
        LUA_TBOOLEAN => {
            return sqlite3_bind_int(
                stmt,
                bidx,
                if lua_toboolean(L, idx) != 0 {
                    1 as std::ffi::c_int
                } else {
                    0 as std::ffi::c_int
                },
            );
        }
        LUA_TSTRING => {
            return sqlite3_bind_text(
                stmt,
                bidx,
                lua_tolstring(L, idx, NULL as *mut usize),
                -(1 as std::ffi::c_int),
                ::core::mem::transmute::<
                    libc::intptr_t,
                    sqlite3_destructor_type,
                >(SQLITE_TRANSIENT as libc::intptr_t),
            );
        }
        _ => {
            _log(
                LOG_LEVEL_warn,
                b"clib/sqlite3.c\0" as *const u8 as *const std::ffi::c_char,
                b"sqlite3: unable to bind Lua value (type %s)\0" as *const u8
                    as *const std::ffi::c_char,
                lua_typename(L, lua_type(L, idx)),
            );
        }
    }
    return SQLITE_OK;
}
unsafe extern "C" fn luaH_sqlite3_do_exec(
    mut L: *mut lua_State,
    mut stmt: *mut sqlite3_stmt,
) -> gint {
    let mut ret = sqlite3_step(stmt);
    let mut rows = 0;
    let mut ncol = 0 as std::ffi::c_int;
    if ret == SQLITE_DONE || ret == SQLITE_ROW {
        ncol = sqlite3_column_count(stmt);
        if ncol != 0 {
            lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
        } else {
            lua_pushnil(L);
        }
    }
    loop {
        match ret {
            SQLITE_DONE => return 1 as std::ffi::c_int,
            SQLITE_ROW => {
                lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
                let mut i = 0 as std::ffi::c_int;
                while i < ncol {
                    lua_pushstring(L, sqlite3_column_name(stmt, i));
                    match sqlite3_column_type(stmt, i) {
                        SQLITE_INTEGER | SQLITE_FLOAT => {
                            lua_pushnumber(L, sqlite3_column_double(stmt, i));
                            lua_rawset(L, -(3 as std::ffi::c_int));
                        }
                        SQLITE_BLOB | SQLITE_TEXT => {
                            lua_pushstring(
                                L,
                                sqlite3_column_blob(stmt, i) as *const std::ffi::c_char,
                            );
                            lua_rawset(L, -(3 as std::ffi::c_int));
                        }
                        SQLITE_NULL | _ => {
                            lua_settop(
                                L,
                                -(1 as std::ffi::c_int) - 1 as std::ffi::c_int,
                            );
                        }
                    }
                    i += 1;
                    i;
                }
                rows += 1;
                lua_rawseti(L, -(2 as std::ffi::c_int), rows);
                ret = sqlite3_step(stmt);
            }
            _ => return -(1 as std::ffi::c_int),
        }
    };
}
unsafe extern "C" fn luaH_sqlite3_exec(mut L: *mut lua_State) -> gint {
    let mut sqlite = luaH_checkudata(L, 1 as std::ffi::c_int, &mut sqlite3_class)
        as *mut sqlite3_t;
    luaH_sqlite3_checkopen(L, sqlite);
    let mut sql = luaL_checklstring(L, 2 as std::ffi::c_int, NULL as *mut size_t);
    let mut tail = 0 as *const gchar;
    if !(lua_type(L, 3 as std::ffi::c_int) <= 0 as std::ffi::c_int) {
        if !(lua_type(L, 3 as std::ffi::c_int) == LUA_TTABLE) {
            luaL_typerror(
                L,
                3 as std::ffi::c_int,
                b"table\0" as *const u8 as *const std::ffi::c_char,
            );
        }
    }
    let mut top = lua_gettop(L);
    let mut ret = 0 as std::ffi::c_int;
    let mut stmt = 0 as *mut sqlite3_stmt;
    loop {
        if sqlite3_prepare_v2(
            (*sqlite).db,
            sql,
            -(1 as std::ffi::c_int),
            &mut stmt,
            &mut tail,
        ) != 0
        {
            lua_pushfstring(
                L,
                b"sqlite3: statement compilation failed (%s)\0" as *const u8
                    as *const std::ffi::c_char,
                sqlite3_errmsg((*sqlite).db),
            );
            sqlite3_finalize(stmt);
            lua_error(L);
        } else if stmt.is_null() {
            return 0 as std::ffi::c_int
        }
        if !(lua_type(L, 3 as std::ffi::c_int) <= 0 as std::ffi::c_int) {
            lua_pushnil(L);
            let mut idx: gint = 0;
            while lua_next(L, 3 as std::ffi::c_int) != 0 {
                idx = luaH_param_index(L, stmt, -(2 as std::ffi::c_int));
                if idx == 0 as std::ffi::c_int {
                    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
                } else {
                    ret = luaH_bind_value(L, stmt, idx, -(1 as std::ffi::c_int));
                    if !(ret == SQLITE_OK || ret == SQLITE_RANGE) {
                        lua_pushfstring(
                            L,
                            b"sqlite3: sqlite3_bind_* failed (%s)\0" as *const u8
                                as *const std::ffi::c_char,
                            sqlite3_errmsg((*sqlite).db),
                        );
                        sqlite3_finalize(stmt);
                        lua_error(L);
                    }
                    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
                }
            }
        }
        ret = luaH_sqlite3_do_exec(L, stmt);
        sqlite3_finalize(stmt);
        if ret == -(1 as std::ffi::c_int) {
            lua_pushfstring(
                L,
                b"sqlite3: exec error (%s)\0" as *const u8 as *const std::ffi::c_char,
                sqlite3_errmsg((*sqlite).db),
            );
            lua_error(L);
        }
        if !(!tail.is_null() && *tail as std::ffi::c_int != 0) {
            break;
        }
        sql = tail;
        lua_settop(L, top);
    }
    return 1 as std::ffi::c_int;
}
unsafe extern "C" fn luaH_sqlite3_stmt_exec(mut L: *mut lua_State) -> gint {
    let mut stmt = luaH_checkudata(L, 1 as std::ffi::c_int, &mut sqlite3_stmt_class)
        as *mut sqlite3_stmt_t;
    let mut sqlite = (*stmt).sqlite;
    luaH_sqlite3_checkopen(L, sqlite);
    sqlite3_reset((*stmt).stmt);
    let mut ret: gint = 0;
    if !(lua_type(L, 2 as std::ffi::c_int) <= 0 as std::ffi::c_int) {
        if !(lua_type(L, 2 as std::ffi::c_int) == LUA_TTABLE) {
            luaL_typerror(
                L,
                2 as std::ffi::c_int,
                b"table\0" as *const u8 as *const std::ffi::c_char,
            );
        }
        sqlite3_clear_bindings((*stmt).stmt);
        lua_pushnil(L);
        let mut idx: gint = 0;
        while lua_next(L, 2 as std::ffi::c_int) != 0 {
            idx = luaH_param_index(L, (*stmt).stmt, -(2 as std::ffi::c_int));
            if idx == 0 as std::ffi::c_int {
                lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
            } else {
                ret = luaH_bind_value(L, (*stmt).stmt, idx, -(1 as std::ffi::c_int));
                if !(ret == SQLITE_OK || ret == SQLITE_RANGE) {
                    lua_pushfstring(
                        L,
                        b"sqlite3: sqlite3_bind_* failed (%s)\0" as *const u8
                            as *const std::ffi::c_char,
                        sqlite3_errmsg((*sqlite).db),
                    );
                    sqlite3_finalize((*stmt).stmt);
                    lua_error(L);
                }
                lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
            }
        }
    }
    ret = luaH_sqlite3_do_exec(L, (*stmt).stmt);
    if ret == -(1 as std::ffi::c_int) {
        lua_pushfstring(
            L,
            b"sqlite3: exec error (%s)\0" as *const u8 as *const std::ffi::c_char,
            sqlite3_errmsg((*sqlite).db),
        );
        lua_error(L);
    }
    return 1 as std::ffi::c_int;
}
unsafe extern "C" fn luaH_sqlite3_new(mut L: *mut lua_State) -> gint {
    luaH_class_new(L, &mut sqlite3_class);
    let mut sqlite = luaH_checkudata(L, -(1 as std::ffi::c_int), &mut sqlite3_class)
        as *mut sqlite3_t;
    if ((*sqlite).db).is_null() {
        lua_pushlstring(
            L,
            b"sqlite3: database not opened, forgot filename?\0" as *const u8
                as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 47]>())
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>(),
                )
                .wrapping_sub(1),
        );
        lua_error(L);
    }
    return 1 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_class_setup(mut L: *mut lua_State) {
    static mut sqlite3_methods: [luaL_Reg; 5] = unsafe {
        [
            {
                let mut init = luaL_Reg {
                    name: b"add_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_sqlite3_class_add_signal
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"remove_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_sqlite3_class_remove_signal
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"emit_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_sqlite3_class_emit_signal
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__call\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_sqlite3_new as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: NULL as *const std::ffi::c_char,
                    func: ::core::mem::transmute::<
                        libc::intptr_t,
                        Function,
                    >(NULL as libc::intptr_t),
                };
                init
            },
        ]
    };
    static mut sqlite3_meta: [luaL_Reg; 13] = unsafe {
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
                    name: b"exec\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_sqlite3_exec as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"close\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_sqlite3_close
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"compile\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_sqlite3_compile
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"changes\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_sqlite3_changes
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__gc\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_sqlite3_gc as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: NULL as *const std::ffi::c_char,
                    func: ::core::mem::transmute::<
                        libc::intptr_t,
                        Function,
                    >(NULL as libc::intptr_t),
                };
                init
            },
        ]
    };
    luaH_class_setup(
        L,
        &mut sqlite3_class,
        b"sqlite3\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut lua_State) -> *mut sqlite3_t>,
            lua_class_allocator_t,
        >(Some(sqlite3_new as unsafe extern "C" fn(*mut lua_State) -> *mut sqlite3_t)),
        ::core::mem::transmute::<
            libc::intptr_t,
            lua_class_propfunc_t,
        >(NULL as libc::intptr_t),
        ::core::mem::transmute::<
            libc::intptr_t,
            lua_class_propfunc_t,
        >(NULL as libc::intptr_t),
        sqlite3_methods.as_ptr(),
        sqlite3_meta.as_ptr(),
    );
    luaH_class_add_property(
        &mut sqlite3_class,
        L_TK_FILENAME,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut lua_State, *mut sqlite3_t) -> gint>,
            lua_class_propfunc_t,
        >(
            Some(
                luaH_sqlite3_set_filename
                    as unsafe extern "C" fn(*mut lua_State, *mut sqlite3_t) -> gint,
            ),
        ),
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut lua_State, *mut sqlite3_t) -> gint>,
            lua_class_propfunc_t,
        >(
            Some(
                luaH_sqlite3_get_filename
                    as unsafe extern "C" fn(*mut lua_State, *mut sqlite3_t) -> gint,
            ),
        ),
        ::core::mem::transmute::<
            libc::intptr_t,
            lua_class_propfunc_t,
        >(NULL as libc::intptr_t),
    );
    static mut sqlite3_stmt_meta: [luaL_Reg; 3] = unsafe {
        [
            {
                let mut init = luaL_Reg {
                    name: b"exec\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_sqlite3_stmt_exec
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__gc\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_sqlite3_stmt_gc
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
                        Function,
                    >(NULL as libc::intptr_t),
                };
                init
            },
        ]
    };
    luaH_class_setup(
        L,
        &mut sqlite3_stmt_class,
        b"sqlite3::statement\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            libc::intptr_t,
            lua_class_allocator_t,
        >(NULL as libc::intptr_t),
        ::core::mem::transmute::<
            libc::intptr_t,
            lua_class_propfunc_t,
        >(NULL as libc::intptr_t),
        ::core::mem::transmute::<
            libc::intptr_t,
            lua_class_propfunc_t,
        >(NULL as libc::intptr_t),
        NULL as *const luaL_Reg,
        sqlite3_stmt_meta.as_ptr(),
    );
}
