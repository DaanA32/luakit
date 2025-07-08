use ::c2rust_bitfields;
use ::libc;
use glib_sys::{g_assertion_message_expr, g_free, g_strdup, gpointer};
use libc::{FILE, chmod, fclose, fopen, mode_t, strcmp};
use lua::ffi::{
    LUA_MULTRET, LUA_TNIL, lua_State, lua_gettop, lua_pushstring, lua_type, luaL_Reg,
    luaL_checklstring, luaL_error,
};
use webkit2gtk::ffi::{
    WEBKIT_COOKIE_PERSISTENT_STORAGE_SQLITE, WEBKIT_COOKIE_POLICY_ACCEPT_ALWAYS,
    WEBKIT_COOKIE_POLICY_ACCEPT_NEVER, WEBKIT_COOKIE_POLICY_ACCEPT_NO_THIRD_PARTY,
    WEBKIT_NETWORK_PROXY_MODE_CUSTOM, WEBKIT_NETWORK_PROXY_MODE_DEFAULT,
    WEBKIT_NETWORK_PROXY_MODE_NO_PROXY, webkit_cookie_manager_set_accept_policy,
    webkit_cookie_manager_set_persistent_storage, webkit_network_proxy_settings_free,
    webkit_network_proxy_settings_new, webkit_web_context_get_cookie_manager,
    webkit_web_context_get_website_data_manager, webkit_web_context_set_network_proxy_settings,
};
pub mod soup_uri_utils_h {
    use glib_sys::{
        G_URI_FLAGS_ENCODED_FRAGMENT, G_URI_FLAGS_ENCODED_PATH, G_URI_FLAGS_ENCODED_QUERY,
        G_URI_FLAGS_HAS_PASSWORD, G_URI_FLAGS_SCHEME_NORMALIZE,
    };

