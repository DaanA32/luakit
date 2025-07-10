use glib_sys::{
    g_assertion_message_expr, g_build_filename, g_free, g_strdup, g_strerror, g_strsplit, gpointer,
};
use libc::{__errno_location, access, c_void};

use crate::gtypes::gchar;
use crate::log::{_log, LOG_LEVEL_debug, LOG_LEVEL_verbose};

static mut resource_path: *mut gchar = 0 as *const gchar as *mut gchar;
static mut resource_paths: *mut *mut gchar = 0 as *const *mut gchar as *mut *mut gchar;
pub unsafe extern "C" fn resource_path_set(mut path: *const gchar) {
    _log(
        LOG_LEVEL_verbose,
        b"common/resource.c\0" as *const u8 as *const std::ffi::c_char,
        b"setting resource path '%s'\0" as *const u8 as *const std::ffi::c_char,
        path,
    );
    g_free(resource_path as gpointer);
    resource_path = g_strdup(path);
    resource_paths = 0 as *mut *mut gchar;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn resource_path_get() -> *mut gchar {
    return resource_path;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn resource_find_file(mut path: *const gchar) -> *mut gchar {
    if !path.is_null() {
    } else {
        g_assertion_message_expr(
            0 as *mut gchar,
            b"common/resource.c\0" as *const u8 as *const std::ffi::c_char,
            47 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 19], &[std::ffi::c_char; 19]>(
                b"resource_find_file\0",
            ))
            .as_ptr(),
            b"path\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    _log(
        LOG_LEVEL_verbose,
        b"common/resource.c\0" as *const u8 as *const std::ffi::c_char,
        b"finding resource file '%s'\0" as *const u8 as *const std::ffi::c_char,
        path,
    );
    if *path.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int == '/' as i32 {
        return g_strdup(path);
    }
    if resource_paths.is_null() {
        resource_paths = g_strsplit(
            resource_path,
            b";\0" as *const u8 as *const std::ffi::c_char,
            0 as std::ffi::c_int,
        );
    }
    let mut p: *mut *mut std::ffi::c_char = resource_paths;
    while !(*p).is_null() {
        let mut full_path: *mut gchar = g_build_filename(*p, path, 0 as *mut std::ffi::c_void);
        if access(full_path, 4 as std::ffi::c_int) != 0 {
            _log(
                LOG_LEVEL_debug,
                b"common/resource.c\0" as *const u8 as *const std::ffi::c_char,
                b"tried path '%s': %s\0" as *const u8 as *const std::ffi::c_char,
                full_path,
                g_strerror(*__errno_location()),
            );
        } else {
            _log(
                LOG_LEVEL_verbose,
                b"common/resource.c\0" as *const u8 as *const std::ffi::c_char,
                b"found resource file at '%s'\0" as *const u8 as *const std::ffi::c_char,
                full_path,
            );
            return full_path;
        }
        g_free(full_path as gpointer);
        p = p.offset(1);
        p;
    }
    _log(
        LOG_LEVEL_verbose,
        b"common/resource.c\0" as *const u8 as *const std::ffi::c_char,
        b"no resource file found for '%s'\0" as *const u8 as *const std::ffi::c_char,
        path,
    );
    return 0 as *mut gchar;
}
