use std::ffi::CStr;

use crate::{
    globalconf::globalconf,
    gtypes::*,
    log::{_log, LOG_LEVEL_verbose},
    widgets::webview::download::download_start_cb,
};
use glib_sys::*;
use gobject_sys::*;
use libc::c_char;
use webkit2gtk_sys::*;

static mut web_context: *mut WebKitWebContext = std::ptr::null_mut();
static mut process_limit: guint = 0 as std::ffi::c_int as guint;
static mut web_context_started: gboolean = 0 as std::ffi::c_int;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn web_context_get() -> *mut WebKitWebContext {
    if !web_context.is_null() {
    } else {
        g_assertion_message_expr(
            0 as *mut gchar,
            b"web_context.c\0" as *const u8 as *const std::ffi::c_char,
            40 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 16], &[std::ffi::c_char; 16]>(b"web_context_get\0"))
                .as_ptr(),
            b"web_context\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    return web_context;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn web_context_process_limit_get() -> guint {
    return process_limit;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn web_context_process_limit_set(mut limit: guint) -> gboolean {
    if web_context_started != 0 {
        return 0 as std::ffi::c_int;
    }
    process_limit = limit;
    return (0 as std::ffi::c_int == 0) as std::ffi::c_int;
}
unsafe extern "C" fn website_data_manager_init() {
    let mut data_mgr: *mut WebKitWebsiteDataManager = webkit_website_data_manager_new(
        b"base-cache-directory\0" as *const u8 as *const std::ffi::c_char,
        globalconf.cache_dir,
        b"base-data-directory\0" as *const u8 as *const std::ffi::c_char,
        globalconf.data_dir,
        0 as *mut std::ffi::c_void,
    );
    web_context = webkit_web_context_new_with_website_data_manager(data_mgr);
    _log(
        LOG_LEVEL_verbose,
        b"web_context.c\0" as *const u8 as *const std::ffi::c_char,
        &format!(
            "base_data_directory:                 {}",
            CStr::from_ptr(webkit_website_data_manager_get_base_data_directory(
                data_mgr
            ))
            .to_string_lossy(),
        ),
    );
    _log(
        LOG_LEVEL_verbose,
        b"web_context.c\0" as *const u8 as *const std::ffi::c_char,
        &format!(
            "base_cache_directory:                {}",
            CStr::from_ptr(webkit_website_data_manager_get_base_cache_directory(
                data_mgr
            ))
            .to_string_lossy(),
        ),
    );
}
unsafe extern "C" fn web_context_set_default_spelling_language() {
    let null: *const *const c_char = { std::ptr::null() };
    eprintln!(
        "web_context_set_default_spelling_language: {:?}",
        web_context
    );
    webkit_web_context_set_spell_checking_languages(web_context, null);
    let mut ret: *mut *mut gchar =
        webkit_web_context_get_spell_checking_languages(web_context) as *mut *mut gchar;
    eprintln!("webkit_web_context_get_spell_checking_languages: {:?}", ret);
    if ret.is_null() {
        return;
    }
    let mut langs: *mut gchar = g_strjoinv(b", \0" as *const u8 as *const std::ffi::c_char, ret);
    _log(
        LOG_LEVEL_verbose,
        b"web_context.c\0" as *const u8 as *const std::ffi::c_char,
        &format!(
            "setting spell check languages: {}",
            CStr::from_ptr(langs).to_string_lossy(),
        ),
    );
    g_free(langs as gpointer);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn web_context_init() {
    website_data_manager_init();
    webkit_web_context_set_favicon_database_directory(web_context, 0 as *const gchar);
    g_signal_connect_data(
        g_type_check_instance_cast(
            web_context as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut GObject,
        b"download-started\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut WebKitWebContext,
                    *mut WebKitDownload,
                    gpointer,
                ) -> gboolean,
            >,
            GCallback,
        >(Some(
            download_start_cb
                as unsafe extern "C" fn(
                    *mut WebKitWebContext,
                    *mut WebKitDownload,
                    gpointer,
                ) -> gboolean,
        )),
        0 as *mut std::ffi::c_void,
        None,
        G_CONNECT_DEFAULT,
    );
    let mut cookie_mgr: *mut WebKitCookieManager =
        webkit_web_context_get_cookie_manager(web_context);
    webkit_cookie_manager_set_accept_policy(cookie_mgr, WEBKIT_COOKIE_POLICY_ACCEPT_NO_THIRD_PARTY);
    web_context_set_default_spelling_language();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn web_context_init_finish() {
    if web_context_started != 0 {
        return;
    }
    web_context_started = (0 as std::ffi::c_int == 0) as std::ffi::c_int;
}
