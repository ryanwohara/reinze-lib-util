extern crate common;

use common::source::Source;

pub fn c_f(s: &Source) -> Result<Vec<String>, ()> {
    let query = &s.query;

    let c = query.parse::<f64>().unwrap_or_default();
    let f = c * 1.8 + 32.0;

    let output = vec![
        s.l("Temperature"),
        s.p("Celsius->Fahrenheit"),
        s.c1(query),
        s.c1(">"),
        s.c2(&format!("{}", f)),
    ]
    .join(" ");

    Ok(vec![output])
}

pub fn f_c(s: &Source) -> Result<Vec<String>, ()> {
    let query = &s.query;

    let f = query.parse::<f64>().unwrap_or_default();
    let c = (f - 32.0) / 1.8;

    let output = vec![
        s.l("Temperature"),
        s.p("Fahrenheit->Celsius"),
        s.c1(query),
        s.c1(">"),
        s.c2(&format!("{}", c)),
    ]
        .join(" ");

    Ok(vec![output])
}
