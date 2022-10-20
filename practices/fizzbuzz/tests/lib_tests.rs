#![cfg_attr(feature = "strict", deny(warnings))]

use fizzbuzz;
use rstest::rstest;

#[rstest(input, expected,
    case(3, "Fizz"),
    case(4, "4"),
    case(5, "Buzz"),
    case(6, "Fizz"),
    case(10, "Buzz"),
    case(11, "11"),
    case(15, "FizzBuzz"),
    case(30, "FizzBuzz"),
)]
#[test]
fn judge_fizz_buzz(input: i32, expected: String) {
    assert_eq!(expected, fizzbuzz::judge_fizz_buzz(input));
}
