use ::libc;
#[c2rust::header_src = "/usr/lib/clang/20/include/__stddef_size_t.h:19"]
pub mod __stddef_size_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type size_t = std::ffi::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:19"]
pub mod types_h {
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = std::ffi::c_long;
    #[c2rust::src_loc = "162:1"]
    pub type __suseconds_t = std::ffi::c_long;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtypes.h:19"]
pub mod gtypes_h {
    #[c2rust::src_loc = "52:1"]
    pub type gchar = std::ffi::c_char;
    #[c2rust::src_loc = "55:1"]
    pub type gint = std::ffi::c_int;
    #[c2rust::src_loc = "56:1"]
    pub type gboolean = gint;
    #[c2rust::src_loc = "61:1"]
    pub type guint = std::ffi::c_uint;
    #[c2rust::src_loc = "64:1"]
    pub type gdouble = std::ffi::c_double;
    #[c2rust::src_loc = "109:1"]
    pub type gpointer = *mut std::ffi::c_void;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gmain.h:19"]
pub mod gmain_h {
    #[c2rust::src_loc = "199:1"]
    pub type GSourceFunc = Option::<unsafe extern "C" fn(gpointer) -> gboolean>;
    use super::gtypes_h::{gboolean, gpointer, guint};
    extern "C" {
        #[c2rust::src_loc = "962:1"]
        pub fn g_idle_add(function: GSourceFunc, data: gpointer) -> guint;
        #[c2rust::src_loc = "973:1"]
        pub fn g_idle_remove_by_data(data: gpointer) -> gboolean;
    }
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timeval.h:21"]
pub mod struct_timeval_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "8:8"]
    pub struct timeval {
        pub tv_sec: __time_t,
        pub tv_usec: __suseconds_t,
    }
    use super::types_h::{__time_t, __suseconds_t};
}
#[c2rust::header_src = "/usr/include/luajit-2.1/lua.h:25"]
pub mod lua_h {
    #[c2rust::src_loc = "53:1"]
    pub type lua_CFunction = Option::<
        unsafe extern "C" fn(*mut lua_State) -> std::ffi::c_int,
    >;
    #[c2rust::src_loc = "100:1"]
    pub type lua_Number = std::ffi::c_double;
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
        #[c2rust::src_loc = "149:1"]
        pub fn lua_toboolean(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;
        #[c2rust::src_loc = "150:1"]
        pub fn lua_tolstring(
            L: *mut lua_State,
            idx: std::ffi::c_int,
            len: *mut size_t,
        ) -> *const std::ffi::c_char;
        #[c2rust::src_loc = "155:1"]
        pub fn lua_topointer(
            L: *mut lua_State,
            idx: std::ffi::c_int,
        ) -> *const std::ffi::c_void;
        #[c2rust::src_loc = "162:1"]
        pub fn lua_pushnumber(L: *mut lua_State, n: lua_Number);
        #[c2rust::src_loc = "164:1"]
        pub fn lua_pushlstring(L: *mut lua_State, s: *const std::ffi::c_char, l: size_t);
        #[c2rust::src_loc = "165:1"]
        pub fn lua_pushstring(L: *mut lua_State, s: *const std::ffi::c_char);
        #[c2rust::src_loc = "169:1"]
        pub fn lua_pushcclosure(
            L: *mut lua_State,
            fn_0: lua_CFunction,
            n: std::ffi::c_int,
        );
        #[c2rust::src_loc = "170:1"]
        pub fn lua_pushboolean(L: *mut lua_State, b: std::ffi::c_int);
        #[c2rust::src_loc = "171:1"]
        pub fn lua_pushlightuserdata(L: *mut lua_State, p: *mut std::ffi::c_void);
        #[c2rust::src_loc = "180:1"]
        pub fn lua_rawget(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "203:1"]
        pub fn lua_pcall(
            L: *mut lua_State,
            nargs: std::ffi::c_int,
            nresults: std::ffi::c_int,
            errfunc: std::ffi::c_int,
        ) -> std::ffi::c_int;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/common.h:27"]
pub mod common_h {
    #[c2rust::src_loc = "24:1"]
    pub type common_t = _common_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "24:16"]
    pub struct _common_t {
        pub L: *mut lua_State,
    }
    use super::lua_h::lua_State;
    extern "C" {
        #[c2rust::src_loc = "29:17"]
        pub static mut common: common_t;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/log.h:26"]
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
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gmem.h:19"]
pub mod gmem_h {
    use super::gtypes_h::gpointer;
    extern "C" {
        #[c2rust::src_loc = "73:1"]
        pub fn g_free(mem: gpointer);
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/guri.h:19"]
pub mod guri_h {
    use super::gtypes_h::gboolean;
    extern "C" {
        #[c2rust::src_loc = "389:1"]
        pub fn g_uri_unescape_string(
            escaped_string: *const std::ffi::c_char,
            illegal_characters: *const std::ffi::c_char,
        ) -> *mut std::ffi::c_char;
        #[c2rust::src_loc = "402:1"]
        pub fn g_uri_escape_string(
            unescaped: *const std::ffi::c_char,
            reserved_chars_allowed: *const std::ffi::c_char,
            allow_utf8: gboolean,
        ) -> *mut std::ffi::c_char;
    }
}
#[c2rust::header_src = "/usr/include/luajit-2.1/lauxlib.h:25"]
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
        #[c2rust::src_loc = "34:1"]
        pub fn luaL_checklstring(
            L: *mut lua_State,
            numArg: std::ffi::c_int,
            l: *mut size_t,
        ) -> *const std::ffi::c_char;
    }
}
#[c2rust::header_src = "/usr/include/sys/time.h:26"]
pub mod time_h {
    use super::struct_timeval_h::timeval;
    extern "C" {
        #[c2rust::src_loc = "67:1"]
        pub fn gettimeofday(
            __tv: *mut timeval,
            __tz: *mut std::ffi::c_void,
        ) -> std::ffi::c_int;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/util.h:26"]
pub mod util_h {
    #[inline]
    #[c2rust::src_loc = "63:1"]
    pub unsafe extern "C" fn l_time() -> gdouble {
        let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
        gettimeofday(&mut tv, 0 as *mut std::ffi::c_void);
        return tv.tv_sec as std::ffi::c_double
            + tv.tv_usec as std::ffi::c_double / 1e6f64;
    }
    use super::gtypes_h::gdouble;
    use super::struct_timeval_h::timeval;
    use super::types_h::{__time_t, __suseconds_t};
    use super::time_h::gettimeofday;
}
#[c2rust::header_src = "/home/daana/git/luakit/common/luautil.h:27"]
pub mod luautil_h {
    use super::lua_h::lua_State;
    use super::gtypes_h::gint;
    extern "C" {
        #[c2rust::src_loc = "26:1"]
        pub fn luaH_dofunction_on_error(L: *mut lua_State) -> gint;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/lualib.h:27"]
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
#[c2rust::header_src = "/home/daana/git/luakit/common/luaobject.h:27"]
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
    #[c2rust::src_loc = "121:1"]
    pub unsafe extern "C" fn luaH_object_unref(mut L: *mut lua_State, mut p: gpointer) {
        luaH_object_registry_push(L);
        luaH_object_decref(L, -(1 as std::ffi::c_int), p);
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
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
        #[c2rust::src_loc = "41:1"]
        pub fn luaH_object_decref(L: *mut lua_State, tud: gint, oud: gpointer);
    }
}
pub use self::__stddef_size_t_h::size_t;
pub use self::types_h::{__time_t, __suseconds_t};
pub use self::gtypes_h::{gchar, gint, gboolean, guint, gdouble, gpointer};
pub use self::gmain_h::{GSourceFunc, g_idle_add, g_idle_remove_by_data};
pub use self::struct_timeval_h::timeval;
pub use self::lua_h::{
    lua_CFunction, lua_Number, lua_State, lua_gettop, lua_settop, lua_remove, lua_insert,
    lua_type, lua_toboolean, lua_tolstring, lua_topointer, lua_pushnumber,
    lua_pushlstring, lua_pushstring, lua_pushcclosure, lua_pushboolean,
    lua_pushlightuserdata, lua_rawget, lua_pcall,
};
pub use self::common_h::{common_t, _common_t, common};
pub use self::log_h::{
    log_level_t, LOG_LEVEL_debug, LOG_LEVEL_verbose, LOG_LEVEL_info, LOG_LEVEL_warn,
    LOG_LEVEL_error, LOG_LEVEL_fatal, _log,
};
use self::gmem_h::g_free;
use self::guri_h::{g_uri_unescape_string, g_uri_escape_string};
use self::lauxlib_h::{luaL_typerror, luaL_checklstring};
use self::time_h::gettimeofday;
pub use self::util_h::l_time;
use self::luautil_h::luaH_dofunction_on_error;
pub use self::lualib_h::luaH_dofunction;
pub use self::luaobject_h::{
    luaH_object_registry_push, luaH_object_ref, luaH_object_unref, luaH_object_push,
    luaH_object_incref, luaH_object_decref,
};
#[no_mangle]
#[c2rust::src_loc = "36:1"]
pub unsafe extern "C" fn luaH_luakit_time(mut L: *mut lua_State) -> gint {
    lua_pushnumber(L, l_time());
    return 1 as std::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "56:1"]
pub unsafe extern "C" fn luaH_luakit_uri_encode(mut L: *mut lua_State) -> gint {
    let mut string: *const gchar = luaL_checklstring(
        L,
        1 as std::ffi::c_int,
        0 as *mut size_t,
    );
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
#[no_mangle]
#[c2rust::src_loc = "85:1"]
pub unsafe extern "C" fn luaH_luakit_uri_decode(mut L: *mut lua_State) -> gint {
    let mut string: *const gchar = luaL_checklstring(
        L,
        1 as std::ffi::c_int,
        0 as *mut size_t,
    );
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
#[no_mangle]
#[c2rust::src_loc = "112:1"]
pub unsafe extern "C" fn idle_cb(mut func: gpointer) -> gboolean {
    let mut L: *mut lua_State = common.L;
    let mut top: gint = lua_gettop(L);
    luaH_object_push(L, func);
    let mut ok: gboolean = luaH_dofunction(
        L,
        0 as std::ffi::c_int,
        1 as std::ffi::c_int,
    );
    let mut keep: gboolean = lua_toboolean(L, -(1 as std::ffi::c_int));
    if keep == 0 || ok == 0 {
        luaH_object_unref(L, func);
    }
    lua_settop(L, top);
    return (keep != 0 && ok != 0) as std::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "149:1"]
pub unsafe extern "C" fn luaH_luakit_idle_add(mut L: *mut lua_State) -> gint {
    if !(lua_type(L, 1 as std::ffi::c_int) == 6 as std::ffi::c_int) {
        luaL_typerror(
            L,
            1 as std::ffi::c_int,
            b"function\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    let mut func: gpointer = luaH_object_ref(L, 1 as std::ffi::c_int);
    g_idle_add(Some(idle_cb as unsafe extern "C" fn(gpointer) -> gboolean), func);
    return 0 as std::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "168:1"]
pub unsafe extern "C" fn luaH_luakit_idle_remove(mut L: *mut lua_State) -> gint {
    if !(lua_type(L, 1 as std::ffi::c_int) == 6 as std::ffi::c_int) {
        luaL_typerror(
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
