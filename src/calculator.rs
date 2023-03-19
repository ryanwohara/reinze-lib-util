use meval::eval_str;
use regex::Regex;

pub fn calculate(query: &str) -> Result<Vec<String>, ()> {
    let re = Regex::new(r"^[-+/*^\d()><]+$").unwrap();
    let matched = re.is_match(query);

    if !matched {
        return Err(());
    }

    let result = match eval(query) {
        Ok(f) => f,
        Err(_) => return Err(()),
    };

    Ok(vec![result.to_string()])
}

fn eval(query: &str) -> Result<f64, ()> {
    match eval_str(query) {
        Ok(f) => Ok(f),
        Err(e) => {
            println!("Error: {}", e);
            Err(())
        }
    }
}

// A dark future?
// https://github.com/redox-os/calc/blob/master/src/token.rs#L42
