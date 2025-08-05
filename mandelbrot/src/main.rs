use num::Complex;
use std::str::FromStr;

fn escape_time(c: Complex<f64>, limit:usize) -> Option<usize> {
    let mut z = Complex {re : 0.0, im: 0.0};
    for i in 0..limit {    
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z * x * x + c;
    }
    None
}

fn parse_pair<FromStr>(c: &str, seperator: char) -> Option<T> {
    match c.find(seperator) {
        None => None,
        Some(index) => {
            match(T::from_str(&str[..index]), T::from_str(&str[index +1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None    
            }
        }
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<usize>("10,20", ','), Some((10, 20)));
}

fn main() {
    println!("Hello, world!");
}
