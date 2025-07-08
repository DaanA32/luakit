use glib_sys::{
    g_free, g_idle_add, g_idle_remove_by_data, g_uri_escape_string, g_uri_unescape_string,
    gboolean, gpointer,
};
use libc::{gettimeofday, size_t, timeval, timezone};
use lua::ffi::{
    lua_State, lua_gettop, lua_pushboolean, lua_pushnumber, lua_pushstring, lua_settop,
    lua_toboolean, lua_topointer, lua_type, luaL_checklstring,
};

use crate::{
    common::{
        luaobject::{luaH_object_decref, luaH_object_incref, luaH_object_registry_push},
        *,
    },
    gtypes::{gchar, gdouble, gint},
    luah::luaH_dofunction,
};

#[inline]
pub unsafe extern "C" fn l_time() -> gdouble {
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    gettimeofday(&mut tv, 0 as *mut timezone);
    return tv.tv_sec as std::ffi::c_double + tv.tv_usec as std::ffi::c_double / 1e6f64;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_luakit_time(mut L: *mut lua_State) -> gint {
    lua_pushnumber(L, l_time());
    return 1 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_luakit_uri_encode(mut L: *mut lua_State) -> gint {
    let mut string: *const gchar = luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t);
    let mut allowed: *const gchar = 0 as *const gchar;
    if (1 as std::ffi::c_int) < lua_gettop(L)
        && !(lua_type(L, 2 as std::ffi::c_int) == 0 as std::ffi::c_int)
    {
        allowed = luaL_checklstring(L, 2 as std::ffi::c_int, 0 as *mut size_t);
    }
    let mut res: *mut gchar = g_uri_escape_string(string, allowed, 1 as std::ffi::c_int);
    lua_pushstring(L, res);
    g_free(res as gpointer);
    return 1 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_luakit_uri_decode(mut L: *mut lua_State) -> gint {
    let mut string: *const gchar = luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t);
    let mut illegal: *const gchar = 0 as *const gchar;
    if (1 as std::ffi::c_int) < lua_gettop(L)
        && !(lua_type(L, 2 as std::ffi::c_int) == 0 as std::ffi::c_int)
    {
        illegal = luaL_checklstring(L, 2 as std::ffi::c_int, 0 as *mut size_t);
    }
    let mut res: *mut gchar = g_uri_unescape_string(string, illegal);
    if res.is_null() {
        return 0 as std::ffi::c_int;
    }
    lua_pushstring(L, res);
    g_free(res as gpointer);
    return 1 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn idle_cb(mut func: gpointer) -> gboolean {
    let mut L: *mut lua_State = common.L;
    let mut top: gint = lua_gettop(L);
    luaH_object_push(L, func);
    let mut ok: gboolean = luaH_dofunction(L, 0 as std::ffi::c_int, 1 as std::ffi::c_int);
    let mut keep: gboolean = lua_toboolean(L, -(1 as std::ffi::c_int));
    if keep == 0 || ok == 0 {
        luaH_object_unref(L, func);
    }
    lua_settop(L, top);
    return (keep != 0 && ok != 0) as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_luakit_idle_add(mut L: *mut lua_State) -> gint {
    if !(lua_type(L, 1 as std::ffi::c_int) == 6 as std::ffi::c_int) {
        luaL_argerror(
            L,
            1 as std::ffi::c_int,
            b"function\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    let mut func: gpointer = luaH_object_ref(L, 1 as std::ffi::c_int);
    g_idle_add(
        Some(idle_cb as unsafe extern "C" fn(gpointer) -> gboolean),
        func,
    );
    return 0 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_luakit_idle_remove(mut L: *mut lua_State) -> gint {
    if !(lua_type(L, 1 as std::ffi::c_int) == 6 as std::ffi::c_int) {
        luaL_argerror(
            L,
            1 as std::ffi::c_int,
            b"function\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    let mut func: gpointer = lua_topointer(L, 1 as std::ffi::c_int) as gpointer;
    lua_pushboolean(L, g_idle_remove_by_data(func));
    luaH_object_unref(L, func);
    return 1 as std::ffi::c_int;
}

pub unsafe extern "C" fn luaH_object_ref(mut L: *mut lua_State, mut oud: gint) -> gpointer {
    luaH_object_registry_push(L);
    let mut p: gpointer = luaH_object_incref(
        L,
        -(1 as std::ffi::c_int),
        if oud < 0 as std::ffi::c_int {
            oud - 1 as std::ffi::c_int
        } else {
            oud
        },
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    return p;
}

pub unsafe extern "C" fn luaH_object_unref(mut L: *mut lua_State, mut p: gpointer) {
    luaH_object_registry_push(L);
    luaH_object_decref(L, -(1 as std::ffi::c_int), p);
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
}
