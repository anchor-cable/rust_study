# 学んだこと

## Rustの文法テクニックアレコレ

- `for x in 1..=100`
  - 1から100まで繰り返し
  - The Bookには `for x in 1..101` って書き方だったけどこちらの方がより直感的で優れてる
- matchとguard
  ```
    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        _ => println!("Fell through"), // This should not be possible to reach
    }
  ```
  - [参考](https://doc.rust-jp.rs/rust-by-example-ja/flow_control/match/guard.html)
  - 

- 