use ::libc;
#[c2rust::header_src = "/usr/lib/clang/20/include/__stddef_ptrdiff_t.h:22"]
pub mod __stddef_ptrdiff_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type ptrdiff_t = std::ffi::c_long;
}
#[c2rust::header_src = "/usr/lib/clang/20/include/__stddef_size_t.h:22"]
pub mod __stddef_size_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type size_t = std::ffi::c_ulong;
}
#[c2rust::header_src = "/usr/lib/glib-2.0/include/glibconfig.h:22"]
pub mod glibconfig_h {
    #[c2rust::src_loc = "82:1"]
    pub type gssize = std::ffi::c_long;
}
#[c2rust::header_src = "/usr/include/bits/types.h:22"]
pub mod types_h {
    #[c2rust::src_loc = "194:1"]
    pub type __ssize_t = std::ffi::c_long;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtypes.h:22"]
pub mod gtypes_h {
    #[c2rust::src_loc = "52:1"]
    pub type gchar = std::ffi::c_char;
    #[c2rust::src_loc = "54:1"]
    pub type glong = std::ffi::c_long;
    #[c2rust::src_loc = "55:1"]
    pub type gint = std::ffi::c_int;
    #[c2rust::src_loc = "109:1"]
    pub type gpointer = *mut std::ffi::c_void;
}
#[c2rust::header_src = "/usr/include/sys/types.h:22"]
pub mod sys_types_h {
    #[c2rust::src_loc = "108:1"]
    pub type ssize_t = __ssize_t;
    use super::types_h::__ssize_t;
}
#[c2rust::header_src = "/usr/include/luajit-2.1/lua.h:22"]
pub mod lua_h {
    #[c2rust::src_loc = "53:1"]
    pub type lua_CFunction = Option::<
        unsafe extern "C" fn(*mut lua_State) -> std::ffi::c_int,
    >;
    #[c2rust::src_loc = "100:1"]
    pub type lua_Number = std::ffi::c_double;
    #[c2rust::src_loc = "104:1"]
    pub type lua_Integer = ptrdiff_t;
    use super::__stddef_ptrdiff_t_h::ptrdiff_t;
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "51:16"]
        pub type lua_State;
        #[c2rust::src_loc = "121:1"]
        pub fn lua_gettop(L: *mut lua_State) -> std::ffi::c_int;
        #[c2rust::src_loc = "122:1"]
        pub fn lua_settop(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "123:1"]
        pub fn lua_pushvalue(L: *mut lua_State, idx: std::ffi::c_int);
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
        #[c2rust::src_loc = "154:1"]
        pub fn lua_tothread(L: *mut lua_State, idx: std::ffi::c_int) -> *mut lua_State;
        #[c2rust::src_loc = "161:1"]
        pub fn lua_pushnil(L: *mut lua_State);
        #[c2rust::src_loc = "162:1"]
        pub fn lua_pushnumber(L: *mut lua_State, n: lua_Number);
        #[c2rust::src_loc = "163:1"]
        pub fn lua_pushinteger(L: *mut lua_State, n: lua_Integer);
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
        #[c2rust::src_loc = "179:1"]
        pub fn lua_getfield(
            L: *mut lua_State,
            idx: std::ffi::c_int,
            k: *const std::ffi::c_char,
        );
        #[c2rust::src_loc = "181:1"]
        pub fn lua_rawgeti(L: *mut lua_State, idx: std::ffi::c_int, n: std::ffi::c_int);
        #[c2rust::src_loc = "191:1"]
        pub fn lua_settable(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "192:1"]
        pub fn lua_setfield(
            L: *mut lua_State,
            idx: std::ffi::c_int,
            k: *const std::ffi::c_char,
        );
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
        #[c2rust::src_loc = "241:1"]
        pub fn lua_next(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;
        #[c2rust::src_loc = "243:1"]
        pub fn lua_concat(L: *mut lua_State, n: std::ffi::c_int);
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/gio/giotypes.h:25"]
pub mod giotypes_h {
    #[c2rust::src_loc = "74:1"]
    pub type GFile = _GFile;
    extern "C" {
        #[c2rust::src_loc = "74:16"]
        pub type _GFile;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gmem.h:22"]
pub mod gmem_h {
    use super::gtypes_h::gpointer;
    extern "C" {
        #[c2rust::src_loc = "73:1"]
        pub fn g_free(mem: gpointer);
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gunicode.h:22"]
pub mod gunicode_h {
    use super::gtypes_h::{gchar, glong};
    use super::glibconfig_h::gssize;
    extern "C" {
        #[c2rust::src_loc = "848:1"]
        pub fn g_utf8_strlen(p: *const gchar, max: gssize) -> glong;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/util.h:22"]
pub mod util_h {
    use super::gtypes_h::gchar;
    extern "C" {
        #[c2rust::src_loc = "75:1"]
        pub fn strip_ansi_escapes(in_0: *const gchar) -> *mut gchar;
    }
}
#[c2rust::header_src = "/usr/include/luajit-2.1/lauxlib.h:22"]
pub mod lauxlib_h {
    use super::lua_h::{lua_State, lua_Number, lua_Integer};
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "30:1"]
        pub fn luaL_getmetafield(
            L: *mut lua_State,
            obj: std::ffi::c_int,
            e: *const std::ffi::c_char,
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "34:1"]
        pub fn luaL_checklstring(
            L: *mut lua_State,
            numArg: std::ffi::c_int,
            l: *mut size_t,
        ) -> *const std::ffi::c_char;
        #[c2rust::src_loc = "36:1"]
        pub fn luaL_optlstring(
            L: *mut lua_State,
            numArg: std::ffi::c_int,
            def: *const std::ffi::c_char,
            l: *mut size_t,
        ) -> *const std::ffi::c_char;
        #[c2rust::src_loc = "39:1"]
        pub fn luaL_optnumber(
            L: *mut lua_State,
            nArg: std::ffi::c_int,
            def: lua_Number,
        ) -> lua_Number;
        #[c2rust::src_loc = "41:1"]
        pub fn luaL_checkinteger(
            L: *mut lua_State,
            numArg: std::ffi::c_int,
        ) -> lua_Integer;
        #[c2rust::src_loc = "46:1"]
        pub fn luaL_checktype(
            L: *mut lua_State,
            narg: std::ffi::c_int,
            t: std::ffi::c_int,
        );
        #[c2rust::src_loc = "47:1"]
        pub fn luaL_checkany(L: *mut lua_State, narg: std::ffi::c_int);
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/luaclass.h:22"]
pub mod luaclass_h {
    use super::lua_h::lua_State;
    use super::gtypes_h::{gint, gchar};
    extern "C" {
        #[c2rust::src_loc = "62:1"]
        pub fn luaH_typename(_: *mut lua_State, _: gint) -> *const gchar;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/luautil.h:22"]
pub mod luautil_h {
    use super::lua_h::lua_State;
    use super::gtypes_h::gint;
    extern "C" {
        #[c2rust::src_loc = "25:1"]
        pub fn luaH_traceback(L: *mut lua_State, T: *mut lua_State, level: gint) -> gint;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/gio/gfile.h:25"]
pub mod gfile_h {
    use super::giotypes_h::GFile;
    extern "C" {
        #[c2rust::src_loc = "610:1"]
        pub fn g_file_new_for_path(path: *const std::ffi::c_char) -> *mut GFile;
        #[c2rust::src_loc = "658:1"]
        pub fn g_file_get_path(file: *mut GFile) -> *mut std::ffi::c_char;
    }
}
pub use self::__stddef_ptrdiff_t_h::ptrdiff_t;
pub use self::__stddef_size_t_h::size_t;
pub use self::glibconfig_h::gssize;
pub use self::types_h::__ssize_t;
pub use self::gtypes_h::{gchar, glong, gint, gpointer};
pub use self::sys_types_h::ssize_t;
pub use self::lua_h::{
    lua_CFunction, lua_Number, lua_Integer, lua_State, lua_gettop, lua_settop,
    lua_pushvalue, lua_remove, lua_insert, lua_type, lua_tolstring, lua_tothread,
    lua_pushnil, lua_pushnumber, lua_pushinteger, lua_pushlstring, lua_pushstring,
    lua_pushcclosure, lua_getfield, lua_rawgeti, lua_settable, lua_setfield, lua_call,
    lua_pcall, lua_next, lua_concat,
};
pub use self::giotypes_h::{GFile, _GFile};
use self::gmem_h::g_free;
use self::gunicode_h::g_utf8_strlen;
use self::util_h::strip_ansi_escapes;
use self::lauxlib_h::{
    luaL_getmetafield, luaL_checklstring, luaL_optlstring, luaL_optnumber,
    luaL_checkinteger, luaL_checktype, luaL_checkany,
};
use self::luaclass_h::luaH_typename;
use self::luautil_h::luaH_traceback;
use self::gfile_h::{g_file_new_for_path, g_file_get_path};
#[c2rust::src_loc = "30:1"]
unsafe extern "C" fn luaH_utf8_strlen(mut L: *mut lua_State) -> gint {
    let mut cmd: *const gchar = luaL_checklstring(
        L,
        1 as std::ffi::c_int,
        0 as *mut size_t,
    );
    lua_pushnumber(
        L,
        g_utf8_strlen(
            if !cmd.is_null() {
                cmd
            } else {
                b"\0" as *const u8 as *const std::ffi::c_char
            },
            -(1 as std::ffi::c_int) as gssize,
        ) as lua_Number,
    );
    return 1 as std::ffi::c_int;
}
#[c2rust::src_loc = "40:1"]
unsafe extern "C" fn luaHe_next(mut L: *mut lua_State) -> gint {
    if luaL_getmetafield(
        L,
        1 as std::ffi::c_int,
        b"__next\0" as *const u8 as *const std::ffi::c_char,
    ) != 0
    {
        lua_insert(L, 1 as std::ffi::c_int);
        lua_call(L, lua_gettop(L) - 1 as std::ffi::c_int, -(1 as std::ffi::c_int));
        return lua_gettop(L);
    }
    luaL_checktype(L, 1 as std::ffi::c_int, 5 as std::ffi::c_int);
    lua_settop(L, 2 as std::ffi::c_int);
    if lua_next(L, 1 as std::ffi::c_int) != 0 {
        return 2 as std::ffi::c_int;
    }
    lua_pushnil(L);
    return 1 as std::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "59:1"]
pub unsafe extern "C" fn luaH_mtnext(mut L: *mut lua_State, mut idx: gint) -> gint {
    if luaL_getmetafield(L, idx, b"__next\0" as *const u8 as *const std::ffi::c_char)
        != 0
    {
        if idx < 0 as std::ffi::c_int {
            idx -= 1;
            idx;
        }
        lua_pushvalue(L, idx);
        lua_pushvalue(L, -(3 as std::ffi::c_int));
        lua_remove(L, -(4 as std::ffi::c_int));
        lua_pcall(L, 2 as std::ffi::c_int, 2 as std::ffi::c_int, 0 as std::ffi::c_int);
        if lua_type(L, -(1 as std::ffi::c_int)) == 0 as std::ffi::c_int {
            lua_settop(L, -(2 as std::ffi::c_int) - 1 as std::ffi::c_int);
            return 0 as std::ffi::c_int;
        }
        return 1 as std::ffi::c_int;
    } else if lua_type(L, idx) == 5 as std::ffi::c_int {
        return lua_next(L, idx)
    }
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    return 0 as std::ffi::c_int;
}
#[c2rust::src_loc = "87:1"]
unsafe extern "C" fn luaH_generic_pairs(mut L: *mut lua_State) -> gint {
    lua_pushvalue(L, -(10002 as std::ffi::c_int) - 1 as std::ffi::c_int);
    lua_pushvalue(L, 1 as std::ffi::c_int);
    lua_pushnil(L);
    return 3 as std::ffi::c_int;
}
#[c2rust::src_loc = "98:1"]
unsafe extern "C" fn luaHe_pairs(mut L: *mut lua_State) -> gint {
    if luaL_getmetafield(
        L,
        1 as std::ffi::c_int,
        b"__pairs\0" as *const u8 as *const std::ffi::c_char,
    ) != 0
    {
        lua_insert(L, 1 as std::ffi::c_int);
        lua_call(L, lua_gettop(L) - 1 as std::ffi::c_int, -(1 as std::ffi::c_int));
        return lua_gettop(L);
    }
    luaL_checktype(L, 1 as std::ffi::c_int, 5 as std::ffi::c_int);
    return luaH_generic_pairs(L);
}
#[c2rust::src_loc = "110:1"]
unsafe extern "C" fn luaH_ipairs_aux(mut L: *mut lua_State) -> gint {
    let mut i: gint = luaL_checkinteger(L, 2 as std::ffi::c_int) as std::ffi::c_int
        + 1 as std::ffi::c_int;
    luaL_checktype(L, 1 as std::ffi::c_int, 5 as std::ffi::c_int);
    lua_pushinteger(L, i as lua_Integer);
    lua_rawgeti(L, 1 as std::ffi::c_int, i);
    return if lua_type(L, -(1 as std::ffi::c_int)) == 0 as std::ffi::c_int {
        0 as std::ffi::c_int
    } else {
        2 as std::ffi::c_int
    };
}
#[c2rust::src_loc = "122:1"]
unsafe extern "C" fn luaHe_ipairs(mut L: *mut lua_State) -> gint {
    if luaL_getmetafield(
        L,
        1 as std::ffi::c_int,
        b"__ipairs\0" as *const u8 as *const std::ffi::c_char,
    ) != 0
    {
        lua_insert(L, 1 as std::ffi::c_int);
        lua_call(L, lua_gettop(L) - 1 as std::ffi::c_int, -(1 as std::ffi::c_int));
        return lua_gettop(L);
    }
    luaL_checktype(L, 1 as std::ffi::c_int, 5 as std::ffi::c_int);
    lua_pushvalue(L, -(10002 as std::ffi::c_int) - 1 as std::ffi::c_int);
    lua_pushvalue(L, 1 as std::ffi::c_int);
    lua_pushinteger(L, 0 as std::ffi::c_int as lua_Integer);
    return 3 as std::ffi::c_int;
}
#[c2rust::src_loc = "142:1"]
unsafe extern "C" fn luaHe_type(mut L: *mut lua_State) -> gint {
    luaL_checkany(L, 1 as std::ffi::c_int);
    lua_pushstring(L, luaH_typename(L, 1 as std::ffi::c_int));
    return 1 as std::ffi::c_int;
}
#[c2rust::src_loc = "159:1"]
unsafe extern "C" fn luaH_abspath(mut L: *mut lua_State) -> gint {
    let mut path: *const gchar = luaL_checklstring(
        L,
        1 as std::ffi::c_int,
        0 as *mut size_t,
    );
    let mut file: *mut GFile = g_file_new_for_path(path);
    if file.is_null() {
        return 0 as std::ffi::c_int;
    }
    let mut absolute: *mut gchar = g_file_get_path(file);
    if absolute.is_null() {
        return 0 as std::ffi::c_int;
    }
    lua_pushstring(L, absolute);
    g_free(absolute as gpointer);
    return 1 as std::ffi::c_int;
}
#[c2rust::src_loc = "174:1"]
unsafe extern "C" fn luaH_debug_traceback(mut L: *mut lua_State) -> gint {
    let mut thread: *mut lua_State = 0 as *mut lua_State;
    thread = lua_tothread(L, 1 as std::ffi::c_int);
    if !thread.is_null() {
        lua_remove(L, 1 as std::ffi::c_int);
    }
    let mut msg: *const gchar = luaL_optlstring(
        L,
        1 as std::ffi::c_int,
        0 as *const std::ffi::c_char,
        0 as *mut size_t,
    );
    let mut level: std::ffi::c_int = luaL_optnumber(
        L,
        if !msg.is_null() { 2 as std::ffi::c_int } else { 1 as std::ffi::c_int },
        1 as std::ffi::c_int as lua_Number,
    ) as std::ffi::c_int;
    lua_pushstring(
        L,
        if !msg.is_null() { msg } else { b"\0" as *const u8 as *const std::ffi::c_char },
    );
    lua_pushstring(
        L,
        if !msg.is_null() {
            b"\nTraceback:\n\0" as *const u8 as *const std::ffi::c_char
        } else {
            b"Traceback:\n\0" as *const u8 as *const std::ffi::c_char
        },
    );
    luaH_traceback(L, if !thread.is_null() { thread } else { L }, level);
    let mut stripped: *mut gchar = strip_ansi_escapes(
        lua_tolstring(L, -(1 as std::ffi::c_int), 0 as *mut size_t),
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    lua_pushstring(L, stripped);
    lua_concat(L, 3 as std::ffi::c_int);
    g_free(stripped as gpointer);
    return 1 as std::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "195:1"]
pub unsafe extern "C" fn luaH_fixups(mut L: *mut lua_State) {
    lua_getfield(
        L,
        -(10002 as std::ffi::c_int),
        b"string\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_pushcclosure(
        L,
        Some(luaH_utf8_strlen as unsafe extern "C" fn(*mut lua_State) -> gint),
        0 as std::ffi::c_int,
    );
    lua_setfield(
        L,
        -(2 as std::ffi::c_int),
        b"wlen\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    lua_getfield(
        L,
        -(10002 as std::ffi::c_int),
        b"os\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_pushcclosure(
        L,
        Some(luaH_abspath as unsafe extern "C" fn(*mut lua_State) -> gint),
        0 as std::ffi::c_int,
    );
    lua_setfield(
        L,
        -(2 as std::ffi::c_int),
        b"abspath\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    lua_pushlstring(
        L,
        b"next\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 5]>() as std::ffi::c_ulong)
            .wrapping_div(
                ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
            )
            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
    );
    lua_pushcclosure(
        L,
        Some(luaHe_next as unsafe extern "C" fn(*mut lua_State) -> gint),
        0 as std::ffi::c_int,
    );
    lua_settable(L, -(10002 as std::ffi::c_int));
    lua_pushlstring(
        L,
        b"pairs\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 6]>() as std::ffi::c_ulong)
            .wrapping_div(
                ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
            )
            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
    );
    lua_pushcclosure(
        L,
        Some(luaHe_next as unsafe extern "C" fn(*mut lua_State) -> gint),
        0 as std::ffi::c_int,
    );
    lua_pushcclosure(
        L,
        Some(luaHe_pairs as unsafe extern "C" fn(*mut lua_State) -> gint),
        1 as std::ffi::c_int,
    );
    lua_settable(L, -(10002 as std::ffi::c_int));
    lua_pushlstring(
        L,
        b"ipairs\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 7]>() as std::ffi::c_ulong)
            .wrapping_div(
                ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
            )
            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
    );
    lua_pushcclosure(
        L,
        Some(luaH_ipairs_aux as unsafe extern "C" fn(*mut lua_State) -> gint),
        0 as std::ffi::c_int,
    );
    lua_pushcclosure(
        L,
        Some(luaHe_ipairs as unsafe extern "C" fn(*mut lua_State) -> gint),
        1 as std::ffi::c_int,
    );
    lua_settable(L, -(10002 as std::ffi::c_int));
    lua_pushlstring(
        L,
        b"type\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 5]>() as std::ffi::c_ulong)
            .wrapping_div(
                ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
            )
            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
    );
    lua_pushcclosure(
        L,
        Some(luaHe_type as unsafe extern "C" fn(*mut lua_State) -> gint),
        0 as std::ffi::c_int,
    );
    lua_settable(L, -(10002 as std::ffi::c_int));
    lua_getfield(
        L,
        -(10002 as std::ffi::c_int),
        b"debug\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_pushcclosure(
        L,
        Some(luaH_debug_traceback as unsafe extern "C" fn(*mut lua_State) -> gint),
        0 as std::ffi::c_int,
    );
    lua_setfield(
        L,
        -(2 as std::ffi::c_int),
        b"traceback\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
}
