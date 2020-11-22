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
//! use chinese_detection::ChineseDetection;
//! use chinese_detection::ClassificationResult;
//! 
//! let language_detection = ChineseDetection::new();
//!
//! println!("{:?}", language_detection.classify("test")); // --> ClassificationResult::EN
//! println!("{:?}", language_detection.classify("shiyan")); // --> ClassificationResult::PY
//! println!("{:?}", language_detection.classify("实验")); // --> ClassificationResult::ZH

extern crate bincode;

mod language_profiler;
pub use self::language_profiler::Profiler as ChineseDetection;
pub use self::language_profiler::ClassificationResult;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn classify_english() {
		let profiler = ChineseDetection::new();
		let test_cases = vec!["this is a test",
					"test",
					"exam",
					"whats up",
					"desk",
					"window",
					"student"];

		for test in test_cases {
			assert_eq!(ClassificationResult::EN, profiler.classify(test));
		}	
	}

	#[test]
	fn classify_pinyin() {
		let profiler = ChineseDetection::new();
		let test_cases = vec!["zhe shi ge shiyan",
					"shiyan",
					"kaoshi",
					"zenmeyang",
					"zhuozi",
					"chuanghu",
					"xuesheng"];

		for test in test_cases {
			assert_eq!(ClassificationResult::PY, profiler.classify(test));
		}
	}

	#[test]
	fn classify_chinese() {
		let profiler = ChineseDetection::new();
		let test_cases = vec!["这是个实验",
								"实验",
								"考试",
								"怎么样",
								"桌子",
								"窗户",
								"学生"];

		for test in test_cases {
			assert_eq!(ClassificationResult::ZH, profiler.classify(test));
		}
	}
}
