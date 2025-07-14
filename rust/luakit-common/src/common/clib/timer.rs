use glib_sys::{
    GMainContext, GSource, g_main_context_find_source_by_id, g_source_destroy, g_timeout_add,
    gboolean, gpointer,
};
use libc::{c_void, memset, size_t};
use mlua::ffi::{
    lua_Integer, lua_State, lua_createtable, lua_gettop, lua_newuserdata, lua_pushboolean,
    lua_pushinteger, lua_pushvalue, lua_setmetatable, lua_settop, lua_tolstring, luaL_Reg,
    luaL_checkinteger, luaL_checklstring, luaL_error,
};

use crate::common::common;
use crate::common::luaclass::signal_h::{signal_new, signal_t};
use crate::common::luaclass::{
    lua_class_allocator_t, lua_class_property_array_t, lua_class_propfunc_t, lua_class_t,
    lua_object_t, luaH_checkudata, luaH_class_add_property, luaH_class_add_signal,
    luaH_class_emit_signal, luaH_class_index, luaH_class_new, luaH_class_newindex,
    luaH_class_remove_signal, luaH_class_setup,
};
use crate::common::luah::luaH_warn;
use crate::common::luaobject::{
    luaH_object_add_signal_simple, luaH_object_emit_signal, luaH_object_emit_signal_simple,
    luaH_object_gc, luaH_object_push, luaH_object_ref, luaH_object_remove_signal_simple,
    luaH_object_remove_signals_simple, luaH_object_tostring, luaH_object_unref, luaH_settype,
};
use crate::common::tokenize::{L_TK_INTERVAL, L_TK_STARTED};

