mod calculator;
mod common;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn exported(
    cmd: *const c_char,
    raw_query: *const c_char,
    raw_author: *const c_char,
) -> *mut c_char {
    let nil = CString::new("").unwrap().into_raw();

    if cmd.is_null() || raw_query.is_null() || raw_author.is_null() {
        return nil; // using unwrap() here is safe because we know the string is valid UTF-8
    }

    let unsafe_cmd = unsafe { CStr::from_ptr(cmd) };
    let unsafe_query = unsafe { CStr::from_ptr(raw_query) };
    let unsafe_author = unsafe { CStr::from_ptr(raw_author) };

    let command = match unsafe_cmd.to_str() {
        Ok(command) => command,
        Err(_) => return nil,
    };

    let query = match unsafe_query.to_str() {
        Ok(query) => query,
        Err(_) => return nil,
    };

    let _author = match unsafe_author.to_str() {
        Ok(author) => author,
        Err(_) => return nil,
    };

    match match command {
        "calc" | "calculate" | "calculator" => calculator::calculate(query),
        "help" => Ok(vec!["calc".to_string()]),
        "" => Ok("calc"
            .split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>()),
        _ => Ok(vec![]),
    } {
        Ok(output) => match CString::new(output.join("\n")) {
            Ok(output) => output.into_raw(),
            Err(_) => nil,
        },
        Err(_) => nil,
    }
}
