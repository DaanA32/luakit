use gdk_sys::*;
use glib_sys::*;
use libc::*;
use mlua_sys::*;

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

static mut wrap_function_ref: *mut std::ffi::c_void =
    0 as *const std::ffi::c_void as *mut std::ffi::c_void;
static mut yield_ref: *mut std::ffi::c_void = 0 as *const std::ffi::c_void as *mut std::ffi::c_void;
static mut unlock_ref: *mut std::ffi::c_void =
    0 as *const std::ffi::c_void as *mut std::ffi::c_void;
static mut sched_src: *const std::ffi::c_char = b" local y = {}                                                                               \n                                                                                            \n local wrap_function = function (fn)                                                        \n     return function (...)                                                                  \n         assert(coroutine.running(), 'cannot call asynchronous function from main thread!') \n         y.yieldable = true                                                                 \n         local ret = {fn(...)}                                                              \n         y.yieldable = false                                                                \n         if y.yield then                                                                    \n             y.yield = false                                                                \n             y[coroutine.running()] = true                                                  \n             repeat                                                                         \n                 ret = {coroutine.yield()}                                                  \n             until not y[coroutine.running()]                                               \n         end                                                                                \n         return unpack(ret)                                                                 \n     end                                                                                    \n end                                                                                        \n                                                                                            \n local yield = function ()                                                                  \n     assert(y.yieldable, 'attempted to yield from unwrapped operation!')                    \n     y.yield = true                                                                         \n end                                                                                        \n                                                                                            \n local unlock = function ()                                                                 \n     y[coroutine.running()] = nil                                                           \n end                                                                                        \n                                                                                            \n return {                                                                                   \n     wrap_function = wrap_function,                                                         \n     yield = yield,                                                                         \n     unlock = unlock,                                                                       \n }                                                                                          \n\0"
    as *const u8 as *const std::ffi::c_char;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_yield_setup(mut L: *mut lua_State) {
    let mut top: gint = lua_gettop(L);
    luaL_loadbuffer(
        L,
        sched_src,
        strlen(sched_src),
        b"luakit_yield_handler\0" as *const u8 as *const std::ffi::c_char,
    );
    luaH_dofunction(L, 0 as std::ffi::c_int, 1 as std::ffi::c_int);
    lua_getfield(
        L,
        -(1 as std::ffi::c_int),
        b"yield\0" as *const u8 as *const std::ffi::c_char,
    );
    yield_ref = luaH_object_ref(L, -(1 as std::ffi::c_int));
    lua_getfield(
        L,
        -(1 as std::ffi::c_int),
        b"wrap_function\0" as *const u8 as *const std::ffi::c_char,
    );
    wrap_function_ref = luaH_object_ref(L, -(1 as std::ffi::c_int));
    lua_getfield(
        L,
        -(1 as std::ffi::c_int),
        b"unlock\0" as *const u8 as *const std::ffi::c_char,
    );
    unlock_ref = luaH_object_ref(L, -(1 as std::ffi::c_int));
    lua_settop(L, top);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_yield_wrap_function(mut L: *mut lua_State) {
    if !(lua_type(L, -(1 as std::ffi::c_int)) == 6 as std::ffi::c_int) {
        luaL_typerror(
            L,
            -(1 as std::ffi::c_int),
            b"function\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    luaH_object_push(L, wrap_function_ref);
    luaH_dofunction(L, 1 as std::ffi::c_int, 1 as std::ffi::c_int);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_yield(mut L: *mut lua_State) -> std::ffi::c_int {
    luaH_object_push(L, yield_ref);
    luaH_dofunction(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    return 0 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_resume(mut L: *mut lua_State, mut nret: gint) -> gboolean {
    luaH_object_push(L, unlock_ref);
    luaH_dofunction(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    let mut top: gint = lua_gettop(L) - nret;
    let mut ret: gint = lua_resume(L, std::ptr::null_mut(), nret, std::ptr::null_mut());
    if ret == 0 as std::ffi::c_int || ret == 1 as std::ffi::c_int {
        return (0 as std::ffi::c_int == 0) as std::ffi::c_int;
    }
    lua_pushcclosure(L, luaH_dofunction_on_error, 0 as std::ffi::c_int);
    lua_insert(L, -(2 as std::ffi::c_int));
    lua_call(L, 1 as std::ffi::c_int, 1 as std::ffi::c_int);
    _log(
        LOG_LEVEL_error,
        b"common/luayield.c\0" as *const u8 as *const std::ffi::c_char,
        b"%s\0" as *const u8 as *const std::ffi::c_char,
        lua_tolstring(L, -(1 as std::ffi::c_int), 0 as *mut size_t),
    );
    lua_settop(L, top);
    return 0 as std::ffi::c_int;
}
