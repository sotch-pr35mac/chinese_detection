# language_profiler
##### v1.0.0
---

### About
Classify a string as either English, Chinese, or Pinyin. 

### Usage
```rust
extern crate chinese_detection;

use chinese_detection::ChineseDetection;
use chinese_detection::ClassificationResult;

let language_detection = ChineseDetection::new();

println!("{}", language_detection.classify("test")); // --> ClassificationResult::EN
println!("{}", language_detection.classify("shiyan")); // --> ClassificationResult::PY
println!("{}", language_detection.classify("实验")); // --> ClassificationResult::ZH
```

### License
[MIT](https://github.com/sotch-pr35mac/language_profiler/blob/master/LICENSE)
