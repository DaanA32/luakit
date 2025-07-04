use ::libc;
#[c2rust::header_src = "/usr/lib/clang/20/include/__stddef_size_t.h:20"]
pub mod __stddef_size_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type size_t = std::ffi::c_ulong;
}
#[c2rust::header_src = "/usr/lib/glib-2.0/include/glibconfig.h:23"]
pub mod glibconfig_h {
    #[c2rust::src_loc = "83:1"]
    pub type gsize = std::ffi::c_ulong;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtypes.h:23"]
pub mod gtypes_h {
    #[c2rust::src_loc = "52:1"]
    pub type gchar = std::ffi::c_char;
    #[c2rust::src_loc = "55:1"]
    pub type gint = std::ffi::c_int;
    #[c2rust::src_loc = "109:1"]
    pub type gpointer = *mut std::ffi::c_void;
}
#[c2rust::header_src = "/home/daana/git/luakit/common/log.h:24"]
pub mod log_h {
    #[c2rust::src_loc = "36:9"]
    pub type log_level_t = std::ffi::c_uint;
    #[c2rust::src_loc = "36:16"]
    pub const LOG_LEVEL_debug: log_level_t = 5;
    #[c2rust::src_loc = "36:16"]
    pub const LOG_LEVEL_verbose: log_level_t = 4;
    #[c2rust::src_loc = "36:16"]
    pub const LOG_LEVEL_info: log_level_t = 3;
    #[c2rust::src_loc = "36:16"]
    pub const LOG_LEVEL_warn: log_level_t = 2;
    #[c2rust::src_loc = "36:16"]
    pub const LOG_LEVEL_error: log_level_t = 1;
    #[c2rust::src_loc = "36:16"]
    pub const LOG_LEVEL_fatal: log_level_t = 0;
    use super::gtypes_h::gchar;
    extern "C" {
        #[c2rust::src_loc = "54:1"]
        pub fn _log(lvl: log_level_t, _: *const gchar, _: *const gchar, _: ...);
    }
}
#[c2rust::header_src = "/usr/include/errno.h:19"]
pub mod errno_h {
    extern "C" {
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut std::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:20"]
pub mod unistd_h {
    extern "C" {
        #[c2rust::src_loc = "287:1"]
        pub fn access(
            __name: *const std::ffi::c_char,
            __type: std::ffi::c_int,
        ) -> std::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:23"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "43:14"]
        pub fn memcpy(
            _: *mut std::ffi::c_void,
            _: *const std::ffi::c_void,
            _: std::ffi::c_ulong,
        ) -> *mut std::ffi::c_void;
        #[c2rust::src_loc = "407:15"]
        pub fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gfileutils.h:23"]
pub mod gfileutils_h {
    use super::gtypes_h::gchar;
    extern "C" {
        #[c2rust::src_loc = "174:1"]
        pub fn g_build_filename(first_element: *const gchar, _: ...) -> *mut gchar;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gmem.h:23"]
pub mod gmem_h {
    use super::gtypes_h::gpointer;
    use super::glibconfig_h::gsize;
    extern "C" {
        #[c2rust::src_loc = "73:1"]
        pub fn g_free(mem: gpointer);
        #[c2rust::src_loc = "83:1"]
        pub fn g_malloc(n_bytes: gsize) -> gpointer;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gstrfuncs.h:23"]
pub mod gstrfuncs_h {
    #[inline(always)]
    #[c2rust::src_loc = "308:1"]
    pub unsafe extern "C" fn g_strdup_inline(
        mut str: *const std::ffi::c_char,
    ) -> *mut std::ffi::c_char {
        if 0 != 0 && str.is_null() {
            return 0 as *mut std::ffi::c_char;
        }
        if 0 != 0 && !str.is_null() && 0 != 0 {
            let len: size_t = (strlen(str))
                .wrapping_add(1 as std::ffi::c_int as std::ffi::c_ulong);
            let mut dup_str: *mut std::ffi::c_char = g_malloc(len)
                as *mut std::ffi::c_char;
            return memcpy(
                dup_str as *mut std::ffi::c_void,
                str as *const std::ffi::c_void,
                len,
            ) as *mut std::ffi::c_char;
        }
        return g_strdup(str);
    }
    use super::gtypes_h::{gint, gchar};
    use super::string_h::{strlen, memcpy};
    use super::__stddef_size_t_h::size_t;
    use super::gmem_h::g_malloc;
    extern "C" {
        #[c2rust::src_loc = "116:1"]
        pub fn g_strerror(errnum: gint) -> *const gchar;
        #[c2rust::src_loc = "283:1"]
        pub fn g_strdup(str: *const gchar) -> *mut gchar;
        #[c2rust::src_loc = "355:1"]
        pub fn g_strsplit(
            string: *const gchar,
            delimiter: *const gchar,
            max_tokens: gint,
        ) -> *mut *mut gchar;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtestutils.h:23"]
pub mod gtestutils_h {
    extern "C" {
        #[c2rust::src_loc = "624:1"]
        pub fn g_assertion_message_expr(
            domain: *const std::ffi::c_char,
            file: *const std::ffi::c_char,
            line: std::ffi::c_int,
            func: *const std::ffi::c_char,
            expr: *const std::ffi::c_char,
        ) -> !;
    }
}
pub use self::__stddef_size_t_h::size_t;
pub use self::glibconfig_h::gsize;
pub use self::gtypes_h::{gchar, gint, gpointer};
pub use self::log_h::{
    log_level_t, LOG_LEVEL_debug, LOG_LEVEL_verbose, LOG_LEVEL_info, LOG_LEVEL_warn,
    LOG_LEVEL_error, LOG_LEVEL_fatal, _log,
};
use self::errno_h::__errno_location;
use self::unistd_h::access;
use self::string_h::{memcpy, strlen};
use self::gfileutils_h::g_build_filename;
use self::gmem_h::{g_free, g_malloc};
pub use self::gstrfuncs_h::{g_strdup_inline, g_strerror, g_strdup, g_strsplit};
use self::gtestutils_h::g_assertion_message_expr;
#[c2rust::src_loc = "26:15"]
static mut resource_path: *mut gchar = 0 as *const gchar as *mut gchar;
#[c2rust::src_loc = "27:16"]
static mut resource_paths: *mut *mut gchar = 0 as *const *mut gchar as *mut *mut gchar;
#[no_mangle]
#[c2rust::src_loc = "29:1"]
pub unsafe extern "C" fn resource_path_set(mut path: *const gchar) {
    _log(
        LOG_LEVEL_verbose,
        b"common/resource.c\0" as *const u8 as *const std::ffi::c_char,
        b"setting resource path '%s'\0" as *const u8 as *const std::ffi::c_char,
        path,
    );
    g_free(resource_path as gpointer);
    resource_path = g_strdup_inline(path);
    resource_paths = 0 as *mut *mut gchar;
}
#[no_mangle]
#[c2rust::src_loc = "38:1"]
pub unsafe extern "C" fn resource_path_get() -> *mut gchar {
    return resource_path;
}
#[no_mangle]
#[c2rust::src_loc = "44:1"]
pub unsafe extern "C" fn resource_find_file(mut path: *const gchar) -> *mut gchar {
    if !path.is_null() {} else {
        g_assertion_message_expr(
            0 as *mut gchar,
            b"common/resource.c\0" as *const u8 as *const std::ffi::c_char,
            47 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 19],
                &[std::ffi::c_char; 19],
            >(b"resource_find_file\0"))
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
        return g_strdup_inline(path);
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
        let mut full_path: *mut gchar = g_build_filename(
            *p,
            path,
            0 as *mut std::ffi::c_void,
        );
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
