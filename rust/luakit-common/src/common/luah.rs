use gio_sys::{GFile, g_file_get_path, g_file_new_for_path};
use glib_sys::*;
use libc::*;
use mlua_sys::*;
use std::mem::MaybeUninit;

use crate::{
    common::{
        luaclass::luaH_typename, lualib::luaH_dofunction, luautil::luaH_traceback,
        util::strip_ansi_escapes,
    },
    gtypes::*,
    log::{_log, LOG_LEVEL_warn},
};

unsafe extern "C-unwind" fn luaH_utf8_strlen(mut L: *mut lua_State) -> gint {
    let mut cmd: *const gchar = luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t);
    lua_pushnumber(
        L,
        g_utf8_strlen(
            if !cmd.is_null() {
                cmd
            } else {
                b"\0" as *const u8 as *const std::ffi::c_char
            },
            -1,
        ) as lua_Number,
    );
    return 1 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaHe_next(mut L: *mut lua_State) -> gint {
    if luaL_getmetafield(
        L,
        1 as std::ffi::c_int,
        b"__next\0" as *const u8 as *const std::ffi::c_char,
    ) != 0
    {
        lua_insert(L, 1 as std::ffi::c_int);
        lua_call(
            L,
            lua_gettop(L) - 1 as std::ffi::c_int,
            -(1 as std::ffi::c_int),
        );
        return lua_gettop(L);
    }
    luaL_checktype(L, 1 as std::ffi::c_int, 5 as std::ffi::c_int);
    lua_settop(L, 2 as std::ffi::c_int);
    if lua_next(L, 1 as std::ffi::c_int) != 0 {
        return 2 as std::ffi::c_int;
    }
    lua_pushnil(L);
    return 1 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_mtnext(mut L: *mut lua_State, mut idx: gint) -> gint {
    if luaL_getmetafield(L, idx, b"__next\0" as *const u8 as *const std::ffi::c_char) != 0 {
        if idx < 0 as std::ffi::c_int {
            idx -= 1;
            idx;
        }
        lua_pushvalue(L, idx);
        lua_pushvalue(L, -(3 as std::ffi::c_int));
        lua_remove(L, -(4 as std::ffi::c_int));
        lua_pcall(
            L,
            2 as std::ffi::c_int,
            2 as std::ffi::c_int,
            0 as std::ffi::c_int,
        );
        if lua_type(L, -(1 as std::ffi::c_int)) == 0 as std::ffi::c_int {
            lua_settop(L, -(2 as std::ffi::c_int) - 1 as std::ffi::c_int);
            return 0 as std::ffi::c_int;
        }
        return 1 as std::ffi::c_int;
    } else if lua_type(L, idx) == 5 as std::ffi::c_int {
        return lua_next(L, idx);
    }
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    return 0 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_generic_pairs(mut L: *mut lua_State) -> gint {
    lua_pushvalue(L, -(10002 as std::ffi::c_int) - 1 as std::ffi::c_int);
    lua_pushvalue(L, 1 as std::ffi::c_int);
    lua_pushnil(L);
    return 3 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaHe_pairs(mut L: *mut lua_State) -> gint {
    if luaL_getmetafield(
        L,
        1 as std::ffi::c_int,
        b"__pairs\0" as *const u8 as *const std::ffi::c_char,
    ) != 0
    {
        lua_insert(L, 1 as std::ffi::c_int);
        lua_call(
            L,
            lua_gettop(L) - 1 as std::ffi::c_int,
            -(1 as std::ffi::c_int),
        );
        return lua_gettop(L);
    }
    luaL_checktype(L, 1 as std::ffi::c_int, 5 as std::ffi::c_int);
    return luaH_generic_pairs(L);
}
unsafe extern "C-unwind" fn luaH_ipairs_aux(mut L: *mut lua_State) -> gint {
    let mut i: gint =
        luaL_checkinteger(L, 2 as std::ffi::c_int) as std::ffi::c_int + 1 as std::ffi::c_int;
    luaL_checktype(L, 1 as std::ffi::c_int, 5 as std::ffi::c_int);
    lua_pushinteger(L, i as lua_Integer);
    lua_rawgeti(L, 1 as std::ffi::c_int, i as lua_Integer);
    return if lua_type(L, -(1 as std::ffi::c_int)) == 0 as std::ffi::c_int {
        0 as std::ffi::c_int
    } else {
        2 as std::ffi::c_int
    };
}
unsafe extern "C-unwind" fn luaHe_ipairs(mut L: *mut lua_State) -> gint {
    if luaL_getmetafield(
        L,
        1 as std::ffi::c_int,
        b"__ipairs\0" as *const u8 as *const std::ffi::c_char,
    ) != 0
    {
        lua_insert(L, 1 as std::ffi::c_int);
        lua_call(
            L,
            lua_gettop(L) - 1 as std::ffi::c_int,
            -(1 as std::ffi::c_int),
        );
        return lua_gettop(L);
    }
    luaL_checktype(L, 1 as std::ffi::c_int, 5 as std::ffi::c_int);
    lua_pushvalue(L, -(10002 as std::ffi::c_int) - 1 as std::ffi::c_int);
    lua_pushvalue(L, 1 as std::ffi::c_int);
    lua_pushinteger(L, 0 as std::ffi::c_int as lua_Integer);
    return 3 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaHe_type(mut L: *mut lua_State) -> gint {
    luaL_checkany(L, 1 as std::ffi::c_int);
    lua_pushstring(L, luaH_typename(L, 1 as std::ffi::c_int));
    return 1 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_abspath(mut L: *mut lua_State) -> gint {
    let mut path: *const gchar = luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t);
    let mut file: *mut GFile = g_file_new_for_path(path);
    if file.is_null() {
        return 0 as std::ffi::c_int;
    }
    let mut absolute: *mut gchar = g_file_get_path(file);
    if absolute.is_null() {
        return 0 as std::ffi::c_int;
    }
    lua_pushstring(L, absolute);
    g_free(absolute as gpointer);
    return 1 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_debug_traceback(mut L: *mut lua_State) -> gint {
    let mut thread: *mut lua_State = 0 as *mut lua_State;
    thread = lua_tothread(L, 1 as std::ffi::c_int);
    if !thread.is_null() {
        lua_remove(L, 1 as std::ffi::c_int);
    }
    let mut msg: *const gchar = luaL_optlstring(
        L,
        1 as std::ffi::c_int,
        0 as *const std::ffi::c_char,
        0 as *mut size_t,
    );
    let mut level: std::ffi::c_int = luaL_optnumber(
        L,
        if !msg.is_null() {
            2 as std::ffi::c_int
        } else {
            1 as std::ffi::c_int
        },
        1 as std::ffi::c_int as lua_Number,
    ) as std::ffi::c_int;
    lua_pushstring(
        L,
        if !msg.is_null() {
            msg
        } else {
            b"\0" as *const u8 as *const std::ffi::c_char
        },
    );
    lua_pushstring(
        L,
        if !msg.is_null() {
            b"\nTraceback:\n\0" as *const u8 as *const std::ffi::c_char
        } else {
            b"Traceback:\n\0" as *const u8 as *const std::ffi::c_char
        },
    );
    luaH_traceback(L, if !thread.is_null() { thread } else { L }, level);
    let mut stripped: *mut gchar =
        strip_ansi_escapes(lua_tolstring(L, -(1 as std::ffi::c_int), 0 as *mut size_t));
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    lua_pushstring(L, stripped);
    lua_concat(L, 3 as std::ffi::c_int);
    g_free(stripped as gpointer);
    return 1 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_fixups(mut L: *mut lua_State) {
    lua_getfield(
        L,
        -(10002 as std::ffi::c_int),
        b"string\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_pushcclosure(L, luaH_utf8_strlen, 0 as std::ffi::c_int);
    lua_setfield(
        L,
        -(2 as std::ffi::c_int),
        b"wlen\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    lua_getfield(
        L,
        -(10002 as std::ffi::c_int),
        b"os\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_pushcclosure(L, luaH_abspath, 0 as std::ffi::c_int);
    lua_setfield(
        L,
        -(2 as std::ffi::c_int),
        b"abspath\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    lua_pushlstring(
        L,
        b"next\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 5]>())
            .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
            .wrapping_sub(1),
    );
    lua_pushcclosure(L, luaHe_next, 0 as std::ffi::c_int);
    lua_settable(L, -(10002 as std::ffi::c_int));
    lua_pushlstring(
        L,
        b"pairs\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 6]>())
            .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
            .wrapping_sub(1),
    );
    lua_pushcclosure(L, luaHe_next, 0 as std::ffi::c_int);
    lua_pushcclosure(L, luaHe_pairs, 1 as std::ffi::c_int);
    lua_settable(L, -(10002 as std::ffi::c_int));
    lua_pushlstring(
        L,
        b"ipairs\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 7]>())
            .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
            .wrapping_sub(1),
    );
    lua_pushcclosure(L, luaH_ipairs_aux, 0 as std::ffi::c_int);
    lua_pushcclosure(L, luaHe_ipairs, 1 as std::ffi::c_int);
    lua_settable(L, -(10002 as std::ffi::c_int));
    lua_pushlstring(
        L,
        b"type\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 5]>())
            .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
            .wrapping_sub(1),
    );
    lua_pushcclosure(L, luaHe_type, 0 as std::ffi::c_int);
    lua_settable(L, -(10002 as std::ffi::c_int));
    lua_getfield(
        L,
        -(10002 as std::ffi::c_int),
        b"debug\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_pushcclosure(L, luaH_debug_traceback, 0 as std::ffi::c_int);
    lua_setfield(
        L,
        -(2 as std::ffi::c_int),
        b"traceback\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
}

pub unsafe extern "C" fn luaH_warn(mut L: *mut lua_State, mut fmt: *const gchar, mut args: ...) {
    let mut top: gint = lua_gettop(L);
    let mut ar = MaybeUninit::<lua_Debug>::uninit();
    lua_getstack(L, 1 as std::ffi::c_int, ar.as_mut_ptr());
    lua_getinfo(
        L,
        b"Sln\0" as *const u8 as *const std::ffi::c_char,
        ar.as_mut_ptr(),
    );
    let mut __n1: gint64 = top as gint64;
    let mut __n2: gint64 = lua_gettop(L) as gint64;
    if !(__n1 == __n2) {
        /*
        g_assertion_message_cmpint(
            0 as *mut gchar,
            b"./common/luah.h\0" as *const u8 as *const std::ffi::c_char,
            142 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 10], &[std::ffi::c_char; 10]>(b"luaH_warn\0"))
                .as_ptr(),
            b"top == lua_gettop(L)\0" as *const u8 as *const std::ffi::c_char,
            __n1 as guint64,
            b"==\0" as *const u8 as *const std::ffi::c_char,
            __n2 as guint64,
            'i' as i32 as std::ffi::c_char,
        );
        */
    }
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    _log(
        LOG_LEVEL_warn,
        ((*ar.as_ptr()).short_src).as_ptr(),
        fmt,
        ap.as_va_list(),
    );
}

pub unsafe extern "C" fn luaH_dofunction_from_registry(
    mut L: *mut lua_State,
    mut ref_0: gint,
    mut nargs: gint,
    mut nret: gint,
) -> gboolean {
    lua_rawgeti(L, LUA_REGISTRYINDEX, ref_0 as i64);
    return luaH_dofunction(L, nargs, nret);
}

pub unsafe extern "C" fn luaH_checkboolean(mut L: *mut lua_State, mut n: gint) -> gboolean {
    if !(lua_type(L, n) == LUA_TBOOLEAN) {
        luaL_typerror(L, n, b"boolean\0" as *const u8 as *const std::ffi::c_char);
    }
    return lua_toboolean(L, n);
}
pub unsafe extern "C" fn luaH_rawfield(
    mut L: *mut lua_State,
    mut idx: gint,
    mut field: *const gchar,
) -> gint {
    lua_pushstring(L, field);
    lua_rawget(L, idx);
    let mut type_0 = lua_type(L, -(1 as std::ffi::c_int));
    if type_0 == LUA_TNIL {
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    }
    return type_0;
}
