use gdk_sys::*;
use glib_sys::*;
use javascriptcore_rs_sys::*;
use libc::*;
use mlua_sys::*;
use webkit2gtk::glib::gobject_ffi::*;

use crate::clib::luakit::*;
use crate::clib::msg::*;
use crate::clib::soup::*;
use crate::clib::sqlite3::*;
use crate::clib::stylesheet::*;
use crate::clib::web_module::*;
use crate::clib::widget::*;
use crate::common::luaclass::*;
use crate::common::luah::*;
use crate::common::lualib::*;
use crate::common::luaobject::*;
use crate::common::luautil::*;
use crate::common::util::*;
use crate::common::*;
use crate::globalconf::*;
use crate::gtypes::*;
use crate::log::*;
use crate::luah::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct cb_data {
    pub ctx: *mut luajs_func_ctx_t,
    pub context: *mut JSCContext,
}
pub type luajs_func_ctx_t = _luajs_func_ctx_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _luajs_func_ctx_t {
    pub ref_0: gpointer,
    pub page_id: guint64,
}
pub type js_promise_t = _js_promise_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _js_promise_t {
    pub promise: *mut JSCValue,
    pub resolve: *mut JSCValue,
    pub reject: *mut JSCValue,
}
static mut lua_string_find_ref: gint = LUA_REFNIL;
unsafe extern "C" fn promise_executor_cb(
    mut resolve: *mut JSCValue,
    mut reject: *mut JSCValue,
    mut promise: *mut js_promise_t,
) {
    if jsc_value_is_function(resolve) != 0 {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN as *const std::ffi::c_char,
            b"extension/luajs.c\0" as *const u8 as *const std::ffi::c_char,
            54 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 20], &[std::ffi::c_char; 20]>(
                b"promise_executor_cb\0",
            ))
            .as_ptr(),
            b"jsc_value_is_function(resolve)\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    if jsc_value_is_function(reject) != 0 {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN as *const std::ffi::c_char,
            b"extension/luajs.c\0" as *const u8 as *const std::ffi::c_char,
            55 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 20], &[std::ffi::c_char; 20]>(
                b"promise_executor_cb\0",
            ))
            .as_ptr(),
            b"jsc_value_is_function(reject)\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    g_object_ref(resolve as gpointer);
    g_object_ref(reject as gpointer);
    (*promise).resolve = resolve;
    (*promise).reject = reject;
}
unsafe extern "C" fn new_promise(mut context: *mut JSCContext, mut promise: *mut js_promise_t) {
    let mut promise_ctor = jsc_context_get_value(
        context,
        b"Promise\0" as *const u8 as *const std::ffi::c_char,
    );
    if jsc_value_is_constructor(promise_ctor) != 0 {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN as *const std::ffi::c_char,
            b"extension/luajs.c\0" as *const u8 as *const std::ffi::c_char,
            68 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 12], &[std::ffi::c_char; 12]>(b"new_promise\0"))
                .as_ptr(),
            b"jsc_value_is_constructor(promise_ctor)\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    let mut func = jsc_value_new_function(
        context,
        NULL as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut JSCValue, *mut JSCValue, *mut js_promise_t) -> ()>,
            GCallback,
        >(Some(
            promise_executor_cb
                as unsafe extern "C" fn(*mut JSCValue, *mut JSCValue, *mut js_promise_t) -> (),
        )),
        promise as gpointer,
        ::core::mem::transmute::<libc::intptr_t, GDestroyNotify>(NULL as libc::intptr_t),
        G_TYPE_NONE as GType,
        2 as std::ffi::c_int as guint,
        JSC_TYPE_VALUE,
        JSC_TYPE_VALUE,
    );
    (*promise).promise =
        jsc_value_constructor_call(promise_ctor, JSC_TYPE_VALUE, func, G_TYPE_NONE as GType);
    g_object_unref(func as gpointer);
    g_object_unref(promise_ctor as gpointer);
}
unsafe extern "C" fn luaJS_promise_resolve_reject(mut L: *mut lua_State) -> std::ffi::c_int {
    let mut page_id = lua_tointeger(L, LUA_GLOBALSINDEX - 1 as std::ffi::c_int) as guint64;
    let mut page = webkit_web_extension_get_page(extension.ext, page_id);
    if page.is_null()
        || ({
            let mut __inst = page as *mut GTypeInstance;
            let mut __t = webkit_web_page_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = FALSE;
            } else if !((*__inst).g_class).is_null() && (*(*__inst).g_class).g_type == __t {
                __r = TRUE;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) == 0
    {
        return luaL_error(
            L,
            b"promise no longer valid (associated page closed)\0" as *const u8
                as *const std::ffi::c_char,
        );
    }
    let mut context = webkit_frame_get_js_context(webkit_web_page_get_main_frame(page));
    let mut promise =
        lua_topointer(L, LUA_GLOBALSINDEX - 2 as std::ffi::c_int) as *mut js_promise_t;
    let mut cb = if lua_toboolean(L, LUA_GLOBALSINDEX - 3 as std::ffi::c_int) != 0 {
        (*promise).resolve
    } else {
        (*promise).reject
    };
    let mut ret = luajs_tovalue(L, 1 as std::ffi::c_int, context);
    let mut undefined = jsc_value_function_call(cb, JSC_TYPE_VALUE, ret, G_TYPE_NONE as GType);
    g_object_unref(undefined as gpointer);
    g_object_unref((*promise).reject as gpointer);
    g_object_unref((*promise).resolve as gpointer);
    g_slice_free1(
        ::core::mem::size_of::<js_promise_t>() as std::ffi::c_ulong,
        promise as gpointer,
    );
    g_object_unref(ret as gpointer);
    g_object_unref(context as gpointer);
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn luaJS_registered_function_callback(
    mut args: *mut GPtrArray,
    mut user_data: *mut cb_data,
) -> *mut JSCValue {
    let mut L = common.L;
    let mut top = lua_gettop(L);
    let mut ctx = (*user_data).ctx;
    let mut context = (*user_data).context;
    let mut argc = (*args).len;
    let mut argv = (*args).pdata as *mut *mut JSCValue;
    let mut promise = g_slice_alloc(::core::mem::size_of::<js_promise_t>() as std::ffi::c_ulong)
        as *mut js_promise_t;
    new_promise(context, promise);
    luaH_page_from_web_page(
        L,
        webkit_web_extension_get_page(extension.ext, (*ctx).page_id),
    );
    lua_pushinteger(L, (*ctx).page_id as lua_Integer);
    lua_pushlightuserdata(L, promise as *mut std::ffi::c_void);
    lua_pushboolean(L, TRUE);
    lua_pushcclosure(
        L,
        Some(
            luaJS_promise_resolve_reject as unsafe extern "C" fn(*mut lua_State) -> std::ffi::c_int,
        ),
        3 as std::ffi::c_int,
    );
    lua_pushinteger(L, (*ctx).page_id as lua_Integer);
    lua_pushlightuserdata(L, promise as *mut std::ffi::c_void);
    lua_pushboolean(L, FALSE);
    lua_pushcclosure(
        L,
        Some(
            luaJS_promise_resolve_reject as unsafe extern "C" fn(*mut lua_State) -> std::ffi::c_int,
        ),
        3 as std::ffi::c_int,
    );
    let mut i = 0 as std::ffi::c_int as guint;
    while i < argc {
        if luajs_pushvalue(L, *argv.offset(i as isize)) != 0 {
            i = i.wrapping_add(1);
            i;
        } else {
            jsc_context_throw_exception(
                context,
                jsc_exception_new_printf(
                    context,
                    b"bad argument #%d to Lua function\0" as *const u8 as *const std::ffi::c_char,
                    i,
                ),
            );
            lua_settop(L, top);
            return jsc_value_new_undefined(context);
        }
    }
    luaH_object_push(L, (*ctx).ref_0);
    luaH_dofunction(
        L,
        argc.wrapping_add(3 as std::ffi::c_int as guint) as gint,
        0 as std::ffi::c_int,
    );
    lua_settop(L, top);
    return (*promise).promise;
}
unsafe extern "C" fn luaJS_registered_function_destroy(mut user_data: *mut std::ffi::c_void) {
    let mut cb_data = user_data as *mut cb_data;
    g_object_unref((*cb_data).context as gpointer);
    g_slice_free1(
        ::core::mem::size_of::<luajs_func_ctx_t>() as std::ffi::c_ulong,
        (*cb_data).ctx as gpointer,
    );
    g_slice_free1(
        ::core::mem::size_of::<cb_data>() as std::ffi::c_ulong,
        cb_data as gpointer,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaJS_register_function(mut L: *mut lua_State) {
    if lua_isstring(L, -(3 as std::ffi::c_int)) != 0 {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN as *const std::ffi::c_char,
            b"extension/luajs.c\0" as *const u8 as *const std::ffi::c_char,
            163 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 24], &[std::ffi::c_char; 24]>(
                b"luaJS_register_function\0",
            ))
            .as_ptr(),
            b"lua_isstring(L, -3)\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    if lua_isstring(L, -(2 as std::ffi::c_int)) != 0 {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN as *const std::ffi::c_char,
            b"extension/luajs.c\0" as *const u8 as *const std::ffi::c_char,
            164 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 24], &[std::ffi::c_char; 24]>(
                b"luaJS_register_function\0",
            ))
            .as_ptr(),
            b"lua_isstring(L, -2)\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    if lua_type(L, -(1 as std::ffi::c_int)) == 6 as std::ffi::c_int {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN as *const std::ffi::c_char,
            b"extension/luajs.c\0" as *const u8 as *const std::ffi::c_char,
            165 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 24], &[std::ffi::c_char; 24]>(
                b"luaJS_register_function\0",
            ))
            .as_ptr(),
            b"lua_isfunction(L, -1)\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    lua_pushlstring(
        L,
        b"luakit.luajs.registry\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 22]>() as std::ffi::c_ulong)
            .wrapping_div(::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong)
            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
    );
    lua_rawget(L, LUA_REGISTRYINDEX);
    lua_pushvalue(L, -(4 as std::ffi::c_int));
    lua_rawget(L, -(2 as std::ffi::c_int));
    if lua_type(L, -(1 as std::ffi::c_int)) == LUA_TNIL {
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        lua_pushvalue(L, -(4 as std::ffi::c_int));
        lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
        lua_rawset(L, -(3 as std::ffi::c_int));
        lua_pushvalue(L, -(4 as std::ffi::c_int));
        lua_rawget(L, -(2 as std::ffi::c_int));
    }
    lua_replace(L, -(2 as std::ffi::c_int));
    lua_insert(L, -(3 as std::ffi::c_int));
    lua_rawset(L, -(3 as std::ffi::c_int));
    lua_settop(L, -(2 as std::ffi::c_int) - 1 as std::ffi::c_int);
}
unsafe extern "C" fn register_func(
    mut world: *mut WebKitScriptWorld,
    mut web_page: *mut WebKitWebPage,
    mut frame: *mut WebKitFrame,
    mut name: *const gchar,
    mut ref_0: gpointer,
) {
    let mut context = webkit_frame_get_js_context_for_script_world(frame, world);
    let mut ctx = g_slice_alloc(::core::mem::size_of::<luajs_func_ctx_t>() as std::ffi::c_ulong)
        as *mut luajs_func_ctx_t;
    (*ctx).page_id = webkit_web_page_get_id(web_page);
    (*ctx).ref_0 = ref_0;
    let mut user_data =
        g_slice_alloc(::core::mem::size_of::<cb_data>() as std::ffi::c_ulong) as *mut cb_data;
    (*user_data).ctx = ctx;
    (*user_data).context = context;
    let mut fun = jsc_value_new_function_variadic(
        context,
        name,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GPtrArray, *mut cb_data) -> *mut JSCValue>,
            GCallback,
        >(Some(
            luaJS_registered_function_callback
                as unsafe extern "C" fn(*mut GPtrArray, *mut cb_data) -> *mut JSCValue,
        )),
        user_data as gpointer,
        Some(
            luaJS_registered_function_destroy as unsafe extern "C" fn(*mut std::ffi::c_void) -> (),
        ),
        JSC_TYPE_VALUE,
    );
    jsc_context_set_value(context, name, fun);
    g_object_unref(fun as gpointer);
}
unsafe extern "C" fn window_object_cleared_cb(
    mut world: *mut WebKitScriptWorld,
    mut web_page: *mut WebKitWebPage,
    mut frame: *mut WebKitFrame,
    mut UNUSED_user_data: gpointer,
) {
    if webkit_frame_is_main_frame(frame) == 0 {
        return;
    }
    let mut L = common.L;
    let ref mut fresh0 = webkit_web_page_get_uri(web_page);
    let mut uri = if !(*fresh0).is_null() {
        *fresh0
    } else {
        b"about:blank\0" as *const u8 as *const std::ffi::c_char
    };
    lua_pushlstring(
        L,
        b"luakit.luajs.registry\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 22]>() as std::ffi::c_ulong)
            .wrapping_div(::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong)
            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
    );
    lua_rawget(L, LUA_REGISTRYINDEX);
    lua_pushnil(L);
    while lua_next(L, -(2 as std::ffi::c_int)) != 0 as std::ffi::c_int {
        if lua_isstring(L, -(2 as std::ffi::c_int)) != 0 {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN as *const std::ffi::c_char,
                b"extension/luajs.c\0" as *const u8 as *const std::ffi::c_char,
                227 as std::ffi::c_int,
                (*::core::mem::transmute::<&[u8; 25], &[std::ffi::c_char; 25]>(
                    b"window_object_cleared_cb\0",
                ))
                .as_ptr(),
                b"lua_isstring(L, -2)\0" as *const u8 as *const std::ffi::c_char,
            );
        }
        if lua_type(L, -(1 as std::ffi::c_int)) == 5 as std::ffi::c_int {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN as *const std::ffi::c_char,
                b"extension/luajs.c\0" as *const u8 as *const std::ffi::c_char,
                228 as std::ffi::c_int,
                (*::core::mem::transmute::<&[u8; 25], &[std::ffi::c_char; 25]>(
                    b"window_object_cleared_cb\0",
                ))
                .as_ptr(),
                b"lua_istable(L, -1)\0" as *const u8 as *const std::ffi::c_char,
            );
        }
        lua_pushstring(L, uri);
        lua_pushvalue(L, -(3 as std::ffi::c_int));
        luaH_dofunction_from_registry(
            L,
            lua_string_find_ref,
            2 as std::ffi::c_int,
            1 as std::ffi::c_int,
        );
        if !(lua_type(L, -(1 as std::ffi::c_int)) == LUA_TNIL) {
            lua_pushnil(L);
            while lua_next(L, -(3 as std::ffi::c_int)) != 0 as std::ffi::c_int {
                if lua_isstring(L, -(2 as std::ffi::c_int)) != 0 {
                } else {
                    g_assertion_message_expr(
                        G_LOG_DOMAIN as *const std::ffi::c_char,
                        b"extension/luajs.c\0" as *const u8 as *const std::ffi::c_char,
                        240 as std::ffi::c_int,
                        (*::core::mem::transmute::<&[u8; 25], &[std::ffi::c_char; 25]>(
                            b"window_object_cleared_cb\0",
                        ))
                        .as_ptr(),
                        b"lua_isstring(L, -2)\0" as *const u8 as *const std::ffi::c_char,
                    );
                }
                if lua_type(L, -(1 as std::ffi::c_int)) == 6 as std::ffi::c_int {
                } else {
                    g_assertion_message_expr(
                        G_LOG_DOMAIN as *const std::ffi::c_char,
                        b"extension/luajs.c\0" as *const u8 as *const std::ffi::c_char,
                        241 as std::ffi::c_int,
                        (*::core::mem::transmute::<&[u8; 25], &[std::ffi::c_char; 25]>(
                            b"window_object_cleared_cb\0",
                        ))
                        .as_ptr(),
                        b"lua_isfunction(L, -1)\0" as *const u8 as *const std::ffi::c_char,
                    );
                }
                let mut ref_0 = luaH_object_ref(L, -(1 as std::ffi::c_int));
                register_func(
                    world,
                    web_page,
                    frame,
                    lua_tolstring(L, -(1 as std::ffi::c_int), NULL as *mut size_t),
                    ref_0,
                );
            }
        }
        lua_settop(L, -(2 as std::ffi::c_int) - 1 as std::ffi::c_int);
    }
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn web_luajs_init() {
    g_signal_connect_data(
        webkit_script_world_get_default() as gpointer,
        b"window-object-cleared\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut WebKitScriptWorld,
                    *mut WebKitWebPage,
                    *mut WebKitFrame,
                    gpointer,
                ) -> (),
            >,
            GCallback,
        >(Some(
            window_object_cleared_cb
                as unsafe extern "C" fn(
                    *mut WebKitScriptWorld,
                    *mut WebKitWebPage,
                    *mut WebKitFrame,
                    gpointer,
                ) -> (),
        )),
        0 as *mut std::ffi::c_void,
        ::core::mem::transmute::<libc::intptr_t, GClosureNotify>(NULL as libc::intptr_t),
        G_CONNECT_DEFAULT,
    );
    let mut L = common.L;
    lua_pushlstring(
        L,
        b"luakit.luajs.registry\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 22]>() as std::ffi::c_ulong)
            .wrapping_div(::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong)
            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
    );
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_rawset(L, LUA_REGISTRYINDEX);
    lua_getfield(
        L,
        LUA_GLOBALSINDEX,
        b"string\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_getfield(
        L,
        -(1 as std::ffi::c_int),
        b"find\0" as *const u8 as *const std::ffi::c_char,
    );
    luaH_registerfct(L, -(1 as std::ffi::c_int), &mut lua_string_find_ref);
    lua_settop(L, -(2 as std::ffi::c_int) - 1 as std::ffi::c_int);
}
