# chinese_detection
##### v2.0.0

### About
Classify a string as either English, Chinese, or Pinyin. 

### Usage
```rust
extern crate chinese_detection;

use chinese_detection::{ClassificationResult, classify};

assert_eq!(ClassificationResult::EN, classify("test"));
assert_eq!(ClassificationResult::PY, classify("shiyan")); 
assert_eq!(ClassificationResult::ZH, classify("实验")); 
```

### License
[MIT](https://github.com/sotch-pr35mac/language_profiler/blob/master/LICENSE)
