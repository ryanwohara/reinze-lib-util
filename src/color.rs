use common::Colors;
use common::source::Source;

pub fn query(source: Source) -> Result<Vec<String>, ()> {
    let syntax = Ok(vec![
        "Syntax: -colors get | -colors set 14,04 | -colors del".to_string(),
    ]);

    if source.query.len() == 0 {
        return syntax;
    }

    let mut split = source.query.split_whitespace();
    let first = split.next().unwrap();

    match first {
        "del" => del(&source),
        "get" => get(&source),
        "set" => set(&source),
        _ => syntax,
    }
}

fn del(s: &Source) -> Result<Vec<String>, ()> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(s.clear_colors());

    Ok(vec![
        vec![s.l("Colors"), s.c2("Colors cleared".to_string())].join(" "),
    ])
}

fn get(s: &Source) -> Result<Vec<String>, ()> {
    Ok(vec![
        vec![
            s.l("Colors"),
            s.c1("Color 1!"),
            s.c2("Color 2!"),
        ]
        .join(" "),
    ])
}

fn validate<T>(s: T) -> bool
where
    T: ToString,
{
    let pattern = regex::Regex::new(r"^\d\d$").unwrap();

    pattern.is_match(s.to_string().as_str())
}

fn set(s: &Source) -> Result<Vec<String>, ()> {
    let error = Ok(vec![
        vec![s.l("Colors"), s.c2("Please provide two colors")].join(" "),
    ]);
    let mut split = s.query.split_whitespace();
    let _ = split.next();
    let second = split.next();
    if second.is_none() {
        return error;
    }
    let mut c1 = second.unwrap().to_string();
    let c2;
    if c1.contains(",") {
        let split = c1.split_once(",");
        if split.is_none() {
            return error;
        }

        let split_c1 = split.unwrap().0.to_string();
        let split_c2 = split.unwrap().1.to_string();

        c1 = split_c1;
        c2 = split_c2;
    } else {
        let split_c2 = split.next();
        if split_c2.is_none() {
            return error;
        }
        c2 = split_c2.unwrap().to_string();
    }

    if !validate(&c1) || !validate(&c2) {
        return error;
    }

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(s.set_colors(Colors { c1, c2 }));

    get(s)
}
