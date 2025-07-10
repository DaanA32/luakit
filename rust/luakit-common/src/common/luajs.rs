use glib_sys::{GType, g_strfreev};
use gobject_sys::{GObject, g_object_unref};
use javascriptcore_rs_sys::{
    JSCClass, JSCContext, JSCException, JSCValue, jsc_context_evaluate_with_source_uri,
    jsc_context_get_exception, jsc_exception_to_string, jsc_value_is_boolean, jsc_value_is_null,
    jsc_value_is_number, jsc_value_is_object, jsc_value_is_string, jsc_value_is_undefined,
    jsc_value_new_array, jsc_value_new_boolean, jsc_value_new_null, jsc_value_new_number,
    jsc_value_new_object, jsc_value_new_string, jsc_value_new_undefined,
    jsc_value_object_enumerate_properties, jsc_value_object_get_property,
    jsc_value_object_set_property, jsc_value_object_set_property_at_index, jsc_value_to_boolean,
    jsc_value_to_double, jsc_value_to_string,
};
use libc::{c_void, free, size_t, strtol};
use mlua_sys::{
    lua_State, lua_createtable, lua_gettop, lua_next, lua_objlen, lua_pushboolean, lua_pushinteger,
    lua_pushnil, lua_pushnumber, lua_pushstring, lua_rawset, lua_settop, lua_toboolean,
    lua_tolstring, lua_tonumber, lua_type,
};

use crate::gtypes::guint;

pub unsafe extern "C" fn luajs_tovalue(
    mut L: *mut lua_State,
    mut idx: std::ffi::c_int,
    mut ctx: *mut JSCContext,
) -> *mut JSCValue {
    match lua_type(L, idx) {
        1 => return jsc_value_new_boolean(ctx, lua_toboolean(L, idx)),
        3 => return jsc_value_new_number(ctx, lua_tonumber(L, idx)),
        0 => return jsc_value_new_null(ctx),
        -1 => return jsc_value_new_undefined(ctx),
        4 => return jsc_value_new_string(ctx, lua_tolstring(L, idx, 0 as *mut size_t)),
        5 => {
            let mut len: size_t = lua_objlen(L, idx);
            let mut top: std::ffi::c_int = lua_gettop(L);
            let mut res: *mut JSCValue = 0 as *mut JSCValue;
            let mut val: *mut JSCValue = 0 as *mut JSCValue;
            if idx < 0 as std::ffi::c_int {
                idx += top + 1 as std::ffi::c_int;
            }
            if len != 0 {
                res = jsc_value_new_array(
                    ctx,
                    ((1 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
                );
                lua_pushnil(L);
                let mut i: std::ffi::c_int = 0 as std::ffi::c_int;
                while lua_next(L, idx) != 0 {
                    val = luajs_tovalue(L, -(1 as std::ffi::c_int), ctx);
                    if val.is_null() {
                        lua_settop(L, top);
                        g_object_unref(res as *mut GObject);
                        return std::ptr::null_mut();
                    }
                    let fresh0 = i;
                    i = i + 1;
                    jsc_value_object_set_property_at_index(res, fresh0 as guint, val);
                    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
                    g_object_unref(val as *mut GObject);
                }
            } else {
                res = jsc_value_new_object(ctx, 0 as *mut std::ffi::c_void, 0 as *mut JSCClass);
                lua_pushnil(L);
                while lua_next(L, idx) != 0 {
                    if lua_type(L, -(2 as std::ffi::c_int)) != 4 as std::ffi::c_int {
                        continue;
                    }
                    val = luajs_tovalue(L, -(1 as std::ffi::c_int), ctx);
                    if val.is_null() {
                        lua_settop(L, top);
                        g_object_unref(res as *mut GObject);
                        return 0 as *mut JSCValue;
                    }
                    jsc_value_object_set_property(
                        res,
                        lua_tolstring(L, -(2 as std::ffi::c_int), 0 as *mut size_t),
                        val,
                    );
                    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
                    g_object_unref(val as *mut GObject);
                }
            }
            return res;
        }
        _ => {}
    }
    return 0 as *mut JSCValue;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luajs_pushvalue(
    mut L: *mut lua_State,
    mut value: *mut JSCValue,
) -> std::ffi::c_int {
    if jsc_value_is_undefined(value) != 0 || jsc_value_is_null(value) != 0 {
        lua_pushnil(L);
    } else if jsc_value_is_boolean(value) != 0 {
        lua_pushboolean(L, jsc_value_to_boolean(value));
    } else if jsc_value_is_number(value) != 0 {
        lua_pushnumber(L, jsc_value_to_double(value));
    } else if jsc_value_is_string(value) != 0 {
        let mut str: *mut std::ffi::c_char = jsc_value_to_string(value);
        lua_pushstring(L, str);
        free(str as *mut std::ffi::c_void);
    } else if jsc_value_is_object(value) != 0 {
        let mut keys: *mut *mut std::ffi::c_char = jsc_value_object_enumerate_properties(value);
        let mut top: std::ffi::c_int = lua_gettop(L);
        let mut val: *mut JSCValue = 0 as *mut JSCValue;
        let mut eptr: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
        let mut key: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
        let mut i: std::ffi::c_int = 0 as std::ffi::c_int;
        let mut n: std::ffi::c_long = 0;
        lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
        while !keys.is_null() && {
            let fresh1 = i;
            i = i + 1;
            key = *keys.offset(fresh1 as isize);
            !key.is_null()
        } {
            if *key as std::ffi::c_int != 0 && {
                n = strtol(key, &mut eptr, 10 as std::ffi::c_int);
                *eptr == 0
            } {
                n += 1;
                lua_pushinteger(L, n);
            } else {
                lua_pushstring(L, key);
            }
            val = jsc_value_object_get_property(value, key);
            if luajs_pushvalue(L, val) == 0 {
                g_object_unref(val as *mut GObject);
                lua_settop(L, top);
                g_strfreev(keys);
                return 0 as std::ffi::c_int;
            }
            g_object_unref(val as *mut GObject);
            lua_rawset(L, -(3 as std::ffi::c_int));
        }
        g_strfreev(keys);
    } else {
        return 0 as std::ffi::c_int;
    }
    return 1 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luajs_eval_js(
    mut L: *mut lua_State,
    mut ctx: *mut JSCContext,
    mut code: *const std::ffi::c_char,
    mut source: *const std::ffi::c_char,
    mut line: guint,
    mut no_return: bool,
) -> std::ffi::c_int {
    let mut result: *mut JSCValue =
        jsc_context_evaluate_with_source_uri(ctx, code, -1, source, line);
    let mut exception: *mut JSCException = jsc_context_get_exception(ctx);
    if !exception.is_null() {
        let mut e: *mut std::ffi::c_char = jsc_exception_to_string(exception);
        lua_pushnil(L);
        lua_pushstring(L, e);
        free(e as *mut std::ffi::c_void);
        return 2 as std::ffi::c_int;
    }
    if no_return {
        return 0 as std::ffi::c_int;
    }
    let mut ret: std::ffi::c_int = luajs_pushvalue(L, result);
    g_object_unref(result as *mut GObject);
    if ret == 0 {
        lua_pushnil(L);
        lua_pushstring(
            L,
            b"unable to push the result onto the Lua stack\0" as *const u8
                as *const std::ffi::c_char,
        );
        return 2 as std::ffi::c_int;
    }
    return ret;
}
