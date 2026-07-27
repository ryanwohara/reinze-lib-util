use common::source::Source;
use std::ffi::CString;

pub fn query(source: Source) -> anyhow::Result<Vec<String>> {
    let syntax = Ok(vec![
        "Syntax: -colors get | -colors set 4 1 | -colors set 14 04 | -colors set 14,01 04,01 | -colors del".to_string(),
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
    let pattern = regex::Regex::new(r"^\d\d?(,\d\d?)?$").unwrap();

    pattern.is_match(s.to_string().as_str())
}

/// Parses the `set` subcommand arguments into (color1, color2) specs.
/// Each spec is a foreground code `fg` or a foreground+background `fg,bg`.
/// Returns None unless there are two whitespace-separated tokens after `set`
/// and both pass `validate`. Single-digit codes are zero-padded to two digits
/// (see `pad_color`) so IRC clients parse them unambiguously.
fn parse_set_colors(query: &str) -> Option<(String, String)> {
    let mut split = query.split_whitespace();
    let _ = split.next(); // discard the "set" subcommand token
    let c1 = split.next()?;
    let c2 = split.next()?;

    if !validate(c1) || !validate(c2) {
        return None;
    }

    Some((pad_color(c1), pad_color(c2)))
}

/// Zero-pads each single-digit component of a color spec to two digits, so IRC
/// clients read the code unambiguously (e.g. `4` -> `04`, `4,1` -> `04,01`).
/// A spec is `fg` or `fg,bg`; each side is padded independently. Callers pass
/// only specs that already passed `validate`.
fn pad_color(spec: &str) -> String {
    spec.split(',')
        .map(|part| {
            if part.len() == 1 {
                format!("0{part}")
            } else {
                part.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(",")
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
    fn test_validate_accepts_single_digits() {
        assert!(validate("1")); // single-digit foreground
        assert!(validate("4,1")); // single-digit fg and bg
        assert!(validate("14,1")); // single-digit background
        assert!(validate("4,12")); // single-digit fg, two-digit bg
    }

    #[test]
    fn test_validate_rejects_bad_input() {
        assert!(!validate("14,")); // trailing comma
        assert!(!validate(",01")); // missing foreground
        assert!(!validate("abc")); // non-numeric
        assert!(!validate("140")); // too many digits (foreground)
        assert!(!validate("140,01")); // too many digits
        assert!(!validate("14,010")); // too many digits (background)
        assert!(!validate("")); // empty
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
        assert_eq!(parse_set_colors("set 14 abc"), None);
        assert_eq!(parse_set_colors("set 140 04"), None);
    }

    #[test]
    fn test_parse_set_colors_pads_single_digit_foregrounds() {
        assert_eq!(
            parse_set_colors("set 4 1"),
            Some(("04".to_string(), "01".to_string()))
        );
    }

    #[test]
    fn test_parse_set_colors_pads_single_digit_backgrounds() {
        assert_eq!(
            parse_set_colors("set 4,1 14,1"),
            Some(("04,01".to_string(), "14,01".to_string()))
        );
    }

    #[test]
    fn test_parse_set_colors_pads_mixed_widths() {
        assert_eq!(
            parse_set_colors("set 4,12 3"),
            Some(("04,12".to_string(), "03".to_string()))
        );
    }

    #[test]
    fn test_parse_set_colors_leaves_two_digits_unchanged() {
        assert_eq!(
            parse_set_colors("set 14,01 04,09"),
            Some(("14,01".to_string(), "04,09".to_string()))
        );
    }

    #[test]
    fn test_pad_color() {
        assert_eq!(pad_color("4"), "04");
        assert_eq!(pad_color("14"), "14");
        assert_eq!(pad_color("4,1"), "04,01");
        assert_eq!(pad_color("14,1"), "14,01");
        assert_eq!(pad_color("4,12"), "04,12");
        assert_eq!(pad_color("14,09"), "14,09");
    }
}
