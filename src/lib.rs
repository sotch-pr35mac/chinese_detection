// @author	:: Preston Wang-Stosur-Bassett <p.wanstobas@gmail.com>
// @date	:: May 5, 2020
// @description	:: Classify a string as either English, Chinese, or Pinyin.

//! ### About
//! Classify a string as either English, Chinese, or Pinyin.
//!
//! ### Usage
//! ```rust
//! extern crate chinese_detection;
//!
//! use chinese_detection::{ClassificationResult, classify};
//!
//! assert_eq!(ClassificationResult::EN, classify("test"));
//! assert_eq!(ClassificationResult::PY, classify("shiyan"));
//! assert_eq!(ClassificationResult::ZH, classify("实验"));

extern crate bincode;
extern crate once_cell;

mod language_profiler;
pub use self::language_profiler::{classify, init, ClassificationResult};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_english() {
        let test_cases = vec![
            "this is a test",
            "test",
            "exam",
            "whats up",
            "desk",
            "window",
            "student",
        ];

        for test in test_cases {
            assert_eq!(ClassificationResult::EN, classify(test));
        }
    }

    #[test]
    fn classify_pinyin() {
        let test_cases = vec![
            "zhe shi ge shiyan",
            "shiyan",
            "kaoshi",
            "zenmeyang",
            "zhuozi",
            "chuanghu",
            "xuesheng",
        ];

        for test in test_cases {
            assert_eq!(ClassificationResult::PY, classify(test));
        }
    }

    #[test]
    fn classify_chinese() {
        let test_cases = vec![
            "这是个实验",
            "实验",
            "考试",
            "怎么样",
            "桌子",
            "窗户",
            "学生",
        ];

        for test in test_cases {
            assert_eq!(ClassificationResult::ZH, classify(test));
        }
    }
}
