extern crate common;

use common::{c1, c2, l, p};

pub fn c_f(query: &str) -> Result<Vec<String>, ()> {
    let c = query.parse::<f64>().unwrap_or_default();
    let f = c * 1.8 + 32.0;

    let output = vec![
        l("Temperature"),
        p("Celsius->Fahrenheit"),
        c1(query),
        c2(&format!("{}", f)),
    ]
    .join(" ");

    Ok(vec![output])
}

pub fn f_c(query: &str) -> Result<Vec<String>, ()> {
    let f = query.parse::<f64>().unwrap_or_default();
    let c = (f - 32.0) / 1.8;

    let output = vec![
        l("Temperature"),
        p("Fahrenheit->Celsius"),
        c1(query),
        c2(&format!("{}", c)),
    ]
        .join(" ");

    Ok(vec![output])
}
