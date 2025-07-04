use ::libc;
#[c2rust::header_src = "/usr/lib/clang/20/include/__stddef_ptrdiff_t.h:22"]
pub mod __stddef_ptrdiff_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type ptrdiff_t = std::ffi::c_long;
}
#[c2rust::header_src = "/usr/lib/clang/20/include/__stddef_size_t.h:22"]
pub mod __stddef_size_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type size_t = std::ffi::c_ulong;
}
#[c2rust::header_src = "/usr/lib/glib-2.0/include/glibconfig.h:22"]
pub mod glibconfig_h {
    #[c2rust::src_loc = "83:1"]
    pub type gsize = std::ffi::c_ulong;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtypes.h:22"]
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
    #[c2rust::src_loc = "110:1"]
    pub type gconstpointer = *const std::ffi::c_void;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/garray.h:22"]
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
    use super::gtypes_h::{gpointer, guint, gboolean};
    extern "C" {
        #[c2rust::src_loc = "150:1"]
        pub fn g_ptr_array_new() -> *mut GPtrArray;
        #[c2rust::src_loc = "188:1"]
        pub fn g_ptr_array_free(
            array: *mut GPtrArray,
            free_segment: gboolean,
        ) -> *mut gpointer;
        #[c2rust::src_loc = "213:1"]
        pub fn g_ptr_array_remove(array: *mut GPtrArray, data: gpointer) -> gboolean;
        #[c2rust::src_loc = "223:1"]
        pub fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/ghash.h:22"]
pub mod ghash_h {
    #[c2rust::src_loc = "40:1"]
    pub type GHashTable = _GHashTable;
    extern "C" {
        #[c2rust::src_loc = "40:16"]
        pub type _GHashTable;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtree.h:22"]
pub mod gtree_h {
    #[c2rust::src_loc = "40:1"]
    pub type GTree = _GTree;
    #[c2rust::src_loc = "51:1"]
    pub type GTraverseFunc = Option::<
        unsafe extern "C" fn(gpointer, gpointer, gpointer) -> gboolean,
    >;
    use super::gtypes_h::{gboolean, gpointer, gconstpointer};
    extern "C" {
        #[c2rust::src_loc = "40:16"]
        pub type _GTree;
        #[c2rust::src_loc = "95:1"]
        pub fn g_tree_destroy(tree: *mut GTree);
        #[c2rust::src_loc = "101:1"]
        pub fn g_tree_insert(tree: *mut GTree, key: gpointer, value: gpointer);
        #[c2rust::src_loc = "113:1"]
        pub fn g_tree_remove(tree: *mut GTree, key: gconstpointer) -> gboolean;
        #[c2rust::src_loc = "130:1"]
        pub fn g_tree_lookup(tree: *mut GTree, key: gconstpointer) -> gpointer;
        #[c2rust::src_loc = "138:1"]
        pub fn g_tree_foreach(
            tree: *mut GTree,
            func: GTraverseFunc,
            user_data: gpointer,
        );
    }
}
#[c2rust::header_src = "/usr/include/luajit-2.1/lua.h:22"]
pub mod lua_h {
    #[c2rust::src_loc = "53:1"]
    pub type lua_CFunction = Option::<
        unsafe extern "C" fn(*mut lua_State) -> std::ffi::c_int,
    >;
    #[c2rust::src_loc = "100:1"]
    pub type lua_Number = std::ffi::c_double;
    #[c2rust::src_loc = "104:1"]
    pub type lua_Integer = ptrdiff_t;
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
        #[c2rust::src_loc = "140:1"]
        pub fn lua_type(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;
        #[c2rust::src_loc = "147:1"]
        pub fn lua_tonumber(L: *mut lua_State, idx: std::ffi::c_int) -> lua_Number;
        #[c2rust::src_loc = "150:1"]
        pub fn lua_tolstring(
            L: *mut lua_State,
            idx: std::ffi::c_int,
            len: *mut size_t,
        ) -> *const std::ffi::c_char;
        #[c2rust::src_loc = "153:1"]
        pub fn lua_touserdata(
            L: *mut lua_State,
            idx: std::ffi::c_int,
        ) -> *mut std::ffi::c_void;
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
        #[c2rust::src_loc = "168:1"]
        pub fn lua_pushfstring(
            L: *mut lua_State,
            fmt: *const std::ffi::c_char,
            _: ...
        ) -> *const std::ffi::c_char;
        #[c2rust::src_loc = "169:1"]
        pub fn lua_pushcclosure(
            L: *mut lua_State,
            fn_0: lua_CFunction,
            n: std::ffi::c_int,
        );
        #[c2rust::src_loc = "171:1"]
        pub fn lua_pushlightuserdata(L: *mut lua_State, p: *mut std::ffi::c_void);
        #[c2rust::src_loc = "180:1"]
        pub fn lua_rawget(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "182:1"]
        pub fn lua_createtable(
            L: *mut lua_State,
            narr: std::ffi::c_int,
            nrec: std::ffi::c_int,
        );
        #[c2rust::src_loc = "184:1"]
        pub fn lua_getmetatable(
            L: *mut lua_State,
            objindex: std::ffi::c_int,
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "185:1"]
        pub fn lua_getfenv(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "193:1"]
        pub fn lua_rawset(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "195:1"]
        pub fn lua_setmetatable(
            L: *mut lua_State,
            objindex: std::ffi::c_int,
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "203:1"]
        pub fn lua_pcall(
            L: *mut lua_State,
            nargs: std::ffi::c_int,
            nresults: std::ffi::c_int,
            errfunc: std::ffi::c_int,
        ) -> std::ffi::c_int;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/log.h:22"]
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
#[c2rust::header_src = "/home/daana/git/luakit/common/signal.h:22"]
pub mod signal_h {
    #[c2rust::src_loc = "29:1"]
    pub type signal_t = GTree;
    #[c2rust::src_loc = "30:1"]
    pub type signal_array_t = GPtrArray;
    #[inline]
    #[c2rust::src_loc = "55:1"]
    pub unsafe extern "C" fn signal_destroy(mut signals: *mut signal_t) {
        g_tree_destroy(signals as *mut GTree);
    }
    #[inline]
    #[c2rust::src_loc = "61:1"]
    pub unsafe extern "C" fn signal_lookup(
        mut signals: *mut signal_t,
        mut name: *const gchar,
    ) -> *mut signal_array_t {
        return g_tree_lookup(signals as *mut GTree, name as gpointer as gconstpointer)
            as *mut signal_array_t;
    }
    #[inline]
    #[c2rust::src_loc = "68:1"]
    pub unsafe extern "C" fn signal_add(
        mut signals: *mut signal_t,
        mut name: *const gchar,
        mut func: gpointer,
    ) {
        let mut sigfuncs: *mut signal_array_t = signal_lookup(signals, name);
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
    #[c2rust::src_loc = "80:1"]
    pub unsafe extern "C" fn signal_remove(
        mut signals: *mut signal_t,
        mut name: *const gchar,
        mut func: gpointer,
    ) {
        let mut sigfuncs: *mut signal_array_t = signal_lookup(signals, name);
        if !sigfuncs.is_null() {
            g_ptr_array_remove(sigfuncs as *mut GPtrArray, func);
            if (*sigfuncs).len == 0 {
                g_tree_remove(signals as *mut GTree, name as gpointer as gconstpointer);
            }
        }
    }
    #[inline]
    #[c2rust::src_loc = "93:1"]
    pub unsafe extern "C" fn signals_remove(
        mut signals: *mut signal_t,
        mut name: *const gchar,
    ) {
        let mut sigfuncs: *mut signal_array_t = signal_lookup(signals, name);
        if !sigfuncs.is_null() {
            g_tree_remove(signals as *mut GTree, name as gpointer as gconstpointer);
        }
    }
    use super::gtree_h::{
        GTree, g_tree_destroy, g_tree_lookup, g_tree_insert, g_tree_remove,
    };
    use super::garray_h::{
        GPtrArray, g_ptr_array_new, g_ptr_array_add, g_ptr_array_remove,
    };
    use super::gtypes_h::{gchar, gpointer, gconstpointer};
    use super::gstrfuncs_h::g_strdup_inline;
}
#[c2rust::header_src = "/home/daana/git/luakit/common/tokenize.h:22"]
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
    use super::gtypes_h::gchar;
    extern "C" {
        #[c2rust::src_loc = "286:1"]
        pub fn token_tostring(_: luakit_token_t) -> *const gchar;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/luaclass.h:22"]
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
        #[c2rust::src_loc = "63:1"]
        pub fn luaH_class_get(_: *mut lua_State, _: gint) -> *mut lua_class_t;
        #[c2rust::src_loc = "88:1"]
        pub fn luaH_checkudata(
            _: *mut lua_State,
            _: gint,
            _: *mut lua_class_t,
        ) -> gpointer;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/common.h:22"]
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
#[c2rust::header_src = "/usr/include/string.h:22"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "43:14"]
        pub fn memcpy(
            _: *mut std::ffi::c_void,
            _: *const std::ffi::c_void,
            _: std::ffi::c_ulong,
        ) -> *mut std::ffi::c_void;
        #[c2rust::src_loc = "407:15"]
        pub fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gmem.h:22"]
pub mod gmem_h {
    use super::gtypes_h::gpointer;
    use super::glibconfig_h::gsize;
    extern "C" {
        #[c2rust::src_loc = "73:1"]
        pub fn g_free(mem: gpointer);
        #[c2rust::src_loc = "83:1"]
        pub fn g_malloc(n_bytes: gsize) -> gpointer;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gstrfuncs.h:22"]
pub mod gstrfuncs_h {
    #[inline(always)]
    #[c2rust::src_loc = "308:1"]
    pub unsafe extern "C" fn g_strdup_inline(
        mut str: *const std::ffi::c_char,
    ) -> *mut std::ffi::c_char {
        if 0 != 0 && str.is_null() {
            return 0 as *mut std::ffi::c_char;
        }
        if 0 != 0 && !str.is_null() && 0 != 0 {
            let len: size_t = (strlen(str))
                .wrapping_add(1 as std::ffi::c_int as std::ffi::c_ulong);
            let mut dup_str: *mut std::ffi::c_char = g_malloc(len)
                as *mut std::ffi::c_char;
            return memcpy(
                dup_str as *mut std::ffi::c_void,
                str as *const std::ffi::c_void,
                len,
            ) as *mut std::ffi::c_char;
        }
        return g_strdup(str);
    }
    use super::gtypes_h::gchar;
    use super::string_h::{strlen, memcpy};
    use super::__stddef_size_t_h::size_t;
    use super::gmem_h::g_malloc;
    extern "C" {
        #[c2rust::src_loc = "283:1"]
        pub fn g_strdup(str: *const gchar) -> *mut gchar;
        #[c2rust::src_loc = "285:1"]
        pub fn g_strdup_printf(format: *const gchar, _: ...) -> *mut gchar;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/util.h:22"]
pub mod util_h {
    use super::lua_h::lua_State;
    use super::gtypes_h::gchar;
    extern "C" {
        #[c2rust::src_loc = "73:1"]
        pub fn luaH_callerinfo(_: *mut lua_State) -> *mut gchar;
    }
}
#[c2rust::header_src = "/usr/include/luajit-2.1/lauxlib.h:22"]
pub mod lauxlib_h {
    use super::lua_h::lua_State;
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "32:1"]
        pub fn luaL_typerror(
            L: *mut lua_State,
            narg: std::ffi::c_int,
            tname: *const std::ffi::c_char,
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "34:1"]
        pub fn luaL_checklstring(
            L: *mut lua_State,
            numArg: std::ffi::c_int,
            l: *mut size_t,
        ) -> *const std::ffi::c_char;
        #[c2rust::src_loc = "45:1"]
        pub fn luaL_checkstack(
            L: *mut lua_State,
            sz: std::ffi::c_int,
            msg: *const std::ffi::c_char,
        );
        #[c2rust::src_loc = "53:1"]
        pub fn luaL_error(
            L: *mut lua_State,
            fmt: *const std::ffi::c_char,
            _: ...
        ) -> std::ffi::c_int;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/luautil.h:22"]
pub mod luautil_h {
    use super::lua_h::lua_State;
    use super::gtypes_h::gint;
    extern "C" {
        #[c2rust::src_loc = "26:1"]
        pub fn luaH_dofunction_on_error(L: *mut lua_State) -> gint;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/lualib.h:22"]
pub mod lualib_h {
    #[inline]
    #[c2rust::src_loc = "90:1"]
    pub unsafe extern "C" fn luaH_absindex(mut L: *mut lua_State, mut ud: gint) -> gint {
        return if ud >= 0 as std::ffi::c_int || ud <= -(10000 as std::ffi::c_int) {
            ud
        } else {
            lua_gettop(L) + ud + 1 as std::ffi::c_int
        };
    }
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
        let mut error_func_pos: gint = lua_gettop(L) - nargs - 1 as std::ffi::c_int;
        if lua_pcall(L, nargs, nret, -nargs - 2 as std::ffi::c_int) != 0 {
            _log(
                LOG_LEVEL_error,
                b"./common/lualib.h\0" as *const u8 as *const std::ffi::c_char,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                lua_tolstring(L, -(1 as std::ffi::c_int), 0 as *mut size_t),
            );
            lua_settop(L, -(2 as std::ffi::c_int) - 1 as std::ffi::c_int);
            return 0 as std::ffi::c_int;
        }
        lua_remove(L, error_func_pos);
        return (0 as std::ffi::c_int == 0) as std::ffi::c_int;
    }
    use super::lua_h::{
        lua_State, lua_gettop, lua_insert, lua_pushcclosure, lua_pcall, lua_tolstring,
        lua_settop, lua_remove,
    };
    use super::gtypes_h::{gint, gboolean};
    use super::luautil_h::luaH_dofunction_on_error;
    use super::log_h::{_log, LOG_LEVEL_error, log_level_t};
    use super::__stddef_size_t_h::size_t;
}
#[c2rust::header_src = "/home/daana/git/luakit/common/luaobject.h:22"]
pub mod luaobject_h {
    #[inline]
    #[c2rust::src_loc = "48:1"]
    pub unsafe extern "C" fn luaH_object_ref_item(
        mut L: *mut lua_State,
        mut ud: gint,
        mut iud: gint,
    ) -> gpointer {
        lua_getfenv(L, ud);
        let mut p: gpointer = luaH_object_incref(
            L,
            -(1 as std::ffi::c_int),
            if iud < 0 as std::ffi::c_int { iud - 1 as std::ffi::c_int } else { iud },
        );
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        return p;
    }
    #[inline]
    #[c2rust::src_loc = "61:1"]
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
    #[c2rust::src_loc = "75:1"]
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
        lua_rawget(L, -(10000 as std::ffi::c_int));
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
        lua_State, lua_getfenv, lua_settop, lua_pushlightuserdata, lua_rawget,
        lua_remove, lua_pushlstring,
    };
    use super::gtypes_h::{gint, gpointer};
    use super::{luaH_object_incref, luaH_object_decref};
}
pub use self::__stddef_ptrdiff_t_h::ptrdiff_t;
pub use self::__stddef_size_t_h::size_t;
pub use self::glibconfig_h::gsize;
pub use self::gtypes_h::{gchar, gint, gboolean, guint, gpointer, gconstpointer};
pub use self::garray_h::{
    _GPtrArray, GPtrArray, g_ptr_array_new, g_ptr_array_free, g_ptr_array_remove,
    g_ptr_array_add,
};
pub use self::ghash_h::{GHashTable, _GHashTable};
pub use self::gtree_h::{
    GTree, GTraverseFunc, _GTree, g_tree_destroy, g_tree_insert, g_tree_remove,
    g_tree_lookup, g_tree_foreach,
};
pub use self::lua_h::{
    lua_CFunction, lua_Number, lua_Integer, lua_State, lua_gettop, lua_settop,
    lua_pushvalue, lua_remove, lua_insert, lua_type, lua_tonumber, lua_tolstring,
    lua_touserdata, lua_topointer, lua_pushnil, lua_pushinteger, lua_pushlstring,
    lua_pushstring, lua_pushfstring, lua_pushcclosure, lua_pushlightuserdata, lua_rawget,
    lua_createtable, lua_getmetatable, lua_getfenv, lua_rawset, lua_setmetatable,
    lua_pcall,
};
pub use self::log_h::{
    log_level_t, LOG_LEVEL_debug, LOG_LEVEL_verbose, LOG_LEVEL_info, LOG_LEVEL_warn,
    LOG_LEVEL_error, LOG_LEVEL_fatal, _log,
};
pub use self::signal_h::{
    signal_t, signal_array_t, signal_destroy, signal_lookup, signal_add, signal_remove,
    signals_remove,
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
    L_TK_UNKNOWN, token_tostring,
};
pub use self::luaclass_h::{
    lua_class_property_array_t, lua_object_t, lua_class_allocator_t,
    lua_class_propfunc_t, lua_class_t, luaH_class_get, luaH_checkudata,
};
pub use self::common_h::{_common_t, common_t, common};
use self::string_h::{memcpy, strlen};
use self::gmem_h::{g_free, g_malloc};
pub use self::gstrfuncs_h::{g_strdup_inline, g_strdup, g_strdup_printf};
use self::util_h::luaH_callerinfo;
use self::lauxlib_h::{luaL_typerror, luaL_checklstring, luaL_checkstack, luaL_error};
use self::luautil_h::luaH_dofunction_on_error;
pub use self::lualib_h::{luaH_absindex, luaH_dofunction};
pub use self::luaobject_h::{
    luaH_object_ref_item, luaH_object_unref_item, luaH_object_push_item,
    luaH_object_registry_push, luaH_object_push,
};
#[no_mangle]
#[c2rust::src_loc = "25:1"]
pub unsafe extern "C" fn luaH_object_setup(mut L: *mut lua_State) {
    lua_pushlstring(
        L,
        b"luakit.object.registry\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 23]>() as std::ffi::c_ulong)
            .wrapping_div(
                ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
            )
            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
    );
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_setmetatable(L, -(2 as std::ffi::c_int));
    lua_rawset(L, -(10000 as std::ffi::c_int));
}
#[no_mangle]
#[c2rust::src_loc = "45:1"]
pub unsafe extern "C" fn luaH_object_incref(
    mut L: *mut lua_State,
    mut tud: gint,
    mut oud: gint,
) -> gpointer {
    let mut p: gpointer = lua_topointer(L, oud) as gpointer;
    if p.is_null() {
        lua_remove(L, oud);
        return 0 as *mut std::ffi::c_void;
    }
    lua_pushlightuserdata(L, p);
    lua_pushvalue(
        L,
        if oud < 0 as std::ffi::c_int { oud - 1 as std::ffi::c_int } else { oud },
    );
    lua_rawset(
        L,
        if tud < 0 as std::ffi::c_int { tud - 2 as std::ffi::c_int } else { tud },
    );
    lua_getmetatable(L, tud);
    lua_pushlightuserdata(L, p);
    lua_rawget(L, -(2 as std::ffi::c_int));
    let mut count: gint = (lua_tonumber(L, -(1 as std::ffi::c_int))
        + 1 as std::ffi::c_int as lua_Number) as gint;
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    lua_pushlightuserdata(L, p);
    lua_pushinteger(L, count as lua_Integer);
    lua_rawset(L, -(3 as std::ffi::c_int));
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    lua_remove(L, oud);
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "94:1"]
pub unsafe extern "C" fn luaH_object_decref(
    mut L: *mut lua_State,
    mut tud: gint,
    mut p: gpointer,
) {
    if p.is_null() {
        return;
    }
    lua_getmetatable(L, tud);
    lua_pushlightuserdata(L, p);
    lua_rawget(L, -(2 as std::ffi::c_int));
    let mut count: gint = (lua_tonumber(L, -(1 as std::ffi::c_int))
        - 1 as std::ffi::c_int as lua_Number) as gint;
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    lua_pushlightuserdata(L, p);
    if count != 0 {
        lua_pushinteger(L, count as lua_Integer);
    } else {
        lua_pushnil(L);
    }
    lua_rawset(L, -(3 as std::ffi::c_int));
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    if count == 0 {
        lua_pushlightuserdata(L, p);
        lua_pushnil(L);
        lua_rawset(
            L,
            if tud < 0 as std::ffi::c_int { tud - 2 as std::ffi::c_int } else { tud },
        );
    }
}
#[no_mangle]
#[c2rust::src_loc = "134:1"]
pub unsafe extern "C" fn luaH_settype(
    mut L: *mut lua_State,
    mut lua_class: *mut lua_class_t,
) -> gint {
    lua_pushlightuserdata(L, lua_class as *mut std::ffi::c_void);
    lua_rawget(L, -(10000 as std::ffi::c_int));
    lua_setmetatable(L, -(2 as std::ffi::c_int));
    return 1 as std::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "146:1"]
pub unsafe extern "C" fn luaH_object_add_signal(
    mut L: *mut lua_State,
    mut oud: gint,
    mut name: *const gchar,
    mut ud: gint,
) {
    if !(lua_type(L, ud) == 6 as std::ffi::c_int) {
        luaL_typerror(L, ud, b"function\0" as *const u8 as *const std::ffi::c_char);
    }
    let mut obj: *mut lua_object_t = lua_touserdata(L, oud) as *mut lua_object_t;
    if obj.is_null() {
        _log(
            LOG_LEVEL_warn,
            b"common/luaobject.c\0" as *const u8 as *const std::ffi::c_char,
            b"object add signal on non object\0" as *const u8 as *const std::ffi::c_char,
        );
        return;
    }
    let mut origin: *mut gchar = luaH_callerinfo(L);
    _log(
        LOG_LEVEL_debug,
        b"common/luaobject.c\0" as *const u8 as *const std::ffi::c_char,
        b"add \x1B[34m\"%s\"\x1B[0m on %p from \x1B[32m%s\x1B[0m\0" as *const u8
            as *const std::ffi::c_char,
        name,
        obj,
        origin,
    );
    g_free(origin as gpointer);
    signal_add((*obj).signals, name, luaH_object_ref_item(L, oud, ud));
}
#[no_mangle]
#[c2rust::src_loc = "170:1"]
pub unsafe extern "C" fn luaH_object_remove_signal(
    mut L: *mut lua_State,
    mut oud: gint,
    mut name: *const gchar,
    mut ud: gint,
) {
    if !(lua_type(L, ud) == 6 as std::ffi::c_int) {
        luaL_typerror(L, ud, b"function\0" as *const u8 as *const std::ffi::c_char);
    }
    let mut obj: *mut lua_object_t = lua_touserdata(L, oud) as *mut lua_object_t;
    if obj.is_null() {
        _log(
            LOG_LEVEL_warn,
            b"common/luaobject.c\0" as *const u8 as *const std::ffi::c_char,
            b"object remove signal on non object\0" as *const u8
                as *const std::ffi::c_char,
        );
        return;
    }
    let mut ref_0: gpointer = lua_topointer(L, ud) as gpointer;
    signal_remove((*obj).signals, name, ref_0);
    luaH_object_unref_item(L, oud, ref_0);
    lua_remove(L, ud);
}
#[no_mangle]
#[c2rust::src_loc = "189:1"]
pub unsafe extern "C" fn luaH_object_remove_signals(
    mut L: *mut lua_State,
    mut oud: gint,
    mut name: *const gchar,
) {
    let mut obj: *mut lua_object_t = lua_touserdata(L, oud) as *mut lua_object_t;
    if obj.is_null() {
        _log(
            LOG_LEVEL_warn,
            b"common/luaobject.c\0" as *const u8 as *const std::ffi::c_char,
            b"object remove signals on non object\0" as *const u8
                as *const std::ffi::c_char,
        );
        return;
    }
    let mut sigfuncs: *mut signal_array_t = signal_lookup((*obj).signals, name);
    if sigfuncs.is_null() {
        return;
    }
    let mut i: guint = 0 as std::ffi::c_int as guint;
    while i < (*sigfuncs).len {
        let mut ref_0: gpointer = *((*sigfuncs).pdata).offset(i as isize);
        luaH_object_unref_item(L, oud, ref_0);
        i = i.wrapping_add(1);
        i;
    }
    signals_remove((*obj).signals, name);
}
#[no_mangle]
#[c2rust::src_loc = "208:1"]
pub unsafe extern "C" fn signal_array_emit(
    mut L: *mut lua_State,
    mut signals: *mut signal_t,
    mut array_name: *const gchar,
    mut name: *const gchar,
    mut nargs: gint,
    mut nret: gint,
) -> gint {
    let mut sigfuncs: *mut signal_array_t = signal_lookup(signals, array_name);
    let mut origin: *mut gchar = luaH_callerinfo(L);
    _log(
        LOG_LEVEL_debug,
        b"common/luaobject.c\0" as *const u8 as *const std::ffi::c_char,
        b"emit \x1B[34m\"%s\"\x1B[0m on %p from \x1B[32m%s\x1B[0m (%d args, %d nret)\0"
            as *const u8 as *const std::ffi::c_char,
        name,
        signals,
        if !origin.is_null() {
            origin as *const gchar
        } else {
            b"<GTK>\0" as *const u8 as *const std::ffi::c_char
        },
        nargs,
        nret,
    );
    g_free(origin as gpointer);
    if !sigfuncs.is_null() {
        let mut nbfunc: gint = (*sigfuncs).len as gint;
        luaL_checkstack(
            L,
            lua_gettop(L) + nbfunc + nargs + 1 as std::ffi::c_int,
            b"too many signal handlers; need a new implementation!\0" as *const u8
                as *const std::ffi::c_char,
        );
        let mut i: gint = 0 as std::ffi::c_int;
        while i < nbfunc {
            luaH_object_push(L, *((*sigfuncs).pdata).offset(i as isize));
            i += 1;
            i;
        }
        let mut i_0: gint = 0 as std::ffi::c_int;
        while i_0 < nbfunc {
            let mut stacksize: gint = lua_gettop(L);
            let mut j: gint = 0 as std::ffi::c_int;
            while j < nargs {
                lua_pushvalue(L, -nargs - nbfunc + i_0);
                j += 1;
                j;
            }
            lua_pushvalue(L, -nargs - nbfunc + i_0);
            lua_remove(L, -nargs - nbfunc - 1 as std::ffi::c_int + i_0);
            luaH_dofunction(L, nargs, -(1 as std::ffi::c_int));
            let mut ret: gint = lua_gettop(L) - stacksize + 1 as std::ffi::c_int;
            if nret != 0 && ret != 0 && !(lua_type(L, -ret) == 0 as std::ffi::c_int) {
                let mut j_0: gint = 0 as std::ffi::c_int;
                while j_0 < nargs + nbfunc - i_0 - 1 as std::ffi::c_int {
                    lua_remove(L, -ret - 1 as std::ffi::c_int);
                    j_0 += 1;
                    j_0;
                }
                if nret != -(1 as std::ffi::c_int) && ret != nret {
                    while ret < nret {
                        lua_pushnil(L);
                        ret += 1;
                        ret;
                    }
                    if ret > nret {
                        lua_settop(L, -(ret - nret) - 1 as std::ffi::c_int);
                        ret = nret;
                    }
                }
                return ret;
            } else if nret == 0 as std::ffi::c_int {
                lua_settop(L, -ret - 1 as std::ffi::c_int);
            }
            i_0 += 1;
            i_0;
        }
    }
    lua_settop(L, -nargs - 1 as std::ffi::c_int);
    return 0 as std::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "292:1"]
pub unsafe extern "C" fn signal_object_emit(
    mut L: *mut lua_State,
    mut signals: *mut signal_t,
    mut name: *const gchar,
    mut nargs: gint,
    mut nret: gint,
) -> gint {
    return signal_array_emit(L, signals, name, name, nargs, nret);
}
#[no_mangle]
#[c2rust::src_loc = "311:1"]
pub unsafe extern "C" fn luaH_object_emit_signal(
    mut L: *mut lua_State,
    mut oud: gint,
    mut name: *const gchar,
    mut nargs: gint,
    mut nret: gint,
) -> gint {
    let mut ret: gint = 0;
    let mut top: gint = 0;
    let mut bot: gint = lua_gettop(L) - nargs + 1 as std::ffi::c_int;
    let mut oud_abs: gint = luaH_absindex(L, oud);
    let mut obj: *mut lua_object_t = lua_touserdata(L, oud) as *mut lua_object_t;
    if obj.is_null() {
        return luaL_error(
            L,
            b"trying to emit \x1B[34m\"%s\"\x1B[0m on non-object\0" as *const u8
                as *const std::ffi::c_char,
            name,
        );
    }
    let mut origin: *mut gchar = luaH_callerinfo(L);
    _log(
        LOG_LEVEL_debug,
        b"common/luaobject.c\0" as *const u8 as *const std::ffi::c_char,
        b"emit \x1B[34m\"%s\"\x1B[0m on %p from \x1B[32m%s\x1B[0m (%d args, %d nret)\0"
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
            b"trying to emit \x1B[34m\"%s\"\x1B[0m on non-object\0" as *const u8
                as *const std::ffi::c_char,
            name,
        );
    }
    let mut sigfuncs: *mut signal_array_t = signal_lookup((*obj).signals, name);
    if !sigfuncs.is_null() {
        let mut nbfunc: guint = (*sigfuncs).len;
        luaL_checkstack(
            L,
            (lua_gettop(L) as guint)
                .wrapping_add(nbfunc)
                .wrapping_add(nargs as guint)
                .wrapping_add(2 as std::ffi::c_int as guint) as std::ffi::c_int,
            b"too many signal handlers; need a new implementation!\0" as *const u8
                as *const std::ffi::c_char,
        );
        let mut i: guint = 0 as std::ffi::c_int as guint;
        while i < nbfunc {
            luaH_object_push_item(L, oud_abs, *((*sigfuncs).pdata).offset(i as isize));
            i = i.wrapping_add(1);
            i;
        }
        let mut i_0: guint = 0 as std::ffi::c_int as guint;
        while i_0 < nbfunc {
            lua_pushvalue(L, oud_abs);
            let mut j: gint = 0 as std::ffi::c_int;
            while j < nargs {
                lua_pushvalue(
                    L,
                    (-nargs as guint)
                        .wrapping_sub(nbfunc)
                        .wrapping_sub(1 as std::ffi::c_int as guint)
                        .wrapping_add(i_0) as std::ffi::c_int,
                );
                j += 1;
                j;
            }
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
            luaH_dofunction(L, nargs + 1 as std::ffi::c_int, -(1 as std::ffi::c_int));
            ret = lua_gettop(L) - top;
            if nret != 0 && ret != 0 && !(lua_type(L, -ret) == 0 as std::ffi::c_int) {
                if nret != -(1 as std::ffi::c_int) && ret != nret {
                    while ret < nret {
                        lua_pushnil(L);
                        ret += 1;
                        ret;
                    }
                    if ret > nret {
                        lua_settop(L, -(ret - nret) - 1 as std::ffi::c_int);
                        ret = nret;
                    }
                }
                let mut i_1: gint = bot;
                while i_1 <= top {
                    lua_remove(L, bot);
                    i_1 += 1;
                    i_1;
                }
                return ret;
            } else if nret == 0 as std::ffi::c_int {
                lua_settop(L, -ret - 1 as std::ffi::c_int);
            }
            i_0 = i_0.wrapping_add(1);
            i_0;
        }
    }
    lua_settop(L, -nargs - 1 as std::ffi::c_int);
    return 0 as std::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "386:1"]
pub unsafe extern "C" fn luaH_object_property_signal(
    mut L: *mut lua_State,
    mut oud: gint,
    mut tok: luakit_token_t,
) -> gint {
    let mut signame: *mut gchar = g_strdup_printf(
        b"property::%s\0" as *const u8 as *const std::ffi::c_char,
        token_tostring(tok),
    );
    luaH_object_emit_signal(L, oud, signame, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    g_free(signame as gpointer);
    return 0 as std::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "395:1"]
pub unsafe extern "C" fn luaH_object_add_signal_simple(mut L: *mut lua_State) -> gint {
    luaH_object_add_signal(
        L,
        1 as std::ffi::c_int,
        luaL_checklstring(L, 2 as std::ffi::c_int, 0 as *mut size_t),
        3 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "401:1"]
pub unsafe extern "C" fn luaH_object_remove_signal_simple(
    mut L: *mut lua_State,
) -> gint {
    luaH_object_remove_signal(
        L,
        1 as std::ffi::c_int,
        luaL_checklstring(L, 2 as std::ffi::c_int, 0 as *mut size_t),
        3 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "407:1"]
pub unsafe extern "C" fn luaH_object_remove_signals_simple(
    mut L: *mut lua_State,
) -> gint {
    luaH_object_remove_signals(
        L,
        1 as std::ffi::c_int,
        luaL_checklstring(L, 2 as std::ffi::c_int, 0 as *mut size_t),
    );
    return 0 as std::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "413:1"]
pub unsafe extern "C" fn luaH_object_collect_signal_keys(
    mut key: gpointer,
    mut UNUSED_value: gpointer,
    mut keys: *mut GPtrArray,
) -> gboolean {
    g_ptr_array_add(keys, key);
    return 0 as std::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "420:1"]
pub unsafe extern "C" fn luaH_object_remove_all_signals(
    mut signals: *mut signal_t,
) -> gint {
    if !signals.is_null() {
        let mut L: *mut lua_State = common.L;
        let mut keys: *mut GPtrArray = g_ptr_array_new();
        g_tree_foreach(
            signals,
            ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(gpointer, gpointer, *mut GPtrArray) -> gboolean,
                >,
                GTraverseFunc,
            >(
                Some(
                    luaH_object_collect_signal_keys
                        as unsafe extern "C" fn(
                            gpointer,
                            gpointer,
                            *mut GPtrArray,
                        ) -> gboolean,
                ),
            ),
            keys as gpointer,
        );
        let mut i: guint = 0 as std::ffi::c_int as guint;
        while i < (*keys).len {
            let mut type_0: *mut std::ffi::c_char = *((*keys).pdata).offset(i as isize)
                as *mut std::ffi::c_char;
            lua_pushstring(L, type_0);
            luaH_object_remove_signals_simple(L);
            i = i.wrapping_add(1);
            i;
        }
        g_ptr_array_free(keys, 0 as std::ffi::c_int);
    }
    return 0 as std::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "437:1"]
pub unsafe extern "C" fn luaH_object_emit_signal_simple(mut L: *mut lua_State) -> gint {
    return luaH_object_emit_signal(
        L,
        1 as std::ffi::c_int,
        luaL_checklstring(L, 2 as std::ffi::c_int, 0 as *mut size_t),
        lua_gettop(L) - 2 as std::ffi::c_int,
        -(1 as std::ffi::c_int),
    );
}
#[no_mangle]
#[c2rust::src_loc = "442:1"]
pub unsafe extern "C" fn luaH_object_tostring(mut L: *mut lua_State) -> gint {
    let mut lua_class: *mut lua_class_t = luaH_class_get(L, 1 as std::ffi::c_int);
    lua_pushfstring(
        L,
        b"%s: %p\0" as *const u8 as *const std::ffi::c_char,
        (*lua_class).name,
        luaH_checkudata(L, 1 as std::ffi::c_int, lua_class),
    );
    return 1 as std::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "453:1"]
pub unsafe extern "C" fn luaH_object_gc(mut L: *mut lua_State) -> gint {
    let mut item: *mut lua_object_t = lua_touserdata(L, 1 as std::ffi::c_int)
        as *mut lua_object_t;
    if item.is_null() {
        _log(
            LOG_LEVEL_warn,
            b"common/luaobject.c\0" as *const u8 as *const std::ffi::c_char,
            b"garbage collect on non-object\0" as *const u8 as *const std::ffi::c_char,
        );
        return 0 as std::ffi::c_int;
    }
    if !((*item).signals).is_null() {
        luaH_object_remove_all_signals((*item).signals);
        signal_destroy((*item).signals);
    }
    return 0 as std::ffi::c_int;
}