use crate::gtypes::{gchar, gint, guint};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ltimer_t {
    pub signals: *mut signal_t,
    pub ref_0: gpointer,
    pub id: std::ffi::c_int,
    pub interval: std::ffi::c_int,
}
static mut timer_class: lua_class_t = lua_class_t {
    name: 0 as *const gchar,
    signals: 0 as *const signal_t as *mut signal_t,
    allocator: None,
    properties: 0 as *const lua_class_property_array_t as *mut lua_class_property_array_t,
    index_miss_property: None,
    newindex_miss_property: None,
};
#[inline]
unsafe extern "C-unwind" fn timer_new(mut L: *mut lua_State) -> *mut ltimer_t {
    let mut p: *mut ltimer_t =
        lua_newuserdata(L, ::core::mem::size_of::<ltimer_t>()) as *mut ltimer_t;
    memset(
        p as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        (::core::mem::size_of::<ltimer_t>()).wrapping_mul(1),
    );
    (*p).signals = signal_new();
    luaH_settype(L, &mut timer_class);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_setmetatable(L, -(2 as std::ffi::c_int));
    // lua_setfenv(L, -(2 as std::ffi::c_int));
    lua_pushvalue(L, -(1 as std::ffi::c_int));
    luaH_class_emit_signal(
        L,
        &mut timer_class,
        b"new\0" as *const u8 as *const std::ffi::c_char,
        1 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    return p;
}
#[inline]
unsafe extern "C-unwind" fn luaH_timer_class_emit_signal(mut L: *mut lua_State) -> gint {
    return luaH_class_emit_signal(
        L,
        &mut timer_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t),
        lua_gettop(L) - 1 as std::ffi::c_int,
        -(1 as std::ffi::c_int),
    );
}
#[inline]
unsafe extern "C-unwind" fn luaH_timer_class_remove_signal(mut L: *mut lua_State) -> gint {
    luaH_class_remove_signal(
        L,
        &mut timer_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn luaH_timer_class_add_signal(mut L: *mut lua_State) -> gint {
    luaH_class_add_signal(
        L,
        &mut timer_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_timer_destroy(mut L: *mut lua_State, mut timer: *mut ltimer_t) {
    let mut source: *mut GSource =
        g_main_context_find_source_by_id(0 as *mut GMainContext, (*timer).id as guint);
    if !source.is_null() {
        g_source_destroy(source);
    }
    luaH_object_unref(L, (*timer).ref_0);
    (*timer).ref_0 = 0 as *mut std::ffi::c_void;
    (*timer).id = -(1 as std::ffi::c_int);
}
unsafe extern "C" fn timer_handle_timeout(mut data: gpointer) -> gboolean {
    let mut timer: *mut ltimer_t = data as *mut ltimer_t;
    luaH_object_push(common.L, (*timer).ref_0);
    luaH_object_emit_signal(
        common.L,
        -(1 as std::ffi::c_int),
        b"timeout\0" as *const u8 as *const std::ffi::c_char,
        0 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(common.L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    return (0 as std::ffi::c_int == 0) as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_timer_new(mut L: *mut lua_State) -> std::ffi::c_int {
    luaH_class_new(L, &mut timer_class);
    let mut timer: *mut ltimer_t =
        luaH_checkudata(L, -(1 as std::ffi::c_int), &mut timer_class) as *mut ltimer_t;
    (*timer).id = -(1 as std::ffi::c_int);
    return 1 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_timer_start(mut L: *mut lua_State) -> std::ffi::c_int {
    let mut timer: *mut ltimer_t =
        luaH_checkudata(L, 1 as std::ffi::c_int, &mut timer_class) as *mut ltimer_t;
    if (*timer).interval == 0 {
        luaL_error(
            L,
            b"interval not set\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    if (*timer).id == -(1 as std::ffi::c_int) {
        (*timer).ref_0 = luaH_object_ref(L, 1 as std::ffi::c_int);
        (*timer).id = g_timeout_add(
            (*timer).interval as guint,
            Some(timer_handle_timeout),
            timer as gpointer,
        ) as std::ffi::c_int;
    } else {
        luaH_warn(L, "timer already started");
    }
    return 0 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_timer_stop(mut L: *mut lua_State) -> std::ffi::c_int {
    let mut timer: *mut ltimer_t =
        luaH_checkudata(L, 1 as std::ffi::c_int, &mut timer_class) as *mut ltimer_t;
    if (*timer).id == -(1 as std::ffi::c_int) {
        luaH_warn(L, "timer already stopped");
    } else {
        luaH_timer_destroy(L, timer);
    }
    return 0 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_timer_set_interval(
    mut L: *mut lua_State,
    mut timer: *mut ltimer_t,
) -> std::ffi::c_int {
    (*timer).interval = luaL_checkinteger(L, -(1 as std::ffi::c_int)) as std::ffi::c_int;
    return 0 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_timer_get_interval(
    mut L: *mut lua_State,
    mut timer: *mut ltimer_t,
) -> std::ffi::c_int {
    lua_pushinteger(L, (*timer).interval as lua_Integer);
    return 1 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_timer_get_started(
    mut L: *mut lua_State,
    mut timer: *mut ltimer_t,
) -> std::ffi::c_int {
    lua_pushboolean(
        L,
        ((*timer).id != -(1 as std::ffi::c_int)) as std::ffi::c_int,
    );
    return 1 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_timer_index_miss_property(
    mut L: *mut lua_State,
    mut UNUSED_obj: *mut lua_object_t,
) -> gint {
    return luaL_error(
        L,
        b"timer index miss; key %s\0" as *const u8 as *const std::ffi::c_char,
        lua_tolstring(L, 2 as std::ffi::c_int, 0 as *mut size_t),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_timer_newindex_miss_property(
    mut L: *mut lua_State,
    mut UNUSED_obj: *mut lua_object_t,
) -> gint {
    return luaL_error(
        L,
        b"timer newindex miss; key %s\0" as *const u8 as *const std::ffi::c_char,
        lua_tolstring(L, 2 as std::ffi::c_int, 0 as *mut size_t),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn timer_class_setup(mut L: *mut lua_State) {
    let timer_methods = unsafe {
        [
            {
                let mut init = luaL_Reg {
                    name: b"add_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_timer_class_add_signal,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"remove_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_timer_class_remove_signal,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"emit_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_timer_class_emit_signal,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__call\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_timer_new,
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
    let timer_meta = unsafe {
        [
            {
                let mut init = luaL_Reg {
                    name: b"__tostring\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_object_tostring,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"add_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_object_add_signal_simple,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"remove_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_object_remove_signal_simple,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"remove_signals\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_object_remove_signals_simple,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"emit_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_object_emit_signal_simple,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__index\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_class_index,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__newindex\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_class_newindex,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"start\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_timer_start,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"stop\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_timer_stop,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__gc\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_object_gc,
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
    luaH_class_setup(
        L,
        &mut timer_class,
        b"timer\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option<unsafe extern "C-unwind" fn(*mut lua_State) -> *mut ltimer_t>,
            lua_class_allocator_t,
        >(Some(
            timer_new as unsafe extern "C-unwind" fn(*mut lua_State) -> *mut ltimer_t,
        )),
        Some(
            luaH_timer_index_miss_property
                as unsafe extern "C-unwind" fn(*mut lua_State, *mut lua_object_t) -> gint,
        ),
        Some(
            luaH_timer_newindex_miss_property
                as unsafe extern "C-unwind" fn(*mut lua_State, *mut lua_object_t) -> gint,
        ),
        timer_methods.as_ptr(),
        timer_meta.as_ptr(),
    );
    luaH_class_add_property(
        &mut timer_class,
        L_TK_INTERVAL,
        ::core::mem::transmute::<
            Option<unsafe extern "C-unwind" fn(*mut lua_State, *mut ltimer_t) -> std::ffi::c_int>,
            lua_class_propfunc_t,
        >(Some(
            luaH_timer_set_interval
                as unsafe extern "C-unwind" fn(*mut lua_State, *mut ltimer_t) -> std::ffi::c_int,
        )),
        ::core::mem::transmute::<
            Option<unsafe extern "C-unwind" fn(*mut lua_State, *mut ltimer_t) -> std::ffi::c_int>,
            lua_class_propfunc_t,
        >(Some(
            luaH_timer_get_interval
                as unsafe extern "C-unwind" fn(*mut lua_State, *mut ltimer_t) -> std::ffi::c_int,
        )),
        ::core::mem::transmute::<
            Option<unsafe extern "C-unwind" fn(*mut lua_State, *mut ltimer_t) -> std::ffi::c_int>,
            lua_class_propfunc_t,
        >(Some(
            luaH_timer_set_interval
                as unsafe extern "C-unwind" fn(*mut lua_State, *mut ltimer_t) -> std::ffi::c_int,
        )),
    );
    luaH_class_add_property(
        &mut timer_class,
        L_TK_STARTED,
        None,
        ::core::mem::transmute::<
            Option<unsafe extern "C-unwind" fn(*mut lua_State, *mut ltimer_t) -> std::ffi::c_int>,
            lua_class_propfunc_t,
        >(Some(
            luaH_timer_get_started
                as unsafe extern "C-unwind" fn(*mut lua_State, *mut ltimer_t) -> std::ffi::c_int,
        )),
        None,
    );
}
