use std::mem::MaybeUninit;

use ::libc;
use lua::ffi::{lua_Debug, lua_State, lua_getinfo, lua_getstack, lua_tolstring};
pub mod __stddef_size_t_h {
    pub type size_t = std::ffi::c_ulong;
}
pub mod glibconfig_h {
    pub type guint32 = std::ffi::c_uint;
    pub type gssize = std::ffi::c_long;
}
pub mod gtypes_h {
    pub type gchar = std::ffi::c_char;
    pub type gint = std::ffi::c_int;
    pub type gboolean = gint;
}
pub mod gquark_h {
    pub type GQuark = guint32;
    use super::glibconfig_h::guint32;
    use super::gtypes_h::gchar;
    unsafe extern "C" {
        pub fn g_quark_from_static_string(string: *const gchar) -> GQuark;
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
    use super::gerror_h::GError;
    use super::glibconfig_h::gssize;
    use super::gtypes_h::{gchar, gint};
    unsafe extern "C" {
        pub type _GRegex;
        pub fn g_regex_new(
            pattern: *const gchar,
            compile_options: GRegexCompileFlags,
            match_options: GRegexMatchFlags,
            error: *mut *mut GError,
        ) -> *mut GRegex;
        pub fn g_regex_replace_literal(
            regex: *const GRegex,
            string: *const gchar,
            string_len: gssize,
            start_position: gint,
            replacement: *const gchar,
            match_options: GRegexMatchFlags,
            error: *mut *mut GError,
        ) -> *mut gchar;
    }
}
pub mod lua_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
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
    unsafe extern "C" {
        pub type lua_State;
        pub fn lua_tolstring(
            L: *mut lua_State,
            idx: std::ffi::c_int,
            len: *mut size_t,
        ) -> *const std::ffi::c_char;
        pub fn lua_getstack(
            L: *mut lua_State,
            level: std::ffi::c_int,
            ar: *mut lua_Debug,
        ) -> std::ffi::c_int;
        pub fn lua_getinfo(
            L: *mut lua_State,
            what: *const std::ffi::c_char,
            ar: *mut lua_Debug,
        ) -> std::ffi::c_int;
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
pub mod gstrfuncs_h {
    use super::gtypes_h::gchar;
    unsafe extern "C" {
        pub fn g_strdup_printf(format: *const gchar, _: ...) -> *mut gchar;
    }
}
pub mod gtestutils_h {
    use super::gerror_h::GError;
    use super::gquark_h::GQuark;
    unsafe extern "C" {
        pub fn g_assertion_message_error(
            domain: *const std::ffi::c_char,
            file: *const std::ffi::c_char,
            line: std::ffi::c_int,
            func: *const std::ffi::c_char,
            expr: *const std::ffi::c_char,
            error: *const GError,
            error_domain: GQuark,
            error_code: std::ffi::c_int,
        );
    }
}
pub mod unistd_h {
    unsafe extern "C" {
        pub fn access(__name: *const std::ffi::c_char, __type: std::ffi::c_int) -> std::ffi::c_int;
    }
}
pub use self::__stddef_size_t_h::size_t;
pub use self::gerror_h::{_GError, GError};
pub use self::glibconfig_h::{gssize, guint32};
pub use self::gquark_h::{GQuark, g_quark_from_static_string};
pub use self::gregex_h::{
    _GRegex, G_REGEX_ANCHORED, G_REGEX_BSR_ANYCRLF, G_REGEX_CASELESS, G_REGEX_DEFAULT,
    G_REGEX_DOLLAR_ENDONLY, G_REGEX_DOTALL, G_REGEX_DUPNAMES, G_REGEX_EXTENDED, G_REGEX_FIRSTLINE,
    G_REGEX_JAVASCRIPT_COMPAT, G_REGEX_MATCH_ANCHORED, G_REGEX_MATCH_BSR_ANY,
    G_REGEX_MATCH_BSR_ANYCRLF, G_REGEX_MATCH_DEFAULT, G_REGEX_MATCH_NEWLINE_ANY,
    G_REGEX_MATCH_NEWLINE_ANYCRLF, G_REGEX_MATCH_NEWLINE_CR, G_REGEX_MATCH_NEWLINE_CRLF,
    G_REGEX_MATCH_NEWLINE_LF, G_REGEX_MATCH_NOTBOL, G_REGEX_MATCH_NOTEMPTY,
    G_REGEX_MATCH_NOTEMPTY_ATSTART, G_REGEX_MATCH_NOTEOL, G_REGEX_MATCH_PARTIAL,
    G_REGEX_MATCH_PARTIAL_HARD, G_REGEX_MATCH_PARTIAL_SOFT, G_REGEX_MULTILINE,
    G_REGEX_NEWLINE_ANYCRLF, G_REGEX_NEWLINE_CR, G_REGEX_NEWLINE_CRLF, G_REGEX_NEWLINE_LF,
    G_REGEX_NO_AUTO_CAPTURE, G_REGEX_OPTIMIZE, G_REGEX_RAW, G_REGEX_UNGREEDY, GRegex,
    GRegexCompileFlags, GRegexMatchFlags, g_regex_new, g_regex_replace_literal,
};
use self::gstrfuncs_h::g_strdup_printf;
use self::gtestutils_h::g_assertion_message_error;
pub use self::gtypes_h::{gboolean, gchar, gint};
pub use self::log_h::{
    _log, LOG_LEVEL_debug, LOG_LEVEL_error, LOG_LEVEL_fatal, LOG_LEVEL_info, LOG_LEVEL_verbose,
    LOG_LEVEL_warn, log_level_t,
};
use self::unistd_h::access;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn file_exists(mut filename: *const gchar) -> gboolean {
    return (access(filename, 0 as std::ffi::c_int) == 0 as std::ffi::c_int) as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_callerinfo(mut L: *mut lua_State) -> *mut gchar {
    let mut ar = MaybeUninit::<lua_Debug>::uninit();
    if lua_getstack(L, 1 as std::ffi::c_int, ar.as_mut_ptr()) != 0
        && lua_getinfo(
            L,
            b"Sln\0" as *const u8 as *const std::ffi::c_char,
            ar.as_mut_ptr(),
        ) != 0
    {
        return g_strdup_printf(
            b"%s%s%s:%d\0" as *const u8 as *const std::ffi::c_char,
            ((*ar.as_ptr()).short_src).as_ptr(),
            if !((*ar.as_ptr()).name).is_null() {
                b":\0" as *const u8 as *const std::ffi::c_char
            } else {
                b"\0" as *const u8 as *const std::ffi::c_char
            },
            if !(*ar.as_ptr()).name.is_null() {
                (*ar.as_ptr()).name
            } else {
                b"\0" as *const u8 as *const std::ffi::c_char
            },
            (*ar.as_ptr()).currentline,
        );
    }
    return 0 as *mut gchar;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_panic(mut L: *mut lua_State) -> std::ffi::c_int {
    _log(
        LOG_LEVEL_error,
        b"common/util.c\0" as *const u8 as *const std::ffi::c_char,
        b"unprotected error in call to Lua API (%s)\0" as *const u8 as *const std::ffi::c_char,
        lua_tolstring(L, -(1 as std::ffi::c_int), 0 as *mut usize),
    );
    return 0 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn strip_ansi_escapes(mut in_0: *const gchar) -> *mut gchar {
    static mut reg: *mut GRegex = 0 as *const GRegex as *mut GRegex;
    if reg.is_null() {
        let mut expr: *const gchar =
            b"[\x1B\x9B][[()#;?]*(?:[0-9]{1,4}(?:;[0-9]{0,4})*)?[0-9A-ORZcf-nqry=><]\0" as *const u8
                as *const std::ffi::c_char;
        let mut err: *mut GError = 0 as *mut GError;
        reg = g_regex_new(
            expr,
            (G_REGEX_DOTALL as std::ffi::c_int
                | G_REGEX_EXTENDED as std::ffi::c_int
                | G_REGEX_RAW as std::ffi::c_int) as GRegexCompileFlags,
            G_REGEX_MATCH_DEFAULT,
            &mut err,
        );
        if !err.is_null() {
            g_assertion_message_error(
                0 as *mut gchar,
                b"common/util.c\0" as *const u8 as *const std::ffi::c_char,
                67 as std::ffi::c_int,
                (*::core::mem::transmute::<&[u8; 19], &[std::ffi::c_char; 19]>(
                    b"strip_ansi_escapes\0",
                ))
                .as_ptr(),
                b"err\0" as *const u8 as *const std::ffi::c_char,
                err,
                0 as std::ffi::c_int as GQuark,
                0 as std::ffi::c_int,
            );
        }
    }
    return g_regex_replace_literal(
        reg,
        in_0,
        -(1 as std::ffi::c_int) as gssize,
        0 as std::ffi::c_int,
        b"\0" as *const u8 as *const std::ffi::c_char,
        G_REGEX_MATCH_DEFAULT,
        0 as *mut *mut GError,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luakit_error_quark() -> GQuark {
    return g_quark_from_static_string(b"LuakitError\0" as *const u8 as *const std::ffi::c_char);
}
