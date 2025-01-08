use std::num::ParseIntError;

fn to_int(s: String) -> Result<i32, ParseIntError> {
    //s.parse().unwrap()
    //s.parse().expect("Not a number")
    //s.parse().unwrap_or(0)
    //s.parse().ok()
    s.parse()
}

#[derive(Debug)]
struct SummationError;

fn sum_str_vec(strings: Vec<String>) -> Result<i32, SummationError> {
    let mut acc = 0i32;
    for s in strings {
        // let n: i32 = to_int(s);
        // acc += n;

        // acc += match to_int(s) {
        //     Some(n) => n,
        //     None => 0,
        // }

        // if let Some(n) = to_int(s) {
        //     acc += n;
        // }

        //acc += to_int(s).unwrap_or(0);
        // acc += to_int(s)?;
        acc += to_int(s).map_err(|_| SummationError)?;
    }
    Ok(acc)
}

fn main() {
    let v = vec!["1".to_string(), "2".to_string(), "3".to_string()];
    let total = sum_str_vec(v);
    println!("Total1: {:?}", total);

    let v2 = vec!["4".to_string(), "5".to_string(), "abc".to_string()];
    let total2 = sum_str_vec(v2);
    println!("Total2: {:?}", total2);
}
