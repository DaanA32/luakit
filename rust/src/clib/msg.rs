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
        #[c2rust::src_loc = "203:1"]
        pub fn lua_pcall(
            L: *mut lua_State,
            nargs: std::ffi::c_int,
            nresults: std::ffi::c_int,
            errfunc: std::ffi::c_int,
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "335:1"]
        pub fn lua_getstack(
            L: *mut lua_State,
            level: std::ffi::c_int,
            ar: *mut lua_Debug,
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "336:1"]
        pub fn lua_getinfo(
            L: *mut lua_State,
            what: *const std::ffi::c_char,
            ar: *mut lua_Debug,
        ) -> std::ffi::c_int;
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
    #[c2rust::src_loc = "61:1"]
    pub type guint = std::ffi::c_uint;
    #[c2rust::src_loc = "109:1"]
    pub type gpointer = *mut std::ffi::c_void;
    #[c2rust::src_loc = "110:1"]
    pub type gconstpointer = *const std::ffi::c_void;
    #[c2rust::src_loc = "114:1"]
    pub type GCompareDataFunc = Option::<
        unsafe extern "C" fn(gconstpointer, gconstpointer, gpointer) -> gint,
    >;
    #[c2rust::src_loc = "140:1"]
    pub type GDestroyNotify = Option::<unsafe extern "C" fn(gpointer) -> ()>;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/garray.h:21"]
