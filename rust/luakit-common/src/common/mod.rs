pub mod clib;
pub mod luaclass;
pub mod luah;
pub mod luajs;
pub mod lualib;
pub mod luaobject;
pub mod luaserialize;
pub mod luauniq;
pub mod luautil;
pub mod luayield;
pub mod property;
pub mod resource;
pub mod tokenize;
pub mod util;

use mlua::ffi::lua_State;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct common_t {
    pub L: *mut lua_State,
}

pub static mut common: common_t = common_t {
    L: std::ptr::null_mut(),
};

pub mod messages {
    pub const G_LOG_DOMAIN: std::ffi::c_int = 0;
}
