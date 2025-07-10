use glib_sys::{g_assertion_message_expr, gpointer};
use mlua_sys::{
    lua_State, lua_createtable, lua_pushlightuserdata, lua_pushnil, lua_pushstring, lua_pushvalue,
    lua_rawget, lua_rawset, lua_remove, lua_setmetatable, lua_settop, lua_type,
};

use crate::gtypes::gchar;

pub unsafe extern "C" fn luaH_uniq_setup(
    mut L: *mut lua_State,
    mut reg: *const gchar,
    mut mode: *const gchar,
) {
    lua_pushstring(
        L,
        if !reg.is_null() {
            reg
        } else {
            b"luakit.uniq.registry\0" as *const u8 as *const std::ffi::c_char
        },
    );
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_pushstring(L, b"__mode\0" as *const u8 as *const std::ffi::c_char);
    lua_pushstring(L, mode);
    lua_rawset(L, -(3 as std::ffi::c_int));
    lua_setmetatable(L, -(2 as std::ffi::c_int));
    lua_rawset(L, -(10000 as std::ffi::c_int));
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_uniq_add(
    mut L: *mut lua_State,
    mut reg: *const gchar,
    mut k: std::ffi::c_int,
    mut oud: std::ffi::c_int,
) -> std::ffi::c_int {
    lua_pushstring(
        L,
        if !reg.is_null() {
            reg
        } else {
            b"luakit.uniq.registry\0" as *const u8 as *const std::ffi::c_char
        },
    );
    lua_rawget(L, -(10000 as std::ffi::c_int));
    lua_pushvalue(
        L,
        if k > 0 as std::ffi::c_int {
            k
        } else {
            k - 1 as std::ffi::c_int
        },
    );
    lua_rawget(L, -(2 as std::ffi::c_int));
    if lua_type(L, -(1 as std::ffi::c_int)) == 0 as std::ffi::c_int {
    } else {
        g_assertion_message_expr(
            0 as *mut gchar,
            b"common/luauniq.c\0" as *const u8 as *const std::ffi::c_char,
            56 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 14], &[std::ffi::c_char; 14]>(b"luaH_uniq_add\0"))
                .as_ptr(),
            b"lua_isnil(L, -1)\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    lua_pushvalue(
        L,
        if k > 0 as std::ffi::c_int {
            k
        } else {
            k - 1 as std::ffi::c_int
        },
    );
    lua_pushvalue(
        L,
        if oud < 0 as std::ffi::c_int {
            oud - 2 as std::ffi::c_int
        } else {
            oud
        },
    );
    lua_rawset(L, -(3 as std::ffi::c_int));
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    return 0 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_uniq_add_ptr(
    mut L: *mut lua_State,
    mut reg: *const gchar,
    mut key: gpointer,
    mut oud: std::ffi::c_int,
) -> std::ffi::c_int {
    lua_pushlightuserdata(L, key);
    luaH_uniq_add(
        L,
        reg,
        -(1 as std::ffi::c_int),
        if oud > 0 as std::ffi::c_int {
            oud
        } else {
            oud - 1 as std::ffi::c_int
        },
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    return 0 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_uniq_get(
    mut L: *mut lua_State,
    mut reg: *const gchar,
    mut k: std::ffi::c_int,
) -> std::ffi::c_int {
    lua_pushstring(
        L,
        if !reg.is_null() {
            reg
        } else {
            b"luakit.uniq.registry\0" as *const u8 as *const std::ffi::c_char
        },
    );
    lua_rawget(L, -(10000 as std::ffi::c_int));
    lua_pushvalue(
        L,
        if k > 0 as std::ffi::c_int {
            k
        } else {
            k - 1 as std::ffi::c_int
        },
    );
    lua_rawget(L, -(2 as std::ffi::c_int));
    lua_remove(L, -(2 as std::ffi::c_int));
    if lua_type(L, -(1 as std::ffi::c_int)) == 0 as std::ffi::c_int {
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        return 0 as std::ffi::c_int;
    }
    return 1 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_uniq_get_ptr(
    mut L: *mut lua_State,
    mut reg: *const gchar,
    mut key: gpointer,
) -> std::ffi::c_int {
    lua_pushlightuserdata(L, key);
    let mut n: std::ffi::c_int = luaH_uniq_get(L, reg, -(1 as std::ffi::c_int));
    lua_remove(L, -(1 as std::ffi::c_int) - n);
    return n;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_uniq_del(
    mut L: *mut lua_State,
    mut reg: *const gchar,
    mut k: std::ffi::c_int,
) {
    lua_pushstring(
        L,
        if !reg.is_null() {
            reg
        } else {
            b"luakit.uniq.registry\0" as *const u8 as *const std::ffi::c_char
        },
    );
    lua_rawget(L, -(10000 as std::ffi::c_int));
    lua_pushvalue(
        L,
        if k > 0 as std::ffi::c_int {
            k
        } else {
            k - 1 as std::ffi::c_int
        },
    );
    lua_rawget(L, -(2 as std::ffi::c_int));
    if !(lua_type(L, -(1 as std::ffi::c_int)) == 0 as std::ffi::c_int) {
    } else {
        g_assertion_message_expr(
            0 as *mut gchar,
            b"common/luauniq.c\0" as *const u8 as *const std::ffi::c_char,
            121 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 14], &[std::ffi::c_char; 14]>(b"luaH_uniq_del\0"))
                .as_ptr(),
            b"!lua_isnil(L, -1)\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    lua_pushvalue(
        L,
        if k > 0 as std::ffi::c_int {
            k
        } else {
            k - 1 as std::ffi::c_int
        },
    );
    lua_pushnil(L);
    lua_rawset(L, -(3 as std::ffi::c_int));
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_uniq_del_ptr(
    mut L: *mut lua_State,
    mut reg: *const gchar,
    mut key: gpointer,
) {
    lua_pushlightuserdata(L, key);
    luaH_uniq_del(L, reg, -(1 as std::ffi::c_int));
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
}
