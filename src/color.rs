use common::source::Source;
use std::ffi::CString;

pub fn query(source: Source) -> anyhow::Result<Vec<String>> {
    let syntax = Ok(vec![
        "Syntax: -colors get | -colors set 14 04 | -colors set 14,01 04,01 | -colors del".to_string(),
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

fn del(s: &Source) -> anyhow::Result<Vec<String>> {
    s.clear_colors();

    Ok(vec![
        vec![s.l("Colors"), s.c2("Colors cleared".to_string())].join(" "),
    ])
}

fn get(s: &Source) -> anyhow::Result<Vec<String>> {
    Ok(vec![
        vec![s.l("Colors"), s.c1("Color 1!"), s.c2("Color 2!")].join(" "),
    ])
}

fn validate<T>(s: T) -> bool
where
    T: ToString,
{
    let pattern = regex::Regex::new(r"^\d\d(,\d\d)?$").unwrap();

    pattern.is_match(s.to_string().as_str())
}

/// Parses the `set` subcommand arguments into (color1, color2) specs.
/// Each spec is a foreground code `fg` or a foreground+background `fg,bg`.
/// Returns None unless there are two whitespace-separated tokens after `set`
/// and both pass `validate`.
fn parse_set_colors(query: &str) -> Option<(String, String)> {
    let mut split = query.split_whitespace();
    let _ = split.next(); // discard the "set" subcommand token
    let c1 = split.next()?;
    let c2 = split.next()?;

    if !validate(c1) || !validate(c2) {
        return None;
    }

    Some((c1.to_string(), c2.to_string()))
}

fn set(s: &Source) -> anyhow::Result<Vec<String>> {
    let Some((c1, c2)) = parse_set_colors(&s.query) else {
        return Ok(vec![
            vec![s.l("Colors"), s.c2("Please provide two colors")].join(" "),
        ]);
    };

    let host = CString::new(s.author.host.to_string()).unwrap().into_raw();
    let colors = CString::new(vec![c1, c2].join("|")).unwrap().into_raw();

    (s.author.color)(host, colors);

    get(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_accepts_foreground_only() {
        assert!(validate("14"));
        assert!(validate("04"));
    }

    #[test]
    fn test_validate_accepts_foreground_and_background() {
        assert!(validate("14,01"));
        assert!(validate("04,09"));
    }

    #[test]
    fn test_validate_rejects_bad_input() {
        assert!(!validate("1")); // single digit
        assert!(!validate("14,1")); // single-digit background
        assert!(!validate("14,")); // trailing comma
        assert!(!validate(",01")); // missing foreground
        assert!(!validate("abc")); // non-numeric
        assert!(!validate("140,01")); // too many digits
    }

    #[test]
    fn test_parse_set_colors_two_foregrounds() {
        assert_eq!(
            parse_set_colors("set 14 04"),
            Some(("14".to_string(), "04".to_string()))
        );
    }

    #[test]
    fn test_parse_set_colors_with_backgrounds() {
        assert_eq!(
            parse_set_colors("set 14,01 04,01"),
            Some(("14,01".to_string(), "04,01".to_string()))
        );
    }

    #[test]
    fn test_parse_set_colors_mixed() {
        assert_eq!(
            parse_set_colors("set 14,01 04"),
            Some(("14,01".to_string(), "04".to_string()))
        );
    }

    #[test]
    fn test_parse_set_colors_single_token_is_none() {
        // The old "two colors via comma" form is now one fg,bg token → only one color.
        assert_eq!(parse_set_colors("set 14,04"), None);
        assert_eq!(parse_set_colors("set 14"), None);
    }

    #[test]
    fn test_parse_set_colors_no_colors_is_none() {
        assert_eq!(parse_set_colors("set"), None);
    }

    #[test]
    fn test_parse_set_colors_invalid_is_none() {
        assert_eq!(parse_set_colors("set 1 04"), None);
        assert_eq!(parse_set_colors("set 14 abc"), None);
    }
}
