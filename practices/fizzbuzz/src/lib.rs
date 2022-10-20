#![cfg_attr(feature = "strict", deny(warnings))]

pub fn judge_fizz_buzz(num: i32) -> String {
    if num%15 == 0 {
        return "FizzBuzz".to_string();
    };
    if num%3 == 0 {
        return "Fizz".to_string();
    };
    if num%5 == 0 {
        return "Buzz".to_string();
    };
    num.to_string()
}
