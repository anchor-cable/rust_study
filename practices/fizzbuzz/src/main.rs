use fizzbuzz::judge_fizz_buzz;

fn main() {
    for n in 1..101 {
        println!("{}", judge_fizz_buzz(n))
    }
}