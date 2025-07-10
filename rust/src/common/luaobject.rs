use gdk_sys::*;
use glib_sys::*;
use libc::getenv;
use mlua_sys::*;

use crate::clib::luakit::*;
use crate::clib::msg::*;
use crate::clib::soup::*;
use crate::clib::sqlite3::*;
use crate::clib::stylesheet::*;
use crate::clib::web_module::*;
use crate::clib::widget::*;
use crate::common::luaclass::luaH_typename;
use crate::common::luah::*;
use crate::common::luautil::*;
use crate::common::util::*;
use crate::common::*;
use crate::globalconf::*;
use crate::gtypes::*;
use crate::log::*;

use crate::{
    common::{
        luaclass::{
            lua_class_t, lua_object_t, luaH_checkudata, luaH_class_get,
            signal_h::{
                signal_add, signal_array_t, signal_destroy, signal_lookup, signal_remove, signal_t,
                signals_remove,
            },
        },
        lualib::luaH_dofunction,
        tokenize::{luakit_token_t, token_tostring},
        util::luaH_callerinfo,
    },
    gtypes::{gchar, gint, guint},
    log::*,
};

#[inline]
pub unsafe extern "C-unwind" fn luaH_object_ref_item(
    mut L: *mut lua_State,
    mut ud: gint,
    mut iud: gint,
) -> gpointer {
    // lua_getfenv(L, ud);
    let mut p: gpointer = luaH_object_incref(
        L,
        -(1 as std::ffi::c_int),
        if iud < 0 as std::ffi::c_int {
            iud - 1 as std::ffi::c_int
        } else {
            iud
        },
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    return p;
}
#[inline]
pub unsafe extern "C-unwind" fn luaH_object_unref_item(
    mut L: *mut lua_State,
    mut ud: gint,
    mut p: gpointer,
) {
    // lua_getfenv(L, ud);
    luaH_object_decref(L, -(1 as std::ffi::c_int), p);
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
}
#[inline]
pub unsafe extern "C-unwind" fn luaH_object_push_item(
    mut L: *mut lua_State,
    mut ud: gint,
    mut p: gpointer,
) -> gint {
    // lua_getfenv(L, ud);
    lua_pushlightuserdata(L, p);
    lua_rawget(L, -(2 as std::ffi::c_int));
    lua_remove(L, -(2 as std::ffi::c_int));
    return 1 as std::ffi::c_int;
}
#[inline]
pub unsafe extern "C-unwind" fn luaH_object_registry_push(mut L: *mut lua_State) {
    lua_pushlstring(
        L,
        b"luakit.object.registry\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 23]>())
            .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
            .wrapping_sub(1),
    );
    lua_rawget(L, -(10000 as std::ffi::c_int));
}
#[inline]
pub unsafe extern "C-unwind" fn luaH_object_push(mut L: *mut lua_State, mut p: gpointer) -> gint {
    luaH_object_registry_push(L);
    lua_pushlightuserdata(L, p);
    lua_rawget(L, -(2 as std::ffi::c_int));
    lua_remove(L, -(2 as std::ffi::c_int));
    return 1 as std::ffi::c_int;
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_object_setup(mut L: *mut lua_State) {
    lua_pushlstring(
        L,
        b"luakit.object.registry\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 23]>())
            .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
            .wrapping_sub(1),
    );
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_setmetatable(L, -(2 as std::ffi::c_int));
    lua_rawset(L, -(10000 as std::ffi::c_int));
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_object_incref(
    mut L: *mut lua_State,
    mut tud: gint,
    mut oud: gint,
) -> gpointer {
    let mut p: gpointer = lua_topointer(L, oud) as gpointer;
    if p.is_null() {
        lua_remove(L, oud);
        return std::ptr::null_mut();
    }
    lua_pushlightuserdata(L, p);
    lua_pushvalue(
        L,
        if oud < 0 as std::ffi::c_int {
            oud - 1 as std::ffi::c_int
        } else {
            oud
        },
    );
    lua_rawset(
        L,
        if tud < 0 as std::ffi::c_int {
            tud - 2 as std::ffi::c_int
        } else {
            tud
        },
    );
    lua_getmetatable(L, tud);
    lua_pushlightuserdata(L, p);
    lua_rawget(L, -(2 as std::ffi::c_int));
    let mut count: gint =
        (lua_tonumber(L, -(1 as std::ffi::c_int)) + 1 as std::ffi::c_int as lua_Number) as gint;
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    lua_pushlightuserdata(L, p);
    lua_pushinteger(L, count as lua_Integer);
    lua_rawset(L, -(3 as std::ffi::c_int));
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    lua_remove(L, oud);
    return p;
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_object_decref(
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
    let mut count: gint =
        (lua_tonumber(L, -(1 as std::ffi::c_int)) - 1 as std::ffi::c_int as lua_Number) as gint;
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
            if tud < 0 as std::ffi::c_int {
                tud - 2 as std::ffi::c_int
            } else {
                tud
            },
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_settype(
    mut L: *mut lua_State,
    mut lua_class: *mut lua_class_t,
) -> gint {
    lua_pushlightuserdata(L, lua_class as *mut std::ffi::c_void);
    lua_rawget(L, -(10000 as std::ffi::c_int));
    lua_setmetatable(L, -(2 as std::ffi::c_int));
    return 1 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_object_add_signal(
    mut L: *mut lua_State,
    mut oud: gint,
    mut name: *const gchar,
    mut ud: gint,
) {
    if !(lua_type(L, ud) == 6 as std::ffi::c_int) {
        luaL_argerror(L, ud, b"function\0" as *const u8 as *const std::ffi::c_char);
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
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_object_remove_signal(
    mut L: *mut lua_State,
    mut oud: gint,
    mut name: *const gchar,
    mut ud: gint,
) {
    if !(lua_type(L, ud) == 6 as std::ffi::c_int) {
        luaL_argerror(L, ud, b"function\0" as *const u8 as *const std::ffi::c_char);
    }
    let mut obj: *mut lua_object_t = lua_touserdata(L, oud) as *mut lua_object_t;
    if obj.is_null() {
        _log(
            LOG_LEVEL_warn,
            b"common/luaobject.c\0" as *const u8 as *const std::ffi::c_char,
            b"object remove signal on non object\0" as *const u8 as *const std::ffi::c_char,
        );
        return;
    }
    let mut ref_0: gpointer = lua_topointer(L, ud) as gpointer;
    signal_remove((*obj).signals, name, ref_0);
    luaH_object_unref_item(L, oud, ref_0);
    lua_remove(L, ud);
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_object_remove_signals(
    mut L: *mut lua_State,
    mut oud: gint,
    mut name: *const gchar,
) {
    let mut obj: *mut lua_object_t = lua_touserdata(L, oud) as *mut lua_object_t;
    if obj.is_null() {
        _log(
            LOG_LEVEL_warn,
            b"common/luaobject.c\0" as *const u8 as *const std::ffi::c_char,
            b"object remove signals on non object\0" as *const u8 as *const std::ffi::c_char,
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
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn signal_array_emit(
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
        b"emit \x1B[34m\"%s\"\x1B[0m on %p from \x1B[32m%s\x1B[0m (%d args, %d nret)\0" as *const u8
            as *const std::ffi::c_char,
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
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn signal_object_emit(
    mut L: *mut lua_State,
    mut signals: *mut signal_t,
    mut name: *const gchar,
    mut nargs: gint,
    mut nret: gint,
) -> gint {
    return signal_array_emit(L, signals, name, name, nargs, nret);
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_object_emit_signal(
    mut L: *mut lua_State,
    mut oud: gint,
    mut name: *const gchar,
    mut nargs: gint,
    mut nret: gint,
) -> gint {
    let mut ret: gint = 0;
    let mut top: gint = 0;
    let mut bot: gint = lua_gettop(L) - nargs + 1 as std::ffi::c_int;
    let mut oud_abs: gint = lua_absindex(L, oud);
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
        b"emit \x1B[34m\"%s\"\x1B[0m on %p from \x1B[32m%s\x1B[0m (%d args, %d nret)\0" as *const u8
            as *const std::ffi::c_char,
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
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_object_property_signal(
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
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_object_add_signal_simple(mut L: *mut lua_State) -> gint {
    luaH_object_add_signal(
        L,
        1 as std::ffi::c_int,
        luaL_checklstring(L, 2 as std::ffi::c_int, std::ptr::null_mut()),
        3 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_object_remove_signal_simple(mut L: *mut lua_State) -> gint {
    luaH_object_remove_signal(
        L,
        1 as std::ffi::c_int,
        luaL_checklstring(L, 2 as std::ffi::c_int, std::ptr::null_mut()),
        3 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_object_remove_signals_simple(mut L: *mut lua_State) -> gint {
    luaH_object_remove_signals(
        L,
        1 as std::ffi::c_int,
        luaL_checklstring(L, 2 as std::ffi::c_int, std::ptr::null_mut()),
    );
    return 0 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_object_collect_signal_keys(
    mut key: gpointer,
    mut UNUSED_value: gpointer,
    mut keys: *mut GPtrArray,
) -> gboolean {
    g_ptr_array_add(keys, key);
    return 0 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_object_remove_all_signals(mut signals: *mut signal_t) -> gint {
    if !signals.is_null() {
        let mut L: *mut lua_State = common.L;
        let mut keys: *mut GPtrArray = g_ptr_array_new();
        g_tree_foreach(
            signals,
            ::core::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(gpointer, gpointer, *mut GPtrArray) -> gboolean>,
                GTraverseFunc,
            >(Some(
                luaH_object_collect_signal_keys
                    as unsafe extern "C-unwind" fn(gpointer, gpointer, *mut GPtrArray) -> gboolean,
            )),
            keys as gpointer,
        );
        let mut i: guint = 0 as std::ffi::c_int as guint;
        while i < (*keys).len {
            let mut type_0: *mut std::ffi::c_char =
                *((*keys).pdata).offset(i as isize) as *mut std::ffi::c_char;
            lua_pushstring(L, type_0);
            luaH_object_remove_signals_simple(L);
            i = i.wrapping_add(1);
            i;
        }
        g_ptr_array_free(keys, 0 as std::ffi::c_int);
    }
    return 0 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_object_emit_signal_simple(mut L: *mut lua_State) -> gint {
    return luaH_object_emit_signal(
        L,
        1 as std::ffi::c_int,
        luaL_checklstring(L, 2 as std::ffi::c_int, std::ptr::null_mut()),
        lua_gettop(L) - 2 as std::ffi::c_int,
        -(1 as std::ffi::c_int),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_object_tostring(mut L: *mut lua_State) -> gint {
    let mut lua_class: *mut lua_class_t = luaH_class_get(L, 1 as std::ffi::c_int);
    lua_pushfstring(
        L,
        b"%s: %p\0" as *const u8 as *const std::ffi::c_char,
        (*lua_class).name,
        luaH_checkudata(L, 1 as std::ffi::c_int, lua_class),
    );
    return 1 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_object_gc(mut L: *mut lua_State) -> gint {
    let mut item: *mut lua_object_t = lua_touserdata(L, 1 as std::ffi::c_int) as *mut lua_object_t;
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

pub unsafe extern "C" fn luaH_object_ref_class(
    mut L: *mut lua_State,
    mut oud: gint,
    mut class: *mut lua_class_t,
) -> gpointer {
    luaH_checkudata(L, oud, class);
    return luaH_object_ref(L, oud);
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
