use std::ffi::CStr;

use ::libc;
use libc::size_t;
use mlua_sys::*;
pub mod signal_h {
    pub type signal_t = GTree;
    pub type signal_array_t = GPtrArray;
    #[inline]
    pub unsafe extern "C-unwind" fn signal_cmp(
        mut a: gconstpointer,
        mut b: gconstpointer,
        mut UNUSED_p: gpointer,
    ) -> gint {
        return g_strcmp0(a as *const std::ffi::c_char, b as *const std::ffi::c_char);
    }
    #[inline]
    pub unsafe extern "C-unwind" fn signal_array_destroy(mut sigfuncs: *mut gpointer) {
        g_ptr_array_free(
            sigfuncs as *mut GPtrArray,
            (0 as std::ffi::c_int == 0) as std::ffi::c_int,
        );
    }
    #[inline]
    pub unsafe extern "C-unwind" fn signal_new() -> *mut signal_t {
        return g_tree_new_full(
            ::core::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(gconstpointer, gconstpointer, gpointer) -> gint>,
                GCompareDataFunc,
            >(Some(
                signal_cmp
                    as unsafe extern "C-unwind" fn(gconstpointer, gconstpointer, gpointer) -> gint,
            )),
            0 as *mut std::ffi::c_void,
            Some(g_free),
            ::core::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(*mut gpointer) -> ()>,
                GDestroyNotify,
            >(Some(
                signal_array_destroy as unsafe extern "C-unwind" fn(*mut gpointer) -> (),
            )),
        ) as *mut signal_t;
    }
    #[inline]
    pub unsafe extern "C-unwind" fn signal_lookup(
        mut signals: *mut signal_t,
        mut name: *const gchar,
    ) -> *mut signal_array_t {
        return g_tree_lookup(signals as *mut GTree, name as gpointer as gconstpointer)
            as *mut signal_array_t;
    }
    #[inline]
    pub unsafe extern "C-unwind" fn signal_add(
        mut signals: *mut signal_t,
        mut name: *const gchar,
        mut func: gpointer,
    ) {
        let mut sigfuncs: *mut signal_array_t = signal_lookup(signals, name);
        if sigfuncs.is_null() {
            sigfuncs = g_ptr_array_new() as *mut signal_array_t;
            g_tree_insert(
                signals as *mut GTree,
                g_strdup(name) as gpointer,
                sigfuncs as gpointer,
            );
        }
        g_ptr_array_add(sigfuncs as *mut GPtrArray, func);
    }
    #[inline]
    pub unsafe extern "C-unwind" fn signal_remove(
        mut signals: *mut signal_t,
        mut name: *const gchar,
        mut func: gpointer,
    ) {
        let mut sigfuncs: *mut signal_array_t = signal_lookup(signals, name);
        if !sigfuncs.is_null() {
            g_ptr_array_remove(sigfuncs as *mut GPtrArray, func);
            if (*sigfuncs).len == 0 {
                g_tree_remove(signals as *mut GTree, name as gpointer as gconstpointer);
            }
        }
    }

    pub unsafe extern "C-unwind" fn signals_remove(
        mut signals: *mut signal_t,
        mut name: *const gchar,
    ) {
        let mut sigfuncs: *mut signal_array_t = signal_lookup(signals, name);
        if !sigfuncs.is_null() {
            g_ptr_array_remove(sigfuncs as *mut GPtrArray, std::ptr::null_mut());
            if (*sigfuncs).len == 0 {
                g_tree_remove(signals as *mut GTree, name as gpointer as gconstpointer);
            }
        }
    }

    pub unsafe extern "C-unwind" fn signal_destroy(mut signals: *mut signal_t) {
        g_tree_destroy(signals as *mut GTree);
    }
    use glib_sys::*;

