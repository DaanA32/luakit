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
        #[c2rust::src_loc = "123:1"]
        pub fn lua_pushvalue(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "124:1"]
        pub fn lua_remove(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "165:1"]
        pub fn lua_pushstring(L: *mut lua_State, s: *const std::ffi::c_char);
        #[c2rust::src_loc = "182:1"]
        pub fn lua_createtable(
            L: *mut lua_State,
            narr: std::ffi::c_int,
            nrec: std::ffi::c_int,
        );
        #[c2rust::src_loc = "183:1"]
        pub fn lua_newuserdata(L: *mut lua_State, sz: size_t) -> *mut std::ffi::c_void;
        #[c2rust::src_loc = "193:1"]
        pub fn lua_rawset(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "195:1"]
        pub fn lua_setmetatable(
            L: *mut lua_State,
            objindex: std::ffi::c_int,
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "196:1"]
        pub fn lua_setfenv(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/lib/glib-2.0/include/glibconfig.h:21"]
pub mod glibconfig_h {
    #[c2rust::src_loc = "83:1"]
    pub type gsize = std::ffi::c_ulong;
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
    use super::gtypes_h::{gint, gchar, gpointer};
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
        #[c2rust::src_loc = "76:1"]
        pub fn luaH_class_setup(
            _: *mut lua_State,
            _: *mut lua_class_t,
            _: *const gchar,
            _: lua_class_allocator_t,
            _: lua_class_propfunc_t,
            _: lua_class_propfunc_t,
            _: *const luaL_Reg,
            _: *const luaL_Reg,
        );
        #[c2rust::src_loc = "86:1"]
        pub fn luaH_class_new(_: *mut lua_State, _: *mut lua_class_t) -> gint;
        #[c2rust::src_loc = "88:1"]
        pub fn luaH_checkudata(
            _: *mut lua_State,
            _: gint,
            _: *mut lua_class_t,
        ) -> gpointer;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/clib/ipc.h:21"]
pub mod ipc_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "29:16"]
    pub struct _ipc_channel_t {
        pub signals: *mut signal_t,
        pub name: *mut std::ffi::c_char,
    }
    #[c2rust::src_loc = "29:1"]
    pub type ipc_channel_t = _ipc_channel_t;
    use super::signal_h::signal_t;
    use super::lua_h::lua_State;
    use super::gtypes_h::gint;
    extern "C" {
        #[c2rust::src_loc = "36:1"]
        pub fn ipc_channel_send(L: *mut lua_State) -> gint;
    }
}
#[c2rust::header_src = "/usr/include/string.h:21"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "43:14"]
        pub fn memcpy(
            _: *mut std::ffi::c_void,
            _: *const std::ffi::c_void,
            _: std::ffi::c_ulong,
        ) -> *mut std::ffi::c_void;
        #[c2rust::src_loc = "61:14"]
        pub fn memset(
            _: *mut std::ffi::c_void,
            _: std::ffi::c_int,
            _: std::ffi::c_ulong,
        ) -> *mut std::ffi::c_void;
        #[c2rust::src_loc = "407:15"]
        pub fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gmem.h:21"]
