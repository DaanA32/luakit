use ::libc;
#[c2rust::header_src = "/usr/lib/clang/20/include/__stddef_size_t.h:19"]
pub mod __stddef_size_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type size_t = std::ffi::c_ulong;
}
#[c2rust::header_src = "/usr/include/luajit-2.1/lua.h:19"]
pub mod lua_h {
    #[c2rust::src_loc = "59:1"]
    pub type lua_Reader = Option::<
        unsafe extern "C" fn(
            *mut lua_State,
            *mut std::ffi::c_void,
            *mut size_t,
        ) -> *const std::ffi::c_char,
    >;
    #[c2rust::src_loc = "61:1"]
    pub type lua_Writer = Option::<
        unsafe extern "C" fn(
            *mut lua_State,
            *const std::ffi::c_void,
            size_t,
            *mut std::ffi::c_void,
        ) -> std::ffi::c_int,
    >;
    #[c2rust::src_loc = "100:1"]
    pub type lua_Number = std::ffi::c_double;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "360:8"]
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
        #[c2rust::src_loc = "140:1"]
        pub fn lua_type(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;
        #[c2rust::src_loc = "141:1"]
        pub fn lua_typename(
            L: *mut lua_State,
            tp: std::ffi::c_int,
        ) -> *const std::ffi::c_char;
        #[c2rust::src_loc = "147:1"]
        pub fn lua_tonumber(L: *mut lua_State, idx: std::ffi::c_int) -> lua_Number;
        #[c2rust::src_loc = "149:1"]
        pub fn lua_toboolean(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;
        #[c2rust::src_loc = "150:1"]
        pub fn lua_tolstring(
            L: *mut lua_State,
            idx: std::ffi::c_int,
            len: *mut size_t,
        ) -> *const std::ffi::c_char;
        #[c2rust::src_loc = "153:1"]
        pub fn lua_touserdata(
            L: *mut lua_State,
            idx: std::ffi::c_int,
        ) -> *mut std::ffi::c_void;
        #[c2rust::src_loc = "161:1"]
        pub fn lua_pushnil(L: *mut lua_State);
        #[c2rust::src_loc = "162:1"]
        pub fn lua_pushnumber(L: *mut lua_State, n: lua_Number);
        #[c2rust::src_loc = "164:1"]
        pub fn lua_pushlstring(L: *mut lua_State, s: *const std::ffi::c_char, l: size_t);
        #[c2rust::src_loc = "170:1"]
        pub fn lua_pushboolean(L: *mut lua_State, b: std::ffi::c_int);
        #[c2rust::src_loc = "171:1"]
        pub fn lua_pushlightuserdata(L: *mut lua_State, p: *mut std::ffi::c_void);
        #[c2rust::src_loc = "182:1"]
        pub fn lua_createtable(
            L: *mut lua_State,
            narr: std::ffi::c_int,
            nrec: std::ffi::c_int,
        );
        #[c2rust::src_loc = "193:1"]
        pub fn lua_rawset(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "205:1"]
        pub fn lua_load(
            L: *mut lua_State,
            reader: lua_Reader,
            dt: *mut std::ffi::c_void,
            chunkname: *const std::ffi::c_char,
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "208:1"]
        pub fn lua_dump(
            L: *mut lua_State,
            writer: lua_Writer,
            data: *mut std::ffi::c_void,
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "241:1"]
        pub fn lua_next(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;
        #[c2rust::src_loc = "336:1"]
        pub fn lua_getinfo(
            L: *mut lua_State,
            what: *const std::ffi::c_char,
            ar: *mut lua_Debug,
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "339:1"]
        pub fn lua_getupvalue(
            L: *mut lua_State,
            funcindex: std::ffi::c_int,
            n: std::ffi::c_int,
        ) -> *const std::ffi::c_char;
        #[c2rust::src_loc = "340:1"]
        pub fn lua_setupvalue(
            L: *mut lua_State,
            funcindex: std::ffi::c_int,
            n: std::ffi::c_int,
        ) -> *const std::ffi::c_char;
    }
}
#[c2rust::header_src = "/usr/lib/glib-2.0/include/glibconfig.h:19"]
pub mod glibconfig_h {
    #[c2rust::src_loc = "45:1"]
    pub type gint8 = std::ffi::c_schar;
    #[c2rust::src_loc = "46:1"]
    pub type guint8 = std::ffi::c_uchar;
    #[c2rust::src_loc = "66:1"]
    pub type gint64 = std::ffi::c_long;
    #[c2rust::src_loc = "67:1"]
    pub type guint64 = std::ffi::c_ulong;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtypes.h:19"]
pub mod gtypes_h {
    #[c2rust::src_loc = "52:1"]
    pub type gchar = std::ffi::c_char;
    #[c2rust::src_loc = "55:1"]
    pub type gint = std::ffi::c_int;
    #[c2rust::src_loc = "61:1"]
    pub type guint = std::ffi::c_uint;
    #[c2rust::src_loc = "109:1"]
    pub type gpointer = *mut std::ffi::c_void;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/garray.h:19"]
pub mod garray_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:8"]
    pub struct _GByteArray {
        pub data: *mut guint8,
        pub len: guint,
    }
    #[c2rust::src_loc = "40:1"]
    pub type GByteArray = _GByteArray;
    use super::glibconfig_h::guint8;
    use super::gtypes_h::guint;
    extern "C" {
        #[c2rust::src_loc = "273:1"]
        pub fn g_byte_array_new() -> *mut GByteArray;
        #[c2rust::src_loc = "292:1"]
        pub fn g_byte_array_append(
            array: *mut GByteArray,
            data: *const guint8,
            len: guint,
        ) -> *mut GByteArray;
        #[c2rust::src_loc = "300:1"]
        pub fn g_byte_array_set_size(
            array: *mut GByteArray,
            length: guint,
        ) -> *mut GByteArray;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/log.h:20"]
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
#[c2rust::header_src = "/usr/include/string.h:19"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "43:14"]
        pub fn memcpy(
            _: *mut std::ffi::c_void,
            _: *const std::ffi::c_void,
            _: std::ffi::c_ulong,
        ) -> *mut std::ffi::c_void;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtestutils.h:19"]
pub mod gtestutils_h {
    use super::glibconfig_h::guint64;
    extern "C" {
        #[c2rust::src_loc = "650:1"]
        pub fn g_assertion_message_cmpint(
            domain: *const std::ffi::c_char,
            file: *const std::ffi::c_char,
            line: std::ffi::c_int,
            func: *const std::ffi::c_char,
            expr: *const std::ffi::c_char,
            arg1: guint64,
            cmp: *const std::ffi::c_char,
            arg2: guint64,
            numtype: std::ffi::c_char,
        );
    }
}
#[c2rust::header_src = "/usr/include/luajit-2.1/lauxlib.h:20"]
pub mod lauxlib_h {
    use super::lua_h::lua_State;
    extern "C" {
        #[c2rust::src_loc = "53:1"]
        pub fn luaL_error(
            L: *mut lua_State,
            fmt: *const std::ffi::c_char,
            _: ...
        ) -> std::ffi::c_int;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/lualib.h:20"]
pub mod lualib_h {
    #[inline]
    #[c2rust::src_loc = "90:1"]
    pub unsafe extern "C" fn luaH_absindex(mut L: *mut lua_State, mut ud: gint) -> gint {
        return if ud >= 0 as std::ffi::c_int || ud <= -(10000 as std::ffi::c_int) {
            ud
        } else {
            lua_gettop(L) + ud + 1 as std::ffi::c_int
        };
    }
    use super::lua_h::{lua_State, lua_gettop};
    use super::gtypes_h::gint;
}
pub use self::__stddef_size_t_h::size_t;
pub use self::lua_h::{
    lua_Reader, lua_Writer, lua_Number, lua_Debug, lua_State, lua_gettop, lua_settop,
    lua_pushvalue, lua_type, lua_typename, lua_tonumber, lua_toboolean, lua_tolstring,
    lua_touserdata, lua_pushnil, lua_pushnumber, lua_pushlstring, lua_pushboolean,
    lua_pushlightuserdata, lua_createtable, lua_rawset, lua_load, lua_dump, lua_next,
    lua_getinfo, lua_getupvalue, lua_setupvalue,
};
pub use self::glibconfig_h::{gint8, guint8, gint64, guint64};
pub use self::gtypes_h::{gchar, gint, guint, gpointer};
pub use self::garray_h::{
    _GByteArray, GByteArray, g_byte_array_new, g_byte_array_append, g_byte_array_set_size,
};
pub use self::log_h::{
    log_level_t, LOG_LEVEL_debug, LOG_LEVEL_verbose, LOG_LEVEL_info, LOG_LEVEL_warn,
    LOG_LEVEL_error, LOG_LEVEL_fatal, _log,
};
use self::string_h::memcpy;
use self::gtestutils_h::g_assertion_message_cmpint;
use self::lauxlib_h::luaL_error;
pub use self::lualib_h::luaH_absindex;
#[c2rust::src_loc = "24:20"]
static mut bytecode_buf: *mut GByteArray = 0 as *const GByteArray as *mut GByteArray;
#[c2rust::src_loc = "25:15"]
static mut bytecode_len: size_t = 0;
#[c2rust::src_loc = "27:1"]
unsafe extern "C" fn lua_function_writer(
    mut UNUSED_L: *mut lua_State,
    mut p: *const std::ffi::c_void,
    mut sz: size_t,
    mut UNUSED_ud: *mut std::ffi::c_void,
) -> std::ffi::c_int {
    g_byte_array_append(bytecode_buf, p as *mut guint8, sz as guint);
    return 0 as std::ffi::c_int;
}
#[c2rust::src_loc = "34:1"]
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
#[c2rust::src_loc = "45:1"]
unsafe extern "C" fn lua_serialize_value(
    mut L: *mut lua_State,
    mut out: *mut GByteArray,
    mut index: std::ffi::c_int,
) {
    let mut type_0: gint8 = lua_type(L, index) as gint8;
    let mut top: std::ffi::c_int = lua_gettop(L);
    match type_0 as std::ffi::c_int {
        7 | 8 => {
            luaL_error(
                L,
                b"cannot serialize variable of type %s\0" as *const u8
                    as *const std::ffi::c_char,
                lua_typename(L, type_0 as std::ffi::c_int),
            );
            return;
        }
        _ => {}
    }
    g_byte_array_append(
        out,
        &mut type_0 as *mut gint8 as *mut guint8,
        ::core::mem::size_of::<gint8>() as std::ffi::c_ulong as guint,
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
            let mut b: gint8 = lua_toboolean(L, index) as gint8;
            g_byte_array_append(
                out,
                &mut b as *mut gint8 as *mut guint8,
                ::core::mem::size_of::<gint8>() as std::ffi::c_ulong as guint,
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
            let mut end: gint8 = -(1 as std::ffi::c_int) as gint8;
            g_byte_array_append(
                out,
                &mut end as *mut gint8 as *mut guint8,
                ::core::mem::size_of::<gint8>() as std::ffi::c_ulong as guint,
            );
        }
        2 => {
            let mut p: gpointer = lua_touserdata(L, index);
            if p.is_null() {
                _log(
                    LOG_LEVEL_warn,
                    b"common/luaserialize.c\0" as *const u8 as *const std::ffi::c_char,
                    b"serialize lua lightuserdata on non object\0" as *const u8
                        as *const std::ffi::c_char,
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
            lua_dump(
                L,
                Some(
                    lua_function_writer
                        as unsafe extern "C" fn(
                            *mut lua_State,
                            *const std::ffi::c_void,
                            size_t,
                            *mut std::ffi::c_void,
                        ) -> std::ffi::c_int,
                ),
                0 as *mut std::ffi::c_void,
            );
            lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
            let mut len_0: size_t = (*bytecode_buf).len as size_t;
            g_byte_array_append(
                out,
                &mut len_0 as *mut size_t as *mut guint8,
                ::core::mem::size_of::<size_t>() as std::ffi::c_ulong as guint,
            );
            g_byte_array_append(out, (*bytecode_buf).data, len_0 as guint);
            g_byte_array_set_size(bytecode_buf, 0 as std::ffi::c_int as guint);
            let mut ar: lua_Debug = lua_Debug {
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
            lua_pushvalue(L, index);
            lua_getinfo(L, b">u\0" as *const u8 as *const std::ffi::c_char, &mut ar);
            g_byte_array_append(
                out,
                &mut ar.nups as *mut std::ffi::c_int as *mut guint8,
                ::core::mem::size_of::<std::ffi::c_int>() as std::ffi::c_ulong as guint,
            );
            let mut i: std::ffi::c_int = 1 as std::ffi::c_int;
            while i <= ar.nups {
                lua_getupvalue(L, -(1 as std::ffi::c_int), i);
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
        g_assertion_message_cmpint(
            0 as *mut gchar,
            b"common/luaserialize.c\0" as *const u8 as *const std::ffi::c_char,
            130 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 20],
                &[std::ffi::c_char; 20],
            >(b"lua_serialize_value\0"))
                .as_ptr(),
            b"lua_gettop(L) == top\0" as *const u8 as *const std::ffi::c_char,
            __n1 as guint64,
            b"==\0" as *const u8 as *const std::ffi::c_char,
            __n2 as guint64,
            'i' as i32 as std::ffi::c_char,
        );
    }
}
#[c2rust::src_loc = "133:1"]
unsafe extern "C" fn lua_deserialize_value(
    mut L: *mut lua_State,
    mut bytes: *mut *const guint8,
) -> std::ffi::c_int {
    let mut type_0: gint8 = 0;
    memcpy(
        &mut type_0 as *mut gint8 as *mut std::ffi::c_void,
        *bytes as *const std::ffi::c_void,
        ::core::mem::size_of::<gint8>() as std::ffi::c_ulong,
    );
    *bytes = (*bytes)
        .offset(::core::mem::size_of::<gint8>() as std::ffi::c_ulong as isize);
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
                ::core::mem::size_of::<lua_Number>() as std::ffi::c_ulong,
            );
            *bytes = (*bytes)
                .offset(
                    ::core::mem::size_of::<lua_Number>() as std::ffi::c_ulong as isize,
                );
            lua_pushnumber(L, n);
        }
        1 => {
            let mut b: gint8 = 0;
            memcpy(
                &mut b as *mut gint8 as *mut std::ffi::c_void,
                *bytes as *const std::ffi::c_void,
                ::core::mem::size_of::<gint8>() as std::ffi::c_ulong,
            );
            *bytes = (*bytes)
                .offset(::core::mem::size_of::<gint8>() as std::ffi::c_ulong as isize);
            lua_pushboolean(L, b as std::ffi::c_int);
        }
        4 => {
            let mut len: size_t = 0;
            memcpy(
                &mut len as *mut size_t as *mut std::ffi::c_void,
                *bytes as *const std::ffi::c_void,
                ::core::mem::size_of::<size_t>() as std::ffi::c_ulong,
            );
            *bytes = (*bytes)
                .offset(::core::mem::size_of::<size_t>() as std::ffi::c_ulong as isize);
            lua_pushlstring(L, *bytes as *mut std::ffi::c_char, len);
            *bytes = (*bytes)
                .offset(len.wrapping_add(1 as std::ffi::c_int as size_t) as isize);
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
                ::core::mem::size_of::<gpointer>() as std::ffi::c_ulong,
            );
            *bytes = (*bytes)
                .offset(
                    ::core::mem::size_of::<gpointer>() as std::ffi::c_ulong as isize,
                );
            lua_pushlightuserdata(L, p);
        }
        6 => {
            memcpy(
                &mut bytecode_len as *mut size_t as *mut std::ffi::c_void,
                *bytes as *const std::ffi::c_void,
                ::core::mem::size_of::<size_t>() as std::ffi::c_ulong,
            );
            *bytes = (*bytes)
                .offset(::core::mem::size_of::<size_t>() as std::ffi::c_ulong as isize);
            let mut status: std::ffi::c_int = lua_load(
                L,
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut lua_State,
                            *mut *const guint8,
                            *mut size_t,
                        ) -> *const std::ffi::c_char,
                    >,
                    lua_Reader,
                >(
                    Some(
                        lua_function_reader
                            as unsafe extern "C" fn(
                                *mut lua_State,
                                *mut *const guint8,
                                *mut size_t,
                            ) -> *const std::ffi::c_char,
                    ),
                ),
                bytes as *mut std::ffi::c_void,
                0 as *const std::ffi::c_char,
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
                ::core::mem::size_of::<std::ffi::c_int>() as std::ffi::c_ulong,
            );
            *bytes = (*bytes)
                .offset(
                    ::core::mem::size_of::<std::ffi::c_int>() as std::ffi::c_ulong
                        as isize,
                );
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
            (*::core::mem::transmute::<
                &[u8; 22],
                &[std::ffi::c_char; 22],
            >(b"lua_deserialize_value\0"))
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
#[no_mangle]
#[c2rust::src_loc = "207:1"]
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
#[no_mangle]
#[c2rust::src_loc = "217:1"]
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
