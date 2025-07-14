use std::mem::MaybeUninit;

use glib_sys::{
    GByteArray, g_assertion_message_cmpint, g_byte_array_append, g_byte_array_new,
    g_byte_array_set_size, gpointer,
};
use libc::{c_int, c_void, memcpy, size_t};
use mlua::ffi::{
    lua_Debug, lua_Number, lua_Reader, lua_State, lua_createtable, lua_dump, lua_getinfo,
    lua_gettop, lua_getupvalue, lua_load, lua_next, lua_pushboolean, lua_pushlightuserdata,
    lua_pushlstring, lua_pushnil, lua_pushnumber, lua_pushvalue, lua_rawset, lua_settop,
    lua_setupvalue, lua_toboolean, lua_tolstring, lua_tonumber, lua_touserdata, lua_type,
    lua_typename, luaL_error,
};

use crate::common::lualib::lualib_h::luaH_absindex;
use crate::gtypes::{gchar, gint64, guint, guint8, guint64};
use crate::log::{_log, LOG_LEVEL_warn};

static mut bytecode_buf: *mut GByteArray = 0 as *const GByteArray as *mut GByteArray;
static mut bytecode_len: size_t = 0;
unsafe extern "C-unwind" fn lua_function_writer(
    mut UNUSED_L: *mut lua_State,
    mut p: *const std::ffi::c_void,
    mut sz: size_t,
    mut UNUSED_ud: *mut std::ffi::c_void,
) -> std::ffi::c_int {
    g_byte_array_append(bytecode_buf, p as *mut guint8, sz as guint);
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn lua_function_reader(
    mut UNUSED_L: *mut lua_State,
    mut bytes: *mut *const guint8,
    mut sz: *mut size_t,
) -> *const std::ffi::c_char {
    if bytecode_len == 0 as std::ffi::c_int as size_t {
        return 0 as *const std::ffi::c_char;
    }
    let mut ret: *const std::ffi::c_char = *bytes as *const std::ffi::c_char;
    *bytes = (*bytes).offset(bytecode_len as isize);
    *sz = bytecode_len;
    return ret;
}
unsafe extern "C" fn lua_serialize_value(
    mut L: *mut lua_State,
    mut out: *mut GByteArray,
    mut index: std::ffi::c_int,
) {
    let mut type_0: c_int = lua_type(L, index) as c_int;
    let mut top: std::ffi::c_int = lua_gettop(L);
    match type_0 as std::ffi::c_int {
        7 | 8 => {
            luaL_error(
                L,
                b"cannot serialize variable of type %s\0" as *const u8 as *const std::ffi::c_char,
                lua_typename(L, type_0 as std::ffi::c_int),
            );
            return;
        }
        _ => {}
    }
    g_byte_array_append(
        out,
        &mut type_0 as *mut c_int as *mut guint8,
        ::core::mem::size_of::<c_int>() as std::ffi::c_ulong as guint,
    );
    match type_0 as std::ffi::c_int {
        3 => {
            let mut n: lua_Number = lua_tonumber(L, index);
            g_byte_array_append(
                out,
                &mut n as *mut lua_Number as *mut guint8,
                ::core::mem::size_of::<lua_Number>() as std::ffi::c_ulong as guint,
            );
        }
        1 => {
            let mut b: c_int = lua_toboolean(L, index) as c_int;
            g_byte_array_append(
                out,
                &mut b as *mut c_int as *mut guint8,
                ::core::mem::size_of::<c_int>() as std::ffi::c_ulong as guint,
            );
        }
        4 => {
            let mut len: size_t = 0;
            let mut s: *const std::ffi::c_char = lua_tolstring(L, index, &mut len);
            g_byte_array_append(
                out,
                &mut len as *mut size_t as *mut guint8,
                ::core::mem::size_of::<size_t>() as std::ffi::c_ulong as guint,
            );
            g_byte_array_append(
                out,
                s as *mut guint8,
                len.wrapping_add(1 as std::ffi::c_int as size_t) as guint,
            );
        }
        5 => {
            index = if index > 0 as std::ffi::c_int {
                index
            } else {
                lua_gettop(L) + 1 as std::ffi::c_int + index
            };
            lua_pushnil(L);
            while lua_next(L, index) != 0 as std::ffi::c_int {
                lua_serialize_value(L, out, -(2 as std::ffi::c_int));
                lua_serialize_value(L, out, -(1 as std::ffi::c_int));
                lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
            }
            let mut end: c_int = -(1 as std::ffi::c_int) as c_int;
            g_byte_array_append(
                out,
                &mut end as *mut c_int as *mut guint8,
                ::core::mem::size_of::<c_int>() as std::ffi::c_ulong as guint,
            );
        }
        2 => {
            let mut p: gpointer = lua_touserdata(L, index);
            if p.is_null() {
                _log(
                    LOG_LEVEL_warn,
                    b"common/luaserialize.c\0" as *const u8 as *const std::ffi::c_char,
                    "serialize lua lightuserdata on non object",
                );
            } else {
                g_byte_array_append(
                    out,
                    &mut p as *mut gpointer as *mut guint8,
                    ::core::mem::size_of::<gpointer>() as std::ffi::c_ulong as guint,
                );
            }
        }
        6 => {
            bytecode_buf = if !bytecode_buf.is_null() {
                bytecode_buf
            } else {
                g_byte_array_new()
            };
            g_byte_array_set_size(bytecode_buf, 0 as std::ffi::c_int as guint);
            lua_pushvalue(L, index);
            lua_dump(L, lua_function_writer, std::ptr::null_mut(), 0);
            lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
            let mut len_0: size_t = (*bytecode_buf).len as size_t;
            g_byte_array_append(
                out,
                &mut len_0 as *mut size_t as *mut guint8,
                ::core::mem::size_of::<size_t>() as std::ffi::c_ulong as guint,
            );
            g_byte_array_append(out, (*bytecode_buf).data, len_0 as guint);
            g_byte_array_set_size(bytecode_buf, 0 as std::ffi::c_int as guint);
            let mut ar = MaybeUninit::<lua_Debug>::uninit();
            lua_pushvalue(L, index);
            lua_getinfo(
                L,
                b">u\0" as *const u8 as *const std::ffi::c_char,
                ar.as_mut_ptr(),
            );
            g_byte_array_append(
                out,
                (*ar.as_mut_ptr()).nups as *mut std::ffi::c_int as *mut guint8,
                ::core::mem::size_of::<std::ffi::c_int>() as std::ffi::c_ulong as guint,
            );
            let mut i = 1;
            while i <= (*ar.as_ptr()).nups {
                lua_getupvalue(L, -(1 as std::ffi::c_int), i as std::ffi::c_int);
                lua_serialize_value(L, out, -(1 as std::ffi::c_int));
                lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
                i += 1;
                i;
            }
        }
        0 | _ => {}
    }
    let mut __n1: gint64 = lua_gettop(L) as gint64;
    let mut __n2: gint64 = top as gint64;
    if !(__n1 == __n2) {
        /*
        g_assertion_message_cmpint(
            0 as *mut gchar,
            b"common/luaserialize.c\0" as *const u8 as *const std::ffi::c_char,
            130 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 20], &[std::ffi::c_char; 20]>(
                b"lua_serialize_value\0",
            ))
            .as_ptr(),
            b"lua_gettop(L) == top\0" as *const u8 as *const std::ffi::c_char,
            __n1 as guint64,
            b"==\0" as *const u8 as *const std::ffi::c_char,
            __n2 as guint64,
            'i' as i32 as std::ffi::c_char,
        );
        */
    }
}
unsafe extern "C" fn lua_deserialize_value(
    mut L: *mut lua_State,
    mut bytes: *mut *const guint8,
) -> std::ffi::c_int {
    let mut type_0: c_int = 0;
    memcpy(
        &mut type_0 as *mut c_int as *mut std::ffi::c_void,
        *bytes as *const std::ffi::c_void,
        ::core::mem::size_of::<c_int>(),
    );
    *bytes = (*bytes).offset(::core::mem::size_of::<c_int>() as std::ffi::c_ulong as isize);
    let mut top: std::ffi::c_int = lua_gettop(L);
    match type_0 as std::ffi::c_int {
        0 => {
            lua_pushnil(L);
        }
        3 => {
            let mut n: lua_Number = 0.;
            memcpy(
                &mut n as *mut lua_Number as *mut std::ffi::c_void,
                *bytes as *const std::ffi::c_void,
                ::core::mem::size_of::<lua_Number>(),
            );
            *bytes =
                (*bytes).offset(::core::mem::size_of::<lua_Number>() as std::ffi::c_ulong as isize);
            lua_pushnumber(L, n);
        }
        1 => {
            let mut b: c_int = 0;
            memcpy(
                &mut b as *mut c_int as *mut std::ffi::c_void,
                *bytes as *const std::ffi::c_void,
                ::core::mem::size_of::<c_int>(),
            );
            *bytes = (*bytes).offset(::core::mem::size_of::<c_int>() as std::ffi::c_ulong as isize);
            lua_pushboolean(L, b as std::ffi::c_int);
        }
        4 => {
            let mut len: size_t = 0;
            memcpy(
                &mut len as *mut size_t as *mut std::ffi::c_void,
                *bytes as *const std::ffi::c_void,
                ::core::mem::size_of::<size_t>(),
            );
            *bytes =
                (*bytes).offset(::core::mem::size_of::<size_t>() as std::ffi::c_ulong as isize);
            lua_pushlstring(L, *bytes as *mut std::ffi::c_char, len);
            *bytes = (*bytes).offset(len.wrapping_add(1 as std::ffi::c_int as size_t) as isize);
        }
        5 => {
            lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
            while lua_deserialize_value(L, bytes) == 1 as std::ffi::c_int {
                lua_deserialize_value(L, bytes);
                lua_rawset(L, -(3 as std::ffi::c_int));
            }
        }
        2 => {
            let mut p: gpointer = 0 as *mut std::ffi::c_void;
            memcpy(
                &mut p as *mut gpointer as *mut std::ffi::c_void,
                *bytes as *const std::ffi::c_void,
                ::core::mem::size_of::<gpointer>(),
            );
            *bytes =
                (*bytes).offset(::core::mem::size_of::<gpointer>() as std::ffi::c_ulong as isize);
            lua_pushlightuserdata(L, p);
        }
        6 => {
            memcpy(
                &mut bytecode_len as *mut size_t as *mut std::ffi::c_void,
                *bytes as *const std::ffi::c_void,
                ::core::mem::size_of::<size_t>(),
            );
            *bytes =
                (*bytes).offset(::core::mem::size_of::<size_t>() as std::ffi::c_ulong as isize);
            let mut status: std::ffi::c_int = lua_load(
                L,
                ::core::mem::transmute::<
                    Option<
                        unsafe extern "C" fn(
                            *mut lua_State,
                            *mut *const guint8,
                            *mut size_t,
                        ) -> *const std::ffi::c_char,
                    >,
                    lua_Reader,
                >(Some(
                    lua_function_reader
                        as unsafe extern "C" fn(
                            *mut lua_State,
                            *mut *const guint8,
                            *mut size_t,
                        ) -> *const std::ffi::c_char,
                )),
                bytes as *mut std::ffi::c_void,
                std::ptr::null_mut(),
                //std::ptr::null_mut(),
            );
            if status != 0 as std::ffi::c_int {
                return luaL_error(
                    L,
                    b"deserialize error: %s\0" as *const u8 as *const std::ffi::c_char,
                    lua_tolstring(L, -(1 as std::ffi::c_int), 0 as *mut size_t),
                );
            }
            let mut nups: std::ffi::c_int = 0;
            memcpy(
                &mut nups as *mut std::ffi::c_int as *mut std::ffi::c_void,
                *bytes as *const std::ffi::c_void,
                ::core::mem::size_of::<std::ffi::c_int>(),
            );
            *bytes = (*bytes)
                .offset(::core::mem::size_of::<std::ffi::c_int>() as std::ffi::c_ulong as isize);
            let mut i: std::ffi::c_int = 1 as std::ffi::c_int;
            while i <= nups {
                lua_deserialize_value(L, bytes);
                lua_setupvalue(L, -(2 as std::ffi::c_int), i);
                i += 1;
                i;
            }
        }
        -1 => return 0 as std::ffi::c_int,
        _ => {}
    }
    let mut __n1: gint64 = lua_gettop(L) as gint64;
    let mut __n2: gint64 = (top + 1 as std::ffi::c_int) as gint64;
    if !(__n1 == __n2) {
        g_assertion_message_cmpint(
            0 as *mut gchar,
            b"common/luaserialize.c\0" as *const u8 as *const std::ffi::c_char,
            202 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 22], &[std::ffi::c_char; 22]>(
                b"lua_deserialize_value\0",
            ))
            .as_ptr(),
            b"lua_gettop(L) == top + 1\0" as *const u8 as *const std::ffi::c_char,
            __n1 as guint64,
            b"==\0" as *const u8 as *const std::ffi::c_char,
            __n2 as guint64,
            'i' as i32 as std::ffi::c_char,
        );
    }
    return 1 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_serialize_range(
    mut L: *mut lua_State,
    mut out: *mut GByteArray,
    mut start: std::ffi::c_int,
    mut end: std::ffi::c_int,
) {
    start = luaH_absindex(L, start);
    end = luaH_absindex(L, end);
    let mut i: std::ffi::c_int = start;
    while i <= end {
        lua_serialize_value(L, out, i);
        i += 1;
        i;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_deserialize_range(
    mut L: *mut lua_State,
    mut in_0: *const guint8,
    mut length: guint,
) -> std::ffi::c_int {
    let mut bytes: *const guint8 = in_0;
    let mut i: std::ffi::c_int = 0 as std::ffi::c_int;
    while bytes < in_0.offset(length as isize) {
        lua_deserialize_value(L, &mut bytes);
        i += 1;
        i;
    }
    return i;
}
