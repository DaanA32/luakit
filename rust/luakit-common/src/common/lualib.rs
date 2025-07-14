use std::ffi::CStr;

use glib_sys::gboolean;
use libc::getenv;
use mlua_sys::{
    lua_Integer, lua_State, lua_gettop, lua_insert, lua_next, lua_objlen, lua_pcall,
    lua_pushcclosure, lua_pushnil, lua_pushvalue, lua_remove, lua_settop, lua_toboolean,
    lua_tointeger, lua_tolstring, lua_tonumber, lua_topointer, lua_type, lua_typename,
};

use crate::common::luaclass::luaH_typename;
use crate::common::luautil::luaH_dofunction_on_error;
use crate::gtypes::guint;

pub mod lualib_h {
    #[inline]
    pub unsafe extern "C" fn luaH_dump_table_keys(mut L: *mut lua_State, mut idx: gint) {
        let mut len: gint = lua_objlen(L, idx) as gint;
        let mut limit: guint = 5 as std::ffi::c_int as guint;
        let mut rem: guint = 0 as std::ffi::c_int as guint;
        eprintln!("  Keys: ");
        lua_pushvalue(L, idx);
        lua_pushnil(L);
        while lua_next(L, -(2 as std::ffi::c_int)) != 0 {
            if limit == 0 as std::ffi::c_int as guint {
                rem = rem.wrapping_add(1);
                rem;
            } else {
                limit = limit.wrapping_sub(1);
                limit;
                let mut key_type: gint = lua_type(L, -(2 as std::ffi::c_int));
                if key_type == 3 as std::ffi::c_int
                    && lua_tointeger(L, -(2 as std::ffi::c_int)) > len as lua_Integer
                {
                    eprint!("{:?}, ", lua_tointeger(L, -(2 as std::ffi::c_int)));
                } else if key_type == 4 as std::ffi::c_int {
                    eprint!(
                        "{:?}, ",
                        lua_tolstring(L, -(2 as std::ffi::c_int), std::ptr::null_mut())
                    );
                } else {
                    eprint!("[{:?}]", lua_typename(L, key_type),);
                }
            }
            lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        }
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        eprintln!("and {} more", rem,);
    }

    pub unsafe extern "C" fn luaH_absindex(mut L: *mut lua_State, mut ud: gint) -> gint {
        return if ud >= 0 as std::ffi::c_int || ud <= -(10000 as std::ffi::c_int) {
            ud
        } else {
            lua_gettop(L) + ud + 1 as std::ffi::c_int
        };
    }
    use glib_sys::*;
    use mlua_sys::*;

    use crate::gtypes::*;
}
use glib_sys::*;
use mlua_sys::*;

use crate::{
    gtypes::gint,
    log::{_log, LOG_LEVEL_error},
};

pub use self::lualib_h::luaH_dump_table_keys;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_dump_stack(mut L: *mut lua_State) {
    eprintln!("-------- Lua stack dump ---------",);
    let mut i: std::ffi::c_int = lua_gettop(L);
    while i != 0 {
        let mut t: std::ffi::c_int = lua_type(L, i);
        match t {
            4 => {
                eprintln!(
                    "{}: string: `{:?}`",
                    i as i32,
                    lua_tolstring(L, i, std::ptr::null_mut()),
                );
            }
            1 => {
                eprintln!(
                    "{}: bool:   {}",
                    i as i32,
                    if lua_toboolean(L, i) != 0 {
                        "true"
                    } else {
                        "false"
                    },
                );
            }
            3 => {
                eprintln!("{}: number: {}", i, lua_tonumber(L, i),);
            }
            0 => {
                eprintln!("{}: nil", i);
            }
            7 => {
                eprintln!(
                    "{}: <{:?}>\t\t{:?}",
                    i,
                    luaH_typename(L, i),
                    lua_topointer(L, i),
                );
            }
            5 => {
                eprintln!(
                    "{}: table\t#{:?}\t{:?}",
                    i,
                    lua_objlen(L, i),
                    lua_topointer(L, i),
                );
                luaH_dump_table_keys(L, i);
            }
            _ => {
                eprintln!(
                    "{:?}: {:?}\t#{:?}\t{:?}",
                    i,
                    lua_typename(L, t),
                    lua_objlen(L, i),
                    lua_topointer(L, i),
                );
            }
        }
        i -= 1;
        i;
    }
    eprintln!("------- Lua stack dump end ------",);
}

pub unsafe extern "C" fn luaH_dofunction(
    mut L: *mut lua_State,
    mut nargs: gint,
    mut nret: gint,
) -> gboolean {
    lua_insert(L, -nargs - 1 as std::ffi::c_int);
    lua_pushcclosure(L, luaH_dofunction_on_error, 0 as std::ffi::c_int);
    lua_insert(L, -nargs - 2 as std::ffi::c_int);
    let mut error_func_pos = lua_gettop(L) - nargs - 1 as std::ffi::c_int;
    if lua_pcall(L, nargs, nret, -nargs - 2 as std::ffi::c_int) != 0 {
        _log(
            LOG_LEVEL_error,
            b"./common/lualib.h\0" as *const u8 as *const std::ffi::c_char,
            &CStr::from_ptr(lua_tolstring(
                L,
                -(1 as std::ffi::c_int),
                std::ptr::null_mut(),
            ))
            .to_string_lossy(),
        );
        lua_settop(L, -(2 as std::ffi::c_int) - 1 as std::ffi::c_int);
        return GFALSE;
    }
    lua_remove(L, error_func_pos);
    return GTRUE;
}