    use crate::gtypes::{gchar, gint};
}
pub type lua_class_propfunc_t =
    Option<unsafe extern "C-unwind" fn(*mut lua_State, *mut lua_object_t) -> gint>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lua_object_t {
    pub signals: *mut signal_t,
}
pub type lua_class_property_t = lua_class_property;
pub type lua_class_property_array_t = GHashTable;
pub type lua_class_allocator_t =
    Option<unsafe extern "C-unwind" fn(*mut lua_State) -> *mut lua_object_t>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lua_class_t {
    pub name: *const gchar,
    pub signals: *mut signal_t,
    pub allocator: lua_class_allocator_t,
    pub properties: *mut lua_class_property_array_t,
    pub index_miss_property: lua_class_propfunc_t,
    pub newindex_miss_property: lua_class_propfunc_t,
}
use crate::{
    common::{
        luaclass::signal_h::{signal_add, signal_new, signal_remove},
        luaobject::{luaH_object_ref, luaH_object_unref, signal_object_emit},
        tokenize::{L_TK_UNKNOWN, l_tokenize, luakit_token_t, token_tostring},
        util::luaH_callerinfo,
    },
    gtypes::{gchar, gint, guint},
    log::{_log, LOG_LEVEL_debug},
};
use glib_sys::{
    GHashTable, GPtrArray, g_assertion_message_expr, g_direct_equal, g_direct_hash, g_free,
    g_hash_table_insert, g_hash_table_lookup, g_hash_table_new, g_malloc0_n, g_ptr_array_add,
    g_ptr_array_new, g_strdup_printf, gboolean, gconstpointer, gpointer,
};
use mlua_sys::lua_State;
use signal_h::signal_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct lua_class_property {
    pub new: lua_class_propfunc_t,
    pub index: lua_class_propfunc_t,
    pub newindex: lua_class_propfunc_t,
}
static mut luaH_classes: *mut GPtrArray = 0 as *const GPtrArray as *mut GPtrArray;
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_toudata(
    mut L: *mut lua_State,
    mut ud: gint,
    mut class: *mut lua_class_t,
) -> gpointer {
    let mut p: gpointer = lua_touserdata(L, ud);
    if !p.is_null() {
        if lua_getmetatable(L, ud) != 0 {
            lua_pushlightuserdata(L, class as *mut std::ffi::c_void);
            lua_rawget(L, -(10000 as std::ffi::c_int));
            if lua_rawequal(L, -(1 as std::ffi::c_int), -(2 as std::ffi::c_int)) == 0 {
                p = 0 as *mut std::ffi::c_void;
            }
            lua_settop(L, -(2 as std::ffi::c_int) - 1 as std::ffi::c_int);
        }
    }
    return p;
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_checkudata(
    mut L: *mut lua_State,
    mut ud: gint,
    mut class: *mut lua_class_t,
) -> gpointer {
    let mut p: gpointer = luaH_toudata(L, ud, class);
    if p.is_null() {
        luaL_argerror(L, ud, (*class).name);
    }
    return p;
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_class_get(
    mut L: *mut lua_State,
    mut idx: gint,
) -> *mut lua_class_t {
    let mut type_0: gint = lua_type(L, idx);
    let mut class: *mut lua_class_t = 0 as *mut lua_class_t;
    if type_0 == 7 as std::ffi::c_int && !luaH_classes.is_null() {
        let mut i: guint = 0 as std::ffi::c_int as guint;
        while i < (*luaH_classes).len {
            class = *((*luaH_classes).pdata).offset(i as isize) as *mut lua_class_t;
            if !(luaH_toudata(L, idx, class)).is_null() {
                return class;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return 0 as *mut lua_class_t;
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_typename(
    mut L: *mut lua_State,
    mut idx: gint,
) -> *const gchar {
    let mut type_0: gint = lua_type(L, idx);
    if type_0 == 7 as std::ffi::c_int {
        let mut lua_class: *mut lua_class_t = luaH_class_get(L, idx);
        if !lua_class.is_null() {
            return (*lua_class).name;
        }
    }
    return lua_typename(L, type_0);
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_openlib(
    mut L: *mut lua_State,
    mut name: *const gchar,
    mut methods: *const luaL_Reg,
    mut meta: *const luaL_Reg,
) {
    luaL_newmetatable(L, name);
    lua_pushvalue(L, -(1 as std::ffi::c_int));
    lua_setfield(
        L,
        -(2 as std::ffi::c_int),
        b"__index\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_register(L, 0 as *const std::ffi::c_char, (*meta).func);
    lua_register(L, name, (*methods).func);
    lua_pushvalue(L, -(1 as std::ffi::c_int));
    lua_setmetatable(L, -(2 as std::ffi::c_int));
    lua_settop(L, -(2 as std::ffi::c_int) - 1 as std::ffi::c_int);
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_class_add_property(
    mut lua_class: *mut lua_class_t,
    mut token: luakit_token_t,
    mut cb_new: lua_class_propfunc_t,
    mut cb_index: lua_class_propfunc_t,
    mut cb_newindex: lua_class_propfunc_t,
) {
    let mut prop: *mut lua_class_property_t = 0 as *mut lua_class_property_t;
    if token as std::ffi::c_uint != L_TK_UNKNOWN as std::ffi::c_int as std::ffi::c_uint {
    } else {
        g_assertion_message_expr(
            0 as *mut gchar,
            b"common/luaclass.c\0" as *const u8 as *const std::ffi::c_char,
            154 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 24], &[std::ffi::c_char; 24]>(
                b"luaH_class_add_property\0",
            ))
            .as_ptr(),
            b"token != L_TK_UNKNOWN\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    prop = g_malloc0_n(1 as usize, ::core::mem::size_of::<lua_class_property_t>())
        as *mut lua_class_property_t;
    (*prop).new = cb_new;
    (*prop).index = cb_index;
    (*prop).newindex = cb_newindex;
    g_hash_table_insert(
        (*lua_class).properties as *mut GHashTable,
        token as gpointer,
        prop as gpointer,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_class_setup(
    mut L: *mut lua_State,
    mut class: *mut lua_class_t,
    mut name: *const gchar,
    mut allocator: lua_class_allocator_t,
    mut index_miss_property: lua_class_propfunc_t,
    mut newindex_miss_property: lua_class_propfunc_t,
    mut methods: *const luaL_Reg,
    mut meta: *const luaL_Reg,
) {
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_pushlightuserdata(L, class as *mut std::ffi::c_void);
    lua_pushvalue(L, -(2 as std::ffi::c_int));
    lua_rawset(L, -(10000 as std::ffi::c_int));
    lua_pushvalue(L, -(1 as std::ffi::c_int));
    lua_setfield(
        L,
        -(2 as std::ffi::c_int),
        b"__index\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_register(L, 0 as *const std::ffi::c_char, (*meta).func);
    if !methods.is_null() {
        lua_register(L, name, (*methods).func);
        lua_pushvalue(L, -(1 as std::ffi::c_int));
        lua_setmetatable(L, -(2 as std::ffi::c_int));
        lua_settop(L, -(2 as std::ffi::c_int) - 1 as std::ffi::c_int);
    } else {
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    }
    (*class).allocator = allocator;
    (*class).name = name;
    (*class).index_miss_property = index_miss_property;
    (*class).newindex_miss_property = newindex_miss_property;
    (*class).signals = signal_new();
    (*class).properties = g_hash_table_new(Some(g_direct_hash), Some(g_direct_equal))
        as *mut lua_class_property_array_t;
    if luaH_classes.is_null() {
        luaH_classes = g_ptr_array_new();
    }
    g_ptr_array_add(luaH_classes, class as gpointer);
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_class_add_signal(
    mut L: *mut lua_State,
    mut lua_class: *mut lua_class_t,
    mut name: *const gchar,
    mut ud: gint,
) {
    if !(lua_type(L, ud) == 6 as std::ffi::c_int) {
        luaL_argerror(L, ud, b"function\0" as *const u8 as *const std::ffi::c_char);
    }
    let mut origin: *mut gchar = luaH_callerinfo(L);
    _log(
        LOG_LEVEL_debug,
        b"common/luaclass.c\0" as *const u8 as *const std::ffi::c_char,
        &format!(
            "add \x1B[34m\"{}\"\x1B[0m on {} from \x1B[32m{}\x1B[0m\0",
            CStr::from_ptr(name).to_string_lossy(),
            lua_class as usize,
            CStr::from_ptr(origin).to_string_lossy()
        ),
    );
    g_free(origin as gpointer);
    signal_add((*lua_class).signals, name, luaH_object_ref(L, ud));
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_class_remove_signal(
    mut L: *mut lua_State,
    mut lua_class: *mut lua_class_t,
    mut name: *const gchar,
    mut ud: gint,
) {
    if !(lua_type(L, ud) == 6 as std::ffi::c_int) {
        luaL_argerror(L, ud, b"function\0" as *const u8 as *const std::ffi::c_char);
    }
    let mut ref_0: gpointer = lua_topointer(L, ud) as gpointer;
    signal_remove((*lua_class).signals, name, ref_0);
    luaH_object_unref(L, ref_0);
    lua_remove(L, ud);
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_class_emit_signal(
    mut L: *mut lua_State,
    mut lua_class: *mut lua_class_t,
    mut name: *const gchar,
    mut nargs: gint,
    mut nret: gint,
) -> gint {
    return signal_object_emit(L, (*lua_class).signals, name, nargs, nret);
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_class_property_signal(
    mut L: *mut lua_State,
    mut lua_class: *mut lua_class_t,
    mut tok: luakit_token_t,
) -> gint {
    let mut signame: *mut gchar = g_strdup_printf(
        b"property::%s\0" as *const u8 as *const std::ffi::c_char,
        token_tostring(tok),
    );
    signal_object_emit(
        L,
        (*lua_class).signals,
        signame,
        0 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    g_free(signame as gpointer);
    return 0 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_usemetatable(
    mut L: *mut lua_State,
    mut idxobj: gint,
    mut idxfield: gint,
) -> gint {
    lua_getmetatable(L, idxobj);
    lua_pushvalue(L, idxfield);
    lua_rawget(L, -(2 as std::ffi::c_int));
    if !(lua_type(L, -(1 as std::ffi::c_int)) == 0 as std::ffi::c_int) {
        lua_remove(L, -(2 as std::ffi::c_int));
        return 1 as std::ffi::c_int;
    }
    lua_settop(L, -(2 as std::ffi::c_int) - 1 as std::ffi::c_int);
    return 0 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_class_property_get(
    mut L: *mut lua_State,
    mut lua_class: *mut lua_class_t,
    mut fieldidx: gint,
) -> *mut lua_class_property_t {
    let mut attr: *const gchar = luaL_checklstring(L, fieldidx, 0 as *mut usize);
    let mut token: luakit_token_t = l_tokenize(attr);
    return g_hash_table_lookup(
        (*lua_class).properties as *mut GHashTable,
        token as gpointer as gconstpointer,
    ) as *mut lua_class_property_t;
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_class_index(mut L: *mut lua_State) -> gint {
    if luaH_usemetatable(L, 1 as std::ffi::c_int, 2 as std::ffi::c_int) != 0 {
        return 1 as std::ffi::c_int;
    }
    let mut class: *mut lua_class_t = luaH_class_get(L, 1 as std::ffi::c_int);
    let mut prop: *mut lua_class_property_t =
        luaH_class_property_get(L, class, 2 as std::ffi::c_int);
    if !prop.is_null() {
        if ((*prop).index).is_some() {
            return ((*prop).index).expect("non-null function pointer")(
                L,
                luaH_checkudata(L, 1 as std::ffi::c_int, class) as *mut lua_object_t,
            );
        }
    } else if ((*class).index_miss_property).is_some() {
        return ((*class).index_miss_property).expect("non-null function pointer")(
            L,
            luaH_checkudata(L, 1 as std::ffi::c_int, class) as *mut lua_object_t,
        );
    }
    return 0 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_class_newindex(mut L: *mut lua_State) -> gint {
    if luaH_usemetatable(L, 1 as std::ffi::c_int, 2 as std::ffi::c_int) != 0 {
        return 1 as std::ffi::c_int;
    }
    let mut class: *mut lua_class_t = luaH_class_get(L, 1 as std::ffi::c_int);
    let mut prop: *mut lua_class_property_t =
        luaH_class_property_get(L, class, 2 as std::ffi::c_int);
    if !prop.is_null() {
        if ((*prop).newindex).is_some() {
            return ((*prop).newindex).expect("non-null function pointer")(
                L,
                luaH_checkudata(L, 1 as std::ffi::c_int, class) as *mut lua_object_t,
            );
        }
    } else if ((*class).newindex_miss_property).is_some() {
        return ((*class).newindex_miss_property).expect("non-null function pointer")(
            L,
            luaH_checkudata(L, 1 as std::ffi::c_int, class) as *mut lua_object_t,
        );
    }
    return 0 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_class_new(
    mut L: *mut lua_State,
    mut lua_class: *mut lua_class_t,
) -> gint {
    let mut idx: gint = lua_gettop(L);
    if !(lua_type(L, idx) == 5 as std::ffi::c_int) {
        luaL_argerror(L, idx, b"table\0" as *const u8 as *const std::ffi::c_char);
    }
    let mut object: *mut lua_object_t =
        ((*lua_class).allocator).expect("non-null function pointer")(L);
    lua_pushnil(L);
    while lua_next(L, idx) != 0 {
        if lua_isstring(L, -(2 as std::ffi::c_int)) != 0 {
            let mut attr: *const std::ffi::c_char =
                lua_tolstring(L, -(2 as std::ffi::c_int), 0 as *mut usize);
            let mut prop: *mut lua_class_property_t = g_hash_table_lookup(
                (*lua_class).properties as *mut GHashTable,
                l_tokenize(attr) as gpointer as gconstpointer,
            ) as *mut lua_class_property_t;
            if !prop.is_null() && ((*prop).new).is_some() {
                ((*prop).new).expect("non-null function pointer")(L, object);
            }
        }
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    }
    return 1 as std::ffi::c_int;
}
