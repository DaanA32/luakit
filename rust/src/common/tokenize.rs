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
use glib_sys::{
    GHashTable, g_hash_table_insert, g_hash_table_lookup, g_hash_table_new, g_str_equal,
    g_str_hash, gboolean, gconstpointer, gpointer,
};

use crate::gtypes::{gchar, guint};

pub use self::tokenize_h::{
    L_TK_ACCEPT_POLICY, L_TK_ADD_EVENT_LISTENER, L_TK_ALIGN, L_TK_ALLOW_CERTIFICATE,
    L_TK_ALLOW_FILE_ACCESS_FROM_FILE_URLS, L_TK_ALLOW_MODAL_DIALOGS, L_TK_ALLOW_OVERWRITE,
    L_TK_ALLOW_UNIVERSAL_ACCESS_FROM_FILE_URLS, L_TK_APPEND, L_TK_ATTR, L_TK_AUTO_LOAD_IMAGES,
    L_TK_BASELINE, L_TK_BG, L_TK_BODY, L_TK_BOTTOM, L_TK_CACHE_DIR, L_TK_CAN_FOCUS,
    L_TK_CAN_GO_BACK, L_TK_CAN_GO_FORWARD, L_TK_CENTER, L_TK_CERTIFICATE, L_TK_CHECKED, L_TK_CHILD,
    L_TK_CHILD_COUNT, L_TK_CHILDREN, L_TK_CLEAR, L_TK_CLEAR_SEARCH, L_TK_CLICK, L_TK_CLIENT_RECTS,
    L_TK_CLIPBOARD, L_TK_CLOSE_INSPECTOR, L_TK_CONFIG_DIR, L_TK_CONFPATH, L_TK_COOKIES_STORAGE,
    L_TK_COUNT, L_TK_CRASH, L_TK_CREATE_ELEMENT, L_TK_CSS, L_TK_CURRENT, L_TK_CURRENT_SIZE,
    L_TK_CURSIVE_FONT_FAMILY, L_TK_DATA_DIR, L_TK_DECORATED, L_TK_DEFAULT_CHARSET,
    L_TK_DEFAULT_FONT_FAMILY, L_TK_DEFAULT_FONT_SIZE, L_TK_DEFAULT_MONOSPACE_FONT_SIZE,
    L_TK_DESKTOP_DIR, L_TK_DESTINATION, L_TK_DESTROY, L_TK_DEV_PATHS, L_TK_DOCUMENT,
    L_TK_DOCUMENTS_DIR, L_TK_DOWNLOAD_DIR, L_TK_DRAW_COMPOSITING_INDICATORS, L_TK_DRAWING_AREA,
    L_TK_EDITABLE, L_TK_ELAPSED_TIME, L_TK_ELEMENT_FROM_POINT, L_TK_ENABLE_ACCELERATED_2D_CANVAS,
    L_TK_ENABLE_CARET_BROWSING, L_TK_ENABLE_DEVELOPER_EXTRAS, L_TK_ENABLE_DNS_PREFETCHING,
    L_TK_ENABLE_FRAME_FLATTENING, L_TK_ENABLE_FULLSCREEN, L_TK_ENABLE_HTML5_DATABASE,
    L_TK_ENABLE_HTML5_LOCAL_STORAGE, L_TK_ENABLE_HYPERLINK_AUDITING, L_TK_ENABLE_JAVA,
    L_TK_ENABLE_JAVASCRIPT, L_TK_ENABLE_MEDIA_STREAM, L_TK_ENABLE_MEDIASOURCE,
    L_TK_ENABLE_PAGE_CACHE, L_TK_ENABLE_PLUGINS, L_TK_ENABLE_RESIZABLE_TEXT_AREAS,
    L_TK_ENABLE_SCRIPTS, L_TK_ENABLE_SITE_SPECIFIC_QUIRKS, L_TK_ENABLE_SMOOTH_SCROLLING,
    L_TK_ENABLE_SPATIAL_NAVIGATION, L_TK_ENABLE_SPELL_CHECKING, L_TK_ENABLE_TABS_TO_LINKS,
    L_TK_ENABLE_WEBAUDIO, L_TK_ENABLE_WEBGL, L_TK_ENABLE_WRITE_CONSOLE_MESSAGES_TO_STDOUT,
    L_TK_ENABLE_XSS_AUDITOR, L_TK_END, L_TK_ENTRY, L_TK_ERROR, L_TK_EVAL_JS, L_TK_EVENTBOX,
    L_TK_EXECPATH, L_TK_FANTASY_FONT_FAMILY, L_TK_FETCH, L_TK_FG, L_TK_FILENAME, L_TK_FILL,
    L_TK_FINISHED, L_TK_FIRST_CHILD, L_TK_FOCUS, L_TK_FOCUSED, L_TK_FONT, L_TK_FULLSCREEN,
    L_TK_GET_SOURCE, L_TK_GET_TITLE, L_TK_GO_BACK, L_TK_GO_FORWARD,
    L_TK_HARDWARE_ACCELERATION_POLICY, L_TK_HBOX, L_TK_HEIGHT, L_TK_HIDE, L_TK_HISTORY,
    L_TK_HOMOGENEOUS, L_TK_HOVERED_URI, L_TK_HPANED, L_TK_HREF, L_TK_ICON, L_TK_ID, L_TK_IMAGE,
    L_TK_INDEXOF, L_TK_INNER_HEIGHT, L_TK_INNER_HTML, L_TK_INNER_WIDTH, L_TK_INSERT,
    L_TK_INSPECTOR, L_TK_INSTALL_PATH, L_TK_INSTALL_PATHS, L_TK_INTERVAL, L_TK_INVALIDATE,
    L_TK_IS_ALIVE, L_TK_IS_LOADING, L_TK_IS_PLAYING_AUDIO, L_TK_JAVASCRIPT_CAN_ACCESS_CLIPBOARD,
    L_TK_JAVASCRIPT_CAN_OPEN_WINDOWS_AUTOMATICALLY, L_TK_LABEL, L_TK_LAST_CHILD, L_TK_LEFT,
    L_TK_LOAD_STRING, L_TK_LOADING, L_TK_MARGIN, L_TK_MARGIN_BOTTOM, L_TK_MARGIN_LEFT,
    L_TK_MARGIN_RIGHT, L_TK_MARGIN_TOP, L_TK_MAXIMIZED, L_TK_MEDIA_PLAYBACK_ALLOWS_INLINE,
    L_TK_MEDIA_PLAYBACK_REQUIRES_GESTURE, L_TK_MIME_TYPE, L_TK_MIN_SIZE, L_TK_MINIMUM_FONT_SIZE,
    L_TK_MONOSPACE_FONT_FAMILY, L_TK_MUSIC_DIR, L_TK_NAME, L_TK_NEXT_SIBLING, L_TK_NOTEBOOK,
    L_TK_NOUNIQUE, L_TK_OPTIONS, L_TK_OVERLAY, L_TK_OWNER_DOCUMENT, L_TK_PACK, L_TK_PACK1,
    L_TK_PACK2, L_TK_PARENT, L_TK_PATTERN, L_TK_PICTOGRAPH_FONT_FAMILY, L_TK_PICTURES_DIR,
    L_TK_PLUGGED, L_TK_POSITION, L_TK_PREV_SIBLING, L_TK_PRIMARY, L_TK_PRINT_BACKGROUNDS,
    L_TK_PRIVATE, L_TK_PROCESS_LIMIT, L_TK_PROGRESS, L_TK_PROXY_URI, L_TK_PUBLIC_SHARE_DIR,
    L_TK_QUERY, L_TK_RECT, L_TK_RELOAD, L_TK_RELOAD_BYPASS_CACHE, L_TK_REMOVE,
    L_TK_REMOVE_EVENT_LISTENER, L_TK_REORDER, L_TK_REPLACE, L_TK_RESOURCE_PATH, L_TK_RIGHT,
    L_TK_ROOT_WIN_XID, L_TK_SANS_SERIF_FONT_FAMILY, L_TK_SAVE, L_TK_SCALE, L_TK_SCREEN,
    L_TK_SCROLL, L_TK_SCROLL_X, L_TK_SCROLL_Y, L_TK_SCROLLBARS, L_TK_SCROLLED, L_TK_SEARCH,
    L_TK_SEARCH_NEXT, L_TK_SEARCH_PREVIOUS, L_TK_SECONDARY, L_TK_SELECT_REGION, L_TK_SELECTABLE,
    L_TK_SELECTION, L_TK_SEND_KEY, L_TK_SERIF_FONT_FAMILY, L_TK_SESSION_STATE, L_TK_SET_DARK_MODE,
    L_TK_SET_DEFAULT_SIZE, L_TK_SET_FAVICON_FOR_URI, L_TK_SET_PDFJS, L_TK_SET_TITLE, L_TK_SHOW,
    L_TK_SHOW_BORDER, L_TK_SHOW_FRAME, L_TK_SHOW_INSPECTOR, L_TK_SHOW_TABS, L_TK_SOCKET,
    L_TK_SOURCE, L_TK_SPACING, L_TK_SPELL_CHECKING_LANGUAGES, L_TK_SPINNER, L_TK_SRC,
    L_TK_SSL_TRUSTED, L_TK_STACK, L_TK_START, L_TK_STARTED, L_TK_STATUS, L_TK_STOP, L_TK_STYLE,
    L_TK_STYLESHEETS, L_TK_SUBMIT, L_TK_SUGGESTED_FILENAME, L_TK_SWITCH, L_TK_SYSTEM_CONFIG_DIRS,
    L_TK_SYSTEM_DATA_DIRS, L_TK_TAG_NAME, L_TK_TEMPLATES_DIR, L_TK_TEXT, L_TK_TEXT_CONTENT,
    L_TK_TEXTWIDTH, L_TK_TITLE, L_TK_TOOLTIP, L_TK_TOP, L_TK_TOTAL_SIZE, L_TK_TYPE, L_TK_UNKNOWN,
    L_TK_URGENCY_HINT, L_TK_URI, L_TK_USER_AGENT, L_TK_VALUE, L_TK_VBOX, L_TK_VERBOSE,
    L_TK_VERSION, L_TK_VIDEOS_DIR, L_TK_VISIBLE, L_TK_VISIBLE_CHILD, L_TK_VPANED,
    L_TK_WEB_PROCESS_ID, L_TK_WEBKIT_USER_AGENT_VERSION, L_TK_WEBKIT_VERSION, L_TK_WEBKIT2,
    L_TK_WEBSITE_DATA, L_TK_WEBVIEW, L_TK_WIDTH, L_TK_WIN_XID, L_TK_WINDOW, L_TK_WINDOWS,
    L_TK_WRAP_JS, L_TK_X, L_TK_XMAX, L_TK_XPAGE_SIZE, L_TK_Y, L_TK_YMAX, L_TK_YPAGE_SIZE,
    L_TK_ZOOM_LEVEL, L_TK_ZOOM_TEXT_ONLY, luakit_token_t,
};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct token_map_t {
    pub tok: luakit_token_t,
    pub name: *const gchar,
}
static mut tokens_table: [token_map_t; 275] = [
    {
        let mut init = token_map_t {
            tok: L_TK_ACCEPT_POLICY,
            name: b"accept_policy\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_ADD_EVENT_LISTENER,
            name: b"add_event_listener\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_ALIGN,
            name: b"align\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_ALLOW_CERTIFICATE,
            name: b"allow_certificate\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_ALLOW_FILE_ACCESS_FROM_FILE_URLS,
            name: b"allow_file_access_from_file_urls\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_ALLOW_MODAL_DIALOGS,
            name: b"allow_modal_dialogs\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_ALLOW_OVERWRITE,
            name: b"allow_overwrite\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_ALLOW_UNIVERSAL_ACCESS_FROM_FILE_URLS,
            name: b"allow_universal_access_from_file_urls\0" as *const u8
                as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_APPEND,
            name: b"append\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_ATTR,
            name: b"attr\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_AUTO_LOAD_IMAGES,
            name: b"auto_load_images\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_BASELINE,
            name: b"baseline\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_BG,
            name: b"bg\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_BODY,
            name: b"body\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_BOTTOM,
            name: b"bottom\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_CACHE_DIR,
            name: b"cache_dir\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_CAN_FOCUS,
            name: b"can_focus\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_CAN_GO_BACK,
            name: b"can_go_back\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_CAN_GO_FORWARD,
            name: b"can_go_forward\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_CENTER,
            name: b"center\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_CERTIFICATE,
            name: b"certificate\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_CHECKED,
            name: b"checked\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_CHILD,
            name: b"child\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_CHILDREN,
            name: b"children\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_CHILD_COUNT,
            name: b"child_count\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_CLEAR,
            name: b"clear\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_CLEAR_SEARCH,
            name: b"clear_search\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_CLICK,
            name: b"click\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_CLIENT_RECTS,
            name: b"client_rects\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_CLIPBOARD,
            name: b"clipboard\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_CLOSE_INSPECTOR,
            name: b"close_inspector\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_CONFIG_DIR,
            name: b"config_dir\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_CONFPATH,
            name: b"confpath\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_COOKIES_STORAGE,
            name: b"cookies_storage\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_COUNT,
            name: b"count\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_CRASH,
            name: b"crash\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_CREATE_ELEMENT,
            name: b"create_element\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_CSS,
            name: b"css\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_CURRENT,
            name: b"current\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_CURRENT_SIZE,
            name: b"current_size\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_CURSIVE_FONT_FAMILY,
            name: b"cursive_font_family\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_DATA_DIR,
            name: b"data_dir\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_DECORATED,
            name: b"decorated\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_DEFAULT_CHARSET,
            name: b"default_charset\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_DEFAULT_FONT_FAMILY,
            name: b"default_font_family\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_DEFAULT_FONT_SIZE,
            name: b"default_font_size\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_DEFAULT_MONOSPACE_FONT_SIZE,
            name: b"default_monospace_font_size\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_DESKTOP_DIR,
            name: b"desktop_dir\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_DESTINATION,
            name: b"destination\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_DESTROY,
            name: b"destroy\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_DEV_PATHS,
            name: b"dev_paths\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_DOCUMENT,
            name: b"document\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_DOCUMENTS_DIR,
            name: b"documents_dir\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_DOWNLOAD_DIR,
            name: b"download_dir\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_DRAWING_AREA,
            name: b"drawing_area\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_DRAW_COMPOSITING_INDICATORS,
            name: b"draw_compositing_indicators\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_EDITABLE,
            name: b"editable\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_ELAPSED_TIME,
            name: b"elapsed_time\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_ELEMENT_FROM_POINT,
            name: b"element_from_point\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_ENABLE_ACCELERATED_2D_CANVAS,
            name: b"enable_accelerated_2d_canvas\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_ENABLE_CARET_BROWSING,
            name: b"enable_caret_browsing\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_ENABLE_DEVELOPER_EXTRAS,
            name: b"enable_developer_extras\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_ENABLE_DNS_PREFETCHING,
            name: b"enable_dns_prefetching\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_ENABLE_FRAME_FLATTENING,
            name: b"enable_frame_flattening\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_ENABLE_FULLSCREEN,
            name: b"enable_fullscreen\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_ENABLE_HTML5_DATABASE,
            name: b"enable_html5_database\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_ENABLE_HTML5_LOCAL_STORAGE,
            name: b"enable_html5_local_storage\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_ENABLE_HYPERLINK_AUDITING,
            name: b"enable_hyperlink_auditing\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_ENABLE_JAVA,
            name: b"enable_java\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_ENABLE_JAVASCRIPT,
            name: b"enable_javascript\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_ENABLE_MEDIASOURCE,
            name: b"enable_mediasource\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_ENABLE_MEDIA_STREAM,
            name: b"enable_media_stream\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_ENABLE_PAGE_CACHE,
            name: b"enable_page_cache\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_ENABLE_PLUGINS,
            name: b"enable_plugins\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_ENABLE_RESIZABLE_TEXT_AREAS,
            name: b"enable_resizable_text_areas\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_ENABLE_SCRIPTS,
            name: b"enable_scripts\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_ENABLE_SITE_SPECIFIC_QUIRKS,
            name: b"enable_site_specific_quirks\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_ENABLE_SMOOTH_SCROLLING,
            name: b"enable_smooth_scrolling\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_ENABLE_SPATIAL_NAVIGATION,
            name: b"enable_spatial_navigation\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_ENABLE_SPELL_CHECKING,
            name: b"enable_spell_checking\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_ENABLE_TABS_TO_LINKS,
            name: b"enable_tabs_to_links\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_ENABLE_WEBAUDIO,
            name: b"enable_webaudio\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_ENABLE_WEBGL,
            name: b"enable_webgl\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_ENABLE_WRITE_CONSOLE_MESSAGES_TO_STDOUT,
            name: b"enable_write_console_messages_to_stdout\0" as *const u8
                as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_ENABLE_XSS_AUDITOR,
            name: b"enable_xss_auditor\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_END,
            name: b"end\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_ENTRY,
            name: b"entry\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_ERROR,
            name: b"error\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_EVAL_JS,
            name: b"eval_js\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_EVENTBOX,
            name: b"eventbox\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_EXECPATH,
            name: b"execpath\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_FANTASY_FONT_FAMILY,
            name: b"fantasy_font_family\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_FETCH,
            name: b"fetch\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_FG,
            name: b"fg\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_FILENAME,
            name: b"filename\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_FILL,
            name: b"fill\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_FINISHED,
            name: b"finished\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_FIRST_CHILD,
            name: b"first_child\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_FOCUS,
            name: b"focus\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_FOCUSED,
            name: b"focused\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_FONT,
            name: b"font\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_FULLSCREEN,
            name: b"fullscreen\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_GET_SOURCE,
            name: b"get_source\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_GET_TITLE,
            name: b"get_title\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_GO_BACK,
            name: b"go_back\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_GO_FORWARD,
            name: b"go_forward\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_HARDWARE_ACCELERATION_POLICY,
            name: b"hardware_acceleration_policy\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_HBOX,
            name: b"hbox\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_HEIGHT,
            name: b"height\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_HIDE,
            name: b"hide\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_HISTORY,
            name: b"history\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_HOMOGENEOUS,
            name: b"homogeneous\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_HOVERED_URI,
            name: b"hovered_uri\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_HPANED,
            name: b"hpaned\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_HREF,
            name: b"href\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_ICON,
            name: b"icon\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_ID,
            name: b"id\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_IMAGE,
            name: b"image\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_INDEXOF,
            name: b"indexof\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_INNER_HEIGHT,
            name: b"inner_height\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_INNER_HTML,
            name: b"inner_html\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_INNER_WIDTH,
            name: b"inner_width\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_INSERT,
            name: b"insert\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_INSPECTOR,
            name: b"inspector\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_INSTALL_PATH,
            name: b"install_path\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_INSTALL_PATHS,
            name: b"install_paths\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_INTERVAL,
            name: b"interval\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_INVALIDATE,
            name: b"invalidate\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_IS_ALIVE,
            name: b"is_alive\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_IS_LOADING,
            name: b"is_loading\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_IS_PLAYING_AUDIO,
            name: b"is_playing_audio\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_JAVASCRIPT_CAN_ACCESS_CLIPBOARD,
            name: b"javascript_can_access_clipboard\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_JAVASCRIPT_CAN_OPEN_WINDOWS_AUTOMATICALLY,
            name: b"javascript_can_open_windows_automatically\0" as *const u8
                as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_LABEL,
            name: b"label\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_LAST_CHILD,
            name: b"last_child\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_LEFT,
            name: b"left\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_LOADING,
            name: b"loading\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_LOAD_STRING,
            name: b"load_string\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_MARGIN,
            name: b"margin\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_MARGIN_BOTTOM,
            name: b"margin_bottom\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_MARGIN_LEFT,
            name: b"margin_left\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_MARGIN_RIGHT,
            name: b"margin_right\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_MARGIN_TOP,
            name: b"margin_top\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_MAXIMIZED,
            name: b"maximized\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_MEDIA_PLAYBACK_ALLOWS_INLINE,
            name: b"media_playback_allows_inline\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_MEDIA_PLAYBACK_REQUIRES_GESTURE,
            name: b"media_playback_requires_gesture\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_MIME_TYPE,
            name: b"mime_type\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_MINIMUM_FONT_SIZE,
            name: b"minimum_font_size\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_MIN_SIZE,
            name: b"min_size\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_MONOSPACE_FONT_FAMILY,
            name: b"monospace_font_family\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_MUSIC_DIR,
            name: b"music_dir\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_NAME,
            name: b"name\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_NEXT_SIBLING,
            name: b"next_sibling\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_NOTEBOOK,
            name: b"notebook\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_NOUNIQUE,
            name: b"nounique\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_OPTIONS,
            name: b"options\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_OVERLAY,
            name: b"overlay\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_OWNER_DOCUMENT,
            name: b"owner_document\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_PACK,
            name: b"pack\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_PACK1,
            name: b"pack1\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_PACK2,
            name: b"pack2\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_PARENT,
            name: b"parent\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_PATTERN,
            name: b"pattern\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_PICTOGRAPH_FONT_FAMILY,
            name: b"pictograph_font_family\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_PICTURES_DIR,
            name: b"pictures_dir\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_PLUGGED,
            name: b"plugged\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_POSITION,
            name: b"position\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_PREV_SIBLING,
            name: b"prev_sibling\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_PRIMARY,
            name: b"primary\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_PRINT_BACKGROUNDS,
            name: b"print_backgrounds\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_PRIVATE,
            name: b"private\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_PROCESS_LIMIT,
            name: b"process_limit\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_PROGRESS,
            name: b"progress\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_PROXY_URI,
            name: b"proxy_uri\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_PUBLIC_SHARE_DIR,
            name: b"public_share_dir\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_QUERY,
            name: b"query\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_RECT,
            name: b"rect\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_RELOAD,
            name: b"reload\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_RELOAD_BYPASS_CACHE,
            name: b"reload_bypass_cache\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_REMOVE,
            name: b"remove\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_REMOVE_EVENT_LISTENER,
            name: b"remove_event_listener\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_REORDER,
            name: b"reorder\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_REPLACE,
            name: b"replace\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_RESOURCE_PATH,
            name: b"resource_path\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_RIGHT,
            name: b"right\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_ROOT_WIN_XID,
            name: b"root_win_xid\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_SANS_SERIF_FONT_FAMILY,
            name: b"sans_serif_font_family\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_SAVE,
            name: b"save\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_SCALE,
            name: b"scale\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_SCREEN,
            name: b"screen\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_SCROLL,
            name: b"scroll\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_SCROLLBARS,
            name: b"scrollbars\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_SCROLLED,
            name: b"scrolled\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_SCROLL_X,
            name: b"scroll_x\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_SCROLL_Y,
            name: b"scroll_y\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_SEARCH,
            name: b"search\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_SEARCH_NEXT,
            name: b"search_next\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_SEARCH_PREVIOUS,
            name: b"search_previous\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_SECONDARY,
            name: b"secondary\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_SELECTABLE,
            name: b"selectable\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_SELECTION,
            name: b"selection\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_SELECT_REGION,
            name: b"select_region\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_SEND_KEY,
            name: b"send_key\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_SERIF_FONT_FAMILY,
            name: b"serif_font_family\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_SESSION_STATE,
            name: b"session_state\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_SET_DARK_MODE,
            name: b"set_dark_mode\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_SET_DEFAULT_SIZE,
            name: b"set_default_size\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_SET_FAVICON_FOR_URI,
            name: b"set_favicon_for_uri\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_SET_PDFJS,
            name: b"set_pdfjs\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_SET_TITLE,
            name: b"set_title\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_SHOW,
            name: b"show\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_SHOW_BORDER,
            name: b"show_border\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_SHOW_FRAME,
            name: b"show_frame\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_SHOW_INSPECTOR,
            name: b"show_inspector\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_SHOW_TABS,
            name: b"show_tabs\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_SOCKET,
            name: b"socket\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_SOURCE,
            name: b"source\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_SPACING,
            name: b"spacing\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_SPELL_CHECKING_LANGUAGES,
            name: b"spell_checking_languages\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_SPINNER,
            name: b"spinner\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_SRC,
            name: b"src\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_SSL_TRUSTED,
            name: b"ssl_trusted\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_STACK,
            name: b"stack\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_START,
            name: b"start\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_STARTED,
            name: b"started\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_STATUS,
            name: b"status\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_STOP,
            name: b"stop\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_STYLE,
            name: b"style\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_STYLESHEETS,
            name: b"stylesheets\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_SUBMIT,
            name: b"submit\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_SUGGESTED_FILENAME,
            name: b"suggested_filename\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_SWITCH,
            name: b"switch\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_SYSTEM_CONFIG_DIRS,
            name: b"system_config_dirs\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_SYSTEM_DATA_DIRS,
            name: b"system_data_dirs\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_TAG_NAME,
            name: b"tag_name\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_TEMPLATES_DIR,
            name: b"templates_dir\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_TEXT,
            name: b"text\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_TEXTWIDTH,
            name: b"textwidth\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_TEXT_CONTENT,
            name: b"text_content\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_TITLE,
            name: b"title\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_TOOLTIP,
            name: b"tooltip\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_TOP,
            name: b"top\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_TOTAL_SIZE,
            name: b"total_size\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_TYPE,
            name: b"type\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_URGENCY_HINT,
            name: b"urgency_hint\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_URI,
            name: b"uri\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_USER_AGENT,
            name: b"user_agent\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_VALUE,
            name: b"value\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_VBOX,
            name: b"vbox\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_VERBOSE,
            name: b"verbose\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_VERSION,
            name: b"version\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_VIDEOS_DIR,
            name: b"videos_dir\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_VISIBLE,
            name: b"visible\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_VISIBLE_CHILD,
            name: b"visible_child\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_VPANED,
            name: b"vpaned\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_WEBKIT2,
            name: b"webkit2\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_WEBKIT_USER_AGENT_VERSION,
            name: b"webkit_user_agent_version\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_WEBKIT_VERSION,
            name: b"webkit_version\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_WEBSITE_DATA,
            name: b"website_data\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_WEBVIEW,
            name: b"webview\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_WEB_PROCESS_ID,
            name: b"web_process_id\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_WIDTH,
            name: b"width\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_WINDOW,
            name: b"window\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_WINDOWS,
            name: b"windows\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_WIN_XID,
            name: b"win_xid\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_WRAP_JS,
            name: b"wrap_js\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_X,
            name: b"x\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_XMAX,
            name: b"xmax\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_XPAGE_SIZE,
            name: b"xpage_size\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_Y,
            name: b"y\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_YMAX,
            name: b"ymax\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_YPAGE_SIZE,
            name: b"ypage_size\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_ZOOM_LEVEL,
            name: b"zoom_level\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_ZOOM_TEXT_ONLY,
            name: b"zoom_text_only\0" as *const u8 as *const std::ffi::c_char,
        };
        init
    },
    {
        let mut init = token_map_t {
            tok: L_TK_UNKNOWN,
            name: std::ptr::null(),
        };
        init
    },
];
#[unsafe(no_mangle)]
pub unsafe extern "C" fn l_tokenize(mut s: *const gchar) -> luakit_token_t {
    static mut tokens: *mut GHashTable = std::ptr::null_mut();
    if tokens.is_null() {
        tokens = g_hash_table_new(
            Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
            Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        );
        let mut t = tokens_table.as_mut_ptr();
        while !((*t).name).is_null() {
            g_hash_table_insert(tokens, (*t).name as gpointer, (*t).tok as gpointer);
            t = t.offset(1);
            t;
        }
    }
    return g_hash_table_lookup(tokens, s as gconstpointer) as luakit_token_t;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn token_tostring(mut tok: luakit_token_t) -> *const gchar {
    if tok as std::ffi::c_uint == L_TK_UNKNOWN as std::ffi::c_int as std::ffi::c_uint {
        return std::ptr::null();
    }
    return tokens_table[(tok - 1) as usize].name;
}
