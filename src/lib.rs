mod calculator;
mod color;
mod temp;

use common::{to_str_or_default, PluginContext};
use common::author::Author;
use common::source::Source;
use std::ffi::{CString};
use std::os::raw::c_char;

#[unsafe(no_mangle)]
pub extern "C" fn exported(context: *const PluginContext) -> *mut c_char {
    unsafe {
        let full_command = to_str_or_default((*context).cmd);
        let full_query = to_str_or_default((*context).param);
        let full_author = to_str_or_default((*context).author);
        let color = (*context).color;

        let command = full_command.as_str();
        let query = full_query.as_str();
        let author = full_author.as_str();

        let source = Source::create("0", Author::create(author, color), command, query);

        let result = match command {
            "calc" | "calculate" | "calculator" => calculator::calculate(&source),
            "color" | "colors" => color::query(source),
            "c" | "c-f" => temp::c_f(&source),
            "f" | "f-c" => temp::f_c(&source),
            "help" => Ok("calc
colors
c-f
f-c"
            .split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>()),
            "" => Ok("calc
colors?$
^c(-?f)?$
^f(-?c)?$"
                .split("\n")
                .map(|s| s.to_string())
                .collect::<Vec<String>>()),
            _ => Ok(vec![]),
        };

        let output = result.unwrap_or_default();

        CString::new(output.join("\n")).unwrap().into_raw()
    }
}