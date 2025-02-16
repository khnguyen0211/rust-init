use core::num;

fn get_input() -> Option<String> {
    // return None;
    return Some("16abc".to_owned());
}

fn parse(s: &str) -> Option<i32> {
    return s.parse::<i32>().ok();
}

fn is_perfect_square(n: i32) -> Option<i32> {
    // If closure is True -> `find` will return Some(i), else return None
    return (0..=n).find(|i| i * i == n);
}

fn compute_01() -> Option<i32> {
    // `?` will return None immediately if the Option is None
    let input: String = get_input()?;
    let number: i32 = parse(&input)?;
    return is_perfect_square(number);
}

fn compute_02() -> Option<i32> {
    // `and_then` will return None if the Option is None, else call the closure
    return get_input()
        .and_then(|input: String| parse(&input))
        .and_then(|input: i32| is_perfect_square(input));
}

fn compute_03() -> Result<i32, String> {
    // `ok_or` will convert `Option` to `Result`
    let input: String = get_input().ok_or("Invalid Input".to_owned())?;
    let number: i32 = parse(&input).ok_or("Error when parse value".to_owned())?;
    return is_perfect_square(number).ok_or("Error when check is perfect square".to_owned());
}

fn compute_04() -> Result<i32, String> {
    let input: String = get_input().ok_or("Invalid Input".to_owned())?;
    // if the Option is None, `ok_or_else` will run the closure
    let number: i32 = parse(&input).ok_or_else(|| {
        let mut message: String = String::new();
        core::fmt::write(&mut message, format_args!("{} is invalid number", input)).unwrap();
        return message;
    })?;
    return is_perfect_square(number).ok_or("Error when check is perfect square".to_owned());
}
fn main() {
    let rs_01: Option<i32> = compute_01();

    match rs_01 {
        Some(value) => {
            println!("Result: {value}")
        }
        None => {
            println!("Error")
        }
    }

    let rs_02: Option<i32> = compute_02();
    match rs_02 {
        Some(value) => {
            println!("Result: {value}")
        }
        None => {
            println!("Error")
        }
    }

    let rs_03: Result<i32, String> = compute_03();
    match rs_03 {
        Ok(value) => {
            println!("Result: {value}")
        },
        Err(error) => {
            println!("{error}")
        }
    }

    let rs_04: Result<i32, String> = compute_04();
    match rs_04 {
        Ok(value) => {
            println!("Result: {value}")
        },
        Err(error) => {
            println!("{error}")
        }
    }
}
