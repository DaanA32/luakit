use std::{ffi::CStr, mem::MaybeUninit};

use ::libc;
use glib_sys::{
    G_REGEX_DOTALL, G_REGEX_EXTENDED, G_REGEX_MATCH_DEFAULT, G_REGEX_RAW, GError, GQuark, GRegex,
    GRegexCompileFlags, g_assertion_message_error, g_quark_from_static_string, g_regex_new,
    g_regex_replace_literal, g_strdup_printf, gboolean,
};
use libc::{access, ssize_t};
use mlua_sys::{lua_Debug, lua_State, lua_getinfo, lua_getstack, lua_tolstring};

use crate::{
    gtypes::gchar,
    log::{_log, LOG_LEVEL_error},
};
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn file_exists(mut filename: *const gchar) -> gboolean {
    return (access(filename, 0 as std::ffi::c_int) == 0 as std::ffi::c_int) as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_callerinfo(mut L: *mut lua_State) -> *mut gchar {
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
pub unsafe extern "C-unwind" fn luaH_panic(mut L: *mut lua_State) -> std::ffi::c_int {
    _log(
        LOG_LEVEL_error,
        b"common/util.c\0" as *const u8 as *const std::ffi::c_char,
        &format!(
            "unprotected error in call to Lua API ({})",
            CStr::from_ptr(lua_tolstring(L, -(1 as std::ffi::c_int), 0 as *mut usize))
                .to_string_lossy()
        ),
    );
    return 0 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn strip_ansi_escapes(mut in_0: *const gchar) -> *mut gchar {
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
        -(1 as std::ffi::c_int) as ssize_t,
        0 as std::ffi::c_int,
        b"\0" as *const u8 as *const std::ffi::c_char,
        G_REGEX_MATCH_DEFAULT,
        0 as *mut *mut GError,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luakit_error_quark() -> GQuark {
    return g_quark_from_static_string(b"LuakitError\0" as *const u8 as *const std::ffi::c_char);
}
