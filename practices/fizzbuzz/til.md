# 学んだこと

- Rustのパラメータライズドテストの書き方
  - rstestを使うと楽
  - [参考](https://caddi.tech/archives/1849)
- `#![cfg_attr(feature = "strict", deny(warnings))]`より
  - cfg_attrが何者なのか
    - `[cfg_attr(condition, attribute)] allows you to only add the attribute to the thing it decorates if the condition is true.`
    - [引用元](https://chrismorgan.info/blog/rust-cfg_attr/)
  - フィーチャーフラグ
    - [参考](https://qiita.com/osanshouo/items/43271813b5d62e89d598)
  - Cargo.tomlに利用可能なfeatureについての記述を置く
  - これらによって `cargo test --features strict` とすることで、build errorをwarningに留めてテスト実行が可能
