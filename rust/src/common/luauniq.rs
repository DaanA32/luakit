use ::libc;
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtypes.h:19"]
pub mod gtypes_h {
    #[c2rust::src_loc = "52:1"]
    pub type gchar = std::ffi::c_char;
    #[c2rust::src_loc = "109:1"]
    pub type gpointer = *mut std::ffi::c_void;
}
#[c2rust::header_src = "/usr/include/luajit-2.1/lua.h:19"]
pub mod lua_h {
    extern "C" {
        #[c2rust::src_loc = "51:16"]
        pub type lua_State;
        #[c2rust::src_loc = "122:1"]
        pub fn lua_settop(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "123:1"]
        pub fn lua_pushvalue(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "124:1"]
        pub fn lua_remove(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "140:1"]
        pub fn lua_type(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;
        #[c2rust::src_loc = "161:1"]
        pub fn lua_pushnil(L: *mut lua_State);
        #[c2rust::src_loc = "165:1"]
        pub fn lua_pushstring(L: *mut lua_State, s: *const std::ffi::c_char);
        #[c2rust::src_loc = "171:1"]
        pub fn lua_pushlightuserdata(L: *mut lua_State, p: *mut std::ffi::c_void);
        #[c2rust::src_loc = "180:1"]
        pub fn lua_rawget(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "182:1"]
        pub fn lua_createtable(
            L: *mut lua_State,
            narr: std::ffi::c_int,
            nrec: std::ffi::c_int,
        );
        #[c2rust::src_loc = "193:1"]
        pub fn lua_rawset(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "195:1"]
        pub fn lua_setmetatable(
            L: *mut lua_State,
            objindex: std::ffi::c_int,
        ) -> std::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtestutils.h:19"]
pub mod gtestutils_h {
    extern "C" {
        #[c2rust::src_loc = "624:1"]
        pub fn g_assertion_message_expr(
            domain: *const std::ffi::c_char,
            file: *const std::ffi::c_char,
            line: std::ffi::c_int,
            func: *const std::ffi::c_char,
            expr: *const std::ffi::c_char,
        ) -> !;
    }
}
pub use self::gtypes_h::{gchar, gpointer};
use self::lua_h::{
    lua_State, lua_settop, lua_pushvalue, lua_remove, lua_type, lua_pushnil,
    lua_pushstring, lua_pushlightuserdata, lua_rawget, lua_createtable, lua_rawset,
    lua_setmetatable,
};
use self::gtestutils_h::g_assertion_message_expr;
#[no_mangle]
#[c2rust::src_loc = "25:1"]
pub unsafe extern "C" fn luaH_uniq_setup(
    mut L: *mut lua_State,
    mut reg: *const gchar,
    mut mode: *const gchar,
) {
    lua_pushstring(
        L,
        if !reg.is_null() {
            reg
        } else {
            b"luakit.uniq.registry\0" as *const u8 as *const std::ffi::c_char
        },
    );
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_pushstring(L, b"__mode\0" as *const u8 as *const std::ffi::c_char);
    lua_pushstring(L, mode);
    lua_rawset(L, -(3 as std::ffi::c_int));
    lua_setmetatable(L, -(2 as std::ffi::c_int));
    lua_rawset(L, -(10000 as std::ffi::c_int));
}
#[no_mangle]
#[c2rust::src_loc = "46:1"]
pub unsafe extern "C" fn luaH_uniq_add(
    mut L: *mut lua_State,
    mut reg: *const gchar,
    mut k: std::ffi::c_int,
    mut oud: std::ffi::c_int,
) -> std::ffi::c_int {
    lua_pushstring(
        L,
        if !reg.is_null() {
            reg
        } else {
            b"luakit.uniq.registry\0" as *const u8 as *const std::ffi::c_char
        },
    );
    lua_rawget(L, -(10000 as std::ffi::c_int));
    lua_pushvalue(
        L,
        if k > 0 as std::ffi::c_int { k } else { k - 1 as std::ffi::c_int },
    );
    lua_rawget(L, -(2 as std::ffi::c_int));
    if lua_type(L, -(1 as std::ffi::c_int)) == 0 as std::ffi::c_int {} else {
        g_assertion_message_expr(
            0 as *mut gchar,
            b"common/luauniq.c\0" as *const u8 as *const std::ffi::c_char,
            56 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 14],
                &[std::ffi::c_char; 14],
            >(b"luaH_uniq_add\0"))
                .as_ptr(),
            b"lua_isnil(L, -1)\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    lua_pushvalue(
        L,
        if k > 0 as std::ffi::c_int { k } else { k - 1 as std::ffi::c_int },
    );
    lua_pushvalue(
        L,
        if oud < 0 as std::ffi::c_int { oud - 2 as std::ffi::c_int } else { oud },
    );
    lua_rawset(L, -(3 as std::ffi::c_int));
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    return 0 as std::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "69:1"]
pub unsafe extern "C" fn luaH_uniq_add_ptr(
    mut L: *mut lua_State,
    mut reg: *const gchar,
    mut key: gpointer,
    mut oud: std::ffi::c_int,
) -> std::ffi::c_int {
    lua_pushlightuserdata(L, key);
    luaH_uniq_add(
        L,
        reg,
        -(1 as std::ffi::c_int),
        if oud > 0 as std::ffi::c_int { oud } else { oud - 1 as std::ffi::c_int },
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    return 0 as std::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "80:1"]
pub unsafe extern "C" fn luaH_uniq_get(
    mut L: *mut lua_State,
    mut reg: *const gchar,
    mut k: std::ffi::c_int,
) -> std::ffi::c_int {
    lua_pushstring(
        L,
        if !reg.is_null() {
            reg
        } else {
            b"luakit.uniq.registry\0" as *const u8 as *const std::ffi::c_char
        },
    );
    lua_rawget(L, -(10000 as std::ffi::c_int));
    lua_pushvalue(
        L,
        if k > 0 as std::ffi::c_int { k } else { k - 1 as std::ffi::c_int },
    );
    lua_rawget(L, -(2 as std::ffi::c_int));
    lua_remove(L, -(2 as std::ffi::c_int));
    if lua_type(L, -(1 as std::ffi::c_int)) == 0 as std::ffi::c_int {
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        return 0 as std::ffi::c_int;
    }
    return 1 as std::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "102:1"]
pub unsafe extern "C" fn luaH_uniq_get_ptr(
    mut L: *mut lua_State,
    mut reg: *const gchar,
    mut key: gpointer,
) -> std::ffi::c_int {
    lua_pushlightuserdata(L, key);
    let mut n: std::ffi::c_int = luaH_uniq_get(L, reg, -(1 as std::ffi::c_int));
    lua_remove(L, -(1 as std::ffi::c_int) - n);
    return n;
}
#[no_mangle]
#[c2rust::src_loc = "111:1"]
pub unsafe extern "C" fn luaH_uniq_del(
    mut L: *mut lua_State,
    mut reg: *const gchar,
    mut k: std::ffi::c_int,
) {
    lua_pushstring(
        L,
        if !reg.is_null() {
            reg
        } else {
            b"luakit.uniq.registry\0" as *const u8 as *const std::ffi::c_char
        },
    );
    lua_rawget(L, -(10000 as std::ffi::c_int));
    lua_pushvalue(
        L,
        if k > 0 as std::ffi::c_int { k } else { k - 1 as std::ffi::c_int },
    );
    lua_rawget(L, -(2 as std::ffi::c_int));
    if !(lua_type(L, -(1 as std::ffi::c_int)) == 0 as std::ffi::c_int) {} else {
        g_assertion_message_expr(
            0 as *mut gchar,
            b"common/luauniq.c\0" as *const u8 as *const std::ffi::c_char,
            121 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 14],
                &[std::ffi::c_char; 14],
            >(b"luaH_uniq_del\0"))
                .as_ptr(),
            b"!lua_isnil(L, -1)\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    lua_pushvalue(
        L,
        if k > 0 as std::ffi::c_int { k } else { k - 1 as std::ffi::c_int },
    );
    lua_pushnil(L);
    lua_rawset(L, -(3 as std::ffi::c_int));
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "133:1"]
pub unsafe extern "C" fn luaH_uniq_del_ptr(
    mut L: *mut lua_State,
    mut reg: *const gchar,
    mut key: gpointer,
) {
    lua_pushlightuserdata(L, key);
    luaH_uniq_del(L, reg, -(1 as std::ffi::c_int));
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
}
