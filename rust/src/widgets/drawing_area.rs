use ::libc;
#[c2rust::header_src = "/usr/lib/clang/20/include/__stddef_size_t.h:19"]
pub mod __stddef_size_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type size_t = std::ffi::c_ulong;
}
#[c2rust::header_src = "/usr/lib/glib-2.0/include/glibconfig.h:19"]
pub mod glibconfig_h {
    #[c2rust::src_loc = "45:1"]
    pub type gint8 = std::ffi::c_schar;
    #[c2rust::src_loc = "48:1"]
    pub type gint16 = std::ffi::c_short;
    #[c2rust::src_loc = "83:1"]
    pub type gsize = std::ffi::c_ulong;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtypes.h:19"]
pub mod gtypes_h {
    #[c2rust::src_loc = "52:1"]
    pub type gchar = std::ffi::c_char;
    #[c2rust::src_loc = "55:1"]
    pub type gint = std::ffi::c_int;
    #[c2rust::src_loc = "56:1"]
    pub type gboolean = gint;
    #[c2rust::src_loc = "61:1"]
    pub type guint = std::ffi::c_uint;
    #[c2rust::src_loc = "109:1"]
    pub type gpointer = *mut std::ffi::c_void;
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
#[c2rust::header_src = "/usr/include/glib-2.0/glib/ghash.h:19"]
pub mod ghash_h {
    #[c2rust::src_loc = "40:1"]
    pub type GHashTable = _GHashTable;
    extern "C" {
        #[c2rust::src_loc = "40:16"]
        pub type _GHashTable;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtree.h:19"]
pub mod gtree_h {
    #[c2rust::src_loc = "40:1"]
    pub type GTree = _GTree;
    extern "C" {
        #[c2rust::src_loc = "40:16"]
        pub type _GTree;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/gobject/gtype.h:19"]
pub mod gtype_h {
    #[c2rust::src_loc = "427:1"]
    pub type GType = gsize;
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
    use super::gtypes_h::gboolean;
    extern "C" {
        #[c2rust::src_loc = "2626:1"]
        pub fn g_type_check_instance_cast(
            instance: *mut GTypeInstance,
            iface_type: GType,
        ) -> *mut GTypeInstance;
        #[c2rust::src_loc = "2629:1"]
        pub fn g_type_check_instance_is_a(
            instance: *mut GTypeInstance,
            iface_type: GType,
        ) -> gboolean;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/gobject/gclosure.h:19"]
pub mod gclosure_h {
    #[c2rust::src_loc = "92:1"]
    pub type GCallback = Option::<unsafe extern "C" fn() -> ()>;
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
    #[c2rust::src_loc = "194:1"]
    pub type GInitiallyUnowned = _GObject;
    use super::gtype_h::GTypeInstance;
    use super::gtypes_h::{guint, gpointer, gchar};
    use super::gdataset_h::GData;
    extern "C" {
        #[c2rust::src_loc = "462:1"]
        pub fn g_object_connect(
            object: gpointer,
            signal_spec: *const gchar,
            _: ...
        ) -> gpointer;
    }
}
#[c2rust::header_src = "/usr/include/luajit-2.1/lua.h:19"]
pub mod lua_h {
    #[c2rust::src_loc = "53:1"]
    pub type lua_CFunction = Option::<
        unsafe extern "C" fn(*mut lua_State) -> std::ffi::c_int,
    >;
    #[c2rust::src_loc = "36:9"]
    pub const LUA_REGISTRYINDEX: std::ffi::c_int = -(10000 as std::ffi::c_int);
    #[c2rust::src_loc = "38:9"]
    pub const LUA_GLOBALSINDEX: std::ffi::c_int = -(10002 as std::ffi::c_int);
    #[c2rust::src_loc = "80:9"]
    pub const LUA_TTABLE: std::ffi::c_int = 5 as std::ffi::c_int;
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "51:16"]
        pub type lua_State;
        #[c2rust::src_loc = "122:1"]
        pub fn lua_settop(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "124:1"]
        pub fn lua_remove(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "140:1"]
        pub fn lua_type(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;
        #[c2rust::src_loc = "164:1"]
        pub fn lua_pushlstring(L: *mut lua_State, s: *const std::ffi::c_char, l: size_t);
        #[c2rust::src_loc = "169:1"]
        pub fn lua_pushcclosure(
            L: *mut lua_State,
            fn_0: lua_CFunction,
            n: std::ffi::c_int,
        );
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
        #[c2rust::src_loc = "203:1"]
        pub fn lua_pcall(
            L: *mut lua_State,
            nargs: std::ffi::c_int,
            nresults: std::ffi::c_int,
            errfunc: std::ffi::c_int,
        ) -> std::ffi::c_int;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/signal.h:19"]
pub mod signal_h {
    #[c2rust::src_loc = "29:1"]
    pub type signal_t = GTree;
    use super::gtree_h::GTree;
}
#[c2rust::header_src = "/home/daana/git/luakit/common/tokenize.h:19"]
pub mod tokenize_h {
    #[c2rust::src_loc = "7:9"]
    pub type luakit_token_t = std::ffi::c_uint;
    #[c2rust::src_loc = "282:5"]
    pub const L_TK_ZOOM_TEXT_ONLY: luakit_token_t = 274;
    #[c2rust::src_loc = "281:5"]
    pub const L_TK_ZOOM_LEVEL: luakit_token_t = 273;
    #[c2rust::src_loc = "280:5"]
    pub const L_TK_YPAGE_SIZE: luakit_token_t = 272;
    #[c2rust::src_loc = "279:5"]
    pub const L_TK_YMAX: luakit_token_t = 271;
    #[c2rust::src_loc = "278:5"]
    pub const L_TK_Y: luakit_token_t = 270;
    #[c2rust::src_loc = "277:5"]
    pub const L_TK_XPAGE_SIZE: luakit_token_t = 269;
    #[c2rust::src_loc = "276:5"]
    pub const L_TK_XMAX: luakit_token_t = 268;
    #[c2rust::src_loc = "275:5"]
    pub const L_TK_X: luakit_token_t = 267;
    #[c2rust::src_loc = "274:5"]
    pub const L_TK_WRAP_JS: luakit_token_t = 266;
    #[c2rust::src_loc = "273:5"]
    pub const L_TK_WIN_XID: luakit_token_t = 265;
    #[c2rust::src_loc = "272:5"]
    pub const L_TK_WINDOWS: luakit_token_t = 264;
    #[c2rust::src_loc = "271:5"]
    pub const L_TK_WINDOW: luakit_token_t = 263;
    #[c2rust::src_loc = "270:5"]
    pub const L_TK_WIDTH: luakit_token_t = 262;
    #[c2rust::src_loc = "269:5"]
    pub const L_TK_WEB_PROCESS_ID: luakit_token_t = 261;
    #[c2rust::src_loc = "268:5"]
    pub const L_TK_WEBVIEW: luakit_token_t = 260;
    #[c2rust::src_loc = "267:5"]
    pub const L_TK_WEBSITE_DATA: luakit_token_t = 259;
    #[c2rust::src_loc = "266:5"]
    pub const L_TK_WEBKIT_VERSION: luakit_token_t = 258;
    #[c2rust::src_loc = "265:5"]
    pub const L_TK_WEBKIT_USER_AGENT_VERSION: luakit_token_t = 257;
    #[c2rust::src_loc = "264:5"]
    pub const L_TK_WEBKIT2: luakit_token_t = 256;
    #[c2rust::src_loc = "263:5"]
    pub const L_TK_VPANED: luakit_token_t = 255;
    #[c2rust::src_loc = "262:5"]
    pub const L_TK_VISIBLE_CHILD: luakit_token_t = 254;
    #[c2rust::src_loc = "261:5"]
    pub const L_TK_VISIBLE: luakit_token_t = 253;
    #[c2rust::src_loc = "260:5"]
    pub const L_TK_VIDEOS_DIR: luakit_token_t = 252;
    #[c2rust::src_loc = "259:5"]
    pub const L_TK_VERSION: luakit_token_t = 251;
    #[c2rust::src_loc = "258:5"]
    pub const L_TK_VERBOSE: luakit_token_t = 250;
    #[c2rust::src_loc = "257:5"]
    pub const L_TK_VBOX: luakit_token_t = 249;
    #[c2rust::src_loc = "256:5"]
    pub const L_TK_VALUE: luakit_token_t = 248;
    #[c2rust::src_loc = "255:5"]
    pub const L_TK_USER_AGENT: luakit_token_t = 247;
    #[c2rust::src_loc = "254:5"]
    pub const L_TK_URI: luakit_token_t = 246;
    #[c2rust::src_loc = "253:5"]
    pub const L_TK_URGENCY_HINT: luakit_token_t = 245;
    #[c2rust::src_loc = "252:5"]
    pub const L_TK_TYPE: luakit_token_t = 244;
    #[c2rust::src_loc = "251:5"]
    pub const L_TK_TOTAL_SIZE: luakit_token_t = 243;
    #[c2rust::src_loc = "250:5"]
    pub const L_TK_TOP: luakit_token_t = 242;
    #[c2rust::src_loc = "249:5"]
    pub const L_TK_TOOLTIP: luakit_token_t = 241;
    #[c2rust::src_loc = "248:5"]
    pub const L_TK_TITLE: luakit_token_t = 240;
    #[c2rust::src_loc = "247:5"]
    pub const L_TK_TEXT_CONTENT: luakit_token_t = 239;
    #[c2rust::src_loc = "246:5"]
    pub const L_TK_TEXTWIDTH: luakit_token_t = 238;
    #[c2rust::src_loc = "245:5"]
    pub const L_TK_TEXT: luakit_token_t = 237;
    #[c2rust::src_loc = "244:5"]
    pub const L_TK_TEMPLATES_DIR: luakit_token_t = 236;
    #[c2rust::src_loc = "243:5"]
    pub const L_TK_TAG_NAME: luakit_token_t = 235;
    #[c2rust::src_loc = "242:5"]
    pub const L_TK_SYSTEM_DATA_DIRS: luakit_token_t = 234;
    #[c2rust::src_loc = "241:5"]
    pub const L_TK_SYSTEM_CONFIG_DIRS: luakit_token_t = 233;
    #[c2rust::src_loc = "240:5"]
    pub const L_TK_SWITCH: luakit_token_t = 232;
    #[c2rust::src_loc = "239:5"]
    pub const L_TK_SUGGESTED_FILENAME: luakit_token_t = 231;
    #[c2rust::src_loc = "238:5"]
    pub const L_TK_SUBMIT: luakit_token_t = 230;
    #[c2rust::src_loc = "237:5"]
    pub const L_TK_STYLESHEETS: luakit_token_t = 229;
    #[c2rust::src_loc = "236:5"]
    pub const L_TK_STYLE: luakit_token_t = 228;
    #[c2rust::src_loc = "235:5"]
    pub const L_TK_STOP: luakit_token_t = 227;
    #[c2rust::src_loc = "234:5"]
    pub const L_TK_STATUS: luakit_token_t = 226;
    #[c2rust::src_loc = "233:5"]
    pub const L_TK_STARTED: luakit_token_t = 225;
    #[c2rust::src_loc = "232:5"]
    pub const L_TK_START: luakit_token_t = 224;
    #[c2rust::src_loc = "231:5"]
    pub const L_TK_STACK: luakit_token_t = 223;
    #[c2rust::src_loc = "230:5"]
    pub const L_TK_SSL_TRUSTED: luakit_token_t = 222;
    #[c2rust::src_loc = "229:5"]
    pub const L_TK_SRC: luakit_token_t = 221;
    #[c2rust::src_loc = "228:5"]
    pub const L_TK_SPINNER: luakit_token_t = 220;
    #[c2rust::src_loc = "227:5"]
    pub const L_TK_SPELL_CHECKING_LANGUAGES: luakit_token_t = 219;
    #[c2rust::src_loc = "226:5"]
    pub const L_TK_SPACING: luakit_token_t = 218;
    #[c2rust::src_loc = "225:5"]
    pub const L_TK_SOURCE: luakit_token_t = 217;
    #[c2rust::src_loc = "224:5"]
    pub const L_TK_SOCKET: luakit_token_t = 216;
    #[c2rust::src_loc = "223:5"]
    pub const L_TK_SHOW_TABS: luakit_token_t = 215;
    #[c2rust::src_loc = "222:5"]
    pub const L_TK_SHOW_INSPECTOR: luakit_token_t = 214;
    #[c2rust::src_loc = "221:5"]
    pub const L_TK_SHOW_FRAME: luakit_token_t = 213;
    #[c2rust::src_loc = "220:5"]
    pub const L_TK_SHOW_BORDER: luakit_token_t = 212;
    #[c2rust::src_loc = "219:5"]
    pub const L_TK_SHOW: luakit_token_t = 211;
    #[c2rust::src_loc = "218:5"]
    pub const L_TK_SET_TITLE: luakit_token_t = 210;
    #[c2rust::src_loc = "217:5"]
    pub const L_TK_SET_PDFJS: luakit_token_t = 209;
    #[c2rust::src_loc = "216:5"]
    pub const L_TK_SET_FAVICON_FOR_URI: luakit_token_t = 208;
    #[c2rust::src_loc = "215:5"]
    pub const L_TK_SET_DEFAULT_SIZE: luakit_token_t = 207;
    #[c2rust::src_loc = "214:5"]
    pub const L_TK_SET_DARK_MODE: luakit_token_t = 206;
    #[c2rust::src_loc = "213:5"]
    pub const L_TK_SESSION_STATE: luakit_token_t = 205;
    #[c2rust::src_loc = "212:5"]
    pub const L_TK_SERIF_FONT_FAMILY: luakit_token_t = 204;
    #[c2rust::src_loc = "211:5"]
    pub const L_TK_SEND_KEY: luakit_token_t = 203;
    #[c2rust::src_loc = "210:5"]
    pub const L_TK_SELECT_REGION: luakit_token_t = 202;
    #[c2rust::src_loc = "209:5"]
    pub const L_TK_SELECTION: luakit_token_t = 201;
    #[c2rust::src_loc = "208:5"]
    pub const L_TK_SELECTABLE: luakit_token_t = 200;
    #[c2rust::src_loc = "207:5"]
    pub const L_TK_SECONDARY: luakit_token_t = 199;
    #[c2rust::src_loc = "206:5"]
    pub const L_TK_SEARCH_PREVIOUS: luakit_token_t = 198;
    #[c2rust::src_loc = "205:5"]
    pub const L_TK_SEARCH_NEXT: luakit_token_t = 197;
    #[c2rust::src_loc = "204:5"]
    pub const L_TK_SEARCH: luakit_token_t = 196;
    #[c2rust::src_loc = "203:5"]
    pub const L_TK_SCROLL_Y: luakit_token_t = 195;
    #[c2rust::src_loc = "202:5"]
    pub const L_TK_SCROLL_X: luakit_token_t = 194;
    #[c2rust::src_loc = "201:5"]
    pub const L_TK_SCROLLED: luakit_token_t = 193;
    #[c2rust::src_loc = "200:5"]
    pub const L_TK_SCROLLBARS: luakit_token_t = 192;
    #[c2rust::src_loc = "199:5"]
    pub const L_TK_SCROLL: luakit_token_t = 191;
    #[c2rust::src_loc = "198:5"]
    pub const L_TK_SCREEN: luakit_token_t = 190;
    #[c2rust::src_loc = "197:5"]
    pub const L_TK_SCALE: luakit_token_t = 189;
    #[c2rust::src_loc = "196:5"]
    pub const L_TK_SAVE: luakit_token_t = 188;
    #[c2rust::src_loc = "195:5"]
    pub const L_TK_SANS_SERIF_FONT_FAMILY: luakit_token_t = 187;
    #[c2rust::src_loc = "194:5"]
    pub const L_TK_ROOT_WIN_XID: luakit_token_t = 186;
    #[c2rust::src_loc = "193:5"]
    pub const L_TK_RIGHT: luakit_token_t = 185;
    #[c2rust::src_loc = "192:5"]
    pub const L_TK_RESOURCE_PATH: luakit_token_t = 184;
    #[c2rust::src_loc = "191:5"]
    pub const L_TK_REPLACE: luakit_token_t = 183;
    #[c2rust::src_loc = "190:5"]
    pub const L_TK_REORDER: luakit_token_t = 182;
    #[c2rust::src_loc = "189:5"]
    pub const L_TK_REMOVE_EVENT_LISTENER: luakit_token_t = 181;
    #[c2rust::src_loc = "188:5"]
    pub const L_TK_REMOVE: luakit_token_t = 180;
    #[c2rust::src_loc = "187:5"]
    pub const L_TK_RELOAD_BYPASS_CACHE: luakit_token_t = 179;
    #[c2rust::src_loc = "186:5"]
    pub const L_TK_RELOAD: luakit_token_t = 178;
    #[c2rust::src_loc = "185:5"]
    pub const L_TK_RECT: luakit_token_t = 177;
    #[c2rust::src_loc = "184:5"]
    pub const L_TK_QUERY: luakit_token_t = 176;
    #[c2rust::src_loc = "183:5"]
    pub const L_TK_PUBLIC_SHARE_DIR: luakit_token_t = 175;
    #[c2rust::src_loc = "182:5"]
    pub const L_TK_PROXY_URI: luakit_token_t = 174;
    #[c2rust::src_loc = "181:5"]
    pub const L_TK_PROGRESS: luakit_token_t = 173;
    #[c2rust::src_loc = "180:5"]
    pub const L_TK_PROCESS_LIMIT: luakit_token_t = 172;
    #[c2rust::src_loc = "179:5"]
    pub const L_TK_PRIVATE: luakit_token_t = 171;
    #[c2rust::src_loc = "178:5"]
    pub const L_TK_PRINT_BACKGROUNDS: luakit_token_t = 170;
    #[c2rust::src_loc = "177:5"]
    pub const L_TK_PRIMARY: luakit_token_t = 169;
    #[c2rust::src_loc = "176:5"]
    pub const L_TK_PREV_SIBLING: luakit_token_t = 168;
    #[c2rust::src_loc = "175:5"]
    pub const L_TK_POSITION: luakit_token_t = 167;
    #[c2rust::src_loc = "174:5"]
    pub const L_TK_PLUGGED: luakit_token_t = 166;
    #[c2rust::src_loc = "173:5"]
    pub const L_TK_PICTURES_DIR: luakit_token_t = 165;
    #[c2rust::src_loc = "172:5"]
    pub const L_TK_PICTOGRAPH_FONT_FAMILY: luakit_token_t = 164;
    #[c2rust::src_loc = "171:5"]
    pub const L_TK_PATTERN: luakit_token_t = 163;
    #[c2rust::src_loc = "170:5"]
    pub const L_TK_PARENT: luakit_token_t = 162;
    #[c2rust::src_loc = "169:5"]
    pub const L_TK_PACK2: luakit_token_t = 161;
    #[c2rust::src_loc = "168:5"]
    pub const L_TK_PACK1: luakit_token_t = 160;
    #[c2rust::src_loc = "167:5"]
    pub const L_TK_PACK: luakit_token_t = 159;
    #[c2rust::src_loc = "166:5"]
    pub const L_TK_OWNER_DOCUMENT: luakit_token_t = 158;
    #[c2rust::src_loc = "165:5"]
    pub const L_TK_OVERLAY: luakit_token_t = 157;
    #[c2rust::src_loc = "164:5"]
    pub const L_TK_OPTIONS: luakit_token_t = 156;
    #[c2rust::src_loc = "163:5"]
    pub const L_TK_NOUNIQUE: luakit_token_t = 155;
    #[c2rust::src_loc = "162:5"]
    pub const L_TK_NOTEBOOK: luakit_token_t = 154;
    #[c2rust::src_loc = "161:5"]
    pub const L_TK_NEXT_SIBLING: luakit_token_t = 153;
    #[c2rust::src_loc = "160:5"]
    pub const L_TK_NAME: luakit_token_t = 152;
    #[c2rust::src_loc = "159:5"]
    pub const L_TK_MUSIC_DIR: luakit_token_t = 151;
    #[c2rust::src_loc = "158:5"]
    pub const L_TK_MONOSPACE_FONT_FAMILY: luakit_token_t = 150;
    #[c2rust::src_loc = "157:5"]
    pub const L_TK_MIN_SIZE: luakit_token_t = 149;
    #[c2rust::src_loc = "156:5"]
    pub const L_TK_MINIMUM_FONT_SIZE: luakit_token_t = 148;
    #[c2rust::src_loc = "155:5"]
    pub const L_TK_MIME_TYPE: luakit_token_t = 147;
    #[c2rust::src_loc = "154:5"]
    pub const L_TK_MEDIA_PLAYBACK_REQUIRES_GESTURE: luakit_token_t = 146;
    #[c2rust::src_loc = "153:5"]
    pub const L_TK_MEDIA_PLAYBACK_ALLOWS_INLINE: luakit_token_t = 145;
    #[c2rust::src_loc = "152:5"]
    pub const L_TK_MAXIMIZED: luakit_token_t = 144;
    #[c2rust::src_loc = "151:5"]
    pub const L_TK_MARGIN_TOP: luakit_token_t = 143;
    #[c2rust::src_loc = "150:5"]
    pub const L_TK_MARGIN_RIGHT: luakit_token_t = 142;
    #[c2rust::src_loc = "149:5"]
    pub const L_TK_MARGIN_LEFT: luakit_token_t = 141;
    #[c2rust::src_loc = "148:5"]
    pub const L_TK_MARGIN_BOTTOM: luakit_token_t = 140;
    #[c2rust::src_loc = "147:5"]
    pub const L_TK_MARGIN: luakit_token_t = 139;
    #[c2rust::src_loc = "146:5"]
    pub const L_TK_LOAD_STRING: luakit_token_t = 138;
    #[c2rust::src_loc = "145:5"]
    pub const L_TK_LOADING: luakit_token_t = 137;
    #[c2rust::src_loc = "144:5"]
    pub const L_TK_LEFT: luakit_token_t = 136;
    #[c2rust::src_loc = "143:5"]
    pub const L_TK_LAST_CHILD: luakit_token_t = 135;
    #[c2rust::src_loc = "142:5"]
    pub const L_TK_LABEL: luakit_token_t = 134;
    #[c2rust::src_loc = "141:5"]
    pub const L_TK_JAVASCRIPT_CAN_OPEN_WINDOWS_AUTOMATICALLY: luakit_token_t = 133;
    #[c2rust::src_loc = "140:5"]
    pub const L_TK_JAVASCRIPT_CAN_ACCESS_CLIPBOARD: luakit_token_t = 132;
    #[c2rust::src_loc = "139:5"]
    pub const L_TK_IS_PLAYING_AUDIO: luakit_token_t = 131;
    #[c2rust::src_loc = "138:5"]
    pub const L_TK_IS_LOADING: luakit_token_t = 130;
    #[c2rust::src_loc = "137:5"]
    pub const L_TK_IS_ALIVE: luakit_token_t = 129;
    #[c2rust::src_loc = "136:5"]
    pub const L_TK_INVALIDATE: luakit_token_t = 128;
    #[c2rust::src_loc = "135:5"]
    pub const L_TK_INTERVAL: luakit_token_t = 127;
    #[c2rust::src_loc = "134:5"]
    pub const L_TK_INSTALL_PATHS: luakit_token_t = 126;
    #[c2rust::src_loc = "133:5"]
    pub const L_TK_INSTALL_PATH: luakit_token_t = 125;
    #[c2rust::src_loc = "132:5"]
    pub const L_TK_INSPECTOR: luakit_token_t = 124;
    #[c2rust::src_loc = "131:5"]
    pub const L_TK_INSERT: luakit_token_t = 123;
    #[c2rust::src_loc = "130:5"]
    pub const L_TK_INNER_WIDTH: luakit_token_t = 122;
    #[c2rust::src_loc = "129:5"]
    pub const L_TK_INNER_HTML: luakit_token_t = 121;
    #[c2rust::src_loc = "128:5"]
    pub const L_TK_INNER_HEIGHT: luakit_token_t = 120;
    #[c2rust::src_loc = "127:5"]
    pub const L_TK_INDEXOF: luakit_token_t = 119;
    #[c2rust::src_loc = "126:5"]
    pub const L_TK_IMAGE: luakit_token_t = 118;
    #[c2rust::src_loc = "125:5"]
    pub const L_TK_ID: luakit_token_t = 117;
    #[c2rust::src_loc = "124:5"]
    pub const L_TK_ICON: luakit_token_t = 116;
    #[c2rust::src_loc = "123:5"]
    pub const L_TK_HREF: luakit_token_t = 115;
    #[c2rust::src_loc = "122:5"]
    pub const L_TK_HPANED: luakit_token_t = 114;
    #[c2rust::src_loc = "121:5"]
    pub const L_TK_HOVERED_URI: luakit_token_t = 113;
    #[c2rust::src_loc = "120:5"]
    pub const L_TK_HOMOGENEOUS: luakit_token_t = 112;
    #[c2rust::src_loc = "119:5"]
    pub const L_TK_HISTORY: luakit_token_t = 111;
    #[c2rust::src_loc = "118:5"]
    pub const L_TK_HIDE: luakit_token_t = 110;
    #[c2rust::src_loc = "117:5"]
    pub const L_TK_HEIGHT: luakit_token_t = 109;
    #[c2rust::src_loc = "116:5"]
    pub const L_TK_HBOX: luakit_token_t = 108;
    #[c2rust::src_loc = "115:5"]
    pub const L_TK_HARDWARE_ACCELERATION_POLICY: luakit_token_t = 107;
    #[c2rust::src_loc = "114:5"]
    pub const L_TK_GO_FORWARD: luakit_token_t = 106;
    #[c2rust::src_loc = "113:5"]
    pub const L_TK_GO_BACK: luakit_token_t = 105;
    #[c2rust::src_loc = "112:5"]
    pub const L_TK_GET_TITLE: luakit_token_t = 104;
    #[c2rust::src_loc = "111:5"]
    pub const L_TK_GET_SOURCE: luakit_token_t = 103;
    #[c2rust::src_loc = "110:5"]
    pub const L_TK_FULLSCREEN: luakit_token_t = 102;
    #[c2rust::src_loc = "109:5"]
    pub const L_TK_FONT: luakit_token_t = 101;
    #[c2rust::src_loc = "108:5"]
    pub const L_TK_FOCUSED: luakit_token_t = 100;
    #[c2rust::src_loc = "107:5"]
    pub const L_TK_FOCUS: luakit_token_t = 99;
    #[c2rust::src_loc = "106:5"]
    pub const L_TK_FIRST_CHILD: luakit_token_t = 98;
    #[c2rust::src_loc = "105:5"]
    pub const L_TK_FINISHED: luakit_token_t = 97;
    #[c2rust::src_loc = "104:5"]
    pub const L_TK_FILL: luakit_token_t = 96;
    #[c2rust::src_loc = "103:5"]
    pub const L_TK_FILENAME: luakit_token_t = 95;
    #[c2rust::src_loc = "102:5"]
    pub const L_TK_FG: luakit_token_t = 94;
    #[c2rust::src_loc = "101:5"]
    pub const L_TK_FETCH: luakit_token_t = 93;
    #[c2rust::src_loc = "100:5"]
    pub const L_TK_FANTASY_FONT_FAMILY: luakit_token_t = 92;
    #[c2rust::src_loc = "99:5"]
    pub const L_TK_EXECPATH: luakit_token_t = 91;
    #[c2rust::src_loc = "98:5"]
    pub const L_TK_EVENTBOX: luakit_token_t = 90;
    #[c2rust::src_loc = "97:5"]
    pub const L_TK_EVAL_JS: luakit_token_t = 89;
    #[c2rust::src_loc = "96:5"]
    pub const L_TK_ERROR: luakit_token_t = 88;
    #[c2rust::src_loc = "95:5"]
    pub const L_TK_ENTRY: luakit_token_t = 87;
    #[c2rust::src_loc = "94:5"]
    pub const L_TK_END: luakit_token_t = 86;
    #[c2rust::src_loc = "93:5"]
    pub const L_TK_ENABLE_XSS_AUDITOR: luakit_token_t = 85;
    #[c2rust::src_loc = "92:5"]
    pub const L_TK_ENABLE_WRITE_CONSOLE_MESSAGES_TO_STDOUT: luakit_token_t = 84;
    #[c2rust::src_loc = "91:5"]
    pub const L_TK_ENABLE_WEBGL: luakit_token_t = 83;
    #[c2rust::src_loc = "90:5"]
    pub const L_TK_ENABLE_WEBAUDIO: luakit_token_t = 82;
    #[c2rust::src_loc = "89:5"]
    pub const L_TK_ENABLE_TABS_TO_LINKS: luakit_token_t = 81;
    #[c2rust::src_loc = "88:5"]
    pub const L_TK_ENABLE_SPELL_CHECKING: luakit_token_t = 80;
    #[c2rust::src_loc = "87:5"]
    pub const L_TK_ENABLE_SPATIAL_NAVIGATION: luakit_token_t = 79;
    #[c2rust::src_loc = "86:5"]
    pub const L_TK_ENABLE_SMOOTH_SCROLLING: luakit_token_t = 78;
    #[c2rust::src_loc = "85:5"]
    pub const L_TK_ENABLE_SITE_SPECIFIC_QUIRKS: luakit_token_t = 77;
    #[c2rust::src_loc = "84:5"]
    pub const L_TK_ENABLE_SCRIPTS: luakit_token_t = 76;
    #[c2rust::src_loc = "83:5"]
    pub const L_TK_ENABLE_RESIZABLE_TEXT_AREAS: luakit_token_t = 75;
    #[c2rust::src_loc = "82:5"]
    pub const L_TK_ENABLE_PLUGINS: luakit_token_t = 74;
    #[c2rust::src_loc = "81:5"]
    pub const L_TK_ENABLE_PAGE_CACHE: luakit_token_t = 73;
    #[c2rust::src_loc = "80:5"]
    pub const L_TK_ENABLE_MEDIA_STREAM: luakit_token_t = 72;
    #[c2rust::src_loc = "79:5"]
    pub const L_TK_ENABLE_MEDIASOURCE: luakit_token_t = 71;
    #[c2rust::src_loc = "78:5"]
    pub const L_TK_ENABLE_JAVASCRIPT: luakit_token_t = 70;
    #[c2rust::src_loc = "77:5"]
    pub const L_TK_ENABLE_JAVA: luakit_token_t = 69;
    #[c2rust::src_loc = "76:5"]
    pub const L_TK_ENABLE_HYPERLINK_AUDITING: luakit_token_t = 68;
    #[c2rust::src_loc = "75:5"]
    pub const L_TK_ENABLE_HTML5_LOCAL_STORAGE: luakit_token_t = 67;
    #[c2rust::src_loc = "74:5"]
    pub const L_TK_ENABLE_HTML5_DATABASE: luakit_token_t = 66;
    #[c2rust::src_loc = "73:5"]
    pub const L_TK_ENABLE_FULLSCREEN: luakit_token_t = 65;
    #[c2rust::src_loc = "72:5"]
    pub const L_TK_ENABLE_FRAME_FLATTENING: luakit_token_t = 64;
    #[c2rust::src_loc = "71:5"]
    pub const L_TK_ENABLE_DNS_PREFETCHING: luakit_token_t = 63;
    #[c2rust::src_loc = "70:5"]
    pub const L_TK_ENABLE_DEVELOPER_EXTRAS: luakit_token_t = 62;
    #[c2rust::src_loc = "69:5"]
    pub const L_TK_ENABLE_CARET_BROWSING: luakit_token_t = 61;
    #[c2rust::src_loc = "68:5"]
    pub const L_TK_ENABLE_ACCELERATED_2D_CANVAS: luakit_token_t = 60;
    #[c2rust::src_loc = "67:5"]
    pub const L_TK_ELEMENT_FROM_POINT: luakit_token_t = 59;
    #[c2rust::src_loc = "66:5"]
    pub const L_TK_ELAPSED_TIME: luakit_token_t = 58;
    #[c2rust::src_loc = "65:5"]
    pub const L_TK_EDITABLE: luakit_token_t = 57;
    #[c2rust::src_loc = "64:5"]
    pub const L_TK_DRAW_COMPOSITING_INDICATORS: luakit_token_t = 56;
    #[c2rust::src_loc = "63:5"]
    pub const L_TK_DRAWING_AREA: luakit_token_t = 55;
    #[c2rust::src_loc = "62:5"]
    pub const L_TK_DOWNLOAD_DIR: luakit_token_t = 54;
    #[c2rust::src_loc = "61:5"]
    pub const L_TK_DOCUMENTS_DIR: luakit_token_t = 53;
    #[c2rust::src_loc = "60:5"]
    pub const L_TK_DOCUMENT: luakit_token_t = 52;
    #[c2rust::src_loc = "59:5"]
    pub const L_TK_DEV_PATHS: luakit_token_t = 51;
    #[c2rust::src_loc = "58:5"]
    pub const L_TK_DESTROY: luakit_token_t = 50;
    #[c2rust::src_loc = "57:5"]
    pub const L_TK_DESTINATION: luakit_token_t = 49;
    #[c2rust::src_loc = "56:5"]
    pub const L_TK_DESKTOP_DIR: luakit_token_t = 48;
    #[c2rust::src_loc = "55:5"]
    pub const L_TK_DEFAULT_MONOSPACE_FONT_SIZE: luakit_token_t = 47;
    #[c2rust::src_loc = "54:5"]
    pub const L_TK_DEFAULT_FONT_SIZE: luakit_token_t = 46;
    #[c2rust::src_loc = "53:5"]
    pub const L_TK_DEFAULT_FONT_FAMILY: luakit_token_t = 45;
    #[c2rust::src_loc = "52:5"]
    pub const L_TK_DEFAULT_CHARSET: luakit_token_t = 44;
    #[c2rust::src_loc = "51:5"]
    pub const L_TK_DECORATED: luakit_token_t = 43;
    #[c2rust::src_loc = "50:5"]
    pub const L_TK_DATA_DIR: luakit_token_t = 42;
    #[c2rust::src_loc = "49:5"]
    pub const L_TK_CURSIVE_FONT_FAMILY: luakit_token_t = 41;
    #[c2rust::src_loc = "48:5"]
    pub const L_TK_CURRENT_SIZE: luakit_token_t = 40;
    #[c2rust::src_loc = "47:5"]
    pub const L_TK_CURRENT: luakit_token_t = 39;
    #[c2rust::src_loc = "46:5"]
    pub const L_TK_CSS: luakit_token_t = 38;
    #[c2rust::src_loc = "45:5"]
    pub const L_TK_CREATE_ELEMENT: luakit_token_t = 37;
    #[c2rust::src_loc = "44:5"]
    pub const L_TK_CRASH: luakit_token_t = 36;
    #[c2rust::src_loc = "43:5"]
    pub const L_TK_COUNT: luakit_token_t = 35;
    #[c2rust::src_loc = "42:5"]
    pub const L_TK_COOKIES_STORAGE: luakit_token_t = 34;
    #[c2rust::src_loc = "41:5"]
    pub const L_TK_CONFPATH: luakit_token_t = 33;
    #[c2rust::src_loc = "40:5"]
    pub const L_TK_CONFIG_DIR: luakit_token_t = 32;
    #[c2rust::src_loc = "39:5"]
    pub const L_TK_CLOSE_INSPECTOR: luakit_token_t = 31;
    #[c2rust::src_loc = "38:5"]
    pub const L_TK_CLIPBOARD: luakit_token_t = 30;
    #[c2rust::src_loc = "37:5"]
    pub const L_TK_CLIENT_RECTS: luakit_token_t = 29;
    #[c2rust::src_loc = "36:5"]
    pub const L_TK_CLICK: luakit_token_t = 28;
    #[c2rust::src_loc = "35:5"]
    pub const L_TK_CLEAR_SEARCH: luakit_token_t = 27;
    #[c2rust::src_loc = "34:5"]
    pub const L_TK_CLEAR: luakit_token_t = 26;
    #[c2rust::src_loc = "33:5"]
    pub const L_TK_CHILD_COUNT: luakit_token_t = 25;
    #[c2rust::src_loc = "32:5"]
    pub const L_TK_CHILDREN: luakit_token_t = 24;
    #[c2rust::src_loc = "31:5"]
    pub const L_TK_CHILD: luakit_token_t = 23;
    #[c2rust::src_loc = "30:5"]
    pub const L_TK_CHECKED: luakit_token_t = 22;
    #[c2rust::src_loc = "29:5"]
    pub const L_TK_CERTIFICATE: luakit_token_t = 21;
    #[c2rust::src_loc = "28:5"]
    pub const L_TK_CENTER: luakit_token_t = 20;
    #[c2rust::src_loc = "27:5"]
    pub const L_TK_CAN_GO_FORWARD: luakit_token_t = 19;
    #[c2rust::src_loc = "26:5"]
    pub const L_TK_CAN_GO_BACK: luakit_token_t = 18;
    #[c2rust::src_loc = "25:5"]
    pub const L_TK_CAN_FOCUS: luakit_token_t = 17;
    #[c2rust::src_loc = "24:5"]
    pub const L_TK_CACHE_DIR: luakit_token_t = 16;
    #[c2rust::src_loc = "23:5"]
    pub const L_TK_BOTTOM: luakit_token_t = 15;
    #[c2rust::src_loc = "22:5"]
    pub const L_TK_BODY: luakit_token_t = 14;
    #[c2rust::src_loc = "21:5"]
    pub const L_TK_BG: luakit_token_t = 13;
    #[c2rust::src_loc = "20:5"]
    pub const L_TK_BASELINE: luakit_token_t = 12;
    #[c2rust::src_loc = "19:5"]
    pub const L_TK_AUTO_LOAD_IMAGES: luakit_token_t = 11;
    #[c2rust::src_loc = "18:5"]
    pub const L_TK_ATTR: luakit_token_t = 10;
    #[c2rust::src_loc = "17:5"]
    pub const L_TK_APPEND: luakit_token_t = 9;
    #[c2rust::src_loc = "16:5"]
    pub const L_TK_ALLOW_UNIVERSAL_ACCESS_FROM_FILE_URLS: luakit_token_t = 8;
    #[c2rust::src_loc = "15:5"]
    pub const L_TK_ALLOW_OVERWRITE: luakit_token_t = 7;
    #[c2rust::src_loc = "14:5"]
    pub const L_TK_ALLOW_MODAL_DIALOGS: luakit_token_t = 6;
    #[c2rust::src_loc = "13:5"]
    pub const L_TK_ALLOW_FILE_ACCESS_FROM_FILE_URLS: luakit_token_t = 5;
    #[c2rust::src_loc = "12:5"]
    pub const L_TK_ALLOW_CERTIFICATE: luakit_token_t = 4;
    #[c2rust::src_loc = "11:5"]
    pub const L_TK_ALIGN: luakit_token_t = 3;
    #[c2rust::src_loc = "10:5"]
    pub const L_TK_ADD_EVENT_LISTENER: luakit_token_t = 2;
    #[c2rust::src_loc = "9:5"]
    pub const L_TK_ACCEPT_POLICY: luakit_token_t = 1;
    #[c2rust::src_loc = "8:5"]
    pub const L_TK_UNKNOWN: luakit_token_t = 0;
}
#[c2rust::header_src = "/home/daana/git/luakit/common/luaclass.h:19"]
pub mod luaclass_h {
    #[c2rust::src_loc = "32:1"]
    pub type lua_class_property_array_t = GHashTable;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "39:9"]
    pub struct lua_object_t {
        pub signals: *mut signal_t,
    }
    #[c2rust::src_loc = "43:1"]
    pub type lua_class_allocator_t = Option::<
        unsafe extern "C" fn(*mut lua_State) -> *mut lua_object_t,
    >;
    #[c2rust::src_loc = "45:1"]
    pub type lua_class_propfunc_t = Option::<
        unsafe extern "C" fn(*mut lua_State, *mut lua_object_t) -> gint,
    >;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "47:9"]
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
    extern "C" {
        #[c2rust::src_loc = "88:1"]
        pub fn luaH_checkudata(
            _: *mut lua_State,
            _: gint,
            _: *mut lua_class_t,
        ) -> gpointer;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/common.h:19"]
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
#[c2rust::header_src = "/home/daana/git/luakit/clib/widget.h:20"]
pub mod widget_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "65:8"]
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
    #[c2rust::src_loc = "41:1"]
    pub type widget_destructor_t = unsafe extern "C" fn(*mut widget_t) -> ();
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "58:15"]
    pub struct widget_info_t {
        pub tok: luakit_token_t,
        pub name: *const gchar,
        pub wc: Option::<widget_constructor_t>,
    }
    #[c2rust::src_loc = "40:1"]
    pub type widget_constructor_t = unsafe extern "C" fn(
        *mut lua_State,
        *mut widget_t,
        luakit_token_t,
    ) -> *mut widget_t;
    #[inline]
    #[c2rust::src_loc = "95:1"]
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
    extern "C" {
        #[c2rust::src_loc = "90:20"]
        pub static mut widget_class: lua_class_t;
    }
}
#[c2rust::header_src = "/usr/include/gtk-3.0/gtk/gtkcssprovider.h:20"]
pub mod gtkcssprovider_h {
    #[c2rust::src_loc = "64:1"]
    pub type GtkCssProvider = _GtkCssProvider;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "68:8"]
    pub struct _GtkCssProvider {
        pub parent_instance: GObject,
        pub priv_0: *mut GtkCssProviderPrivate,
    }
    #[c2rust::src_loc = "66:1"]
    pub type GtkCssProviderPrivate = _GtkCssProviderPrivate;
    use super::gobject_h::GObject;
    extern "C" {
        #[c2rust::src_loc = "66:16"]
        pub type _GtkCssProviderPrivate;
    }
}
#[c2rust::header_src = "/usr/include/gtk-3.0/gtk/gtktypes.h:20"]
pub mod gtktypes_h {
    #[c2rust::src_loc = "46:1"]
    pub type GtkWidget = _GtkWidget;
    use super::gtkwidget_h::_GtkWidget;
}
#[c2rust::header_src = "/usr/include/gtk-3.0/gtk/gtkwidget.h:20"]
pub mod gtkwidget_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "133:8"]
    pub struct _GtkWidget {
        pub parent_instance: GInitiallyUnowned,
        pub priv_0: *mut GtkWidgetPrivate,
    }
    #[c2rust::src_loc = "66:1"]
    pub type GtkWidgetPrivate = _GtkWidgetPrivate;
    use super::gobject_h::GInitiallyUnowned;
    use super::gtype_h::GType;
    use super::gtktypes_h::GtkWidget;
    use super::gtypes_h::gint;
    extern "C" {
        #[c2rust::src_loc = "66:16"]
        pub type _GtkWidgetPrivate;
        #[c2rust::src_loc = "612:1"]
        pub fn gtk_widget_get_type() -> GType;
        #[c2rust::src_loc = "625:1"]
        pub fn gtk_widget_show(widget: *mut GtkWidget);
        #[c2rust::src_loc = "653:1"]
        pub fn gtk_widget_queue_draw_area(
            widget: *mut GtkWidget,
            x: gint,
            y: gint,
            width: gint,
            height: gint,
        );
        #[c2rust::src_loc = "922:1"]
        pub fn gtk_widget_get_allocated_width(widget: *mut GtkWidget) -> std::ffi::c_int;
        #[c2rust::src_loc = "924:1"]
        pub fn gtk_widget_get_allocated_height(
            widget: *mut GtkWidget,
        ) -> std::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/cairo/cairo.h:20"]
pub mod cairo_h {
    #[c2rust::src_loc = "135:1"]
    pub type cairo_t = _cairo;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "532:16"]
    pub struct _cairo_rectangle_int {
        pub x: std::ffi::c_int,
        pub y: std::ffi::c_int,
        pub width: std::ffi::c_int,
        pub height: std::ffi::c_int,
    }
    #[c2rust::src_loc = "532:1"]
    pub type cairo_rectangle_int_t = _cairo_rectangle_int;
    extern "C" {
        #[c2rust::src_loc = "135:16"]
        pub type _cairo;
    }
}
#[c2rust::header_src = "/usr/include/gtk-3.0/gdk/gdktypes.h:20"]
pub mod gdktypes_h {
    #[c2rust::src_loc = "93:1"]
    pub type GdkRectangle = cairo_rectangle_int_t;
    #[c2rust::src_loc = "143:1"]
    pub type GdkWindow = _GdkWindow;
    use super::cairo_h::cairo_rectangle_int_t;
    extern "C" {
        #[c2rust::src_loc = "143:16"]
        pub type _GdkWindow;
    }
}
#[c2rust::header_src = "/usr/include/gtk-3.0/gdk/gdkevents.h:20"]
pub mod gdkevents_h {
    #[c2rust::src_loc = "309:9"]
    pub type GdkEventType = std::ffi::c_int;
    #[c2rust::src_loc = "361:3"]
    pub const GDK_EVENT_LAST: GdkEventType = 48;
    #[c2rust::src_loc = "360:3"]
    pub const GDK_PAD_GROUP_MODE: GdkEventType = 47;
    #[c2rust::src_loc = "359:3"]
    pub const GDK_PAD_STRIP: GdkEventType = 46;
    #[c2rust::src_loc = "358:3"]
    pub const GDK_PAD_RING: GdkEventType = 45;
    #[c2rust::src_loc = "357:3"]
    pub const GDK_PAD_BUTTON_RELEASE: GdkEventType = 44;
    #[c2rust::src_loc = "356:3"]
    pub const GDK_PAD_BUTTON_PRESS: GdkEventType = 43;
    #[c2rust::src_loc = "355:3"]
    pub const GDK_TOUCHPAD_PINCH: GdkEventType = 42;
    #[c2rust::src_loc = "354:3"]
    pub const GDK_TOUCHPAD_SWIPE: GdkEventType = 41;
    #[c2rust::src_loc = "353:3"]
    pub const GDK_TOUCH_CANCEL: GdkEventType = 40;
    #[c2rust::src_loc = "352:3"]
    pub const GDK_TOUCH_END: GdkEventType = 39;
    #[c2rust::src_loc = "351:3"]
    pub const GDK_TOUCH_UPDATE: GdkEventType = 38;
    #[c2rust::src_loc = "350:3"]
    pub const GDK_TOUCH_BEGIN: GdkEventType = 37;
    #[c2rust::src_loc = "349:3"]
    pub const GDK_DAMAGE: GdkEventType = 36;
    #[c2rust::src_loc = "348:3"]
    pub const GDK_GRAB_BROKEN: GdkEventType = 35;
    #[c2rust::src_loc = "347:3"]
    pub const GDK_OWNER_CHANGE: GdkEventType = 34;
    #[c2rust::src_loc = "346:3"]
    pub const GDK_SETTING: GdkEventType = 33;
    #[c2rust::src_loc = "345:3"]
    pub const GDK_WINDOW_STATE: GdkEventType = 32;
    #[c2rust::src_loc = "344:3"]
    pub const GDK_SCROLL: GdkEventType = 31;
    #[c2rust::src_loc = "343:3"]
    pub const GDK_VISIBILITY_NOTIFY: GdkEventType = 29;
    #[c2rust::src_loc = "342:3"]
    pub const GDK_CLIENT_EVENT: GdkEventType = 28;
    #[c2rust::src_loc = "341:3"]
    pub const GDK_DROP_FINISHED: GdkEventType = 27;
    #[c2rust::src_loc = "340:3"]
    pub const GDK_DROP_START: GdkEventType = 26;
    #[c2rust::src_loc = "339:3"]
    pub const GDK_DRAG_STATUS: GdkEventType = 25;
    #[c2rust::src_loc = "338:3"]
    pub const GDK_DRAG_MOTION: GdkEventType = 24;
    #[c2rust::src_loc = "337:3"]
    pub const GDK_DRAG_LEAVE: GdkEventType = 23;
    #[c2rust::src_loc = "336:3"]
    pub const GDK_DRAG_ENTER: GdkEventType = 22;
    #[c2rust::src_loc = "335:3"]
    pub const GDK_PROXIMITY_OUT: GdkEventType = 21;
    #[c2rust::src_loc = "334:3"]
    pub const GDK_PROXIMITY_IN: GdkEventType = 20;
    #[c2rust::src_loc = "333:3"]
    pub const GDK_SELECTION_NOTIFY: GdkEventType = 19;
    #[c2rust::src_loc = "332:3"]
    pub const GDK_SELECTION_REQUEST: GdkEventType = 18;
    #[c2rust::src_loc = "331:3"]
    pub const GDK_SELECTION_CLEAR: GdkEventType = 17;
    #[c2rust::src_loc = "330:3"]
    pub const GDK_PROPERTY_NOTIFY: GdkEventType = 16;
    #[c2rust::src_loc = "329:3"]
    pub const GDK_UNMAP: GdkEventType = 15;
    #[c2rust::src_loc = "328:3"]
    pub const GDK_MAP: GdkEventType = 14;
    #[c2rust::src_loc = "327:3"]
    pub const GDK_CONFIGURE: GdkEventType = 13;
    #[c2rust::src_loc = "326:3"]
    pub const GDK_FOCUS_CHANGE: GdkEventType = 12;
    #[c2rust::src_loc = "325:3"]
    pub const GDK_LEAVE_NOTIFY: GdkEventType = 11;
    #[c2rust::src_loc = "324:3"]
    pub const GDK_ENTER_NOTIFY: GdkEventType = 10;
    #[c2rust::src_loc = "323:3"]
    pub const GDK_KEY_RELEASE: GdkEventType = 9;
    #[c2rust::src_loc = "322:3"]
    pub const GDK_KEY_PRESS: GdkEventType = 8;
    #[c2rust::src_loc = "321:3"]
    pub const GDK_BUTTON_RELEASE: GdkEventType = 7;
    #[c2rust::src_loc = "320:3"]
    pub const GDK_TRIPLE_BUTTON_PRESS: GdkEventType = 6;
    #[c2rust::src_loc = "319:3"]
    pub const GDK_3BUTTON_PRESS: GdkEventType = 6;
    #[c2rust::src_loc = "318:3"]
    pub const GDK_DOUBLE_BUTTON_PRESS: GdkEventType = 5;
    #[c2rust::src_loc = "317:3"]
    pub const GDK_2BUTTON_PRESS: GdkEventType = 5;
    #[c2rust::src_loc = "316:3"]
    pub const GDK_BUTTON_PRESS: GdkEventType = 4;
    #[c2rust::src_loc = "315:3"]
    pub const GDK_MOTION_NOTIFY: GdkEventType = 3;
    #[c2rust::src_loc = "314:3"]
    pub const GDK_EXPOSE: GdkEventType = 2;
    #[c2rust::src_loc = "313:3"]
    pub const GDK_DESTROY: GdkEventType = 1;
    #[c2rust::src_loc = "312:3"]
    pub const GDK_DELETE: GdkEventType = 0;
    #[c2rust::src_loc = "311:3"]
    pub const GDK_NOTHING: GdkEventType = -1;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "965:8"]
    pub struct _GdkEventFocus {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub in_0: gint16,
    }
    #[c2rust::src_loc = "132:1"]
    pub type GdkEventFocus = _GdkEventFocus;
    use super::gdktypes_h::GdkWindow;
    use super::glibconfig_h::{gint8, gint16};
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gmessages.h:19"]
pub mod gmessages_h {
    #[c2rust::src_loc = "320:9"]
    pub const G_LOG_DOMAIN: std::ffi::c_int = 0 as std::ffi::c_int;
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
#[c2rust::header_src = "/usr/include/luajit-2.1/lauxlib.h:19"]
pub mod lauxlib_h {
    use super::lua_h::lua_State;
    extern "C" {
        #[c2rust::src_loc = "53:1"]
        pub fn luaL_error(
            L: *mut lua_State,
            fmt: *const std::ffi::c_char,
            _: ...
        ) -> std::ffi::c_int;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/luaobject.h:19"]
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
    use super::gtypes_h::{gint, gpointer, gchar};
    use super::tokenize_h::{luakit_token_t, L_TK_UNKNOWN};
    extern "C" {
        #[c2rust::src_loc = "40:1"]
        pub fn luaH_object_incref(L: *mut lua_State, tud: gint, oud: gint) -> gpointer;
        #[c2rust::src_loc = "164:1"]
        pub fn luaH_object_emit_signal(
            L: *mut lua_State,
            oud: gint,
            name: *const gchar,
            nargs: gint,
            nret: gint,
        ) -> gint;
        #[c2rust::src_loc = "171:1"]
        pub fn luaH_object_property_signal(
            _: *mut lua_State,
            _: gint,
            _: luakit_token_t,
        ) -> gint;
    }
}
#[c2rust::header_src = "/usr/include/gtk-3.0/gtk/gtkdrawingarea.h:20"]
pub mod gtkdrawingarea_h {
    use super::gtktypes_h::GtkWidget;
    extern "C" {
        #[c2rust::src_loc = "71:1"]
        pub fn gtk_drawing_area_new() -> *mut GtkWidget;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/widgets/common.h:20"]
pub mod widgets_common_h {
    use super::gtktypes_h::GtkWidget;
    use super::gdkevents_h::GdkEventFocus;
    use super::widget_h::widget_t;
    use super::gtypes_h::{gboolean, gint};
    use super::lua_h::lua_State;
    use super::gdktypes_h::GdkRectangle;
    extern "C" {
        #[c2rust::src_loc = "102:1"]
        pub fn focus_cb(
            _: *mut GtkWidget,
            _: *mut GdkEventFocus,
            _: *mut widget_t,
        ) -> gboolean;
        #[c2rust::src_loc = "107:1"]
        pub fn luaH_widget_destroy(_: *mut lua_State) -> gint;
        #[c2rust::src_loc = "108:1"]
        pub fn luaH_widget_focus(_: *mut lua_State) -> gint;
        #[c2rust::src_loc = "110:1"]
        pub fn luaH_widget_get_children(_: *mut lua_State, _: *mut widget_t) -> gint;
        #[c2rust::src_loc = "111:1"]
        pub fn luaH_widget_hide(_: *mut lua_State) -> gint;
        #[c2rust::src_loc = "114:1"]
        pub fn luaH_widget_show(_: *mut lua_State) -> gint;
        #[c2rust::src_loc = "115:1"]
        pub fn luaH_widget_replace(_: *mut lua_State) -> gint;
        #[c2rust::src_loc = "116:1"]
        pub fn luaH_widget_send_key(_: *mut lua_State) -> gint;
        #[c2rust::src_loc = "117:1"]
        pub fn luaH_widget_get_parent(L: *mut lua_State, w: *mut widget_t) -> gint;
        #[c2rust::src_loc = "118:1"]
        pub fn luaH_widget_get_focused(L: *mut lua_State, _: *mut widget_t) -> gint;
        #[c2rust::src_loc = "119:1"]
        pub fn luaH_widget_get_visible(L: *mut lua_State, _: *mut widget_t) -> gint;
        #[c2rust::src_loc = "120:1"]
        pub fn luaH_widget_get_width(L: *mut lua_State, _: *mut widget_t) -> gint;
        #[c2rust::src_loc = "121:1"]
        pub fn luaH_widget_get_height(L: *mut lua_State, _: *mut widget_t) -> gint;
        #[c2rust::src_loc = "122:1"]
        pub fn luaH_widget_set_visible(L: *mut lua_State, _: *mut widget_t) -> gint;
        #[c2rust::src_loc = "123:1"]
        pub fn luaH_widget_set_tooltip(L: *mut lua_State, w: *mut widget_t) -> gint;
        #[c2rust::src_loc = "124:1"]
        pub fn luaH_widget_get_tooltip(L: *mut lua_State, w: *mut widget_t) -> gint;
        #[c2rust::src_loc = "125:1"]
        pub fn luaH_widget_set_min_size(L: *mut lua_State, w: *mut widget_t) -> gint;
        #[c2rust::src_loc = "126:1"]
        pub fn luaH_widget_get_min_size(L: *mut lua_State, w: *mut widget_t) -> gint;
        #[c2rust::src_loc = "127:1"]
        pub fn luaH_widget_set_align(L: *mut lua_State, w: *mut widget_t) -> gint;
        #[c2rust::src_loc = "128:1"]
        pub fn luaH_widget_get_align(L: *mut lua_State, w: *mut widget_t) -> gint;
        #[c2rust::src_loc = "132:1"]
        pub fn parent_set_cb(_: *mut GtkWidget, _: *mut GtkWidget, _: *mut widget_t);
        #[c2rust::src_loc = "133:1"]
        pub fn resize_cb(_: *mut GtkWidget, _: *mut GdkRectangle, _: *mut widget_t);
        #[c2rust::src_loc = "135:1"]
        pub fn destroy_cb(UNUSED_win: *mut GtkWidget, w: *mut widget_t);
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gmacros.h:19"]
pub mod gmacros_h {
    #[c2rust::src_loc = "931:9"]
    pub const FALSE: std::ffi::c_int = 0 as std::ffi::c_int;
}
#[c2rust::header_src = "/usr/lib/clang/20/include/__stddef_null.h:19"]
pub mod __stddef_null_h {
    #[c2rust::src_loc = "26:9"]
    pub const NULL: std::ffi::c_int = 0 as std::ffi::c_int;
}
pub use self::__stddef_size_t_h::size_t;
pub use self::glibconfig_h::{gint8, gint16, gsize};
pub use self::gtypes_h::{gchar, gint, gboolean, guint, gpointer};
pub use self::gdataset_h::{GData, _GData};
pub use self::ghash_h::{GHashTable, _GHashTable};
pub use self::gtree_h::{GTree, _GTree};
pub use self::gtype_h::{
    GType, _GTypeClass, GTypeClass, _GTypeInstance, GTypeInstance,
    g_type_check_instance_cast, g_type_check_instance_is_a,
};
pub use self::gclosure_h::GCallback;
pub use self::gobject_h::{_GObject, GObject, GInitiallyUnowned, g_object_connect};
pub use self::lua_h::{
    lua_CFunction, LUA_REGISTRYINDEX, LUA_GLOBALSINDEX, LUA_TTABLE, lua_State,
    lua_settop, lua_remove, lua_type, lua_pushlstring, lua_pushcclosure,
    lua_pushlightuserdata, lua_getfield, lua_rawget, lua_pcall,
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
    widget_t, widget_destructor_t, widget_info_t, widget_constructor_t, luaH_checkwidget,
    widget_class,
};
pub use self::gtkcssprovider_h::{
    GtkCssProvider, _GtkCssProvider, GtkCssProviderPrivate, _GtkCssProviderPrivate,
};
pub use self::gtktypes_h::GtkWidget;
pub use self::gtkwidget_h::{
    _GtkWidget, GtkWidgetPrivate, _GtkWidgetPrivate, gtk_widget_get_type,
    gtk_widget_show, gtk_widget_queue_draw_area, gtk_widget_get_allocated_width,
    gtk_widget_get_allocated_height,
};
pub use self::cairo_h::{cairo_t, _cairo_rectangle_int, cairo_rectangle_int_t, _cairo};
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
    GDK_EXPOSE, GDK_DESTROY, GDK_DELETE, GDK_NOTHING, _GdkEventFocus, GdkEventFocus,
};
pub use self::gmessages_h::G_LOG_DOMAIN;
use self::gtestutils_h::g_assertion_message_expr;
use self::lauxlib_h::luaL_error;
pub use self::luaobject_h::{
    luaH_object_registry_push, luaH_object_ref, luaH_object_push, luaH_object_incref,
    luaH_object_emit_signal, luaH_object_property_signal,
};
use self::gtkdrawingarea_h::gtk_drawing_area_new;
use self::widgets_common_h::{
    focus_cb, luaH_widget_destroy, luaH_widget_focus, luaH_widget_get_children,
    luaH_widget_hide, luaH_widget_show, luaH_widget_replace, luaH_widget_send_key,
    luaH_widget_get_parent, luaH_widget_get_focused, luaH_widget_get_visible,
    luaH_widget_get_width, luaH_widget_get_height, luaH_widget_set_visible,
    luaH_widget_set_tooltip, luaH_widget_get_tooltip, luaH_widget_set_min_size,
    luaH_widget_get_min_size, luaH_widget_set_align, luaH_widget_get_align,
    parent_set_cb, resize_cb, destroy_cb,
};
pub use self::gmacros_h::FALSE;
pub use self::__stddef_null_h::NULL;
#[c2rust::src_loc = "22:17"]
static mut ffi_new_ref: gpointer = 0 as *const std::ffi::c_void as *mut std::ffi::c_void;
#[c2rust::src_loc = "24:1"]
unsafe extern "C" fn luaH_drawing_area_invalidate(mut L: *mut lua_State) -> gint {
    let mut w = luaH_checkwidget(L, 1 as std::ffi::c_int);
    let mut width = gtk_widget_get_allocated_width((*w).widget) as guint;
    let mut height = gtk_widget_get_allocated_height((*w).widget) as guint;
    gtk_widget_queue_draw_area(
        (*w).widget,
        0 as std::ffi::c_int,
        0 as std::ffi::c_int,
        width as gint,
        height as gint,
    );
    return 0 as std::ffi::c_int;
}
#[c2rust::src_loc = "34:1"]
unsafe extern "C" fn luaH_drawing_area_index(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
    mut token: luakit_token_t,
) -> gint {
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
        128 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_drawing_area_invalidate
                        as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        _ => {}
    }
    return 0 as std::ffi::c_int;
}
#[c2rust::src_loc = "46:1"]
unsafe extern "C" fn luaH_drawing_area_newindex(
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
        _ => {}
    }
    return luaH_object_property_signal(L, 1 as std::ffi::c_int, token);
}
#[c2rust::src_loc = "58:1"]
unsafe extern "C" fn drawing_area_draw_cb(
    mut UNUSED_widget: *mut GtkWidget,
    mut cr: *mut cairo_t,
    mut w: *mut widget_t,
) -> gboolean {
    let mut L = common.L;
    luaH_object_push(L, (*w).ref_0);
    luaH_object_push(L, ffi_new_ref);
    lua_pushlstring(
        L,
        b"cairo_t *\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 10]>() as std::ffi::c_ulong)
            .wrapping_div(
                ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
            )
            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
    );
    lua_pushlightuserdata(L, cr as *mut std::ffi::c_void);
    let mut error = lua_pcall(
        L,
        2 as std::ffi::c_int,
        1 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    if error == 0 as std::ffi::c_int {} else {
        g_assertion_message_expr(
            G_LOG_DOMAIN as *const std::ffi::c_char,
            b"widgets/drawing_area.c\0" as *const u8 as *const std::ffi::c_char,
            68 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 21],
                &[std::ffi::c_char; 21],
            >(b"drawing_area_draw_cb\0"))
                .as_ptr(),
            b"error == 0\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    luaH_object_emit_signal(
        L,
        -(2 as std::ffi::c_int),
        b"draw\0" as *const u8 as *const std::ffi::c_char,
        1 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    return FALSE;
}
#[no_mangle]
#[c2rust::src_loc = "74:1"]
pub unsafe extern "C" fn widget_drawing_area(
    mut UNUSED_L: *mut lua_State,
    mut w: *mut widget_t,
    mut UNUSED_token: luakit_token_t,
) -> *mut widget_t {
    (*w)
        .index = Some(
        luaH_drawing_area_index
            as unsafe extern "C" fn(
                *mut lua_State,
                *mut widget_t,
                luakit_token_t,
            ) -> gint,
    );
    (*w)
        .newindex = Some(
        luaH_drawing_area_newindex
            as unsafe extern "C" fn(
                *mut lua_State,
                *mut widget_t,
                luakit_token_t,
            ) -> gint,
    );
    if ffi_new_ref.is_null() {
        let mut L = common.L;
        lua_getfield(
            L,
            LUA_GLOBALSINDEX,
            b"require\0" as *const u8 as *const std::ffi::c_char,
        );
        lua_pushlstring(
            L,
            b"ffi\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 4]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        let mut error = lua_pcall(
            L,
            1 as std::ffi::c_int,
            1 as std::ffi::c_int,
            0 as std::ffi::c_int,
        );
        if error == 0 as std::ffi::c_int {} else {
            g_assertion_message_expr(
                G_LOG_DOMAIN as *const std::ffi::c_char,
                b"widgets/drawing_area.c\0" as *const u8 as *const std::ffi::c_char,
                88 as std::ffi::c_int,
                (*::core::mem::transmute::<
                    &[u8; 20],
                    &[std::ffi::c_char; 20],
                >(b"widget_drawing_area\0"))
                    .as_ptr(),
                b"error == 0\0" as *const u8 as *const std::ffi::c_char,
            );
        }
        if !(lua_type(L, -(1 as std::ffi::c_int)) == LUA_TTABLE) {
            luaL_error(
                L,
                b"Cannot create/use drawing area without ffi\0" as *const u8
                    as *const std::ffi::c_char,
            );
        }
        lua_getfield(
            L,
            -(1 as std::ffi::c_int),
            b"new\0" as *const u8 as *const std::ffi::c_char,
        );
        ffi_new_ref = luaH_object_ref(L, -(1 as std::ffi::c_int));
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    }
    (*w).widget = gtk_drawing_area_new();
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
        b"draw\0" as *const u8 as *const std::ffi::c_char,
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
                drawing_area_draw_cb
                    as unsafe extern "C" fn(
                        *mut GtkWidget,
                        *mut cairo_t,
                        *mut widget_t,
                    ) -> gboolean,
            ),
        ),
        w,
        NULL as *mut std::ffi::c_void,
    );
    gtk_widget_show((*w).widget);
    return w;
}
