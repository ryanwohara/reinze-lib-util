mod calculator;
mod common;

#[no_mangle]
pub extern "C" fn exported(
    mut command: &str,
    query: &str,
    author: &str,
) -> Result<Vec<String>, ()> {
    match command {
        "calc" => calculator::calculate(query),
        "calculate" => calculator::calculate(query),
        "calculator" => calculator::calculate(query),
        "help" => Ok(vec!["calc".to_string()]),
        "" => Ok("calc"
            .split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>()),
        _ => Ok(vec![]),
    }
}
