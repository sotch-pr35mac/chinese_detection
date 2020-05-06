/*
 * @author	:: Preston Wang-Stosur-Bassett <http://stosur.info>
 * @date	:: May 5, 2020
 * @description	:: Classify a string as either English, Chinese, or Pinyin.
*/

//! ### About
//! Classify a string as either English, Chinese, or Pinyin.
//! 
//! ### Usage
//! ```rust
//! extern crate chinese_detection;
//! 
//! use chinese_detection::ChineseDetection;
//! 
//! let language_detection = ChineseDetection::new();
//!
//! println!("{}", language_detection.classify(String::from("test"))); // --> EN
//! println!("{}", language_detection.classify(String::from("shiyan"))); // --> PY
//! println!("{}", language_detection.classify(String::from("实验"))); // --> ZH

extern crate bincode;

mod language_profiler;
pub use self::language_profiler::Profiler as ChineseDetection;

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
			assert_eq!("EN", profiler.classify(test.to_string()));
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
			assert_eq!("PY", profiler.classify(test.to_string()));
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
			assert_eq!("ZH", profiler.classify(test.to_string()));
		}
	}
}