    pub const SOUP_HTTP_URI_FLAGS: std::ffi::c_int = G_URI_FLAGS_HAS_PASSWORD as std::ffi::c_int
        | G_URI_FLAGS_ENCODED_PATH as std::ffi::c_int
        | G_URI_FLAGS_ENCODED_QUERY as std::ffi::c_int
        | G_URI_FLAGS_ENCODED_FRAGMENT as std::ffi::c_int
        | G_URI_FLAGS_SCHEME_NORMALIZE as std::ffi::c_int;
}
pub mod soup_h {
    pub static mut scheme_reg: *mut GRegex = 0 as *const GRegex as *mut GRegex;
    pub unsafe extern "C" fn luaH_soup_uri_tostring(mut L: *mut lua_State) -> gint {
        let mut p = 0 as *const gchar;
        let mut port: gint = 0;
        if !(lua_type(L, 1 as std::ffi::c_int) == LUA_TTABLE) {
            luaL_argerror(
                L,
                1 as std::ffi::c_int,
                b"table\0" as *const u8 as *const std::ffi::c_char,
            );
        }
        let mut scheme = b"http\0" as *const u8 as *const std::ffi::c_char;
        let mut user = std::ptr::null();
        let mut host = std::ptr::null();
        let mut path = std::ptr::null();
        let mut query = std::ptr::null();
        let mut fragment = std::ptr::null();
        let mut uri = 0 as *mut gchar;
        lua_pushlstring(
            L,
            b"scheme\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 7]>())
                .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
                .wrapping_sub(1),
        );
        lua_rawget(L, 1 as std::ffi::c_int);
        if !(lua_type(L, -(1 as std::ffi::c_int)) == LUA_TNIL)
            && {
                p = lua_tolstring(L, -(1 as std::ffi::c_int), std::ptr::null_mut());
                !p.is_null()
            }
            && *p.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int != 0
        {
            scheme = p;
        }
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        if g_strcmp0(scheme, b"file\0" as *const u8 as *const std::ffi::c_char) == 0 {
            host = b"\0" as *const u8 as *const std::ffi::c_char;
        }
        lua_pushlstring(
            L,
            b"user\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 5]>())
                .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
                .wrapping_sub(1),
        );
        lua_rawget(L, 1 as std::ffi::c_int);
        if !(lua_type(L, -(1 as std::ffi::c_int)) == LUA_TNIL)
            && {
                p = lua_tolstring(L, -(1 as std::ffi::c_int), std::ptr::null_mut());
                !p.is_null()
            }
            && *p.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int != 0
        {
            user = p;
        }
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        lua_pushlstring(
            L,
            b"host\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 5]>())
                .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
                .wrapping_sub(1),
        );
        lua_rawget(L, 1 as std::ffi::c_int);
        if !(lua_type(L, -(1 as std::ffi::c_int)) == LUA_TNIL)
            && {
                p = lua_tolstring(L, -(1 as std::ffi::c_int), std::ptr::null());
                !p.is_null()
            }
            && *p.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int != 0
        {
            host = p;
        }
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        lua_pushlstring(
            L,
            b"path\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 5]>())
                .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
                .wrapping_sub(1),
        );
        lua_rawget(L, 1 as std::ffi::c_int);
        if !(lua_type(L, -(1 as std::ffi::c_int)) == LUA_TNIL)
            && {
                p = lua_tolstring(L, -(1 as std::ffi::c_int), std::ptr::null());
                !p.is_null()
            }
            && *p.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int != 0
        {
            path = p;
        }
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        lua_pushlstring(
            L,
            b"query\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 6]>())
                .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
                .wrapping_sub(1),
        );
        lua_rawget(L, 1 as std::ffi::c_int);
        if !(lua_type(L, -(1 as std::ffi::c_int)) == LUA_TNIL)
            && {
                p = lua_tolstring(L, -(1 as std::ffi::c_int), std::ptr::null());
                !p.is_null()
            }
            && *p.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int != 0
        {
            query = p;
        }
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        lua_pushlstring(
            L,
            b"fragment\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 9]>())
                .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
                .wrapping_sub(1),
        );
        lua_rawget(L, 1 as std::ffi::c_int);
        if !(lua_type(L, -(1 as std::ffi::c_int)) == LUA_TNIL)
            && {
                p = lua_tolstring(L, -(1 as std::ffi::c_int), std::ptr::null());
                !p.is_null()
            }
            && *p.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int != 0
        {
            fragment = p;
        }
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        lua_pushlstring(
            L,
            b"port\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 5]>())
                .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
                .wrapping_sub(1),
        );
        lua_rawget(L, 1 as std::ffi::c_int);
        if lua_type(L, -(1 as std::ffi::c_int)) == LUA_TNIL || {
            port = lua_tonumber(L, -(1 as std::ffi::c_int)) as gint;
            port == 0
        } {
            port = -(1 as std::ffi::c_int);
        }
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        uri = g_uri_join_with_user(
            SOUP_HTTP_URI_FLAGS as GUriFlags,
            scheme,
            user,
            std::ptr::null(),
            std::ptr::null(),
            host,
            port,
            path,
            query,
            fragment,
        );
        lua_pushstring(L, uri);
        g_free(uri as gpointer);
        return 1 as std::ffi::c_int;
    }
    pub unsafe extern "C" fn luaH_soup_push_uri(mut L: *mut lua_State, mut uri: *mut GUri) -> gint {
        let mut p = 0 as *const gchar;
        let mut port: gint = 0;
        lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
        p = g_uri_get_scheme(uri);
        if !p.is_null() && *p.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int != 0 {
            lua_pushlstring(
                L,
                b"scheme\0" as *const u8 as *const std::ffi::c_char,
                (::core::mem::size_of::<[std::ffi::c_char; 7]>())
                    .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
                    .wrapping_sub(1),
            );
            lua_pushstring(L, p);
            lua_rawset(L, -(3 as std::ffi::c_int));
        }
        p = g_uri_get_user(uri);
        if !p.is_null() && *p.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int != 0 {
            lua_pushlstring(
                L,
                b"user\0" as *const u8 as *const std::ffi::c_char,
                (::core::mem::size_of::<[std::ffi::c_char; 5]>())
                    .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
                    .wrapping_sub(1),
            );
            lua_pushstring(L, p);
            lua_rawset(L, -(3 as std::ffi::c_int));
        }
        p = g_uri_get_password(uri);
        if !p.is_null() && *p.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int != 0 {
            lua_pushlstring(
                L,
                b"password\0" as *const u8 as *const std::ffi::c_char,
                (::core::mem::size_of::<[std::ffi::c_char; 9]>())
                    .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
                    .wrapping_sub(1),
            );
            lua_pushstring(L, p);
            lua_rawset(L, -(3 as std::ffi::c_int));
        }
        p = g_uri_get_host(uri);
        if !p.is_null() && *p.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int != 0 {
            lua_pushlstring(
                L,
                b"host\0" as *const u8 as *const std::ffi::c_char,
                (::core::mem::size_of::<[std::ffi::c_char; 5]>())
                    .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
                    .wrapping_sub(1),
            );
            lua_pushstring(L, p);
            lua_rawset(L, -(3 as std::ffi::c_int));
        }
        p = g_uri_get_path(uri);
        if !p.is_null() && *p.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int != 0 {
            lua_pushlstring(
                L,
                b"path\0" as *const u8 as *const std::ffi::c_char,
                (::core::mem::size_of::<[std::ffi::c_char; 5]>())
                    .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
                    .wrapping_sub(1),
            );
            lua_pushstring(L, p);
            lua_rawset(L, -(3 as std::ffi::c_int));
        }
        p = g_uri_get_query(uri);
        if !p.is_null() && *p.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int != 0 {
            lua_pushlstring(
                L,
                b"query\0" as *const u8 as *const std::ffi::c_char,
                (::core::mem::size_of::<[std::ffi::c_char; 6]>())
                    .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
                    .wrapping_sub(1),
            );
            lua_pushstring(L, p);
            lua_rawset(L, -(3 as std::ffi::c_int));
        }
        p = g_uri_get_fragment(uri);
        if !p.is_null() && *p.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int != 0 {
            lua_pushlstring(
                L,
                b"fragment\0" as *const u8 as *const std::ffi::c_char,
                (::core::mem::size_of::<[std::ffi::c_char; 9]>())
                    .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
                    .wrapping_sub(1),
            );
            lua_pushstring(L, p);
            lua_rawset(L, -(3 as std::ffi::c_int));
        }
        port = g_uri_get_port(uri);
        if port > 0 as std::ffi::c_int {
            lua_pushlstring(
                L,
                b"port\0" as *const u8 as *const std::ffi::c_char,
                (::core::mem::size_of::<[std::ffi::c_char; 5]>())
                    .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
                    .wrapping_sub(1),
            );
            lua_pushnumber(L, port as Number);
            lua_rawset(L, -(3 as std::ffi::c_int));
        }
        return 1 as std::ffi::c_int;
    }
    pub unsafe extern "C" fn luaH_soup_parse_uri(mut L: *mut lua_State) -> gint {
        let mut str =
            luaL_checklstring(L, 1 as std::ffi::c_int, std::ptr::null_mut()) as *mut gchar;
        if *str.offset(0 as std::ffi::c_int as isize) == 0 {
            return 0 as std::ffi::c_int;
        }
        if g_regex_match(
            scheme_reg,
            str,
            G_REGEX_MATCH_DEFAULT,
            0 as *mut *mut GMatchInfo,
        ) == 0
        {
            str = g_strdup_printf(b"http://%s\0" as *const u8 as *const std::ffi::c_char, str);
        } else {
            str = g_strdup(str);
        }
        let mut uri = g_uri_parse(str, SOUP_HTTP_URI_FLAGS as GUriFlags, std::ptr::null_mut());
        g_free(str as gpointer);
        if !uri.is_null() {
            luaH_soup_push_uri(L, uri);
            g_uri_unref(uri);
            return 1 as std::ffi::c_int;
        }
        return 0 as std::ffi::c_int;
    }
    pub unsafe extern "C" fn soup_lib_setup_common() {
        scheme_reg = g_regex_new(
            b"^[a-z][a-z0-9\\+\\-\\.]*:\0" as *const u8 as *const std::ffi::c_char,
            G_REGEX_DEFAULT,
            G_REGEX_MATCH_DEFAULT,
            std::ptr::null_mut(),
        );
    }
    use glib_sys::{
        G_REGEX_DEFAULT, G_REGEX_MATCH_DEFAULT, GError, GMatchInfo, GRegex, GUri, GUriFlags,
        g_free, g_regex_match, g_regex_new, g_strcmp0, g_strdup, g_strdup_printf,
        g_uri_get_fragment, g_uri_get_host, g_uri_get_password, g_uri_get_path, g_uri_get_port,
        g_uri_get_query, g_uri_get_scheme, g_uri_get_user, g_uri_join_with_user, g_uri_parse,
        g_uri_unref, gpointer,
    };
    use lua::Number;
    use lua::ffi::{
        LUA_TNIL, LUA_TTABLE, lua_State, lua_createtable, lua_pushlstring, lua_pushnumber,
        lua_pushstring, lua_rawget, lua_rawset, lua_settop, lua_tolstring, lua_tonumber, lua_type,
        luaL_argerror, luaL_checklstring,
    };

