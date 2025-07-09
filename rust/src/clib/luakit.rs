pub mod luakit_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct proc_callback_data_t {
        pub cb_ref: gpointer,
        pub stdout_fd: gint,
        pub stderr_fd: gint,
    }
    use glib_sys::gpointer;

    use crate::gtypes::gint;
}
use gdk_sys::*;
use glib_sys::*;
use libc::*;
use mlua_sys::*;

use crate::common::clib::luakit::*;
use crate::common::common;
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
use gdk_sys::*;
use glib_sys::*;
use libc::getenv;
use mlua_sys::*;
use webkit2gtk::{ffi::*, glib::gobject_ffi::*};

use crate::clib::msg::*;
use crate::clib::soup::*;
use crate::clib::sqlite3::*;
use crate::clib::stylesheet::*;
use crate::clib::web_module::*;
use crate::clib::widget::*;
use crate::common::luaclass::luaH_typename;
use crate::common::luah::*;
use crate::common::luaobject::*;
use crate::common::luautil::*;
use crate::common::luayield::luaH_yield;
use crate::common::util::*;
use crate::common::*;
use crate::globalconf::*;
use crate::gtypes::*;
use crate::log::*;
use gdk_sys::GdkAtom;
use gio_sys::{GAsyncReadyCallback, GAsyncResult, GCancellable, GFile};
use glib_sys::*;
use gtk_sys::*;
use libc::*;
use mlua_sys::*;
use webkit2gtk::{
    ffi::*,
    gio::ffi::{GTlsCertificate, g_tls_certificate_new_from_pem},
    glib::gobject_ffi::{
        GObject, GTypeInstance, g_object_unref, g_type_check_instance_cast,
        g_type_check_instance_is_a,
    },
    *,
};

use crate::{
    clib::widget::{widget_class, widget_t},
    common::{
        clib::luakit::{
            luaH_luakit_idle_add, luaH_luakit_idle_remove, luaH_luakit_uri_decode, luaH_object_ref,
            luaH_object_unref,
        },
        luaclass::{
            lua_class_property_array_t, lua_class_t, lua_object_t, luaH_checkudata,
            luaH_class_add_signal, luaH_class_emit_signal, luaH_class_remove_signal, luaH_openlib,
            luaH_usemetatable,
            signal_h::{signal_new, signal_t},
        },
        lualib::luaH_dofunction,
        luaobject::luaH_object_push,
        tokenize::{L_TK_PRIMARY, L_TK_SECONDARY, l_tokenize, luakit_token_t},
    },
    globalconf::globalconf,
    gtypes::{gchar, gint, gint64, gsize, guint, guint64},
    ipc::ipc_remove_socket_file,
    log::{_log, LOG_LEVEL_fatal, LOG_LEVEL_verbose, log_get_verbosity},
    web_context::web_context_get,
};

pub use self::luakit_h::proc_callback_data_t;

