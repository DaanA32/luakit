use crate::gtypes::{gchar, gdouble};
use glib_sys::{GPtrArray, gboolean};
use gtk_sys::GtkApplication;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct globalconf_t {
    pub application: *mut GtkApplication,
    pub config_dir: *mut gchar,
    pub data_dir: *mut gchar,
    pub cache_dir: *mut gchar,
    pub profile: *mut gchar,
    pub confpath: *mut gchar,
    pub execpath: *mut gchar,
    pub nounique: gboolean,
    pub argv: *mut GPtrArray,
    pub windows: *mut GPtrArray,
    pub webviews: *mut GPtrArray,
    pub stylesheets: *mut GPtrArray,
    pub starttime: gdouble,
}

pub static mut globalconf: globalconf_t = globalconf_t {
    application: std::ptr::null_mut(),
    config_dir: std::ptr::null_mut(),
    data_dir: std::ptr::null_mut(),
    cache_dir: std::ptr::null_mut(),
    profile: std::ptr::null_mut(),
    confpath: std::ptr::null_mut(),
    execpath: std::ptr::null_mut(),
    nounique: 0,
    argv: std::ptr::null_mut(),
    windows: std::ptr::null_mut(),
    webviews: std::ptr::null_mut(),
    stylesheets: std::ptr::null_mut(),
    starttime: 0.0,
};
