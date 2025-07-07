use ::libc;
use ::c2rust_bitfields;
use lua::ffi::{lua_State, lua_gettop, lua_pushstring, lua_type, LUA_MULTRET, LUA_TNIL};
pub mod __stddef_size_t_h {
    pub type size_t = std::ffi::c_ulong;
}
pub mod glibconfig_h {
    pub type guint32 = std::ffi::c_uint;
    pub type gsize = std::ffi::c_ulong;
}
pub mod types_h {
    pub type __mode_t = std::ffi::c_uint;
    pub type __off_t = std::ffi::c_long;
    pub type __off64_t = std::ffi::c_long;
}
pub mod gtypes_h {
    pub type gchar = std::ffi::c_char;
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
    unsafe extern "C" {
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
pub mod gregex_h {
    pub type GRegexCompileFlags = std::ffi::c_uint;
    pub const G_REGEX_JAVASCRIPT_COMPAT: GRegexCompileFlags = 33554432;
    pub const G_REGEX_BSR_ANYCRLF: GRegexCompileFlags = 8388608;
    pub const G_REGEX_NEWLINE_ANYCRLF: GRegexCompileFlags = 5242880;
    pub const G_REGEX_NEWLINE_CRLF: GRegexCompileFlags = 3145728;
    pub const G_REGEX_NEWLINE_LF: GRegexCompileFlags = 2097152;
    pub const G_REGEX_NEWLINE_CR: GRegexCompileFlags = 1048576;
    pub const G_REGEX_DUPNAMES: GRegexCompileFlags = 524288;
    pub const G_REGEX_FIRSTLINE: GRegexCompileFlags = 262144;
    pub const G_REGEX_OPTIMIZE: GRegexCompileFlags = 8192;
    pub const G_REGEX_NO_AUTO_CAPTURE: GRegexCompileFlags = 4096;
    pub const G_REGEX_RAW: GRegexCompileFlags = 2048;
    pub const G_REGEX_UNGREEDY: GRegexCompileFlags = 512;
    pub const G_REGEX_DOLLAR_ENDONLY: GRegexCompileFlags = 32;
    pub const G_REGEX_ANCHORED: GRegexCompileFlags = 16;
    pub const G_REGEX_EXTENDED: GRegexCompileFlags = 8;
    pub const G_REGEX_DOTALL: GRegexCompileFlags = 4;
    pub const G_REGEX_MULTILINE: GRegexCompileFlags = 2;
    pub const G_REGEX_CASELESS: GRegexCompileFlags = 1;
    pub const G_REGEX_DEFAULT: GRegexCompileFlags = 0;
    pub type GRegexMatchFlags = std::ffi::c_uint;
    pub const G_REGEX_MATCH_NOTEMPTY_ATSTART: GRegexMatchFlags = 268435456;
    pub const G_REGEX_MATCH_PARTIAL_HARD: GRegexMatchFlags = 134217728;
    pub const G_REGEX_MATCH_PARTIAL_SOFT: GRegexMatchFlags = 32768;
    pub const G_REGEX_MATCH_BSR_ANY: GRegexMatchFlags = 16777216;
    pub const G_REGEX_MATCH_BSR_ANYCRLF: GRegexMatchFlags = 8388608;
    pub const G_REGEX_MATCH_NEWLINE_ANYCRLF: GRegexMatchFlags = 5242880;
    pub const G_REGEX_MATCH_NEWLINE_ANY: GRegexMatchFlags = 4194304;
    pub const G_REGEX_MATCH_NEWLINE_CRLF: GRegexMatchFlags = 3145728;
    pub const G_REGEX_MATCH_NEWLINE_LF: GRegexMatchFlags = 2097152;
    pub const G_REGEX_MATCH_NEWLINE_CR: GRegexMatchFlags = 1048576;
    pub const G_REGEX_MATCH_PARTIAL: GRegexMatchFlags = 32768;
    pub const G_REGEX_MATCH_NOTEMPTY: GRegexMatchFlags = 1024;
    pub const G_REGEX_MATCH_NOTEOL: GRegexMatchFlags = 256;
    pub const G_REGEX_MATCH_NOTBOL: GRegexMatchFlags = 128;
    pub const G_REGEX_MATCH_ANCHORED: GRegexMatchFlags = 16;
    pub const G_REGEX_MATCH_DEFAULT: GRegexMatchFlags = 0;
    pub type GRegex = _GRegex;
    pub type GMatchInfo = _GMatchInfo;
    use super::gtypes_h::{gchar, gboolean};
    use super::gerror_h::GError;
    unsafe extern "C" {
        pub type _GRegex;
        pub type _GMatchInfo;
        pub fn g_regex_new(
            pattern: *const gchar,
            compile_options: GRegexCompileFlags,
            match_options: GRegexMatchFlags,
            error: *mut *mut GError,
        ) -> *mut GRegex;
        pub fn g_regex_match(
            regex: *const GRegex,
            string: *const gchar,
            match_options: GRegexMatchFlags,
            match_info: *mut *mut GMatchInfo,
        ) -> gboolean;
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
pub mod guri_h {
    pub type GUri = _GUri;
    pub type GUriFlags = std::ffi::c_uint;
    pub const G_URI_FLAGS_SCHEME_NORMALIZE: GUriFlags = 256;
    pub const G_URI_FLAGS_ENCODED_FRAGMENT: GUriFlags = 128;
    pub const G_URI_FLAGS_ENCODED_PATH: GUriFlags = 64;
    pub const G_URI_FLAGS_ENCODED_QUERY: GUriFlags = 32;
    pub const G_URI_FLAGS_NON_DNS: GUriFlags = 16;
    pub const G_URI_FLAGS_ENCODED: GUriFlags = 8;
    pub const G_URI_FLAGS_HAS_AUTH_PARAMS: GUriFlags = 4;
    pub const G_URI_FLAGS_HAS_PASSWORD: GUriFlags = 2;
    pub const G_URI_FLAGS_PARSE_RELAXED: GUriFlags = 1;
    pub const G_URI_FLAGS_NONE: GUriFlags = 0;
    use super::gtypes_h::{gchar, gint};
    use super::gerror_h::GError;
    unsafe extern "C" {
        pub type _GUri;
        pub fn g_uri_unref(uri: *mut GUri);
        pub fn g_uri_join_with_user(
            flags: GUriFlags,
            scheme: *const gchar,
            user: *const gchar,
            password: *const gchar,
            auth_params: *const gchar,
            host: *const gchar,
            port: gint,
            path: *const gchar,
            query: *const gchar,
            fragment: *const gchar,
        ) -> *mut gchar;
        pub fn g_uri_parse(
            uri_string: *const gchar,
            flags: GUriFlags,
            error: *mut *mut GError,
        ) -> *mut GUri;
        pub fn g_uri_get_scheme(uri: *mut GUri) -> *const gchar;
        pub fn g_uri_get_user(uri: *mut GUri) -> *const gchar;
        pub fn g_uri_get_password(uri: *mut GUri) -> *const gchar;
        pub fn g_uri_get_host(uri: *mut GUri) -> *const gchar;
        pub fn g_uri_get_port(uri: *mut GUri) -> gint;
        pub fn g_uri_get_path(uri: *mut GUri) -> *const gchar;
        pub fn g_uri_get_query(uri: *mut GUri) -> *const gchar;
        pub fn g_uri_get_fragment(uri: *mut GUri) -> *const gchar;
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
    use super::gtype_h::GTypeInstance;
    use super::gtypes_h::guint;
    use super::gdataset_h::GData;
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
            NULL_0 as *mut std::ffi::c_void,
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
    use super::__stddef_null_h::NULL_0;
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
    unsafe extern "C" {
        pub fn l_tokenize(_: *const gchar) -> luakit_token_t;
    }
}
pub mod struct_FILE_h {
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    pub struct _IO_FILE {
        pub _flags: std::ffi::c_int,
        pub _IO_read_ptr: *mut std::ffi::c_char,
        pub _IO_read_end: *mut std::ffi::c_char,
        pub _IO_read_base: *mut std::ffi::c_char,
        pub _IO_write_base: *mut std::ffi::c_char,
        pub _IO_write_ptr: *mut std::ffi::c_char,
        pub _IO_write_end: *mut std::ffi::c_char,
        pub _IO_buf_base: *mut std::ffi::c_char,
        pub _IO_buf_end: *mut std::ffi::c_char,
        pub _IO_save_base: *mut std::ffi::c_char,
        pub _IO_backup_base: *mut std::ffi::c_char,
        pub _IO_save_end: *mut std::ffi::c_char,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: std::ffi::c_int,
        #[bitfield(name = "_flags2", ty = "std::ffi::c_int", bits = "0..=23")]
        pub _flags2: [u8; 3],
        pub _short_backupbuf: [std::ffi::c_char; 1],
        pub _old_offset: __off_t,
        pub _cur_column: std::ffi::c_ushort,
        pub _vtable_offset: std::ffi::c_schar,
        pub _shortbuf: [std::ffi::c_char; 1],
        pub _lock: *mut std::ffi::c_void,
        pub _offset: __off64_t,
        pub _codecvt: *mut _IO_codecvt,
        pub _wide_data: *mut _IO_wide_data,
        pub _freeres_list: *mut _IO_FILE,
        pub _freeres_buf: *mut std::ffi::c_void,
        pub _prevchain: *mut *mut _IO_FILE,
        pub _mode: std::ffi::c_int,
        pub _unused2: [std::ffi::c_char; 20],
    }
    pub type _IO_lock_t = ();
    use c2rust_bitfields::BitfieldStruct;

    use super::types_h::{__off_t, __off64_t};
    unsafe extern "C" {
        pub type _IO_wide_data;
        pub type _IO_codecvt;
        pub type _IO_marker;
    }
}
pub mod FILE_h {
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
pub mod lauxlib_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct luaL_Reg {
        pub name: *const std::ffi::c_char,
        pub func: Function,
    }
    use lua::{ffi::lua_State, Function};

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
        pub fn luaL_error(
            L: *mut lua_State,
            fmt: *const std::ffi::c_char,
            _: ...
        ) -> std::ffi::c_int;
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
    use lua::ffi::lua_State;

    use super::ghash_h::GHashTable;
    use super::signal_h::signal_t;
    use super::gtypes_h::{gint, gchar};
    use super::lauxlib_h::luaL_Reg;
    unsafe extern "C" {
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
        pub fn luaH_openlib(
            _: *mut lua_State,
            _: *const gchar,
            _: *const luaL_Reg,
            _: *const luaL_Reg,
        );
    }
}
pub mod WebKitCookieManager_h {
    pub type WebKitCookiePersistentStorage = std::ffi::c_uint;
    pub const WEBKIT_COOKIE_PERSISTENT_STORAGE_SQLITE: WebKitCookiePersistentStorage = 1;
    pub const WEBKIT_COOKIE_PERSISTENT_STORAGE_TEXT: WebKitCookiePersistentStorage = 0;
    pub type WebKitCookieManager = _WebKitCookieManager;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _WebKitCookieManager {
        pub parent: GObject,
        pub priv_0: *mut WebKitCookieManagerPrivate,
    }
    pub type WebKitCookieManagerPrivate = _WebKitCookieManagerPrivate;
    pub type WebKitCookieAcceptPolicy = std::ffi::c_uint;
    pub const WEBKIT_COOKIE_POLICY_ACCEPT_NO_THIRD_PARTY: WebKitCookieAcceptPolicy = 2;
    pub const WEBKIT_COOKIE_POLICY_ACCEPT_NEVER: WebKitCookieAcceptPolicy = 1;
    pub const WEBKIT_COOKIE_POLICY_ACCEPT_ALWAYS: WebKitCookieAcceptPolicy = 0;
    use super::gobject_h::GObject;
    use super::gtypes_h::gchar;
    unsafe extern "C" {
        pub type _WebKitCookieManagerPrivate;
        pub fn webkit_cookie_manager_set_persistent_storage(
            cookie_manager: *mut WebKitCookieManager,
            filename: *const gchar,
            storage: WebKitCookiePersistentStorage,
        );
        pub fn webkit_cookie_manager_set_accept_policy(
            cookie_manager: *mut WebKitCookieManager,
            policy: WebKitCookieAcceptPolicy,
        );
    }
}
pub mod WebKitWebContext_h {
    pub type WebKitWebContext = _WebKitWebContext;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _WebKitWebContext {
        pub parent: GObject,
        pub priv_0: *mut WebKitWebContextPrivate,
    }
    pub type WebKitWebContextPrivate = _WebKitWebContextPrivate;
    use super::gobject_h::GObject;
    use super::WebKitWebsiteDataManager_h::WebKitWebsiteDataManager;
    use super::WebKitCookieManager_h::WebKitCookieManager;
    unsafe extern "C" {
        pub type _WebKitWebContextPrivate;
        pub fn webkit_web_context_get_website_data_manager(
            context: *mut WebKitWebContext,
        ) -> *mut WebKitWebsiteDataManager;
        pub fn webkit_web_context_get_cookie_manager(
            context: *mut WebKitWebContext,
        ) -> *mut WebKitCookieManager;
    }
}
pub mod WebKitNetworkProxySettings_h {
    pub type WebKitNetworkProxySettings = _WebKitNetworkProxySettings;
    pub type WebKitNetworkProxyMode = std::ffi::c_uint;
    pub const WEBKIT_NETWORK_PROXY_MODE_CUSTOM: WebKitNetworkProxyMode = 2;
    pub const WEBKIT_NETWORK_PROXY_MODE_NO_PROXY: WebKitNetworkProxyMode = 1;
    pub const WEBKIT_NETWORK_PROXY_MODE_DEFAULT: WebKitNetworkProxyMode = 0;
    use super::gtypes_h::gchar;
    unsafe extern "C" {
        pub type _WebKitNetworkProxySettings;
        pub fn webkit_network_proxy_settings_new(
            default_proxy_uri: *const gchar,
            ignore_hosts: *const *const gchar,
        ) -> *mut WebKitNetworkProxySettings;
        pub fn webkit_network_proxy_settings_free(
            proxy_settings: *mut WebKitNetworkProxySettings,
        );
    }
}
pub mod WebKitWebsiteDataManager_h {
    pub type WebKitWebsiteDataManager = _WebKitWebsiteDataManager;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _WebKitWebsiteDataManager {
        pub parent: GObject,
        pub priv_0: *mut WebKitWebsiteDataManagerPrivate,
    }
    pub type WebKitWebsiteDataManagerPrivate = _WebKitWebsiteDataManagerPrivate;
    use super::gobject_h::GObject;
    use super::WebKitNetworkProxySettings_h::{
        WebKitNetworkProxyMode, WEBKIT_NETWORK_PROXY_MODE_DEFAULT,
        WebKitNetworkProxySettings,
    };
    unsafe extern "C" {
        pub type _WebKitWebsiteDataManagerPrivate;
        pub fn webkit_website_data_manager_set_network_proxy_settings(
            manager: *mut WebKitWebsiteDataManager,
            proxy_mode: WebKitNetworkProxyMode,
            proxy_settings: *mut WebKitNetworkProxySettings,
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
        pub fn g_strdup_printf(format: *const gchar, _: ...) -> *mut gchar;
    }
}
pub mod gmessages_h {
    pub const G_LOG_DOMAIN: std::ffi::c_int = 0 as std::ffi::c_int;
}
pub mod gtestutils_h {
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
    }
}
pub mod stdio_h {
    use super::FILE_h::FILE;
    unsafe extern "C" {
        pub fn fclose(__stream: *mut FILE) -> std::ffi::c_int;
        pub fn fopen(
            _: *const std::ffi::c_char,
            _: *const std::ffi::c_char,
        ) -> *mut FILE;
    }
}
pub mod soup_uri_utils_h {
    pub const SOUP_HTTP_URI_FLAGS: std::ffi::c_int = G_URI_FLAGS_HAS_PASSWORD
        as std::ffi::c_int | G_URI_FLAGS_ENCODED_PATH as std::ffi::c_int
        | G_URI_FLAGS_ENCODED_QUERY as std::ffi::c_int
        | G_URI_FLAGS_ENCODED_FRAGMENT as std::ffi::c_int
        | G_URI_FLAGS_SCHEME_NORMALIZE as std::ffi::c_int;
    use super::guri_h::{
        G_URI_FLAGS_HAS_PASSWORD, G_URI_FLAGS_ENCODED_PATH, G_URI_FLAGS_ENCODED_QUERY,
        G_URI_FLAGS_ENCODED_FRAGMENT, G_URI_FLAGS_SCHEME_NORMALIZE,
    };
}
pub mod web_context_h {
    use super::WebKitWebContext_h::WebKitWebContext;
    unsafe extern "C" {
        pub fn web_context_get() -> *mut WebKitWebContext;
    }
}
pub mod stat_h {
    use super::types_h::__mode_t;
    unsafe extern "C" {
        pub fn chmod(
            __file: *const std::ffi::c_char,
            __mode: __mode_t,
        ) -> std::ffi::c_int;
    }
}
pub mod gstdio_h {
    pub const g_chmod: unsafe extern "C" fn(
        *const std::ffi::c_char,
        __mode_t,
    ) -> std::ffi::c_int = chmod;
    pub const g_fopen: unsafe extern "C" fn(
        *const std::ffi::c_char,
        *const std::ffi::c_char,
    ) -> *mut FILE = fopen;
    use super::stat_h::chmod;
    use super::types_h::__mode_t;
    use super::stdio_h::fopen;
    use super::FILE_h::FILE;
}
pub mod soup_h {
    pub static mut scheme_reg: *mut GRegex = 0 as *const GRegex as *mut GRegex;
    pub unsafe extern "C" fn luaH_soup_uri_tostring(mut L: *mut lua_State) -> gint {
        let mut p = 0 as *const gchar;
        let mut port: gint = 0;
        if !(lua_type(L, 1 as std::ffi::c_int) == LUA_TTABLE) {
            luaL_typerror(
                L,
                1 as std::ffi::c_int,
                b"table\0" as *const u8 as *const std::ffi::c_char,
            );
        }
        let mut scheme = b"http\0" as *const u8 as *const std::ffi::c_char;
        let mut user = NULL_1 as *const gchar;
        let mut host = NULL_1 as *const gchar;
        let mut path = NULL_1 as *const gchar;
        let mut query = NULL_1 as *const gchar;
        let mut fragment = NULL_1 as *const gchar;
        let mut uri = 0 as *mut gchar;
        lua_pushlstring(
            L,
            b"scheme\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 7]>())
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>(),
                )
                .wrapping_sub(1),
        );
        lua_rawget(L, 1 as std::ffi::c_int);
        if !(lua_type(L, -(1 as std::ffi::c_int)) == LUA_TNIL)
            && {
                p = lua_tolstring(L, -(1 as std::ffi::c_int), NULL_1 as *mut usize);
                !p.is_null()
            } && *p.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int != 0
        {
            scheme = p;
        }
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        if g_strcmp0(scheme, b"file\0" as *const u8 as *const std::ffi::c_char) == 0 {
            host = b"\0" as *const u8 as *const std::ffi::c_char;
        }
        lua_pushlstring(
            L,
            b"user\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 5]>())
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>(),
                )
                .wrapping_sub(1),
        );
        lua_rawget(L, 1 as std::ffi::c_int);
        if !(lua_type(L, -(1 as std::ffi::c_int)) == LUA_TNIL)
            && {
                p = lua_tolstring(L, -(1 as std::ffi::c_int), NULL_1 as *mut usize);
                !p.is_null()
            } && *p.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int != 0
        {
            user = p;
        }
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        lua_pushlstring(
            L,
            b"host\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 5]>())
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>(),
                )
                .wrapping_sub(1),
        );
        lua_rawget(L, 1 as std::ffi::c_int);
        if !(lua_type(L, -(1 as std::ffi::c_int)) == LUA_TNIL)
            && {
                p = lua_tolstring(L, -(1 as std::ffi::c_int), NULL_1 as *mut usize);
                !p.is_null()
            } && *p.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int != 0
        {
            host = p;
        }
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        lua_pushlstring(
            L,
            b"path\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 5]>())
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>(),
                )
                .wrapping_sub(1),
        );
        lua_rawget(L, 1 as std::ffi::c_int);
        if !(lua_type(L, -(1 as std::ffi::c_int)) == LUA_TNIL)
            && {
                p = lua_tolstring(L, -(1 as std::ffi::c_int), NULL_1 as *mut usize);
                !p.is_null()
            } && *p.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int != 0
        {
            path = p;
        }
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        lua_pushlstring(
            L,
            b"query\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 6]>())
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>(),
                )
                .wrapping_sub(1),
        );
        lua_rawget(L, 1 as std::ffi::c_int);
        if !(lua_type(L, -(1 as std::ffi::c_int)) == LUA_TNIL)
            && {
                p = lua_tolstring(L, -(1 as std::ffi::c_int), NULL_1 as *mut usize);
                !p.is_null()
            } && *p.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int != 0
        {
            query = p;
        }
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        lua_pushlstring(
            L,
            b"fragment\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 9]>())
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>(),
                )
                .wrapping_sub(1),
        );
        lua_rawget(L, 1 as std::ffi::c_int);
        if !(lua_type(L, -(1 as std::ffi::c_int)) == LUA_TNIL)
            && {
                p = lua_tolstring(L, -(1 as std::ffi::c_int), NULL_1 as *mut usize);
                !p.is_null()
            } && *p.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int != 0
        {
            fragment = p;
        }
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        lua_pushlstring(
            L,
            b"port\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 5]>())
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>(),
                )
                .wrapping_sub(1),
        );
        lua_rawget(L, 1 as std::ffi::c_int);
        if lua_type(L, -(1 as std::ffi::c_int)) == LUA_TNIL
            || {
                port = lua_tonumber(L, -(1 as std::ffi::c_int)) as gint;
                port == 0
            }
        {
            port = -(1 as std::ffi::c_int);
        }
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        uri = g_uri_join_with_user(
            SOUP_HTTP_URI_FLAGS as GUriFlags,
            scheme,
            user,
            NULL_1 as *const gchar,
            NULL_1 as *const gchar,
            host,
            port,
            path,
            query,
            fragment,
        );
        lua_pushstring(L, uri);
        g_free(uri as gpointer);
        return 1 as std::ffi::c_int;
    }
    pub unsafe extern "C" fn luaH_soup_push_uri(
        mut L: *mut lua_State,
        mut uri: *mut GUri,
    ) -> gint {
        let mut p = 0 as *const gchar;
        let mut port: gint = 0;
        lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
        p = g_uri_get_scheme(uri);
        if !p.is_null()
            && *p.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int != 0
        {
            lua_pushlstring(
                L,
                b"scheme\0" as *const u8 as *const std::ffi::c_char,
                (::core::mem::size_of::<[std::ffi::c_char; 7]>())
                    .wrapping_div(
                        ::core::mem::size_of::<std::ffi::c_char>(),
                    )
                    .wrapping_sub(1),
            );
            lua_pushstring(L, p);
            lua_rawset(L, -(3 as std::ffi::c_int));
        }
        p = g_uri_get_user(uri);
        if !p.is_null()
            && *p.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int != 0
        {
            lua_pushlstring(
                L,
                b"user\0" as *const u8 as *const std::ffi::c_char,
                (::core::mem::size_of::<[std::ffi::c_char; 5]>())
                    .wrapping_div(
                        ::core::mem::size_of::<std::ffi::c_char>(),
                    )
                    .wrapping_sub(1),
            );
            lua_pushstring(L, p);
            lua_rawset(L, -(3 as std::ffi::c_int));
        }
        p = g_uri_get_password(uri);
        if !p.is_null()
            && *p.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int != 0
        {
            lua_pushlstring(
                L,
                b"password\0" as *const u8 as *const std::ffi::c_char,
                (::core::mem::size_of::<[std::ffi::c_char; 9]>())
                    .wrapping_div(
                        ::core::mem::size_of::<std::ffi::c_char>(),
                    )
                    .wrapping_sub(1),
            );
            lua_pushstring(L, p);
            lua_rawset(L, -(3 as std::ffi::c_int));
        }
        p = g_uri_get_host(uri);
        if !p.is_null()
            && *p.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int != 0
        {
            lua_pushlstring(
                L,
                b"host\0" as *const u8 as *const std::ffi::c_char,
                (::core::mem::size_of::<[std::ffi::c_char; 5]>())
                    .wrapping_div(
                        ::core::mem::size_of::<std::ffi::c_char>(),
                    )
                    .wrapping_sub(1),
            );
            lua_pushstring(L, p);
            lua_rawset(L, -(3 as std::ffi::c_int));
        }
        p = g_uri_get_path(uri);
        if !p.is_null()
            && *p.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int != 0
        {
            lua_pushlstring(
                L,
                b"path\0" as *const u8 as *const std::ffi::c_char,
                (::core::mem::size_of::<[std::ffi::c_char; 5]>())
                    .wrapping_div(
                        ::core::mem::size_of::<std::ffi::c_char>(),
                    )
                    .wrapping_sub(1),
            );
            lua_pushstring(L, p);
            lua_rawset(L, -(3 as std::ffi::c_int));
        }
        p = g_uri_get_query(uri);
        if !p.is_null()
            && *p.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int != 0
        {
            lua_pushlstring(
                L,
                b"query\0" as *const u8 as *const std::ffi::c_char,
                (::core::mem::size_of::<[std::ffi::c_char; 6]>())
                    .wrapping_div(
                        ::core::mem::size_of::<std::ffi::c_char>(),
                    )
                    .wrapping_sub(1),
            );
            lua_pushstring(L, p);
            lua_rawset(L, -(3 as std::ffi::c_int));
        }
        p = g_uri_get_fragment(uri);
        if !p.is_null()
            && *p.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int != 0
        {
            lua_pushlstring(
                L,
                b"fragment\0" as *const u8 as *const std::ffi::c_char,
                (::core::mem::size_of::<[std::ffi::c_char; 9]>())
                    .wrapping_div(
                        ::core::mem::size_of::<std::ffi::c_char>(),
                    )
                    .wrapping_sub(1),
            );
            lua_pushstring(L, p);
            lua_rawset(L, -(3 as std::ffi::c_int));
        }
        port = g_uri_get_port(uri);
        if port > 0 as std::ffi::c_int {
            lua_pushlstring(
                L,
                b"port\0" as *const u8 as *const std::ffi::c_char,
                (::core::mem::size_of::<[std::ffi::c_char; 5]>())
                    .wrapping_div(
                        ::core::mem::size_of::<std::ffi::c_char>(),
                    )
                    .wrapping_sub(1),
            );
            lua_pushnumber(L, port as Number);
            lua_rawset(L, -(3 as std::ffi::c_int));
        }
        return 1 as std::ffi::c_int;
    }
    pub unsafe extern "C" fn luaH_soup_parse_uri(mut L: *mut lua_State) -> gint {
        let mut str = luaL_checklstring(L, 1 as std::ffi::c_int, NULL_1 as *mut size_t)
            as *mut gchar;
        if *str.offset(0 as std::ffi::c_int as isize) == 0 {
            return 0 as std::ffi::c_int;
        }
        if g_regex_match(
            scheme_reg,
            str,
            G_REGEX_MATCH_DEFAULT,
            0 as *mut *mut GMatchInfo,
        ) == 0
        {
            str = g_strdup_printf(
                b"http://%s\0" as *const u8 as *const std::ffi::c_char,
                str,
            );
        } else {
            str = g_strdup_inline(str);
        }
        let mut uri = g_uri_parse(
            str,
            SOUP_HTTP_URI_FLAGS as GUriFlags,
            NULL_1 as *mut *mut GError,
        );
        g_free(str as gpointer);
        if !uri.is_null() {
            luaH_soup_push_uri(L, uri);
            g_uri_unref(uri);
            return 1 as std::ffi::c_int;
        }
        return 0 as std::ffi::c_int;
    }
    pub unsafe extern "C" fn soup_lib_setup_common() {
        scheme_reg = g_regex_new(
            b"^[a-z][a-z0-9\\+\\-\\.]*:\0" as *const u8 as *const std::ffi::c_char,
            G_REGEX_DEFAULT,
            G_REGEX_MATCH_DEFAULT,
            NULL_1 as *mut *mut GError,
        );
    }
    use lua::ffi::{lua_State, lua_createtable, lua_pushlstring, lua_pushnumber, lua_pushstring, lua_rawget, lua_rawset, lua_settop, lua_tolstring, lua_tonumber, lua_type, LUA_TNIL, LUA_TTABLE};
    use lua::Number;

    use super::gregex_h::{
        GRegex, g_regex_match, GRegexMatchFlags, G_REGEX_MATCH_DEFAULT, GMatchInfo,
        g_regex_new, GRegexCompileFlags, G_REGEX_DEFAULT,
    };
    use super::gtypes_h::{gint, gchar, gpointer};
    use super::lauxlib_h::{luaL_typerror, luaL_checklstring};
    use super::__stddef_null_h::NULL_1;
    use super::__stddef_size_t_h::size_t;
    use super::gtestutils_h::g_strcmp0;
    use super::guri_h::{
        g_uri_join_with_user, GUriFlags, GUri, g_uri_get_scheme, g_uri_get_user,
        g_uri_get_password, g_uri_get_host, g_uri_get_path, g_uri_get_query,
        g_uri_get_fragment, g_uri_get_port, g_uri_parse, g_uri_unref,
    };
    use super::soup_uri_utils_h::SOUP_HTTP_URI_FLAGS;
    use super::gmem_h::g_free;
    use super::gstrfuncs_h::{g_strdup_printf, g_strdup_inline};
    use super::gerror_h::GError;
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
pub use self::__stddef_size_t_h::size_t;
pub use self::glibconfig_h::{guint32, gsize};
pub use self::types_h::{__mode_t, __off_t, __off64_t};
pub use self::gtypes_h::{
    gchar, gint, gboolean, guint, gpointer, gconstpointer, GCompareDataFunc,
    GDestroyNotify,
};
pub use self::garray_h::{_GPtrArray, GPtrArray, g_ptr_array_free};
pub use self::gquark_h::GQuark;
pub use self::gerror_h::{_GError, GError};
pub use self::gdataset_h::{GData, _GData};
pub use self::ghash_h::{GHashTable, _GHashTable};
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
    G_REGEX_MATCH_NOTBOL, G_REGEX_MATCH_ANCHORED, G_REGEX_MATCH_DEFAULT, GRegex,
    GMatchInfo, _GRegex, _GMatchInfo, g_regex_new, g_regex_match,
};
pub use self::gtree_h::{GTree, _GTree, g_tree_new_full};
pub use self::guri_h::{
    GUri, GUriFlags, G_URI_FLAGS_SCHEME_NORMALIZE, G_URI_FLAGS_ENCODED_FRAGMENT,
    G_URI_FLAGS_ENCODED_PATH, G_URI_FLAGS_ENCODED_QUERY, G_URI_FLAGS_NON_DNS,
    G_URI_FLAGS_ENCODED, G_URI_FLAGS_HAS_AUTH_PARAMS, G_URI_FLAGS_HAS_PASSWORD,
    G_URI_FLAGS_PARSE_RELAXED, G_URI_FLAGS_NONE, _GUri, g_uri_unref,
    g_uri_join_with_user, g_uri_parse, g_uri_get_scheme, g_uri_get_user,
    g_uri_get_password, g_uri_get_host, g_uri_get_port, g_uri_get_path, g_uri_get_query,
    g_uri_get_fragment,
};
pub use self::gtype_h::{GType, _GTypeClass, GTypeClass, _GTypeInstance, GTypeInstance};
pub use self::gobject_h::{_GObject, GObject};
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
pub use self::struct_FILE_h::{
    _IO_FILE, _IO_lock_t, _IO_wide_data, _IO_codecvt, _IO_marker,
};
pub use self::FILE_h::FILE;
pub use self::lauxlib_h::{luaL_Reg, luaL_typerror, luaL_checklstring, luaL_error};
pub use self::luaclass_h::{
    lua_class_property_array_t, lua_object_t, lua_class_allocator_t,
    lua_class_propfunc_t, lua_class_t, luaH_class_add_signal, luaH_class_remove_signal,
    luaH_class_emit_signal, luaH_openlib,
};
pub use self::WebKitCookieManager_h::{
    WebKitCookiePersistentStorage, WEBKIT_COOKIE_PERSISTENT_STORAGE_SQLITE,
    WEBKIT_COOKIE_PERSISTENT_STORAGE_TEXT, WebKitCookieManager, _WebKitCookieManager,
    WebKitCookieManagerPrivate, WebKitCookieAcceptPolicy,
    WEBKIT_COOKIE_POLICY_ACCEPT_NO_THIRD_PARTY, WEBKIT_COOKIE_POLICY_ACCEPT_NEVER,
    WEBKIT_COOKIE_POLICY_ACCEPT_ALWAYS, _WebKitCookieManagerPrivate,
    webkit_cookie_manager_set_persistent_storage, webkit_cookie_manager_set_accept_policy,
};
pub use self::WebKitWebContext_h::{
    WebKitWebContext, _WebKitWebContext, WebKitWebContextPrivate,
    _WebKitWebContextPrivate, webkit_web_context_get_website_data_manager,
    webkit_web_context_get_cookie_manager,
};
pub use self::WebKitNetworkProxySettings_h::{
    WebKitNetworkProxySettings, WebKitNetworkProxyMode, WEBKIT_NETWORK_PROXY_MODE_CUSTOM,
    WEBKIT_NETWORK_PROXY_MODE_NO_PROXY, WEBKIT_NETWORK_PROXY_MODE_DEFAULT,
    _WebKitNetworkProxySettings, webkit_network_proxy_settings_new,
    webkit_network_proxy_settings_free,
};
pub use self::WebKitWebsiteDataManager_h::{
    WebKitWebsiteDataManager, _WebKitWebsiteDataManager, WebKitWebsiteDataManagerPrivate,
    _WebKitWebsiteDataManagerPrivate,
    webkit_website_data_manager_set_network_proxy_settings,
};
use self::string_h::{memcpy, strcmp, strlen};
use self::gmem_h::{g_free, g_malloc};
pub use self::gstrfuncs_h::{g_strdup_inline, g_strdup, g_strdup_printf};
pub use self::gmessages_h::G_LOG_DOMAIN;
use self::gtestutils_h::{g_strcmp0, g_assertion_message_expr};
use self::stdio_h::{fclose, fopen};
pub use self::soup_uri_utils_h::SOUP_HTTP_URI_FLAGS;
use self::web_context_h::web_context_get;
use self::stat_h::chmod;
pub use self::gstdio_h::{g_chmod, g_fopen};
pub use self::soup_h::{
    scheme_reg, luaH_soup_uri_tostring, luaH_soup_push_uri, luaH_soup_parse_uri,
    soup_lib_setup_common,
};
pub use self::gmacros_h::{FALSE, TRUE};
pub use self::__stddef_null_h::{NULL, NULL_0, NULL_1};
static mut soup_class: lua_class_t = lua_class_t {
    name: 0 as *const gchar,
    signals: 0 as *const signal_t as *mut signal_t,
    allocator: None,
    properties: 0 as *const lua_class_property_array_t
        as *mut lua_class_property_array_t,
    index_miss_property: None,
    newindex_miss_property: None,
};
static mut proxy_uri: *mut gchar = 0 as *const gchar as *mut gchar;
static mut accept_policy: *mut gchar = 0 as *const gchar as *mut gchar;
static mut cookies_storage: *mut gchar = 0 as *const gchar as *mut gchar;
#[inline]
unsafe extern "C" fn luaH_soup_class_emit_signal(mut L: *mut lua_State) -> gint {
    return luaH_class_emit_signal(
        L,
        &mut soup_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, NULL_1 as *mut size_t),
        lua_gettop(L) - 1 as std::ffi::c_int,
        LUA_MULTRET,
    );
}
#[inline]
unsafe extern "C" fn luaH_soup_class_remove_signal(mut L: *mut lua_State) -> gint {
    luaH_class_remove_signal(
        L,
        &mut soup_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, NULL_1 as *mut size_t),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[inline]
unsafe extern "C" fn luaH_soup_class_add_signal(mut L: *mut lua_State) -> gint {
    luaH_class_add_signal(
        L,
        &mut soup_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, NULL_1 as *mut size_t),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn luaH_soup_index(mut L: *mut lua_State) -> gint {
    let mut prop = luaL_checklstring(L, 2 as std::ffi::c_int, NULL_1 as *mut size_t);
    let mut token = l_tokenize(prop);
    match token as std::ffi::c_uint {
        174 => {
            lua_pushstring(L, proxy_uri);
            return 1 as std::ffi::c_int;
        }
        1 => {
            lua_pushstring(L, accept_policy);
            return 1 as std::ffi::c_int;
        }
        34 => {
            lua_pushstring(L, cookies_storage);
            return 1 as std::ffi::c_int;
        }
        _ => {}
    }
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn luaH_soup_set_proxy_uri(mut L: *mut lua_State) {
    let mut ctx = web_context_get();
    let mut dm = webkit_web_context_get_website_data_manager(ctx);
    let mut new_proxy_uri = if lua_type(L, 3 as std::ffi::c_int) == LUA_TNIL {
        b"default\0" as *const u8 as *const std::ffi::c_char
    } else {
        luaL_checklstring(L, 3 as std::ffi::c_int, NULL_1 as *mut size_t)
    };
    g_free(proxy_uri as gpointer);
    proxy_uri = g_strdup_inline(new_proxy_uri);
    if proxy_uri.is_null()
        || strcmp(
            proxy_uri as *const std::ffi::c_char,
            b"default\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
    {
        webkit_website_data_manager_set_network_proxy_settings(
            dm,
            WEBKIT_NETWORK_PROXY_MODE_DEFAULT,
            NULL_1 as *mut WebKitNetworkProxySettings,
        );
    } else if strcmp(
        proxy_uri as *const std::ffi::c_char,
        b"no_proxy\0" as *const u8 as *const std::ffi::c_char,
    ) == 0 as std::ffi::c_int
    {
        webkit_website_data_manager_set_network_proxy_settings(
            dm,
            WEBKIT_NETWORK_PROXY_MODE_NO_PROXY,
            NULL_1 as *mut WebKitNetworkProxySettings,
        );
    } else {
        let mut proxy_settings = webkit_network_proxy_settings_new(
            proxy_uri,
            NULL_1 as *const *const gchar,
        );
        webkit_website_data_manager_set_network_proxy_settings(
            dm,
            WEBKIT_NETWORK_PROXY_MODE_CUSTOM,
            proxy_settings,
        );
        webkit_network_proxy_settings_free(proxy_settings);
    };
}
unsafe extern "C" fn luaH_soup_set_accept_policy(mut L: *mut lua_State) {
    let mut new_policy = luaL_checklstring(
        L,
        3 as std::ffi::c_int,
        NULL_1 as *mut size_t,
    );
    if !(strcmp(
        new_policy as *const std::ffi::c_char,
        b"always\0" as *const u8 as *const std::ffi::c_char,
    ) == 0 as std::ffi::c_int)
    {
        if !(strcmp(
            new_policy as *const std::ffi::c_char,
            b"never\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int)
        {
            if !(strcmp(
                new_policy as *const std::ffi::c_char,
                b"no_third_party\0" as *const u8 as *const std::ffi::c_char,
            ) == 0 as std::ffi::c_int)
            {
                luaL_error(
                    L,
                    b"accept_policy must be one of 'always', 'never', 'no_third_party'\0"
                        as *const u8 as *const std::ffi::c_char,
                );
            }
        }
    }
    g_free(accept_policy as gpointer);
    accept_policy = g_strdup_inline(new_policy);
    let mut web_context = web_context_get();
    let mut cookie_mgr = webkit_web_context_get_cookie_manager(web_context);
    let mut policy = WEBKIT_COOKIE_POLICY_ACCEPT_ALWAYS;
    if strcmp(
        new_policy as *const std::ffi::c_char,
        b"always\0" as *const u8 as *const std::ffi::c_char,
    ) == 0 as std::ffi::c_int
    {
        policy = WEBKIT_COOKIE_POLICY_ACCEPT_ALWAYS;
    } else if strcmp(
        new_policy as *const std::ffi::c_char,
        b"never\0" as *const u8 as *const std::ffi::c_char,
    ) == 0 as std::ffi::c_int
    {
        policy = WEBKIT_COOKIE_POLICY_ACCEPT_NEVER;
    } else if strcmp(
        new_policy as *const std::ffi::c_char,
        b"no_third_party\0" as *const u8 as *const std::ffi::c_char,
    ) == 0 as std::ffi::c_int
    {
        policy = WEBKIT_COOKIE_POLICY_ACCEPT_NO_THIRD_PARTY;
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN as *const std::ffi::c_char,
            b"clib/soup.c\0" as *const u8 as *const std::ffi::c_char,
            97 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[std::ffi::c_char; 28],
            >(b"luaH_soup_set_accept_policy\0"))
                .as_ptr(),
            NULL_1 as *const std::ffi::c_char,
        );
    }
    webkit_cookie_manager_set_accept_policy(cookie_mgr, policy);
}
unsafe extern "C" fn luaH_soup_set_cookies_storage(mut L: *mut lua_State) {
    let mut new_path = luaL_checklstring(L, 3 as std::ffi::c_int, NULL_1 as *mut size_t);
    let mut f = 0 as *mut FILE;
    if strcmp(
        new_path as *const std::ffi::c_char,
        b"\0" as *const u8 as *const std::ffi::c_char,
    ) == 0 as std::ffi::c_int
    {
        luaL_error(
            L,
            b"cookies_storage cannot be empty\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    g_free(cookies_storage as gpointer);
    cookies_storage = g_strdup_inline(new_path);
    f = fopen(cookies_storage, b"a\0" as *const u8 as *const std::ffi::c_char);
    if !f.is_null() {
        chmod(cookies_storage, 0o600 as std::ffi::c_int as __mode_t);
        fclose(f);
    }
    let mut web_context = web_context_get();
    let mut cookie_mgr = webkit_web_context_get_cookie_manager(web_context);
    webkit_cookie_manager_set_persistent_storage(
        cookie_mgr,
        cookies_storage,
        WEBKIT_COOKIE_PERSISTENT_STORAGE_SQLITE,
    );
}
unsafe extern "C" fn luaH_soup_newindex(mut L: *mut lua_State) -> gint {
    let mut prop = luaL_checklstring(L, 2 as std::ffi::c_int, NULL_1 as *mut size_t);
    let mut token = l_tokenize(prop);
    match token as std::ffi::c_uint {
        174 => {
            luaH_soup_set_proxy_uri(L);
        }
        1 => {
            luaH_soup_set_accept_policy(L);
        }
        34 => {
            luaH_soup_set_cookies_storage(L);
        }
        _ => return 0 as std::ffi::c_int,
    }
    return 0 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn soup_lib_setup(mut L: *mut lua_State) {
    soup_lib_setup_common();
    static mut soup_lib: [luaL_Reg; 8] = unsafe {
        [
            {
                let mut init = luaL_Reg {
                    name: b"add_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_soup_class_add_signal
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"remove_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_soup_class_remove_signal
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"emit_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_soup_class_emit_signal
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__index\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_soup_index as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__newindex\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_soup_newindex
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"parse_uri\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_soup_parse_uri
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"uri_tostring\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_soup_uri_tostring
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: NULL_1 as *const std::ffi::c_char,
                    func: ::core::mem::transmute::<
                        libc::intptr_t,
                        lua::Function,
                    >(NULL_1 as libc::intptr_t),
                };
                init
            },
        ]
    };
    soup_class.signals = signal_new();
    luaH_openlib(
        L,
        b"soup\0" as *const u8 as *const std::ffi::c_char,
        soup_lib.as_ptr(),
        soup_lib.as_ptr(),
    );
    proxy_uri = g_strdup_inline(b"default\0" as *const u8 as *const std::ffi::c_char);
    accept_policy = g_strdup_inline(
        b"no_third_party\0" as *const u8 as *const std::ffi::c_char,
    );
}
