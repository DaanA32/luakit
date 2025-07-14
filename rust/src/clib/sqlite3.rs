use std::ffi::CStr;

use glib_sys::{g_free, g_strdup, gpointer};
use libc::{c_void, intptr_t, memset};
use mlua_sys::{
    LUA_MULTRET, LUA_TBOOLEAN, LUA_TNUMBER, LUA_TSTRING, LUA_TTABLE, lua_Number, lua_State,
    lua_createtable, lua_error, lua_gettop, lua_newuserdata, lua_next, lua_pushfstring,
    lua_pushlstring, lua_pushnil, lua_pushnumber, lua_pushstring, lua_pushvalue, lua_rawset,
    lua_rawseti, lua_setmetatable, lua_settop, lua_toboolean, lua_tointeger, lua_tolstring,
    lua_tonumber, lua_type, lua_typename, luaL_Reg, luaL_argerror, luaL_checklstring,
};
use sqlite3_sys::*;

use crate::common::luaclass::signal_h::{signal_new, signal_t};
use crate::common::luaclass::{
    lua_class_allocator_t, lua_class_property_array_t, lua_class_propfunc_t, lua_class_t,
    luaH_checkudata, luaH_class_add_property, luaH_class_add_signal, luaH_class_emit_signal,
    luaH_class_index, luaH_class_new, luaH_class_newindex, luaH_class_remove_signal,
    luaH_class_setup,
};
use crate::common::luaobject::{
    luaH_object_add_signal_simple, luaH_object_emit_signal_simple, luaH_object_gc,
    luaH_object_remove_signal_simple, luaH_object_remove_signals_simple, luaH_object_tostring,
    luaH_settype,
};
use crate::common::luaobject::{luaH_object_ref, luaH_object_unref};
use crate::common::tokenize::L_TK_FILENAME;
use crate::gtypes::{gchar, gint};
use crate::log::{_log, LOG_LEVEL_warn};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_stmt_t {
    pub sqlite: *mut sqlite3_t,
    pub stmt: *mut sqlite3_stmt,
    pub parent_ref: gpointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_t {
    pub signals: *mut signal_t,
    pub filename: *mut std::ffi::c_char,
    pub db: *mut sqlite3,
}
static mut sqlite3_class: lua_class_t = lua_class_t {
    name: 0 as *const gchar,
    signals: 0 as *const signal_t as *mut signal_t,
    allocator: None,
    properties: 0 as *const lua_class_property_array_t as *mut lua_class_property_array_t,
    index_miss_property: None,
    newindex_miss_property: None,
};
static mut sqlite3_stmt_class: lua_class_t = lua_class_t {
    name: 0 as *const gchar,
    signals: 0 as *const signal_t as *mut signal_t,
    allocator: None,
    properties: 0 as *const lua_class_property_array_t as *mut lua_class_property_array_t,
    index_miss_property: None,
    newindex_miss_property: None,
};
#[inline]
unsafe extern "C-unwind" fn luaH_sqlite3_class_emit_signal(mut L: *mut lua_State) -> gint {
    return luaH_class_emit_signal(
        L,
        &mut sqlite3_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, std::ptr::null_mut()),
        lua_gettop(L) - 1 as std::ffi::c_int,
        LUA_MULTRET,
    );
}
#[inline]
unsafe extern "C-unwind" fn luaH_sqlite3_class_remove_signal(mut L: *mut lua_State) -> gint {
    luaH_class_remove_signal(
        L,
        &mut sqlite3_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, std::ptr::null_mut()),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn luaH_sqlite3_class_add_signal(mut L: *mut lua_State) -> gint {
    luaH_class_add_signal(
        L,
        &mut sqlite3_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, std::ptr::null_mut()),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn sqlite3_new(mut L: *mut lua_State) -> *mut sqlite3_t {
    let mut p = lua_newuserdata(L, ::core::mem::size_of::<sqlite3_t>()) as *mut sqlite3_t;
    memset(
        p as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        (::core::mem::size_of::<sqlite3_t>()).wrapping_mul(1),
    );
    (*p).signals = signal_new();
    luaH_settype(L, &mut sqlite3_class);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_setmetatable(L, -(2 as std::ffi::c_int));
    println!("lua_setfenv(L, -(2 as std::ffi::c_int));");
    lua_pushvalue(L, -(1 as std::ffi::c_int));
    luaH_class_emit_signal(
        L,
        &mut sqlite3_class,
        b"new\0" as *const u8 as *const std::ffi::c_char,
        1 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    return p;
}
#[inline]
unsafe extern "C-unwind" fn luaH_sqlite3_checkopen(
    mut L: *mut lua_State,
    mut sqlite: *mut sqlite3_t,
) {
    if ((*sqlite).db).is_null() {
        lua_pushlstring(
            L,
            b"sqlite3: database handle closed\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 32]>())
                .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
                .wrapping_sub(1),
        );
        lua_error(L);
    }
}
unsafe extern "C-unwind" fn luaH_sqlite3_stmt_gc(mut L: *mut lua_State) -> gint {
    let mut stmt =
        luaH_checkudata(L, 1 as std::ffi::c_int, &mut sqlite3_stmt_class) as *mut sqlite3_stmt_t;
    luaH_object_unref(L, (*stmt).parent_ref);
    sqlite3_finalize((*stmt).stmt);
    return 0 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn sqlite3_stmt_new(mut L: *mut lua_State) -> *mut sqlite3_stmt_t {
    let mut p = lua_newuserdata(L, ::core::mem::size_of::<sqlite3_stmt_t>()) as *mut sqlite3_stmt_t;
    memset(
        p as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        (::core::mem::size_of::<sqlite3_stmt_t>()).wrapping_mul(1),
    );
    luaH_settype(L, &mut sqlite3_stmt_class);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_setmetatable(L, -(2 as std::ffi::c_int));
    println!("lua_setfenv(L, -(2 as std::ffi::c_int));");
    return p;
}
unsafe extern "C-unwind" fn luaH_sqlite3_compile(mut L: *mut lua_State) -> gint {
    let mut sqlite = luaH_checkudata(L, 1 as std::ffi::c_int, &mut sqlite3_class) as *mut sqlite3_t;
    luaH_sqlite3_checkopen(L, sqlite);
    let mut sql = luaL_checklstring(L, 2 as std::ffi::c_int, std::ptr::null_mut());
    let mut tail = 0 as *const gchar;
    let mut stmt = 0 as *mut sqlite3_stmt;
    if sqlite3_prepare_v2(
        (*sqlite).db,
        sql,
        -(1 as std::ffi::c_int),
        &mut stmt,
        &mut tail,
    ) != 0
    {
        lua_pushfstring(
            L,
            b"sqlite3: statement compilation failed (%s)\0" as *const u8 as *const std::ffi::c_char,
            sqlite3_errmsg((*sqlite).db),
        );
        sqlite3_finalize(stmt);
        lua_error(L);
    } else if stmt.is_null() {
        lua_pushfstring(
            L,
            b"sqlite3: no SQL found in string: \"%s\"\0" as *const u8 as *const std::ffi::c_char,
            sql,
        );
        lua_error(L);
    }
    let mut p = sqlite3_stmt_new(L);
    (*p).sqlite = sqlite;
    (*p).stmt = stmt;
    (*p).parent_ref = luaH_object_ref(L, 1 as std::ffi::c_int);
    if !tail.is_null() && *tail as std::ffi::c_int != 0 {
        lua_pushstring(L, tail);
        return 2 as std::ffi::c_int;
    }
    return 1 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_sqlite3_close(mut L: *mut lua_State) -> gint {
    let mut sqlite = luaH_checkudata(L, 1 as std::ffi::c_int, &mut sqlite3_class) as *mut sqlite3_t;
    if !((*sqlite).filename).is_null() {
        g_free((*sqlite).filename as gpointer);
        (*sqlite).filename = std::ptr::null_mut();
    }
    if !((*sqlite).db).is_null() {
        sqlite3_close((*sqlite).db);
        (*sqlite).db = std::ptr::null_mut();
    }
    return 0 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_sqlite3_gc(mut L: *mut lua_State) -> gint {
    luaH_sqlite3_close(L);
    return luaH_object_gc(L);
}
unsafe extern "C-unwind" fn luaH_sqlite3_set_filename(
    mut L: *mut lua_State,
    mut sqlite: *mut sqlite3_t,
) -> gint {
    let mut filename = luaL_checklstring(L, -(1 as std::ffi::c_int), std::ptr::null_mut());
    if sqlite3_open(filename, &mut (*sqlite).db) != 0 {
        lua_pushfstring(
            L,
            b"sqlite3: failed to open \"%s\" (%s)\0" as *const u8 as *const std::ffi::c_char,
            filename,
            sqlite3_errmsg((*sqlite).db),
        );
        sqlite3_close((*sqlite).db);
        lua_error(L);
    }
    (*sqlite).filename = g_strdup(filename);
    return 0 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_sqlite3_get_filename(
    mut L: *mut lua_State,
    mut sqlite: *mut sqlite3_t,
) -> gint {
    lua_pushstring(L, (*sqlite).filename);
    return 1 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_sqlite3_changes(mut L: *mut lua_State) -> gint {
    let mut sqlite = luaH_checkudata(L, 1 as std::ffi::c_int, &mut sqlite3_class) as *mut sqlite3_t;
    luaH_sqlite3_checkopen(L, sqlite);
    lua_pushnumber(L, sqlite3_changes((*sqlite).db) as lua_Number);
    return 1 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_param_index(
    mut L: *mut lua_State,
    mut stmt: *mut sqlite3_stmt,
    mut idx: gint,
) -> gint {
    let mut type_0 = lua_type(L, idx);
    if type_0 == LUA_TNUMBER {
        return lua_tointeger(L, idx) as gint;
    } else if type_0 == LUA_TSTRING {
        return sqlite3_bind_parameter_index(stmt, lua_tolstring(L, idx, std::ptr::null_mut()));
    }
    return 0 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_bind_value(
    mut L: *mut lua_State,
    mut stmt: *mut sqlite3_stmt,
    mut bidx: gint,
    mut idx: gint,
) -> gint {
    match lua_type(L, idx) {
        LUA_TNUMBER => return sqlite3_bind_double(stmt, bidx, lua_tonumber(L, idx)),
        LUA_TBOOLEAN => {
            return sqlite3_bind_int(
                stmt,
                bidx,
                if lua_toboolean(L, idx) != 0 {
                    1 as std::ffi::c_int
                } else {
                    0 as std::ffi::c_int
                },
            );
        }
        LUA_TSTRING => {
            return sqlite3_bind_text(
                stmt,
                bidx,
                lua_tolstring(L, idx, std::ptr::null_mut()),
                -(1 as std::ffi::c_int),
                ::core::mem::transmute::<libc::intptr_t, sqlite3_destructor_type>(
                    SQLITE_OPEN_TRANSIENT_DB as libc::intptr_t,
                    // SQLITE_TRANSIENT as libc::intptr_t,
                ),
            );
        }
        _ => {
            _log(
                LOG_LEVEL_warn,
                b"clib/sqlite3.c\0" as *const u8 as *const std::ffi::c_char,
                &format!(
                    "sqlite3: unable to bind Lua value (type {})",
                    CStr::from_ptr(lua_typename(L, lua_type(L, idx))).to_string_lossy()
                ),
            );
        }
    }
    return SQLITE_OK;
}
unsafe extern "C-unwind" fn luaH_sqlite3_do_exec(
    mut L: *mut lua_State,
    mut stmt: *mut sqlite3_stmt,
) -> gint {
    let mut ret = sqlite3_step(stmt);
    let mut rows = 0;
    let mut ncol = 0 as std::ffi::c_int;
    if ret == SQLITE_DONE || ret == SQLITE_ROW {
        ncol = sqlite3_column_count(stmt);
        if ncol != 0 {
            lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
        } else {
            lua_pushnil(L);
        }
    }
    loop {
        match ret {
            SQLITE_DONE => return 1 as std::ffi::c_int,
            SQLITE_ROW => {
                lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
                let mut i = 0 as std::ffi::c_int;
                while i < ncol {
                    lua_pushstring(L, sqlite3_column_name(stmt, i));
                    match sqlite3_column_type(stmt, i) {
                        SQLITE_INTEGER | SQLITE_FLOAT => {
                            lua_pushnumber(L, sqlite3_column_double(stmt, i));
                            lua_rawset(L, -(3 as std::ffi::c_int));
                        }
                        SQLITE_BLOB | SQLITE_TEXT => {
                            lua_pushstring(
                                L,
                                sqlite3_column_blob(stmt, i) as *const std::ffi::c_char,
                            );
                            lua_rawset(L, -(3 as std::ffi::c_int));
                        }
                        SQLITE_NULL | _ => {
                            lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
                        }
                    }
                    i += 1;
                    i;
                }
                rows += 1;
                lua_rawseti(L, -(2 as std::ffi::c_int), rows);
                ret = sqlite3_step(stmt);
            }
            _ => return -(1 as std::ffi::c_int),
        }
    }
}
unsafe extern "C-unwind" fn luaH_sqlite3_exec(mut L: *mut lua_State) -> gint {
    let mut sqlite = luaH_checkudata(L, 1 as std::ffi::c_int, &mut sqlite3_class) as *mut sqlite3_t;
    luaH_sqlite3_checkopen(L, sqlite);
    let mut sql = luaL_checklstring(L, 2 as std::ffi::c_int, std::ptr::null_mut());
    let mut tail = 0 as *const gchar;
    if !(lua_type(L, 3 as std::ffi::c_int) <= 0 as std::ffi::c_int) {
        if !(lua_type(L, 3 as std::ffi::c_int) == LUA_TTABLE) {
            luaL_argerror(
                L,
                3 as std::ffi::c_int,
                b"table\0" as *const u8 as *const std::ffi::c_char,
            );
        }
    }
    let mut top = lua_gettop(L);
    let mut ret = 0 as std::ffi::c_int;
    let mut stmt = 0 as *mut sqlite3_stmt;
    loop {
        if sqlite3_prepare_v2(
            (*sqlite).db,
            sql,
            -(1 as std::ffi::c_int),
            &mut stmt,
            &mut tail,
        ) != 0
        {
            lua_pushfstring(
                L,
                b"sqlite3: statement compilation failed (%s)\0" as *const u8
                    as *const std::ffi::c_char,
                sqlite3_errmsg((*sqlite).db),
            );
            sqlite3_finalize(stmt);
            lua_error(L);
        } else if stmt.is_null() {
            return 0 as std::ffi::c_int;
        }
        if !(lua_type(L, 3 as std::ffi::c_int) <= 0 as std::ffi::c_int) {
            lua_pushnil(L);
            let mut idx: gint = 0;
            while lua_next(L, 3 as std::ffi::c_int) != 0 {
                idx = luaH_param_index(L, stmt, -(2 as std::ffi::c_int));
                if idx == 0 as std::ffi::c_int {
                    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
                } else {
                    ret = luaH_bind_value(L, stmt, idx, -(1 as std::ffi::c_int));
                    if !(ret == SQLITE_OK || ret == SQLITE_RANGE) {
                        lua_pushfstring(
                            L,
                            b"sqlite3: sqlite3_bind_* failed (%s)\0" as *const u8
                                as *const std::ffi::c_char,
                            sqlite3_errmsg((*sqlite).db),
                        );
                        sqlite3_finalize(stmt);
                        lua_error(L);
                    }
                    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
                }
            }
        }
        ret = luaH_sqlite3_do_exec(L, stmt);
        sqlite3_finalize(stmt);
        if ret == -(1 as std::ffi::c_int) {
            lua_pushfstring(
                L,
                b"sqlite3: exec error (%s)\0" as *const u8 as *const std::ffi::c_char,
                sqlite3_errmsg((*sqlite).db),
            );
            lua_error(L);
        }
        if !(!tail.is_null() && *tail as std::ffi::c_int != 0) {
            break;
        }
        sql = tail;
        lua_settop(L, top);
    }
    return 1 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_sqlite3_stmt_exec(mut L: *mut lua_State) -> gint {
    let mut stmt =
        luaH_checkudata(L, 1 as std::ffi::c_int, &mut sqlite3_stmt_class) as *mut sqlite3_stmt_t;
    let mut sqlite = (*stmt).sqlite;
    luaH_sqlite3_checkopen(L, sqlite);
    sqlite3_reset((*stmt).stmt);
    let mut ret: gint = 0;
    if !(lua_type(L, 2 as std::ffi::c_int) <= 0 as std::ffi::c_int) {
        if !(lua_type(L, 2 as std::ffi::c_int) == LUA_TTABLE) {
            luaL_argerror(
                L,
                2 as std::ffi::c_int,
                b"table\0" as *const u8 as *const std::ffi::c_char,
            );
        }
        sqlite3_clear_bindings((*stmt).stmt);
        lua_pushnil(L);
        let mut idx: gint = 0;
        while lua_next(L, 2 as std::ffi::c_int) != 0 {
            idx = luaH_param_index(L, (*stmt).stmt, -(2 as std::ffi::c_int));
            if idx == 0 as std::ffi::c_int {
                lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
            } else {
                ret = luaH_bind_value(L, (*stmt).stmt, idx, -(1 as std::ffi::c_int));
                if !(ret == SQLITE_OK || ret == SQLITE_RANGE) {
                    lua_pushfstring(
                        L,
                        b"sqlite3: sqlite3_bind_* failed (%s)\0" as *const u8
                            as *const std::ffi::c_char,
                        sqlite3_errmsg((*sqlite).db),
                    );
                    sqlite3_finalize((*stmt).stmt);
                    lua_error(L);
                }
                lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
            }
        }
    }
    ret = luaH_sqlite3_do_exec(L, (*stmt).stmt);
    if ret == -(1 as std::ffi::c_int) {
        lua_pushfstring(
            L,
            b"sqlite3: exec error (%s)\0" as *const u8 as *const std::ffi::c_char,
            sqlite3_errmsg((*sqlite).db),
        );
        lua_error(L);
    }
    return 1 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_sqlite3_new(mut L: *mut lua_State) -> gint {
    luaH_class_new(L, &mut sqlite3_class);
    let mut sqlite =
        luaH_checkudata(L, -(1 as std::ffi::c_int), &mut sqlite3_class) as *mut sqlite3_t;
    if ((*sqlite).db).is_null() {
        lua_pushlstring(
            L,
            b"sqlite3: database not opened, forgot filename?\0" as *const u8
                as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 47]>())
                .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
                .wrapping_sub(1),
        );
        lua_error(L);
    }
    return 1 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn sqlite3_class_setup(mut L: *mut lua_State) {
    static mut sqlite3_methods: [luaL_Reg; 4] = unsafe {
        [
            {
                let mut init = luaL_Reg {
                    name: b"add_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_sqlite3_class_add_signal,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"remove_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_sqlite3_class_remove_signal,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"emit_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_sqlite3_class_emit_signal,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__call\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_sqlite3_new,
                };
                init
            },
            // {
            //     let mut init = luaL_Reg {
            //         name: std::ptr::null(),
            //         func: None,
            //     };
            //     init
            // },
        ]
    };
    static mut sqlite3_meta: [luaL_Reg; 12] = unsafe {
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
                    name: b"exec\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_sqlite3_exec,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"close\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_sqlite3_close,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"compile\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_sqlite3_compile,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"changes\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_sqlite3_changes,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__gc\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_sqlite3_gc,
                };
                init
            },
            // {
            //     let mut init = luaL_Reg {
            //         name: std::ptr::null(),
            //         func: 0 as unsafe extern "C-unwind" fn(*mut lua_State) -> i32,
            //     };
            //     init
            // },
        ]
    };
    luaH_class_setup(
        L,
        &mut sqlite3_class,
        b"sqlite3\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option<unsafe extern "C-unwind" fn(*mut lua_State) -> *mut sqlite3_t>,
            lua_class_allocator_t,
        >(Some(
            sqlite3_new as unsafe extern "C-unwind" fn(*mut lua_State) -> *mut sqlite3_t,
        )),
        None,
        None,
        sqlite3_methods.as_ptr(),
        sqlite3_meta.as_ptr(),
    );
    luaH_class_add_property(
        &mut sqlite3_class,
        L_TK_FILENAME,
        ::core::mem::transmute::<
            Option<unsafe extern "C-unwind" fn(*mut lua_State, *mut sqlite3_t) -> gint>,
            lua_class_propfunc_t,
        >(Some(
            luaH_sqlite3_set_filename
                as unsafe extern "C-unwind" fn(*mut lua_State, *mut sqlite3_t) -> gint,
        )),
        ::core::mem::transmute::<
            Option<unsafe extern "C-unwind" fn(*mut lua_State, *mut sqlite3_t) -> gint>,
            lua_class_propfunc_t,
        >(Some(
            luaH_sqlite3_get_filename
                as unsafe extern "C-unwind" fn(*mut lua_State, *mut sqlite3_t) -> gint,
        )),
        None,
    );
    static mut sqlite3_stmt_meta: [luaL_Reg; 2] = unsafe {
        [
            {
                let mut init = luaL_Reg {
                    name: b"exec\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_sqlite3_stmt_exec,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__gc\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_sqlite3_stmt_gc,
                };
                init
            },
            // {
            //     let mut init = luaL_Reg {
            //         name: std::ptr::null(),
            //         func: None,
            //     };
            //     init
            // },
        ]
    };
    luaH_class_setup(
        L,
        &mut sqlite3_stmt_class,
        b"sqlite3::statement\0" as *const u8 as *const std::ffi::c_char,
        None,
        None,
        None,
        std::ptr::null(),
        sqlite3_stmt_meta.as_ptr(),
    );
}
