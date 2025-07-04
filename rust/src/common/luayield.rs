use ::libc;
#[c2rust::header_src = "/usr/lib/clang/20/include/__stddef_size_t.h:21"]
pub mod __stddef_size_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type size_t = std::ffi::c_ulong;
}
#[c2rust::header_src = "/usr/include/luajit-2.1/lua.h:21"]
pub mod lua_h {
    #[c2rust::src_loc = "53:1"]
    pub type lua_CFunction = Option::<
        unsafe extern "C" fn(*mut lua_State) -> std::ffi::c_int,
    >;
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "51:16"]
        pub type lua_State;
        #[c2rust::src_loc = "121:1"]
        pub fn lua_gettop(L: *mut lua_State) -> std::ffi::c_int;
        #[c2rust::src_loc = "122:1"]
        pub fn lua_settop(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "124:1"]
        pub fn lua_remove(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "125:1"]
        pub fn lua_insert(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "140:1"]
        pub fn lua_type(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;
        #[c2rust::src_loc = "150:1"]
        pub fn lua_tolstring(
            L: *mut lua_State,
            idx: std::ffi::c_int,
            len: *mut size_t,
        ) -> *const std::ffi::c_char;
        #[c2rust::src_loc = "164:1"]
        pub fn lua_pushlstring(L: *mut lua_State, s: *const std::ffi::c_char, l: size_t);
        #[c2rust::src_loc = "169:1"]
        pub fn lua_pushcclosure(
            L: *mut lua_State,
            fn_0: lua_CFunction,
            n: std::ffi::c_int,
        );
        #[c2rust::src_loc = "171:1"]
        pub fn lua_pushlightuserdata(L: *mut lua_State, p: *mut std::ffi::c_void);
        #[c2rust::src_loc = "179:1"]
        pub fn lua_getfield(
            L: *mut lua_State,
            idx: std::ffi::c_int,
            k: *const std::ffi::c_char,
        );
        #[c2rust::src_loc = "180:1"]
        pub fn lua_rawget(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "202:1"]
        pub fn lua_call(
            L: *mut lua_State,
            nargs: std::ffi::c_int,
            nresults: std::ffi::c_int,
        );
        #[c2rust::src_loc = "203:1"]
        pub fn lua_pcall(
            L: *mut lua_State,
            nargs: std::ffi::c_int,
            nresults: std::ffi::c_int,
            errfunc: std::ffi::c_int,
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "215:1"]
        pub fn lua_resume(L: *mut lua_State, narg: std::ffi::c_int) -> std::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtypes.h:21"]
pub mod gtypes_h {
    #[c2rust::src_loc = "52:1"]
    pub type gchar = std::ffi::c_char;
    #[c2rust::src_loc = "55:1"]
    pub type gint = std::ffi::c_int;
    #[c2rust::src_loc = "56:1"]
    pub type gboolean = gint;
    #[c2rust::src_loc = "109:1"]
    pub type gpointer = *mut std::ffi::c_void;
}
#[c2rust::header_src = "/home/daana/git/luakit/common/log.h:22"]
pub mod log_h {
    #[c2rust::src_loc = "36:9"]
    pub type log_level_t = std::ffi::c_uint;
    #[c2rust::src_loc = "36:16"]
    pub const LOG_LEVEL_debug: log_level_t = 5;
    #[c2rust::src_loc = "36:16"]
    pub const LOG_LEVEL_verbose: log_level_t = 4;
    #[c2rust::src_loc = "36:16"]
    pub const LOG_LEVEL_info: log_level_t = 3;
    #[c2rust::src_loc = "36:16"]
    pub const LOG_LEVEL_warn: log_level_t = 2;
    #[c2rust::src_loc = "36:16"]
    pub const LOG_LEVEL_error: log_level_t = 1;
    #[c2rust::src_loc = "36:16"]
    pub const LOG_LEVEL_fatal: log_level_t = 0;
    use super::gtypes_h::gchar;
    extern "C" {
        #[c2rust::src_loc = "54:1"]
        pub fn _log(lvl: log_level_t, _: *const gchar, _: *const gchar, _: ...);
    }
}
#[c2rust::header_src = "/usr/include/luajit-2.1/lauxlib.h:21"]
pub mod lauxlib_h {
    use super::lua_h::lua_State;
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "32:1"]
        pub fn luaL_typerror(
            L: *mut lua_State,
            narg: std::ffi::c_int,
            tname: *const std::ffi::c_char,
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "66:1"]
        pub fn luaL_loadbuffer(
            L: *mut lua_State,
            buff: *const std::ffi::c_char,
            sz: size_t,
            name: *const std::ffi::c_char,
        ) -> std::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:21"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "407:15"]
        pub fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/luautil.h:22"]
pub mod luautil_h {
    use super::lua_h::lua_State;
    use super::gtypes_h::gint;
    extern "C" {
        #[c2rust::src_loc = "26:1"]
        pub fn luaH_dofunction_on_error(L: *mut lua_State) -> gint;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/lualib.h:22"]
pub mod lualib_h {
    #[inline]
    #[c2rust::src_loc = "101:1"]
    pub unsafe extern "C" fn luaH_dofunction(
        mut L: *mut lua_State,
        mut nargs: gint,
        mut nret: gint,
    ) -> gboolean {
        lua_insert(L, -nargs - 1 as std::ffi::c_int);
        lua_pushcclosure(
            L,
            Some(
                luaH_dofunction_on_error as unsafe extern "C" fn(*mut lua_State) -> gint,
            ),
            0 as std::ffi::c_int,
        );
        lua_insert(L, -nargs - 2 as std::ffi::c_int);
        let mut error_func_pos: gint = lua_gettop(L) - nargs - 1 as std::ffi::c_int;
        if lua_pcall(L, nargs, nret, -nargs - 2 as std::ffi::c_int) != 0 {
            _log(
                LOG_LEVEL_error,
                b"./common/lualib.h\0" as *const u8 as *const std::ffi::c_char,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                lua_tolstring(L, -(1 as std::ffi::c_int), 0 as *mut size_t),
            );
            lua_settop(L, -(2 as std::ffi::c_int) - 1 as std::ffi::c_int);
            return 0 as std::ffi::c_int;
        }
        lua_remove(L, error_func_pos);
        return (0 as std::ffi::c_int == 0) as std::ffi::c_int;
    }
    use super::lua_h::{
        lua_State, lua_insert, lua_pushcclosure, lua_gettop, lua_pcall, lua_tolstring,
        lua_settop, lua_remove,
    };
    use super::gtypes_h::{gint, gboolean};
    use super::luautil_h::luaH_dofunction_on_error;
    use super::log_h::{_log, LOG_LEVEL_error, log_level_t};
    use super::__stddef_size_t_h::size_t;
}
#[c2rust::header_src = "/home/daana/git/luakit/common/luaobject.h:23"]
pub mod luaobject_h {
    #[inline]
    #[c2rust::src_loc = "88:1"]
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
        lua_rawget(L, -(10000 as std::ffi::c_int));
    }
    #[inline]
    #[c2rust::src_loc = "99:1"]
    pub unsafe extern "C" fn luaH_object_ref(
        mut L: *mut lua_State,
        mut oud: gint,
    ) -> gpointer {
        luaH_object_registry_push(L);
        let mut p: gpointer = luaH_object_incref(
            L,
            -(1 as std::ffi::c_int),
            if oud < 0 as std::ffi::c_int { oud - 1 as std::ffi::c_int } else { oud },
        );
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        return p;
    }
    #[inline]
    #[c2rust::src_loc = "132:1"]
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
        lua_State, lua_pushlstring, lua_rawget, lua_settop, lua_pushlightuserdata,
        lua_remove,
    };
    use super::gtypes_h::{gint, gpointer};
    extern "C" {
        #[c2rust::src_loc = "40:1"]
        pub fn luaH_object_incref(L: *mut lua_State, tud: gint, oud: gint) -> gpointer;
    }
}
pub use self::__stddef_size_t_h::size_t;
pub use self::lua_h::{
    lua_CFunction, lua_State, lua_gettop, lua_settop, lua_remove, lua_insert, lua_type,
    lua_tolstring, lua_pushlstring, lua_pushcclosure, lua_pushlightuserdata,
    lua_getfield, lua_rawget, lua_call, lua_pcall, lua_resume,
};
pub use self::gtypes_h::{gchar, gint, gboolean, gpointer};
pub use self::log_h::{
    log_level_t, LOG_LEVEL_debug, LOG_LEVEL_verbose, LOG_LEVEL_info, LOG_LEVEL_warn,
    LOG_LEVEL_error, LOG_LEVEL_fatal, _log,
};
use self::lauxlib_h::{luaL_typerror, luaL_loadbuffer};
use self::string_h::strlen;
use self::luautil_h::luaH_dofunction_on_error;
pub use self::lualib_h::luaH_dofunction;
pub use self::luaobject_h::{
    luaH_object_registry_push, luaH_object_ref, luaH_object_push, luaH_object_incref,
};
#[c2rust::src_loc = "27:14"]
static mut wrap_function_ref: *mut std::ffi::c_void = 0 as *const std::ffi::c_void
    as *mut std::ffi::c_void;
#[c2rust::src_loc = "28:14"]
static mut yield_ref: *mut std::ffi::c_void = 0 as *const std::ffi::c_void
    as *mut std::ffi::c_void;
#[c2rust::src_loc = "29:14"]
static mut unlock_ref: *mut std::ffi::c_void = 0 as *const std::ffi::c_void
    as *mut std::ffi::c_void;
#[c2rust::src_loc = "30:21"]
static mut sched_src: *const std::ffi::c_char = b" local y = {}                                                                               \n                                                                                            \n local wrap_function = function (fn)                                                        \n     return function (...)                                                                  \n         assert(coroutine.running(), 'cannot call asynchronous function from main thread!') \n         y.yieldable = true                                                                 \n         local ret = {fn(...)}                                                              \n         y.yieldable = false                                                                \n         if y.yield then                                                                    \n             y.yield = false                                                                \n             y[coroutine.running()] = true                                                  \n             repeat                                                                         \n                 ret = {coroutine.yield()}                                                  \n             until not y[coroutine.running()]                                               \n         end                                                                                \n         return unpack(ret)                                                                 \n     end                                                                                    \n end                                                                                        \n                                                                                            \n local yield = function ()                                                                  \n     assert(y.yieldable, 'attempted to yield from unwrapped operation!')                    \n     y.yield = true                                                                         \n end                                                                                        \n                                                                                            \n local unlock = function ()                                                                 \n     y[coroutine.running()] = nil                                                           \n end                                                                                        \n                                                                                            \n return {                                                                                   \n     wrap_function = wrap_function,                                                         \n     yield = yield,                                                                         \n     unlock = unlock,                                                                       \n }                                                                                          \n\0"
    as *const u8 as *const std::ffi::c_char;
#[no_mangle]
#[c2rust::src_loc = "67:1"]
pub unsafe extern "C" fn luaH_yield_setup(mut L: *mut lua_State) {
    let mut top: gint = lua_gettop(L);
    luaL_loadbuffer(
        L,
        sched_src,
        strlen(sched_src),
        b"luakit_yield_handler\0" as *const u8 as *const std::ffi::c_char,
    );
    luaH_dofunction(L, 0 as std::ffi::c_int, 1 as std::ffi::c_int);
    lua_getfield(
        L,
        -(1 as std::ffi::c_int),
        b"yield\0" as *const u8 as *const std::ffi::c_char,
    );
    yield_ref = luaH_object_ref(L, -(1 as std::ffi::c_int));
    lua_getfield(
        L,
        -(1 as std::ffi::c_int),
        b"wrap_function\0" as *const u8 as *const std::ffi::c_char,
    );
    wrap_function_ref = luaH_object_ref(L, -(1 as std::ffi::c_int));
    lua_getfield(
        L,
        -(1 as std::ffi::c_int),
        b"unlock\0" as *const u8 as *const std::ffi::c_char,
    );
    unlock_ref = luaH_object_ref(L, -(1 as std::ffi::c_int));
    lua_settop(L, top);
}
#[no_mangle]
#[c2rust::src_loc = "82:1"]
pub unsafe extern "C" fn luaH_yield_wrap_function(mut L: *mut lua_State) {
    if !(lua_type(L, -(1 as std::ffi::c_int)) == 6 as std::ffi::c_int) {
        luaL_typerror(
            L,
            -(1 as std::ffi::c_int),
            b"function\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    luaH_object_push(L, wrap_function_ref);
    luaH_dofunction(L, 1 as std::ffi::c_int, 1 as std::ffi::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "90:1"]
pub unsafe extern "C" fn luaH_yield(mut L: *mut lua_State) -> std::ffi::c_int {
    luaH_object_push(L, yield_ref);
    luaH_dofunction(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    return 0 as std::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "103:1"]
pub unsafe extern "C" fn luaH_resume(mut L: *mut lua_State, mut nret: gint) -> gboolean {
    luaH_object_push(L, unlock_ref);
    luaH_dofunction(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    let mut top: gint = lua_gettop(L) - nret;
    let mut ret: gint = lua_resume(L, nret);
    if ret == 0 as std::ffi::c_int || ret == 1 as std::ffi::c_int {
        return (0 as std::ffi::c_int == 0) as std::ffi::c_int;
    }
    lua_pushcclosure(
        L,
        Some(luaH_dofunction_on_error as unsafe extern "C" fn(*mut lua_State) -> gint),
        0 as std::ffi::c_int,
    );
    lua_insert(L, -(2 as std::ffi::c_int));
    lua_call(L, 1 as std::ffi::c_int, 1 as std::ffi::c_int);
    _log(
        LOG_LEVEL_error,
        b"common/luayield.c\0" as *const u8 as *const std::ffi::c_char,
        b"%s\0" as *const u8 as *const std::ffi::c_char,
        lua_tolstring(L, -(1 as std::ffi::c_int), 0 as *mut size_t),
    );
    lua_settop(L, top);
    return 0 as std::ffi::c_int;
}
