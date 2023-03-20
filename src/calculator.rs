use crate::common;
use meval::eval_str;
use regex::Regex;

pub fn calculate(query: &str) -> Result<Vec<String>, ()> {
    // From iKick's source
    // alias calc2 return $calc($regsubex($remove($1-,$chr(44)),/([\d.]+)([kmb])/gi,$calc(\1 * $+(1,$str(000,$pos(kmb,\2,1))))))

    let re_verify = Regex::new(r"^[-+%/*^\d()><.kmb]+$").unwrap();

    if !re_verify.is_match(query) {
        return Err(());
    }

    let no_commas = query.replace(",", "");

    let re_kmb = Regex::new(r"(?P<num>[\d.]+)(?P<kmb>[kmb])").unwrap();
    let processed = re_kmb
        .replace_all(&no_commas, |caps: &regex::Captures| {
            let num = caps.name("num").unwrap().as_str();
            let kmb = caps.name("kmb").unwrap().as_str();

            let mut num = match num.parse::<f64>() {
                Ok(n) => n,
                Err(_) => 0.0,
            };

            match kmb {
                "k" => num *= 1000.0,
                "m" => num *= 1000000.0,
                "b" => num *= 1000000000.0,
                _ => (),
            }

            num.to_string()
        })
        .to_string();

    let result = match eval_str(processed) {
        Ok(f) => f.to_string(),
        Err(e) => {
            println!("Error: {}", e);
            return Err(());
        }
    };

    Ok(vec![format!(
        "{} {} {} {}",
        common::l("Calc"),
        common::c1(query),
        common::c1("="),
        common::c2(&common::commas_from_string(&result))
    )])
}
