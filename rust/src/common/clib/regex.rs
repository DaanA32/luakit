use gdk_sys::*;
use glib_sys::*;
use gtk_sys::*;
use libc::*;
use mlua_sys::*;

use crate::clib::luakit::*;
use crate::clib::ipc::*;
use crate::common::clib::luakit::*;
use crate::common::common;
use crate::common::luaclass::signal_h::*;
use crate::common::luaclass::*;
use crate::common::luah::*;
use crate::common::luaobject::*;
use crate::common::luauniq::*;
use crate::common::tokenize::*;
use crate::globalconf::*;
use crate::log::*;
use crate::luah::*;
use crate::web_context::*;

use crate::gtypes::*;
use webkit2gtk::{ffi::*, glib::gobject_ffi::*};

pub mod gregex_h {
    pub type GRegexMatchFlags = std::ffi::c_uint;
    pub const G_REGEX_MATCH_NOTEMPTY_ATSTART: GRegexMatchFlags = 268435456;
    pub const G_REGEX_MATCH_PARTIAL_HARD: GRegexMatchFlags = 134217728;
    pub const G_REGEX_MATCH_PARTIAL_SOFT: GRegexMatchFlags = 32768;
    pub const G_REGEX_MATCH_BSR_ANY: GRegexMatchFlags = 16777216;
    pub const G_REGEX_MATCH_BSR_ANYCRLF: GRegexMatchFlags = 8388608;
    pub const G_REGEX_MATCH_NEWLINE_ANYCRLF: GRegexMatchFlags = 5242880;
    pub const G_REGEX_MATCH_NEWLINE_ANY: GRegexMatchFlags = 4194304;
    pub const G_REGEX_MATCH_NEWLINE_CRLF: GRegexMatchFlags = 3145728;
    pub const G_REGEX_MATCH_NEWLINE_LF: GRegexMatchFlags = 2097152;
    pub const G_REGEX_MATCH_NEWLINE_CR: GRegexMatchFlags = 1048576;
    pub const G_REGEX_MATCH_PARTIAL: GRegexMatchFlags = 32768;
    pub const G_REGEX_MATCH_NOTEMPTY: GRegexMatchFlags = 1024;
    pub const G_REGEX_MATCH_NOTEOL: GRegexMatchFlags = 256;
    pub const G_REGEX_MATCH_NOTBOL: GRegexMatchFlags = 128;
    pub const G_REGEX_MATCH_ANCHORED: GRegexMatchFlags = 16;
    pub const G_REGEX_MATCH_DEFAULT: GRegexMatchFlags = 0;
    pub type GRegexCompileFlags = std::ffi::c_uint;
    pub const G_REGEX_JAVASCRIPT_COMPAT: GRegexCompileFlags = 33554432;
    pub const G_REGEX_BSR_ANYCRLF: GRegexCompileFlags = 8388608;
    pub const G_REGEX_NEWLINE_ANYCRLF: GRegexCompileFlags = 5242880;
    pub const G_REGEX_NEWLINE_CRLF: GRegexCompileFlags = 3145728;
    pub const G_REGEX_NEWLINE_LF: GRegexCompileFlags = 2097152;
    pub const G_REGEX_NEWLINE_CR: GRegexCompileFlags = 1048576;
    pub const G_REGEX_DUPNAMES: GRegexCompileFlags = 524288;
    pub const G_REGEX_FIRSTLINE: GRegexCompileFlags = 262144;
    pub const G_REGEX_OPTIMIZE: GRegexCompileFlags = 8192;
    pub const G_REGEX_NO_AUTO_CAPTURE: GRegexCompileFlags = 4096;
    pub const G_REGEX_RAW: GRegexCompileFlags = 2048;
    pub const G_REGEX_UNGREEDY: GRegexCompileFlags = 512;
    pub const G_REGEX_DOLLAR_ENDONLY: GRegexCompileFlags = 32;
    pub const G_REGEX_ANCHORED: GRegexCompileFlags = 16;
    pub const G_REGEX_EXTENDED: GRegexCompileFlags = 8;
    pub const G_REGEX_DOTALL: GRegexCompileFlags = 4;
    pub const G_REGEX_MULTILINE: GRegexCompileFlags = 2;
    pub const G_REGEX_CASELESS: GRegexCompileFlags = 1;
    pub const G_REGEX_DEFAULT: GRegexCompileFlags = 0;
    pub type GRegex = _GRegex;
    pub type GMatchInfo = _GMatchInfo;
    unsafe extern "C-unwind" {
        pub type _GRegex;
        pub type _GMatchInfo;
        pub type _GError;
        pub fn g_regex_new(
            pattern: *const gchar,
            compile_options: GRegexCompileFlags,
            match_options: GRegexMatchFlags,
            error: *mut *mut GError,
        ) -> *mut GRegex;
        pub fn g_regex_unref(regex: *mut GRegex);
        pub fn g_regex_match(
            regex: *const GRegex,
            string: *const gchar,
            match_options: GRegexMatchFlags,
            match_info: *mut *mut GMatchInfo,
        ) -> gboolean;
    }
    use crate::gtypes::*;
    use glib_sys::*;
}
pub use self::gregex_h::{
    GRegexMatchFlags, G_REGEX_MATCH_NOTEMPTY_ATSTART, G_REGEX_MATCH_PARTIAL_HARD,
    G_REGEX_MATCH_PARTIAL_SOFT, G_REGEX_MATCH_BSR_ANY, G_REGEX_MATCH_BSR_ANYCRLF,
    G_REGEX_MATCH_NEWLINE_ANYCRLF, G_REGEX_MATCH_NEWLINE_ANY, G_REGEX_MATCH_NEWLINE_CRLF,
    G_REGEX_MATCH_NEWLINE_LF, G_REGEX_MATCH_NEWLINE_CR, G_REGEX_MATCH_PARTIAL,
    G_REGEX_MATCH_NOTEMPTY, G_REGEX_MATCH_NOTEOL, G_REGEX_MATCH_NOTBOL,
    G_REGEX_MATCH_ANCHORED, G_REGEX_MATCH_DEFAULT, GRegexCompileFlags,
    G_REGEX_JAVASCRIPT_COMPAT, G_REGEX_BSR_ANYCRLF, G_REGEX_NEWLINE_ANYCRLF,
    G_REGEX_NEWLINE_CRLF, G_REGEX_NEWLINE_LF, G_REGEX_NEWLINE_CR, G_REGEX_DUPNAMES,
    G_REGEX_FIRSTLINE, G_REGEX_OPTIMIZE, G_REGEX_NO_AUTO_CAPTURE, G_REGEX_RAW,
    G_REGEX_UNGREEDY, G_REGEX_DOLLAR_ENDONLY, G_REGEX_ANCHORED, G_REGEX_EXTENDED,
    G_REGEX_DOTALL, G_REGEX_MULTILINE, G_REGEX_CASELESS, G_REGEX_DEFAULT, GRegex,
    GMatchInfo, _GRegex, _GMatchInfo, g_regex_new, g_regex_unref, g_regex_match,
};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lregex_t {
    pub signals: *mut signal_t,
    pub reg: *mut GRegex,
    pub pattern: *mut gchar,
    pub compile_options: GRegexCompileFlags,
    pub match_options: GRegexMatchFlags,
}
static mut regex_class: lua_class_t = lua_class_t {
    name: 0 as *const gchar,
    signals: 0 as *const signal_t as *mut signal_t,
    allocator: None,
    properties: 0 as *const lua_class_property_array_t
        as *mut lua_class_property_array_t,
    index_miss_property: None,
    newindex_miss_property: None,
};
#[inline]
unsafe extern "C-unwind" fn luaH_regex_class_remove_signal(mut L: *mut lua_State) -> gint {
    luaH_class_remove_signal(
        L,
        &mut regex_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn regex_new(mut L: *mut lua_State) -> *mut lregex_t {
    let mut p: *mut lregex_t = lua_newuserdata(
        L,
        ::core::mem::size_of::<lregex_t>(),
    ) as *mut lregex_t;
    memset(
        p as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        (::core::mem::size_of::<lregex_t>())
            .wrapping_mul(1),
    );
    (*p).signals = signal_new();
    luaH_settype(L, &mut regex_class);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_setmetatable(L, -(2 as std::ffi::c_int));
    lua_setfenv(L, -(2 as std::ffi::c_int));
    lua_pushvalue(L, -(1 as std::ffi::c_int));
    luaH_class_emit_signal(
        L,
        &mut regex_class,
        b"new\0" as *const u8 as *const std::ffi::c_char,
        1 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    return p;
}
#[inline]
unsafe extern "C-unwind" fn luaH_regex_class_add_signal(mut L: *mut lua_State) -> gint {
    luaH_class_add_signal(
        L,
        &mut regex_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn luaH_regex_class_emit_signal(mut L: *mut lua_State) -> gint {
    return luaH_class_emit_signal(
        L,
        &mut regex_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t),
        lua_gettop(L) - 1 as std::ffi::c_int,
        -(1 as std::ffi::c_int),
    );
}
unsafe extern "C-unwind" fn luaH_regex_gc(mut L: *mut lua_State) -> gint {
    let mut regex: *mut lregex_t = luaH_checkudata(
        L,
        1 as std::ffi::c_int,
        &mut regex_class,
    ) as *mut lregex_t;
    if !((*regex).reg).is_null() {
        g_regex_unref((*regex).reg);
    }
    g_free((*regex).pattern as gpointer);
    return luaH_object_gc(L);
}
unsafe extern "C-unwind" fn luaH_regenerate_regex(
    mut L: *mut lua_State,
    mut regex: *mut lregex_t,
) {
    if !((*regex).pattern).is_null() {} else {
        g_assertion_message_expr(
            0 as *mut gchar,
            b"common/clib/regex.c\0" as *const u8 as *const std::ffi::c_char,
            55 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 22],
                &[std::ffi::c_char; 22],
            >(b"luaH_regenerate_regex\0"))
                .as_ptr(),
            b"regex->pattern\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    if !((*regex).reg).is_null() {
        g_regex_unref((*regex).reg);
    }
    let mut error: *mut GError = 0 as *mut GError;
    (*regex)
        .reg = g_regex_new(
        (*regex).pattern,
        G_REGEX_DOTALL,
        G_REGEX_MATCH_DEFAULT,
        &mut error,
    );
    if !error.is_null() {
        lua_pushstring(L, (*error).message);
        g_error_free(error);
        luaL_error(L, lua_tolstring(L, -(1 as std::ffi::c_int), 0 as *mut size_t));
    }
}
unsafe extern "C-unwind" fn luaH_regex_new(mut L: *mut lua_State) -> std::ffi::c_int {
    luaH_class_new(L, &mut regex_class);
    let mut regex: *mut lregex_t = lua_touserdata(L, -(1 as std::ffi::c_int))
        as *mut lregex_t;
    if ((*regex).pattern).is_null() {
        return luaL_error(
            L,
            b"pattern not set\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    return 1 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_regex_match(mut L: *mut lua_State) -> std::ffi::c_int {
    let mut regex: *mut lregex_t = luaH_checkudata(
        L,
        1 as std::ffi::c_int,
        &mut regex_class,
    ) as *mut lregex_t;
    let mut haystack: *const gchar = luaL_checklstring(
        L,
        2 as std::ffi::c_int,
        0 as *mut size_t,
    );
    if !((*regex).reg).is_null() {} else {
        g_assertion_message_expr(
            0 as *mut gchar,
            b"common/clib/regex.c\0" as *const u8 as *const std::ffi::c_char,
            88 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[std::ffi::c_char; 17],
            >(b"luaH_regex_match\0"))
                .as_ptr(),
            b"regex->reg\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    let mut matched: gboolean = g_regex_match(
        (*regex).reg,
        haystack,
        G_REGEX_MATCH_DEFAULT,
        0 as *mut *mut GMatchInfo,
    );
    lua_pushboolean(L, matched);
    return 1 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_regex_get_pattern(
    mut L: *mut lua_State,
    mut regex: *mut lregex_t,
) -> std::ffi::c_int {
    lua_pushstring(L, (*regex).pattern);
    return 1 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_regex_set_pattern(
    mut L: *mut lua_State,
    mut regex: *mut lregex_t,
) -> std::ffi::c_int {
    let mut new_pattern: *mut gchar = g_strdup(
        luaL_checklstring(L, -(1 as std::ffi::c_int), 0 as *mut size_t),
    );
    g_free((*regex).pattern as gpointer);
    (*regex).pattern = new_pattern;
    luaH_regenerate_regex(L, regex);
    return 0 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn regex_class_setup(mut L: *mut lua_State) {
    static mut regex_methods: [luaL_Reg; 4] = unsafe {
        [
            {
                let mut init = luaL_Reg {
                    name: b"add_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: 
                        luaH_regex_class_add_signal
                    ,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"remove_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: 
                        luaH_regex_class_remove_signal
                    ,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"emit_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: 
                        luaH_regex_class_emit_signal
                    ,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__call\0" as *const u8 as *const std::ffi::c_char,
                    func: 
                        luaH_regex_new
                    ,
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
    static mut regex_meta: [luaL_Reg; 9] = unsafe {
        [
            {
                let mut init = luaL_Reg {
                    name: b"__tostring\0" as *const u8 as *const std::ffi::c_char,
                    func: 
                        luaH_object_tostring
                    ,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"add_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: 
                        luaH_object_add_signal_simple
                    ,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"remove_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: 
                        luaH_object_remove_signal_simple
                    ,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"remove_signals\0" as *const u8 as *const std::ffi::c_char,
                    func: 
                        luaH_object_remove_signals_simple
                    ,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"emit_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: 
                        luaH_object_emit_signal_simple
                    ,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__index\0" as *const u8 as *const std::ffi::c_char,
                    func: 
                        luaH_class_index,
                }
                ;
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__newindex\0" as *const u8 as *const std::ffi::c_char,
                    func: 
                        luaH_class_newindex
                    ,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"match\0" as *const u8 as *const std::ffi::c_char,
                    func: 
                        luaH_regex_match
                    ,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__gc\0" as *const u8 as *const std::ffi::c_char,
                    func: 
                        luaH_regex_gc ,
                }
                ;
                init
            },
            //{
            //    let mut init = luaL_Reg {
            //        name: 0 as *const std::ffi::c_char,
            //        func: None,
            //    };
            //    init
            //},
        ]
    };
    luaH_class_setup(
        L,
        &mut regex_class,
        b"regex\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<unsafe extern "C-unwind" fn(*mut lua_State) -> *mut lregex_t>,
            lua_class_allocator_t,
        >(Some(regex_new as unsafe extern "C-unwind" fn(*mut lua_State) -> *mut lregex_t)),
        None,
        None,
        regex_methods.as_ptr(),
        regex_meta.as_ptr(),
    );
    luaH_class_add_property(
        &mut regex_class,
        L_TK_PATTERN,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C-unwind" fn(*mut lua_State, *mut lregex_t) -> std::ffi::c_int,
            >,
            lua_class_propfunc_t,
        >(
            Some(
                luaH_regex_set_pattern
                    as unsafe extern "C-unwind" fn(
                        *mut lua_State,
                        *mut lregex_t,
                    ) -> std::ffi::c_int,
            ),
        ),
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C-unwind" fn(*mut lua_State, *mut lregex_t) -> std::ffi::c_int,
            >,
            lua_class_propfunc_t,
        >(
            Some(
                luaH_regex_get_pattern
                    as unsafe extern "C-unwind" fn(
                        *mut lua_State,
                        *mut lregex_t,
                    ) -> std::ffi::c_int,
            ),
        ),
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C-unwind" fn(*mut lua_State, *mut lregex_t) -> std::ffi::c_int,
            >,
            lua_class_propfunc_t,
        >(
            Some(
                luaH_regex_set_pattern
                    as unsafe extern "C-unwind" fn(
                        *mut lua_State,
                        *mut lregex_t,
                    ) -> std::ffi::c_int,
            ),
        ),
    );
}
