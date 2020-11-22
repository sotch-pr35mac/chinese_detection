// @author		:: Preston Wang-Stosur-Bassett
// @date 		:: January 24, 2020
// @description		:: This package classifies a string as Chinese characters, Pinyin, or English

use bincode::deserialize_from;
use std::collections::HashMap;

static CHINESE_DATA: &'static [u8] = include_bytes!("../profiles/zh.profile");
static ENGLISH_DATA: &'static [u8] = include_bytes!("../profiles/en.profile");
static PINYIN_DATA: &'static [u8] = include_bytes!("../profiles/py.profile");
static GRAM_LENGTH: usize = 2;

#[derive(Debug, PartialEq)]
pub enum ClassificationResult {
	ZH, // Chinese
	EN, // English
	PY, // Pinyin
	UN, // Uncertain
}

pub struct Profiler {
	english_profile: HashMap<String, u64>,
	chinese_profile: HashMap<String, u64>,
	pinyin_profile: HashMap<String, u64>,
	english_total: u64,
	chinese_total: u64,
	pinyin_total: u64
}

impl Profiler {	
	pub fn new() -> Profiler {		
		let english_profile: HashMap<String, u64> = deserialize_from(ENGLISH_DATA).unwrap();
		let chinese_profile: HashMap<String, u64> = deserialize_from(CHINESE_DATA).unwrap();
		let pinyin_profile: HashMap<String, u64> = deserialize_from(PINYIN_DATA).unwrap();
		
		return Profiler {
			english_profile: english_profile.clone(),
			chinese_profile: chinese_profile.clone(),
			pinyin_profile: pinyin_profile.clone(),
			english_total: Profiler::calc_total(&english_profile),
			chinese_total: Profiler::calc_total(&chinese_profile),
			pinyin_total: Profiler::calc_total(&pinyin_profile)
		}
	}
	
	fn calc_total(profile_data: &HashMap<String, u64>) -> u64 {
		let mut accumulator: u64 = 0;
		for (_, value) in profile_data.iter() {
			accumulator += value;
		}
		return accumulator;
	}
	
	fn calc_prob(substring: &str, profile: &HashMap<String, u64>, total: f64) -> f64 {
		let occurances: f64 = *profile.get(substring).unwrap_or(&1) as f64;
		return (occurances / total).log10() * -1 as f64;
	}
	
	fn score(&self, sentence: &str, profile: &HashMap<String, u64>, total: &u64) -> f64 {
		let mut accumulator: f64 = 0.0;
		let words: Vec<_> = sentence.split(" ").collect();
		for word in words {
			if word.chars().count() < GRAM_LENGTH {
				accumulator += Profiler::calc_prob(word, profile, *total as f64);
			} else {
				let mut i = 0;
				while i < (word.chars().count() - (GRAM_LENGTH - 1)) {
					let substring: String = word.chars().skip(i).take(GRAM_LENGTH).collect();
					accumulator += Profiler::calc_prob(&substring, profile, *total as f64);
					i += 1;	
				}
			}	
		}
		
		return accumulator;
	}

	pub fn classify(&self, sentence: &str) -> ClassificationResult {
		let english_score = &self.score(sentence, &self.english_profile, &self.english_total);
		let chinese_score = &self.score(sentence, &self.chinese_profile, &self.chinese_total);
		let pinyin_score = &self.score(sentence, &self.pinyin_profile, &self.pinyin_total);

		if chinese_score < english_score && chinese_score < pinyin_score {
			return ClassificationResult::ZH;
		} else if pinyin_score < english_score && pinyin_score < chinese_score {
			return ClassificationResult::PY;
		} else if english_score < pinyin_score && english_score < chinese_score {
			return ClassificationResult::EN;
		} else {
			return ClassificationResult::UN;
		}
	}
}
