use ::libc;
#[c2rust::header_src = "/usr/lib/clang/20/include/__stddef_size_t.h:23"]
pub mod __stddef_size_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type size_t = std::ffi::c_ulong;
}
#[c2rust::header_src = "/usr/lib/glib-2.0/include/glibconfig.h:23"]
pub mod glibconfig_h {
    #[c2rust::src_loc = "57:1"]
    pub type guint32 = std::ffi::c_uint;
    #[c2rust::src_loc = "82:1"]
    pub type gssize = std::ffi::c_long;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtypes.h:23"]
pub mod gtypes_h {
    #[c2rust::src_loc = "52:1"]
    pub type gchar = std::ffi::c_char;
    #[c2rust::src_loc = "55:1"]
    pub type gint = std::ffi::c_int;
    #[c2rust::src_loc = "56:1"]
    pub type gboolean = gint;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gquark.h:23"]
pub mod gquark_h {
    #[c2rust::src_loc = "38:1"]
    pub type GQuark = guint32;
    use super::glibconfig_h::guint32;
    use super::gtypes_h::gchar;
    extern "C" {
        #[c2rust::src_loc = "44:1"]
        pub fn g_quark_from_static_string(string: *const gchar) -> GQuark;
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
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gregex.h:23"]
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
    use super::gtypes_h::{gchar, gint};
    use super::gerror_h::GError;
    use super::glibconfig_h::gssize;
    extern "C" {
        #[c2rust::src_loc = "416:16"]
        pub type _GRegex;
        #[c2rust::src_loc = "449:1"]
        pub fn g_regex_new(
            pattern: *const gchar,
            compile_options: GRegexCompileFlags,
            match_options: GRegexMatchFlags,
            error: *mut *mut GError,
        ) -> *mut GRegex;
        #[c2rust::src_loc = "544:1"]
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
#[c2rust::header_src = "/usr/include/luajit-2.1/lua.h:23"]
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
        #[c2rust::src_loc = "150:1"]
        pub fn lua_tolstring(
            L: *mut lua_State,
            idx: std::ffi::c_int,
            len: *mut size_t,
        ) -> *const std::ffi::c_char;
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
#[c2rust::header_src = "/home/daana/git/luakit/common/log.h:23"]
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
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gstrfuncs.h:23"]
pub mod gstrfuncs_h {
    use super::gtypes_h::gchar;
    extern "C" {
        #[c2rust::src_loc = "285:1"]
        pub fn g_strdup_printf(format: *const gchar, _: ...) -> *mut gchar;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtestutils.h:23"]
pub mod gtestutils_h {
    use super::gerror_h::GError;
    use super::gquark_h::GQuark;
    extern "C" {
        #[c2rust::src_loc = "670:1"]
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
#[c2rust::header_src = "/usr/include/unistd.h:23"]
pub mod unistd_h {
    extern "C" {
        #[c2rust::src_loc = "287:1"]
        pub fn access(
            __name: *const std::ffi::c_char,
            __type: std::ffi::c_int,
        ) -> std::ffi::c_int;
    }
}
pub use self::__stddef_size_t_h::size_t;
pub use self::glibconfig_h::{guint32, gssize};
pub use self::gtypes_h::{gchar, gint, gboolean};
pub use self::gquark_h::{GQuark, g_quark_from_static_string};
pub use self::gerror_h::{_GError, GError};
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
    G_REGEX_MATCH_NOTBOL, G_REGEX_MATCH_ANCHORED, G_REGEX_MATCH_DEFAULT, GRegex, _GRegex,
    g_regex_new, g_regex_replace_literal,
};
pub use self::lua_h::{lua_Debug, lua_State, lua_tolstring, lua_getstack, lua_getinfo};
pub use self::log_h::{
    log_level_t, LOG_LEVEL_debug, LOG_LEVEL_verbose, LOG_LEVEL_info, LOG_LEVEL_warn,
    LOG_LEVEL_error, LOG_LEVEL_fatal, _log,
};
use self::gstrfuncs_h::g_strdup_printf;
use self::gtestutils_h::g_assertion_message_error;
use self::unistd_h::access;
#[no_mangle]
#[c2rust::src_loc = "30:1"]
pub unsafe extern "C" fn file_exists(mut filename: *const gchar) -> gboolean {
    return (access(filename, 0 as std::ffi::c_int) == 0 as std::ffi::c_int)
        as std::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "38:1"]
pub unsafe extern "C" fn luaH_callerinfo(mut L: *mut lua_State) -> *mut gchar {
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
    if lua_getstack(L, 1 as std::ffi::c_int, &mut ar) != 0
        && lua_getinfo(L, b"Sln\0" as *const u8 as *const std::ffi::c_char, &mut ar) != 0
    {
        return g_strdup_printf(
            b"%s%s%s:%d\0" as *const u8 as *const std::ffi::c_char,
            (ar.short_src).as_mut_ptr(),
            if !(ar.name).is_null() {
                b":\0" as *const u8 as *const std::ffi::c_char
            } else {
                b"\0" as *const u8 as *const std::ffi::c_char
            },
            if !(ar.name).is_null() {
                ar.name
            } else {
                b"\0" as *const u8 as *const std::ffi::c_char
            },
            ar.currentline,
        );
    }
    return 0 as *mut gchar;
}
#[no_mangle]
#[c2rust::src_loc = "51:1"]
pub unsafe extern "C" fn luaH_panic(mut L: *mut lua_State) -> gint {
    _log(
        LOG_LEVEL_error,
        b"common/util.c\0" as *const u8 as *const std::ffi::c_char,
        b"unprotected error in call to Lua API (%s)\0" as *const u8
            as *const std::ffi::c_char,
        lua_tolstring(L, -(1 as std::ffi::c_int), 0 as *mut size_t),
    );
    return 0 as std::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "58:1"]
pub unsafe extern "C" fn strip_ansi_escapes(mut in_0: *const gchar) -> *mut gchar {
    static mut reg: *mut GRegex = 0 as *const GRegex as *mut GRegex;
    if reg.is_null() {
        let mut expr: *const gchar = b"[\x1B\x9B][[()#;?]*(?:[0-9]{1,4}(?:;[0-9]{0,4})*)?[0-9A-ORZcf-nqry=><]\0"
            as *const u8 as *const std::ffi::c_char;
        let mut err: *mut GError = 0 as *mut GError;
        reg = g_regex_new(
            expr,
            (G_REGEX_DOTALL as std::ffi::c_int | G_REGEX_EXTENDED as std::ffi::c_int
                | G_REGEX_RAW as std::ffi::c_int) as GRegexCompileFlags,
            G_REGEX_MATCH_DEFAULT,
            &mut err,
        );
        if !err.is_null() {
            g_assertion_message_error(
                0 as *mut gchar,
                b"common/util.c\0" as *const u8 as *const std::ffi::c_char,
                67 as std::ffi::c_int,
                (*::core::mem::transmute::<
                    &[u8; 19],
                    &[std::ffi::c_char; 19],
                >(b"strip_ansi_escapes\0"))
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
#[no_mangle]
#[c2rust::src_loc = "73:1"]
pub unsafe extern "C" fn luakit_error_quark() -> GQuark {
    return g_quark_from_static_string(
        b"LuakitError\0" as *const u8 as *const std::ffi::c_char,
    );
}
