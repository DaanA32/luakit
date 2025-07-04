use ::libc;
#[c2rust::header_src = "/usr/lib/clang/20/include/__stddef_ptrdiff_t.h:21"]
pub mod __stddef_ptrdiff_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type ptrdiff_t = std::ffi::c_long;
}
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
    #[c2rust::src_loc = "104:1"]
    pub type lua_Integer = ptrdiff_t;
    use super::__stddef_ptrdiff_t_h::ptrdiff_t;
    extern "C" {
        #[c2rust::src_loc = "51:16"]
        pub type lua_State;
        #[c2rust::src_loc = "122:1"]
        pub fn lua_settop(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "161:1"]
        pub fn lua_pushnil(L: *mut lua_State);
        #[c2rust::src_loc = "163:1"]
        pub fn lua_pushinteger(L: *mut lua_State, n: lua_Integer);
        #[c2rust::src_loc = "165:1"]
        pub fn lua_pushstring(L: *mut lua_State, s: *const std::ffi::c_char);
        #[c2rust::src_loc = "179:1"]
        pub fn lua_getfield(
            L: *mut lua_State,
            idx: std::ffi::c_int,
            k: *const std::ffi::c_char,
        );
        #[c2rust::src_loc = "192:1"]
        pub fn lua_setfield(
            L: *mut lua_State,
            idx: std::ffi::c_int,
            k: *const std::ffi::c_char,
        );
    }
}
#[c2rust::header_src = "/usr/include/luajit-2.1/lauxlib.h:22"]
pub mod lauxlib_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "21:16"]
    pub struct luaL_Reg {
        pub name: *const std::ffi::c_char,
        pub func: lua_CFunction,
    }
    use super::lua_h::{lua_CFunction, lua_State, lua_Integer};
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "33:1"]
        pub fn luaL_argerror(
            L: *mut lua_State,
            numarg: std::ffi::c_int,
            extramsg: *const std::ffi::c_char,
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "34:1"]
        pub fn luaL_checklstring(
            L: *mut lua_State,
            numArg: std::ffi::c_int,
            l: *mut size_t,
        ) -> *const std::ffi::c_char;
        #[c2rust::src_loc = "41:1"]
        pub fn luaL_checkinteger(
            L: *mut lua_State,
            numArg: std::ffi::c_int,
        ) -> lua_Integer;
        #[c2rust::src_loc = "42:1"]
        pub fn luaL_optinteger(
            L: *mut lua_State,
            nArg: std::ffi::c_int,
            def: lua_Integer,
        ) -> lua_Integer;
        #[c2rust::src_loc = "53:1"]
        pub fn luaL_error(
            L: *mut lua_State,
            fmt: *const std::ffi::c_char,
            _: ...
        ) -> std::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtypes.h:22"]
