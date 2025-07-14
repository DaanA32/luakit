use std::ffi::CStr;

use gdk_sys::*;
use glib_sys::*;
use libc::*;
use mlua_sys::*;

use crate::clib::luakit::*;
use crate::common::clib::luakit::*;
use crate::common::common;
use crate::common::luaclass::signal_h::*;
use crate::common::luaclass::*;
use crate::common::luah::*;
use crate::common::luaobject::*;
use crate::common::luauniq::*;
use crate::common::tokenize::*;
use crate::globalconf::*;
use crate::log::*;
use crate::luah::*;
use crate::web_context::*;

use crate::gtypes::*;
use gobject_sys::*;
use webkit2gtk_sys::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct download_t {
    pub signals: *mut signal_t,
    pub webkit_download: *mut WebKitDownload,
    pub ref_0: gpointer,
    pub uri: *mut gchar,
    pub destination: *mut gchar,
    pub error: *mut gchar,
    pub status: luakit_download_status_t,
}
pub type luakit_download_status_t = std::ffi::c_uint;
pub const LUAKIT_DOWNLOAD_STATUS_FAILED: luakit_download_status_t = 4;
pub const LUAKIT_DOWNLOAD_STATUS_CANCELLED: luakit_download_status_t = 3;
pub const LUAKIT_DOWNLOAD_STATUS_STARTED: luakit_download_status_t = 2;
pub const LUAKIT_DOWNLOAD_STATUS_CREATED: luakit_download_status_t = 1;
pub const LUAKIT_DOWNLOAD_STATUS_FINISHED: luakit_download_status_t = 0;
static mut download_class: lua_class_t = lua_class_t {
    name: 0 as *const gchar,
    signals: 0 as *const signal_t as *mut signal_t,
    allocator: None,
    properties: 0 as *const lua_class_property_array_t as *mut lua_class_property_array_t,
    index_miss_property: None,
    newindex_miss_property: None,
};
#[inline]
unsafe extern "C-unwind" fn luaH_download_class_emit_signal(mut L: *mut lua_State) -> gint {
    return luaH_class_emit_signal(
        L,
        &mut download_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t),
        lua_gettop(L) - 1 as std::ffi::c_int,
        -(1 as std::ffi::c_int),
    );
}
#[inline]
unsafe extern "C-unwind" fn luaH_download_class_remove_signal(mut L: *mut lua_State) -> gint {
    luaH_class_remove_signal(
        L,
        &mut download_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn download_new(mut L: *mut lua_State) -> *mut download_t {
    let mut p: *mut download_t =
        lua_newuserdata(L, ::core::mem::size_of::<download_t>()) as *mut download_t;
    memset(
        p as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        (::core::mem::size_of::<download_t>()).wrapping_mul(1),
    );
    (*p).signals = signal_new();
    luaH_settype(L, &mut download_class);
    lua_createtable(L, 0, 0);
    lua_createtable(L, 0, 0);
    lua_setmetatable(L, -(2 as std::ffi::c_int));
    // TODO: lua_setfenv(L, -(2 as std::ffi::c_int));
    lua_pushvalue(L, -(1 as std::ffi::c_int));
    luaH_class_emit_signal(
        L,
        &mut download_class,
        b"new\0" as *const u8 as *const std::ffi::c_char,
        1 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    return p;
}
#[inline]
unsafe extern "C-unwind" fn luaH_download_class_add_signal(mut L: *mut lua_State) -> gint {
    luaH_class_add_signal(
        L,
        &mut download_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
static mut current_destination_cb: *mut download_t = 0 as *const download_t as *mut download_t;
unsafe extern "C-unwind" fn luaH_download_unref(
    mut L: *mut lua_State,
    mut download: *mut download_t,
) {
    if !((*download).ref_0).is_null() {
        luaH_object_unref(L, (*download).ref_0);
        (*download).ref_0 = 0 as *mut std::ffi::c_void;
    }
    let mut backup: *mut gchar = g_strdup_printf(
        b"%s~\0" as *const u8 as *const std::ffi::c_char,
        (*download).destination,
    );
    g_unlink(backup);
    g_free(backup as gpointer);
}
unsafe extern "C-unwind" fn luaH_download_gc(mut L: *mut lua_State) -> gint {
    let mut download: *mut download_t =
        luaH_checkudata(L, 1 as std::ffi::c_int, &mut download_class) as *mut download_t;
    g_object_unref(g_type_check_instance_cast(
        (*download).webkit_download as *mut GTypeInstance,
        ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
    ) as *mut std::ffi::c_void as *mut GObject);
    g_free((*download).destination as gpointer);
    g_free((*download).uri as gpointer);
    g_free((*download).error as gpointer);
    return luaH_object_gc(L);
}
unsafe extern "C-unwind" fn decide_destination_cb(
    mut UNUSED_dl: *mut WebKitDownload,
    mut suggested_filename: *mut gchar,
    mut download: *mut download_t,
) -> gboolean {
    let mut L: *mut lua_State = common.L;
    luaH_object_push(L, (*download).ref_0);
    lua_pushstring(L, suggested_filename);
    current_destination_cb = download;
    let mut ret: gint = luaH_object_emit_signal(
        L,
        -(2 as std::ffi::c_int),
        b"decide-destination\0" as *const u8 as *const std::ffi::c_char,
        1 as std::ffi::c_int,
        1 as std::ffi::c_int,
    );
    let mut handled: gboolean =
        (ret != 0 && lua_toboolean(L, -(1 as std::ffi::c_int)) != 0) as std::ffi::c_int;
    lua_settop(L, -(1 as std::ffi::c_int + ret) - 1 as std::ffi::c_int);
    current_destination_cb = 0 as *mut download_t;
    if (*download).status as std::ffi::c_uint
        == LUAKIT_DOWNLOAD_STATUS_CANCELLED as std::ffi::c_int as std::ffi::c_uint
    {
        webkit_download_set_destination(
            (*download).webkit_download,
            b"/tmp/\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    return handled;
}
unsafe extern "C-unwind" fn created_destination_cb(
    mut UNUSED_dl: *mut WebKitDownload,
    mut destination: *mut gchar,
    mut download: *mut download_t,
) {
    let mut L: *mut lua_State = common.L;
    luaH_object_push(L, (*download).ref_0);
    lua_pushstring(L, destination);
    (*download).status = LUAKIT_DOWNLOAD_STATUS_CREATED;
    if !((*download).error).is_null() {
        g_free((*download).error as gpointer);
        (*download).error = 0 as *mut gchar;
    }
    luaH_object_emit_signal(
        L,
        -(2 as std::ffi::c_int),
        b"created-destination\0" as *const u8 as *const std::ffi::c_char,
        1 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
}
unsafe extern "C-unwind" fn failed_cb(
    mut UNUSED_d: *mut WebKitDownload,
    mut error: *mut GError,
    mut download: *mut download_t,
) {
    if !((*download).error).is_null() {
        g_free((*download).error as gpointer);
    }
    (*download).error = g_strdup((*error).message);
    if (*error).code == WEBKIT_DOWNLOAD_ERROR_CANCELLED_BY_USER as std::ffi::c_int {
        (*download).status = LUAKIT_DOWNLOAD_STATUS_CANCELLED;
    } else {
        _log(
            LOG_LEVEL_warn,
            b"clib/download.c\0" as *const u8 as *const std::ffi::c_char,
            &format!(
                "download {} failed: {}",
                download as usize,
                CStr::from_ptr((*error).message).to_string_lossy()
            ),
        );
        (*download).status = LUAKIT_DOWNLOAD_STATUS_FAILED;
        if !((*download).ref_0).is_null() {
            let mut L: *mut lua_State = common.L;
            luaH_object_push(L, (*download).ref_0);
            lua_pushstring(L, (*error).message);
            luaH_object_emit_signal(
                L,
                -(2 as std::ffi::c_int),
                b"error\0" as *const u8 as *const std::ffi::c_char,
                1 as std::ffi::c_int,
                0 as std::ffi::c_int,
            );
            lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        }
    };
}
unsafe extern "C-unwind" fn progress_cb(
    mut UNUSED_dl: *mut WebKitDownload,
    mut UNUSED_ps: *mut GParamSpec,
    mut download: *mut download_t,
) {
    (*download).status = LUAKIT_DOWNLOAD_STATUS_STARTED;
}
unsafe extern "C-unwind" fn finished_cb(
    mut UNUSED_dl: *mut WebKitDownload,
    mut download: *mut download_t,
) {
    let mut L: *mut lua_State = common.L;
    luaH_object_push(L, (*download).ref_0);
    if (*download).status as std::ffi::c_uint
        != LUAKIT_DOWNLOAD_STATUS_CANCELLED as std::ffi::c_int as std::ffi::c_uint
        && (*download).status as std::ffi::c_uint
            != LUAKIT_DOWNLOAD_STATUS_FAILED as std::ffi::c_int as std::ffi::c_uint
    {
        (*download).status = LUAKIT_DOWNLOAD_STATUS_FINISHED;
    }
    let mut ret: gint = luaH_object_emit_signal(
        L,
        -(1 as std::ffi::c_int),
        b"finished\0" as *const u8 as *const std::ffi::c_char,
        0 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int + ret) - 1 as std::ffi::c_int);
    luaH_download_unref(L, download);
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_download_new(mut L: *mut lua_State) -> gint {
    if !(lua_type(L, 2 as std::ffi::c_int) == 5 as std::ffi::c_int) {
        luaL_argerror(
            L,
            2 as std::ffi::c_int,
            b"table\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    let mut uri: *const gchar = 0 as *const gchar;
    let mut top: gint = lua_gettop(L);
    if luaH_rawfield(
        L,
        2 as std::ffi::c_int,
        b"uri\0" as *const u8 as *const std::ffi::c_char,
    ) != 0
        && lua_isstring(L, -(1 as std::ffi::c_int)) != 0
    {
        uri = lua_tolstring(L, -(1 as std::ffi::c_int), 0 as *mut size_t);
    }
    lua_settop(L, top);
    if uri.is_null() {
        return luaL_error(
            L,
            b"download requires a URI\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    let mut d: *mut WebKitDownload = webkit_web_context_download_uri(web_context_get(), uri);
    return luaH_download_push(L, d);
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_download_push(
    mut L: *mut lua_State,
    mut d: *mut WebKitDownload,
) -> gint {
    if luaH_uniq_get_ptr(
        L,
        b"luakit.uniq.registry.download\0" as *const u8 as *const std::ffi::c_char,
        d as gpointer,
    ) != 0
    {
        return 1 as std::ffi::c_int;
    }
    (download_class.allocator).expect("non-null function pointer")(L);
    let mut download: *mut download_t =
        luaH_checkudata(L, -(1 as std::ffi::c_int), &mut download_class) as *mut download_t;
    let mut r: *mut WebKitURIRequest = webkit_download_get_request(d);
    (*download).uri = g_strdup(webkit_uri_request_get_uri(r));
    (*download).webkit_download = d;
    g_object_ref(g_type_check_instance_cast(
        (*download).webkit_download as *mut GTypeInstance,
        ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
    ) as *mut std::ffi::c_void as *mut GObject);
    g_signal_connect_data(
        g_type_check_instance_cast(
            (*download).webkit_download as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject,
        b"decide-destination\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C-unwind" fn(
                    *mut WebKitDownload,
                    *mut gchar,
                    *mut download_t,
                ) -> gboolean,
            >,
            GCallback,
        >(Some(
            decide_destination_cb
                as unsafe extern "C-unwind" fn(
                    *mut WebKitDownload,
                    *mut gchar,
                    *mut download_t,
                ) -> gboolean,
        )),
        download as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        g_type_check_instance_cast(
            (*download).webkit_download as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject,
        b"created-destination\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C-unwind" fn(*mut WebKitDownload, *mut gchar, *mut download_t) -> (),
            >,
            GCallback,
        >(Some(
            created_destination_cb
                as unsafe extern "C-unwind" fn(
                    *mut WebKitDownload,
                    *mut gchar,
                    *mut download_t,
                ) -> (),
        )),
        download as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        g_type_check_instance_cast(
            (*download).webkit_download as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject,
        b"notify::estimated-progress\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C-unwind" fn(
                    *mut WebKitDownload,
                    *mut GParamSpec,
                    *mut download_t,
                ) -> (),
            >,
            GCallback,
        >(Some(
            progress_cb
                as unsafe extern "C-unwind" fn(
                    *mut WebKitDownload,
                    *mut GParamSpec,
                    *mut download_t,
                ) -> (),
        )),
        download as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        g_type_check_instance_cast(
            (*download).webkit_download as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject,
        b"finished\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option<unsafe extern "C-unwind" fn(*mut WebKitDownload, *mut download_t) -> ()>,
            GCallback,
        >(Some(
            finished_cb as unsafe extern "C-unwind" fn(*mut WebKitDownload, *mut download_t) -> (),
        )),
        download as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        g_type_check_instance_cast(
            (*download).webkit_download as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject,
        b"failed\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C-unwind" fn(
                    *mut WebKitDownload,
                    *mut GError,
                    *mut download_t,
                ) -> (),
            >,
            GCallback,
        >(Some(
            failed_cb
                as unsafe extern "C-unwind" fn(
                    *mut WebKitDownload,
                    *mut GError,
                    *mut download_t,
                ) -> (),
        )),
        download as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    lua_pushvalue(L, -(1 as std::ffi::c_int));
    (*download).ref_0 = luaH_object_ref(L, -(1 as std::ffi::c_int));
    luaH_uniq_add_ptr(
        L,
        b"luakit.uniq.registry.download\0" as *const u8 as *const std::ffi::c_char,
        d as gpointer,
        -(1 as std::ffi::c_int),
    );
    return 1 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_download_set_allow_overwrite(
    mut L: *mut lua_State,
    mut download: *mut download_t,
) -> gint {
    let mut allow: gboolean = lua_toboolean(L, -(1 as std::ffi::c_int));
    webkit_download_set_allow_overwrite((*download).webkit_download, allow);
    luaH_object_emit_signal(
        L,
        -(3 as std::ffi::c_int),
        b"property::allow-overwrite\0" as *const u8 as *const std::ffi::c_char,
        0 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_download_get_allow_overwrite(
    mut L: *mut lua_State,
    mut download: *mut download_t,
) -> gint {
    lua_pushboolean(
        L,
        webkit_download_get_allow_overwrite((*download).webkit_download),
    );
    return 1 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_download_set_destination(
    mut L: *mut lua_State,
    mut download: *mut download_t,
) -> gint {
    if download != current_destination_cb {
        luaH_warn(
            L,
            "cannot set destination outside decide-destination handler",
        );
        return 0 as std::ffi::c_int;
    }
    let mut destination: *const gchar =
        luaL_checklstring(L, -(1 as std::ffi::c_int), 0 as *mut size_t);
    let mut uri: *mut gchar =
        g_filename_to_uri(destination, 0 as *const gchar, 0 as *mut *mut GError);
    if !uri.is_null() {
        (*download).destination = g_strdup(destination);
        webkit_download_set_destination((*download).webkit_download, uri);
        g_free(uri as gpointer);
        luaH_object_emit_signal(
            L,
            -(3 as std::ffi::c_int),
            b"property::destination\0" as *const u8 as *const std::ffi::c_char,
            0 as std::ffi::c_int,
            0 as std::ffi::c_int,
        );
    } else {
        lua_pushfstring(
            L,
            b"invalid destination: '%s'\0" as *const u8 as *const std::ffi::c_char,
            destination,
        );
        lua_error(L);
    }
    return 0 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_download_get_destination(
    mut L: *mut lua_State,
    mut object: *mut download_t,
) -> gint {
    lua_pushstring(L, (*object).destination);
    return 1 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_download_get_progress(
    mut L: *mut lua_State,
    mut download: *mut download_t,
) -> gint {
    let mut progress: gdouble = webkit_download_get_estimated_progress((*download).webkit_download);
    lua_pushnumber(L, progress);
    return 1 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_download_get_mime_type(
    mut L: *mut lua_State,
    mut download: *mut download_t,
) -> gint {
    let mut response: *mut WebKitURIResponse =
        webkit_download_get_response((*download).webkit_download);
    if response.is_null() {
        return 0 as std::ffi::c_int;
    }
    let mut mime_type: *const gchar = webkit_uri_response_get_mime_type(response);
    if !mime_type.is_null() {
        lua_pushstring(L, mime_type);
        return 1 as std::ffi::c_int;
    }
    return 0 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_download_get_status(
    mut L: *mut lua_State,
    mut download: *mut download_t,
) -> gint {
    match (*download).status as std::ffi::c_uint {
        0 => {
            lua_pushstring(L, b"finished\0" as *const u8 as *const std::ffi::c_char);
        }
        1 => {
            lua_pushstring(L, b"created\0" as *const u8 as *const std::ffi::c_char);
        }
        2 => {
            lua_pushstring(L, b"started\0" as *const u8 as *const std::ffi::c_char);
        }
        3 => {
            lua_pushstring(L, b"cancelled\0" as *const u8 as *const std::ffi::c_char);
        }
        4 => {
            lua_pushstring(L, b"failed\0" as *const u8 as *const std::ffi::c_char);
        }
        _ => {
            luaH_warn(L, "unknown download status");
            return 0 as std::ffi::c_int;
        }
    }
    return 1 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_download_get_error(
    mut L: *mut lua_State,
    mut object: *mut download_t,
) -> gint {
    lua_pushstring(L, (*object).error);
    return 1 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_download_get_content_length(
    mut L: *mut lua_State,
    mut download: *mut download_t,
) -> gint {
    let mut total_size: gdouble = webkit_uri_response_get_content_length(
        webkit_download_get_response((*download).webkit_download),
    ) as gdouble;
    lua_pushnumber(L, total_size);
    return 1 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_download_get_received_data_length(
    mut L: *mut lua_State,
    mut download: *mut download_t,
) -> gint {
    let mut current_size: gdouble =
        webkit_download_get_received_data_length((*download).webkit_download) as gdouble;
    lua_pushnumber(L, current_size);
    return 1 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_download_get_elapsed_time(
    mut L: *mut lua_State,
    mut download: *mut download_t,
) -> gint {
    let mut elapsed_time: gdouble = webkit_download_get_elapsed_time((*download).webkit_download);
    lua_pushnumber(L, elapsed_time);
    return 1 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_download_get_suggested_filename(
    mut L: *mut lua_State,
    mut download: *mut download_t,
) -> gint {
    let mut suggested_filename: *const gchar = webkit_uri_response_get_suggested_filename(
        webkit_download_get_response((*download).webkit_download),
    );
    lua_pushstring(L, suggested_filename);
    return 1 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_download_set_uri(
    mut L: *mut lua_State,
    mut download: *mut download_t,
) -> gint {
    let mut uri: *mut gchar =
        luaL_checklstring(L, -(1 as std::ffi::c_int), 0 as *mut size_t) as *mut gchar;
    if !(g_strrstr(uri, b"://\0" as *const u8 as *const std::ffi::c_char)).is_null() {
        uri = g_strdup(uri);
    } else {
        uri = g_strdup_printf(b"http://%s\0" as *const u8 as *const std::ffi::c_char, uri);
    }
    (*download).uri = uri;
    return 0 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_download_get_uri(
    mut L: *mut lua_State,
    mut object: *mut download_t,
) -> gint {
    lua_pushstring(L, (*object).uri);
    return 1 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_download_start(mut UNUSED_L: *mut lua_State) -> gint {
    return 0 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_download_cancel(mut L: *mut lua_State) -> gint {
    let mut download: *mut download_t =
        luaH_checkudata(L, 1 as std::ffi::c_int, &mut download_class) as *mut download_t;
    webkit_download_cancel((*download).webkit_download);
    (*download).status = LUAKIT_DOWNLOAD_STATUS_CANCELLED;
    return 0 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn download_class_setup(mut L: *mut lua_State) {
    static mut download_methods: [luaL_Reg; 4] = unsafe {
        [
            {
                let mut init = luaL_Reg {
                    name: b"add_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_download_class_add_signal,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"remove_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_download_class_remove_signal,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"emit_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_download_class_emit_signal,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__call\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_download_new,
                };
                init
            },
            // {
            //     let mut init = luaL_Reg {
            //         name: 0 as *const std::ffi::c_char,
            //         func: None,
            //     };
            //     init
            // },
        ]
    };
    static mut download_meta: [luaL_Reg; 10] = unsafe {
        [
            {
                let mut init = luaL_Reg {
                    name: b"__tostring\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_object_tostring,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"add_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_object_add_signal_simple,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"remove_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_object_remove_signal_simple,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"remove_signals\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_object_remove_signals_simple,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"emit_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_object_emit_signal_simple,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__index\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_class_index,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__newindex\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_class_newindex,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"start\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_download_start,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"cancel\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_download_cancel,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__gc\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_download_gc,
                };
                init
            },
            // {
            //     let mut init = luaL_Reg {
            //         name: 0 as *const std::ffi::c_char,
            //         func: None,
            //     };
            //     init
            // },
        ]
    };
    luaH_class_setup(
        L,
        &mut download_class,
        b"download\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option<unsafe extern "C-unwind" fn(*mut lua_State) -> *mut download_t>,
            lua_class_allocator_t,
        >(Some(
            download_new as unsafe extern "C-unwind" fn(*mut lua_State) -> *mut download_t,
        )),
        None,
        None,
        download_methods.as_ptr(),
        download_meta.as_ptr(),
    );
    luaH_class_add_property(
        &mut download_class,
        L_TK_ALLOW_OVERWRITE,
        ::core::mem::transmute::<
            Option<unsafe extern "C-unwind" fn(*mut lua_State, *mut download_t) -> gint>,
            lua_class_propfunc_t,
        >(Some(
            luaH_download_set_allow_overwrite
                as unsafe extern "C-unwind" fn(*mut lua_State, *mut download_t) -> gint,
        )),
        ::core::mem::transmute::<
            Option<unsafe extern "C-unwind" fn(*mut lua_State, *mut download_t) -> gint>,
            lua_class_propfunc_t,
        >(Some(
            luaH_download_get_allow_overwrite
                as unsafe extern "C-unwind" fn(*mut lua_State, *mut download_t) -> gint,
        )),
        ::core::mem::transmute::<
            Option<unsafe extern "C-unwind" fn(*mut lua_State, *mut download_t) -> gint>,
            lua_class_propfunc_t,
        >(Some(
            luaH_download_set_allow_overwrite
                as unsafe extern "C-unwind" fn(*mut lua_State, *mut download_t) -> gint,
        )),
    );
    luaH_class_add_property(
        &mut download_class,
        L_TK_DESTINATION,
        ::core::mem::transmute::<
            Option<unsafe extern "C-unwind" fn(*mut lua_State, *mut download_t) -> gint>,
            lua_class_propfunc_t,
        >(Some(
            luaH_download_set_destination
                as unsafe extern "C-unwind" fn(*mut lua_State, *mut download_t) -> gint,
        )),
        ::core::mem::transmute::<
            Option<unsafe extern "C-unwind" fn(*mut lua_State, *mut download_t) -> gint>,
            lua_class_propfunc_t,
        >(Some(
            luaH_download_get_destination
                as unsafe extern "C-unwind" fn(*mut lua_State, *mut download_t) -> gint,
        )),
        ::core::mem::transmute::<
            Option<unsafe extern "C-unwind" fn(*mut lua_State, *mut download_t) -> gint>,
            lua_class_propfunc_t,
        >(Some(
            luaH_download_set_destination
                as unsafe extern "C-unwind" fn(*mut lua_State, *mut download_t) -> gint,
        )),
    );
    luaH_class_add_property(
        &mut download_class,
        L_TK_PROGRESS,
        None,
        ::core::mem::transmute::<
            Option<unsafe extern "C-unwind" fn(*mut lua_State, *mut download_t) -> gint>,
            lua_class_propfunc_t,
        >(Some(
            luaH_download_get_progress
                as unsafe extern "C-unwind" fn(*mut lua_State, *mut download_t) -> gint,
        )),
        None,
    );
    luaH_class_add_property(
        &mut download_class,
        L_TK_STATUS,
        None,
        ::core::mem::transmute::<
            Option<unsafe extern "C-unwind" fn(*mut lua_State, *mut download_t) -> gint>,
            lua_class_propfunc_t,
        >(Some(
            luaH_download_get_status
                as unsafe extern "C-unwind" fn(*mut lua_State, *mut download_t) -> gint,
        )),
        None,
    );
    luaH_class_add_property(
        &mut download_class,
        L_TK_ERROR,
        None,
        ::core::mem::transmute::<
            Option<unsafe extern "C-unwind" fn(*mut lua_State, *mut download_t) -> gint>,
            lua_class_propfunc_t,
        >(Some(
            luaH_download_get_error
                as unsafe extern "C-unwind" fn(*mut lua_State, *mut download_t) -> gint,
        )),
        None,
    );
    luaH_class_add_property(
        &mut download_class,
        L_TK_TOTAL_SIZE,
        None,
        ::core::mem::transmute::<
            Option<unsafe extern "C-unwind" fn(*mut lua_State, *mut download_t) -> gint>,
            lua_class_propfunc_t,
        >(Some(
            luaH_download_get_content_length
                as unsafe extern "C-unwind" fn(*mut lua_State, *mut download_t) -> gint,
        )),
        None,
    );
    luaH_class_add_property(
        &mut download_class,
        L_TK_CURRENT_SIZE,
        None,
        ::core::mem::transmute::<
            Option<unsafe extern "C-unwind" fn(*mut lua_State, *mut download_t) -> gint>,
            lua_class_propfunc_t,
        >(Some(
            luaH_download_get_received_data_length
                as unsafe extern "C-unwind" fn(*mut lua_State, *mut download_t) -> gint,
        )),
        None,
    );
    luaH_class_add_property(
        &mut download_class,
        L_TK_ELAPSED_TIME,
        None,
        ::core::mem::transmute::<
            Option<unsafe extern "C-unwind" fn(*mut lua_State, *mut download_t) -> gint>,
            lua_class_propfunc_t,
        >(Some(
            luaH_download_get_elapsed_time
                as unsafe extern "C-unwind" fn(*mut lua_State, *mut download_t) -> gint,
        )),
        None,
    );
    luaH_class_add_property(
        &mut download_class,
        L_TK_MIME_TYPE,
        None,
        ::core::mem::transmute::<
            Option<unsafe extern "C-unwind" fn(*mut lua_State, *mut download_t) -> gint>,
            lua_class_propfunc_t,
        >(Some(
            luaH_download_get_mime_type
                as unsafe extern "C-unwind" fn(*mut lua_State, *mut download_t) -> gint,
        )),
        None,
    );
    luaH_class_add_property(
        &mut download_class,
        L_TK_SUGGESTED_FILENAME,
        None,
        ::core::mem::transmute::<
            Option<unsafe extern "C-unwind" fn(*mut lua_State, *mut download_t) -> gint>,
            lua_class_propfunc_t,
        >(Some(
            luaH_download_get_suggested_filename
                as unsafe extern "C-unwind" fn(*mut lua_State, *mut download_t) -> gint,
        )),
        None,
    );
    luaH_class_add_property(
        &mut download_class,
        L_TK_URI,
        ::core::mem::transmute::<
            Option<unsafe extern "C-unwind" fn(*mut lua_State, *mut download_t) -> gint>,
            lua_class_propfunc_t,
        >(Some(
            luaH_download_set_uri
                as unsafe extern "C-unwind" fn(*mut lua_State, *mut download_t) -> gint,
        )),
        ::core::mem::transmute::<
            Option<unsafe extern "C-unwind" fn(*mut lua_State, *mut download_t) -> gint>,
            lua_class_propfunc_t,
        >(Some(
            luaH_download_get_uri
                as unsafe extern "C-unwind" fn(*mut lua_State, *mut download_t) -> gint,
        )),
        ::core::mem::transmute::<*mut std::ffi::c_void, lua_class_propfunc_t>(
            0 as *mut std::ffi::c_void,
        ),
    );
    luaH_uniq_setup(
        L,
        b"luakit.uniq.registry.download\0" as *const u8 as *const std::ffi::c_char,
        b"v\0" as *const u8 as *const std::ffi::c_char,
    );
}
