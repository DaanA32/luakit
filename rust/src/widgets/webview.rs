use core::intrinsics::AtomicOrdering;
use gdk_sys::*;
use glib_sys::*;
use libc::*;
use mlua_sys::*;

use gobject_sys::*;

use crate::clib::luakit::*;
use crate::clib::msg::*;
use crate::clib::soup::*;
use crate::clib::sqlite3::*;
use crate::clib::stylesheet::*;
use crate::clib::web_module::*;
use crate::clib::widget::*;
use crate::common::luaclass::luaH_typename;
use crate::common::luah::*;
use crate::common::luaobject::*;
use crate::common::luaserialize::*;
use crate::common::luautil::*;
use crate::common::util::*;
use crate::common::*;
use crate::globalconf::*;
use crate::gtypes::*;
use crate::ipc::ipc_h::*;
use crate::ipc::*;
use crate::log::*;

    pub unsafe extern "C" fn luaH_webview_push_history(
        mut L: *mut lua_State,
        mut view: *mut WebKitWebView,
    ) -> gint {
        let mut bflist = webkit_web_view_get_back_forward_list(view);
        let mut item = 0 as *mut WebKitBackForwardListItem;
        let mut backlen = g_list_length(webkit_back_forward_list_get_back_list(bflist))
            as gint;
        let mut forwardlen = g_list_length(
            webkit_back_forward_list_get_forward_list(bflist),
        ) as gint;
        lua_createtable(L, 0 as std::ffi::c_int, 2 as std::ffi::c_int);
        lua_pushlstring(
            L,
            b"index\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 6]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        lua_pushnumber(L, (backlen + 1 as std::ffi::c_int) as lua_Number);
        lua_rawset(L, -(3 as std::ffi::c_int));
        lua_createtable(
            L,
            backlen + forwardlen + 1 as std::ffi::c_int,
            0 as std::ffi::c_int,
        );
        let mut i = -backlen;
        while i <= forwardlen {
            item = webkit_back_forward_list_get_nth_item(bflist, i);
            lua_createtable(L, 0 as std::ffi::c_int, 2 as std::ffi::c_int);
            lua_pushlstring(
                L,
                b"uri\0" as *const u8 as *const std::ffi::c_char,
                (::core::mem::size_of::<[std::ffi::c_char; 4]>() as std::ffi::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                    )
                    .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
            );
            lua_pushstring(
                L,
                if !item.is_null() {
                    webkit_back_forward_list_item_get_uri(item)
                } else {
                    b"about:blank\0" as *const u8 as *const std::ffi::c_char
                },
            );
            lua_rawset(L, -(3 as std::ffi::c_int));
            lua_pushlstring(
                L,
                b"title\0" as *const u8 as *const std::ffi::c_char,
                (::core::mem::size_of::<[std::ffi::c_char; 6]>() as std::ffi::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                    )
                    .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
            );
            lua_pushstring(
                L,
                if !item.is_null() {
                    webkit_back_forward_list_item_get_title(item)
                } else {
                    b"\0" as *const u8 as *const std::ffi::c_char
                },
            );
            lua_rawset(L, -(3 as std::ffi::c_int));
            lua_rawseti(L, -(2 as std::ffi::c_int), backlen + i + 1 as std::ffi::c_int);
            i += 1;
            i;
        }
        lua_pushlstring(
            L,
            b"items\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 6]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        lua_insert(L, lua_gettop(L) - 1 as std::ffi::c_int);
        lua_rawset(L, -(3 as std::ffi::c_int));
        return 1 as std::ffi::c_int;
    }
    pub unsafe extern "C" fn luaH_webview_can_go_back(mut L: *mut lua_State) -> gint {
        let mut d = (*luaH_checkwebview(L, 1 as std::ffi::c_int)).data
            as *mut webview_data_t;
        lua_pushboolean(L, webkit_web_view_can_go_back((*d).view));
        return 1 as std::ffi::c_int;
    }
    pub unsafe extern "C" fn luaH_webview_can_go_forward(mut L: *mut lua_State) -> gint {
        let mut d = (*luaH_checkwebview(L, 1 as std::ffi::c_int)).data
            as *mut webview_data_t;
        lua_pushboolean(L, webkit_web_view_can_go_forward((*d).view));
        return 1 as std::ffi::c_int;
    }
    pub unsafe extern "C" fn webview_history_go(
        mut L: *mut lua_State,
        mut direction: gint,
    ) -> gint {
        let mut d = (*luaH_checkwebview(L, 1 as std::ffi::c_int)).data
            as *mut webview_data_t;
        let mut steps = luaL_checknumber(L, 2 as std::ffi::c_int) as gint * direction;
        let mut item = webkit_back_forward_list_get_nth_item(
            webkit_web_view_get_back_forward_list((*d).view),
            steps,
        );
        if !item.is_null() {
            webkit_web_view_go_to_back_forward_list_item((*d).view, item);
        }
        lua_pushboolean(
            L,
            (item != NULL_0 as *mut WebKitBackForwardListItem) as std::ffi::c_int,
        );
        return 1 as std::ffi::c_int;
    }
    pub unsafe extern "C" fn luaH_webview_go_back(mut L: *mut lua_State) -> gint {
        return webview_history_go(L, -(1 as std::ffi::c_int));
    }
    pub unsafe extern "C" fn luaH_webview_go_forward(mut L: *mut lua_State) -> gint {
        return webview_history_go(L, 1 as std::ffi::c_int);
    }
    pub unsafe extern "C" fn luaH_webview_set_session_state(
        mut L: *mut lua_State,
        mut d: *mut webview_data_t,
    ) {
        let mut len: size_t = 0;
        let mut str = lua_tolstring(L, 3 as std::ffi::c_int, &mut len);
        let mut bytes = g_bytes_new(str as gconstpointer, len);
        let mut state = webkit_web_view_session_state_new(bytes);
        g_bytes_unref(bytes);
        if state.is_null() {
            luaL_error(
                L,
                b"Invalid session state\0" as *const u8 as *const std::ffi::c_char,
            );
        }
        webkit_web_view_restore_session_state((*d).view, state);
        webkit_web_view_session_state_unref(state);
        let mut bfl = webkit_web_view_get_back_forward_list((*d).view);
        let mut item = webkit_back_forward_list_get_current_item(bfl);
        if !item.is_null() {
            webkit_web_view_go_to_back_forward_list_item((*d).view, item);
            update_uri((*d).widget, webkit_back_forward_list_item_get_uri(item));
        }
    }
    pub unsafe extern "C" fn luaH_webview_push_session_state(
        mut L: *mut lua_State,
        mut d: *mut webview_data_t,
    ) -> std::ffi::c_int {
        let mut state = webkit_web_view_get_session_state((*d).view);
        let mut bytes = webkit_web_view_session_state_serialize(state);
        let mut len: gsize = 0;
        let mut str = g_bytes_get_data(bytes, &mut len) as *const gchar;
        lua_pushlstring(L, str, len);
        g_bytes_unref(bytes);
        webkit_web_view_session_state_unref(state);
        return 1 as std::ffi::c_int;
    }
pub mod scroll_c {
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn webview_scroll_recv(
        mut w: *mut widget_t,
        mut msg: *const ipc_scroll_t,
    ) {
        let mut d = (*w).data as *mut webview_data_t;
        if webkit_web_view_get_page_id((*d).view) != (*msg).page_id {
            return;
        }
        match (*msg).subtype as std::ffi::c_uint {
            0 => {
                (*d).doc_w = (*msg).h;
                (*d).doc_h = (*msg).v;
            }
            1 => {
                (*d).win_w = (*msg).h;
                (*d).win_h = (*msg).v;
            }
            2 => {
                (*d).scroll_x = (*msg).h;
                (*d).scroll_y = (*msg).v;
            }
            _ => {}
        };
    }
    pub unsafe extern "C" fn luaH_webview_scroll_newindex(
        mut L: *mut lua_State,
    ) -> gint {
        let mut d = (*luaH_checkwebview(
            L,
            -(10002 as std::ffi::c_int) - 1 as std::ffi::c_int,
        ))
            .data as *mut webview_data_t;
        let mut prop = luaL_checklstring(L, 2 as std::ffi::c_int, NULL_0 as *mut size_t);
        let mut t = l_tokenize(prop);
        if t as std::ffi::c_uint == L_TK_X as std::ffi::c_int as std::ffi::c_uint {
            (*d).scroll_x = luaL_checknumber(L, 3 as std::ffi::c_int) as gint;
        } else if t as std::ffi::c_uint == L_TK_Y as std::ffi::c_int as std::ffi::c_uint
        {
            (*d).scroll_y = luaL_checknumber(L, 3 as std::ffi::c_int) as gint;
        } else {
            return 0 as std::ffi::c_int
        }
        lua_pushinteger(L, webkit_web_view_get_page_id((*d).view) as lua_Integer);
        lua_pushinteger(L, (*d).scroll_x as lua_Integer);
        lua_pushinteger(L, (*d).scroll_y as lua_Integer);
        ipc_send_lua(
            (*d).ipc,
            IPC_TYPE_scroll,
            L,
            4 as std::ffi::c_int,
            6 as std::ffi::c_int,
        );
        return 0 as std::ffi::c_int;
    }
    pub unsafe extern "C" fn luaH_webview_scroll_index(mut L: *mut lua_State) -> gint {
        let mut d = (*luaH_checkwebview(
            L,
            -(10002 as std::ffi::c_int) - 1 as std::ffi::c_int,
        ))
            .data as *mut webview_data_t;
        let mut prop = luaL_checklstring(L, 2 as std::ffi::c_int, NULL_0 as *mut size_t);
        let mut t = l_tokenize(prop);
        match t as std::ffi::c_uint {
            267 => {
                lua_pushnumber(L, (*d).scroll_x as lua_Number);
                return 1 as std::ffi::c_int;
            }
            270 => {
                lua_pushnumber(L, (*d).scroll_y as lua_Number);
                return 1 as std::ffi::c_int;
            }
            268 => {
                lua_pushnumber(L, ((*d).doc_w - (*d).win_w) as lua_Number);
                return 1 as std::ffi::c_int;
            }
            271 => {
                lua_pushnumber(L, ((*d).doc_h - (*d).win_h) as lua_Number);
                return 1 as std::ffi::c_int;
            }
            269 => {
                lua_pushnumber(L, (*d).win_w as lua_Number);
                return 1 as std::ffi::c_int;
            }
            272 => {
                lua_pushnumber(L, (*d).win_h as lua_Number);
                return 1 as std::ffi::c_int;
            }
            _ => return 0 as std::ffi::c_int,
        };
    }
    pub unsafe extern "C" fn luaH_webview_push_scroll_table(
        mut L: *mut lua_State,
    ) -> gint {
        lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
        lua_createtable(L, 0 as std::ffi::c_int, 2 as std::ffi::c_int);
        lua_pushlstring(
            L,
            b"__index\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 8]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        lua_pushvalue(L, 1 as std::ffi::c_int);
        lua_pushcclosure(
            L,
            Some(
                luaH_webview_scroll_index as unsafe extern "C" fn(*mut lua_State) -> gint,
            ),
            1 as std::ffi::c_int,
        );
        lua_rawset(L, -(3 as std::ffi::c_int));
        lua_pushlstring(
            L,
            b"__newindex\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 11]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        lua_pushvalue(L, 1 as std::ffi::c_int);
        lua_pushcclosure(
            L,
            Some(
                luaH_webview_scroll_newindex
                    as unsafe extern "C" fn(*mut lua_State) -> gint,
            ),
            1 as std::ffi::c_int,
        );
        lua_rawset(L, -(3 as std::ffi::c_int));
        lua_setmetatable(L, -(2 as std::ffi::c_int));
        return 1 as std::ffi::c_int;
    }

}
pub mod inspector_c {
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn inspector_open_window_cb(
        mut UNUSED_inspector: *mut WebKitWebInspector,
        mut w: *mut widget_t,
    ) -> gboolean {
        let mut L = common.L;
        luaH_object_push(L, (*w).ref_0);
        let mut nret = luaH_object_emit_signal(
            L,
            -(1 as std::ffi::c_int),
            b"create-inspector-window\0" as *const u8 as *const std::ffi::c_char,
            0 as std::ffi::c_int,
            1 as std::ffi::c_int,
        );
        let mut ret = (nret != 0 && lua_toboolean(L, -(1 as std::ffi::c_int)) != 0)
            as std::ffi::c_int;
        lua_settop(L, -(1 as std::ffi::c_int + nret) - 1 as std::ffi::c_int);
        return ret;
    }
    pub unsafe extern "C" fn inspector_show_window_cb(
        mut UNUSED_inspector: *mut WebKitWebInspector,
        mut w: *mut widget_t,
    ) -> gboolean {
        let mut L = common.L;
        luaH_object_push(L, (*w).ref_0);
        let mut nret = luaH_object_emit_signal(
            L,
            -(1 as std::ffi::c_int),
            b"show-inspector\0" as *const u8 as *const std::ffi::c_char,
            0 as std::ffi::c_int,
            1 as std::ffi::c_int,
        );
        let mut ret = (nret != 0 && lua_toboolean(L, -(1 as std::ffi::c_int)) != 0)
            as std::ffi::c_int;
        lua_settop(L, -(1 as std::ffi::c_int + nret) - 1 as std::ffi::c_int);
        return ret;
    }
    pub unsafe extern "C" fn inspector_close_window_cb(
        mut UNUSED_inspector: *mut WebKitWebInspector,
        mut w: *mut widget_t,
    ) -> gboolean {
        let mut L = common.L;
        luaH_object_push(L, (*w).ref_0);
        let mut d = (*w).data as *mut webview_data_t;
        lua_pushnil(L);
        (*d).inspector_open = FALSE;
        let mut nret = luaH_object_emit_signal(
            L,
            -(2 as std::ffi::c_int),
            b"close-inspector\0" as *const u8 as *const std::ffi::c_char,
            1 as std::ffi::c_int,
            0 as std::ffi::c_int,
        );
        let mut ret = (nret != 0 && lua_toboolean(L, -(1 as std::ffi::c_int)) != 0)
            as std::ffi::c_int;
        lua_settop(L, -(1 as std::ffi::c_int + nret) - 1 as std::ffi::c_int);
        return ret;
    }
    pub unsafe extern "C" fn inspector_attach_window_cb(
        mut UNUSED_inspector: *mut WebKitWebInspector,
        mut w: *mut widget_t,
    ) -> gboolean {
        let mut L = common.L;
        let mut d = (*w).data as *mut webview_data_t;
        (*d).inspector_open = TRUE;
        luaH_object_push(L, (*w).ref_0);
        let mut nret = luaH_object_emit_signal(
            L,
            -(1 as std::ffi::c_int),
            b"attach-inspector\0" as *const u8 as *const std::ffi::c_char,
            0 as std::ffi::c_int,
            0 as std::ffi::c_int,
        );
        let mut ret = (nret != 0 && lua_toboolean(L, -(1 as std::ffi::c_int)) != 0)
            as std::ffi::c_int;
        lua_settop(L, -(1 as std::ffi::c_int + nret) - 1 as std::ffi::c_int);
        return ret;
    }
    pub unsafe extern "C" fn inspector_detach_window_cb(
        mut UNUSED_inspector: *mut WebKitWebInspector,
        mut w: *mut widget_t,
    ) -> gboolean {
        let mut L = common.L;
        luaH_object_push(L, (*w).ref_0);
        let mut nret = luaH_object_emit_signal(
            L,
            -(1 as std::ffi::c_int),
            b"detach-inspector\0" as *const u8 as *const std::ffi::c_char,
            0 as std::ffi::c_int,
            0 as std::ffi::c_int,
        );
        let mut ret = (nret != 0 && lua_toboolean(L, -(1 as std::ffi::c_int)) != 0)
            as std::ffi::c_int;
        lua_settop(L, -(1 as std::ffi::c_int + nret) - 1 as std::ffi::c_int);
        return ret;
    }
    pub unsafe extern "C" fn luaH_webview_show_inspector(mut L: *mut lua_State) -> gint {
        let mut d = (*luaH_checkwebview(L, 1 as std::ffi::c_int)).data
            as *mut webview_data_t;
        webkit_web_inspector_show((*d).inspector);
        return 0 as std::ffi::c_int;
    }
    pub unsafe extern "C" fn luaH_webview_close_inspector(
        mut L: *mut lua_State,
    ) -> gint {
        let mut d = (*luaH_checkwebview(L, 1 as std::ffi::c_int)).data
            as *mut webview_data_t;
        webkit_web_inspector_close((*d).inspector);
        return 0 as std::ffi::c_int;
    }

}
pub mod find_controller_c {
    pub unsafe extern "C" fn found_text_cb(
        mut UNUSED_find_controller: *mut WebKitFindController,
        mut match_count: guint,
        mut w: *mut widget_t,
    ) {
        let mut L = common.L;
        luaH_object_push(L, (*w).ref_0);
        lua_pushinteger(L, match_count as lua_Integer);
        luaH_object_emit_signal(
            L,
            -(2 as std::ffi::c_int),
            b"found-text\0" as *const u8 as *const std::ffi::c_char,
            1 as std::ffi::c_int,
            0 as std::ffi::c_int,
        );
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    }
    pub unsafe extern "C" fn failed_to_find_text_cb(
        mut UNUSED_find_controller: *mut WebKitFindController,
        mut w: *mut widget_t,
    ) {
        let mut L = common.L;
        luaH_object_push(L, (*w).ref_0);
        luaH_object_emit_signal(
            L,
            -(1 as std::ffi::c_int),
            b"failed-to-find-text\0" as *const u8 as *const std::ffi::c_char,
            0 as std::ffi::c_int,
            0 as std::ffi::c_int,
        );
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    }
}
pub mod stylesheets_c {
    pub static mut inside_stylesheet_cb: gboolean = FALSE;
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn webview_stylesheets_regenerate_stylesheet(
        mut w: *mut widget_t,
        mut stylesheet: *mut lstylesheet_t,
    ) {
        let mut d = (*w).data as *mut webview_data_t;
        if !(g_list_find((*d).stylesheets, stylesheet as gconstpointer)).is_null() {
            (*d).stylesheet_refreshed = TRUE;
        }
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn webview_stylesheets_regenerate(mut w: *mut widget_t) {
        let mut d = (*w).data as *mut webview_data_t;
        if (*d).stylesheet_added != 0 || (*d).stylesheet_removed != 0
            || (*d).stylesheet_refreshed != 0
        {
            webkit_user_content_manager_remove_all_style_sheets((*d).user_content);
            let mut l = 0 as *mut GList;
            l = (*d).stylesheets;
            while !l.is_null() {
                let mut stylesheet = (*l).data as *mut lstylesheet_t;
                webkit_user_content_manager_add_style_sheet(
                    (*d).user_content,
                    (*stylesheet).stylesheet,
                );
                l = (*l).next;
            }
            (*d).stylesheet_added = FALSE;
            (*d).stylesheet_removed = FALSE;
        }
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn webview_stylesheet_set_enabled(
        mut w: *mut widget_t,
        mut stylesheet: *mut lstylesheet_t,
        mut enable: gboolean,
    ) -> std::ffi::c_int {
        let mut d = (*w).data as *mut webview_data_t;
        let mut item = g_list_find((*d).stylesheets, stylesheet as gconstpointer);
        if enable == (item != NULL_0 as *mut GList) as std::ffi::c_int {
            return 0 as std::ffi::c_int;
        }
        if enable != 0 {
            (*d).stylesheets = g_list_prepend((*d).stylesheets, stylesheet as gpointer);
            (*d).stylesheet_added = TRUE;
        } else {
            (*d).stylesheets = g_list_remove_link((*d).stylesheets, item);
            (*d).stylesheet_removed = TRUE;
        }
        if inside_stylesheet_cb == 0 {
            webview_stylesheets_regenerate(w);
        }
        return 0 as std::ffi::c_int;
    }
    pub unsafe extern "C" fn luaH_webview_stylesheets_index(
        mut L: *mut lua_State,
    ) -> gint {
        let mut d = (*luaH_checkwebview(
            L,
            -(10002 as std::ffi::c_int) - 1 as std::ffi::c_int,
        ))
            .data as *mut webview_data_t;
        let mut stylesheet = luaH_checkstylesheet(L, 2 as std::ffi::c_int)
            as *mut lstylesheet_t;
        let mut enabled = (g_list_find((*d).stylesheets, stylesheet as gconstpointer)
            != NULL_0 as *mut GList) as std::ffi::c_int;
        lua_pushboolean(L, enabled);
        return 1 as std::ffi::c_int;
    }
    pub unsafe extern "C" fn luaH_webview_stylesheets_newindex(
        mut L: *mut lua_State,
    ) -> gint {
        let mut d = (*luaH_checkwebview(
            L,
            -(10002 as std::ffi::c_int) - 1 as std::ffi::c_int,
        ))
            .data as *mut webview_data_t;
        let mut stylesheet = luaH_checkstylesheet(L, 2 as std::ffi::c_int)
            as *mut lstylesheet_t;
        let mut enable = lua_toboolean(L, 3 as std::ffi::c_int);
        webview_stylesheet_set_enabled((*d).widget, stylesheet, enable);
        return 0 as std::ffi::c_int;
    }
    pub unsafe extern "C" fn luaH_webview_push_stylesheets_table(
        mut L: *mut lua_State,
    ) -> gint {
        lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
        lua_createtable(L, 0 as std::ffi::c_int, 2 as std::ffi::c_int);
        lua_pushlstring(
            L,
            b"__index\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 8]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        lua_pushvalue(L, 1 as std::ffi::c_int);
        lua_pushcclosure(
            L,
            Some(
                luaH_webview_stylesheets_index
                    as unsafe extern "C" fn(*mut lua_State) -> gint,
            ),
            1 as std::ffi::c_int,
        );
        lua_rawset(L, -(3 as std::ffi::c_int));
        lua_pushlstring(
            L,
            b"__newindex\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 11]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        lua_pushvalue(L, 1 as std::ffi::c_int);
        lua_pushcclosure(
            L,
            Some(
                luaH_webview_stylesheets_newindex
                    as unsafe extern "C" fn(*mut lua_State) -> gint,
            ),
            1 as std::ffi::c_int,
        );
        lua_rawset(L, -(3 as std::ffi::c_int));
        lua_setmetatable(L, -(2 as std::ffi::c_int));
        return 1 as std::ffi::c_int;
    }
    pub unsafe extern "C" fn webview_update_stylesheets(
        mut L: *mut lua_State,
        mut w: *mut widget_t,
    ) {
        let mut d = (*w).data as *mut webview_data_t;
        (*d).stylesheet_added = FALSE;
        (*d).stylesheet_removed = FALSE;
        inside_stylesheet_cb = TRUE;
        luaH_object_push(L, (*w).ref_0);
        luaH_object_emit_signal(
            L,
            -(1 as std::ffi::c_int),
            b"stylesheet\0" as *const u8 as *const std::ffi::c_char,
            0 as std::ffi::c_int,
            0 as std::ffi::c_int,
        );
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        inside_stylesheet_cb = FALSE;
        webview_stylesheets_regenerate(w);
    }

}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct webview_data_t {
    pub widget: *mut widget_t,
    pub view: *mut WebKitWebView,
    pub user_content: *mut WebKitUserContentManager,
    pub stylesheets: *mut GList,
    pub stylesheet_added: gboolean,
    pub stylesheet_removed: gboolean,
    pub stylesheet_refreshed: gboolean,
    pub uri: *mut gchar,
    pub hover: *mut gchar,
    pub inspector: *mut WebKitWebInspector,
    pub inspector_open: gboolean,
    pub htr_context: guint,
    pub is_committed: gboolean,
    pub is_failed: gboolean,
    pub private: gboolean,
    pub doc_w: gint,
    pub doc_h: gint,
    pub win_w: gint,
    pub win_h: gint,
    pub scroll_x: gint,
    pub scroll_y: gint,
    pub cert: *mut GTlsCertificate,
    pub ipc: *mut ipc_endpoint_t,
    pub web_process_id: pid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub refs: *mut GSList,
    pub old_refs: *mut GSList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct save_cb_s {
    pub filename: *const gchar,
    pub window: *mut widget_t,
}
static mut related_view: *mut WebKitWebView = 0 as *const WebKitWebView
    as *mut WebKitWebView;
static mut last_popup: C2RustUnnamed_3 = {
    let init = C2RustUnnamed_3 {
        refs: NULL_0 as *mut GSList,
        old_refs: NULL_0 as *mut GSList,
    };
    init
};
static mut webview_properties: [property_t; 7] = [
    {
        let init = property_t {
            tok: L_TK_EDITABLE,
            name: b"editable\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_PROGRESS,
            name: b"estimated-load-progress\0" as *const u8 as *const std::ffi::c_char,
            type_0: DOUBLE,
            writable: FALSE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_IS_LOADING,
            name: b"is-loading\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: FALSE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_IS_PLAYING_AUDIO,
            name: b"is-playing-audio\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: FALSE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_TITLE,
            name: b"title\0" as *const u8 as *const std::ffi::c_char,
            type_0: CHAR,
            writable: FALSE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ZOOM_LEVEL,
            name: b"zoom-level\0" as *const u8 as *const std::ffi::c_char,
            type_0: DOUBLE,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_UNKNOWN,
            name: NULL_0 as *const gchar,
            type_0: BOOL,
            writable: 0 as std::ffi::c_int,
        };
        init
    },
];
static mut webview_settings_properties: [property_t; 48] = [
    {
        let init = property_t {
            tok: L_TK_ALLOW_FILE_ACCESS_FROM_FILE_URLS,
            name: b"allow-file-access-from-file-urls\0" as *const u8
                as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ALLOW_MODAL_DIALOGS,
            name: b"allow-modal-dialogs\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ALLOW_UNIVERSAL_ACCESS_FROM_FILE_URLS,
            name: b"allow-universal-access-from-file-urls\0" as *const u8
                as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_AUTO_LOAD_IMAGES,
            name: b"auto-load-images\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_CURSIVE_FONT_FAMILY,
            name: b"cursive-font-family\0" as *const u8 as *const std::ffi::c_char,
            type_0: CHAR,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_DEFAULT_CHARSET,
            name: b"default-charset\0" as *const u8 as *const std::ffi::c_char,
            type_0: CHAR,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_DEFAULT_FONT_FAMILY,
            name: b"default-font-family\0" as *const u8 as *const std::ffi::c_char,
            type_0: CHAR,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_DEFAULT_FONT_SIZE,
            name: b"default-font-size\0" as *const u8 as *const std::ffi::c_char,
            type_0: INT,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_DEFAULT_MONOSPACE_FONT_SIZE,
            name: b"default-monospace-font-size\0" as *const u8
                as *const std::ffi::c_char,
            type_0: INT,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_DRAW_COMPOSITING_INDICATORS,
            name: b"draw-compositing-indicators\0" as *const u8
                as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_ACCELERATED_2D_CANVAS,
            name: b"enable-accelerated-2d-canvas\0" as *const u8
                as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_CARET_BROWSING,
            name: b"enable-caret-browsing\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_DEVELOPER_EXTRAS,
            name: b"enable-developer-extras\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_DNS_PREFETCHING,
            name: b"enable-dns-prefetching\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_FRAME_FLATTENING,
            name: b"enable-frame-flattening\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_FULLSCREEN,
            name: b"enable-fullscreen\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_HTML5_DATABASE,
            name: b"enable-html5-database\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_HTML5_LOCAL_STORAGE,
            name: b"enable-html5-local-storage\0" as *const u8
                as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_HYPERLINK_AUDITING,
            name: b"enable-hyperlink-auditing\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_JAVA,
            name: b"enable-java\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_JAVASCRIPT,
            name: b"enable-javascript\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_MEDIA_STREAM,
            name: b"enable-media-stream\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_MEDIASOURCE,
            name: b"enable-mediasource\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_PAGE_CACHE,
            name: b"enable-page-cache\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_PLUGINS,
            name: b"enable-plugins\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_RESIZABLE_TEXT_AREAS,
            name: b"enable-resizable-text-areas\0" as *const u8
                as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_SITE_SPECIFIC_QUIRKS,
            name: b"enable-site-specific-quirks\0" as *const u8
                as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_SMOOTH_SCROLLING,
            name: b"enable-smooth-scrolling\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_SPATIAL_NAVIGATION,
            name: b"enable-spatial-navigation\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_WEBGL,
            name: b"enable-webgl\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_TABS_TO_LINKS,
            name: b"enable-tabs-to-links\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_WEBAUDIO,
            name: b"enable-webaudio\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_WRITE_CONSOLE_MESSAGES_TO_STDOUT,
            name: b"enable-write-console-messages-to-stdout\0" as *const u8
                as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ENABLE_XSS_AUDITOR,
            name: b"enable-xss-auditor\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_FANTASY_FONT_FAMILY,
            name: b"fantasy-font-family\0" as *const u8 as *const std::ffi::c_char,
            type_0: CHAR,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_JAVASCRIPT_CAN_ACCESS_CLIPBOARD,
            name: b"javascript-can-access-clipboard\0" as *const u8
                as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_JAVASCRIPT_CAN_OPEN_WINDOWS_AUTOMATICALLY,
            name: b"javascript-can-open-windows-automatically\0" as *const u8
                as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_MEDIA_PLAYBACK_ALLOWS_INLINE,
            name: b"media-playback-allows-inline\0" as *const u8
                as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_MEDIA_PLAYBACK_REQUIRES_GESTURE,
            name: b"media-playback-requires-user-gesture\0" as *const u8
                as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_MINIMUM_FONT_SIZE,
            name: b"minimum-font-size\0" as *const u8 as *const std::ffi::c_char,
            type_0: INT,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_MONOSPACE_FONT_FAMILY,
            name: b"monospace-font-family\0" as *const u8 as *const std::ffi::c_char,
            type_0: CHAR,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_PICTOGRAPH_FONT_FAMILY,
            name: b"pictograph-font-family\0" as *const u8 as *const std::ffi::c_char,
            type_0: CHAR,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_PRINT_BACKGROUNDS,
            name: b"print-backgrounds\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_SANS_SERIF_FONT_FAMILY,
            name: b"sans-serif-font-family\0" as *const u8 as *const std::ffi::c_char,
            type_0: CHAR,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_SERIF_FONT_FAMILY,
            name: b"serif-font-family\0" as *const u8 as *const std::ffi::c_char,
            type_0: CHAR,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_USER_AGENT,
            name: b"user-agent\0" as *const u8 as *const std::ffi::c_char,
            type_0: CHAR,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_ZOOM_TEXT_ONLY,
            name: b"zoom-text-only\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let init = property_t {
            tok: L_TK_UNKNOWN,
            name: NULL_0 as *const gchar,
            type_0: BOOL,
            writable: 0 as std::ffi::c_int,
        };
        init
    },
];
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_checkwebview(
    mut L: *mut lua_State,
    mut udx: gint,
) -> *mut widget_t {
    let mut w = luaH_checkwidget(L, udx);
    if (*(*w).info).tok as std::ffi::c_uint
        != L_TK_WEBVIEW as std::ffi::c_int as std::ffi::c_uint
    {
        luaL_argerror(
            L,
            udx,
            b"incorrect widget type (expected webview)\0" as *const u8
                as *const std::ffi::c_char,
        );
    }
    return w;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn webview_get_by_id(mut view_id: guint64) -> *mut widget_t {
    let mut i = 0 as std::ffi::c_int as std::ffi::c_uint;
    while i < (*globalconf.webviews).len {
        let mut w = *((*globalconf.webviews).pdata).offset(i as isize) as *mut widget_t;
        if webkit_web_view_get_page_id(
            g_type_check_instance_cast(
                (*w).widget as *mut GTypeInstance,
                webkit_web_view_get_type(),
            ) as *mut std::ffi::c_void as *mut WebKitWebView,
        ) == view_id
        {
            return w;
        }
        i = i.wrapping_add(1);
        i;
    }
    return NULL_0 as *mut widget_t;
}
unsafe extern "C" fn luaH_webview_load_string(mut L: *mut lua_State) -> gint {
    let mut d = (*luaH_checkwebview(L, 1 as std::ffi::c_int)).data
        as *mut webview_data_t;
    let mut string = luaL_checklstring(L, 2 as std::ffi::c_int, NULL_0 as *mut size_t);
    let mut base_uri = luaL_checklstring(L, 3 as std::ffi::c_int, NULL_0 as *mut size_t);
    webkit_web_view_load_alternate_html(
        (*d).view,
        string,
        base_uri,
        NULL_0 as *const gchar,
    );
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn save_cb(
    mut o: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut view = o as *mut WebKitWebView;
    let mut scbs = user_data as *mut save_cb_s;
    let mut L = common.L;
    let mut err = NULL_0 as *mut GError;
    let mut result: gboolean = 0;
    result = webkit_web_view_save_to_file_finish(view, res, &mut err);
    luaH_object_push(L, (*(*scbs).window).ref_0);
    lua_pushstring(L, (*scbs).filename);
    if result != 0 {
        lua_pushnil(L);
    } else {
        lua_pushstring(L, (*err).message);
    }
    luaH_object_emit_signal(
        L,
        -(3 as std::ffi::c_int),
        b"save-finished\0" as *const u8 as *const std::ffi::c_char,
        2 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    g_free(scbs as gpointer);
}
unsafe extern "C" fn luaH_webview_save(mut L: *mut lua_State) -> gint {
    let mut scbs = g_malloc0_n(
        1 as std::ffi::c_int as gsize,
        ::core::mem::size_of::<save_cb_s>() as std::ffi::c_ulong,
    ) as *mut save_cb_s;
    let mut d = (*luaH_checkwebview(L, 1 as std::ffi::c_int)).data
        as *mut webview_data_t;
    (*scbs).filename = luaL_checklstring(L, 2 as std::ffi::c_int, NULL_0 as *mut size_t);
    (*scbs).window = (*d).widget;
    let mut fd = g_file_new_for_path((*scbs).filename);
    webkit_web_view_save_to_file(
        (*d).view,
        fd,
        WEBKIT_SAVE_MODE_MHTML,
        NULL_0 as *mut GCancellable,
        Some(
            save_cb
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        scbs as gpointer,
    );
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn notify_cb(
    mut UNUSED_v: *mut WebKitWebView,
    mut ps: *mut GParamSpec,
    mut w: *mut widget_t,
) {
    static mut wvprops: *mut GHashTable = NULL_0 as *mut GHashTable;
    let mut p = 0 as *mut property_t;
    if wvprops.is_null() {
        wvprops = g_hash_table_new(
            Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
            Some(
                g_str_equal
                    as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean,
            ),
        );
        p = webview_properties.as_mut_ptr();
        while !((*p).name).is_null() {
            g_hash_table_insert(wvprops, (*p).name as gpointer, p as gpointer);
            p = p.offset(1);
            p;
        }
    }
    p = g_hash_table_lookup(wvprops, (*ps).name as gconstpointer) as *mut property_t;
    if !p.is_null() {
        let mut L = common.L;
        luaH_object_push(L, (*w).ref_0);
        luaH_object_property_signal(L, -(1 as std::ffi::c_int), (*p).tok);
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    }
}
unsafe extern "C" fn update_uri(mut w: *mut widget_t, mut uri: *const gchar) {
    let mut d = (*w).data as *mut webview_data_t;
    if ((*w).destructor).is_none() {
        return;
    }
    if uri.is_null() {
        uri = webkit_web_view_get_uri((*d).view);
        if uri.is_null() || *uri.offset(0 as std::ffi::c_int as isize) == 0 {
            uri = b"about:blank\0" as *const u8 as *const std::ffi::c_char;
        }
    }
    if g_strcmp0((*d).uri, uri) != 0 {
        g_free((*d).uri as gpointer);
        (*d).uri = g_strdup_inline(uri);
        let mut L = common.L;
        luaH_object_push(L, (*w).ref_0);
        luaH_object_emit_signal(
            L,
            -(1 as std::ffi::c_int),
            b"property::uri\0" as *const u8 as *const std::ffi::c_char,
            0 as std::ffi::c_int,
            0 as std::ffi::c_int,
        );
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    }
}
unsafe extern "C" fn load_failed_cb(
    mut UNUSED_v: *mut WebKitWebView,
    mut UNUSED_e: WebKitLoadEvent,
    mut failing_uri: *mut gchar,
    mut error: *mut GError,
    mut w: *mut widget_t,
) -> gboolean {
    update_uri(w, failing_uri);
    let mut L = common.L;
    (*((*w).data as *mut webview_data_t)).is_failed = TRUE;
    luaH_object_push(L, (*w).ref_0);
    lua_pushstring(L, b"failed\0" as *const u8 as *const std::ffi::c_char);
    lua_pushstring(L, failing_uri);
    luaH_push_gerror(L, error);
    let mut ret = luaH_object_emit_signal(
        L,
        -(4 as std::ffi::c_int),
        b"load-status\0" as *const u8 as *const std::ffi::c_char,
        3 as std::ffi::c_int,
        1 as std::ffi::c_int,
    );
    let mut ignore = (ret != 0 && lua_toboolean(L, -(1 as std::ffi::c_int)) != 0)
        as std::ffi::c_int;
    lua_settop(L, -(ret + 1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    return ignore;
}
unsafe extern "C" fn luaH_webview_push_certificate_flags(
    mut L: *mut lua_State,
    mut errors: GTlsCertificateFlags,
) -> std::ffi::c_int {
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    let mut n = 1 as std::ffi::c_int;
    if errors as std::ffi::c_uint
        & G_TLS_CERTIFICATE_UNKNOWN_CA as std::ffi::c_int as std::ffi::c_uint != 0
    {
        lua_pushlstring(
            L,
            b"unknown-ca\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 11]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        let fresh0 = n;
        n = n + 1;
        lua_rawseti(L, -(2 as std::ffi::c_int), fresh0);
    }
    if errors as std::ffi::c_uint
        & G_TLS_CERTIFICATE_BAD_IDENTITY as std::ffi::c_int as std::ffi::c_uint != 0
    {
        lua_pushlstring(
            L,
            b"bad-identity\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 13]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        let fresh1 = n;
        n = n + 1;
        lua_rawseti(L, -(2 as std::ffi::c_int), fresh1);
    }
    if errors as std::ffi::c_uint
        & G_TLS_CERTIFICATE_NOT_ACTIVATED as std::ffi::c_int as std::ffi::c_uint != 0
    {
        lua_pushlstring(
            L,
            b"not-activated\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 14]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        let fresh2 = n;
        n = n + 1;
        lua_rawseti(L, -(2 as std::ffi::c_int), fresh2);
    }
    if errors as std::ffi::c_uint
        & G_TLS_CERTIFICATE_EXPIRED as std::ffi::c_int as std::ffi::c_uint != 0
    {
        lua_pushlstring(
            L,
            b"expired\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 8]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        let fresh3 = n;
        n = n + 1;
        lua_rawseti(L, -(2 as std::ffi::c_int), fresh3);
    }
    if errors as std::ffi::c_uint
        & G_TLS_CERTIFICATE_REVOKED as std::ffi::c_int as std::ffi::c_uint != 0
    {
        lua_pushlstring(
            L,
            b"revoked\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 8]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        let fresh4 = n;
        n = n + 1;
        lua_rawseti(L, -(2 as std::ffi::c_int), fresh4);
    }
    if errors as std::ffi::c_uint
        & G_TLS_CERTIFICATE_INSECURE as std::ffi::c_int as std::ffi::c_uint != 0
    {
        lua_pushlstring(
            L,
            b"insecure\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 9]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        let fresh5 = n;
        n = n + 1;
        lua_rawseti(L, -(2 as std::ffi::c_int), fresh5);
    }
    if errors as std::ffi::c_uint
        & G_TLS_CERTIFICATE_GENERIC_ERROR as std::ffi::c_int as std::ffi::c_uint != 0
    {
        lua_pushlstring(
            L,
            b"generic-error\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 14]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        let fresh6 = n;
        n = n + 1;
        lua_rawseti(L, -(2 as std::ffi::c_int), fresh6);
    }
    return 1 as std::ffi::c_int;
}
unsafe extern "C" fn load_failed_tls_cb(
    mut UNUSED_v: *mut WebKitWebView,
    mut failing_uri: *mut gchar,
    mut certificate: *mut GTlsCertificate,
    mut errors: GTlsCertificateFlags,
    mut w: *mut widget_t,
) -> gboolean {
    let mut L = common.L;
    let mut d = (*w).data as *mut webview_data_t;
    update_uri(w, failing_uri);
    (*((*w).data as *mut webview_data_t)).is_failed = TRUE;
    if !((*d).cert).is_null() {
        g_object_unref(
            g_type_check_instance_cast(
                (*d).cert as *mut GTypeInstance,
                ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
            ) as *mut std::ffi::c_void as *mut GObject as gpointer,
        );
        (*d).cert = NULL_0 as *mut GTlsCertificate;
    }
    (*d).cert = certificate;
    g_object_ref(
        g_type_check_instance_cast(
            (*d).cert as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject as gpointer,
    );
    luaH_object_push(L, (*w).ref_0);
    lua_pushlstring(
        L,
        b"failed\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 7]>() as std::ffi::c_ulong)
            .wrapping_div(
                ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
            )
            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
    );
    lua_pushstring(L, failing_uri);
    let mut error = g_error_new_literal(
        luakit_error_quark(),
        LUAKIT_ERROR_TLS as std::ffi::c_int,
        b"Unacceptable TLS certificate\0" as *const u8 as *const std::ffi::c_char,
    );
    luaH_push_gerror(L, error);
    g_error_free(error);
    luaH_webview_push_certificate_flags(L, errors);
    lua_setfield(
        L,
        -(2 as std::ffi::c_int),
        b"certificate_flags\0" as *const u8 as *const std::ffi::c_char,
    );
    luaH_object_emit_signal(
        L,
        -(4 as std::ffi::c_int),
        b"load-status\0" as *const u8 as *const std::ffi::c_char,
        3 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    return TRUE;
}
unsafe extern "C" fn webview_get_source_finished(
    mut main_resource: *mut WebKitWebResource,
    mut res: *mut GAsyncResult,
    mut L: *mut lua_State,
) {
    let mut length: gsize = 0;
    let mut source: *const gchar = webkit_web_resource_get_data_finish(
        main_resource,
        res,
        &mut length,
        NULL_0 as *mut *mut GError,
    ) as *mut gchar;
    g_object_unref(main_resource as gpointer);
    lua_pushlstring(L, source, length);
    luaH_resume(L, 1 as std::ffi::c_int);
}
unsafe extern "C" fn luaH_webview_push_source(mut L: *mut lua_State) -> gint {
    let mut d = (*luaH_checkwebview(L, 1 as std::ffi::c_int)).data
        as *mut webview_data_t;
    let mut main_resource = webkit_web_view_get_main_resource((*d).view);
    if main_resource.is_null() {
        return 0 as std::ffi::c_int;
    }
    g_object_ref(main_resource as gpointer);
    webkit_web_resource_get_data(
        main_resource,
        NULL_0 as *mut GCancellable,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut WebKitWebResource,
                    *mut GAsyncResult,
                    *mut lua_State,
                ) -> (),
            >,
            GAsyncReadyCallback,
        >(
            Some(
                webview_get_source_finished
                    as unsafe extern "C" fn(
                        *mut WebKitWebResource,
                        *mut GAsyncResult,
                        *mut lua_State,
                    ) -> (),
            ),
        ),
        L as gpointer,
    );
    return luaH_yield(L);
}
unsafe extern "C" fn load_changed_cb(
    mut UNUSED_v: *mut WebKitWebView,
    mut e: WebKitLoadEvent,
    mut w: *mut widget_t,
) {
    let mut d = (*w).data as *mut webview_data_t;
    let mut L = common.L;
    let mut name = NULL_0 as *mut gchar;
    match e as std::ffi::c_uint {
        0 => {
            name = b"provisional\0" as *const u8 as *const std::ffi::c_char
                as *mut gchar;
        }
        1 => {
            name = b"redirected\0" as *const u8 as *const std::ffi::c_char as *mut gchar;
        }
        2 => {
            name = b"committed\0" as *const u8 as *const std::ffi::c_char as *mut gchar;
        }
        3 => {
            name = b"finished\0" as *const u8 as *const std::ffi::c_char as *mut gchar;
        }
        _ => {
            _log(
                LOG_LEVEL_warn,
                b"widgets/webview.c\0" as *const u8 as *const std::ffi::c_char,
                b"programmer error, unable to get load status literal\0" as *const u8
                    as *const std::ffi::c_char,
            );
        }
    }
    update_uri(w, NULL_0 as *const gchar);
    if e as std::ffi::c_uint
        == WEBKIT_LOAD_STARTED as std::ffi::c_int as std::ffi::c_uint
    {
        (*((*w).data as *mut webview_data_t)).is_committed = FALSE;
    } else if e as std::ffi::c_uint
        == WEBKIT_LOAD_COMMITTED as std::ffi::c_int as std::ffi::c_uint
        || e as std::ffi::c_uint
            == WEBKIT_LOAD_FINISHED as std::ffi::c_int as std::ffi::c_uint
    {
        (*((*w).data as *mut webview_data_t)).is_committed = TRUE;
    }
    if e as std::ffi::c_uint
        == WEBKIT_LOAD_STARTED as std::ffi::c_int as std::ffi::c_uint
    {
        if !((*d).cert).is_null() {
            g_object_unref(
                g_type_check_instance_cast(
                    (*d).cert as *mut GTypeInstance,
                    ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
                ) as *mut std::ffi::c_void as *mut GObject as gpointer,
            );
            (*d).cert = NULL_0 as *mut GTlsCertificate;
        }
    } else if e as std::ffi::c_uint
        == WEBKIT_LOAD_COMMITTED as std::ffi::c_int as std::ffi::c_uint
    {
        if ((*d).cert).is_null() {} else {
            g_assertion_message_expr(
                G_LOG_DOMAIN as *const std::ffi::c_char,
                b"widgets/webview.c\0" as *const u8 as *const std::ffi::c_char,
                411 as std::ffi::c_int,
                (*::core::mem::transmute::<
                    &[u8; 16],
                    &[std::ffi::c_char; 16],
                >(b"load_changed_cb\0"))
                    .as_ptr(),
                b"!d->cert\0" as *const u8 as *const std::ffi::c_char,
            );
        }
        webkit_web_view_get_tls_info(
            (*d).view,
            &mut (*d).cert,
            NULL_0 as *mut GTlsCertificateFlags,
        );
        if !((*d).cert).is_null() {
            g_object_ref(
                g_type_check_instance_cast(
                    (*d).cert as *mut GTypeInstance,
                    ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
                ) as *mut std::ffi::c_void as *mut GObject as gpointer,
            );
        }
    }
    if e as std::ffi::c_uint
        == WEBKIT_LOAD_COMMITTED as std::ffi::c_int as std::ffi::c_uint
    {
        webview_update_stylesheets(L, w);
    }
    if e as std::ffi::c_uint
        == WEBKIT_LOAD_STARTED as std::ffi::c_int as std::ffi::c_uint
    {
        (*((*w).data as *mut webview_data_t)).is_failed = FALSE;
    }
    if e as std::ffi::c_uint
        == WEBKIT_LOAD_FINISHED as std::ffi::c_int as std::ffi::c_uint
        && (*((*w).data as *mut webview_data_t)).is_failed != 0
    {
        return;
    }
    luaH_object_push(L, (*w).ref_0);
    lua_pushstring(L, name);
    luaH_object_emit_signal(
        L,
        -(2 as std::ffi::c_int),
        b"load-status\0" as *const u8 as *const std::ffi::c_char,
        1 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
}
unsafe extern "C" fn create_cb(
    mut v: *mut WebKitWebView,
    mut UNUSED_a: *mut WebKitNavigationAction,
    mut w: *mut widget_t,
) -> *mut GtkWidget {
    let mut view = NULL_0 as *mut WebKitWebView;
    let mut new = 0 as *mut widget_t;
    if related_view.is_null() {} else {
        g_assertion_message_expr(
            G_LOG_DOMAIN as *const std::ffi::c_char,
            b"widgets/webview.c\0" as *const u8 as *const std::ffi::c_char,
            439 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 10],
                &[std::ffi::c_char; 10],
            >(b"create_cb\0"))
                .as_ptr(),
            b"!related_view\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    related_view = v;
    let mut L = common.L;
    let mut top = lua_gettop(L);
    luaH_object_push(L, (*w).ref_0);
    let mut ret = luaH_object_emit_signal(
        L,
        -(1 as std::ffi::c_int),
        b"create-web-view\0" as *const u8 as *const std::ffi::c_char,
        0 as std::ffi::c_int,
        1 as std::ffi::c_int,
    );
    related_view = NULL_0 as *mut WebKitWebView;
    if ret != 0 {
        new = luaH_toudata(L, -(1 as std::ffi::c_int), &mut widget_class)
            as *mut widget_t;
        if !new.is_null() {
            if (*(*new).info).tok as std::ffi::c_uint
                == L_TK_WEBVIEW as std::ffi::c_int as std::ffi::c_uint
            {
                view = g_type_check_instance_cast(
                    (*((*new).data as *mut webview_data_t)).view as *mut GTypeInstance,
                    webkit_web_view_get_type(),
                ) as *mut std::ffi::c_void as *mut WebKitWebView;
            } else {
                _log(
                    LOG_LEVEL_warn,
                    b"widgets/webview.c\0" as *const u8 as *const std::ffi::c_char,
                    b"invalid return widget type (expected webview, got %s)\0"
                        as *const u8 as *const std::ffi::c_char,
                    (*(*new).info).name,
                );
            }
        } else {
            _log(
                LOG_LEVEL_warn,
                b"widgets/webview.c\0" as *const u8 as *const std::ffi::c_char,
                b"invalid signal return object type (expected webview widget, got %s)\0"
                    as *const u8 as *const std::ffi::c_char,
                lua_typename(L, lua_type(L, -(1 as std::ffi::c_int))),
            );
        }
    }
    lua_settop(L, top);
    return g_type_check_instance_cast(view as *mut GTypeInstance, gtk_widget_get_type())
        as *mut std::ffi::c_void as *mut GtkWidget;
}
unsafe extern "C" fn decide_policy_cb(
    mut UNUSED_v: *mut WebKitWebView,
    mut p: *mut WebKitPolicyDecision,
    mut type_0: WebKitPolicyDecisionType,
    mut w: *mut widget_t,
) -> gboolean {
    let mut L = common.L;
    match type_0 as std::ffi::c_uint {
        0 | 1 => {
            let mut top = lua_gettop(L);
            let mut np = g_type_check_instance_cast(
                p as *mut GTypeInstance,
                webkit_navigation_policy_decision_get_type(),
            ) as *mut std::ffi::c_void as *mut WebKitNavigationPolicyDecision;
            let mut na = webkit_navigation_policy_decision_get_navigation_action(np);
            let mut signal_name = if type_0 as std::ffi::c_uint
                == WEBKIT_POLICY_DECISION_TYPE_NAVIGATION_ACTION as std::ffi::c_int
                    as std::ffi::c_uint
            {
                b"navigation-request\0" as *const u8 as *const std::ffi::c_char
            } else {
                b"new-window-decision\0" as *const u8 as *const std::ffi::c_char
            };
            let mut uri = webkit_uri_request_get_uri(
                webkit_navigation_action_get_request(na),
            );
            let mut reason = NULL_0 as *mut gchar;
            match webkit_navigation_action_get_navigation_type(na) as std::ffi::c_uint {
                0 => {
                    reason = b"link-clicked\0" as *const u8 as *const std::ffi::c_char
                        as *mut gchar;
                }
                1 => {
                    reason = b"form-submitted\0" as *const u8 as *const std::ffi::c_char
                        as *mut gchar;
                }
                2 => {
                    reason = b"back-forward\0" as *const u8 as *const std::ffi::c_char
                        as *mut gchar;
                }
                3 => {
                    reason = b"reload\0" as *const u8 as *const std::ffi::c_char
                        as *mut gchar;
                }
                4 => {
                    reason = b"form-resubmitted\0" as *const u8
                        as *const std::ffi::c_char as *mut gchar;
                }
                5 => {
                    reason = b"other\0" as *const u8 as *const std::ffi::c_char
                        as *mut gchar;
                }
                _ => {
                    _log(
                        LOG_LEVEL_warn,
                        b"widgets/webview.c\0" as *const u8 as *const std::ffi::c_char,
                        b"programmer error, unable to get web navigation reason literal\0"
                            as *const u8 as *const std::ffi::c_char,
                    );
                }
            }
            luaH_object_push(L, (*w).ref_0);
            lua_pushstring(L, uri);
            lua_pushstring(L, reason);
            let mut ret = luaH_object_emit_signal(
                L,
                -(3 as std::ffi::c_int),
                signal_name,
                2 as std::ffi::c_int,
                1 as std::ffi::c_int,
            );
            let mut ignore = (ret != 0 && lua_toboolean(L, -(1 as std::ffi::c_int)) == 0)
                as std::ffi::c_int;
            if ignore != 0 {
                webkit_policy_decision_ignore(p);
            }
            lua_settop(L, top);
            return ignore;
        }
        2 => {
            let mut rp = g_type_check_instance_cast(
                p as *mut GTypeInstance,
                webkit_response_policy_decision_get_type(),
            ) as *mut std::ffi::c_void as *mut WebKitResponsePolicyDecision;
            let mut r = webkit_response_policy_decision_get_response(rp);
            let mut uri_0 = webkit_uri_response_get_uri(r);
            let mut mime = webkit_uri_response_get_mime_type(r);
            if webkit_uri_response_get_status_code(r) != 0
                && !(webkit_uri_response_get_status_code(r)
                    >= 200 as std::ffi::c_int as guint
                    && webkit_uri_response_get_status_code(r)
                        < 300 as std::ffi::c_int as guint)
            {
                return FALSE;
            }
            luaH_object_push(L, (*w).ref_0);
            lua_pushstring(L, uri_0);
            lua_pushstring(L, mime);
            let mut ret_0 = luaH_object_emit_signal(
                L,
                -(3 as std::ffi::c_int),
                b"mime-type-decision\0" as *const u8 as *const std::ffi::c_char,
                2 as std::ffi::c_int,
                1 as std::ffi::c_int,
            );
            let mut ignore_0 = (ret_0 != 0
                && lua_toboolean(L, -(1 as std::ffi::c_int)) == 0) as std::ffi::c_int;
            if ignore_0 != 0 {
                webkit_policy_decision_ignore(p);
            } else if strcmp(
                mime as *const std::ffi::c_char,
                b"application/x-extension-html\0" as *const u8 as *const std::ffi::c_char,
            ) == 0 as std::ffi::c_int
            {
                webkit_policy_decision_use(p);
            } else if webkit_response_policy_decision_is_mime_type_supported(rp) != 0 {
                webkit_policy_decision_use(p);
            } else {
                webkit_policy_decision_download(p);
            }
            lua_settop(L, -(ret_0 + 1 as std::ffi::c_int) - 1 as std::ffi::c_int);
            return TRUE;
        }
        _ => {}
    }
    return FALSE;
}
unsafe extern "C" fn luaH_webview_reload(mut L: *mut lua_State) -> gint {
    let mut d = (*luaH_checkwebview(L, 1 as std::ffi::c_int)).data
        as *mut webview_data_t;
    webkit_web_view_reload((*d).view);
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn luaH_webview_reload_bypass_cache(mut L: *mut lua_State) -> gint {
    let mut d = (*luaH_checkwebview(L, 1 as std::ffi::c_int)).data
        as *mut webview_data_t;
    webkit_web_view_reload_bypass_cache((*d).view);
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn luaH_webview_search(mut L: *mut lua_State) -> gint {
    let mut d = (*luaH_checkwebview(L, 1 as std::ffi::c_int)).data
        as *mut webview_data_t;
    let mut text = luaL_checklstring(L, 2 as std::ffi::c_int, NULL_0 as *mut size_t);
    let mut case_sensitive = luaH_checkboolean(L, 3 as std::ffi::c_int);
    let mut forward = luaH_checkboolean(L, 4 as std::ffi::c_int);
    let mut wrap = luaH_checkboolean(L, 5 as std::ffi::c_int);
    let mut textlen = strlen(text);
    let mut max_match_count = if textlen < 5 as std::ffi::c_int as size_t {
        100 as std::ffi::c_int as std::ffi::c_uint
    } else {
        G_MAXUINT
    };
    let mut webkit_fc = webkit_web_view_get_find_controller((*d).view);
    webkit_find_controller_search_finish(webkit_fc);
    webkit_find_controller_search(
        webkit_fc,
        text,
        (WEBKIT_FIND_OPTIONS_CASE_INSENSITIVE as std::ffi::c_int
            * (case_sensitive == 0) as std::ffi::c_int
            | WEBKIT_FIND_OPTIONS_BACKWARDS as std::ffi::c_int
                * (forward == 0) as std::ffi::c_int
            | WEBKIT_FIND_OPTIONS_WRAP_AROUND as std::ffi::c_int * wrap) as guint32,
        max_match_count,
    );
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn luaH_webview_search_next(mut L: *mut lua_State) -> gint {
    let mut d = (*luaH_checkwebview(L, 1 as std::ffi::c_int)).data
        as *mut webview_data_t;
    let mut webkit_fc = webkit_web_view_get_find_controller((*d).view);
    webkit_find_controller_search_next(webkit_fc);
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn luaH_webview_search_previous(mut L: *mut lua_State) -> gint {
    let mut d = (*luaH_checkwebview(L, 1 as std::ffi::c_int)).data
        as *mut webview_data_t;
    let mut webkit_fc = webkit_web_view_get_find_controller((*d).view);
    webkit_find_controller_search_previous(webkit_fc);
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn luaH_webview_clear_search(mut L: *mut lua_State) -> gint {
    let mut d = (*luaH_checkwebview(L, 1 as std::ffi::c_int)).data
        as *mut webview_data_t;
    let mut webkit_fc = webkit_web_view_get_find_controller((*d).view);
    webkit_find_controller_search_finish(webkit_fc);
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn luaH_webview_loading(mut L: *mut lua_State) -> gint {
    let mut d = (*luaH_checkwebview(L, 1 as std::ffi::c_int)).data
        as *mut webview_data_t;
    luaH_gobject_index(
        L,
        webview_properties.as_mut_ptr(),
        L_TK_IS_LOADING,
        g_type_check_instance_cast(
            (*d).view as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject,
    );
    return 1 as std::ffi::c_int;
}
unsafe extern "C" fn luaH_webview_stop(mut L: *mut lua_State) -> gint {
    let mut d = (*luaH_checkwebview(L, 1 as std::ffi::c_int)).data
        as *mut webview_data_t;
    webkit_web_view_stop_loading((*d).view);
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn luaH_webview_crash(mut L: *mut lua_State) -> gint {
    let mut d = (*luaH_checkwebview(L, 1 as std::ffi::c_int)).data
        as *mut webview_data_t;
    let mut header = {
        let mut init = _ipc_header_t {
            length: 0 as std::ffi::c_int as guint,
            type_0: IPC_TYPE_crash,
        };
        init
    };
    ipc_send((*d).ipc, &mut header, NULL_0 as *const std::ffi::c_void);
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn luaH_webview_ssl_trusted(mut L: *mut lua_State) -> gint {
    let mut d = (*luaH_checkwebview(L, 1 as std::ffi::c_int)).data
        as *mut webview_data_t;
    let mut uri = webkit_web_view_get_uri((*d).view);
    let mut cert = 0 as *mut GTlsCertificate;
    let mut cert_errors = G_TLS_CERTIFICATE_NO_FLAGS;
    if !uri.is_null() && (*d).is_committed != 0
        && webkit_web_view_get_tls_info((*d).view, &mut cert, &mut cert_errors) != 0
    {
        let mut is_trusted = (cert_errors as std::ffi::c_uint
            == 0 as std::ffi::c_int as std::ffi::c_uint) as std::ffi::c_int;
        lua_pushboolean(L, is_trusted);
        return 1 as std::ffi::c_int;
    }
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn luaH_webview_allow_certificate(mut L: *mut lua_State) -> gint {
    _log(
        LOG_LEVEL_warn,
        b"widgets/webview.c\0" as *const u8 as *const std::ffi::c_char,
        b"webview:allow_certificate() is deprecated: use luakit.allow_certificate() instead\0"
            as *const u8 as *const std::ffi::c_char,
    );
    luaH_checkwebview(L, 1 as std::ffi::c_int);
    lua_remove(L, 1 as std::ffi::c_int);
    luaL_checklstring(L, 1 as std::ffi::c_int, NULL_0 as *mut size_t);
    luaL_checklstring(L, 2 as std::ffi::c_int, NULL_0 as *mut size_t);
    return luaH_luakit_allow_certificate(L);
}
unsafe extern "C" fn luaH_webview_push_certificate(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
) -> gint {
    let mut d = (*w).data as *mut webview_data_t;
    if ((*d).cert).is_null() {
        return 0 as std::ffi::c_int;
    }
    let mut cert_pem = 0 as *mut gchar;
    g_object_get(
        g_type_check_instance_cast(
            (*d).cert as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject as gpointer,
        b"certificate-pem\0" as *const u8 as *const std::ffi::c_char,
        &mut cert_pem as *mut *mut gchar,
        NULL_0 as *mut std::ffi::c_void,
    );
    lua_pushstring(L, cert_pem);
    g_free(cert_pem as gpointer);
    return 1 as std::ffi::c_int;
}
unsafe extern "C" fn webview_translate_old_token(
    mut token: luakit_token_t,
) -> luakit_token_t {
    match token as std::ffi::c_uint {
        76 => return L_TK_ENABLE_JAVASCRIPT,
        _ => return token,
    };
}
unsafe extern "C" fn favicon_cb(
    mut UNUSED_v: *mut WebKitWebView,
    mut UNUSED_param_spec: *mut GParamSpec,
    mut w: *mut widget_t,
) {
    let mut L = common.L;
    luaH_object_push(L, (*w).ref_0);
    luaH_object_emit_signal(
        L,
        -(1 as std::ffi::c_int),
        b"favicon\0" as *const u8 as *const std::ffi::c_char,
        0 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
}
unsafe extern "C" fn uri_cb(
    mut UNUSED_v: *mut WebKitWebView,
    mut UNUSED_param_spec: *mut GParamSpec,
    mut w: *mut widget_t,
) {
    update_uri(w, NULL_0 as *const gchar);
}
unsafe extern "C" fn permission_request_cb(
    mut UNUSED_v: *mut WebKitWebView,
    mut request: *mut WebKitPermissionRequest,
    mut w: *mut widget_t,
) -> gboolean {
    let mut L = common.L;
    let mut top = lua_gettop(L);
    if ({
        let mut __inst = request as *mut GTypeInstance;
        let mut __t = webkit_notification_permission_request_get_type();
        let mut __r: gboolean = 0;
        if __inst.is_null() {
            __r = FALSE;
        } else if !((*__inst).g_class).is_null() && (*(*__inst).g_class).g_type == __t {
            __r = TRUE;
        } else {
            __r = g_type_check_instance_is_a(__inst, __t);
        }
        __r
    }) != 0
    {
        lua_pushlstring(
            L,
            b"notification\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 13]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
    } else if ({
        let mut __inst = request as *mut GTypeInstance;
        let mut __t = webkit_geolocation_permission_request_get_type();
        let mut __r: gboolean = 0;
        if __inst.is_null() {
            __r = FALSE;
        } else if !((*__inst).g_class).is_null() && (*(*__inst).g_class).g_type == __t {
            __r = TRUE;
        } else {
            __r = g_type_check_instance_is_a(__inst, __t);
        }
        __r
    }) != 0
    {
        lua_pushlstring(
            L,
            b"geolocation\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 12]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
    } else if ({
        let mut __inst = request as *mut GTypeInstance;
        let mut __t = webkit_install_missing_media_plugins_permission_request_get_type();
        let mut __r: gboolean = 0;
        if __inst.is_null() {
            __r = FALSE;
        } else if !((*__inst).g_class).is_null() && (*(*__inst).g_class).g_type == __t {
            __r = TRUE;
        } else {
            __r = g_type_check_instance_is_a(__inst, __t);
        }
        __r
    }) != 0
    {
        lua_pushlstring(
            L,
            b"install-missing-media-plugins\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 30]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        let mut ummpr = request
            as *mut WebKitInstallMissingMediaPluginsPermissionRequest;
        lua_pushstring(
            L,
            webkit_install_missing_media_plugins_permission_request_get_description(
                ummpr,
            ),
        );
    } else if ({
        let mut __inst = request as *mut GTypeInstance;
        let mut __t = webkit_user_media_permission_request_get_type();
        let mut __r: gboolean = 0;
        if __inst.is_null() {
            __r = FALSE;
        } else if !((*__inst).g_class).is_null() && (*(*__inst).g_class).g_type == __t {
            __r = TRUE;
        } else {
            __r = g_type_check_instance_is_a(__inst, __t);
        }
        __r
    }) != 0
    {
        lua_pushlstring(
            L,
            b"user-media\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 11]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        lua_createtable(L, 0 as std::ffi::c_int, 2 as std::ffi::c_int);
        let mut umpr = request as *mut WebKitUserMediaPermissionRequest;
        lua_pushboolean(L, webkit_user_media_permission_is_for_audio_device(umpr));
        lua_setfield(
            L,
            -(2 as std::ffi::c_int),
            b"audio\0" as *const u8 as *const std::ffi::c_char,
        );
        lua_pushboolean(L, webkit_user_media_permission_is_for_video_device(umpr));
        lua_setfield(
            L,
            -(2 as std::ffi::c_int),
            b"video\0" as *const u8 as *const std::ffi::c_char,
        );
    } else {
        return FALSE
    }
    let mut argc = lua_gettop(L) - top;
    luaH_object_push(L, (*w).ref_0);
    lua_insert(L, top + 1 as std::ffi::c_int);
    let mut ret = luaH_object_emit_signal(
        L,
        top + 1 as std::ffi::c_int,
        b"permission-request\0" as *const u8 as *const std::ffi::c_char,
        argc,
        1 as std::ffi::c_int,
    );
    if ret != 0 {
        if lua_toboolean(L, -(1 as std::ffi::c_int)) != 0 {
            webkit_permission_request_allow(request);
        } else {
            webkit_permission_request_deny(request);
        }
    }
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    return (ret > 0 as std::ffi::c_int) as std::ffi::c_int;
}
unsafe extern "C" fn luaH_webview_set_pdfjs(mut L: *mut lua_State) -> gint {
    let mut d = (*luaH_checkwebview(L, 1 as std::ffi::c_int)).data
        as *mut webview_data_t;
    let mut enabled = luaH_checkboolean(L, 2 as std::ffi::c_int);
    let mut features = webkit_settings_get_all_features();
    let mut settings = webkit_web_view_get_settings((*d).view);
    let mut i = 0 as std::ffi::c_int as gsize;
    while i < webkit_feature_list_get_length(features) {
        let mut feature = webkit_feature_list_get(features, i);
        if strcmp(
            webkit_feature_get_identifier(feature),
            b"PdfJSViewer\0" as *const u8 as *const std::ffi::c_char,
        ) == 0
        {
            webkit_settings_set_feature_enabled(settings, feature, enabled);
            break;
        } else {
            i = i.wrapping_add(1);
            i;
        }
    }
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn luaH_webview_index(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
    mut token: luakit_token_t,
) -> gint {
    let mut d = (*w).data as *mut webview_data_t;
    let mut ret: gint = 0;
    token = webview_translate_old_token(token);
    match token as std::ffi::c_uint {
        162 => return luaH_widget_get_parent(L, w),
        100 => return luaH_widget_get_focused(L, w),
        253 => return luaH_widget_get_visible(L, w),
        241 => return luaH_widget_get_tooltip(L, w),
        262 => return luaH_widget_get_width(L, w),
        109 => return luaH_widget_get_height(L, w),
        149 => return luaH_widget_get_min_size(L, w),
        3 => return luaH_widget_get_align(L, w),
        24 => return luaH_widget_get_children(L, w),
        211 => {
            lua_pushcclosure(
                L,
                Some(luaH_widget_show as unsafe extern "C" fn(*mut lua_State) -> gint),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        110 => {
            lua_pushcclosure(
                L,
                Some(luaH_widget_hide as unsafe extern "C" fn(*mut lua_State) -> gint),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        99 => {
            lua_pushcclosure(
                L,
                Some(luaH_widget_focus as unsafe extern "C" fn(*mut lua_State) -> gint),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        50 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_widget_destroy as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        183 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_widget_replace as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        203 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_widget_send_key as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        124 => {
            lua_pushboolean(L, (*d).inspector_open);
            return 1 as std::ffi::c_int;
        }
        171 => {
            lua_pushboolean(L, (*d).private);
            return 1 as std::ffi::c_int;
        }
        27 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_webview_clear_search
                        as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        196 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_webview_search as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        197 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_webview_search_next
                        as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        198 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_webview_search_previous
                        as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        105 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_webview_go_back as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        106 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_webview_go_forward
                        as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        18 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_webview_can_go_back
                        as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        19 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_webview_can_go_forward
                        as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        89 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_webview_eval_js as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        138 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_webview_load_string
                        as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        188 => {
            lua_pushcclosure(
                L,
                Some(luaH_webview_save as unsafe extern "C" fn(*mut lua_State) -> gint),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        137 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_webview_loading as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        178 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_webview_reload as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        179 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_webview_reload_bypass_cache
                        as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        222 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_webview_ssl_trusted
                        as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        227 => {
            lua_pushcclosure(
                L,
                Some(luaH_webview_stop as unsafe extern "C" fn(*mut lua_State) -> gint),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        36 => {
            lua_pushcclosure(
                L,
                Some(luaH_webview_crash as unsafe extern "C" fn(*mut lua_State) -> gint),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        214 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_webview_show_inspector
                        as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        31 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_webview_close_inspector
                        as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        4 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_webview_allow_certificate
                        as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        209 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_webview_set_pdfjs
                        as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            return 1 as std::ffi::c_int;
        }
        113 => {
            lua_pushstring(L, (*d).hover);
            return 1 as std::ffi::c_int;
        }
        246 => {
            lua_pushstring(L, (*d).uri);
            return 1 as std::ffi::c_int;
        }
        261 => {
            lua_pushinteger(L, (*d).web_process_id as lua_Integer);
            return 1 as std::ffi::c_int;
        }
        217 => {
            return luaL_error(
                L,
                b"view.source has been removed; use view:get_source() instead\0"
                    as *const u8 as *const std::ffi::c_char,
            );
        }
        103 => {
            lua_pushcclosure(
                L,
                Some(
                    luaH_webview_push_source
                        as unsafe extern "C" fn(*mut lua_State) -> gint,
                ),
                0 as std::ffi::c_int,
            );
            luaH_yield_wrap_function(L);
            return 1 as std::ffi::c_int;
        }
        205 => return luaH_webview_push_session_state(L, d),
        229 => return luaH_webview_push_stylesheets_table(L),
        117 => {
            lua_pushnumber(L, webkit_web_view_get_page_id((*d).view) as lua_Number);
            return 1 as std::ffi::c_int;
        }
        111 => return luaH_webview_push_history(L, (*d).view),
        191 => return luaH_webview_push_scroll_table(L),
        21 => return luaH_webview_push_certificate(L, w),
        _ => {}
    }
    ret = luaH_gobject_index(
        L,
        webview_properties.as_mut_ptr(),
        token,
        g_type_check_instance_cast(
            (*d).view as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject,
    );
    if ret != 0 {
        return ret;
    }
    if token as std::ffi::c_uint
        == L_TK_HARDWARE_ACCELERATION_POLICY as std::ffi::c_int as std::ffi::c_uint
    {
        match webkit_settings_get_hardware_acceleration_policy(
            webkit_web_view_get_settings((*d).view),
        ) as std::ffi::c_uint
        {
            0 => {
                lua_pushstring(
                    L,
                    b"on-demand\0" as *const u8 as *const std::ffi::c_char,
                );
                return 1 as std::ffi::c_int;
            }
            1 => {
                lua_pushstring(L, b"always\0" as *const u8 as *const std::ffi::c_char);
                return 1 as std::ffi::c_int;
            }
            2 => {
                lua_pushstring(L, b"never\0" as *const u8 as *const std::ffi::c_char);
                return 1 as std::ffi::c_int;
            }
            _ => {
                g_assertion_message_expr(
                    G_LOG_DOMAIN as *const std::ffi::c_char,
                    b"widgets/webview.c\0" as *const u8 as *const std::ffi::c_char,
                    867 as std::ffi::c_int,
                    (*::core::mem::transmute::<
                        &[u8; 19],
                        &[std::ffi::c_char; 19],
                    >(b"luaH_webview_index\0"))
                        .as_ptr(),
                    NULL_0 as *const std::ffi::c_char,
                );
            }
        }
    }
    ret = luaH_gobject_index(
        L,
        webview_settings_properties.as_mut_ptr(),
        token,
        g_type_check_instance_cast(
            webkit_web_view_get_settings((*d).view) as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject,
    );
    if ret != 0 {
        return ret;
    }
    return luaL_error(
        L,
        b"cannot get unknown webview property '%s'\0" as *const u8
            as *const std::ffi::c_char,
        lua_tolstring(L, 2 as std::ffi::c_int, NULL_0 as *mut size_t),
    );
}
unsafe extern "C" fn parse_uri(mut uri: *const gchar) -> *mut gchar {
    if uri.is_null() || *uri.offset(0 as std::ffi::c_int as isize) == 0
        || g_strcmp0(uri, b"about:blank\0" as *const u8 as *const std::ffi::c_char) == 0
    {
        return g_strdup_inline(b"about:blank\0" as *const u8 as *const std::ffi::c_char)
    } else if !(g_strrstr(uri, b"://\0" as *const u8 as *const std::ffi::c_char))
        .is_null()
    {
        return g_strdup_inline(uri)
    } else if file_exists(uri) != 0 {
        if g_path_is_absolute(uri) != 0 {
            return g_strdup_printf(
                b"file://%s\0" as *const u8 as *const std::ffi::c_char,
                uri,
            )
        } else {
            let mut cwd = g_get_current_dir();
            let mut path = g_build_filename(cwd, uri, NULL_0 as *mut std::ffi::c_void);
            let mut new = g_strdup_printf(
                b"file://%s\0" as *const u8 as *const std::ffi::c_char,
                path,
            );
            g_free(cwd as gpointer);
            g_free(path as gpointer);
            return new;
        }
    }
    return g_strdup_printf(b"http://%s\0" as *const u8 as *const std::ffi::c_char, uri);
}
unsafe extern "C" fn luaH_webview_newindex(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
    mut token: luakit_token_t,
) -> gint {
    let mut len: size_t = 0;
    let mut d = (*w).data as *mut webview_data_t;
    let mut uri = 0 as *mut gchar;
    token = webview_translate_old_token(token);
    match token as std::ffi::c_uint {
        253 => {
            luaH_widget_set_visible(L, w);
        }
        241 => {
            luaH_widget_set_tooltip(L, w);
        }
        149 => {
            luaH_widget_set_min_size(L, w);
        }
        3 => {
            luaH_widget_set_align(L, w);
        }
        246 => {
            uri = parse_uri(luaL_checklstring(L, 3 as std::ffi::c_int, &mut len));
            webkit_web_view_load_uri((*d).view, uri);
            update_uri(w, uri);
            g_free(uri as gpointer);
            return 0 as std::ffi::c_int;
        }
        205 => {
            luaH_webview_set_session_state(L, d);
            return 0 as std::ffi::c_int;
        }
        _ => {}
    }
    if token as std::ffi::c_uint
        == L_TK_ZOOM_LEVEL as std::ffi::c_int as std::ffi::c_uint
        && lua_isnumber(L, 3 as std::ffi::c_int) != 0
        && lua_tonumber(L, 3 as std::ffi::c_int) != 1.0f64
    {
        g_object_freeze_notify(
            g_type_check_instance_cast(
                (*d).view as *mut GTypeInstance,
                ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
            ) as *mut std::ffi::c_void as *mut GObject,
        );
        g_object_set(
            (*d).view as gpointer,
            b"zoom-level\0" as *const u8 as *const std::ffi::c_char,
            1.0f64,
            NULL_0 as *mut std::ffi::c_void,
        );
        g_object_thaw_notify(
            g_type_check_instance_cast(
                (*d).view as *mut GTypeInstance,
                ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
            ) as *mut std::ffi::c_void as *mut GObject,
        );
    }
    let mut emit = luaH_gobject_newindex(
        L,
        webview_properties.as_mut_ptr(),
        token,
        3 as std::ffi::c_int,
        g_type_check_instance_cast(
            (*d).view as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject,
    );
    if token as std::ffi::c_uint
        == L_TK_HARDWARE_ACCELERATION_POLICY as std::ffi::c_int as std::ffi::c_uint
    {
        let mut str = luaL_checklstring(L, 3 as std::ffi::c_int, NULL_0 as *mut size_t);
        let mut value = WEBKIT_HARDWARE_ACCELERATION_POLICY_ON_DEMAND;
        if strcmp(str, b"on-demand\0" as *const u8 as *const std::ffi::c_char)
            == 0 as std::ffi::c_int
        {
            value = WEBKIT_HARDWARE_ACCELERATION_POLICY_ON_DEMAND;
        } else if strcmp(str, b"always\0" as *const u8 as *const std::ffi::c_char)
            == 0 as std::ffi::c_int
        {
            value = WEBKIT_HARDWARE_ACCELERATION_POLICY_ALWAYS;
        } else if strcmp(str, b"never\0" as *const u8 as *const std::ffi::c_char)
            == 0 as std::ffi::c_int
        {
            value = WEBKIT_HARDWARE_ACCELERATION_POLICY_NEVER;
        } else {
            return luaL_error(
                L,
                b"invalid value (expected one of 'on-demand', 'always', 'never')\0"
                    as *const u8 as *const std::ffi::c_char,
            )
        }
        webkit_settings_set_hardware_acceleration_policy(
            webkit_web_view_get_settings((*d).view),
            value,
        );
        emit = TRUE;
    }
    if emit == 0 {
        emit = luaH_gobject_newindex(
            L,
            webview_settings_properties.as_mut_ptr(),
            token,
            3 as std::ffi::c_int,
            g_type_check_instance_cast(
                webkit_web_view_get_settings((*d).view) as *mut GTypeInstance,
                ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
            ) as *mut std::ffi::c_void as *mut GObject,
        );
    }
    if emit != 0 {
        return luaH_object_property_signal(L, 1 as std::ffi::c_int, token);
    }
    return luaL_error(
        L,
        b"cannot set unknown webview property '%s'\0" as *const u8
            as *const std::ffi::c_char,
        lua_tolstring(L, 2 as std::ffi::c_int, NULL_0 as *mut size_t),
    );
}
unsafe extern "C" fn expose_cb(
    mut UNUSED_widget: *mut GtkWidget,
    mut UNUSED_e: *mut cairo_t,
    mut w: *mut widget_t,
) -> gboolean {
    let mut L = common.L;
    luaH_object_push(L, (*w).ref_0);
    luaH_object_emit_signal(
        L,
        -(1 as std::ffi::c_int),
        b"expose\0" as *const u8 as *const std::ffi::c_char,
        0 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    return FALSE;
}
unsafe extern "C" fn mouse_target_changed_cb(
    mut UNUSED_v: *mut WebKitWebView,
    mut htr: *mut WebKitHitTestResult,
    mut UNUSED_modifiers: guint,
    mut w: *mut widget_t,
) {
    let mut L = common.L;
    let mut d = (*w).data as *mut webview_data_t;
    (*d).htr_context = webkit_hit_test_result_get_context(htr);
    let mut link = NULL_0 as *const std::ffi::c_char;
    if webkit_hit_test_result_context_is_link(htr) != 0 {
        link = webkit_hit_test_result_get_link_uri(htr);
    }
    if !((*d).hover).is_null() && !link.is_null() && strcmp((*d).hover, link) == 0 {
        return;
    }
    luaH_object_push(L, (*w).ref_0);
    if !((*d).hover).is_null() {
        lua_pushstring(L, (*d).hover);
        g_free((*d).hover as gpointer);
        luaH_object_emit_signal(
            L,
            -(2 as std::ffi::c_int),
            b"link-unhover\0" as *const u8 as *const std::ffi::c_char,
            1 as std::ffi::c_int,
            0 as std::ffi::c_int,
        );
    }
    if !link.is_null() {
        (*d).hover = g_strdup_inline(link);
        lua_pushstring(L, (*d).hover);
        luaH_object_emit_signal(
            L,
            -(2 as std::ffi::c_int),
            b"link-hover\0" as *const u8 as *const std::ffi::c_char,
            1 as std::ffi::c_int,
            0 as std::ffi::c_int,
        );
        luaH_object_emit_signal(
            L,
            -(1 as std::ffi::c_int),
            b"property::hovered_uri\0" as *const u8 as *const std::ffi::c_char,
            0 as std::ffi::c_int,
            0 as std::ffi::c_int,
        );
    } else {
        (*d).hover = NULL_0 as *mut gchar;
    }
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
}
unsafe extern "C" fn luaH_push_hit_test(
    mut L: *mut lua_State,
    mut UNUSED_v: *mut WebKitWebView,
    mut w: *mut widget_t,
) -> gint {
    let mut c = (*((*w).data as *mut webview_data_t)).htr_context;
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    let mut name = 0 as *const gchar;
    if c & WEBKIT_HIT_TEST_RESULT_CONTEXT_DOCUMENT as std::ffi::c_int as guint != 0 {
        name = b"document\0" as *const u8 as *const std::ffi::c_char;
        lua_pushstring(L, name);
        lua_pushboolean(L, TRUE);
        lua_rawset(L, -(3 as std::ffi::c_int));
    }
    if c & WEBKIT_HIT_TEST_RESULT_CONTEXT_LINK as std::ffi::c_int as guint != 0 {
        name = b"link\0" as *const u8 as *const std::ffi::c_char;
        lua_pushstring(L, name);
        lua_pushboolean(L, TRUE);
        lua_rawset(L, -(3 as std::ffi::c_int));
    }
    if c & WEBKIT_HIT_TEST_RESULT_CONTEXT_IMAGE as std::ffi::c_int as guint != 0 {
        name = b"image\0" as *const u8 as *const std::ffi::c_char;
        lua_pushstring(L, name);
        lua_pushboolean(L, TRUE);
        lua_rawset(L, -(3 as std::ffi::c_int));
    }
    if c & WEBKIT_HIT_TEST_RESULT_CONTEXT_MEDIA as std::ffi::c_int as guint != 0 {
        name = b"media\0" as *const u8 as *const std::ffi::c_char;
        lua_pushstring(L, name);
        lua_pushboolean(L, TRUE);
        lua_rawset(L, -(3 as std::ffi::c_int));
    }
    if c & WEBKIT_HIT_TEST_RESULT_CONTEXT_EDITABLE as std::ffi::c_int as guint != 0 {
        name = b"editable\0" as *const u8 as *const std::ffi::c_char;
        lua_pushstring(L, name);
        lua_pushboolean(L, TRUE);
        lua_rawset(L, -(3 as std::ffi::c_int));
    }
    if c & WEBKIT_HIT_TEST_RESULT_CONTEXT_SCROLLBAR as std::ffi::c_int as guint != 0 {
        name = b"scrollbar\0" as *const u8 as *const std::ffi::c_char;
        lua_pushstring(L, name);
        lua_pushboolean(L, TRUE);
        lua_rawset(L, -(3 as std::ffi::c_int));
    }
    return 1 as std::ffi::c_int;
}
unsafe extern "C" fn webview_button_cb(
    mut view: *mut GtkWidget,
    mut ev: *mut GdkEventButton,
    mut w: *mut widget_t,
) -> gboolean {
    let mut ret: gint = 0;
    let mut L = common.L;
    luaH_object_push(L, (*w).ref_0);
    luaH_modifier_table_push(L, (*ev).state);
    lua_pushinteger(L, (*ev).button as lua_Integer);
    luaH_push_hit_test(
        L,
        g_type_check_instance_cast(
            view as *mut GTypeInstance,
            webkit_web_view_get_type(),
        ) as *mut std::ffi::c_void as *mut WebKitWebView,
        w,
    );
    match (*ev).type_0 as std::ffi::c_int {
        5 => {
            ret = luaH_object_emit_signal(
                L,
                -(4 as std::ffi::c_int),
                b"button-double-click\0" as *const u8 as *const std::ffi::c_char,
                3 as std::ffi::c_int,
                1 as std::ffi::c_int,
            );
        }
        7 => {
            ret = luaH_object_emit_signal(
                L,
                -(4 as std::ffi::c_int),
                b"button-release\0" as *const u8 as *const std::ffi::c_char,
                3 as std::ffi::c_int,
                1 as std::ffi::c_int,
            );
        }
        _ => {
            ret = luaH_object_emit_signal(
                L,
                -(4 as std::ffi::c_int),
                b"button-press\0" as *const u8 as *const std::ffi::c_char,
                3 as std::ffi::c_int,
                1 as std::ffi::c_int,
            );
        }
    }
    if ret != 0 && lua_toboolean(L, -(1 as std::ffi::c_int)) != 0 {
        lua_settop(L, -(ret + 1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        return TRUE;
    }
    lua_settop(L, -(ret + 1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    return FALSE;
}
unsafe extern "C" fn webview_scroll_cb(
    mut view: *mut GtkWidget,
    mut ev: *mut GdkEventScroll,
    mut w: *mut widget_t,
) -> gboolean {
    let mut dx: std::ffi::c_double = 0.;
    let mut dy: std::ffi::c_double = 0.;
    match (*ev).direction as std::ffi::c_uint {
        0 => {
            dx = 0 as std::ffi::c_int as std::ffi::c_double;
            dy = -(1 as std::ffi::c_int) as std::ffi::c_double;
        }
        1 => {
            dx = 0 as std::ffi::c_int as std::ffi::c_double;
            dy = 1 as std::ffi::c_int as std::ffi::c_double;
        }
        2 => {
            dx = -(1 as std::ffi::c_int) as std::ffi::c_double;
            dy = 0 as std::ffi::c_int as std::ffi::c_double;
        }
        3 => {
            dx = 1 as std::ffi::c_int as std::ffi::c_double;
            dy = 0 as std::ffi::c_int as std::ffi::c_double;
        }
        4 => {
            gdk_event_get_scroll_deltas(ev as *mut GdkEvent, &mut dx, &mut dy);
        }
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN as *const std::ffi::c_char,
                b"widgets/webview.c\0" as *const u8 as *const std::ffi::c_char,
                1091 as std::ffi::c_int,
                (*::core::mem::transmute::<
                    &[u8; 18],
                    &[std::ffi::c_char; 18],
                >(b"webview_scroll_cb\0"))
                    .as_ptr(),
                NULL_0 as *const std::ffi::c_char,
            );
        }
    }
    let mut L = common.L;
    luaH_object_push(L, (*w).ref_0);
    luaH_modifier_table_push(L, (*ev).state);
    lua_pushnumber(L, dx);
    lua_pushnumber(L, dy);
    luaH_push_hit_test(
        L,
        g_type_check_instance_cast(
            view as *mut GTypeInstance,
            webkit_web_view_get_type(),
        ) as *mut std::ffi::c_void as *mut WebKitWebView,
        w,
    );
    let mut ret = luaH_object_emit_signal(
        L,
        -(5 as std::ffi::c_int),
        b"scroll\0" as *const u8 as *const std::ffi::c_char,
        4 as std::ffi::c_int,
        1 as std::ffi::c_int,
    );
    lua_settop(L, -(ret + 1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    return ret;
}
unsafe extern "C" fn menu_item_cb(mut action: *mut GtkAction, mut w: *mut widget_t) {
    let mut L = common.L;
    let mut ref_0 = g_object_get_data(
        g_type_check_instance_cast(
            action as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject,
        b"lua_callback\0" as *const u8 as *const std::ffi::c_char,
    );
    luaH_object_push(L, (*w).ref_0);
    luaH_object_push(L, ref_0);
    luaH_dofunction(L, 1 as std::ffi::c_int, 0 as std::ffi::c_int);
}
unsafe extern "C" fn hide_popup_cb(
    mut UNUSED_v: *mut WebKitWebView,
    mut UNUSED_w: *mut widget_t,
) {
    let mut iter = 0 as *mut GSList;
    let mut L = common.L;
    if !(last_popup.old_refs).is_null() {
        iter = last_popup.old_refs;
        while !iter.is_null() {
            luaH_object_unref(L, (*iter).data);
            iter = (*iter).next;
        }
        g_slist_free(last_popup.old_refs);
        last_popup.old_refs = NULL_0 as *mut GSList;
    }
}
static mut context_menu_actions: *mut GSList = 0 as *const GSList as *mut GSList;
unsafe extern "C" fn table_from_context_menu(
    mut L: *mut lua_State,
    mut menu: *mut WebKitContextMenu,
    mut w: *mut widget_t,
) -> std::ffi::c_int {
    let mut len = webkit_context_menu_get_n_items(menu);
    lua_createtable(L, len as std::ffi::c_int, 0 as std::ffi::c_int);
    let mut i = 1 as std::ffi::c_int as guint;
    while i <= len {
        let mut item = webkit_context_menu_get_item_at_position(
            menu,
            i.wrapping_sub(1 as std::ffi::c_int as guint),
        );
        if webkit_context_menu_item_is_separator(item) != 0 {
            lua_pushboolean(L, TRUE);
        } else {
            let mut action = webkit_context_menu_item_get_action(item);
            let mut stock_action = webkit_context_menu_item_get_stock_action(item);
            let mut submenu = webkit_context_menu_item_get_submenu(item);
            lua_createtable(L, 2 as std::ffi::c_int, 0 as std::ffi::c_int);
            lua_pushstring(L, gtk_action_get_label(action));
            lua_rawseti(L, -(2 as std::ffi::c_int), 1 as std::ffi::c_int);
            if !submenu.is_null() {
                table_from_context_menu(L, submenu, w);
            } else if stock_action as std::ffi::c_uint
                == WEBKIT_CONTEXT_MENU_ACTION_CUSTOM as std::ffi::c_int
                    as std::ffi::c_uint
            {
                context_menu_actions = g_slist_prepend(
                    context_menu_actions,
                    action as gpointer,
                );
                g_object_ref(
                    g_type_check_instance_cast(
                        action as *mut GTypeInstance,
                        ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
                    ) as *mut std::ffi::c_void as *mut GObject as gpointer,
                );
                lua_pushlightuserdata(L, action as *mut std::ffi::c_void);
            } else {
                lua_pushinteger(L, stock_action as lua_Integer);
            }
            lua_rawseti(L, -(2 as std::ffi::c_int), 2 as std::ffi::c_int);
        }
        lua_rawseti(L, -(2 as std::ffi::c_int), i as std::ffi::c_int);
        i = i.wrapping_add(1);
        i;
    }
    return 1 as std::ffi::c_int;
}
unsafe extern "C" fn context_menu_from_table(
    mut L: *mut lua_State,
    mut menu: *mut WebKitContextMenu,
    mut w: *mut widget_t,
) {
    let mut item = 0 as *mut WebKitContextMenuItem;
    let mut submenu = 0 as *mut WebKitContextMenu;
    let mut ref_0 = 0 as *mut std::ffi::c_void;
    let mut label = 0 as *const gchar;
    let mut i: gint = 0;
    let mut len = lua_objlen(L, -(1 as std::ffi::c_int)) as gint;
    i = 1 as std::ffi::c_int;
    while i <= len {
        lua_rawgeti(L, -(1 as std::ffi::c_int), i);
        if lua_type(L, -(1 as std::ffi::c_int)) == LUA_TTABLE
            && lua_objlen(L, -(1 as std::ffi::c_int)) >= 2 as std::ffi::c_int as size_t
        {
            lua_rawgeti(L, -(1 as std::ffi::c_int), 1 as std::ffi::c_int);
            label = lua_tolstring(L, -(1 as std::ffi::c_int), NULL_0 as *mut size_t);
            lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
            lua_rawgeti(L, -(1 as std::ffi::c_int), 2 as std::ffi::c_int);
            if lua_type(L, -(1 as std::ffi::c_int)) == LUA_TTABLE {
                submenu = webkit_context_menu_new();
                item = webkit_context_menu_item_new_with_submenu(label, submenu);
                webkit_context_menu_append(menu, item);
                context_menu_from_table(L, submenu, w);
                lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
            } else if lua_type(L, -(1 as std::ffi::c_int)) == LUA_TFUNCTION {
                let mut action = gtk_action_new(
                    label,
                    label,
                    NULL_0 as *const gchar,
                    NULL_0 as *const gchar,
                );
                item = webkit_context_menu_item_new(action);
                ref_0 = luaH_object_ref(L, -(1 as std::ffi::c_int));
                last_popup.refs = g_slist_prepend(last_popup.refs, ref_0);
                g_object_set_data(
                    g_type_check_instance_cast(
                        action as *mut GTypeInstance,
                        ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
                    ) as *mut std::ffi::c_void as *mut GObject,
                    b"lua_callback\0" as *const u8 as *const std::ffi::c_char,
                    ref_0,
                );
                webkit_context_menu_append(menu, item);
                g_signal_connect_data(
                    action as gpointer,
                    b"activate\0" as *const u8 as *const std::ffi::c_char,
                    ::core::mem::transmute::<
                        Option::<
                            unsafe extern "C" fn(*mut GtkAction, *mut widget_t) -> (),
                        >,
                        GCallback,
                    >(
                        Some(
                            menu_item_cb
                                as unsafe extern "C" fn(*mut GtkAction, *mut widget_t) -> (),
                        ),
                    ),
                    w as gpointer,
                    ::core::mem::transmute::<
                        libc::intptr_t,
                        GClosureNotify,
                    >(NULL_0 as libc::intptr_t),
                    G_CONNECT_DEFAULT,
                );
            } else if lua_type(L, -(1 as std::ffi::c_int)) == LUA_TNUMBER {
                let mut stock_action = lua_tointeger(L, -(1 as std::ffi::c_int))
                    as WebKitContextMenuAction;
                let mut __n1 = stock_action as gint64;
                let mut __n2 = WEBKIT_CONTEXT_MENU_ACTION_CUSTOM as std::ffi::c_int
                    as gint64;
                if !(__n1 != __n2) {
                    g_assertion_message_cmpint(
                        G_LOG_DOMAIN as *const std::ffi::c_char,
                        b"widgets/webview.c\0" as *const u8 as *const std::ffi::c_char,
                        1216 as std::ffi::c_int,
                        (*::core::mem::transmute::<
                            &[u8; 24],
                            &[std::ffi::c_char; 24],
                        >(b"context_menu_from_table\0"))
                            .as_ptr(),
                        b"stock_action != WEBKIT_CONTEXT_MENU_ACTION_CUSTOM\0"
                            as *const u8 as *const std::ffi::c_char,
                        __n1 as guint64,
                        b"!=\0" as *const u8 as *const std::ffi::c_char,
                        __n2 as guint64,
                        'i' as i32 as std::ffi::c_char,
                    );
                }
                item = webkit_context_menu_item_new_from_stock_action_with_label(
                    stock_action,
                    label,
                );
                webkit_context_menu_append(menu, item);
                lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
            } else if lua_type(L, -(1 as std::ffi::c_int)) == LUA_TLIGHTUSERDATA {
                let mut action_0 = lua_topointer(L, -(1 as std::ffi::c_int))
                    as *mut std::ffi::c_void as *mut GtkAction;
                item = webkit_context_menu_item_new(action_0);
                webkit_context_menu_append(menu, item);
                lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
            }
        } else if lua_type(L, -(1 as std::ffi::c_int)) == LUA_TBOOLEAN
            && lua_toboolean(L, -(1 as std::ffi::c_int)) != 0
        {
            item = webkit_context_menu_item_new_separator();
            webkit_context_menu_append(menu, item);
        }
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        i += 1;
        i;
    }
}
unsafe extern "C" fn context_menu_cb(
    mut UNUSED_v: *mut WebKitWebView,
    mut menu: *mut WebKitContextMenu,
    mut UNUSED_e: *mut GdkEvent,
    mut UNUSED_htr: *mut WebKitHitTestResult,
    mut w: *mut widget_t,
) -> gboolean {
    let mut L = common.L;
    if context_menu_actions.is_null() {} else {
        g_assertion_message_expr(
            G_LOG_DOMAIN as *const std::ffi::c_char,
            b"widgets/webview.c\0" as *const u8 as *const std::ffi::c_char,
            1244 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 16],
                &[std::ffi::c_char; 16],
            >(b"context_menu_cb\0"))
                .as_ptr(),
            b"!context_menu_actions\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    table_from_context_menu(L, menu, w);
    luaH_object_push(L, (*w).ref_0);
    lua_pushvalue(L, -(2 as std::ffi::c_int));
    luaH_object_emit_signal(
        L,
        -(2 as std::ffi::c_int),
        b"populate-popup\0" as *const u8 as *const std::ffi::c_char,
        1 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    last_popup.old_refs = last_popup.refs;
    last_popup.refs = NULL_0 as *mut GSList;
    webkit_context_menu_remove_all(menu);
    context_menu_from_table(L, menu, w);
    g_slist_free_full(
        context_menu_actions,
        Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
    );
    context_menu_actions = NULL_0 as *mut GSList;
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    return FALSE;
}
unsafe extern "C" fn webview_destructor(mut w: *mut widget_t) {
    let mut d = (*w).data as *mut webview_data_t;
    g_idle_remove_by_data(w as gpointer);
    if !((*d).ipc).is_null() {} else {
        g_assertion_message_expr(
            G_LOG_DOMAIN as *const std::ffi::c_char,
            b"widgets/webview.c\0" as *const u8 as *const std::ffi::c_char,
            1272 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 19],
                &[std::ffi::c_char; 19],
            >(b"webview_destructor\0"))
                .as_ptr(),
            b"d->ipc\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    ipc_endpoint_decref((*d).ipc);
    (*d).ipc = NULL_0 as *mut ipc_endpoint_t;
    g_ptr_array_remove(globalconf.webviews, w as gpointer);
    g_free((*d).uri as gpointer);
    g_free((*d).hover as gpointer);
    g_object_unref(
        g_type_check_instance_cast(
            (*d).user_content as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject as gpointer,
    );
    if !((*d).cert).is_null() {
        g_object_unref(
            g_type_check_instance_cast(
                (*d).cert as *mut GTypeInstance,
                ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
            ) as *mut std::ffi::c_void as *mut GObject as gpointer,
        );
    }
    g_slice_free1(
        ::core::mem::size_of::<webview_data_t>() as std::ffi::c_ulong,
        d as gpointer,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luakit_uri_scheme_request_cb(
    mut request: *mut WebKitURISchemeRequest,
    mut scheme: *const gchar,
) {
    let mut uri = webkit_uri_scheme_request_get_uri(request);
    let mut view = webkit_uri_scheme_request_get_web_view(request);
    if view.is_null() {
        return;
    }
    let mut w = g_object_get_data(
        g_type_check_instance_cast(
            view as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject,
        GOBJECT_LUAKIT_WIDGET_DATA_KEY.as_ptr(),
    ) as *mut widget_t;
    let mut L = common.L;
    if !scheme.is_null() {} else {
        g_assertion_message_expr(
            G_LOG_DOMAIN as *const std::ffi::c_char,
            b"widgets/webview.c\0" as *const u8 as *const std::ffi::c_char,
            1298 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[std::ffi::c_char; 29],
            >(b"luakit_uri_scheme_request_cb\0"))
                .as_ptr(),
            b"scheme\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    let mut sig = g_strconcat(
        b"scheme-request::\0" as *const u8 as *const std::ffi::c_char,
        scheme,
        NULL_0 as *mut std::ffi::c_void,
    );
    luaH_object_push(L, (*w).ref_0);
    lua_pushstring(L, uri);
    luaH_request_push_uri_scheme_request(L, request);
    luaH_object_emit_signal(
        L,
        -(3 as std::ffi::c_int),
        sig,
        2 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    g_free(sig as gpointer);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn webview_crashed_cb(
    mut UNUSED_view: *mut WebKitWebView,
    mut w: *mut widget_t,
) -> gboolean {
    let mut d = (*w).data as *mut webview_data_t;
    (*d).ipc = ipc_endpoint_new(b"UI\0" as *const u8 as *const std::ffi::c_char);
    let mut L = common.L;
    luaH_object_push(L, (*w).ref_0);
    luaH_object_emit_signal(
        L,
        -(1 as std::ffi::c_int),
        b"crashed\0" as *const u8 as *const std::ffi::c_char,
        0 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn webview_connect_to_endpoint(
    mut w: *mut widget_t,
    mut ipc: *mut ipc_endpoint_t,
) {
    if (*(*w).info).tok as std::ffi::c_uint
        == L_TK_WEBVIEW as std::ffi::c_int as std::ffi::c_uint
    {} else {
        g_assertion_message_expr(
            G_LOG_DOMAIN as *const std::ffi::c_char,
            b"widgets/webview.c\0" as *const u8 as *const std::ffi::c_char,
            1327 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[std::ffi::c_char; 28],
            >(b"webview_connect_to_endpoint\0"))
                .as_ptr(),
            b"w->info->tok == L_TK_WEBVIEW\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    if !ipc.is_null() {} else {
        g_assertion_message_expr(
            G_LOG_DOMAIN as *const std::ffi::c_char,
            b"widgets/webview.c\0" as *const u8 as *const std::ffi::c_char,
            1328 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[std::ffi::c_char; 28],
            >(b"webview_connect_to_endpoint\0"))
                .as_ptr(),
            b"ipc\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    let mut d = (*w).data as *mut webview_data_t;
    (*d).ipc = ipc_endpoint_replace((*d).ipc, ipc);
    let mut L = common.L;
    if (*ipc).creation_notified == 0 {
        (*ipc).creation_notified = TRUE;
        let mut top = lua_gettop(L);
        luaH_object_push(L, (*w).ref_0);
        let mut luakit_class = luakit_lib_get_luakit_class();
        luaH_class_emit_signal(
            L,
            luakit_class,
            b"web-extension-created\0" as *const u8 as *const std::ffi::c_char,
            1 as std::ffi::c_int,
            0 as std::ffi::c_int,
        );
        lua_settop(L, top);
    }
    luaH_object_push(L, (*w).ref_0);
    if !(lua_type(L, -(1 as std::ffi::c_int)) == LUA_TNIL) {
        luaH_object_emit_signal(
            L,
            -(1 as std::ffi::c_int),
            b"web-extension-loaded\0" as *const u8 as *const std::ffi::c_char,
            0 as std::ffi::c_int,
            0 as std::ffi::c_int,
        );
    }
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn webview_get_endpoint(
    mut w: *mut widget_t,
) -> *mut ipc_endpoint_t {
    if (*(*w).info).tok as std::ffi::c_uint
        == L_TK_WEBVIEW as std::ffi::c_int as std::ffi::c_uint
    {} else {
        g_assertion_message_expr(
            G_LOG_DOMAIN as *const std::ffi::c_char,
            b"widgets/webview.c\0" as *const u8 as *const std::ffi::c_char,
            1358 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 21],
                &[std::ffi::c_char; 21],
            >(b"webview_get_endpoint\0"))
                .as_ptr(),
            b"w->info->tok == L_TK_WEBVIEW\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    let mut d = (*w).data as *mut webview_data_t;
    return (*d).ipc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn webview_set_web_process_id(
    mut w: *mut widget_t,
    mut pid: pid_t,
) {
    let mut d = (*w).data as *mut webview_data_t;
    (*d).web_process_id = pid;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn widget_webview(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
    mut UNUSED_token: luakit_token_t,
) -> *mut widget_t {
    (*w)
        .index = Some(
        luaH_webview_index
            as unsafe extern "C" fn(
                *mut lua_State,
                *mut widget_t,
                luakit_token_t,
            ) -> gint,
    );
    (*w)
        .newindex = Some(
        luaH_webview_newindex
            as unsafe extern "C" fn(
                *mut lua_State,
                *mut widget_t,
                luakit_token_t,
            ) -> gint,
    );
    (*w)
        .destructor = Some(
        webview_destructor as unsafe extern "C" fn(*mut widget_t) -> (),
    );
    let mut d = g_slice_alloc0(
        ::core::mem::size_of::<webview_data_t>() as std::ffi::c_ulong,
    ) as *mut webview_data_t;
    (*d).widget = w;
    (*w).data = d as gpointer;
    let mut prop_tbl_idx = luaH_absindex(L, -(4 as std::ffi::c_int));
    if lua_type(L, prop_tbl_idx) == 5 as std::ffi::c_int {} else {
        g_assertion_message_expr(
            G_LOG_DOMAIN as *const std::ffi::c_char,
            b"widgets/webview.c\0" as *const u8 as *const std::ffi::c_char,
            1385 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 15],
                &[std::ffi::c_char; 15],
            >(b"widget_webview\0"))
                .as_ptr(),
            b"lua_istable(L, prop_tbl_idx)\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    lua_pushstring(L, b"private\0" as *const u8 as *const std::ffi::c_char);
    lua_rawget(L, prop_tbl_idx);
    let mut private = if lua_type(L, -(1 as std::ffi::c_int)) == LUA_TNIL {
        FALSE
    } else {
        lua_toboolean(L, -(1 as std::ffi::c_int))
    };
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    (*d).private = private;
    if (globalconf.webviews).is_null() {
        globalconf.webviews = g_ptr_array_new();
    }
    if (globalconf.stylesheets).is_null() {
        globalconf.stylesheets = g_ptr_array_new();
    }
    (*d).stylesheets = NULL_0 as *mut GList;
    web_context_init_finish();
    (*d).user_content = webkit_user_content_manager_new();
    (*d)
        .view = g_object_new(
        webkit_web_view_get_type(),
        b"web-context\0" as *const u8 as *const std::ffi::c_char,
        web_context_get(),
        b"is-ephemeral\0" as *const u8 as *const std::ffi::c_char,
        (*d).private,
        b"user-content-manager\0" as *const u8 as *const std::ffi::c_char,
        (*d).user_content,
        if !related_view.is_null() {
            b"related-view\0" as *const u8 as *const std::ffi::c_char
        } else {
            NULL_0 as *const std::ffi::c_char
        },
        related_view,
        NULL_0 as *mut std::ffi::c_void,
    ) as *mut WebKitWebView;
    (*d).inspector = webkit_web_view_get_inspector((*d).view);
    (*d).is_committed = FALSE;
    (*d).ipc = ipc_endpoint_new(b"UI\0" as *const u8 as *const std::ffi::c_char);
    (*w)
        .widget = g_type_check_instance_cast(
        (*d).view as *mut GTypeInstance,
        gtk_widget_get_type(),
    ) as *mut std::ffi::c_void as *mut GtkWidget;
    g_ptr_array_add(globalconf.webviews, w as gpointer);
    g_object_connect(
        g_type_check_instance_cast(
            (*d).view as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject as gpointer,
        b"signal::destroy\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GtkWidget, *mut widget_t) -> ()>,
            GCallback,
        >(Some(destroy_cb as unsafe extern "C" fn(*mut GtkWidget, *mut widget_t) -> ())),
        w,
        b"signal::size-allocate\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut GdkRectangle,
                    *mut widget_t,
                ) -> (),
            >,
            GCallback,
        >(
            Some(
                resize_cb
                    as unsafe extern "C" fn(
                        *mut GtkWidget,
                        *mut GdkRectangle,
                        *mut widget_t,
                    ) -> (),
            ),
        ),
        w,
        b"signal::focus-in-event\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut GdkEventFocus,
                    *mut widget_t,
                ) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                focus_cb
                    as unsafe extern "C" fn(
                        *mut GtkWidget,
                        *mut GdkEventFocus,
                        *mut widget_t,
                    ) -> gboolean,
            ),
        ),
        w,
        b"signal::focus-out-event\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut GdkEventFocus,
                    *mut widget_t,
                ) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                focus_cb
                    as unsafe extern "C" fn(
                        *mut GtkWidget,
                        *mut GdkEventFocus,
                        *mut widget_t,
                    ) -> gboolean,
            ),
        ),
        w,
        b"signal::parent-set\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut GtkWidget, *mut GtkWidget, *mut widget_t) -> (),
            >,
            GCallback,
        >(
            Some(
                parent_set_cb
                    as unsafe extern "C" fn(
                        *mut GtkWidget,
                        *mut GtkWidget,
                        *mut widget_t,
                    ) -> (),
            ),
        ),
        w,
        b"signal::button-press-event\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut GdkEventButton,
                    *mut widget_t,
                ) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                webview_button_cb
                    as unsafe extern "C" fn(
                        *mut GtkWidget,
                        *mut GdkEventButton,
                        *mut widget_t,
                    ) -> gboolean,
            ),
        ),
        w,
        b"signal::button-release-event\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut GdkEventButton,
                    *mut widget_t,
                ) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                webview_button_cb
                    as unsafe extern "C" fn(
                        *mut GtkWidget,
                        *mut GdkEventButton,
                        *mut widget_t,
                    ) -> gboolean,
            ),
        ),
        w,
        b"signal::scroll-event\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut GdkEventScroll,
                    *mut widget_t,
                ) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                webview_scroll_cb
                    as unsafe extern "C" fn(
                        *mut GtkWidget,
                        *mut GdkEventScroll,
                        *mut widget_t,
                    ) -> gboolean,
            ),
        ),
        w,
        b"signal::create\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut WebKitWebView,
                    *mut WebKitNavigationAction,
                    *mut widget_t,
                ) -> *mut GtkWidget,
            >,
            GCallback,
        >(
            Some(
                create_cb
                    as unsafe extern "C" fn(
                        *mut WebKitWebView,
                        *mut WebKitNavigationAction,
                        *mut widget_t,
                    ) -> *mut GtkWidget,
            ),
        ),
        w,
        b"signal::web-process-crashed\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut WebKitWebView, *mut widget_t) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                webview_crashed_cb
                    as unsafe extern "C" fn(
                        *mut WebKitWebView,
                        *mut widget_t,
                    ) -> gboolean,
            ),
        ),
        w,
        b"signal::draw\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut cairo_t,
                    *mut widget_t,
                ) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                expose_cb
                    as unsafe extern "C" fn(
                        *mut GtkWidget,
                        *mut cairo_t,
                        *mut widget_t,
                    ) -> gboolean,
            ),
        ),
        w,
        b"signal::mouse-target-changed\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut WebKitWebView,
                    *mut WebKitHitTestResult,
                    guint,
                    *mut widget_t,
                ) -> (),
            >,
            GCallback,
        >(
            Some(
                mouse_target_changed_cb
                    as unsafe extern "C" fn(
                        *mut WebKitWebView,
                        *mut WebKitHitTestResult,
                        guint,
                        *mut widget_t,
                    ) -> (),
            ),
        ),
        w,
        b"signal::key-press-event\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut GtkWidget,
                    *mut GdkEventKey,
                    *mut widget_t,
                ) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                key_press_cb
                    as unsafe extern "C" fn(
                        *mut GtkWidget,
                        *mut GdkEventKey,
                        *mut widget_t,
                    ) -> gboolean,
            ),
        ),
        w,
        b"signal::decide-policy\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut WebKitWebView,
                    *mut WebKitPolicyDecision,
                    WebKitPolicyDecisionType,
                    *mut widget_t,
                ) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                decide_policy_cb
                    as unsafe extern "C" fn(
                        *mut WebKitWebView,
                        *mut WebKitPolicyDecision,
                        WebKitPolicyDecisionType,
                        *mut widget_t,
                    ) -> gboolean,
            ),
        ),
        w,
        b"signal::notify\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut WebKitWebView,
                    *mut GParamSpec,
                    *mut widget_t,
                ) -> (),
            >,
            GCallback,
        >(
            Some(
                notify_cb
                    as unsafe extern "C" fn(
                        *mut WebKitWebView,
                        *mut GParamSpec,
                        *mut widget_t,
                    ) -> (),
            ),
        ),
        w,
        b"signal::load-changed\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut WebKitWebView,
                    WebKitLoadEvent,
                    *mut widget_t,
                ) -> (),
            >,
            GCallback,
        >(
            Some(
                load_changed_cb
                    as unsafe extern "C" fn(
                        *mut WebKitWebView,
                        WebKitLoadEvent,
                        *mut widget_t,
                    ) -> (),
            ),
        ),
        w,
        b"signal::load-failed\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut WebKitWebView,
                    WebKitLoadEvent,
                    *mut gchar,
                    *mut GError,
                    *mut widget_t,
                ) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                load_failed_cb
                    as unsafe extern "C" fn(
                        *mut WebKitWebView,
                        WebKitLoadEvent,
                        *mut gchar,
                        *mut GError,
                        *mut widget_t,
                    ) -> gboolean,
            ),
        ),
        w,
        b"signal::load-failed-with-tls-errors\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut WebKitWebView,
                    *mut gchar,
                    *mut GTlsCertificate,
                    GTlsCertificateFlags,
                    *mut widget_t,
                ) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                load_failed_tls_cb
                    as unsafe extern "C" fn(
                        *mut WebKitWebView,
                        *mut gchar,
                        *mut GTlsCertificate,
                        GTlsCertificateFlags,
                        *mut widget_t,
                    ) -> gboolean,
            ),
        ),
        w,
        b"signal::context-menu\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut WebKitWebView,
                    *mut WebKitContextMenu,
                    *mut GdkEvent,
                    *mut WebKitHitTestResult,
                    *mut widget_t,
                ) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                context_menu_cb
                    as unsafe extern "C" fn(
                        *mut WebKitWebView,
                        *mut WebKitContextMenu,
                        *mut GdkEvent,
                        *mut WebKitHitTestResult,
                        *mut widget_t,
                    ) -> gboolean,
            ),
        ),
        w,
        b"signal::context-menu-dismissed\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut WebKitWebView, *mut widget_t) -> ()>,
            GCallback,
        >(
            Some(
                hide_popup_cb
                    as unsafe extern "C" fn(*mut WebKitWebView, *mut widget_t) -> (),
            ),
        ),
        w,
        b"signal::notify::favicon\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut WebKitWebView,
                    *mut GParamSpec,
                    *mut widget_t,
                ) -> (),
            >,
            GCallback,
        >(
            Some(
                favicon_cb
                    as unsafe extern "C" fn(
                        *mut WebKitWebView,
                        *mut GParamSpec,
                        *mut widget_t,
                    ) -> (),
            ),
        ),
        w,
        b"signal::notify::uri\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut WebKitWebView,
                    *mut GParamSpec,
                    *mut widget_t,
                ) -> (),
            >,
            GCallback,
        >(
            Some(
                uri_cb
                    as unsafe extern "C" fn(
                        *mut WebKitWebView,
                        *mut GParamSpec,
                        *mut widget_t,
                    ) -> (),
            ),
        ),
        w,
        b"signal::authenticate\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut WebKitWebView,
                    *mut WebKitAuthenticationRequest,
                    *mut widget_t,
                ) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                session_authenticate
                    as unsafe extern "C" fn(
                        *mut WebKitWebView,
                        *mut WebKitAuthenticationRequest,
                        *mut widget_t,
                    ) -> gboolean,
            ),
        ),
        w,
        b"signal::permission-request\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut WebKitWebView,
                    *mut WebKitPermissionRequest,
                    *mut widget_t,
                ) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                permission_request_cb
                    as unsafe extern "C" fn(
                        *mut WebKitWebView,
                        *mut WebKitPermissionRequest,
                        *mut widget_t,
                    ) -> gboolean,
            ),
        ),
        w,
        NULL_0 as *mut std::ffi::c_void,
    );
    g_object_connect(
        g_type_check_instance_cast(
            webkit_web_view_get_find_controller((*d).view) as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject as gpointer,
        b"signal::found-text\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut WebKitFindController,
                    guint,
                    *mut widget_t,
                ) -> (),
            >,
            GCallback,
        >(
            Some(
                found_text_cb
                    as unsafe extern "C" fn(
                        *mut WebKitFindController,
                        guint,
                        *mut widget_t,
                    ) -> (),
            ),
        ),
        w,
        b"signal::failed-to-find-text\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut WebKitFindController, *mut widget_t) -> (),
            >,
            GCallback,
        >(
            Some(
                failed_to_find_text_cb
                    as unsafe extern "C" fn(
                        *mut WebKitFindController,
                        *mut widget_t,
                    ) -> (),
            ),
        ),
        w,
        NULL_0 as *mut std::ffi::c_void,
    );
    g_object_connect(
        g_type_check_instance_cast(
            (*d).view as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject as gpointer,
        b"signal::parent-set\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut GtkWidget, *mut GtkWidget, *mut widget_t) -> (),
            >,
            GCallback,
        >(
            Some(
                parent_set_cb
                    as unsafe extern "C" fn(
                        *mut GtkWidget,
                        *mut GtkWidget,
                        *mut widget_t,
                    ) -> (),
            ),
        ),
        w,
        NULL_0 as *mut std::ffi::c_void,
    );
    g_object_connect(
        g_type_check_instance_cast(
            (*d).inspector as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject as gpointer,
        b"signal::attach\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut WebKitWebInspector, *mut widget_t) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                inspector_attach_window_cb
                    as unsafe extern "C" fn(
                        *mut WebKitWebInspector,
                        *mut widget_t,
                    ) -> gboolean,
            ),
        ),
        w,
        b"signal::bring-to-front\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut WebKitWebInspector, *mut widget_t) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                inspector_show_window_cb
                    as unsafe extern "C" fn(
                        *mut WebKitWebInspector,
                        *mut widget_t,
                    ) -> gboolean,
            ),
        ),
        w,
        b"signal::closed\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut WebKitWebInspector, *mut widget_t) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                inspector_close_window_cb
                    as unsafe extern "C" fn(
                        *mut WebKitWebInspector,
                        *mut widget_t,
                    ) -> gboolean,
            ),
        ),
        w,
        b"signal::detach\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut WebKitWebInspector, *mut widget_t) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                inspector_detach_window_cb
                    as unsafe extern "C" fn(
                        *mut WebKitWebInspector,
                        *mut widget_t,
                    ) -> gboolean,
            ),
        ),
        w,
        b"signal::open-window\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut WebKitWebInspector, *mut widget_t) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                inspector_open_window_cb
                    as unsafe extern "C" fn(
                        *mut WebKitWebInspector,
                        *mut widget_t,
                    ) -> gboolean,
            ),
        ),
        w,
        NULL_0 as *mut std::ffi::c_void,
    );
    gtk_widget_show(
        g_type_check_instance_cast(
            (*d).view as *mut GTypeInstance,
            gtk_widget_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkWidget,
    );
    return w;
}
