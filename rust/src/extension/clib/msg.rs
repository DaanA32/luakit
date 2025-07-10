use ::libc;

pub mod __stddef_size_t_h {

    pub type size_t = std::ffi::c_ulong;
}

pub mod lua_h {

    pub type lua_CFunction = Option::<
        unsafe extern "C" fn(*mut lua_State) -> std::ffi::c_int,
    >;
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct lua_Debug {
        pub event: std::ffi::c_int,
        pub name: *const std::ffi::c_char,
        pub namewhat: *const std::ffi::c_char,
        pub what: *const std::ffi::c_char,
        pub source: *const std::ffi::c_char,
        pub currentline: std::ffi::c_int,
        pub nups: std::ffi::c_int,
        pub linedefined: std::ffi::c_int,
        pub lastlinedefined: std::ffi::c_int,
        pub short_src: [std::ffi::c_char; 60],
        pub i_ci: std::ffi::c_int,
    }

    pub const LUA_REGISTRYINDEX: std::ffi::c_int = -(10000 as std::ffi::c_int);

    pub const LUA_GLOBALSINDEX: std::ffi::c_int = -(10002 as std::ffi::c_int);

    pub const LUA_TNUMBER: std::ffi::c_int = 3 as std::ffi::c_int;
    use super::__stddef_size_t_h::size_t;
    unsafe extern "C" {

        pub type lua_State;

        pub fn lua_gettop(L: *mut lua_State) -> std::ffi::c_int;

        pub fn lua_settop(L: *mut lua_State, idx: std::ffi::c_int);

        pub fn lua_pushvalue(L: *mut lua_State, idx: std::ffi::c_int);

        pub fn lua_remove(L: *mut lua_State, idx: std::ffi::c_int);

        pub fn lua_insert(L: *mut lua_State, idx: std::ffi::c_int);

        pub fn lua_type(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;

        pub fn lua_tolstring(
            L: *mut lua_State,
            idx: std::ffi::c_int,
            len: *mut size_t,
        ) -> *const std::ffi::c_char;

        pub fn lua_pushlstring(L: *mut lua_State, s: *const std::ffi::c_char, l: size_t);

        pub fn lua_pushlightuserdata(L: *mut lua_State, p: *mut std::ffi::c_void);

        pub fn lua_getfield(
            L: *mut lua_State,
            idx: std::ffi::c_int,
            k: *const std::ffi::c_char,
        );

        pub fn lua_rawget(L: *mut lua_State, idx: std::ffi::c_int);

        pub fn lua_pcall(
            L: *mut lua_State,
            nargs: std::ffi::c_int,
            nresults: std::ffi::c_int,
            errfunc: std::ffi::c_int,
        ) -> std::ffi::c_int;

        pub fn lua_getstack(
            L: *mut lua_State,
            level: std::ffi::c_int,
            ar: *mut lua_Debug,
        ) -> std::ffi::c_int;

        pub fn lua_getinfo(
            L: *mut lua_State,
            what: *const std::ffi::c_char,
            ar: *mut lua_Debug,
        ) -> std::ffi::c_int;
    }
}

pub mod gtypes_h {

    pub type gpointer = *mut std::ffi::c_void;

    pub type gint = std::ffi::c_int;

    pub type gchar = std::ffi::c_char;
}

pub mod lauxlib_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct luaL_Reg {
        pub name: *const std::ffi::c_char,
        pub func: lua_CFunction,
    }
    use super::lua_h::{lua_CFunction, lua_State};
    unsafe extern "C" {

        pub fn luaL_error(
            L: *mut lua_State,
            fmt: *const std::ffi::c_char,
            _: ...
        ) -> std::ffi::c_int;
    }
}

pub mod log_h {

    pub type log_level_t = std::ffi::c_uint;

    pub const LOG_LEVEL_debug: log_level_t = 5;

    pub const LOG_LEVEL_verbose: log_level_t = 4;

    pub const LOG_LEVEL_info: log_level_t = 3;

    pub const LOG_LEVEL_warn: log_level_t = 2;

    pub const LOG_LEVEL_error: log_level_t = 1;

    pub const LOG_LEVEL_fatal: log_level_t = 0;
    use super::gtypes_h::gchar;
    unsafe extern "C" {

        pub fn _log(lvl: log_level_t, _: *const gchar, _: *const gchar, _: ...);
    }
}

