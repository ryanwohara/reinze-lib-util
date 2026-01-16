mod calculator;
mod temp;

extern crate common;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[unsafe(no_mangle)]
pub extern "C" fn exported(
    cmd: *const c_char,
    raw_query: *const c_char,
    raw_author: *const c_char,
) -> *mut c_char {
    let full_command = to_str_or_default(cmd);
    let full_query = to_str_or_default(raw_query);
    let full_author = to_str_or_default(raw_author);

    let command = full_command.as_str();
    let query = full_query.as_str();
    let _author = full_author.as_str();

    let result = match command {
        "calc" | "calculate" | "calculator" => calculator::calculate(query),
        "c-f" | "c_f" => temp::c_f(query),
        "f-c" | "f_c" => temp::f_c(query),
        "help" => Ok("calc
c-f
f-c"
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>()),
        "" => Ok("calc
c[_-]f
f[_-]c"
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
