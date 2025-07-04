use ::libc;
#[c2rust::header_src = "/usr/lib/clang/20/include/__stddef_size_t.h:21"]
pub mod __stddef_size_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type size_t = std::ffi::c_ulong;
}
#[c2rust::header_src = "/usr/lib/glib-2.0/include/glibconfig.h:21"]
pub mod glibconfig_h {
    #[c2rust::src_loc = "57:1"]
    pub type guint32 = std::ffi::c_uint;
    #[c2rust::src_loc = "83:1"]
    pub type gsize = std::ffi::c_ulong;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtypes.h:21"]
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
    #[c2rust::src_loc = "114:1"]
    pub type GCompareDataFunc = Option::<
        unsafe extern "C" fn(gconstpointer, gconstpointer, gpointer) -> gint,
    >;
    #[c2rust::src_loc = "140:1"]
    pub type GDestroyNotify = Option::<unsafe extern "C" fn(gpointer) -> ()>;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/garray.h:21"]
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
        #[c2rust::src_loc = "188:1"]
        pub fn g_ptr_array_free(
            array: *mut GPtrArray,
            free_segment: gboolean,
        ) -> *mut gpointer;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gquark.h:21"]
pub mod gquark_h {
    #[c2rust::src_loc = "38:1"]
    pub type GQuark = guint32;
    use super::glibconfig_h::guint32;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gerror.h:21"]
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
#[c2rust::header_src = "/usr/include/glib-2.0/glib/ghash.h:21"]
pub mod ghash_h {
    #[c2rust::src_loc = "40:1"]
    pub type GHashTable = _GHashTable;
    extern "C" {
        #[c2rust::src_loc = "40:16"]
        pub type _GHashTable;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gregex.h:21"]
pub mod gregex_h {
    #[c2rust::src_loc = "304:9"]
    pub type GRegexCompileFlags = std::ffi::c_uint;
    #[c2rust::src_loc = "324:3"]
    pub const G_REGEX_JAVASCRIPT_COMPAT: GRegexCompileFlags = 33554432;
    #[c2rust::src_loc = "323:3"]
    pub const G_REGEX_BSR_ANYCRLF: GRegexCompileFlags = 8388608;
    #[c2rust::src_loc = "322:3"]
    pub const G_REGEX_NEWLINE_ANYCRLF: GRegexCompileFlags = 5242880;
    #[c2rust::src_loc = "321:3"]
    pub const G_REGEX_NEWLINE_CRLF: GRegexCompileFlags = 3145728;
    #[c2rust::src_loc = "320:3"]
    pub const G_REGEX_NEWLINE_LF: GRegexCompileFlags = 2097152;
    #[c2rust::src_loc = "319:3"]
    pub const G_REGEX_NEWLINE_CR: GRegexCompileFlags = 1048576;
    #[c2rust::src_loc = "318:3"]
    pub const G_REGEX_DUPNAMES: GRegexCompileFlags = 524288;
    #[c2rust::src_loc = "317:3"]
    pub const G_REGEX_FIRSTLINE: GRegexCompileFlags = 262144;
    #[c2rust::src_loc = "316:3"]
    pub const G_REGEX_OPTIMIZE: GRegexCompileFlags = 8192;
    #[c2rust::src_loc = "315:3"]
    pub const G_REGEX_NO_AUTO_CAPTURE: GRegexCompileFlags = 4096;
    #[c2rust::src_loc = "314:3"]
    pub const G_REGEX_RAW: GRegexCompileFlags = 2048;
    #[c2rust::src_loc = "313:3"]
    pub const G_REGEX_UNGREEDY: GRegexCompileFlags = 512;
    #[c2rust::src_loc = "312:3"]
    pub const G_REGEX_DOLLAR_ENDONLY: GRegexCompileFlags = 32;
    #[c2rust::src_loc = "311:3"]
    pub const G_REGEX_ANCHORED: GRegexCompileFlags = 16;
    #[c2rust::src_loc = "310:3"]
    pub const G_REGEX_EXTENDED: GRegexCompileFlags = 8;
    #[c2rust::src_loc = "309:3"]
    pub const G_REGEX_DOTALL: GRegexCompileFlags = 4;
    #[c2rust::src_loc = "308:3"]
    pub const G_REGEX_MULTILINE: GRegexCompileFlags = 2;
    #[c2rust::src_loc = "307:3"]
    pub const G_REGEX_CASELESS: GRegexCompileFlags = 1;
    #[c2rust::src_loc = "306:3"]
    pub const G_REGEX_DEFAULT: GRegexCompileFlags = 0;
    #[c2rust::src_loc = "396:9"]
    pub type GRegexMatchFlags = std::ffi::c_uint;
    #[c2rust::src_loc = "413:3"]
    pub const G_REGEX_MATCH_NOTEMPTY_ATSTART: GRegexMatchFlags = 268435456;
    #[c2rust::src_loc = "412:3"]
    pub const G_REGEX_MATCH_PARTIAL_HARD: GRegexMatchFlags = 134217728;
    #[c2rust::src_loc = "411:3"]
    pub const G_REGEX_MATCH_PARTIAL_SOFT: GRegexMatchFlags = 32768;
    #[c2rust::src_loc = "410:3"]
    pub const G_REGEX_MATCH_BSR_ANY: GRegexMatchFlags = 16777216;
    #[c2rust::src_loc = "409:3"]
    pub const G_REGEX_MATCH_BSR_ANYCRLF: GRegexMatchFlags = 8388608;
    #[c2rust::src_loc = "408:3"]
    pub const G_REGEX_MATCH_NEWLINE_ANYCRLF: GRegexMatchFlags = 5242880;
    #[c2rust::src_loc = "407:3"]
    pub const G_REGEX_MATCH_NEWLINE_ANY: GRegexMatchFlags = 4194304;
    #[c2rust::src_loc = "406:3"]
    pub const G_REGEX_MATCH_NEWLINE_CRLF: GRegexMatchFlags = 3145728;
    #[c2rust::src_loc = "405:3"]
    pub const G_REGEX_MATCH_NEWLINE_LF: GRegexMatchFlags = 2097152;
    #[c2rust::src_loc = "404:3"]
    pub const G_REGEX_MATCH_NEWLINE_CR: GRegexMatchFlags = 1048576;
    #[c2rust::src_loc = "403:3"]
    pub const G_REGEX_MATCH_PARTIAL: GRegexMatchFlags = 32768;
    #[c2rust::src_loc = "402:3"]
    pub const G_REGEX_MATCH_NOTEMPTY: GRegexMatchFlags = 1024;
    #[c2rust::src_loc = "401:3"]
    pub const G_REGEX_MATCH_NOTEOL: GRegexMatchFlags = 256;
    #[c2rust::src_loc = "400:3"]
    pub const G_REGEX_MATCH_NOTBOL: GRegexMatchFlags = 128;
    #[c2rust::src_loc = "399:3"]
    pub const G_REGEX_MATCH_ANCHORED: GRegexMatchFlags = 16;
    #[c2rust::src_loc = "398:3"]
    pub const G_REGEX_MATCH_DEFAULT: GRegexMatchFlags = 0;
    #[c2rust::src_loc = "416:1"]
    pub type GRegex = _GRegex;
    #[c2rust::src_loc = "425:1"]
    pub type GMatchInfo = _GMatchInfo;
    use super::gtypes_h::{gchar, gboolean};
    use super::gerror_h::GError;
    extern "C" {
        #[c2rust::src_loc = "416:16"]
        pub type _GRegex;
        #[c2rust::src_loc = "425:16"]
        pub type _GMatchInfo;
        #[c2rust::src_loc = "449:1"]
        pub fn g_regex_new(
            pattern: *const gchar,
            compile_options: GRegexCompileFlags,
            match_options: GRegexMatchFlags,
            error: *mut *mut GError,
        ) -> *mut GRegex;
        #[c2rust::src_loc = "489:1"]
        pub fn g_regex_match(
            regex: *const GRegex,
            string: *const gchar,
            match_options: GRegexMatchFlags,
            match_info: *mut *mut GMatchInfo,
        ) -> gboolean;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtree.h:21"]
pub mod gtree_h {
    #[c2rust::src_loc = "40:1"]
    pub type GTree = _GTree;
    use super::gtypes_h::{GCompareDataFunc, gpointer, GDestroyNotify};
    extern "C" {
        #[c2rust::src_loc = "40:16"]
        pub type _GTree;
        #[c2rust::src_loc = "78:1"]
        pub fn g_tree_new_full(
            key_compare_func: GCompareDataFunc,
            key_compare_data: gpointer,
            key_destroy_func: GDestroyNotify,
            value_destroy_func: GDestroyNotify,
        ) -> *mut GTree;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/guri.h:21"]
pub mod guri_h {
    #[c2rust::src_loc = "33:1"]
    pub type GUri = _GUri;
    #[c2rust::src_loc = "82:9"]
    pub type GUriFlags = std::ffi::c_uint;
    #[c2rust::src_loc = "92:3"]
    pub const G_URI_FLAGS_SCHEME_NORMALIZE: GUriFlags = 256;
    #[c2rust::src_loc = "91:3"]
    pub const G_URI_FLAGS_ENCODED_FRAGMENT: GUriFlags = 128;
    #[c2rust::src_loc = "90:3"]
    pub const G_URI_FLAGS_ENCODED_PATH: GUriFlags = 64;
    #[c2rust::src_loc = "89:3"]
    pub const G_URI_FLAGS_ENCODED_QUERY: GUriFlags = 32;
    #[c2rust::src_loc = "88:3"]
    pub const G_URI_FLAGS_NON_DNS: GUriFlags = 16;
    #[c2rust::src_loc = "87:3"]
    pub const G_URI_FLAGS_ENCODED: GUriFlags = 8;
    #[c2rust::src_loc = "86:3"]
    pub const G_URI_FLAGS_HAS_AUTH_PARAMS: GUriFlags = 4;
    #[c2rust::src_loc = "85:3"]
    pub const G_URI_FLAGS_HAS_PASSWORD: GUriFlags = 2;
    #[c2rust::src_loc = "84:3"]
    pub const G_URI_FLAGS_PARSE_RELAXED: GUriFlags = 1;
    #[c2rust::src_loc = "83:3"]
    pub const G_URI_FLAGS_NONE: GUriFlags = 0;
    use super::gtypes_h::{gchar, gint};
    use super::gerror_h::GError;
    extern "C" {
        #[c2rust::src_loc = "33:16"]
        pub type _GUri;
        #[c2rust::src_loc = "37:1"]
        pub fn g_uri_unref(uri: *mut GUri);
        #[c2rust::src_loc = "141:1"]
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
        #[c2rust::src_loc = "153:1"]
        pub fn g_uri_parse(
            uri_string: *const gchar,
            flags: GUriFlags,
            error: *mut *mut GError,
        ) -> *mut GUri;
        #[c2rust::src_loc = "222:1"]
        pub fn g_uri_get_scheme(uri: *mut GUri) -> *const gchar;
        #[c2rust::src_loc = "226:1"]
        pub fn g_uri_get_user(uri: *mut GUri) -> *const gchar;
        #[c2rust::src_loc = "228:1"]
        pub fn g_uri_get_password(uri: *mut GUri) -> *const gchar;
        #[c2rust::src_loc = "232:1"]
        pub fn g_uri_get_host(uri: *mut GUri) -> *const gchar;
        #[c2rust::src_loc = "234:1"]
        pub fn g_uri_get_port(uri: *mut GUri) -> gint;
        #[c2rust::src_loc = "236:1"]
        pub fn g_uri_get_path(uri: *mut GUri) -> *const gchar;
        #[c2rust::src_loc = "238:1"]
        pub fn g_uri_get_query(uri: *mut GUri) -> *const gchar;
        #[c2rust::src_loc = "240:1"]
        pub fn g_uri_get_fragment(uri: *mut GUri) -> *const gchar;
    }
}
#[c2rust::header_src = "/usr/include/luajit-2.1/lua.h:21"]
pub mod lua_h {
    #[c2rust::src_loc = "53:1"]
    pub type lua_CFunction = Option::<
        unsafe extern "C" fn(*mut lua_State) -> std::ffi::c_int,
    >;
    #[c2rust::src_loc = "100:1"]
    pub type lua_Number = std::ffi::c_double;
    #[c2rust::src_loc = "30:9"]
    pub const LUA_MULTRET: std::ffi::c_int = -(1 as std::ffi::c_int);
    #[c2rust::src_loc = "75:9"]
    pub const LUA_TNIL: std::ffi::c_int = 0 as std::ffi::c_int;
    #[c2rust::src_loc = "80:9"]
    pub const LUA_TTABLE: std::ffi::c_int = 5 as std::ffi::c_int;
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "51:16"]
        pub type lua_State;
        #[c2rust::src_loc = "121:1"]
        pub fn lua_gettop(L: *mut lua_State) -> std::ffi::c_int;
        #[c2rust::src_loc = "122:1"]
        pub fn lua_settop(L: *mut lua_State, idx: std::ffi::c_int);
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
        #[c2rust::src_loc = "162:1"]
        pub fn lua_pushnumber(L: *mut lua_State, n: lua_Number);
        #[c2rust::src_loc = "164:1"]
        pub fn lua_pushlstring(L: *mut lua_State, s: *const std::ffi::c_char, l: size_t);
        #[c2rust::src_loc = "165:1"]
        pub fn lua_pushstring(L: *mut lua_State, s: *const std::ffi::c_char);
        #[c2rust::src_loc = "180:1"]
        pub fn lua_rawget(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "182:1"]
        pub fn lua_createtable(
            L: *mut lua_State,
            narr: std::ffi::c_int,
            nrec: std::ffi::c_int,
        );
        #[c2rust::src_loc = "193:1"]
        pub fn lua_rawset(L: *mut lua_State, idx: std::ffi::c_int);
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/signal.h:21"]
pub mod signal_h {
    #[c2rust::src_loc = "29:1"]
    pub type signal_t = GTree;
    #[inline]
    #[c2rust::src_loc = "33:1"]
    pub unsafe extern "C" fn signal_cmp(
        mut a: gconstpointer,
        mut b: gconstpointer,
        mut UNUSED_p: gpointer,
    ) -> gint {
        return g_strcmp0(a as *const std::ffi::c_char, b as *const std::ffi::c_char);
    }
    #[inline]
    #[c2rust::src_loc = "40:1"]
    pub unsafe extern "C" fn signal_array_destroy(mut sigfuncs: *mut gpointer) {
        g_ptr_array_free(sigfuncs as *mut GPtrArray, TRUE);
    }
    #[inline]
    #[c2rust::src_loc = "47:1"]
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
#[c2rust::header_src = "/usr/include/luajit-2.1/lauxlib.h:21"]
pub mod lauxlib_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "21:16"]
    pub struct luaL_Reg {
        pub name: *const std::ffi::c_char,
        pub func: lua_CFunction,
    }
    use super::lua_h::{lua_CFunction, lua_State};
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
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/luaclass.h:21"]
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
    use super::gtypes_h::{gint, gchar};
    use super::lauxlib_h::luaL_Reg;
    extern "C" {
        #[c2rust::src_loc = "65:1"]
        pub fn luaH_class_add_signal(
            _: *mut lua_State,
            _: *mut lua_class_t,
            name: *const gchar,
            ud: gint,
        );
        #[c2rust::src_loc = "67:1"]
        pub fn luaH_class_remove_signal(
            _: *mut lua_State,
            _: *mut lua_class_t,
            name: *const gchar,
            ud: gint,
        );
        #[c2rust::src_loc = "69:1"]
        pub fn luaH_class_emit_signal(
            _: *mut lua_State,
            _: *mut lua_class_t,
            name: *const gchar,
            nargs: gint,
            nret: gint,
        ) -> gint;
        #[c2rust::src_loc = "74:1"]
        pub fn luaH_openlib(
            _: *mut lua_State,
            _: *const gchar,
            _: *const luaL_Reg,
            _: *const luaL_Reg,
        );
    }
}
#[c2rust::header_src = "/usr/include/string.h:21"]
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
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gmem.h:21"]
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
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gstrfuncs.h:21"]
pub mod gstrfuncs_h {
    #[inline(always)]
    #[c2rust::src_loc = "308:1"]
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
    extern "C" {
        #[c2rust::src_loc = "283:1"]
        pub fn g_strdup(str: *const gchar) -> *mut gchar;
        #[c2rust::src_loc = "285:1"]
        pub fn g_strdup_printf(format: *const gchar, _: ...) -> *mut gchar;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtestutils.h:21"]
pub mod gtestutils_h {
    extern "C" {
        #[c2rust::src_loc = "282:1"]
        pub fn g_strcmp0(
            str1: *const std::ffi::c_char,
            str2: *const std::ffi::c_char,
        ) -> std::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/libsoup-3.0/libsoup/soup-uri-utils.h:25"]
pub mod soup_uri_utils_h {
    #[c2rust::src_loc = "40:9"]
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
#[c2rust::header_src = "/home/daana/git/luakit/common/clib/soup.h:33"]
pub mod soup_h {
    #[c2rust::src_loc = "37:16"]
    pub static mut scheme_reg: *mut GRegex = 0 as *const GRegex as *mut GRegex;
    #[c2rust::src_loc = "39:1"]
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
            (::core::mem::size_of::<[std::ffi::c_char; 7]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        lua_rawget(L, 1 as std::ffi::c_int);
        if !(lua_type(L, -(1 as std::ffi::c_int)) == LUA_TNIL)
            && {
                p = lua_tolstring(L, -(1 as std::ffi::c_int), NULL_1 as *mut size_t);
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
            (::core::mem::size_of::<[std::ffi::c_char; 5]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        lua_rawget(L, 1 as std::ffi::c_int);
        if !(lua_type(L, -(1 as std::ffi::c_int)) == LUA_TNIL)
            && {
                p = lua_tolstring(L, -(1 as std::ffi::c_int), NULL_1 as *mut size_t);
                !p.is_null()
            } && *p.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int != 0
        {
            user = p;
        }
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        lua_pushlstring(
            L,
            b"host\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 5]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        lua_rawget(L, 1 as std::ffi::c_int);
        if !(lua_type(L, -(1 as std::ffi::c_int)) == LUA_TNIL)
            && {
                p = lua_tolstring(L, -(1 as std::ffi::c_int), NULL_1 as *mut size_t);
                !p.is_null()
            } && *p.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int != 0
        {
            host = p;
        }
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        lua_pushlstring(
            L,
            b"path\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 5]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        lua_rawget(L, 1 as std::ffi::c_int);
        if !(lua_type(L, -(1 as std::ffi::c_int)) == LUA_TNIL)
            && {
                p = lua_tolstring(L, -(1 as std::ffi::c_int), NULL_1 as *mut size_t);
                !p.is_null()
            } && *p.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int != 0
        {
            path = p;
        }
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        lua_pushlstring(
            L,
            b"query\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 6]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        lua_rawget(L, 1 as std::ffi::c_int);
        if !(lua_type(L, -(1 as std::ffi::c_int)) == LUA_TNIL)
            && {
                p = lua_tolstring(L, -(1 as std::ffi::c_int), NULL_1 as *mut size_t);
                !p.is_null()
            } && *p.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int != 0
        {
            query = p;
        }
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        lua_pushlstring(
            L,
            b"fragment\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 9]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        lua_rawget(L, 1 as std::ffi::c_int);
        if !(lua_type(L, -(1 as std::ffi::c_int)) == LUA_TNIL)
            && {
                p = lua_tolstring(L, -(1 as std::ffi::c_int), NULL_1 as *mut size_t);
                !p.is_null()
            } && *p.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int != 0
        {
            fragment = p;
        }
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        lua_pushlstring(
            L,
            b"port\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 5]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
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
    #[c2rust::src_loc = "102:1"]
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
                (::core::mem::size_of::<[std::ffi::c_char; 7]>() as std::ffi::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                    )
                    .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
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
                (::core::mem::size_of::<[std::ffi::c_char; 5]>() as std::ffi::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                    )
                    .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
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
                (::core::mem::size_of::<[std::ffi::c_char; 9]>() as std::ffi::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                    )
                    .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
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
                (::core::mem::size_of::<[std::ffi::c_char; 5]>() as std::ffi::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                    )
                    .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
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
                (::core::mem::size_of::<[std::ffi::c_char; 5]>() as std::ffi::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                    )
                    .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
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
                (::core::mem::size_of::<[std::ffi::c_char; 6]>() as std::ffi::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                    )
                    .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
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
                (::core::mem::size_of::<[std::ffi::c_char; 9]>() as std::ffi::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                    )
                    .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
            );
            lua_pushstring(L, p);
            lua_rawset(L, -(3 as std::ffi::c_int));
        }
        port = g_uri_get_port(uri);
        if port > 0 as std::ffi::c_int {
            lua_pushlstring(
                L,
                b"port\0" as *const u8 as *const std::ffi::c_char,
                (::core::mem::size_of::<[std::ffi::c_char; 5]>() as std::ffi::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                    )
                    .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
            );
            lua_pushnumber(L, port as lua_Number);
            lua_rawset(L, -(3 as std::ffi::c_int));
        }
        return 1 as std::ffi::c_int;
    }
    #[c2rust::src_loc = "135:1"]
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
    #[c2rust::src_loc = "161:1"]
    pub unsafe extern "C" fn soup_lib_setup_common() {
        scheme_reg = g_regex_new(
            b"^[a-z][a-z0-9\\+\\-\\.]*:\0" as *const u8 as *const std::ffi::c_char,
            G_REGEX_DEFAULT,
            G_REGEX_MATCH_DEFAULT,
            NULL_1 as *mut *mut GError,
        );
    }
    use super::gregex_h::{
        GRegex, g_regex_match, GRegexMatchFlags, G_REGEX_MATCH_DEFAULT, GMatchInfo,
        g_regex_new, GRegexCompileFlags, G_REGEX_DEFAULT,
    };
    use super::lua_h::{
        lua_State, lua_type, LUA_TTABLE, lua_pushlstring, lua_rawget, LUA_TNIL,
        lua_tolstring, lua_settop, lua_tonumber, lua_Number, lua_pushstring,
        lua_createtable, lua_rawset, lua_pushnumber,
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
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gmacros.h:21"]
pub mod gmacros_h {
    #[c2rust::src_loc = "931:9"]
    pub const FALSE: std::ffi::c_int = 0 as std::ffi::c_int;
    #[c2rust::src_loc = "935:9"]
    pub const TRUE: std::ffi::c_int = (FALSE == 0) as std::ffi::c_int;
}
#[c2rust::header_src = "/usr/lib/clang/20/include/__stddef_null.h:21"]
pub mod __stddef_null_h {
    #[c2rust::src_loc = "26:9"]
    pub const NULL: std::ffi::c_int = 0 as std::ffi::c_int;
    #[c2rust::src_loc = "26:9"]
    pub const NULL_0: std::ffi::c_int = 0 as std::ffi::c_int;
    #[c2rust::src_loc = "26:9"]
    pub const NULL_1: std::ffi::c_int = 0 as std::ffi::c_int;
}
pub use self::__stddef_size_t_h::size_t;
pub use self::glibconfig_h::{guint32, gsize};
pub use self::gtypes_h::{
    gchar, gint, gboolean, guint, gpointer, gconstpointer, GCompareDataFunc,
    GDestroyNotify,
};
pub use self::garray_h::{_GPtrArray, GPtrArray, g_ptr_array_free};
pub use self::gquark_h::GQuark;
pub use self::gerror_h::{_GError, GError};
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
pub use self::lua_h::{
    lua_CFunction, lua_Number, LUA_MULTRET, LUA_TNIL, LUA_TTABLE, lua_State, lua_gettop,
    lua_settop, lua_type, lua_tonumber, lua_tolstring, lua_pushnumber, lua_pushlstring,
    lua_pushstring, lua_rawget, lua_createtable, lua_rawset,
};
pub use self::signal_h::{signal_t, signal_cmp, signal_array_destroy, signal_new};
pub use self::lauxlib_h::{luaL_Reg, luaL_typerror, luaL_checklstring};
pub use self::luaclass_h::{
    lua_class_property_array_t, lua_object_t, lua_class_allocator_t,
    lua_class_propfunc_t, lua_class_t, luaH_class_add_signal, luaH_class_remove_signal,
    luaH_class_emit_signal, luaH_openlib,
};
use self::string_h::{memcpy, strlen};
use self::gmem_h::{g_free, g_malloc};
pub use self::gstrfuncs_h::{g_strdup_inline, g_strdup, g_strdup_printf};
use self::gtestutils_h::g_strcmp0;
pub use self::soup_uri_utils_h::SOUP_HTTP_URI_FLAGS;
pub use self::soup_h::{
    scheme_reg, luaH_soup_uri_tostring, luaH_soup_push_uri, luaH_soup_parse_uri,
    soup_lib_setup_common,
};
pub use self::gmacros_h::{FALSE, TRUE};
pub use self::__stddef_null_h::{NULL, NULL_0, NULL_1};
#[c2rust::src_loc = "30:20"]
static mut soup_class: lua_class_t = lua_class_t {
    name: 0 as *const gchar,
    signals: 0 as *const signal_t as *mut signal_t,
    allocator: None,
    properties: 0 as *const lua_class_property_array_t
        as *mut lua_class_property_array_t,
    index_miss_property: None,
    newindex_miss_property: None,
};
#[inline]
#[c2rust::src_loc = "31:1"]
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
#[c2rust::src_loc = "31:1"]
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
#[c2rust::src_loc = "31:1"]
unsafe extern "C" fn luaH_soup_class_add_signal(mut L: *mut lua_State) -> gint {
    luaH_class_add_signal(
        L,
        &mut soup_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, NULL_1 as *mut size_t),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn soup_lib_setup(mut L: *mut lua_State) {
    soup_lib_setup_common();
    static mut soup_lib: [luaL_Reg; 6] = unsafe {
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
                    name: b"parse_uri\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_soup_parse_uri
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"uri_tostring\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_soup_uri_tostring
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
                        lua_CFunction,
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
}
