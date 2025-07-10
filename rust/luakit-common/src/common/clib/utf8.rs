use glib_sys::{
    g_utf8_find_next_char, g_utf8_get_char_validated, g_utf8_offset_to_pointer, g_utf8_strlen,
    g_utf8_validate,
};
use libc::{size_t, ssize_t};
use mlua_sys::{
    lua_Integer, lua_State, lua_getfield, lua_pushinteger, lua_pushnil, lua_pushstring,
    lua_setfield, lua_settop, luaL_Reg, luaL_argerror, luaL_checkinteger, luaL_checklstring,
    luaL_error, luaL_optinteger,
};

use crate::common::common;
use crate::common::luaclass::luaH_openlib;

use crate::gtypes::{gchar, gint};

unsafe extern "C" fn abspos(mut offset: i64, mut length: size_t) -> isize {
    if offset == 0 {
        return -1;
    }
    let length = length as i64;
    offset = if offset > 0 {
        offset - 1
    } else {
        offset + length
    };
    if offset < 0 || offset > length {
        return -1;
    }
    return offset as isize;
}
unsafe extern "C-unwind" fn luaH_utf8_len(mut L: *mut lua_State) -> gint {
    let mut blen: usize = 0;
    let mut str: *const gchar = luaL_checklstring(L, 1 as std::ffi::c_int, &mut blen);
    let mut bbeg = abspos(luaL_optinteger(L, 2, 1), blen);
    (bbeg != -1
        || luaL_argerror(
            L,
            2 as std::ffi::c_int,
            b"initial position out of string\0" as *const u8 as *const std::ffi::c_char,
        ) != 0) as std::ffi::c_int;
    let mut bend = bbeg;
    let mut sbend: lua_Integer = luaL_optinteger(L, 3, blen as lua_Integer);
    sbend = if sbend >= 0 {
        sbend - 1
    } else {
        sbend + blen as lua_Integer
    };
    (sbend < blen as lua_Integer
        || luaL_argerror(
            L,
            3,
            b"final position out of string\0" as *const u8 as *const std::ffi::c_char,
        ) != 0) as std::ffi::c_int;
    if sbend >= bbeg as lua_Integer && sbend < blen as lua_Integer {
        bend =
            (g_utf8_find_next_char(str.offset(sbend as isize), std::ptr::null())).offset_from(str);
    }
    let mut valend: *mut gchar = 0 as *mut gchar;
    if g_utf8_validate(
        str.offset(bbeg as isize) as *const u8,
        bend.wrapping_sub(bbeg),
        &mut valend as *mut *mut gchar as *mut *const gchar,
    ) == 0
    {
        lua_pushnil(L);
        lua_pushinteger(L, valend.offset_from(str) as lua_Integer);
        return 2 as std::ffi::c_int;
    }
    lua_pushinteger(
        L,
        g_utf8_strlen(
            str.offset(bbeg as isize),
            bend.wrapping_sub(bbeg) as ssize_t,
        ),
    );
    return 1 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_utf8_offset(mut L: *mut lua_State) -> gint {
    let mut blen: size_t = 0;
    let mut str: *const gchar = luaL_checklstring(L, 1, &mut blen);
    let mut widx: lua_Integer = luaL_checkinteger(L, 2);
    if widx > 0 {
        widx -= 1;
        widx;
    }
    let mut bbase = 0;
    bbase = luaL_optinteger(
        L,
        3,
        (if widx >= 0 {
            1
        } else {
            blen.wrapping_add(1 as std::ffi::c_int as size_t)
        }) as lua_Integer,
    );
    bbase = abspos(bbase, blen) as lua_Integer;
    (bbase != -1
        || luaL_argerror(
            L,
            3,
            b"position out of range\0" as *const u8 as *const std::ffi::c_char,
        ) != 0) as std::ffi::c_int;
    if g_utf8_get_char_validated(str.offset(bbase as isize), ssize_t::MAX) == u32::MAX {
        luaL_error(
            L,
            b"initial position is a continuation byte\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    let mut wseglen = 0;
    let mut bbeg = 0;
    if widx < 0 {
        wseglen = g_utf8_strlen(str, bbase as ssize_t);
        widx = widx.wrapping_add(wseglen as lua_Integer);
    } else {
        wseglen = g_utf8_strlen(
            str.offset(bbase as isize),
            blen.wrapping_sub(bbase as usize) as ssize_t,
        );
        bbeg = bbase;
    }
    let mut ret: ssize_t = 0;
    if widx >= 0 && widx <= wseglen {
        let mut pos: *mut gchar = g_utf8_offset_to_pointer(str.offset(bbeg as isize), widx);
        if !pos.is_null() {
            ret = pos.offset_from(str) + 1;
        }
    }
    if ret > 0 as std::ffi::c_int as ssize_t {
        lua_pushinteger(L, ret as lua_Integer);
    } else {
        lua_pushnil(L);
    }
    return 1 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn utf8_lib_setup(mut L: *mut lua_State) {
    static mut utf8_lib: [luaL_Reg; 2] = unsafe {
        [
            {
                let mut init = luaL_Reg {
                    name: b"len\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_utf8_len,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"offset\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_utf8_offset,
                };
                init
            },
            // {
            //     let mut init = luaL_Reg {
            //         name: 0 as *const std::ffi::c_char,
            //         func: None,
            //     };
            //     init
            // },
        ]
    };
    luaH_openlib(
        L,
        b"utf8\0" as *const u8 as *const std::ffi::c_char,
        utf8_lib.as_ptr(),
        utf8_lib.as_ptr(),
    );
    lua_getfield(
        L,
        -(10002 as std::ffi::c_int),
        b"utf8\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_pushstring(
        L,
        b"[%z\x01-\x7F\xC2-\xF4][\x80-\xBF]*\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_setfield(
        L,
        -(2 as std::ffi::c_int),
        b"charpattern\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
}