unsafe extern "C-unwind" {
    pub fn luakit_uri_scheme_request_cb(_: *mut WebKitURISchemeRequest, _: gpointer);
}
pub type website_data_remove_task_t = _website_data_remove_task_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _website_data_remove_task_t {
    pub L: *mut lua_State,
    pub data_types: WebKitWebsiteDataTypes,
    pub domain: *mut std::ffi::c_char,
}
static mut luakit_class: lua_class_t = lua_class_t {
    name: 0 as *const gchar,
    signals: 0 as *const signal_t as *mut signal_t,
    allocator: None,
    properties: 0 as *const lua_class_property_array_t as *mut lua_class_property_array_t,
    index_miss_property: None,
    newindex_miss_property: None,
};
#[inline]
unsafe extern "C-unwind" fn luaH_luakit_class_add_signal(mut L: *mut lua_State) -> gint {
    luaH_class_add_signal(
        L,
        &mut luakit_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn luaH_luakit_class_remove_signal(mut L: *mut lua_State) -> gint {
    luaH_class_remove_signal(
        L,
        &mut luakit_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn luaH_luakit_class_emit_signal(mut L: *mut lua_State) -> gint {
    return luaH_class_emit_signal(
        L,
        &mut luakit_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t),
        lua_gettop(L) - 1 as std::ffi::c_int,
        -(1 as std::ffi::c_int),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_clipboard_get(
    mut L: *mut lua_State,
    mut idx: gint,
) -> *mut GtkClipboard {
    match l_tokenize(luaL_checklstring(L, idx, 0 as *mut size_t)) as std::ffi::c_uint {
        L_TK_PRIMARY => {
            return gtk_clipboard_get(1 as GdkAtom);
        }
        L_TK_SECONDARY => {
            return gtk_clipboard_get(2 as GdkAtom);
        }
        L_TK_CLIPBOARD => {
            return gtk_clipboard_get(69 as GdkAtom);
        }
        _ => {}
    }
    return 0 as *mut GtkClipboard;
}
unsafe extern "C-unwind" fn luaH_luakit_selection_index(mut L: *mut lua_State) -> gint {
    let mut selection: *mut GtkClipboard = luaH_clipboard_get(L, 2 as std::ffi::c_int);
    if !selection.is_null() {
        let mut text: *mut gchar = gtk_clipboard_wait_for_text(selection);
        if !text.is_null() {
            lua_pushstring(L, text);
            g_free(text as gpointer);
            return 1 as std::ffi::c_int;
        }
    }
    return 0 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_luakit_selection_newindex(mut L: *mut lua_State) -> gint {
    let mut selection: *mut GtkClipboard = luaH_clipboard_get(L, 2 as std::ffi::c_int);
    if !selection.is_null() {
        let mut text: *const gchar = if !(lua_type(L, 3 as std::ffi::c_int) == 0 as std::ffi::c_int)
        {
            luaL_checklstring(L, 3 as std::ffi::c_int, 0 as *mut size_t)
        } else {
            0 as *const std::ffi::c_char
        };
        if !text.is_null() && *text as std::ffi::c_int != 0 {
            gtk_clipboard_set_text(selection, text, -(1 as std::ffi::c_int));
        } else {
            gtk_clipboard_clear(selection);
        }
    }
    return 0 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_luakit_selection_table_push(mut L: *mut lua_State) -> gint {
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_createtable(L, 0 as std::ffi::c_int, 2 as std::ffi::c_int);
    lua_pushlstring(
        L,
        b"__index\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 8]>())
            .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
            .wrapping_sub(1),
    );
    lua_pushcclosure(L, luaH_luakit_selection_index, 0 as std::ffi::c_int);
    lua_rawset(L, -(3 as std::ffi::c_int));
    lua_pushlstring(
        L,
        b"__newindex\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 11]>())
            .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
            .wrapping_sub(1),
    );
    lua_pushcclosure(L, luaH_luakit_selection_newindex, 0 as std::ffi::c_int);
    lua_rawset(L, -(3 as std::ffi::c_int));
    lua_setmetatable(L, -(2 as std::ffi::c_int));
    return 1 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_luakit_save_file(mut L: *mut lua_State) -> gint {
    let mut title: *const gchar = luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t);
    let mut parent_window: *mut GtkWindow = 0 as *mut GtkWindow;
    if !(lua_type(L, 2 as std::ffi::c_int) == 0 as std::ffi::c_int) {
        let mut parent: *mut widget_t =
            luaH_checkudata(L, 2 as std::ffi::c_int, &mut widget_class) as *mut widget_t;
        if {
            let mut __inst: *mut GTypeInstance = (*parent).widget as *mut GTypeInstance;
            let mut __t: GType = gtk_window_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as std::ffi::c_int;
            } else if !((*__inst).g_class).is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as std::ffi::c_int == 0) as std::ffi::c_int;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        } == 0
        {
            luaL_argerror(
                L,
                2 as std::ffi::c_int,
                b"window widget\0" as *const u8 as *const std::ffi::c_char,
            );
        }
        parent_window = g_type_check_instance_cast(
            (*parent).widget as *mut GTypeInstance,
            gtk_window_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkWindow;
    }
    let mut default_folder: *const gchar =
        luaL_checklstring(L, 3 as std::ffi::c_int, 0 as *mut size_t);
    let mut default_name: *const gchar =
        luaL_checklstring(L, 4 as std::ffi::c_int, 0 as *mut size_t);
    let mut dialog: *mut GtkWidget = gtk_file_chooser_dialog_new(
        title,
        parent_window,
        GTK_FILE_CHOOSER_ACTION_SAVE,
        b"_Cancel\0" as *const u8 as *const std::ffi::c_char,
        GTK_RESPONSE_CANCEL as std::ffi::c_int,
        b"_Save\0" as *const u8 as *const std::ffi::c_char,
        GTK_RESPONSE_ACCEPT as std::ffi::c_int,
        0 as *mut std::ffi::c_void,
    );
    gtk_file_chooser_set_current_folder(
        g_type_check_instance_cast(dialog as *mut GTypeInstance, gtk_file_chooser_get_type())
            as *mut std::ffi::c_void as *mut GtkFileChooser,
        default_folder,
    );
    gtk_file_chooser_set_current_name(
        g_type_check_instance_cast(dialog as *mut GTypeInstance, gtk_file_chooser_get_type())
            as *mut std::ffi::c_void as *mut GtkFileChooser,
        default_name,
    );
    gtk_file_chooser_set_do_overwrite_confirmation(
        g_type_check_instance_cast(dialog as *mut GTypeInstance, gtk_file_chooser_get_type())
            as *mut std::ffi::c_void as *mut GtkFileChooser,
        (0 as std::ffi::c_int == 0) as std::ffi::c_int,
    );
    if gtk_dialog_run(g_type_check_instance_cast(
        dialog as *mut GTypeInstance,
        gtk_dialog_get_type(),
    ) as *mut std::ffi::c_void as *mut GtkDialog)
        == GTK_RESPONSE_ACCEPT as std::ffi::c_int
    {
        let mut filename: *mut gchar = gtk_file_chooser_get_filename(g_type_check_instance_cast(
            dialog as *mut GTypeInstance,
            gtk_file_chooser_get_type(),
        )
            as *mut std::ffi::c_void
            as *mut GtkFileChooser);
        lua_pushstring(L, filename);
        g_free(filename as gpointer);
    } else {
        lua_pushnil(L);
    }
    gtk_widget_destroy(dialog);
    return 1 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_luakit_spawn_sync(mut L: *mut lua_State) -> gint {
    let mut e: *mut GError = 0 as *mut GError;
    let mut _stdout: *mut gchar = std::ptr::null_mut();
    let mut _stderr: *mut gchar = std::ptr::null_mut();
    let mut rv: gint = 0;
    let mut sigact: sigaction = sigaction {
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
        sa_sigaction: 0,
    };
    let mut oldact: sigaction = sigaction {
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
        sa_sigaction: 0,
    };
    let mut command: *const gchar = luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t);
    sigemptyset(&mut sigact.sa_mask);
    sigact.sa_flags = 0 as std::ffi::c_int;
    if sigaction(17 as std::ffi::c_int, &mut sigact, &mut oldact) != 0 {
        _log(
            LOG_LEVEL_fatal,
            b"clib/luakit.c\0" as *const u8 as *const std::ffi::c_char,
            b"Can't clear SIGCHLD handler\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    g_spawn_command_line_sync(
        command,
        *(&mut _stdout) as *mut *mut u8,
        *(&mut _stderr) as *mut *mut u8,
        &mut rv,
        &mut e,
    );
    if sigaction(17 as std::ffi::c_int, &mut oldact, 0 as *mut sigaction) != 0 {
        _log(
            LOG_LEVEL_fatal,
            b"clib/luakit.c\0" as *const u8 as *const std::ffi::c_char,
            b"Can't restore SIGCHLD handler\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    if !e.is_null() {
        lua_pushstring(L, (*e).message);
        g_clear_error(&mut e);
        lua_error(L);
    }
    lua_pushinteger(
        L,
        ((rv & 0xff00 as std::ffi::c_int) >> 8 as std::ffi::c_int) as lua_Integer,
    );
    lua_pushstring(L, _stdout);
    lua_pushstring(L, _stderr);
    g_free(_stdout as gpointer);
    g_free(_stderr as gpointer);
    return 3 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn read_proc_output(
    mut fd: std::ffi::c_int,
    mut L: *mut lua_State,
    mut ptr_out: *mut *mut gchar,
    mut len_out: *mut gsize,
) -> bool {
    let mut g_out: *mut GIOChannel = g_io_channel_unix_new(fd);
    let mut e: *mut GError = std::ptr::null_mut();
    g_io_channel_read_to_end(
        g_out,
        ptr_out as *mut *mut u8,
        len_out as *mut usize,
        &mut e,
    );
    if !e.is_null() {
        lua_pushstring(L, (*e).message);
        g_clear_error(&mut e);
        g_free(*ptr_out as gpointer);
        lua_error(L);
        g_io_channel_unref(g_out);
        close(fd);
        return 0 as std::ffi::c_int != 0;
    }
    g_io_channel_unref(g_out);
    close(fd);
    return 1 as std::ffi::c_int != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn async_callback_handler(
    mut pid: GPid,
    mut status: gint,
    mut data: gpointer,
) {
    if data.is_null() {
        g_spawn_close_pid(pid);
        return;
    }
    let mut L: *mut lua_State = common.L;
    let mut cb: *mut proc_callback_data_t = data as *mut proc_callback_data_t;
    let mut cb_ref: gpointer = (*cb).cb_ref;
    let mut stdout_fd: std::ffi::c_int = (*cb).stdout_fd;
    let mut stderr_fd: std::ffi::c_int = (*cb).stderr_fd;
    let mut str_stdout: *mut gchar = 0 as *mut gchar;
    let mut len_stdout: gsize = 0;
    if !read_proc_output(stdout_fd, L, &mut str_stdout, &mut len_stdout) {
        g_spawn_close_pid(pid);
        return;
    }
    let mut str_stderr: *mut gchar = 0 as *mut gchar;
    let mut len_stderr: gsize = 0;
    if !read_proc_output(stderr_fd, L, &mut str_stderr, &mut len_stderr) {
        g_free(str_stdout as gpointer);
        g_spawn_close_pid(pid);
        return;
    }
    g_spawn_close_pid(pid);
    if status & 0x7f as std::ffi::c_int == 0 as std::ffi::c_int {
        lua_pushlstring(
            L,
            b"exit\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 5]>())
                .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
                .wrapping_sub(1),
        );
        lua_pushinteger(
            L,
            ((status & 0xff00 as std::ffi::c_int) >> 8 as std::ffi::c_int) as lua_Integer,
        );
        lua_pushlstring(L, str_stdout, len_stdout as size_t);
        lua_pushlstring(L, str_stderr, len_stderr as size_t);
    } else if ((status & 0x7f as std::ffi::c_int) + 1 as std::ffi::c_int) as std::ffi::c_schar
        as std::ffi::c_int
        >> 1 as std::ffi::c_int
        > 0 as std::ffi::c_int
    {
        lua_pushlstring(
            L,
            b"signal\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 7]>())
                .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
                .wrapping_sub(1),
        );
        lua_pushinteger(L, (status & 0x7f as std::ffi::c_int) as lua_Integer);
        lua_pushlstring(L, str_stdout, len_stdout as size_t);
        lua_pushlstring(L, str_stderr, len_stderr as size_t);
    } else {
        lua_pushlstring(
            L,
            b"unknown\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 8]>())
                .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
                .wrapping_sub(1),
        );
        lua_pushinteger(L, -(1 as std::ffi::c_int) as lua_Integer);
    }
    luaH_object_push(L, cb_ref);
    luaH_dofunction(L, 4 as std::ffi::c_int, 0 as std::ffi::c_int);
    g_free(str_stdout as gpointer);
    g_free(str_stderr as gpointer);
    luaH_object_unref(L, cb_ref);
}
unsafe extern "C-unwind" fn luaH_luakit_spawn(mut L: *mut lua_State) -> gint {
    let mut e: *mut GError = 0 as *mut GError;
    let mut pid: GPid = 0 as std::ffi::c_int;
    let mut command: *const gchar = luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t);
    let mut argc: gint = 0 as std::ffi::c_int;
    let mut argv: *mut *mut gchar = 0 as *mut *mut gchar;
    let mut cb: *mut proc_callback_data_t =
        g_malloc0_n(1, ::core::mem::size_of::<proc_callback_data_t>()) as *mut proc_callback_data_t;
    if lua_gettop(L) > 1 as std::ffi::c_int
        && !(lua_type(L, 2 as std::ffi::c_int) == 0 as std::ffi::c_int)
    {
        if lua_type(L, 2 as std::ffi::c_int) == 6 as std::ffi::c_int {
            (*cb).cb_ref = luaH_object_ref(L, 2 as std::ffi::c_int);
        } else if lua_type(L, 4 as std::ffi::c_int) == 6 as std::ffi::c_int {
            (*cb).cb_ref = luaH_object_ref(L, 4 as std::ffi::c_int);
        } else {
            luaL_argerror(
                L,
                2 as std::ffi::c_int,
                lua_typename(L, 6 as std::ffi::c_int),
            );
        }
    }
    if !(g_shell_parse_argv(command, &mut argc, &mut argv, &mut e) == 0) {
        if !(g_spawn_async_with_pipes(
            0 as *const gchar,
            argv,
            0 as *mut *mut gchar,
            (G_SPAWN_DO_NOT_REAP_CHILD as std::ffi::c_int | G_SPAWN_SEARCH_PATH as std::ffi::c_int)
                as GSpawnFlags,
            None,
            0 as *mut std::ffi::c_void,
            &mut pid,
            0 as *mut gint,
            &mut (*cb).stdout_fd,
            &mut (*cb).stderr_fd,
            &mut e,
        ) == 0)
        {
            g_child_watch_add(pid, Some(async_callback_handler), cb as gpointer);
            g_strfreev(argv);
            lua_pushnumber(L, pid as lua_Number);
            return 1 as std::ffi::c_int;
        }
    }
    luaH_object_unref(L, (*cb).cb_ref);
    lua_pushstring(L, (*e).message);
    g_clear_error(&mut e);
    g_strfreev(argv);
    lua_error(L);
    return 0 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_luakit_exec(mut L: *mut lua_State) -> gint {
    static mut shell: *const gchar = 0 as *const gchar;
    if shell.is_null() && {
        shell = g_getenv(b"SHELL\0" as *const u8 as *const std::ffi::c_char);
        shell.is_null()
    } {
        shell = b"/bin/sh\0" as *const u8 as *const std::ffi::c_char;
    }
    ipc_remove_socket_file();
    execl(
        shell,
        shell,
        b"-c\0" as *const u8 as *const std::ffi::c_char,
        luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t),
        0 as *mut std::ffi::c_void,
    );
    return 0 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_luakit_push_options_table(mut L: *mut lua_State) -> gint {
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    let mut i = 0;
    while i < (*globalconf.argv).len {
        lua_pushstring(
            L,
            *((*globalconf.argv).pdata).offset(i as isize) as *const std::ffi::c_char,
        );
        lua_rawseti(L, -(2 as std::ffi::c_int), i.wrapping_add(1).into());
        i = i.wrapping_add(1);
        i;
    }
    return 1 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_parse_website_data_types_table(
    mut L: *mut lua_State,
    mut idx: gint,
) -> WebKitWebsiteDataTypes {
    let mut types: WebKitWebsiteDataTypes = 0 as WebKitWebsiteDataTypes;
    idx = luaH_absindex(L, idx);
    if !(lua_type(L, idx) == 5 as std::ffi::c_int) {
        luaL_argerror(L, idx, b"table\0" as *const u8 as *const std::ffi::c_char);
    }
    let mut len: size_t = lua_objlen(L, idx);
    let mut i: size_t = 1 as std::ffi::c_int as size_t;
    while i <= len {
        lua_rawgeti(L, idx, i as i64);
        if lua_isstring(L, -(1 as std::ffi::c_int)) == 0 {
            luaL_error(
                L,
                b"website data types must be strings\0" as *const u8 as *const std::ffi::c_char,
            );
        }
        let mut type_0: *const std::ffi::c_char =
            lua_tolstring(L, -(1 as std::ffi::c_int), 0 as *mut size_t);
        if strcmp(
            type_0,
            b"memory_cache\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            types = ::core::mem::transmute::<std::ffi::c_uint, WebKitWebsiteDataTypes>(
                types as std::ffi::c_uint
                    | WEBKIT_WEBSITE_DATA_MEMORY_CACHE as std::ffi::c_int as std::ffi::c_uint,
            );
        }
        if strcmp(
            type_0,
            b"disk_cache\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            types = ::core::mem::transmute::<std::ffi::c_uint, WebKitWebsiteDataTypes>(
                types as std::ffi::c_uint
                    | WEBKIT_WEBSITE_DATA_DISK_CACHE as std::ffi::c_int as std::ffi::c_uint,
            );
        }
        if strcmp(
            type_0,
            b"offline_application_cache\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            types = ::core::mem::transmute::<std::ffi::c_uint, WebKitWebsiteDataTypes>(
                types as std::ffi::c_uint
                    | WEBKIT_WEBSITE_DATA_OFFLINE_APPLICATION_CACHE as std::ffi::c_int
                        as std::ffi::c_uint,
            );
        }
        if strcmp(
            type_0,
            b"session_storage\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            types = ::core::mem::transmute::<std::ffi::c_uint, WebKitWebsiteDataTypes>(
                types as std::ffi::c_uint
                    | WEBKIT_WEBSITE_DATA_SESSION_STORAGE as std::ffi::c_int as std::ffi::c_uint,
            );
        }
        if strcmp(
            type_0,
            b"local_storage\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            types = ::core::mem::transmute::<std::ffi::c_uint, WebKitWebsiteDataTypes>(
                types as std::ffi::c_uint
                    | WEBKIT_WEBSITE_DATA_LOCAL_STORAGE as std::ffi::c_int as std::ffi::c_uint,
            );
        }
        if strcmp(
            type_0,
            b"indexeddb_databases\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            types = ::core::mem::transmute::<std::ffi::c_uint, WebKitWebsiteDataTypes>(
                types as std::ffi::c_uint
                    | WEBKIT_WEBSITE_DATA_INDEXEDDB_DATABASES as std::ffi::c_int
                        as std::ffi::c_uint,
            );
        }
        if strcmp(
            type_0,
            b"plugin_data\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            types = ::core::mem::transmute::<std::ffi::c_uint, WebKitWebsiteDataTypes>(
                types as std::ffi::c_uint
                    | WEBKIT_WEBSITE_DATA_PLUGIN_DATA as std::ffi::c_int as std::ffi::c_uint,
            );
        }
        if strcmp(type_0, b"cookies\0" as *const u8 as *const std::ffi::c_char)
            == 0 as std::ffi::c_int
        {
            types = ::core::mem::transmute::<std::ffi::c_uint, WebKitWebsiteDataTypes>(
                types as std::ffi::c_uint
                    | WEBKIT_WEBSITE_DATA_COOKIES as std::ffi::c_int as std::ffi::c_uint,
            );
        }
        if strcmp(
            type_0,
            b"device_id_hash_salt\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            types = ::core::mem::transmute::<std::ffi::c_uint, WebKitWebsiteDataTypes>(
                types as std::ffi::c_uint
                    | WEBKIT_WEBSITE_DATA_DEVICE_ID_HASH_SALT as std::ffi::c_int
                        as std::ffi::c_uint,
            );
        }
        if strcmp(
            type_0,
            b"hsts_cache\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            types = ::core::mem::transmute::<std::ffi::c_uint, WebKitWebsiteDataTypes>(
                types as std::ffi::c_uint
                    | WEBKIT_WEBSITE_DATA_HSTS_CACHE as std::ffi::c_int as std::ffi::c_uint,
            );
        }
        if strcmp(type_0, b"all\0" as *const u8 as *const std::ffi::c_char) == 0 as std::ffi::c_int
        {
            types = ::core::mem::transmute::<std::ffi::c_uint, WebKitWebsiteDataTypes>(
                types as std::ffi::c_uint
                    | WEBKIT_WEBSITE_DATA_ALL as std::ffi::c_int as std::ffi::c_uint,
            );
        }
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        i = i.wrapping_add(1);
        i;
    }
    return types;
}
unsafe extern "C-unwind" fn website_data_fetch_finish(
    mut manager: *mut WebKitWebsiteDataManager,
    mut result: *mut GAsyncResult,
    mut L: *mut lua_State,
) {
    let mut __n1: gint64 = lua_status(L) as gint64;
    let mut __n2: gint64 = 1 as std::ffi::c_int as gint64;
    if !(__n1 == __n2) {
        /*
        g_assertion_message_cmpint(
            0 as *mut gchar,
            b"clib/luakit.c\0" as *const u8 as *const std::ffi::c_char,
            517 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 26], &[std::ffi::c_char; 26]>(
                b"website_data_fetch_finish\0",
            ))
            .as_ptr(),
            b"lua_status(L) == LUA_YIELD\0" as *const u8 as *const std::ffi::c_char,
            __n1 as guint64,
            b"==\0" as *const u8 as *const std::ffi::c_char,
            __n2 as guint64,
            'i' as i32 as std::ffi::c_char,
        );
        */
    }
    let mut error: *mut GError = 0 as *mut GError;
    let mut items: *mut GList =
        webkit_website_data_manager_fetch_finish(manager, result, &mut error);
    if !error.is_null() {
        lua_pushnil(L);
        lua_pushstring(L, (*error).message);
        g_error_free(error);
    } else {
        lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
        let mut item: *mut GList = items;
        while !item.is_null() {
            let mut website_data: *mut WebKitWebsiteData = (*item).data as *mut WebKitWebsiteData;
            let mut present: WebKitWebsiteDataTypes = webkit_website_data_get_types(website_data);
            lua_pushstring(L, webkit_website_data_get_name(website_data));
            lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
            if present as std::ffi::c_uint
                & WEBKIT_WEBSITE_DATA_MEMORY_CACHE as std::ffi::c_int as std::ffi::c_uint
                != 0
            {
                lua_pushstring(L, b"memory_cache\0" as *const u8 as *const std::ffi::c_char);
                lua_pushinteger(
                    L,
                    webkit_website_data_get_size(website_data, WEBKIT_WEBSITE_DATA_MEMORY_CACHE)
                        as lua_Integer,
                );
                lua_rawset(L, -(3 as std::ffi::c_int));
            }
            if present as std::ffi::c_uint
                & WEBKIT_WEBSITE_DATA_DISK_CACHE as std::ffi::c_int as std::ffi::c_uint
                != 0
            {
                lua_pushstring(L, b"disk_cache\0" as *const u8 as *const std::ffi::c_char);
                lua_pushinteger(
                    L,
                    webkit_website_data_get_size(website_data, WEBKIT_WEBSITE_DATA_DISK_CACHE)
                        as lua_Integer,
                );
                lua_rawset(L, -(3 as std::ffi::c_int));
            }
            if present as std::ffi::c_uint
                & WEBKIT_WEBSITE_DATA_OFFLINE_APPLICATION_CACHE as std::ffi::c_int
                    as std::ffi::c_uint
                != 0
            {
                lua_pushstring(
                    L,
                    b"offline_application_cache\0" as *const u8 as *const std::ffi::c_char,
                );
                lua_pushinteger(
                    L,
                    webkit_website_data_get_size(
                        website_data,
                        WEBKIT_WEBSITE_DATA_OFFLINE_APPLICATION_CACHE,
                    ) as lua_Integer,
                );
                lua_rawset(L, -(3 as std::ffi::c_int));
            }
            if present as std::ffi::c_uint
                & WEBKIT_WEBSITE_DATA_SESSION_STORAGE as std::ffi::c_int as std::ffi::c_uint
                != 0
            {
                lua_pushstring(
                    L,
                    b"session_storage\0" as *const u8 as *const std::ffi::c_char,
                );
                lua_pushinteger(
                    L,
                    webkit_website_data_get_size(website_data, WEBKIT_WEBSITE_DATA_SESSION_STORAGE)
                        as lua_Integer,
                );
                lua_rawset(L, -(3 as std::ffi::c_int));
            }
            if present as std::ffi::c_uint
                & WEBKIT_WEBSITE_DATA_LOCAL_STORAGE as std::ffi::c_int as std::ffi::c_uint
                != 0
            {
                lua_pushstring(
                    L,
                    b"local_storage\0" as *const u8 as *const std::ffi::c_char,
                );
                lua_pushinteger(
                    L,
                    webkit_website_data_get_size(website_data, WEBKIT_WEBSITE_DATA_LOCAL_STORAGE)
                        as lua_Integer,
                );
                lua_rawset(L, -(3 as std::ffi::c_int));
            }
            if present as std::ffi::c_uint
                & WEBKIT_WEBSITE_DATA_INDEXEDDB_DATABASES as std::ffi::c_int as std::ffi::c_uint
                != 0
            {
                lua_pushstring(
                    L,
                    b"indexeddb_databases\0" as *const u8 as *const std::ffi::c_char,
                );
                lua_pushinteger(
                    L,
                    webkit_website_data_get_size(
                        website_data,
                        WEBKIT_WEBSITE_DATA_INDEXEDDB_DATABASES,
                    ) as lua_Integer,
                );
                lua_rawset(L, -(3 as std::ffi::c_int));
            }
            if present as std::ffi::c_uint
                & WEBKIT_WEBSITE_DATA_PLUGIN_DATA as std::ffi::c_int as std::ffi::c_uint
                != 0
            {
                lua_pushstring(L, b"plugin_data\0" as *const u8 as *const std::ffi::c_char);
                lua_pushinteger(
                    L,
                    webkit_website_data_get_size(website_data, WEBKIT_WEBSITE_DATA_PLUGIN_DATA)
                        as lua_Integer,
                );
                lua_rawset(L, -(3 as std::ffi::c_int));
            }
            if present as std::ffi::c_uint
                & WEBKIT_WEBSITE_DATA_COOKIES as std::ffi::c_int as std::ffi::c_uint
                != 0
            {
                lua_pushstring(L, b"cookies\0" as *const u8 as *const std::ffi::c_char);
                lua_pushinteger(
                    L,
                    webkit_website_data_get_size(website_data, WEBKIT_WEBSITE_DATA_COOKIES)
                        as lua_Integer,
                );
                lua_rawset(L, -(3 as std::ffi::c_int));
            }
            if present as std::ffi::c_uint
                & WEBKIT_WEBSITE_DATA_DEVICE_ID_HASH_SALT as std::ffi::c_int as std::ffi::c_uint
                != 0
            {
                lua_pushstring(
                    L,
                    b"device_id_hash_salt\0" as *const u8 as *const std::ffi::c_char,
                );
                lua_pushinteger(
                    L,
                    webkit_website_data_get_size(
                        website_data,
                        WEBKIT_WEBSITE_DATA_DEVICE_ID_HASH_SALT,
                    ) as lua_Integer,
                );
                lua_rawset(L, -(3 as std::ffi::c_int));
            }
            if present as std::ffi::c_uint
                & WEBKIT_WEBSITE_DATA_HSTS_CACHE as std::ffi::c_int as std::ffi::c_uint
                != 0
            {
                lua_pushstring(L, b"hsts_cache\0" as *const u8 as *const std::ffi::c_char);
                lua_pushinteger(
                    L,
                    webkit_website_data_get_size(website_data, WEBKIT_WEBSITE_DATA_HSTS_CACHE)
                        as lua_Integer,
                );
                lua_rawset(L, -(3 as std::ffi::c_int));
            }
            lua_rawset(L, -(3 as std::ffi::c_int));
            webkit_website_data_unref(website_data);
            item = (*item).next;
        }
    }
    g_list_free(items);
    luaH_resume(L, lua_gettop(L));
}
unsafe extern "C-unwind" fn luaH_luakit_website_data_fetch(mut L: *mut lua_State) -> gint {
    let mut data_types: WebKitWebsiteDataTypes =
        luaH_parse_website_data_types_table(L, 1 as std::ffi::c_int);
    if data_types as std::ffi::c_uint == 0 as std::ffi::c_int as std::ffi::c_uint {
        return luaL_error(
            L,
            b"no website data types specified\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    let mut web_context: *mut WebKitWebContext = web_context_get();
    let mut data_manager: *mut WebKitWebsiteDataManager =
        webkit_web_context_get_website_data_manager(web_context);
    webkit_website_data_manager_fetch(
        data_manager,
        data_types,
        0 as *mut GCancellable,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C-unwind" fn(
                    *mut WebKitWebsiteDataManager,
                    *mut GAsyncResult,
                    *mut lua_State,
                ) -> (),
            >,
            GAsyncReadyCallback,
        >(Some(
            website_data_fetch_finish
                as unsafe extern "C-unwind" fn(
                    *mut WebKitWebsiteDataManager,
                    *mut GAsyncResult,
                    *mut lua_State,
                ) -> (),
        )),
        L as gpointer,
    );
    return luaH_yield(L);
}
unsafe extern "C-unwind" fn website_data_remove_finish(
    mut manager: *mut WebKitWebsiteDataManager,
    mut result: *mut GAsyncResult,
    mut wdrt: *mut website_data_remove_task_t,
) {
    let mut L: *mut lua_State = (*wdrt).L;
    let mut __n1: gint64 = lua_status(L) as gint64;
    let mut __n2: gint64 = 1 as std::ffi::c_int as gint64;
    if !(__n1 == __n2) {
        g_assertion_message_cmpint(
            0 as *mut gchar,
            b"clib/luakit.c\0" as *const u8 as *const std::ffi::c_char,
            593 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 27], &[std::ffi::c_char; 27]>(
                b"website_data_remove_finish\0",
            ))
            .as_ptr(),
            b"lua_status(L) == LUA_YIELD\0" as *const u8 as *const std::ffi::c_char,
            __n1 as guint64,
            b"==\0" as *const u8 as *const std::ffi::c_char,
            __n2 as guint64,
            'i' as i32 as std::ffi::c_char,
        );
    }
    let mut error: *mut GError = 0 as *mut GError;
    webkit_website_data_manager_remove_finish(manager, result, &mut error);
    if !error.is_null() {
        lua_pushnil(L);
        lua_pushstring(L, (*error).message);
        g_error_free(error);
    } else {
        lua_pushboolean(L, (0 as std::ffi::c_int == 0) as std::ffi::c_int);
    }
    g_free((*wdrt).domain as gpointer);
    g_slice_free1(
        ::core::mem::size_of::<website_data_remove_task_t>(),
        wdrt as *mut c_void,
    );
    luaH_resume(L, lua_gettop(L));
}
unsafe extern "C-unwind" fn luaH_luakit_website_data_remove_cont(
    mut manager: *mut WebKitWebsiteDataManager,
    mut result: *mut GAsyncResult,
    mut wdrt: *mut website_data_remove_task_t,
) {
    let mut L: *mut lua_State = (*wdrt).L;
    let mut __n1: gint64 = lua_status(L) as gint64;
    let mut __n2: gint64 = 1 as std::ffi::c_int as gint64;
    if !(__n1 == __n2) {
        g_assertion_message_cmpint(
            0 as *mut gchar,
            b"clib/luakit.c\0" as *const u8 as *const std::ffi::c_char,
            613 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 37], &[std::ffi::c_char; 37]>(
                b"luaH_luakit_website_data_remove_cont\0",
            ))
            .as_ptr(),
            b"lua_status(L) == LUA_YIELD\0" as *const u8 as *const std::ffi::c_char,
            __n1 as guint64,
            b"==\0" as *const u8 as *const std::ffi::c_char,
            __n2 as guint64,
            'i' as i32 as std::ffi::c_char,
        );
    }
    let mut error: *mut GError = 0 as *mut GError;
    let mut items: *mut GList =
        webkit_website_data_manager_fetch_finish(manager, result, &mut error);
    if !error.is_null() {
        lua_pushstring(L, (*error).message);
        g_error_free(error);
        g_free((*wdrt).domain as gpointer);
        g_slice_free1(
            ::core::mem::size_of::<website_data_remove_task_t>(),
            wdrt as *mut c_void,
        );
        luaL_error(
            L,
            lua_tolstring(L, -(1 as std::ffi::c_int), 0 as *mut size_t),
        );
    }
    let mut item: *mut GList = items;
    while !item.is_null() {
        let mut website_data: *mut WebKitWebsiteData = (*item).data as *mut WebKitWebsiteData;
        let mut next: *mut GList = (*item).next;
        if !(strcmp(
            webkit_website_data_get_name(website_data),
            (*wdrt).domain as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int)
        {
            webkit_website_data_unref(website_data);
            items = g_list_delete_link(items, item);
        }
        item = next;
    }
    if items.is_null() {
        g_free((*wdrt).domain as gpointer);
        g_slice_free1(
            ::core::mem::size_of::<website_data_remove_task_t>(),
            wdrt as *mut c_void,
        );
        lua_pushboolean(L, (0 as std::ffi::c_int == 0) as std::ffi::c_int);
        luaH_resume(L, 1 as std::ffi::c_int);
        return;
    }
    let mut web_context: *mut WebKitWebContext = web_context_get();
    let mut data_manager: *mut WebKitWebsiteDataManager =
        webkit_web_context_get_website_data_manager(web_context);
    webkit_website_data_manager_remove(
        data_manager,
        (*wdrt).data_types,
        items,
        0 as *mut GCancellable,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C-unwind" fn(
                    *mut WebKitWebsiteDataManager,
                    *mut GAsyncResult,
                    *mut website_data_remove_task_t,
                ) -> (),
            >,
            GAsyncReadyCallback,
        >(Some(
            website_data_remove_finish
                as unsafe extern "C-unwind" fn(
                    *mut WebKitWebsiteDataManager,
                    *mut GAsyncResult,
                    *mut website_data_remove_task_t,
                ) -> (),
        )),
        wdrt as gpointer,
    );
    item = items;
    while !item.is_null() {
        webkit_website_data_unref((*item).data as *mut WebKitWebsiteData);
        item = (*item).next;
    }
    g_list_free(items);
}
unsafe extern "C-unwind" fn luaH_luakit_website_data_remove(mut L: *mut lua_State) -> gint {
    let mut data_types: WebKitWebsiteDataTypes =
        luaH_parse_website_data_types_table(L, 1 as std::ffi::c_int);
    if data_types as std::ffi::c_uint == 0 as std::ffi::c_int as std::ffi::c_uint {
        return luaL_error(
            L,
            b"no website data types specified\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    let mut domain: *const std::ffi::c_char =
        luaL_checklstring(L, 2 as std::ffi::c_int, 0 as *mut size_t);
    let mut wdrt: *mut website_data_remove_task_t = g_slice_alloc0(::core::mem::size_of::<
        website_data_remove_task_t,
    >()) as *mut website_data_remove_task_t;
    (*wdrt).L = L;
    (*wdrt).domain = g_strdup(domain);
    (*wdrt).data_types = data_types;
    let mut web_context: *mut WebKitWebContext = web_context_get();
    let mut data_manager: *mut WebKitWebsiteDataManager =
        webkit_web_context_get_website_data_manager(web_context);
    webkit_website_data_manager_fetch(
        data_manager,
        data_types,
        0 as *mut GCancellable,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C-unwind" fn(
                    *mut WebKitWebsiteDataManager,
                    *mut GAsyncResult,
                    *mut website_data_remove_task_t,
                ) -> (),
            >,
            GAsyncReadyCallback,
        >(Some(
            luaH_luakit_website_data_remove_cont
                as unsafe extern "C-unwind" fn(
                    *mut WebKitWebsiteDataManager,
                    *mut GAsyncResult,
                    *mut website_data_remove_task_t,
                ) -> (),
        )),
        wdrt as gpointer,
    );
    return luaH_yield(L);
}
unsafe extern "C-unwind" fn website_data_clear_finish(
    mut manager: *mut WebKitWebsiteDataManager,
    mut result: *mut GAsyncResult,
    mut L: *mut lua_State,
) {
    let mut __n1: gint64 = lua_status(L) as gint64;
    let mut __n2: gint64 = 1 as std::ffi::c_int as gint64;
    if !(__n1 == __n2) {
        g_assertion_message_cmpint(
            0 as *mut gchar,
            b"clib/luakit.c\0" as *const u8 as *const std::ffi::c_char,
            681 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 26], &[std::ffi::c_char; 26]>(
                b"website_data_clear_finish\0",
            ))
            .as_ptr(),
            b"lua_status(L) == LUA_YIELD\0" as *const u8 as *const std::ffi::c_char,
            __n1 as guint64,
            b"==\0" as *const u8 as *const std::ffi::c_char,
            __n2 as guint64,
            'i' as i32 as std::ffi::c_char,
        );
    }
    let mut error: *mut GError = 0 as *mut GError;
    webkit_website_data_manager_clear_finish(manager, result, &mut error);
    if !error.is_null() {
        lua_pushnil(L);
        lua_pushstring(L, (*error).message);
        g_error_free(error);
    } else {
        lua_pushboolean(L, (0 as std::ffi::c_int == 0) as std::ffi::c_int);
    }
    luaH_resume(L, lua_gettop(L));
}
unsafe extern "C-unwind" fn luaH_luakit_website_data_clear(mut L: *mut lua_State) -> gint {
    let mut data_types: WebKitWebsiteDataTypes =
        luaH_parse_website_data_types_table(L, 1 as std::ffi::c_int);
    if data_types as std::ffi::c_uint == 0 as std::ffi::c_int as std::ffi::c_uint {
        return luaL_error(
            L,
            b"no website data types specified\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    let mut timespan: GTimeSpan =
        luaL_optinteger(L, 2 as std::ffi::c_int, 0 as std::ffi::c_int as lua_Integer);
    let mut web_context: *mut WebKitWebContext = web_context_get();
    let mut data_manager: *mut WebKitWebsiteDataManager =
        webkit_web_context_get_website_data_manager(web_context);
    webkit_website_data_manager_clear(
        data_manager,
        data_types,
        timespan,
        0 as *mut GCancellable,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C-unwind" fn(
                    *mut WebKitWebsiteDataManager,
                    *mut GAsyncResult,
                    *mut lua_State,
                ) -> (),
            >,
            GAsyncReadyCallback,
        >(Some(
            website_data_clear_finish
                as unsafe extern "C-unwind" fn(
                    *mut WebKitWebsiteDataManager,
                    *mut GAsyncResult,
                    *mut lua_State,
                ) -> (),
        )),
        L as gpointer,
    );
    return luaH_yield(L);
}
unsafe extern "C-unwind" fn luaH_luakit_website_data_index(mut L: *mut lua_State) -> gint {
    let mut prop: *const gchar = luaL_checklstring(L, 2 as std::ffi::c_int, 0 as *mut size_t);
    let mut token: luakit_token_t = l_tokenize(prop);
    match token as std::ffi::c_uint {
        93 => {
            lua_pushcclosure(L, luaH_luakit_website_data_fetch, 0);
            luaH_yield_wrap_function(L);
            return 1 as std::ffi::c_int;
        }
        180 => {
            lua_pushcclosure(L, luaH_luakit_website_data_remove, 0);
            luaH_yield_wrap_function(L);
            return 1 as std::ffi::c_int;
        }
        26 => {
            lua_pushcclosure(L, luaH_luakit_website_data_clear, 0);
            luaH_yield_wrap_function(L);
            return 1 as std::ffi::c_int;
        }
        _ => return 0 as std::ffi::c_int,
    };
}
unsafe extern "C-unwind" fn luaH_luakit_push_website_data_table(mut L: *mut lua_State) -> gint {
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_createtable(L, 0 as std::ffi::c_int, 2 as std::ffi::c_int);
    lua_pushlstring(
        L,
        b"__index\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 8]>())
            .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
            .wrapping_sub(1),
    );
    lua_pushvalue(L, 1 as std::ffi::c_int);
    lua_pushcclosure(L, luaH_luakit_website_data_index, 1);
    lua_rawset(L, -(3 as std::ffi::c_int));
    lua_setmetatable(L, -(2 as std::ffi::c_int));
    return 1 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_string_wch_convert_case(
    mut L: *mut lua_State,
    mut key: *const std::ffi::c_char,
    mut upper: gboolean,
) -> gint {
    let mut kval: guint = gdk_keyval_from_name(key);
    if kval == 0xffffff as std::ffi::c_int as guint {
        _log(
            LOG_LEVEL_debug,
            b"clib/luakit.c\0" as *const u8 as *const std::ffi::c_char,
            b"unrecognized key symbol '%s'\0" as *const u8 as *const std::ffi::c_char,
            key,
        );
        lua_pushstring(L, key);
        return 1 as std::ffi::c_int;
    }
    let mut cased: guint = 0;
    gdk_keyval_convert_case(
        kval,
        if upper != 0 {
            0 as *mut guint
        } else {
            &mut cased
        },
        if upper != 0 {
            &mut cased
        } else {
            0 as *mut guint
        },
    );
    luaH_keystr_push(L, cased);
    return 1 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_luakit_wch_lower(mut L: *mut lua_State) -> gint {
    return luaH_string_wch_convert_case(
        L,
        luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t),
        0 as std::ffi::c_int,
    );
}
unsafe extern "C-unwind" fn luaH_luakit_wch_upper(mut L: *mut lua_State) -> gint {
    return luaH_string_wch_convert_case(
        L,
        luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t),
        (0 as std::ffi::c_int == 0) as std::ffi::c_int,
    );
}
unsafe extern "C-unwind" fn luaH_luakit_clear_favicon_database(
    mut UNUSED_L: *mut lua_State,
) -> gint {
    let mut ctx: *mut WebKitWebContext = web_context_get();
    let mut fdb: *mut WebKitFaviconDatabase = webkit_web_context_get_favicon_database(ctx);
    webkit_favicon_database_clear(fdb);
    return 0 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_luakit_push_install_paths_table(mut L: *mut lua_State) -> gint {
    lua_createtable(L, 0 as std::ffi::c_int, 6 as std::ffi::c_int);
    lua_pushlstring(
        L,
        b"/usr/local/share/luakit\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 24]>())
            .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
            .wrapping_sub(1),
    );
    lua_setfield(
        L,
        -(2 as std::ffi::c_int),
        b"install_dir\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_pushlstring(
        L,
        b"/etc/xdg\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 9]>())
            .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
            .wrapping_sub(1),
    );
    lua_setfield(
        L,
        -(2 as std::ffi::c_int),
        b"config_dir\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_pushlstring(
        L,
        b"/usr/local/share/luakit/doc\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 28]>())
            .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
            .wrapping_sub(1),
    );
    lua_setfield(
        L,
        -(2 as std::ffi::c_int),
        b"doc_dir\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_pushlstring(
        L,
        b"/usr/local/share/man\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 21]>())
            .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
            .wrapping_sub(1),
    );
    lua_setfield(
        L,
        -(2 as std::ffi::c_int),
        b"man_dir\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_pushlstring(
        L,
        b"/usr/local/share/pixmaps\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 25]>())
            .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
            .wrapping_sub(1),
    );
    lua_setfield(
        L,
        -(2 as std::ffi::c_int),
        b"pixmap_dir\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_pushlstring(
        L,
        b"/usr/local/share/applications\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 30]>())
            .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
            .wrapping_sub(1),
    );
    lua_setfield(
        L,
        -(2 as std::ffi::c_int),
        b"app_dir\0" as *const u8 as *const std::ffi::c_char,
    );
    return 1 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_luakit_index(mut L: *mut lua_State) -> gint {
    if luaH_usemetatable(L, 1 as std::ffi::c_int, 2 as std::ffi::c_int) != 0 {
        return 1 as std::ffi::c_int;
    }
    let mut w: *mut widget_t = 0 as *mut widget_t;
    let mut prop: *const gchar = luaL_checklstring(L, 2 as std::ffi::c_int, 0 as *mut size_t);
    let mut token: luakit_token_t = l_tokenize(prop);
    match token as std::ffi::c_uint {
        16 => {
            lua_pushstring(L, globalconf.cache_dir);
            return 1 as std::ffi::c_int;
        }
        32 => {
            lua_pushstring(L, globalconf.config_dir);
            return 1 as std::ffi::c_int;
        }
        42 => {
            lua_pushstring(L, globalconf.data_dir);
            return 1 as std::ffi::c_int;
        }
        91 => {
            lua_pushstring(L, globalconf.execpath);
            return 1 as std::ffi::c_int;
        }
        33 => {
            lua_pushstring(L, globalconf.confpath);
            return 1 as std::ffi::c_int;
        }
        184 => {
            lua_pushstring(L, resource_path_get());
            return 1 as std::ffi::c_int;
        }
        250 => {
            lua_pushboolean(
                L,
                (log_get_verbosity(
                    b"all\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char,
                ) as std::ffi::c_uint
                    >= LOG_LEVEL_verbose as std::ffi::c_int as std::ffi::c_uint)
                    as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        155 => {
            lua_pushboolean(L, globalconf.nounique);
            return 1 as std::ffi::c_int;
        }
        80 => {
            lua_pushboolean(
                L,
                webkit_web_context_get_spell_checking_enabled(web_context_get()),
            );
            return 1 as std::ffi::c_int;
        }
        172 => {
            lua_pushinteger(L, web_context_process_limit_get() as lua_Integer);
            return 1 as std::ffi::c_int;
        }
        156 => return luaH_luakit_push_options_table(L),
        259 => return luaH_luakit_push_website_data_table(L),
        256 => {
            lua_pushboolean(L, 1 as std::ffi::c_int);
            return 1 as std::ffi::c_int;
        }
        264 => {
            lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
            let mut i = 0;
            while i < (*globalconf.windows).len {
                w = *((*globalconf.windows).pdata).offset(i as isize) as *mut widget_t;
                luaH_object_push(L, (*w).ref_0);
                lua_rawseti(L, -(2 as std::ffi::c_int), i.wrapping_add(1) as i64);
                i = i.wrapping_add(1);
                i;
            }
            return 1 as std::ffi::c_int;
        }
        258 => {
            lua_pushfstring(
                L,
                b"%d.%d.%d\0" as *const u8 as *const std::ffi::c_char,
                2 as std::ffi::c_int,
                48 as std::ffi::c_int,
                3 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        257 => {
            lua_pushfstring(
                L,
                b"%d.%d\0" as *const u8 as *const std::ffi::c_char,
                2 as std::ffi::c_int,
                48 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        201 => return luaH_luakit_selection_table_push(L),
        125 => {
            _log(
                LOG_LEVEL_warn,
                b"clib/luakit.c\0" as *const u8 as *const std::ffi::c_char,
                b"luakit.install_path is deprecated: use luakit.install_paths.install_dir instead\0"
                    as *const u8 as *const std::ffi::c_char,
            );
            lua_pushlstring(
                L,
                b"/usr/local/share/luakit\0" as *const u8 as *const std::ffi::c_char,
                (::core::mem::size_of::<[std::ffi::c_char; 24]>())
                    .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
                    .wrapping_sub(1),
            );
            return 1 as std::ffi::c_int;
        }
        126 => return luaH_luakit_push_install_paths_table(L),
        251 => {
            lua_pushlstring(
                L,
                b"64175ca2\0" as *const u8 as *const std::ffi::c_char,
                (::core::mem::size_of::<[std::ffi::c_char; 9]>())
                    .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
                    .wrapping_sub(1),
            );
            return 1 as std::ffi::c_int;
        }
        51 => {
            lua_pushboolean(L, 0 as std::ffi::c_int);
            return 1 as std::ffi::c_int;
        }
        219 => {
            luaH_push_strv(
                L,
                webkit_web_context_get_spell_checking_languages(web_context_get()),
            );
            return 1 as std::ffi::c_int;
        }
        _ => {}
    }
    return 0 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_luakit_newindex(mut L: *mut lua_State) -> gint {
    if lua_isstring(L, 2 as std::ffi::c_int) == 0 {
        return 0 as std::ffi::c_int;
    }
    let mut token: luakit_token_t =
        l_tokenize(lua_tolstring(L, 2 as std::ffi::c_int, 0 as *mut size_t));
    match token as std::ffi::c_uint {
        172 => {
            if web_context_process_limit_set(lua_tointeger(L, 3 as std::ffi::c_int) as guint) == 0 {
                return luaL_error(
                    L,
                    b"Too late to set WebKit process limit\0" as *const u8
                        as *const std::ffi::c_char,
                );
            }
        }
        80 => {
            webkit_web_context_set_spell_checking_enabled(
                web_context_get(),
                luaH_checkboolean(L, 3 as std::ffi::c_int),
            );
        }
        219 => {
            let mut langs: *mut *const gchar = luaH_checkstrv(L, 3 as std::ffi::c_int);
            let mut ctx: *mut WebKitWebContext = web_context_get();
            webkit_web_context_set_spell_checking_languages(ctx, langs);
            let mut accepted: *const *const gchar =
                webkit_web_context_get_spell_checking_languages(ctx);
            let mut lang: *mut *const gchar = langs;
            while !(*lang).is_null() {
                if g_strv_contains(accepted, *lang) == 0 {
                    _log(
                        LOG_LEVEL_warn,
                        b"clib/luakit.c\0" as *const u8 as *const std::ffi::c_char,
                        b"unrecognized language code '%s'\0" as *const u8
                            as *const std::ffi::c_char,
                        *lang,
                    );
                }
                lang = lang.offset(1);
                lang;
            }
            g_free(langs as gpointer);
        }
        184 => {
            resource_path_set(luaL_checklstring(L, 3 as std::ffi::c_int, 0 as *mut size_t));
        }
        _ => return 0 as std::ffi::c_int,
    }
    return 0 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_luakit_quit(mut UNUSED_L: *mut lua_State) -> gint {
    if gtk_main_level() != 0 {
        gtk_main_quit();
    } else {
        exit(0 as std::ffi::c_int);
    }
    return 0 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_luakit_register_scheme(mut L: *mut lua_State) -> gint {
    let mut scheme: *const gchar = luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t);
    if strcmp(
        scheme as *const std::ffi::c_char,
        b"\0" as *const u8 as *const std::ffi::c_char,
    ) == 0 as std::ffi::c_int
    {
        return luaL_error(
            L,
            b"scheme cannot be empty\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    if strcmp(
        scheme as *const std::ffi::c_char,
        b"http\0" as *const u8 as *const std::ffi::c_char,
    ) == 0 as std::ffi::c_int
        || strcmp(
            scheme as *const std::ffi::c_char,
            b"https\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
    {
        return luaL_error(
            L,
            b"scheme cannot be 'http' or 'https'\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    if g_regex_match_simple(
        b"^[a-z][a-z0-9\\+\\-\\.]*$\0" as *const u8 as *const std::ffi::c_char,
        scheme,
        G_REGEX_DEFAULT,
        G_REGEX_MATCH_DEFAULT,
    ) == 0
    {
        return luaL_error(
            L,
            b"scheme must match [a-z][a-z0-9\\+\\-\\.]*\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    webkit_web_context_register_uri_scheme(
        web_context_get(),
        scheme,
        ::core::mem::transmute::<
            Option<unsafe extern "C-unwind" fn(*mut WebKitURISchemeRequest, gpointer) -> ()>,
            WebKitURISchemeRequestCallback,
        >(Some(
            luakit_uri_scheme_request_cb
                as unsafe extern "C-unwind" fn(*mut WebKitURISchemeRequest, gpointer) -> (),
        )),
        g_strdup(scheme) as gpointer,
        Some(g_free),
    );
    return 0 as std::ffi::c_int;
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_luakit_allow_certificate(mut L: *mut lua_State) -> gint {
    let mut host: *const gchar = luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t);
    let mut len: size_t = 0;
    let mut cert_pem: *const gchar = luaL_checklstring(L, 2 as std::ffi::c_int, &mut len);
    let mut err: *mut GError = 0 as *mut GError;
    let mut cert: *mut GTlsCertificate =
        g_tls_certificate_new_from_pem(cert_pem, len as ssize_t, &mut err);
    if !err.is_null() {
        lua_pushnil(L);
        lua_pushstring(L, (*err).message);
        return 2 as std::ffi::c_int;
    }
    let mut ctx: *mut WebKitWebContext = web_context_get();
    webkit_web_context_allow_tls_certificate_for_host(ctx, cert, host);
    g_object_unref(g_type_check_instance_cast(
        cert as *mut GTypeInstance,
        ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
    ) as *mut std::ffi::c_void as *mut GObject);
    lua_pushboolean(L, (0 as std::ffi::c_int == 0) as std::ffi::c_int);
    return 1 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_class_index_miss_property(
    mut L: *mut lua_State,
    mut UNUSED_obj: *mut lua_object_t,
) -> gint {
    signal_object_emit(
        L,
        luakit_class.signals,
        b"debug::index::miss\0" as *const u8 as *const std::ffi::c_char,
        2 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_class_newindex_miss_property(
    mut L: *mut lua_State,
    mut UNUSED_obj: *mut lua_object_t,
) -> gint {
    signal_object_emit(
        L,
        luakit_class.signals,
        b"debug::newindex::miss\0" as *const u8 as *const std::ffi::c_char,
        3 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luakit_lib_setup(mut L: *mut lua_State) {
    static mut luakit_lib: [luaL_Reg; 20] = unsafe {
        [
            {
                let mut init = luaL_Reg {
                    name: b"add_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_luakit_class_add_signal,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"remove_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_luakit_class_remove_signal,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"emit_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_luakit_class_emit_signal,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"time\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_luakit_time,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"uri_encode\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_luakit_uri_encode,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"uri_decode\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_luakit_uri_decode,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"idle_add\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_luakit_idle_add,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"idle_remove\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_luakit_idle_remove,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__index\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_luakit_index,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__newindex\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_luakit_newindex,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"exec\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_luakit_exec,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"quit\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_luakit_quit,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"save_file\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_luakit_save_file,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"spawn\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_luakit_spawn,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"spawn_sync\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_luakit_spawn_sync,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"register_scheme\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_luakit_register_scheme,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"allow_certificate\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_luakit_allow_certificate,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"wch_lower\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_luakit_wch_lower,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"wch_upper\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_luakit_wch_upper,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"clear_favicon_database\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_luakit_clear_favicon_database,
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
    luakit_class.signals = signal_new();
    luaH_openlib(
        L,
        b"luakit\0" as *const u8 as *const std::ffi::c_char,
        luakit_lib.as_ptr(),
        luakit_lib.as_ptr(),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luakit_lib_get_luakit_class() -> *mut lua_class_t {
    return &mut luakit_class;
}
