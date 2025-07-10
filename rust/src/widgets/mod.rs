pub mod r#box;
pub mod common;
pub mod drawing_area;
pub mod entry;
pub mod eventbox;
pub mod image;
pub mod label;
pub mod notebook;
pub mod overlay;
pub mod paned;
pub mod scrolled;
pub mod spinner;
pub mod stack;
pub mod webview;
pub mod window;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct widget_t {
    pub signals: *mut signal_t,
    pub info: *const widget_info_t,
    pub destructor: Option<widget_destructor_t>,
    pub index: Option<unsafe extern "C" fn(*mut lua_State, *mut widget_t, luakit_token_t) -> gint>,
    pub newindex:
        Option<unsafe extern "C" fn(*mut lua_State, *mut widget_t, luakit_token_t) -> gint>,
    pub ref_0: gpointer,
    pub widget: *mut GtkWidget,
    pub provider: *mut GtkCssProvider,
    pub prev_width: gint,
    pub prev_height: gint,
    pub data: gpointer,
}

pub type widget_destructor_t = unsafe extern "C" fn(*mut widget_t) -> ();
#[derive(Copy, Clone)]
#[repr(C)]

pub struct widget_info_t {
    pub tok: luakit_token_t,
    pub name: *const gchar,
    pub wc: Option<widget_constructor_t>,
}

pub type widget_constructor_t =
    unsafe extern "C" fn(*mut lua_State, *mut widget_t, luakit_token_t) -> *mut widget_t;
#[inline]

pub unsafe extern "C" fn luaH_checkwidget(mut L: *mut lua_State, mut udx: gint) -> *mut widget_t {
    let mut w = luaH_checkudata(L, udx, &mut widget_class) as *mut widget_t;
    if ((*w).widget).is_null() {
        luaL_error(
            L,
            b"widget %p (%s) has been destroyed\0" as *const u8 as *const std::ffi::c_char,
            w,
            (*(*w).info).name,
        );
    }
    if ({
        let mut __inst = (*w).widget as *mut GTypeInstance;
        let mut __t = gtk_widget_get_type();
        let mut __r: gboolean = 0;
        if __inst.is_null() {
            __r = 0 as std::ffi::c_int;
        } else if !((*__inst).g_class).is_null() && (*(*__inst).g_class).g_type == __t {
            __r = (0 as std::ffi::c_int == 0) as std::ffi::c_int;
        } else {
            __r = g_type_check_instance_is_a(__inst, __t);
        }
        __r
    }) != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN as *const std::ffi::c_char,
            b"./clib/widget.h\0" as *const u8 as *const std::ffi::c_char,
            101 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 17], &[std::ffi::c_char; 17]>(b"luaH_checkwidget\0"))
                .as_ptr(),
            b"GTK_IS_WIDGET(w->widget)\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    return w;
}
use gdk_sys::*;
use glib_sys::*;
use gtk_sys::*;
use libc::*;
use mlua_sys::*;
use webkit2gtk::glib::gobject_ffi::*;

use crate::common::luaclass::luaH_checkudata;
use crate::common::tokenize::*;
use crate::common::{
    luaclass::{lua_class_t, signal_h::signal_t},
    messages::G_LOG_DOMAIN,
};
use crate::gtypes::*;

unsafe extern "C" {

    pub static mut widget_class: lua_class_t;
}