pub mod gmem_h {
    use super::gtypes_h::gpointer;
    use super::glibconfig_h::gsize;
    extern "C" {
        #[c2rust::src_loc = "73:1"]
        pub fn g_free(mem: gpointer);
        #[c2rust::src_loc = "83:1"]
        pub fn g_malloc(n_bytes: gsize) -> gpointer;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gstrfuncs.h:21"]
pub mod gstrfuncs_h {
    #[inline(always)]
    #[c2rust::src_loc = "308:1"]
    pub unsafe extern "C" fn g_strdup_inline(
        mut str: *const std::ffi::c_char,
    ) -> *mut std::ffi::c_char {
        if 0 != 0 && str.is_null() {
            return 0 as *mut std::ffi::c_char;
        }
        if 0 != 0 && !str.is_null() && 0 != 0 {
            let len: size_t = (strlen(str))
                .wrapping_add(1 as std::ffi::c_int as std::ffi::c_ulong);
            let mut dup_str: *mut std::ffi::c_char = g_malloc(len)
                as *mut std::ffi::c_char;
            return memcpy(
                dup_str as *mut std::ffi::c_void,
                str as *const std::ffi::c_void,
                len,
            ) as *mut std::ffi::c_char;
        }
        return g_strdup(str);
    }
    use super::gtypes_h::gchar;
    use super::string_h::{strlen, memcpy};
    use super::__stddef_size_t_h::size_t;
    use super::gmem_h::g_malloc;
    extern "C" {
        #[c2rust::src_loc = "283:1"]
        pub fn g_strdup(str: *const gchar) -> *mut gchar;
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
#[c2rust::header_src = "/home/daana/git/luakit/common/luaobject.h:21"]
pub mod luaobject_h {
    use super::lua_h::lua_State;
    use super::luaclass_h::lua_class_t;
    use super::gtypes_h::gint;
    extern "C" {
        #[c2rust::src_loc = "38:1"]
        pub fn luaH_settype(L: *mut lua_State, lua_class: *mut lua_class_t) -> gint;
        #[c2rust::src_loc = "167:1"]
        pub fn luaH_object_add_signal_simple(L: *mut lua_State) -> gint;
        #[c2rust::src_loc = "168:1"]
        pub fn luaH_object_remove_signal_simple(L: *mut lua_State) -> gint;
        #[c2rust::src_loc = "169:1"]
        pub fn luaH_object_remove_signals_simple(L: *mut lua_State) -> gint;
        #[c2rust::src_loc = "170:1"]
        pub fn luaH_object_emit_signal_simple(L: *mut lua_State) -> gint;
        #[c2rust::src_loc = "203:1"]
        pub fn luaH_object_tostring(_: *mut lua_State) -> gint;
        #[c2rust::src_loc = "204:1"]
        pub fn luaH_object_gc(_: *mut lua_State) -> gint;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/luauniq.h:26"]
pub mod luauniq_h {
    use super::lua_h::lua_State;
    use super::gtypes_h::gchar;
    extern "C" {
        #[c2rust::src_loc = "32:1"]
        pub fn luaH_uniq_add(
            L: *mut lua_State,
            reg: *const gchar,
            k: std::ffi::c_int,
            oud: std::ffi::c_int,
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "34:1"]
        pub fn luaH_uniq_get(
            L: *mut lua_State,
            reg: *const gchar,
            k: std::ffi::c_int,
        ) -> std::ffi::c_int;
    }
}
pub use self::__stddef_size_t_h::size_t;
pub use self::lua_h::{
    lua_CFunction, lua_State, lua_gettop, lua_pushvalue, lua_remove, lua_pushstring,
    lua_createtable, lua_newuserdata, lua_rawset, lua_setmetatable, lua_setfenv,
};
pub use self::glibconfig_h::gsize;
pub use self::gtypes_h::{
    gchar, gint, gboolean, guint, gpointer, gconstpointer, GCompareDataFunc,
    GDestroyNotify,
};
pub use self::garray_h::{_GPtrArray, GPtrArray, g_ptr_array_free};
pub use self::ghash_h::{GHashTable, _GHashTable};
pub use self::gtree_h::{GTree, _GTree, g_tree_new_full};
pub use self::signal_h::{signal_t, signal_cmp, signal_array_destroy, signal_new};
pub use self::lauxlib_h::{luaL_Reg, luaL_checklstring};
pub use self::luaclass_h::{
    lua_class_property_array_t, lua_object_t, lua_class_allocator_t,
    lua_class_propfunc_t, lua_class_t, luaH_class_add_signal, luaH_class_remove_signal,
    luaH_class_emit_signal, luaH_class_setup, luaH_class_new, luaH_checkudata,
};
pub use self::ipc_h::{_ipc_channel_t, ipc_channel_t, ipc_channel_send};
use self::string_h::{memcpy, memset, strlen};
use self::gmem_h::{g_free, g_malloc};
pub use self::gstrfuncs_h::{g_strdup_inline, g_strdup};
use self::gtestutils_h::g_strcmp0;
use self::luaobject_h::{
    luaH_settype, luaH_object_add_signal_simple, luaH_object_remove_signal_simple,
    luaH_object_remove_signals_simple, luaH_object_emit_signal_simple,
    luaH_object_tostring, luaH_object_gc,
};
use self::luauniq_h::{luaH_uniq_add, luaH_uniq_get};
#[c2rust::src_loc = "30:20"]
static mut ipc_channel_class: lua_class_t = lua_class_t {
    name: 0 as *const gchar,
    signals: 0 as *const signal_t as *mut signal_t,
    allocator: None,
    properties: 0 as *const lua_class_property_array_t
        as *mut lua_class_property_array_t,
    index_miss_property: None,
    newindex_miss_property: None,
};
#[inline]
#[c2rust::src_loc = "32:1"]
unsafe extern "C" fn luaH_ipc_channel_class_add_signal(mut L: *mut lua_State) -> gint {
    luaH_class_add_signal(
        L,
        &mut ipc_channel_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[inline]
#[c2rust::src_loc = "32:1"]
unsafe extern "C" fn ipc_channel_new(mut L: *mut lua_State) -> *mut ipc_channel_t {
    let mut p: *mut ipc_channel_t = lua_newuserdata(
        L,
        ::core::mem::size_of::<ipc_channel_t>() as std::ffi::c_ulong,
    ) as *mut ipc_channel_t;
    memset(
        p as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        (::core::mem::size_of::<ipc_channel_t>() as std::ffi::c_ulong)
            .wrapping_mul(1 as std::ffi::c_int as std::ffi::c_ulong),
    );
    (*p).signals = signal_new();
    luaH_settype(L, &mut ipc_channel_class);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_setmetatable(L, -(2 as std::ffi::c_int));
    lua_setfenv(L, -(2 as std::ffi::c_int));
    lua_pushvalue(L, -(1 as std::ffi::c_int));
    luaH_class_emit_signal(
        L,
        &mut ipc_channel_class,
        b"new\0" as *const u8 as *const std::ffi::c_char,
        1 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    return p;
}
#[inline]
#[c2rust::src_loc = "32:1"]
unsafe extern "C" fn luaH_ipc_channel_class_remove_signal(
    mut L: *mut lua_State,
) -> gint {
    luaH_class_remove_signal(
        L,
        &mut ipc_channel_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[inline]
#[c2rust::src_loc = "32:1"]
unsafe extern "C" fn luaH_ipc_channel_class_emit_signal(mut L: *mut lua_State) -> gint {
    return luaH_class_emit_signal(
        L,
        &mut ipc_channel_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t),
        lua_gettop(L) - 1 as std::ffi::c_int,
        -(1 as std::ffi::c_int),
    );
}
#[no_mangle]
#[c2rust::src_loc = "34:1"]
pub unsafe extern "C" fn luaH_check_ipc_channel(
    mut L: *mut lua_State,
    mut idx: gint,
) -> *mut ipc_channel_t {
    return luaH_checkudata(L, idx, &mut ipc_channel_class) as *mut ipc_channel_t;
}
#[no_mangle]
#[c2rust::src_loc = "40:1"]
pub unsafe extern "C" fn luaH_ipc_channel_new(mut L: *mut lua_State) -> gint {
    let mut name: *const std::ffi::c_char = luaL_checklstring(
        L,
        -(1 as std::ffi::c_int),
        0 as *mut size_t,
    );
    if luaH_uniq_get(
        L,
        b"luakit.registry.ipc_channel\0" as *const u8 as *const std::ffi::c_char,
        -(1 as std::ffi::c_int),
    ) != 0
    {
        return 1 as std::ffi::c_int;
    }
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    luaH_class_new(L, &mut ipc_channel_class);
    lua_remove(L, -(2 as std::ffi::c_int));
    let mut ipc_channel: *mut ipc_channel_t = luaH_check_ipc_channel(
        L,
        -(1 as std::ffi::c_int),
    );
    (*ipc_channel).name = g_strdup_inline(name);
    luaH_uniq_add(
        L,
        b"luakit.registry.ipc_channel\0" as *const u8 as *const std::ffi::c_char,
        -(2 as std::ffi::c_int),
        -(1 as std::ffi::c_int),
    );
    return 1 as std::ffi::c_int;
}
#[c2rust::src_loc = "58:1"]
unsafe extern "C" fn luaH_ipc_channel_gc(mut L: *mut lua_State) -> gint {
    let mut ipc_channel: *mut ipc_channel_t = luaH_check_ipc_channel(
        L,
        -(1 as std::ffi::c_int),
    );
    g_free((*ipc_channel).name as gpointer);
    return luaH_object_gc(L);
}
#[no_mangle]
#[c2rust::src_loc = "66:1"]
pub unsafe extern "C" fn ipc_channel_class_setup(mut L: *mut lua_State) {
    static mut ipc_channel_methods: [luaL_Reg; 5] = unsafe {
        [
            {
                let mut init = luaL_Reg {
                    name: b"add_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_ipc_channel_class_add_signal
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"remove_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_ipc_channel_class_remove_signal
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"emit_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_ipc_channel_class_emit_signal
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__call\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_ipc_channel_new
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
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
    static mut ipc_channel_meta: [luaL_Reg; 8] = unsafe {
        [
            {
                let mut init = luaL_Reg {
                    name: b"__tostring\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_object_tostring
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"add_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_object_add_signal_simple
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"remove_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_object_remove_signal_simple
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"remove_signals\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_object_remove_signals_simple
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"emit_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_object_emit_signal_simple
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"emit_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        ipc_channel_send as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__gc\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_ipc_channel_gc
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
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
    luaH_class_setup(
        L,
        &mut ipc_channel_class,
        b"ipc_channel\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut lua_State) -> *mut ipc_channel_t>,
            lua_class_allocator_t,
        >(
            Some(
                ipc_channel_new
                    as unsafe extern "C" fn(*mut lua_State) -> *mut ipc_channel_t,
            ),
        ),
        None,
        None,
        ipc_channel_methods.as_ptr(),
        ipc_channel_meta.as_ptr(),
    );
    lua_pushstring(
        L,
        b"luakit.registry.ipc_channel\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_rawset(L, -(10000 as std::ffi::c_int));
}