pub mod luaclass_h {
    use super::lua_h::lua_State;
    use super::gtypes_h::gchar;
    use super::lauxlib_h::luaL_Reg;
    unsafe extern "C" {

        pub fn luaH_openlib(
            _: *mut lua_State,
            _: *const gchar,
            _: *const luaL_Reg,
            _: *const luaL_Reg,
        );
    }
}

pub mod luaobject_h {
    #[inline]

    pub unsafe extern "C" fn luaH_object_registry_push(mut L: *mut lua_State) {
        lua_pushlstring(
            L,
            b"luakit.object.registry\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 23]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        lua_rawget(L, LUA_REGISTRYINDEX);
    }
    #[inline]

    pub unsafe extern "C" fn luaH_object_ref(
        mut L: *mut lua_State,
        mut oud: gint,
    ) -> gpointer {
        luaH_object_registry_push(L);
        let mut p = luaH_object_incref(
            L,
            -(1 as std::ffi::c_int),
            if oud < 0 as std::ffi::c_int { oud - 1 as std::ffi::c_int } else { oud },
        );
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        return p;
    }
    #[inline]

    pub unsafe extern "C" fn luaH_object_push(
        mut L: *mut lua_State,
        mut p: gpointer,
    ) -> gint {
        luaH_object_registry_push(L);
        lua_pushlightuserdata(L, p);
        lua_rawget(L, -(2 as std::ffi::c_int));
        lua_remove(L, -(2 as std::ffi::c_int));
        return 1 as std::ffi::c_int;
    }
    use super::lua_h::{
        lua_State, lua_pushlstring, lua_rawget, LUA_REGISTRYINDEX, lua_settop,
        lua_pushlightuserdata, lua_remove,
    };
    use super::gtypes_h::{gint, gpointer};
    unsafe extern "C" {

        pub fn luaH_object_incref(L: *mut lua_State, tud: gint, oud: gint) -> gpointer;
    }
}

pub mod msg_h {

    pub static mut string_format_ref: gpointer = 0 as *const std::ffi::c_void
        as *mut std::ffi::c_void;

    pub static mut tostring_ref: gpointer = 0 as *const std::ffi::c_void
        as *mut std::ffi::c_void;

    pub unsafe extern "C" fn luaH_msg_string_from_args(
        mut L: *mut lua_State,
    ) -> *const gchar {
        let mut nargs = lua_gettop(L);
        let mut i = 1 as std::ffi::c_int;
        while i <= nargs {
            if lua_type(L, i) != LUA_TNUMBER {
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
                b"failed to format message: %s\0" as *const u8
                    as *const std::ffi::c_char,
                lua_tolstring(L, -(1 as std::ffi::c_int), NULL as *mut size_t),
            );
        }
        return lua_tolstring(L, -(1 as std::ffi::c_int), NULL as *mut size_t);
    }

    pub unsafe extern "C" fn luaH_msg(
        mut L: *mut lua_State,
        mut lvl: log_level_t,
    ) -> gint {
        let mut ar = lua_Debug {
            event: 0,
            name: 0 as *const std::ffi::c_char,
            namewhat: 0 as *const std::ffi::c_char,
            what: 0 as *const std::ffi::c_char,
            source: 0 as *const std::ffi::c_char,
            currentline: 0,
            nups: 0,
            linedefined: 0,
            lastlinedefined: 0,
            short_src: [0; 60],
            i_ci: 0,
        };
        lua_getstack(L, 1 as std::ffi::c_int, &mut ar);
        lua_getinfo(L, b"Sln\0" as *const u8 as *const std::ffi::c_char, &mut ar);
        let mut src = if *(ar.source).offset(0 as std::ffi::c_int as isize)
            as std::ffi::c_int == '@' as i32
        {
            (ar.source).offset(1 as std::ffi::c_int as isize)
        } else {
            (ar.short_src).as_mut_ptr() as *const std::ffi::c_char
        };
        _log(
            lvl,
            src,
            b"%s\0" as *const u8 as *const std::ffi::c_char,
            luaH_msg_string_from_args(L),
        );
        return 0 as std::ffi::c_int;
    }

    pub unsafe extern "C" fn luaH_msg_debug(mut L: *mut lua_State) -> gint {
        return luaH_msg(L, LOG_LEVEL_debug);
    }