    use crate::clib::soup::SOUP_HTTP_URI_FLAGS;
    use crate::gtypes::{gchar, gint};
}
use crate::{
    clib::soup::soup_h::{luaH_soup_parse_uri, luaH_soup_uri_tostring, soup_lib_setup_common},
    common::{
        luaclass::{
            lua_class_property_array_t, lua_class_t, luaH_class_add_signal, luaH_class_emit_signal,
            luaH_class_remove_signal, luaH_openlib,
            signal_h::{signal_new, signal_t},
        },
        messages::G_LOG_DOMAIN,
        tokenize::l_tokenize,
    },
    gtypes::{gchar, gint},
    web_context::web_context_get,
};

pub use self::soup_uri_utils_h::SOUP_HTTP_URI_FLAGS;

static mut soup_class: lua_class_t = lua_class_t {
    name: 0 as *const gchar,
    signals: 0 as *const signal_t as *mut signal_t,
    allocator: None,
    properties: 0 as *const lua_class_property_array_t as *mut lua_class_property_array_t,
    index_miss_property: None,
    newindex_miss_property: None,
};
static mut proxy_uri: *mut gchar = 0 as *const gchar as *mut gchar;
static mut accept_policy: *mut gchar = 0 as *const gchar as *mut gchar;
static mut cookies_storage: *mut gchar = 0 as *const gchar as *mut gchar;
#[inline]
unsafe extern "C" fn luaH_soup_class_emit_signal(mut L: *mut lua_State) -> gint {
    return luaH_class_emit_signal(
        L,
        &mut soup_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, std::ptr::null_mut()),
        lua_gettop(L) - 1 as std::ffi::c_int,
        LUA_MULTRET,
    );
}
#[inline]
unsafe extern "C" fn luaH_soup_class_remove_signal(mut L: *mut lua_State) -> gint {
    luaH_class_remove_signal(
        L,
        &mut soup_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, std::ptr::null_mut()),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[inline]
unsafe extern "C" fn luaH_soup_class_add_signal(mut L: *mut lua_State) -> gint {
    luaH_class_add_signal(
        L,
        &mut soup_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, std::ptr::null()),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn luaH_soup_index(mut L: *mut lua_State) -> gint {
    let mut prop = luaL_checklstring(L, 2 as std::ffi::c_int, std::ptr::null_mut());
    let mut token = l_tokenize(prop);
    match token as std::ffi::c_uint {
        174 => {
            lua_pushstring(L, proxy_uri);
            return 1 as std::ffi::c_int;
        }
        1 => {
            lua_pushstring(L, accept_policy);
            return 1 as std::ffi::c_int;
        }
        34 => {
            lua_pushstring(L, cookies_storage);
            return 1 as std::ffi::c_int;
        }
        _ => {}
    }
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn luaH_soup_set_proxy_uri(mut L: *mut lua_State) {
    let mut ctx = web_context_get();
    let mut dm = webkit_web_context_get_website_data_manager(ctx);
    let mut new_proxy_uri = if lua_type(L, 3 as std::ffi::c_int) == LUA_TNIL {
        b"default\0" as *const u8 as *const std::ffi::c_char
    } else {
        luaL_checklstring(L, 3 as std::ffi::c_int, std::ptr::null_mut())
    };
    g_free(proxy_uri as gpointer);
    proxy_uri = g_strdup(new_proxy_uri);
    if proxy_uri.is_null()
        || strcmp(
            proxy_uri as *const std::ffi::c_char,
            b"default\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
    {
        webkit_web_context_set_network_proxy_settings(
            ctx,
            WEBKIT_NETWORK_PROXY_MODE_DEFAULT,
            std::ptr::null_mut(),
        );
    } else if strcmp(
        proxy_uri as *const std::ffi::c_char,
        b"no_proxy\0" as *const u8 as *const std::ffi::c_char,
    ) == 0 as std::ffi::c_int
    {
        webkit_web_context_set_network_proxy_settings(
            ctx,
            WEBKIT_NETWORK_PROXY_MODE_NO_PROXY,
            std::ptr::null_mut(),
        );
    } else {
        let mut proxy_settings = webkit_network_proxy_settings_new(proxy_uri, std::ptr::null());
        webkit_web_context_set_network_proxy_settings(
            ctx,
            WEBKIT_NETWORK_PROXY_MODE_CUSTOM,
            proxy_settings,
        );
        webkit_network_proxy_settings_free(proxy_settings);
    };
}
unsafe extern "C" fn luaH_soup_set_accept_policy(mut L: *mut lua_State) {
    let mut new_policy = luaL_checklstring(L, 3 as std::ffi::c_int, std::ptr::null_mut());
    if !(strcmp(
        new_policy as *const std::ffi::c_char,
        b"always\0" as *const u8 as *const std::ffi::c_char,
    ) == 0 as std::ffi::c_int)
    {
        if !(strcmp(
            new_policy as *const std::ffi::c_char,
            b"never\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int)
        {
            if !(strcmp(
                new_policy as *const std::ffi::c_char,
                b"no_third_party\0" as *const u8 as *const std::ffi::c_char,
            ) == 0 as std::ffi::c_int)
            {
                luaL_error(
                    L,
                    b"accept_policy must be one of 'always', 'never', 'no_third_party'\0"
                        as *const u8 as *const std::ffi::c_char,
                );
            }
        }
    }
    g_free(accept_policy as gpointer);
    accept_policy = g_strdup(new_policy);
    let mut web_context = web_context_get();
    let mut cookie_mgr = webkit_web_context_get_cookie_manager(web_context);
    let mut policy = WEBKIT_COOKIE_POLICY_ACCEPT_ALWAYS;
    if strcmp(
        new_policy as *const std::ffi::c_char,
        b"always\0" as *const u8 as *const std::ffi::c_char,
    ) == 0 as std::ffi::c_int
    {
        policy = WEBKIT_COOKIE_POLICY_ACCEPT_ALWAYS;
    } else if strcmp(
        new_policy as *const std::ffi::c_char,
        b"never\0" as *const u8 as *const std::ffi::c_char,
    ) == 0 as std::ffi::c_int
    {
        policy = WEBKIT_COOKIE_POLICY_ACCEPT_NEVER;
    } else if strcmp(
        new_policy as *const std::ffi::c_char,
        b"no_third_party\0" as *const u8 as *const std::ffi::c_char,
    ) == 0 as std::ffi::c_int
    {
        policy = WEBKIT_COOKIE_POLICY_ACCEPT_NO_THIRD_PARTY;
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN as *const std::ffi::c_char,
            b"clib/soup.c\0" as *const u8 as *const std::ffi::c_char,
            97 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 28], &[std::ffi::c_char; 28]>(
                b"luaH_soup_set_accept_policy\0",
            ))
            .as_ptr(),
            std::ptr::null(),
        );
    }
    webkit_cookie_manager_set_accept_policy(cookie_mgr, policy);
}
unsafe extern "C" fn luaH_soup_set_cookies_storage(mut L: *mut lua_State) {
    let mut new_path = luaL_checklstring(L, 3 as std::ffi::c_int, std::ptr::null_mut());
    let mut f = 0 as *mut FILE;
    if strcmp(
        new_path as *const std::ffi::c_char,
        b"\0" as *const u8 as *const std::ffi::c_char,
    ) == 0 as std::ffi::c_int
    {
        luaL_error(
            L,
            b"cookies_storage cannot be empty\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    g_free(cookies_storage as gpointer);
    cookies_storage = g_strdup(new_path);
    f = fopen(
        cookies_storage,
        b"a\0" as *const u8 as *const std::ffi::c_char,
    );
    if !f.is_null() {
        chmod(cookies_storage, 0o600 as std::ffi::c_int as mode_t);
        fclose(f);
    }
    let mut web_context = web_context_get();
    let mut cookie_mgr = webkit_web_context_get_cookie_manager(web_context);
    webkit_cookie_manager_set_persistent_storage(
        cookie_mgr,
        cookies_storage,
        WEBKIT_COOKIE_PERSISTENT_STORAGE_SQLITE,
    );
}
unsafe extern "C" fn luaH_soup_newindex(mut L: *mut lua_State) -> gint {
    let mut prop = luaL_checklstring(L, 2 as std::ffi::c_int, std::ptr::null_mut());
    let mut token = l_tokenize(prop);
    match token as std::ffi::c_uint {
        174 => {
            luaH_soup_set_proxy_uri(L);
        }
        1 => {
            luaH_soup_set_accept_policy(L);
        }
        34 => {
            luaH_soup_set_cookies_storage(L);
        }
        _ => return 0 as std::ffi::c_int,
    }
    return 0 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn soup_lib_setup(mut L: *mut lua_State) {
    soup_lib_setup_common();
    static mut soup_lib: [luaL_Reg; 8] = unsafe {
        [
            {
                let mut init = luaL_Reg {
                    name: b"add_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_soup_class_add_signal as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"remove_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_soup_class_remove_signal
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"emit_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_soup_class_emit_signal as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__index\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(luaH_soup_index as unsafe extern "C" fn(*mut lua_State) -> gint),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__newindex\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(luaH_soup_newindex),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"parse_uri\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(luaH_soup_parse_uri),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"uri_tostring\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(luaH_soup_uri_tostring),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: std::ptr::null(),
                    func: None,
                };
                init
            },
        ]
    };
    soup_class.signals = signal_new();
    luaH_openlib(
        L,
        b"soup\0" as *const u8 as *const std::ffi::c_char,
        soup_lib.as_ptr(),
        soup_lib.as_ptr(),
    );
    proxy_uri = g_strdup(b"default\0" as *const u8 as *const std::ffi::c_char);
    accept_policy = g_strdup(b"no_third_party\0" as *const u8 as *const std::ffi::c_char);
}
