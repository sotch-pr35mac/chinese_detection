// @author		:: Preston Wang-Stosur-Bassett
// @date 		:: January 24, 2020
// @description		:: This package classifies a string as Chinese characters, Pinyin, or English

use bincode::deserialize_from;
use once_cell::sync::Lazy;
use std::collections::HashMap;

type Profile = HashMap<String, u64>;

static CHINESE_DATA: Lazy<Profile> =
    Lazy::new(|| deserialize_from(&include_bytes!("../profiles/zh.profile")[..]).unwrap());
static ENGLISH_DATA: Lazy<Profile> =
    Lazy::new(|| deserialize_from(&include_bytes!("../profiles/en.profile")[..]).unwrap());
static PINYIN_DATA: Lazy<Profile> =
    Lazy::new(|| deserialize_from(&include_bytes!("../profiles/py.profile")[..]).unwrap());
static ENGLISH_TOTAL: Lazy<u64> = Lazy::new(|| calc_total(&ENGLISH_DATA));
static CHINESE_TOTAL: Lazy<u64> = Lazy::new(|| calc_total(&CHINESE_DATA));
static PINYIN_TOTAL: Lazy<u64> = Lazy::new(|| calc_total(&PINYIN_DATA));
static GRAM_LENGTH: usize = 2;

#[derive(Debug, PartialEq)]
pub enum ClassificationResult {
    ZH, // Chinese
    EN, // English
    PY, // Pinyin
    UN, // Uncertain
}

fn calc_total(profile_data: &HashMap<String, u64>) -> u64 {
    let mut accumulator: u64 = 0;
    for (_, value) in profile_data.iter() {
        accumulator += value;
    }

    accumulator
}

fn calc_prob(substring: &str, profile: &HashMap<String, u64>, total: f64) -> f64 {
    let occurances: f64 = *profile.get(substring).unwrap_or(&1) as f64;

    (occurances / total).log10() * -1_f64
}

fn score(sentence: &str, profile: &HashMap<String, u64>, total: &u64) -> f64 {
    let mut accumulator: f64 = 0.0;
    let words: Vec<_> = sentence.split(' ').collect();
    for word in words {
        if word.chars().count() < GRAM_LENGTH {
            accumulator += calc_prob(word, profile, *total as f64);
        } else {
            let mut i = 0;
            while i < (word.chars().count() - (GRAM_LENGTH - 1)) {
                let substring: String = word.chars().skip(i).take(GRAM_LENGTH).collect();
                accumulator += calc_prob(&substring, profile, *total as f64);
                i += 1;
            }
        }
    }

    accumulator
}

pub fn init() {
    Lazy::force(&ENGLISH_TOTAL);
    Lazy::force(&CHINESE_TOTAL);
    Lazy::force(&PINYIN_TOTAL);
    Lazy::force(&CHINESE_DATA);
    Lazy::force(&ENGLISH_DATA);
    Lazy::force(&PINYIN_DATA);
}

pub fn classify(sentence: &str) -> ClassificationResult {
    let english_score = score(sentence, &ENGLISH_DATA, &ENGLISH_TOTAL);
    let chinese_score = score(sentence, &CHINESE_DATA, &CHINESE_TOTAL);
    let pinyin_score = score(sentence, &PINYIN_DATA, &PINYIN_TOTAL);

    if chinese_score < english_score && chinese_score < pinyin_score {
        ClassificationResult::ZH
    } else if pinyin_score < english_score && pinyin_score < chinese_score {
        ClassificationResult::PY
    } else if english_score < pinyin_score && english_score < chinese_score {
        ClassificationResult::EN
    } else {
        ClassificationResult::UN
    }
}
