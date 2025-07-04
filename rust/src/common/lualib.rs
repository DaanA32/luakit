use ::libc;
use ::c2rust_bitfields;
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
#[c2rust::header_src = "/usr/include/bits/types.h:22"]
pub mod types_h {
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = std::ffi::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = std::ffi::c_long;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtypes.h:22"]
pub mod gtypes_h {
    #[c2rust::src_loc = "52:1"]
    pub type gchar = std::ffi::c_char;
    #[c2rust::src_loc = "55:1"]
    pub type gint = std::ffi::c_int;
    #[c2rust::src_loc = "61:1"]
    pub type guint = std::ffi::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:22"]
pub mod struct_FILE_h {
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "50:8"]
    pub struct _IO_FILE {
        pub _flags: std::ffi::c_int,
        pub _IO_read_ptr: *mut std::ffi::c_char,
        pub _IO_read_end: *mut std::ffi::c_char,
        pub _IO_read_base: *mut std::ffi::c_char,
        pub _IO_write_base: *mut std::ffi::c_char,
        pub _IO_write_ptr: *mut std::ffi::c_char,
        pub _IO_write_end: *mut std::ffi::c_char,
        pub _IO_buf_base: *mut std::ffi::c_char,
        pub _IO_buf_end: *mut std::ffi::c_char,
        pub _IO_save_base: *mut std::ffi::c_char,
        pub _IO_backup_base: *mut std::ffi::c_char,
        pub _IO_save_end: *mut std::ffi::c_char,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: std::ffi::c_int,
        #[bitfield(name = "_flags2", ty = "std::ffi::c_int", bits = "0..=23")]
        pub _flags2: [u8; 3],
        pub _short_backupbuf: [std::ffi::c_char; 1],
        pub _old_offset: __off_t,
        pub _cur_column: std::ffi::c_ushort,
        pub _vtable_offset: std::ffi::c_schar,
        pub _shortbuf: [std::ffi::c_char; 1],
        pub _lock: *mut std::ffi::c_void,
        pub _offset: __off64_t,
        pub _codecvt: *mut _IO_codecvt,
        pub _wide_data: *mut _IO_wide_data,
        pub _freeres_list: *mut _IO_FILE,
        pub _freeres_buf: *mut std::ffi::c_void,
        pub _prevchain: *mut *mut _IO_FILE,
        pub _mode: std::ffi::c_int,
        pub _unused2: [std::ffi::c_char; 20],
    }
    #[c2rust::src_loc = "44:1"]
    pub type _IO_lock_t = ();
    use super::types_h::{__off_t, __off64_t};
    extern "C" {
        #[c2rust::src_loc = "39:8"]
        pub type _IO_wide_data;
        #[c2rust::src_loc = "38:8"]
        pub type _IO_codecvt;
        #[c2rust::src_loc = "37:8"]
        pub type _IO_marker;
    }
}
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:22"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/luajit-2.1/lua.h:22"]
pub mod lua_h {
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
        #[c2rust::src_loc = "140:1"]
        pub fn lua_type(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;
        #[c2rust::src_loc = "141:1"]
        pub fn lua_typename(
            L: *mut lua_State,
            tp: std::ffi::c_int,
        ) -> *const std::ffi::c_char;
        #[c2rust::src_loc = "147:1"]
        pub fn lua_tonumber(L: *mut lua_State, idx: std::ffi::c_int) -> lua_Number;
        #[c2rust::src_loc = "148:1"]
        pub fn lua_tointeger(L: *mut lua_State, idx: std::ffi::c_int) -> lua_Integer;
        #[c2rust::src_loc = "149:1"]
        pub fn lua_toboolean(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;
        #[c2rust::src_loc = "150:1"]
        pub fn lua_tolstring(
            L: *mut lua_State,
            idx: std::ffi::c_int,
            len: *mut size_t,
        ) -> *const std::ffi::c_char;
        #[c2rust::src_loc = "151:1"]
        pub fn lua_objlen(L: *mut lua_State, idx: std::ffi::c_int) -> size_t;
        #[c2rust::src_loc = "155:1"]
        pub fn lua_topointer(
            L: *mut lua_State,
            idx: std::ffi::c_int,
        ) -> *const std::ffi::c_void;
        #[c2rust::src_loc = "161:1"]
        pub fn lua_pushnil(L: *mut lua_State);
        #[c2rust::src_loc = "241:1"]
        pub fn lua_next(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:22"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[c2rust::src_loc = "151:14"]
        pub static mut stderr: *mut FILE;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gprintf.h:22"]
pub mod gprintf_h {
    use super::FILE_h::FILE;
    use super::gtypes_h::{gchar, gint};
    extern "C" {
        #[c2rust::src_loc = "32:1"]
        pub fn g_fprintf(file: *mut FILE, format: *const gchar, _: ...) -> gint;
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
#[c2rust::header_src = "/home/daana/git/luakit/common/lualib.h:22"]
pub mod lualib_h {
    #[inline]
    #[c2rust::src_loc = "52:1"]
    pub unsafe extern "C" fn luaH_dump_table_keys(mut L: *mut lua_State, mut idx: gint) {
        let mut len: gint = lua_objlen(L, idx) as gint;
        let mut limit: guint = 5 as std::ffi::c_int as guint;
        let mut rem: guint = 0 as std::ffi::c_int as guint;
        g_fprintf(stderr, b"  Keys: \0" as *const u8 as *const std::ffi::c_char);
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
                    g_fprintf(
                        stderr,
                        b"%zd, \0" as *const u8 as *const std::ffi::c_char,
                        lua_tointeger(L, -(2 as std::ffi::c_int)),
                    );
                } else if key_type == 4 as std::ffi::c_int {
                    g_fprintf(
                        stderr,
                        b"%s, \0" as *const u8 as *const std::ffi::c_char,
                        lua_tolstring(L, -(2 as std::ffi::c_int), 0 as *mut size_t),
                    );
                } else {
                    g_fprintf(
                        stderr,
                        b"[%s]\0" as *const u8 as *const std::ffi::c_char,
                        lua_typename(L, key_type),
                    );
                }
            }
            lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        }
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        g_fprintf(
            stderr,
            b"and %d more\n\0" as *const u8 as *const std::ffi::c_char,
            rem,
        );
    }
    use super::lua_h::{
        lua_State, lua_objlen, lua_pushvalue, lua_pushnil, lua_next, lua_type,
        lua_tointeger, lua_Integer, lua_tolstring, lua_typename, lua_settop,
    };
    use super::gtypes_h::{gint, guint};
    use super::__stddef_size_t_h::size_t;
    use super::gprintf_h::g_fprintf;
    use super::stdio_h::stderr;
}
pub use self::__stddef_ptrdiff_t_h::ptrdiff_t;
pub use self::__stddef_size_t_h::size_t;
pub use self::types_h::{__off_t, __off64_t};
pub use self::gtypes_h::{gchar, gint, guint};
pub use self::struct_FILE_h::{
    _IO_FILE, _IO_lock_t, _IO_wide_data, _IO_codecvt, _IO_marker,
};
pub use self::FILE_h::FILE;
pub use self::lua_h::{
    lua_Number, lua_Integer, lua_State, lua_gettop, lua_settop, lua_pushvalue, lua_type,
    lua_typename, lua_tonumber, lua_tointeger, lua_toboolean, lua_tolstring, lua_objlen,
    lua_topointer, lua_pushnil, lua_next,
};
use self::stdio_h::stderr;
use self::gprintf_h::g_fprintf;
use self::luaclass_h::luaH_typename;
pub use self::lualib_h::luaH_dump_table_keys;
#[no_mangle]
#[c2rust::src_loc = "27:1"]
pub unsafe extern "C" fn luaH_dump_stack(mut L: *mut lua_State) {
    g_fprintf(
        stderr,
        b"-------- Lua stack dump ---------\n\0" as *const u8 as *const std::ffi::c_char,
    );
    let mut i: std::ffi::c_int = lua_gettop(L);
    while i != 0 {
        let mut t: std::ffi::c_int = lua_type(L, i);
        match t {
            4 => {
                g_fprintf(
                    stderr,
                    b"%d: string: `%s'\n\0" as *const u8 as *const std::ffi::c_char,
                    i,
                    lua_tolstring(L, i, 0 as *mut size_t),
                );
            }
            1 => {
                g_fprintf(
                    stderr,
                    b"%d: bool:   %s\n\0" as *const u8 as *const std::ffi::c_char,
                    i,
                    if lua_toboolean(L, i) != 0 {
                        b"true\0" as *const u8 as *const std::ffi::c_char
                    } else {
                        b"false\0" as *const u8 as *const std::ffi::c_char
                    },
                );
            }
            3 => {
                g_fprintf(
                    stderr,
                    b"%d: number: %g\n\0" as *const u8 as *const std::ffi::c_char,
                    i,
                    lua_tonumber(L, i),
                );
            }
            0 => {
                g_fprintf(
                    stderr,
                    b"%d: nil\n\0" as *const u8 as *const std::ffi::c_char,
                    i,
                );
            }
            7 => {
                g_fprintf(
                    stderr,
                    b"%d: <%s>\t\t%p\n\0" as *const u8 as *const std::ffi::c_char,
                    i,
                    luaH_typename(L, i),
                    lua_topointer(L, i),
                );
            }
            5 => {
                g_fprintf(
                    stderr,
                    b"%d: table\t#%zu\t%p\n\0" as *const u8 as *const std::ffi::c_char,
                    i,
                    lua_objlen(L, i),
                    lua_topointer(L, i),
                );
                luaH_dump_table_keys(L, i);
            }
            _ => {
                g_fprintf(
                    stderr,
                    b"%d: %s\t#%d\t%p\n\0" as *const u8 as *const std::ffi::c_char,
                    i,
                    lua_typename(L, t),
                    lua_objlen(L, i) as gint,
                    lua_topointer(L, i),
                );
            }
        }
        i -= 1;
        i;
    }
    g_fprintf(
        stderr,
        b"------- Lua stack dump end ------\n\0" as *const u8 as *const std::ffi::c_char,
    );
}
