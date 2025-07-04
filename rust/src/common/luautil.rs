use ::libc;
#[c2rust::header_src = "/usr/lib/clang/20/include/__stddef_size_t.h:19"]
pub mod __stddef_size_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type size_t = std::ffi::c_ulong;
}
#[c2rust::header_src = "/usr/include/luajit-2.1/lua.h:19"]
pub mod lua_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "360:8"]
    pub struct lua_Debug {
        pub event: std::ffi::c_int,
        pub name: *const std::ffi::c_char,
        pub namewhat: *const std::ffi::c_char,
        pub what: *const std::ffi::c_char,
        pub source: *const std::ffi::c_char,
        pub currentline: std::ffi::c_int,
        pub nups: std::ffi::c_int,
        pub linedefined: std::ffi::c_int,
        pub lastlinedefined: std::ffi::c_int,
        pub short_src: [std::ffi::c_char; 60],
        pub i_ci: std::ffi::c_int,
    }
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "51:16"]
        pub type lua_State;
        #[c2rust::src_loc = "122:1"]
        pub fn lua_settop(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "127:1"]
        pub fn lua_checkstack(L: *mut lua_State, sz: std::ffi::c_int) -> std::ffi::c_int;
        #[c2rust::src_loc = "137:1"]
        pub fn lua_isstring(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;
        #[c2rust::src_loc = "140:1"]
        pub fn lua_type(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;
        #[c2rust::src_loc = "141:1"]
        pub fn lua_typename(
            L: *mut lua_State,
            tp: std::ffi::c_int,
        ) -> *const std::ffi::c_char;
        #[c2rust::src_loc = "150:1"]
        pub fn lua_tolstring(
            L: *mut lua_State,
            idx: std::ffi::c_int,
            len: *mut size_t,
        ) -> *const std::ffi::c_char;
        #[c2rust::src_loc = "151:1"]
        pub fn lua_objlen(L: *mut lua_State, idx: std::ffi::c_int) -> size_t;
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
        #[c2rust::src_loc = "179:1"]
        pub fn lua_getfield(
            L: *mut lua_State,
            idx: std::ffi::c_int,
            k: *const std::ffi::c_char,
        );
        #[c2rust::src_loc = "181:1"]
        pub fn lua_rawgeti(L: *mut lua_State, idx: std::ffi::c_int, n: std::ffi::c_int);
        #[c2rust::src_loc = "182:1"]
        pub fn lua_createtable(
            L: *mut lua_State,
            narr: std::ffi::c_int,
            nrec: std::ffi::c_int,
        );
        #[c2rust::src_loc = "192:1"]
        pub fn lua_setfield(
            L: *mut lua_State,
            idx: std::ffi::c_int,
            k: *const std::ffi::c_char,
        );
        #[c2rust::src_loc = "194:1"]
        pub fn lua_rawseti(L: *mut lua_State, idx: std::ffi::c_int, n: std::ffi::c_int);
        #[c2rust::src_loc = "243:1"]
        pub fn lua_concat(L: *mut lua_State, n: std::ffi::c_int);
        #[c2rust::src_loc = "335:1"]
        pub fn lua_getstack(
            L: *mut lua_State,
            level: std::ffi::c_int,
            ar: *mut lua_Debug,
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "336:1"]
        pub fn lua_getinfo(
            L: *mut lua_State,
            what: *const std::ffi::c_char,
            ar: *mut lua_Debug,
        ) -> std::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/lib/glib-2.0/include/glibconfig.h:23"]
pub mod glibconfig_h {
    #[c2rust::src_loc = "57:1"]
    pub type guint32 = std::ffi::c_uint;
    #[c2rust::src_loc = "82:1"]
    pub type gssize = std::ffi::c_long;
    #[c2rust::src_loc = "83:1"]
    pub type gsize = std::ffi::c_ulong;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtypes.h:23"]
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
    #[c2rust::src_loc = "140:1"]
    pub type GDestroyNotify = Option::<unsafe extern "C" fn(gpointer) -> ()>;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/garray.h:23"]
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
    use super::gtypes_h::{gpointer, guint, GDestroyNotify, gboolean};
    extern "C" {
        #[c2rust::src_loc = "150:1"]
        pub fn g_ptr_array_new() -> *mut GPtrArray;
        #[c2rust::src_loc = "152:1"]
        pub fn g_ptr_array_new_with_free_func(
            element_free_func: GDestroyNotify,
        ) -> *mut GPtrArray;
        #[c2rust::src_loc = "188:1"]
        pub fn g_ptr_array_free(
            array: *mut GPtrArray,
            free_segment: gboolean,
        ) -> *mut gpointer;
        #[c2rust::src_loc = "223:1"]
        pub fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gquark.h:23"]
pub mod gquark_h {
    #[c2rust::src_loc = "38:1"]
    pub type GQuark = guint32;
    use super::glibconfig_h::guint32;
    use super::gtypes_h::gchar;
    extern "C" {
        #[c2rust::src_loc = "48:1"]
        pub fn g_quark_to_string(quark: GQuark) -> *const gchar;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gerror.h:23"]
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
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gstring.h:23"]
pub mod gstring_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "45:8"]
    pub struct _GString {
        pub str_0: *mut gchar,
        pub len: gsize,
        pub allocated_len: gsize,
    }
    #[c2rust::src_loc = "43:1"]
    pub type GString = _GString;
    #[inline(always)]
    #[c2rust::src_loc = "216:1"]
    pub unsafe extern "C" fn g_string_append_len_inline(
        mut gstring: *mut GString,
        mut val: *const std::ffi::c_char,
        mut len: gssize,
    ) -> *mut GString {
        let mut len_unsigned: gsize = 0;
        if gstring.is_null() {
            return g_string_append_len(gstring, val, len);
        }
        if val.is_null() {
            return if len != 0 as std::ffi::c_int as gssize {
                g_string_append_len(gstring, val, len)
            } else {
                gstring
            };
        }
        if len < 0 as std::ffi::c_int as gssize {
            len_unsigned = strlen(val);
        } else {
            len_unsigned = len as gsize;
        }
        if ((*gstring).len).wrapping_add(len_unsigned) < (*gstring).allocated_len {
            let mut end: *mut std::ffi::c_char = ((*gstring).str_0)
                .offset((*gstring).len as isize);
            if val.offset(len_unsigned as isize) <= end as *const std::ffi::c_char
                || val > end.offset(len_unsigned as isize) as *const std::ffi::c_char
            {
                memcpy(
                    end as *mut std::ffi::c_void,
                    val as *const std::ffi::c_void,
                    len_unsigned,
                );
            } else {
                memmove(
                    end as *mut std::ffi::c_void,
                    val as *const std::ffi::c_void,
                    len_unsigned,
                );
            }
            (*gstring).len = ((*gstring).len).wrapping_add(len_unsigned);
            *((*gstring).str_0)
                .offset((*gstring).len as isize) = 0 as std::ffi::c_int as gchar;
            return gstring;
        } else {
            return g_string_insert_len(
                gstring,
                -(1 as std::ffi::c_int) as gssize,
                val,
                len,
            )
        };
    }
    use super::gtypes_h::{gchar, gboolean};
    use super::glibconfig_h::{gsize, gssize};
    use super::string_h::{strlen, memcpy, memmove};
    extern "C" {
        #[c2rust::src_loc = "52:1"]
        pub fn g_string_new(init: *const gchar) -> *mut GString;
        #[c2rust::src_loc = "61:1"]
        pub fn g_string_free(string: *mut GString, free_segment: gboolean) -> *mut gchar;
        #[c2rust::src_loc = "64:1"]
        pub fn g_string_free_and_steal(string: *mut GString) -> *mut gchar;
        #[c2rust::src_loc = "99:1"]
        pub fn g_string_insert_len(
            string: *mut GString,
            pos: gssize,
            val: *const gchar,
            len: gssize,
        ) -> *mut GString;
        #[c2rust::src_loc = "107:1"]
        pub fn g_string_append_len(
            string: *mut GString,
            val: *const gchar,
            len: gssize,
        ) -> *mut GString;
        #[c2rust::src_loc = "178:1"]
        pub fn g_string_append_printf(
            string: *mut GString,
            format: *const gchar,
            _: ...
        );
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/log.h:24"]
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
#[c2rust::header_src = "/usr/include/stdio.h:19"]
pub mod stdio_h {
    extern "C" {
        #[c2rust::src_loc = "385:12"]
        pub fn snprintf(
            _: *mut std::ffi::c_char,
            _: std::ffi::c_ulong,
            _: *const std::ffi::c_char,
            _: ...
        ) -> std::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/luajit-2.1/lauxlib.h:19"]
pub mod lauxlib_h {
    use super::lua_h::lua_State;
    extern "C" {
        #[c2rust::src_loc = "32:1"]
        pub fn luaL_typerror(
            L: *mut lua_State,
            narg: std::ffi::c_int,
            tname: *const std::ffi::c_char,
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "53:1"]
        pub fn luaL_error(
            L: *mut lua_State,
            fmt: *const std::ffi::c_char,
            _: ...
        ) -> std::ffi::c_int;
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
        #[c2rust::src_loc = "47:14"]
        pub fn memmove(
            _: *mut std::ffi::c_void,
            _: *const std::ffi::c_void,
            _: std::ffi::c_ulong,
        ) -> *mut std::ffi::c_void;
        #[c2rust::src_loc = "156:12"]
        pub fn strcmp(
            _: *const std::ffi::c_char,
            _: *const std::ffi::c_char,
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "159:12"]
        pub fn strncmp(
            _: *const std::ffi::c_char,
            _: *const std::ffi::c_char,
            _: std::ffi::c_ulong,
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "246:14"]
        pub fn strchr(
            _: *const std::ffi::c_char,
            _: std::ffi::c_int,
        ) -> *mut std::ffi::c_char;
        #[c2rust::src_loc = "407:15"]
        pub fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gutils.h:23"]
pub mod gutils_h {
    use super::gtypes_h::gchar;
    extern "C" {
        #[c2rust::src_loc = "213:1"]
        pub fn g_get_system_config_dirs() -> *const *const gchar;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gfileutils.h:23"]
pub mod gfileutils_h {
    use super::gtypes_h::gchar;
    extern "C" {
        #[c2rust::src_loc = "174:1"]
        pub fn g_build_filename(first_element: *const gchar, _: ...) -> *mut gchar;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gmem.h:23"]
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
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gstrfuncs.h:23"]
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
    use super::glibconfig_h::gssize;
    use super::string_h::{strlen, memcpy};
    use super::__stddef_size_t_h::size_t;
    use super::gmem_h::g_malloc;
    extern "C" {
        #[c2rust::src_loc = "130:1"]
        pub fn g_strstr_len(
            haystack: *const gchar,
            haystack_len: gssize,
            needle: *const gchar,
        ) -> *mut gchar;
        #[c2rust::src_loc = "283:1"]
        pub fn g_strdup(str: *const gchar) -> *mut gchar;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtestutils.h:23"]
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
pub use self::__stddef_size_t_h::size_t;
pub use self::lua_h::{
    lua_Debug, lua_State, lua_settop, lua_checkstack, lua_isstring, lua_type,
    lua_typename, lua_tolstring, lua_objlen, lua_pushlstring, lua_pushstring,
    lua_pushfstring, lua_getfield, lua_rawgeti, lua_createtable, lua_setfield,
    lua_rawseti, lua_concat, lua_getstack, lua_getinfo,
};
pub use self::glibconfig_h::{guint32, gssize, gsize};
pub use self::gtypes_h::{gchar, gint, gboolean, guint, gpointer, GDestroyNotify};
pub use self::garray_h::{
    _GPtrArray, GPtrArray, g_ptr_array_new, g_ptr_array_new_with_free_func,
    g_ptr_array_free, g_ptr_array_add,
};
pub use self::gquark_h::{GQuark, g_quark_to_string};
pub use self::gerror_h::{_GError, GError};
pub use self::gstring_h::{
    _GString, GString, g_string_append_len_inline, g_string_new, g_string_free,
    g_string_free_and_steal, g_string_insert_len, g_string_append_len,
    g_string_append_printf,
};
pub use self::log_h::{
    log_level_t, LOG_LEVEL_debug, LOG_LEVEL_verbose, LOG_LEVEL_info, LOG_LEVEL_warn,
    LOG_LEVEL_error, LOG_LEVEL_fatal, _log,
};
use self::stdio_h::snprintf;
use self::lauxlib_h::{luaL_typerror, luaL_error};
use self::string_h::{memcpy, memmove, strcmp, strncmp, strchr, strlen};
use self::gutils_h::g_get_system_config_dirs;
use self::gfileutils_h::g_build_filename;
use self::gmem_h::{g_free, g_malloc};
pub use self::gstrfuncs_h::{g_strdup_inline, g_strstr_len, g_strdup};
use self::gtestutils_h::g_assertion_message_expr;
#[no_mangle]
#[c2rust::src_loc = "28:1"]
pub unsafe extern "C" fn luaH_traceback(
    mut L: *mut lua_State,
    mut T: *mut lua_State,
    mut min_level: gint,
) -> gint {
    let mut ar: lua_Debug = lua_Debug {
        event: 0,
        name: 0 as *const std::ffi::c_char,
        namewhat: 0 as *const std::ffi::c_char,
        what: 0 as *const std::ffi::c_char,
        source: 0 as *const std::ffi::c_char,
        currentline: 0,
        nups: 0,
        linedefined: 0,
        lastlinedefined: 0,
        short_src: [0; 60],
        i_ci: 0,
    };
    let mut max_level: gint = 0;
    let mut loc_pad: gint = 0 as std::ffi::c_int;
    if lua_getstack(T, min_level, &mut ar) == 0 {
        lua_pushlstring(
            L,
            b"\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 1]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        return 1 as std::ffi::c_int;
    }
    let mut level: gint = min_level;
    while lua_getstack(T, level, &mut ar) != 0 {
        lua_getinfo(T, b"Sl\0" as *const u8 as *const std::ffi::c_char, &mut ar);
        max_level = level;
        let mut cur_pad: gint = snprintf(
            0 as *mut std::ffi::c_char,
            0 as std::ffi::c_int as std::ffi::c_ulong,
            b"%s:%d\0" as *const u8 as *const std::ffi::c_char,
            if !(g_strstr_len(
                ar.source,
                3 as std::ffi::c_int as gssize,
                b"@./\0" as *const u8 as *const std::ffi::c_char,
            ))
                .is_null()
            {
                (ar.source).offset(3 as std::ffi::c_int as isize)
            } else if *(ar.source).offset(0 as std::ffi::c_int as isize)
                as std::ffi::c_int == '@' as i32
            {
                (ar.source).offset(1 as std::ffi::c_int as isize)
            } else {
                (ar.short_src).as_mut_ptr() as *const std::ffi::c_char
            },
            ar.currentline,
        );
        if cur_pad > loc_pad {
            loc_pad = cur_pad;
        }
        level += 1;
        level;
    }
    let mut tb: *mut GString = g_string_new(
        b"\0" as *const u8 as *const std::ffi::c_char,
    );
    let mut level_pad: gint = snprintf(
        0 as *mut std::ffi::c_char,
        0 as std::ffi::c_int as std::ffi::c_ulong,
        b"%d\0" as *const u8 as *const std::ffi::c_char,
        max_level,
    );
    let mut level_0: gint = min_level;
    while level_0 <= max_level {
        lua_getstack(T, level_0, &mut ar);
        lua_getinfo(T, b"Sln\0" as *const u8 as *const std::ffi::c_char, &mut ar);
        let mut shown_level: gint = level_0 - min_level + 1 as std::ffi::c_int;
        g_string_append_printf(
            tb,
            b"\x1B[37m(%*d)\x1B[0m \0" as *const u8 as *const std::ffi::c_char,
            level_pad,
            shown_level,
        );
        if strcmp(ar.what, b"C\0" as *const u8 as *const std::ffi::c_char)
            == 0 as std::ffi::c_int
        {
            g_string_append_printf(
                tb,
                b"%-*s\0" as *const u8 as *const std::ffi::c_char,
                loc_pad,
                b"[C]\0" as *const u8 as *const std::ffi::c_char,
            );
        } else {
            let mut src: *const std::ffi::c_char = if !(g_strstr_len(
                ar.source,
                3 as std::ffi::c_int as gssize,
                b"@./\0" as *const u8 as *const std::ffi::c_char,
            ))
                .is_null()
            {
                (ar.source).offset(3 as std::ffi::c_int as isize)
            } else if *(ar.source).offset(0 as std::ffi::c_int as isize)
                as std::ffi::c_int == '@' as i32
            {
                (ar.source).offset(1 as std::ffi::c_int as isize)
            } else {
                (ar.short_src).as_mut_ptr() as *const std::ffi::c_char
            };
            let mut n: std::ffi::c_int = 0;
            let mut cl: [std::ffi::c_char; 8] = *::core::mem::transmute::<
                &[u8; 8],
                &mut [std::ffi::c_char; 8],
            >(b"\0\0\0\0\0\0\0\0");
            snprintf(
                cl.as_mut_ptr(),
                ::core::mem::size_of::<[std::ffi::c_char; 8]>() as std::ffi::c_ulong,
                b"%d\0" as *const u8 as *const std::ffi::c_char,
                ar.currentline,
            );
            n = (strlen(src))
                .wrapping_add(strlen(cl.as_mut_ptr()))
                .wrapping_add(1 as std::ffi::c_int as std::ffi::c_ulong)
                as std::ffi::c_int;
            g_string_append_printf(
                tb,
                b"%s:%d\0" as *const u8 as *const std::ffi::c_char,
                src,
                ar.currentline,
            );
            g_string_append_printf(
                tb,
                b"%*.*s\0" as *const u8 as *const std::ffi::c_char,
                loc_pad - n,
                loc_pad - n,
                b"\0" as *const u8 as *const std::ffi::c_char,
            );
        }
        if strcmp(ar.what, b"main\0" as *const u8 as *const std::ffi::c_char)
            == 0 as std::ffi::c_int
        {
            if 0 != 0 {
                ({
                    let __val: *const std::ffi::c_char = b"\x1B[37m in main chunk\x1B[0m\0"
                        as *const u8 as *const std::ffi::c_char;
                    g_string_append_len_inline(
                        tb,
                        __val,
                        if !__val.is_null() {
                            strlen(
                                __val.offset(__val.is_null() as std::ffi::c_int as isize),
                            ) as gssize
                        } else {
                            -(1 as std::ffi::c_int) as gssize
                        },
                    );
                    compile_error!("Function call expression is not supposed to be used")
                });
                ({
                    let __val: *const std::ffi::c_char = b"\x1B[37m in main chunk\x1B[0m\0"
                        as *const u8 as *const std::ffi::c_char;
                    g_string_append_len_inline(
                        tb,
                        __val,
                        if !__val.is_null() {
                            strlen(
                                __val.offset(__val.is_null() as std::ffi::c_int as isize),
                            ) as gssize
                        } else {
                            -(1 as std::ffi::c_int) as gssize
                        },
                    );
                    compile_error!("Function call expression is not supposed to be used")
                });
            } else {
                g_string_append_len_inline(
                    tb,
                    b"\x1B[37m in main chunk\x1B[0m\0" as *const u8
                        as *const std::ffi::c_char,
                    -(1 as std::ffi::c_int) as gssize,
                );
            };
        } else {
            g_string_append_printf(
                tb,
                b"\x1B[37m in function \x1B[0m%s\0" as *const u8
                    as *const std::ffi::c_char,
                if !(ar.name).is_null() {
                    ar.name
                } else {
                    b"[anonymous]\0" as *const u8 as *const std::ffi::c_char
                },
            );
        }
        if level_0 != max_level {
            if 0 != 0 {
                ({
                    let __val: *const std::ffi::c_char = b"\n\0" as *const u8
                        as *const std::ffi::c_char;
                    g_string_append_len_inline(
                        tb,
                        __val,
                        if !__val.is_null() {
                            strlen(
                                __val.offset(__val.is_null() as std::ffi::c_int as isize),
                            ) as gssize
                        } else {
                            -(1 as std::ffi::c_int) as gssize
                        },
                    );
                    compile_error!("Function call expression is not supposed to be used")
                });
                ({
                    let __val: *const std::ffi::c_char = b"\n\0" as *const u8
                        as *const std::ffi::c_char;
                    g_string_append_len_inline(
                        tb,
                        __val,
                        if !__val.is_null() {
                            strlen(
                                __val.offset(__val.is_null() as std::ffi::c_int as isize),
                            ) as gssize
                        } else {
                            -(1 as std::ffi::c_int) as gssize
                        },
                    );
                    compile_error!("Function call expression is not supposed to be used")
                });
            } else {
                g_string_append_len_inline(
                    tb,
                    b"\n\0" as *const u8 as *const std::ffi::c_char,
                    -(1 as std::ffi::c_int) as gssize,
                );
            };
        }
        level_0 += 1;
        level_0;
    }
    lua_pushstring(L, (*tb).str_0);
    if 0 != 0 {
        if 0 as std::ffi::c_int == 0 {
            g_string_free(tb, (0 as std::ffi::c_int == 0) as std::ffi::c_int);
        } else {
            g_string_free_and_steal(tb);
        };
    } else {
        g_string_free(tb, (0 as std::ffi::c_int == 0) as std::ffi::c_int);
    };
    return 1 as std::ffi::c_int;
}
#[c2rust::src_loc = "100:1"]
unsafe extern "C" fn extract_error_message(
    mut L: *mut lua_State,
    mut message: *const gchar,
) -> *const gchar {
    let mut ar: lua_Debug = lua_Debug {
        event: 0,
        name: 0 as *const std::ffi::c_char,
        namewhat: 0 as *const std::ffi::c_char,
        what: 0 as *const std::ffi::c_char,
        source: 0 as *const std::ffi::c_char,
        currentline: 0,
        nups: 0,
        linedefined: 0,
        lastlinedefined: 0,
        short_src: [0; 60],
        i_ci: 0,
    };
    let mut level: gint = 0 as std::ffi::c_int;
    loop {
        if lua_getstack(L, level, &mut ar) == 0 {
            return message;
        }
        lua_getinfo(L, b"Sl\0" as *const u8 as *const std::ffi::c_char, &mut ar);
        if !(strcmp(ar.what, b"C\0" as *const u8 as *const std::ffi::c_char)
            == 0 as std::ffi::c_int)
        {
            break;
        }
        level += 1;
        level;
    }
    if strncmp(message, (ar.short_src).as_mut_ptr(), strlen((ar.short_src).as_mut_ptr()))
        != 0
    {
        return message;
    }
    let mut tail: *const gchar = message
        .offset(strlen((ar.short_src).as_mut_ptr()) as isize);
    if *tail as std::ffi::c_int != ':' as i32 {
        return message;
    }
    tail = tail.offset(1);
    tail;
    return (strchr(tail, ' ' as i32)).offset(1 as std::ffi::c_int as isize);
}
#[no_mangle]
#[c2rust::src_loc = "123:1"]
pub unsafe extern "C" fn luaH_dofunction_on_error(mut L: *mut lua_State) -> gint {
    if lua_checkstack(L, 5 as std::ffi::c_int) != 0 {} else {
        g_assertion_message_expr(
            0 as *mut gchar,
            b"common/luautil.c\0" as *const u8 as *const std::ffi::c_char,
            129 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 25],
                &[std::ffi::c_char; 25],
            >(b"luaH_dofunction_on_error\0"))
                .as_ptr(),
            b"lua_checkstack(L, 5)\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    lua_pushlstring(
        L,
        b"Lua error: \0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 12]>() as std::ffi::c_ulong)
            .wrapping_div(
                ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
            )
            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
    );
    lua_pushstring(
        L,
        extract_error_message(
            L,
            lua_tolstring(L, -(2 as std::ffi::c_int), 0 as *mut size_t),
        ),
    );
    lua_pushlstring(
        L,
        b"\nTraceback:\n\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 13]>() as std::ffi::c_ulong)
            .wrapping_div(
                ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
            )
            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
    );
    luaH_traceback(L, L, 1 as std::ffi::c_int);
    lua_concat(L, 4 as std::ffi::c_int);
    return 1 as std::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "140:1"]
pub unsafe extern "C" fn luaH_add_paths(
    mut L: *mut lua_State,
    mut config_dir: *const gchar,
) {
    lua_getfield(
        L,
        -(10002 as std::ffi::c_int),
        b"package\0" as *const u8 as *const std::ffi::c_char,
    );
    if 5 as std::ffi::c_int != lua_type(L, -(1 as std::ffi::c_int)) {
        _log(
            LOG_LEVEL_warn,
            b"common/luautil.c\0" as *const u8 as *const std::ffi::c_char,
            b"package is not a table\0" as *const u8 as *const std::ffi::c_char,
        );
        return;
    }
    lua_getfield(
        L,
        -(1 as std::ffi::c_int),
        b"path\0" as *const u8 as *const std::ffi::c_char,
    );
    if 4 as std::ffi::c_int != lua_type(L, -(1 as std::ffi::c_int)) {
        _log(
            LOG_LEVEL_warn,
            b"common/luautil.c\0" as *const u8 as *const std::ffi::c_char,
            b"package.path is not a string\0" as *const u8 as *const std::ffi::c_char,
        );
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        return;
    }
    let mut paths: *mut GPtrArray = g_ptr_array_new_with_free_func(
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
    g_ptr_array_add(
        paths,
        g_build_filename(
            b"/usr/local/share/luakit\0" as *const u8 as *const std::ffi::c_char,
            b"lib\0" as *const u8 as *const std::ffi::c_char,
            0 as *mut std::ffi::c_void,
        ) as gpointer,
    );
    if !config_dir.is_null() {
        g_ptr_array_add(paths, g_strdup_inline(config_dir) as gpointer);
    }
    let mut config_dirs: *const *const gchar = g_get_system_config_dirs();
    while !(*config_dirs).is_null() {
        g_ptr_array_add(
            paths,
            g_build_filename(
                *config_dirs,
                b"luakit\0" as *const u8 as *const std::ffi::c_char,
                0 as *mut std::ffi::c_void,
            ) as gpointer,
        );
        config_dirs = config_dirs.offset(1);
        config_dirs;
    }
    let mut path: *const gchar = 0 as *const gchar;
    let mut i: guint = 0 as std::ffi::c_int as guint;
    while i < (*paths).len {
        path = *((*paths).pdata).offset(i as isize) as *const gchar;
        lua_pushlstring(
            L,
            b";\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 2]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        lua_pushstring(L, path);
        lua_pushlstring(
            L,
            b"/?.lua\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 7]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        lua_concat(L, 3 as std::ffi::c_int);
        lua_pushlstring(
            L,
            b";\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 2]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        lua_pushstring(L, path);
        lua_pushlstring(
            L,
            b"/?/init.lua\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 12]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        lua_concat(L, 3 as std::ffi::c_int);
        lua_concat(L, 3 as std::ffi::c_int);
        i = i.wrapping_add(1);
        i;
    }
    g_ptr_array_free(paths, (0 as std::ffi::c_int == 0) as std::ffi::c_int);
    lua_setfield(
        L,
        -(2 as std::ffi::c_int),
        b"path\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "202:1"]
pub unsafe extern "C" fn luaH_push_gerror(
    mut L: *mut lua_State,
    mut error: *mut GError,
) -> gint {
    if !error.is_null() {} else {
        g_assertion_message_expr(
            0 as *mut gchar,
            b"common/luautil.c\0" as *const u8 as *const std::ffi::c_char,
            205 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[std::ffi::c_char; 17],
            >(b"luaH_push_gerror\0"))
                .as_ptr(),
            b"error\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    lua_createtable(L, 0 as std::ffi::c_int, 2 as std::ffi::c_int);
    lua_pushfstring(
        L,
        b"%s-%d\0" as *const u8 as *const std::ffi::c_char,
        g_quark_to_string((*error).domain),
        (*error).code,
    );
    lua_setfield(
        L,
        -(2 as std::ffi::c_int),
        b"code\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_pushstring(L, (*error).message);
    lua_setfield(
        L,
        -(2 as std::ffi::c_int),
        b"message\0" as *const u8 as *const std::ffi::c_char,
    );
    return 1 as std::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "214:1"]
pub unsafe extern "C" fn luaH_push_strv(
    mut L: *mut lua_State,
    mut strv: *const *const gchar,
) -> gint {
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    if strv.is_null() {
        return 1 as std::ffi::c_int;
    }
    let mut n: gint = 1 as std::ffi::c_int;
    while !(*strv).is_null() {
        lua_pushstring(L, *strv);
        let fresh0 = n;
        n = n + 1;
        lua_rawseti(L, -(2 as std::ffi::c_int), fresh0);
        strv = strv.offset(1);
        strv;
    }
    return 1 as std::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "229:1"]
pub unsafe extern "C" fn luaH_checkstrv(
    mut L: *mut lua_State,
    mut idx: gint,
) -> *mut *const gchar {
    if !(lua_type(L, idx) == 5 as std::ffi::c_int) {
        luaL_typerror(L, idx, b"table\0" as *const u8 as *const std::ffi::c_char);
    }
    let mut len: gint = lua_objlen(L, idx) as gint;
    let mut langs: *mut GPtrArray = g_ptr_array_new();
    let mut i: gint = 1 as std::ffi::c_int;
    while i <= len {
        lua_rawgeti(L, idx, i);
        if lua_isstring(L, -(1 as std::ffi::c_int)) == 0 {
            g_ptr_array_free(langs, (0 as std::ffi::c_int == 0) as std::ffi::c_int);
            luaL_error(
                L,
                b"bad argument %d ({string} expected, but array item %d has type %s)\0"
                    as *const u8 as *const std::ffi::c_char,
                idx,
                i,
                lua_typename(L, lua_type(L, -(1 as std::ffi::c_int))),
            );
        }
        g_ptr_array_add(
            langs,
            lua_tolstring(L, -(1 as std::ffi::c_int), 0 as *mut size_t) as *mut gchar
                as gpointer,
        );
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        i += 1;
        i;
    }
    g_ptr_array_add(langs, 0 as *mut std::ffi::c_void);
    let mut strv: *mut *const gchar = (*langs).pdata as *mut *const gchar;
    g_ptr_array_free(langs, 0 as std::ffi::c_int);
    return strv;
}
