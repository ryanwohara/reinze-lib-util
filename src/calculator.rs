extern crate common;
use common::l;
use common::c1;
use common::c2;
use common::remove_trailing_zeroes;
use common::commas;
use meval::eval_str;
use regex::Regex;

pub fn calculate(query: &str) -> Result<Vec<String>, ()> {
    // From iKick's source
    // alias calc2 return $calc($regsubex($remove($1-,$chr(44)),/([\d.]+)([kmb])/gi,$calc(\1 * $+(1,$str(000,$pos(kmb,\2,1))))))
    let re_verify = Regex::new(r"^(?:pi|e|abs|sqrt|exp|ln|sin|cos|tan|asin|acos|atan|atan2|sinh|cosh|tanh|asinh|acosh|atanh|floor|ceil|round|[-+%/*^\d\s()><=,.kmb])+$").unwrap();

    if !re_verify.is_match(query) {
        return Err(());
    }

    let result = eval_query(query.replace(",", ""))?;

    Ok(vec![format!(
        "{} {} = {}",
        l("Calc"),
        c1(query),
        c2(&remove_trailing_zeroes(&commas(
            result, "f"
        )))
    )])
}

fn eval_query(query: String) -> Result<f64, ()> {
    let re_kmb = Regex::new(r"(?P<num>[\d.]+)(?P<kmb>[kmb])").unwrap();
    let processed = re_kmb.replace_all(&query, replace_all).to_string();

    eval_str(&processed).map_err(|e| {
        println!("Error: {}", e);
        ()
    })
}

fn replace_all(caps: &regex::Captures) -> String {
    let (num, kmb) = (
        caps.name("num").unwrap().as_str(),
        caps.name("kmb").unwrap().as_str(),
    );
    let mut num = num.parse::<f64>().unwrap_or_default();

    if let Some(factor) = match kmb {
        "k" => Some(1_000.0),
        "m" => Some(1_000_000.0),
        "b" => Some(1_000_000_000.0),
        _ => None,
    } {
        num *= factor;
    }
    num.to_string()
}
