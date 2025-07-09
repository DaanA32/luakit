use crate::gtypes::*;
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

unsafe extern "C" {
    pub static mut globalconf: globalconf_t;
}
