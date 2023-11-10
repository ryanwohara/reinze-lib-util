mod calculator;

extern crate common;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn exported(
    cmd: *const c_char,
    raw_query: *const c_char,
    raw_author: *const c_char,
) -> *mut c_char {
    let command = to_str_or_default(cmd);
    let query = to_str_or_default(raw_query);
    let _author = to_str_or_default(raw_author);

    let result = match command.as_str() {
        "calc" | "calculate" | "calculator" => calculator::calculate(query.as_str()),
        "help" => Ok(vec!["calc".to_string()]),
        "" => Ok("calc"
            .split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>()),
        _ => Ok(vec![]),
    };

    let output = result.unwrap_or_default();

    CString::new(output.join("\n")).unwrap().into_raw()
}

fn to_str_or_default(ptr: *const c_char) -> String {
    let cstr = unsafe { CStr::from_ptr(ptr) };
    cstr.to_str().unwrap_or_default().to_owned()
}
