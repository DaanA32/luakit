pub mod clib;
pub mod luaclass;
pub mod luah;
pub mod luaobject;
pub mod luautil;
pub mod property;
pub mod tokenize;
pub mod util;

use lua::ffi::lua_State;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct common_t {
    pub L: *mut lua_State,
}
unsafe extern "C" {
    pub static mut common: common_t;
}

pub mod messages {
    pub const G_LOG_DOMAIN: std::ffi::c_int = 0;
}
