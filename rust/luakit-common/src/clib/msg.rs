use libc::size_t;
use mlua::ffi::*;

pub mod msg_h {
    use std::{
        ffi::{CStr, c_void},
        mem::MaybeUninit,
    };

    use glib_sys::*;
    use libc::size_t;
    use mlua::ffi::*;

    use crate::{common::luaobject::luaH_object_push, gtypes::*, log::*};

    pub static mut string_format_ref: gpointer =
        0 as *const std::ffi::c_void as *mut std::ffi::c_void;
    pub static mut tostring_ref: gpointer = 0 as *const std::ffi::c_void as *mut std::ffi::c_void;
    pub unsafe extern "C-unwind" fn luaH_msg_string_from_args(
        mut L: *mut lua_State,
    ) -> *const gchar {
        let mut nargs: gint = lua_gettop(L);
        let mut i: gint = 1 as std::ffi::c_int;
        while i <= nargs {
            if lua_type(L, i) != 3 as std::ffi::c_int {
                luaH_object_push(L, tostring_ref);
                lua_pushvalue(L, i);
                lua_pcall(
                    L,
                    1 as std::ffi::c_int,
                    1 as std::ffi::c_int,
                    0 as std::ffi::c_int,
                );
                lua_remove(L, i);
                lua_insert(L, i);
            }
            i += 1;
            i;
        }
        luaH_object_push(L, string_format_ref);
        lua_insert(L, 1 as std::ffi::c_int);
        if lua_pcall(L, nargs, 1 as std::ffi::c_int, 0 as std::ffi::c_int) != 0 {
            luaL_error(
                L,
                b"failed to format message: %s\0" as *const u8 as *const std::ffi::c_char,
                lua_tolstring(L, -(1 as std::ffi::c_int), 0 as *mut size_t),
            );
        }
        return lua_tolstring(L, -(1 as std::ffi::c_int), 0 as *mut size_t);
    }
    pub unsafe extern "C-unwind" fn luaH_msg(mut L: *mut lua_State, mut lvl: log_level_t) -> gint {
        let mut ar = MaybeUninit::<lua_Debug>::uninit();
        lua_getstack(L, 1 as std::ffi::c_int, ar.as_mut_ptr());
        lua_getinfo(
            L,
            b"Sln\0" as *const u8 as *const std::ffi::c_char,
            ar.as_mut_ptr(),
        );
        let mut src: *const std::ffi::c_char = if *((*ar.as_mut_ptr()).source)
            .offset(0 as std::ffi::c_int as isize)
            as std::ffi::c_int
            == '@' as i32
        {
            (*ar.as_ptr()).source.offset(1 as std::ffi::c_int as isize)
        } else {
            (*ar.as_ptr()).short_src.as_ptr() as *const std::ffi::c_char
        };
        _log(
            lvl,
            src,
            &CStr::from_ptr(luaH_msg_string_from_args(L)).to_string_lossy(),
        );
        return 0 as std::ffi::c_int;
    }
    pub unsafe extern "C-unwind" fn luaH_msg_info(mut L: *mut lua_State) -> gint {
        return luaH_msg(L, LOG_LEVEL_info);
    }
    pub unsafe extern "C-unwind" fn luaH_msg_debug(mut L: *mut lua_State) -> gint {
        return luaH_msg(L, LOG_LEVEL_debug);
    }
    pub unsafe extern "C-unwind" fn luaH_msg_warn(mut L: *mut lua_State) -> gint {
        return luaH_msg(L, LOG_LEVEL_warn);
    }
    pub unsafe extern "C-unwind" fn luaH_msg_verbose(mut L: *mut lua_State) -> gint {
        return luaH_msg(L, LOG_LEVEL_verbose);
    }
    pub unsafe extern "C-unwind" fn luaH_msg_error(mut L: *mut lua_State) -> gint {
        return luaH_msg(L, LOG_LEVEL_error);
    }
    pub unsafe extern "C-unwind" fn luaH_msg_fatal(mut L: *mut lua_State) -> gint {
        return luaH_msg(L, LOG_LEVEL_fatal);
    }
}
use crate::common::luaclass::signal_h::signal_new;
use crate::common::luaclass::{
    lua_class_property_array_t, lua_class_t, luaH_class_add_signal, luaH_class_emit_signal,
    luaH_class_remove_signal, luaH_openlib,
};
use crate::common::luaobject::luaH_object_ref;
use crate::gtypes::{gchar, gint};

pub use self::msg_h::{
    luaH_msg, luaH_msg_debug, luaH_msg_error, luaH_msg_fatal, luaH_msg_info,
    luaH_msg_string_from_args, luaH_msg_verbose, luaH_msg_warn, string_format_ref, tostring_ref,
};

static mut msg_class: lua_class_t = lua_class_t {
    name: 0 as *const gchar,
    signals: std::ptr::null_mut(),
    allocator: None,
    properties: 0 as *const lua_class_property_array_t as *mut lua_class_property_array_t,
    index_miss_property: None,
    newindex_miss_property: None,
};
#[inline]
unsafe extern "C-unwind" fn luaH_msg_class_emit_signal(mut L: *mut lua_State) -> gint {
    return luaH_class_emit_signal(
        L,
        &mut msg_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t),
        lua_gettop(L) - 1 as std::ffi::c_int,
        -(1 as std::ffi::c_int),
    );
}
#[inline]
unsafe extern "C-unwind" fn luaH_msg_class_remove_signal(mut L: *mut lua_State) -> gint {
    luaH_class_remove_signal(
        L,
        &mut msg_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn luaH_msg_class_add_signal(mut L: *mut lua_State) -> gint {
    luaH_class_add_signal(
        L,
        &mut msg_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn msg_lib_get_msg_class() -> *mut lua_class_t {
    return &mut msg_class;
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn msg_lib_setup(mut L: *mut lua_State) {
    let msg_lib = unsafe {
        [
            {
                let mut init = luaL_Reg {
                    name: b"add_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_msg_class_add_signal,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"remove_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_msg_class_remove_signal,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"emit_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_msg_class_emit_signal,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"fatal\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_msg_fatal,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"error\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_msg_error,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"warn\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_msg_warn,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"info\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_msg_info,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"verbose\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_msg_verbose,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"debug\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_msg_debug,
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
        b"msg\0" as *const u8 as *const std::ffi::c_char,
        &msg_lib,
        &msg_lib,
    );
    msg_class.signals = signal_new();
    lua_getfield(
        L,
        -(10002 as std::ffi::c_int),
        b"string\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_getfield(
        L,
        -(1 as std::ffi::c_int),
        b"format\0" as *const u8 as *const std::ffi::c_char,
    );
    string_format_ref = luaH_object_ref(L, -(1 as std::ffi::c_int));
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    lua_getfield(
        L,
        -(10002 as std::ffi::c_int),
        b"tostring\0" as *const u8 as *const std::ffi::c_char,
    );
    tostring_ref = luaH_object_ref(L, -(1 as std::ffi::c_int));
}