pub mod gtypes_h {
    #[c2rust::src_loc = "55:1"]
    pub type gint = std::ffi::c_int;
    #[c2rust::src_loc = "52:1"]
    pub type gchar = std::ffi::c_char;
    #[c2rust::src_loc = "54:1"]
    pub type glong = std::ffi::c_long;
    #[c2rust::src_loc = "56:1"]
    pub type gboolean = gint;
}
#[c2rust::header_src = "/usr/include/sys/types.h:22"]
pub mod sys_types_h {
    #[c2rust::src_loc = "108:1"]
    pub type ssize_t = __ssize_t;
    use super::types_h::__ssize_t;
}
#[c2rust::header_src = "/usr/include/bits/types.h:22"]
pub mod types_h {
    #[c2rust::src_loc = "194:1"]
    pub type __ssize_t = std::ffi::c_long;
}
#[c2rust::header_src = "/usr/lib/glib-2.0/include/glibconfig.h:22"]
pub mod glibconfig_h {
    #[c2rust::src_loc = "82:1"]
    pub type gssize = std::ffi::c_long;
    #[c2rust::src_loc = "57:1"]
    pub type guint32 = std::ffi::c_uint;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gunicode.h:22"]
pub mod gunicode_h {
    #[c2rust::src_loc = "61:1"]
    pub type gunichar = guint32;
    use super::glibconfig_h::{guint32, gssize};
    use super::gtypes_h::{gchar, glong, gboolean};
    extern "C" {
        #[c2rust::src_loc = "829:1"]
        pub fn g_utf8_get_char_validated(p: *const gchar, max_len: gssize) -> gunichar;
        #[c2rust::src_loc = "833:1"]
        pub fn g_utf8_offset_to_pointer(str: *const gchar, offset: glong) -> *mut gchar;
        #[c2rust::src_loc = "841:1"]
        pub fn g_utf8_find_next_char(p: *const gchar, end: *const gchar) -> *mut gchar;
        #[c2rust::src_loc = "848:1"]
        pub fn g_utf8_strlen(p: *const gchar, max: gssize) -> glong;
        #[c2rust::src_loc = "925:1"]
        pub fn g_utf8_validate(
            str: *const gchar,
            max_len: gssize,
            end: *mut *const gchar,
        ) -> gboolean;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/luaclass.h:22"]
pub mod luaclass_h {
    use super::lua_h::lua_State;
    use super::gtypes_h::gchar;
    use super::lauxlib_h::luaL_Reg;
    extern "C" {
        #[c2rust::src_loc = "74:1"]
        pub fn luaH_openlib(
            _: *mut lua_State,
            _: *const gchar,
            _: *const luaL_Reg,
            _: *const luaL_Reg,
        );
    }
}
pub use self::__stddef_ptrdiff_t_h::ptrdiff_t;
pub use self::__stddef_size_t_h::size_t;
pub use self::lua_h::{
    lua_CFunction, lua_Integer, lua_State, lua_settop, lua_pushnil, lua_pushinteger,
    lua_pushstring, lua_getfield, lua_setfield,
};
pub use self::lauxlib_h::{
    luaL_Reg, luaL_argerror, luaL_checklstring, luaL_checkinteger, luaL_optinteger,
    luaL_error,
};
pub use self::gtypes_h::{gint, gchar, glong, gboolean};
pub use self::sys_types_h::ssize_t;
pub use self::types_h::__ssize_t;
pub use self::glibconfig_h::{gssize, guint32};
pub use self::gunicode_h::{
    gunichar, g_utf8_get_char_validated, g_utf8_offset_to_pointer, g_utf8_find_next_char,
    g_utf8_strlen, g_utf8_validate,
};
use self::luaclass_h::luaH_openlib;
#[c2rust::src_loc = "29:1"]
unsafe extern "C" fn abspos(mut offset: ssize_t, mut length: size_t) -> size_t {
    if offset == 0 as std::ffi::c_int as ssize_t {
        return -(1 as std::ffi::c_int) as size_t;
    }
    offset = if offset > 0 as std::ffi::c_int as ssize_t {
        offset - 1 as std::ffi::c_int as ssize_t
    } else {
        offset + length as ssize_t
    };
    if offset < 0 as std::ffi::c_int as ssize_t || offset as size_t > length {
        return -(1 as std::ffi::c_int) as size_t;
    }
    return offset as size_t;
}
#[c2rust::src_loc = "41:1"]
unsafe extern "C" fn luaH_utf8_len(mut L: *mut lua_State) -> gint {
    let mut blen: size_t = 0;
    let mut str: *const gchar = luaL_checklstring(L, 1 as std::ffi::c_int, &mut blen);
    let mut bbeg: size_t = abspos(
        luaL_optinteger(L, 2 as std::ffi::c_int, 1 as std::ffi::c_int as lua_Integer),
        blen,
    );
    (bbeg != -(1 as std::ffi::c_int) as size_t
        || luaL_argerror(
            L,
            2 as std::ffi::c_int,
            b"initial position out of string\0" as *const u8 as *const std::ffi::c_char,
        ) != 0) as std::ffi::c_int;
    let mut bend: size_t = bbeg;
    let mut sbend: ssize_t = luaL_optinteger(
        L,
        3 as std::ffi::c_int,
        blen as lua_Integer,
    );
    sbend = if sbend >= 0 as std::ffi::c_int as ssize_t {
        sbend - 1 as std::ffi::c_int as ssize_t
    } else {
        sbend + blen as ssize_t
    };
    (sbend < blen as ssize_t
        || luaL_argerror(
            L,
            3 as std::ffi::c_int,
            b"final position out of string\0" as *const u8 as *const std::ffi::c_char,
        ) != 0) as std::ffi::c_int;
    if sbend >= bbeg as ssize_t && (sbend as size_t) < blen {
        bend = (g_utf8_find_next_char(
            str.offset(sbend as size_t as isize),
            0 as *const gchar,
        ))
            .offset_from(str) as std::ffi::c_long as size_t;
    }
    let mut valend: *mut gchar = 0 as *mut gchar;
    if g_utf8_validate(
        str.offset(bbeg as isize),
        bend.wrapping_sub(bbeg) as gssize,
        &mut valend as *mut *mut gchar as *mut *const gchar,
    ) == 0
    {
        lua_pushnil(L);
        lua_pushinteger(
            L,
            valend.offset_from(str) as std::ffi::c_long + 1 as std::ffi::c_int as ssize_t,
        );
        return 2 as std::ffi::c_int;
    }
    lua_pushinteger(
        L,
        g_utf8_strlen(str.offset(bbeg as isize), bend.wrapping_sub(bbeg) as gssize),
    );
    return 1 as std::ffi::c_int;
}
#[c2rust::src_loc = "74:1"]
unsafe extern "C" fn luaH_utf8_offset(mut L: *mut lua_State) -> gint {
    let mut blen: size_t = 0;
    let mut str: *const gchar = luaL_checklstring(L, 1 as std::ffi::c_int, &mut blen);
    let mut widx: ssize_t = luaL_checkinteger(L, 2 as std::ffi::c_int);
    if widx > 0 as std::ffi::c_int as ssize_t {
        widx -= 1;
        widx;
    }
    let mut bbase: size_t = 0;
    bbase = luaL_optinteger(
        L,
        3 as std::ffi::c_int,
        (if widx >= 0 as std::ffi::c_int as ssize_t {
            1 as std::ffi::c_int as size_t
        } else {
            blen.wrapping_add(1 as std::ffi::c_int as size_t)
        }) as lua_Integer,
    ) as size_t;
    bbase = abspos(bbase as ssize_t, blen);
    (bbase != -(1 as std::ffi::c_int) as size_t
        || luaL_argerror(
            L,
            3 as std::ffi::c_int,
            b"position out of range\0" as *const u8 as *const std::ffi::c_char,
        ) != 0) as std::ffi::c_int;
    if g_utf8_get_char_validated(
        str.offset(bbase as isize),
        -(1 as std::ffi::c_int) as gssize,
    ) == -(1 as std::ffi::c_int) as gunichar
    {
        luaL_error(
            L,
            b"initial position is a continuation byte\0" as *const u8
                as *const std::ffi::c_char,
        );
    }
    let mut wseglen: size_t = 0;
    let mut bbeg: size_t = 0 as std::ffi::c_int as size_t;
    if widx < 0 as std::ffi::c_int as ssize_t {
        wseglen = g_utf8_strlen(str, bbase as gssize) as size_t;
        widx = (widx as size_t).wrapping_add(wseglen) as ssize_t as ssize_t;
    } else {
        wseglen = g_utf8_strlen(
            str.offset(bbase as isize),
            blen.wrapping_sub(bbase) as gssize,
        ) as size_t;
        bbeg = bbase;
    }
    let mut ret: ssize_t = 0 as std::ffi::c_int as ssize_t;
    if widx >= 0 as std::ffi::c_int as ssize_t && widx as size_t <= wseglen {
        let mut pos: *mut gchar = g_utf8_offset_to_pointer(
            str.offset(bbeg as isize),
            widx,
        );
        if !pos.is_null() {
            ret = pos.offset_from(str) as std::ffi::c_long
                + 1 as std::ffi::c_int as ssize_t;
        }
    }
    if ret > 0 as std::ffi::c_int as ssize_t {
        lua_pushinteger(L, ret);
    } else {
        lua_pushnil(L);
    }
    return 1 as std::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "119:1"]
pub unsafe extern "C" fn utf8_lib_setup(mut L: *mut lua_State) {
    static mut utf8_lib: [luaL_Reg; 3] = unsafe {
        [
            {
                let mut init = luaL_Reg {
                    name: b"len\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_utf8_len as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"offset\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_utf8_offset as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: 0 as *const std::ffi::c_char,
                    func: None,
                };
                init
            },
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
