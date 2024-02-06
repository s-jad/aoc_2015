use itertools::Itertools;
use serde_json::Value;
use std::time::Instant;

fn parse(value: &Value) -> i64 {
    match value {
        Value::Object(obj) => {
            match obj
                .values()
                .any(|val| val == &Value::String("red".to_string()))
            {
                true => 0,
                false => obj.values().fold(0, |sum, v| sum + parse(v)),
            }
        }
        Value::Array(arr) => arr.iter().fold(0, |sum, v| sum + parse(v)),
        Value::Number(num) => num.as_i64().expect("Failed to convert number to i32"),
        _ => 0,
    }
}

fn process(input: &str) -> i64 {
    let json: Value = serde_json::from_str(input).expect("Invalid JSON");

    parse(&json)
}

//fn process(input: &str) -> i32 {
//    let re = Regex::new(r"(?:red|(?:[-]?\d+)+|[{}])").unwrap();
//    let split: Vec<&str> = re.find_iter(input).map(|m| m.as_str()).collect();
//
//    let mut depth = 0;
//    let mut removal_depth = 0;
//    let mut removing = false;
//    let mut parsed = Vec::new();
//
//    let mut ridx = split.len() / 2;
//    let mut lidx = split.len() / 2;
//
//    println!("split: {split:?}");
//
//    1
//}

fn main() {
    let input = include_str!("../../input.txt");

    let start = Instant::now();
    let output = process(input);
    let time = start.elapsed();

    dbg!(output, time);
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = include_str!("../../test.txt");
        let output = process(input);
        assert_eq!(result,);
    }
}