    pub unsafe extern "C" fn luaH_msg_verbose(mut L: *mut lua_State) -> gint {
        return luaH_msg(L, LOG_LEVEL_verbose);
    }

    pub unsafe extern "C" fn luaH_msg_info(mut L: *mut lua_State) -> gint {
        return luaH_msg(L, LOG_LEVEL_info);
    }

    pub unsafe extern "C" fn luaH_msg_warn(mut L: *mut lua_State) -> gint {
        return luaH_msg(L, LOG_LEVEL_warn);
    }

    pub unsafe extern "C" fn luaH_msg_error(mut L: *mut lua_State) -> gint {
        return luaH_msg(L, LOG_LEVEL_error);
    }

    pub unsafe extern "C" fn luaH_msg_fatal(mut L: *mut lua_State) -> gint {
        return luaH_msg(L, LOG_LEVEL_fatal);
    }
    use super::gtypes_h::{gpointer, gchar, gint};
    use super::lua_h::{
        lua_State, lua_gettop, lua_type, LUA_TNUMBER, lua_pushvalue, lua_pcall,
        lua_remove, lua_insert, lua_tolstring, lua_Debug, lua_getstack, lua_getinfo,
    };
    use super::luaobject_h::luaH_object_push;
    use super::lauxlib_h::luaL_error;
    use super::__stddef_null_h::NULL;
    use super::__stddef_size_t_h::size_t;
    use super::log_h::{
        log_level_t, _log, LOG_LEVEL_debug, LOG_LEVEL_verbose, LOG_LEVEL_info,
        LOG_LEVEL_warn, LOG_LEVEL_error, LOG_LEVEL_fatal,
    };
}

pub mod __stddef_null_h {

    pub const NULL: std::ffi::c_int = 0 as std::ffi::c_int;
}
pub use self::__stddef_size_t_h::size_t;
pub use self::lua_h::{
    lua_CFunction, lua_Debug, LUA_REGISTRYINDEX, LUA_GLOBALSINDEX, LUA_TNUMBER,
    lua_State, lua_gettop, lua_settop, lua_pushvalue, lua_remove, lua_insert, lua_type,
    lua_tolstring, lua_pushlstring, lua_pushlightuserdata, lua_getfield, lua_rawget,
    lua_pcall, lua_getstack, lua_getinfo,
};
pub use self::gtypes_h::{gpointer, gint, gchar};
pub use self::lauxlib_h::{luaL_Reg, luaL_error};
pub use self::log_h::{
    log_level_t, LOG_LEVEL_debug, LOG_LEVEL_verbose, LOG_LEVEL_info, LOG_LEVEL_warn,
    LOG_LEVEL_error, LOG_LEVEL_fatal, _log,
};
use self::luaclass_h::luaH_openlib;
pub use self::luaobject_h::{
    luaH_object_registry_push, luaH_object_ref, luaH_object_push, luaH_object_incref,
};
pub use self::msg_h::{
    string_format_ref, tostring_ref, luaH_msg_string_from_args, luaH_msg, luaH_msg_debug,
    luaH_msg_verbose, luaH_msg_info, luaH_msg_warn, luaH_msg_error, luaH_msg_fatal,
};
pub use self::__stddef_null_h::NULL;
#[unsafe(no_mangle)]

pub unsafe extern "C" fn msg_lib_setup(mut L: *mut lua_State) {
    static mut msg_lib: [luaL_Reg; 7] = unsafe {
        [
            {
                let mut init = luaL_Reg {
                    name: b"fatal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_msg_fatal as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"error\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_msg_error as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"warn\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_msg_warn as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"info\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_msg_info as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"verbose\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_msg_verbose as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"debug\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_msg_debug as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: NULL as *const std::ffi::c_char,
                    func: ::core::mem::transmute::<
                        libc::intptr_t,
                        lua_CFunction,
                    >(NULL as libc::intptr_t),
                };
                init
            },
        ]
    };
    luaH_openlib(
        L,
        b"msg\0" as *const u8 as *const std::ffi::c_char,
        msg_lib.as_ptr(),
        msg_lib.as_ptr(),
    );
    lua_getfield(
        L,
        LUA_GLOBALSINDEX,
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
        LUA_GLOBALSINDEX,
        b"tostring\0" as *const u8 as *const std::ffi::c_char,
    );
    tostring_ref = luaH_object_ref(L, -(1 as std::ffi::c_int));
}