pub mod garray_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "55:8"]
    pub struct _GPtrArray {
        pub pdata: *mut gpointer,
        pub len: guint,
    }
    #[c2rust::src_loc = "41:1"]
    pub type GPtrArray = _GPtrArray;
    use super::gtypes_h::{gpointer, guint, gboolean};
    extern "C" {
        #[c2rust::src_loc = "188:1"]
        pub fn g_ptr_array_free(
            array: *mut GPtrArray,
            free_segment: gboolean,
        ) -> *mut gpointer;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/ghash.h:21"]
pub mod ghash_h {
    #[c2rust::src_loc = "40:1"]
    pub type GHashTable = _GHashTable;
    extern "C" {
        #[c2rust::src_loc = "40:16"]
        pub type _GHashTable;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtree.h:21"]
pub mod gtree_h {
    #[c2rust::src_loc = "40:1"]
    pub type GTree = _GTree;
    use super::gtypes_h::{GCompareDataFunc, gpointer, GDestroyNotify};
    extern "C" {
        #[c2rust::src_loc = "40:16"]
        pub type _GTree;
        #[c2rust::src_loc = "78:1"]
        pub fn g_tree_new_full(
            key_compare_func: GCompareDataFunc,
            key_compare_data: gpointer,
            key_destroy_func: GDestroyNotify,
            value_destroy_func: GDestroyNotify,
        ) -> *mut GTree;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/log.h:21"]
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
#[c2rust::header_src = "/home/daana/git/luakit/common/signal.h:21"]
pub mod signal_h {
    #[c2rust::src_loc = "29:1"]
    pub type signal_t = GTree;
    #[inline]
    #[c2rust::src_loc = "33:1"]
    pub unsafe extern "C" fn signal_cmp(
        mut a: gconstpointer,
        mut b: gconstpointer,
        mut UNUSED_p: gpointer,
    ) -> gint {
        return g_strcmp0(a as *const std::ffi::c_char, b as *const std::ffi::c_char);
    }
    #[inline]
    #[c2rust::src_loc = "40:1"]
    pub unsafe extern "C" fn signal_array_destroy(mut sigfuncs: *mut gpointer) {
        g_ptr_array_free(
            sigfuncs as *mut GPtrArray,
            (0 as std::ffi::c_int == 0) as std::ffi::c_int,
        );
    }
    #[inline]
    #[c2rust::src_loc = "47:1"]
    pub unsafe extern "C" fn signal_new() -> *mut signal_t {
        return g_tree_new_full(
            ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(gconstpointer, gconstpointer, gpointer) -> gint,
                >,
                GCompareDataFunc,
            >(
                Some(
                    signal_cmp
                        as unsafe extern "C" fn(
                            gconstpointer,
                            gconstpointer,
                            gpointer,
                        ) -> gint,
                ),
            ),
            0 as *mut std::ffi::c_void,
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut gpointer) -> ()>,
                GDestroyNotify,
            >(Some(signal_array_destroy as unsafe extern "C" fn(*mut gpointer) -> ())),
        ) as *mut signal_t;
    }
    use super::gtree_h::{GTree, g_tree_new_full};
    use super::gtypes_h::{
        gconstpointer, gpointer, gint, GCompareDataFunc, GDestroyNotify,
    };
    use super::gtestutils_h::g_strcmp0;
    use super::garray_h::{g_ptr_array_free, GPtrArray};
    use super::gmem_h::g_free;
}
#[c2rust::header_src = "/usr/include/luajit-2.1/lauxlib.h:21"]
pub mod lauxlib_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "21:16"]
    pub struct luaL_Reg {
        pub name: *const std::ffi::c_char,
        pub func: lua_CFunction,
    }
    use super::lua_h::{lua_CFunction, lua_State};
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "34:1"]
        pub fn luaL_checklstring(
            L: *mut lua_State,
            numArg: std::ffi::c_int,
            l: *mut size_t,
        ) -> *const std::ffi::c_char;
        #[c2rust::src_loc = "53:1"]
        pub fn luaL_error(
            L: *mut lua_State,
            fmt: *const std::ffi::c_char,
            _: ...
        ) -> std::ffi::c_int;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/luaclass.h:21"]
pub mod luaclass_h {
    #[c2rust::src_loc = "32:1"]
    pub type lua_class_property_array_t = GHashTable;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "39:9"]
    pub struct lua_object_t {
        pub signals: *mut signal_t,
    }
    #[c2rust::src_loc = "43:1"]
    pub type lua_class_allocator_t = Option::<
        unsafe extern "C" fn(*mut lua_State) -> *mut lua_object_t,
    >;
    #[c2rust::src_loc = "45:1"]
    pub type lua_class_propfunc_t = Option::<
        unsafe extern "C" fn(*mut lua_State, *mut lua_object_t) -> gint,
    >;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "47:9"]
    pub struct lua_class_t {
        pub name: *const gchar,
        pub signals: *mut signal_t,
        pub allocator: lua_class_allocator_t,
        pub properties: *mut lua_class_property_array_t,
        pub index_miss_property: lua_class_propfunc_t,
        pub newindex_miss_property: lua_class_propfunc_t,
    }
    use super::ghash_h::GHashTable;
    use super::signal_h::signal_t;
    use super::lua_h::lua_State;
    use super::gtypes_h::{gint, gchar};
    use super::lauxlib_h::luaL_Reg;
    extern "C" {
        #[c2rust::src_loc = "65:1"]
        pub fn luaH_class_add_signal(
            _: *mut lua_State,
            _: *mut lua_class_t,
            name: *const gchar,
            ud: gint,
        );
        #[c2rust::src_loc = "67:1"]
        pub fn luaH_class_remove_signal(
            _: *mut lua_State,
            _: *mut lua_class_t,
            name: *const gchar,
            ud: gint,
        );
        #[c2rust::src_loc = "69:1"]
        pub fn luaH_class_emit_signal(
            _: *mut lua_State,
            _: *mut lua_class_t,
            name: *const gchar,
            nargs: gint,
            nret: gint,
        ) -> gint;
        #[c2rust::src_loc = "74:1"]
        pub fn luaH_openlib(
            _: *mut lua_State,
            _: *const gchar,
            _: *const luaL_Reg,
            _: *const luaL_Reg,
        );
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gmem.h:21"]
pub mod gmem_h {
    use super::gtypes_h::gpointer;
    extern "C" {
        #[c2rust::src_loc = "73:1"]
        pub fn g_free(mem: gpointer);
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtestutils.h:21"]
pub mod gtestutils_h {
    extern "C" {
        #[c2rust::src_loc = "282:1"]
        pub fn g_strcmp0(
            str1: *const std::ffi::c_char,
            str2: *const std::ffi::c_char,
        ) -> std::ffi::c_int;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/luaobject.h:22"]
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
#[c2rust::header_src = "/home/daana/git/luakit/common/clib/msg.h:22"]
pub mod msg_h {
    #[c2rust::src_loc = "34:17"]
    pub static mut string_format_ref: gpointer = 0 as *const std::ffi::c_void
        as *mut std::ffi::c_void;
    #[c2rust::src_loc = "35:17"]
    pub static mut tostring_ref: gpointer = 0 as *const std::ffi::c_void
        as *mut std::ffi::c_void;
    #[c2rust::src_loc = "37:1"]
    pub unsafe extern "C" fn luaH_msg_string_from_args(
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
                b"failed to format message: %s\0" as *const u8
                    as *const std::ffi::c_char,
                lua_tolstring(L, -(1 as std::ffi::c_int), 0 as *mut size_t),
            );
        }
        return lua_tolstring(L, -(1 as std::ffi::c_int), 0 as *mut size_t);
    }
    #[c2rust::src_loc = "60:1"]
    pub unsafe extern "C" fn luaH_msg(
        mut L: *mut lua_State,
        mut lvl: log_level_t,
    ) -> gint {
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
        lua_getstack(L, 1 as std::ffi::c_int, &mut ar);
        lua_getinfo(L, b"Sln\0" as *const u8 as *const std::ffi::c_char, &mut ar);
        let mut src: *const std::ffi::c_char = if *(ar.source)
            .offset(0 as std::ffi::c_int as isize) as std::ffi::c_int == '@' as i32
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
    #[c2rust::src_loc = "79:1"]
    pub unsafe extern "C" fn luaH_msg_info(mut L: *mut lua_State) -> gint {
        return luaH_msg(L, LOG_LEVEL_info);
    }
    #[c2rust::src_loc = "79:1"]
    pub unsafe extern "C" fn luaH_msg_debug(mut L: *mut lua_State) -> gint {
        return luaH_msg(L, LOG_LEVEL_debug);
    }
    #[c2rust::src_loc = "79:1"]
    pub unsafe extern "C" fn luaH_msg_warn(mut L: *mut lua_State) -> gint {
        return luaH_msg(L, LOG_LEVEL_warn);
    }
    #[c2rust::src_loc = "79:1"]
    pub unsafe extern "C" fn luaH_msg_verbose(mut L: *mut lua_State) -> gint {
        return luaH_msg(L, LOG_LEVEL_verbose);
    }
    #[c2rust::src_loc = "79:1"]
    pub unsafe extern "C" fn luaH_msg_error(mut L: *mut lua_State) -> gint {
        return luaH_msg(L, LOG_LEVEL_error);
    }
    #[c2rust::src_loc = "79:1"]
    pub unsafe extern "C" fn luaH_msg_fatal(mut L: *mut lua_State) -> gint {
        return luaH_msg(L, LOG_LEVEL_fatal);
    }
    use super::gtypes_h::{gpointer, gchar, gint};
    use super::lua_h::{
        lua_State, lua_gettop, lua_type, lua_pushvalue, lua_pcall, lua_remove,
        lua_insert, lua_tolstring, lua_Debug, lua_getstack, lua_getinfo,
    };
    use super::luaobject_h::luaH_object_push;
    use super::lauxlib_h::luaL_error;
    use super::__stddef_size_t_h::size_t;
    use super::log_h::{
        log_level_t, _log, LOG_LEVEL_info, LOG_LEVEL_debug, LOG_LEVEL_warn,
        LOG_LEVEL_verbose, LOG_LEVEL_error, LOG_LEVEL_fatal,
    };
}
pub use self::__stddef_size_t_h::size_t;
pub use self::lua_h::{
    lua_CFunction, lua_Debug, lua_State, lua_gettop, lua_settop, lua_pushvalue,
    lua_remove, lua_insert, lua_type, lua_tolstring, lua_pushlstring,
    lua_pushlightuserdata, lua_getfield, lua_rawget, lua_pcall, lua_getstack, lua_getinfo,
};
pub use self::gtypes_h::{
    gchar, gint, gboolean, guint, gpointer, gconstpointer, GCompareDataFunc,
    GDestroyNotify,
};
pub use self::garray_h::{_GPtrArray, GPtrArray, g_ptr_array_free};
pub use self::ghash_h::{GHashTable, _GHashTable};
pub use self::gtree_h::{GTree, _GTree, g_tree_new_full};
pub use self::log_h::{
    log_level_t, LOG_LEVEL_debug, LOG_LEVEL_verbose, LOG_LEVEL_info, LOG_LEVEL_warn,
    LOG_LEVEL_error, LOG_LEVEL_fatal, _log,
};
pub use self::signal_h::{signal_t, signal_cmp, signal_array_destroy, signal_new};
pub use self::lauxlib_h::{luaL_Reg, luaL_checklstring, luaL_error};
pub use self::luaclass_h::{
    lua_class_property_array_t, lua_object_t, lua_class_allocator_t,
    lua_class_propfunc_t, lua_class_t, luaH_class_add_signal, luaH_class_remove_signal,
    luaH_class_emit_signal, luaH_openlib,
};
use self::gmem_h::g_free;
use self::gtestutils_h::g_strcmp0;
pub use self::luaobject_h::{
    luaH_object_registry_push, luaH_object_ref, luaH_object_push, luaH_object_incref,
};
pub use self::msg_h::{
    string_format_ref, tostring_ref, luaH_msg_string_from_args, luaH_msg, luaH_msg_info,
    luaH_msg_debug, luaH_msg_warn, luaH_msg_verbose, luaH_msg_error, luaH_msg_fatal,
};
#[c2rust::src_loc = "25:20"]
static mut msg_class: lua_class_t = lua_class_t {
    name: 0 as *const gchar,
    signals: 0 as *const signal_t as *mut signal_t,
    allocator: None,
    properties: 0 as *const lua_class_property_array_t
        as *mut lua_class_property_array_t,
    index_miss_property: None,
    newindex_miss_property: None,
};
#[inline]
#[c2rust::src_loc = "26:1"]
unsafe extern "C" fn luaH_msg_class_emit_signal(mut L: *mut lua_State) -> gint {
    return luaH_class_emit_signal(
        L,
        &mut msg_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t),
        lua_gettop(L) - 1 as std::ffi::c_int,
        -(1 as std::ffi::c_int),
    );
}
#[inline]
#[c2rust::src_loc = "26:1"]
unsafe extern "C" fn luaH_msg_class_remove_signal(mut L: *mut lua_State) -> gint {
    luaH_class_remove_signal(
        L,
        &mut msg_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[inline]
#[c2rust::src_loc = "26:1"]
unsafe extern "C" fn luaH_msg_class_add_signal(mut L: *mut lua_State) -> gint {
    luaH_class_add_signal(
        L,
        &mut msg_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "28:1"]
pub unsafe extern "C" fn msg_lib_get_msg_class() -> *mut lua_class_t {
    return &mut msg_class;
}
#[no_mangle]
#[c2rust::src_loc = "34:1"]
pub unsafe extern "C" fn msg_lib_setup(mut L: *mut lua_State) {
    static mut msg_lib: [luaL_Reg; 10] = unsafe {
        [
            {
                let mut init = luaL_Reg {
                    name: b"add_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_msg_class_add_signal
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"remove_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_msg_class_remove_signal
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"emit_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_msg_class_emit_signal
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
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
                    name: 0 as *const std::ffi::c_char,
                    func: None,
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
