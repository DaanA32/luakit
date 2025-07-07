use std::mem::MaybeUninit;

use ::libc;
use lua::ffi::{
    lua_Debug, lua_Integer, lua_State, lua_checkstack, lua_concat, lua_createtable, lua_getfield,
    lua_getinfo, lua_getstack, lua_isstring, lua_pushfstring, lua_pushlstring, lua_pushstring,
    lua_rawgeti, lua_rawlen, lua_rawseti, lua_setfield, lua_settop, lua_tolstring, lua_type,
    lua_typename,
};
pub mod __stddef_size_t_h {
    pub type size_t = std::ffi::c_ulong;
}
pub mod glibconfig_h {
    pub type guint32 = std::ffi::c_uint;
    pub type gssize = std::ffi::c_long;
    pub type gsize = std::ffi::c_ulong;
}
pub mod gtypes_h {
    pub type gchar = std::ffi::c_char;
    pub type gint = std::ffi::c_int;
    pub type gboolean = gint;
    pub type guint = std::ffi::c_uint;
    pub type gpointer = *mut std::ffi::c_void;
    pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
}
pub mod garray_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GPtrArray {
        pub pdata: *mut gpointer,
        pub len: guint,
    }
    pub type GPtrArray = _GPtrArray;
    use super::gtypes_h::{GDestroyNotify, gboolean, gpointer, guint};
    unsafe extern "C" {
        pub fn g_ptr_array_new() -> *mut GPtrArray;
        pub fn g_ptr_array_new_with_free_func(element_free_func: GDestroyNotify) -> *mut GPtrArray;
        pub fn g_ptr_array_free(array: *mut GPtrArray, free_segment: gboolean) -> *mut gpointer;
        pub fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    }
}
pub mod gquark_h {
    pub type GQuark = guint32;
    use super::glibconfig_h::guint32;
    use super::gtypes_h::gchar;
    unsafe extern "C" {
        pub fn g_quark_to_string(quark: GQuark) -> *const gchar;
    }
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
    use super::gtypes_h::{gchar, gint};
}
pub mod gstring_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GString {
        pub str_0: *mut gchar,
        pub len: gsize,
        pub allocated_len: gsize,
    }
    pub type GString = _GString;
    #[inline(always)]
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
            let mut end: *mut std::ffi::c_char = ((*gstring).str_0).offset((*gstring).len as isize);
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
            *((*gstring).str_0).offset((*gstring).len as isize) = 0 as std::ffi::c_int as gchar;
            return gstring;
        } else {
            return g_string_insert_len(gstring, -(1 as std::ffi::c_int) as gssize, val, len);
        };
    }
    use super::glibconfig_h::{gsize, gssize};
    use super::gtypes_h::{gboolean, gchar};
    use super::string_h::{memcpy, memmove, strlen};
    unsafe extern "C" {
        pub fn g_string_new(init: *const gchar) -> *mut GString;
        pub fn g_string_free(string: *mut GString, free_segment: gboolean) -> *mut gchar;
        pub fn g_string_free_and_steal(string: *mut GString) -> *mut gchar;
        pub fn g_string_insert_len(
            string: *mut GString,
            pos: gssize,
            val: *const gchar,
            len: gssize,
        ) -> *mut GString;
        pub fn g_string_append_len(
            string: *mut GString,
            val: *const gchar,
            len: gssize,
        ) -> *mut GString;
        pub fn g_string_append_printf(string: *mut GString, format: *const gchar, _: ...);
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
pub mod stdio_h {
    unsafe extern "C" {
        pub fn snprintf(
            _: *mut std::ffi::c_char,
            _: std::ffi::c_ulong,
            _: *const std::ffi::c_char,
            _: ...
        ) -> std::ffi::c_int;
    }
}
pub mod lauxlib_h {
    use lua::ffi::lua_State;

    unsafe extern "C" {
        pub fn luaL_typerror(
            L: *mut lua_State,
            narg: std::ffi::c_int,
            tname: *const std::ffi::c_char,
        ) -> std::ffi::c_int;
        pub fn luaL_error(
            L: *mut lua_State,
            fmt: *const std::ffi::c_char,
            _: ...
        ) -> std::ffi::c_int;
    }
}
pub mod string_h {
    unsafe extern "C" {
        pub fn memcpy(
            _: *mut std::ffi::c_void,
            _: *const std::ffi::c_void,
            _: std::ffi::c_ulong,
        ) -> *mut std::ffi::c_void;
        pub fn memmove(
            _: *mut std::ffi::c_void,
            _: *const std::ffi::c_void,
            _: std::ffi::c_ulong,
        ) -> *mut std::ffi::c_void;
        pub fn strcmp(_: *const std::ffi::c_char, _: *const std::ffi::c_char) -> std::ffi::c_int;
        pub fn strncmp(
            _: *const std::ffi::c_char,
            _: *const std::ffi::c_char,
            _: std::ffi::c_ulong,
        ) -> std::ffi::c_int;
        pub fn strchr(_: *const std::ffi::c_char, _: std::ffi::c_int) -> *mut std::ffi::c_char;
        pub fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    }
}
pub mod gutils_h {
    use super::gtypes_h::gchar;
    unsafe extern "C" {
        pub fn g_get_system_config_dirs() -> *const *const gchar;
    }
}
pub mod gfileutils_h {
    use super::gtypes_h::gchar;
    unsafe extern "C" {
        pub fn g_build_filename(first_element: *const gchar, _: ...) -> *mut gchar;
    }
}
pub mod gmem_h {
    use super::glibconfig_h::gsize;
    use super::gtypes_h::gpointer;
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
            return 0 as *mut std::ffi::c_char;
        }
        if 0 != 0 && !str.is_null() && 0 != 0 {
            let len: size_t = (strlen(str)).wrapping_add(1 as std::ffi::c_int as std::ffi::c_ulong);
            let mut dup_str: *mut std::ffi::c_char = g_malloc(len) as *mut std::ffi::c_char;
            return memcpy(
                dup_str as *mut std::ffi::c_void,
                str as *const std::ffi::c_void,
                len,
            ) as *mut std::ffi::c_char;
        }
        return g_strdup(str);
    }
    use super::__stddef_size_t_h::size_t;
    use super::glibconfig_h::gssize;
    use super::gmem_h::g_malloc;
    use super::gtypes_h::gchar;
    use super::string_h::{memcpy, strlen};
    unsafe extern "C" {
        pub fn g_strstr_len(
            haystack: *const gchar,
            haystack_len: gssize,
            needle: *const gchar,
        ) -> *mut gchar;
        pub fn g_strdup(str: *const gchar) -> *mut gchar;
    }
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
pub use self::__stddef_size_t_h::size_t;
pub use self::garray_h::{
    _GPtrArray, GPtrArray, g_ptr_array_add, g_ptr_array_free, g_ptr_array_new,
    g_ptr_array_new_with_free_func,
};
pub use self::gerror_h::{_GError, GError};
use self::gfileutils_h::g_build_filename;
pub use self::glibconfig_h::{gsize, gssize, guint32};
use self::gmem_h::{g_free, g_malloc};
pub use self::gquark_h::{GQuark, g_quark_to_string};
pub use self::gstrfuncs_h::{g_strdup, g_strdup_inline, g_strstr_len};
pub use self::gstring_h::{
    _GString, GString, g_string_append_len, g_string_append_len_inline, g_string_append_printf,
    g_string_free, g_string_free_and_steal, g_string_insert_len, g_string_new,
};
use self::gtestutils_h::g_assertion_message_expr;
pub use self::gtypes_h::{GDestroyNotify, gboolean, gchar, gint, gpointer, guint};
use self::gutils_h::g_get_system_config_dirs;
use self::lauxlib_h::{luaL_error, luaL_typerror};
pub use self::log_h::{
    _log, LOG_LEVEL_debug, LOG_LEVEL_error, LOG_LEVEL_fatal, LOG_LEVEL_info, LOG_LEVEL_verbose,
    LOG_LEVEL_warn, log_level_t,
};
use self::stdio_h::snprintf;
use self::string_h::{memcpy, memmove, strchr, strcmp, strlen, strncmp};
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_traceback(
    mut L: *mut lua_State,
    mut T: *mut lua_State,
    mut min_level: gint,
) -> gint {
    let mut ar = MaybeUninit::uninit();
    let mut max_level: gint = 0;
    let mut loc_pad: gint = 0 as std::ffi::c_int;
    if lua_getstack(T, min_level, ar.as_mut_ptr()) == 0 {
        lua_pushlstring(L, b"\0" as *const u8 as *const std::ffi::c_char, 0);
        return 1 as std::ffi::c_int;
    }
    let mut level: gint = min_level;
    while lua_getstack(T, level, ar.as_mut_ptr()) != 0 {
        lua_getinfo(
            T,
            b"Sl\0" as *const u8 as *const std::ffi::c_char,
            ar.as_mut_ptr(),
        );
        max_level = level;
        let mut cur_pad: gint = snprintf(
            0 as *mut std::ffi::c_char,
            0 as std::ffi::c_int as std::ffi::c_ulong,
            b"%s:%d\0" as *const u8 as *const std::ffi::c_char,
            if !(g_strstr_len(
                (*ar.as_ptr()).source,
                3 as std::ffi::c_int as gssize,
                b"@./\0" as *const u8 as *const std::ffi::c_char,
            ))
            .is_null()
            {
                ((*ar.as_ptr()).source).offset(3 as std::ffi::c_int as isize)
            } else if *((*ar.as_ptr()).source).offset(0 as std::ffi::c_int as isize)
                as std::ffi::c_int
                == '@' as i32
            {
                ((*ar.as_ptr()).source).offset(1 as std::ffi::c_int as isize)
            } else {
                ((*ar.as_ptr()).short_src).as_ptr() as *const std::ffi::c_char
            },
            (*ar.as_ptr()).currentline,
        );
        if cur_pad > loc_pad {
            loc_pad = cur_pad;
        }
        level += 1;
        level;
    }
    let mut tb: *mut GString = g_string_new(b"\0" as *const u8 as *const std::ffi::c_char);
    let mut level_pad: gint = snprintf(
        0 as *mut std::ffi::c_char,
        0 as std::ffi::c_int as std::ffi::c_ulong,
        b"%d\0" as *const u8 as *const std::ffi::c_char,
        max_level,
    );
    let mut level_0: gint = min_level;
    while level_0 <= max_level {
        lua_getstack(T, level_0, ar.as_mut_ptr());
        lua_getinfo(
            T,
            b"Sln\0" as *const u8 as *const std::ffi::c_char,
            ar.as_mut_ptr(),
        );
        let mut shown_level: gint = level_0 - min_level + 1 as std::ffi::c_int;
        g_string_append_printf(
            tb,
            b"\x1B[37m(%*d)\x1B[0m \0" as *const u8 as *const std::ffi::c_char,
            level_pad,
            shown_level,
        );
        if strcmp(
            (*ar.as_ptr()).what,
            b"C\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            g_string_append_printf(
                tb,
                b"%-*s\0" as *const u8 as *const std::ffi::c_char,
                loc_pad,
                b"[C]\0" as *const u8 as *const std::ffi::c_char,
            );
        } else {
            let mut src: *const std::ffi::c_char = if !(g_strstr_len(
                (*ar.as_ptr()).source,
                3 as std::ffi::c_int as gssize,
                b"@./\0" as *const u8 as *const std::ffi::c_char,
            ))
            .is_null()
            {
                ((*ar.as_ptr()).source).offset(3 as std::ffi::c_int as isize)
            } else if *((*ar.as_ptr()).source).offset(0 as std::ffi::c_int as isize)
                as std::ffi::c_int
                == '@' as i32
            {
                ((*ar.as_ptr()).source).offset(1 as std::ffi::c_int as isize)
            } else {
                ((*ar.as_ptr()).short_src).as_ptr() as *const std::ffi::c_char
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
                (*ar.as_ptr()).currentline,
            );
            n = (strlen(src))
                .wrapping_add(strlen(cl.as_mut_ptr()))
                .wrapping_add(1 as std::ffi::c_int as std::ffi::c_ulong)
                as std::ffi::c_int;
            g_string_append_printf(
                tb,
                b"%s:%d\0" as *const u8 as *const std::ffi::c_char,
                src,
                (*ar.as_ptr()).currentline,
            );
            g_string_append_printf(
                tb,
                b"%*.*s\0" as *const u8 as *const std::ffi::c_char,
                loc_pad - n,
                loc_pad - n,
                b"\0" as *const u8 as *const std::ffi::c_char,
            );
        }
        if strcmp(
            (*ar.as_ptr()).what,
            b"main\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            if 0 != 0 {
                ({
                    let __val: *const std::ffi::c_char =
                        b"\x1B[37m in main chunk\x1B[0m\0" as *const u8 as *const std::ffi::c_char;
                    g_string_append_len_inline(
                        tb,
                        __val,
                        if !__val.is_null() {
                            strlen(__val.offset(__val.is_null() as std::ffi::c_int as isize))
                                as gssize
                        } else {
                            -(1 as std::ffi::c_int) as gssize
                        },
                    );
                });
                ({
                    let __val: *const std::ffi::c_char =
                        b"\x1B[37m in main chunk\x1B[0m\0" as *const u8 as *const std::ffi::c_char;
                    g_string_append_len_inline(
                        tb,
                        __val,
                        if !__val.is_null() {
                            strlen(__val.offset(__val.is_null() as std::ffi::c_int as isize))
                                as gssize
                        } else {
                            -(1 as std::ffi::c_int) as gssize
                        },
                    );
                });
            } else {
                g_string_append_len_inline(
                    tb,
                    b"\x1B[37m in main chunk\x1B[0m\0" as *const u8 as *const std::ffi::c_char,
                    -(1 as std::ffi::c_int) as gssize,
                );
            };
        } else {
            g_string_append_printf(
                tb,
                b"\x1B[37m in function \x1B[0m%s\0" as *const u8 as *const std::ffi::c_char,
                if !((*ar.as_ptr()).name).is_null() {
                    (*ar.as_ptr()).name
                } else {
                    b"[anonymous]\0" as *const u8 as *const std::ffi::c_char
                },
            );
        }
        if level_0 != max_level {
            if 0 != 0 {
                ({
                    let __val: *const std::ffi::c_char =
                        b"\n\0" as *const u8 as *const std::ffi::c_char;
                    g_string_append_len_inline(
                        tb,
                        __val,
                        if !__val.is_null() {
                            strlen(__val.offset(__val.is_null() as std::ffi::c_int as isize))
                                as gssize
                        } else {
                            -(1 as std::ffi::c_int) as gssize
                        },
                    );
                });
                ({
                    let __val: *const std::ffi::c_char =
                        b"\n\0" as *const u8 as *const std::ffi::c_char;
                    g_string_append_len_inline(
                        tb,
                        __val,
                        if !__val.is_null() {
                            strlen(__val.offset(__val.is_null() as std::ffi::c_int as isize))
                                as gssize
                        } else {
                            -(1 as std::ffi::c_int) as gssize
                        },
                    );
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
unsafe extern "C" fn extract_error_message(
    mut L: *mut lua_State,
    mut message: *const gchar,
) -> *const gchar {
    let mut ar = MaybeUninit::uninit();
    let mut level: gint = 0 as std::ffi::c_int;
    loop {
        if lua_getstack(L, level, ar.as_mut_ptr()) == 0 {
            return message;
        }
        lua_getinfo(
            L,
            b"Sl\0" as *const u8 as *const std::ffi::c_char,
            ar.as_mut_ptr(),
        );
        if !(strcmp(
            (*ar.as_ptr()).what,
            b"C\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int)
        {
            break;
        }
        level += 1;
        level;
    }
    if strncmp(
        message,
        ((*ar.as_ptr()).short_src).as_ptr(),
        strlen(((*ar.as_ptr()).short_src).as_ptr()),
    ) != 0
    {
        return message;
    }
    let mut tail: *const gchar = message.offset(strlen((*ar.as_ptr()).short_src.as_ptr()) as isize);
    if *tail as std::ffi::c_int != ':' as i32 {
        return message;
    }
    tail = tail.offset(1);
    tail;
    return (strchr(tail, ' ' as i32)).offset(1 as std::ffi::c_int as isize);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_dofunction_on_error(mut L: *mut lua_State) -> gint {
    if lua_checkstack(L, 5 as std::ffi::c_int) != 0 {
    } else {
        g_assertion_message_expr(
            0 as *mut gchar,
            b"common/luautil.c\0" as *const u8 as *const std::ffi::c_char,
            129 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 25], &[std::ffi::c_char; 25]>(
                b"luaH_dofunction_on_error\0",
            ))
            .as_ptr(),
            b"lua_checkstack(L, 5)\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    lua_pushlstring(
        L,
        b"Lua error: \0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 12]>())
            .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
            .wrapping_sub(1),
    );
    lua_pushstring(
        L,
        extract_error_message(
            L,
            lua_tolstring(L, -(2 as std::ffi::c_int), 0 as *mut usize),
        ),
    );
    lua_pushlstring(
        L,
        b"\nTraceback:\n\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 13]>())
            .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
            .wrapping_sub(1),
    );
    luaH_traceback(L, L, 1 as std::ffi::c_int);
    lua_concat(L, 4 as std::ffi::c_int);
    return 1 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_add_paths(mut L: *mut lua_State, mut config_dir: *const gchar) {
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
    let mut paths: *mut GPtrArray =
        g_ptr_array_new_with_free_func(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
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
            (::core::mem::size_of::<[std::ffi::c_char; 2]>())
                .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
                .wrapping_sub(1),
        );
        lua_pushstring(L, path);
        lua_pushlstring(
            L,
            b"/?.lua\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 7]>())
                .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
                .wrapping_sub(1),
        );
        lua_concat(L, 3 as std::ffi::c_int);
        lua_pushlstring(
            L,
            b";\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 2]>())
                .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
                .wrapping_sub(1),
        );
        lua_pushstring(L, path);
        lua_pushlstring(
            L,
            b"/?/init.lua\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 12]>())
                .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
                .wrapping_sub(1),
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_push_gerror(mut L: *mut lua_State, mut error: *mut GError) -> gint {
    if !error.is_null() {
    } else {
        g_assertion_message_expr(
            0 as *mut gchar,
            b"common/luautil.c\0" as *const u8 as *const std::ffi::c_char,
            205 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 17], &[std::ffi::c_char; 17]>(b"luaH_push_gerror\0"))
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_push_strv(
    mut L: *mut lua_State,
    mut strv: *const *const gchar,
) -> gint {
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    if strv.is_null() {
        return 1 as std::ffi::c_int;
    }
    let mut n = 1;
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_checkstrv(mut L: *mut lua_State, mut idx: gint) -> *mut *const gchar {
    if !(lua_type(L, idx) == 5 as std::ffi::c_int) {
        luaL_typerror(L, idx, b"table\0" as *const u8 as *const std::ffi::c_char);
    }
    let mut len: gint = lua_rawlen(L, idx) as gint;
    let mut langs: *mut GPtrArray = g_ptr_array_new();
    let mut i: gint = 1 as std::ffi::c_int;
    while i <= len {
        lua_rawgeti(L, idx, i as lua_Integer);
        if lua_isstring(L, -(1 as std::ffi::c_int)) == 0 {
            g_ptr_array_free(langs, (0 as std::ffi::c_int == 0) as std::ffi::c_int);
            luaL_error(
                L,
                b"bad argument %d ({string} expected, but array item %d has type %s)\0" as *const u8
                    as *const std::ffi::c_char,
                idx,
                i,
                lua_typename(L, lua_type(L, -(1 as std::ffi::c_int))),
            );
        }
        g_ptr_array_add(
            langs,
            lua_tolstring(L, -(1 as std::ffi::c_int), std::ptr::null_mut()) as *mut gchar
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
