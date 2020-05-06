# language_profiler
##### v0.1.0
---

### About
Classify a string as either English, Chinese, or Pinyin. 

### Usage
```rust
extern crate chinese_detection;

use chinese_detection::ChineseDetection;

let language_detection = ChineseDetection::new();

println!("{}", language_detection.classify(String::from("test"))); // --> EN
println!("{}", language_detection.classify(String::from*("shiyan"))); // --> PY
println!("{}", language_detection.classify(String::from("实验"))); // --> ZH
```

### Contributors
- [Preston Wang-Stosur-Bassett](http://stosur.info)

### License
[MIT](https://github.com/sotch-pr35mac/language_profiler/blob/master/LICENSE)
