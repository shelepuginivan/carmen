use lingua::{LanguageDetector, LanguageDetectorBuilder};

use crate::env::{read_env, read_env_vec};
use crate::error::Result;

use super::Language;

pub struct LangDetector {
    detector: LanguageDetector,
}

impl LangDetector {
    pub fn new_from_env() -> Result<Self> {
        let low_accuracy = read_env("CARMEN_LANG_LOW_ACCURACY_MODE")?.unwrap_or_default();
        let min_distance = read_env::<f64>("CARMEN_LANG_MIN_DISTANCE")?
            .unwrap_or(0.0)
            .clamp(0.0, 0.99);

        let preload_models = read_env("CARMEN_LANG_PRELOAD_MODELS")?.unwrap_or_default();
        let languages: Vec<Language> = read_env_vec("CARMEN_LANG_LIST")?.unwrap_or_else(|| {
            vec![
                Language::Arabic,
                Language::English,
                Language::French,
                Language::German,
                Language::Portuguese,
                Language::Russian,
                Language::Spanish,
            ]
        });

        let languages: Vec<_> = languages.into_iter().map(lingua::Language::from).collect();

        let mut builder = LanguageDetectorBuilder::from_languages(&languages);
        builder.with_minimum_relative_distance(min_distance);
        if low_accuracy {
            builder.with_low_accuracy_mode();
        }
        if preload_models {
            builder.with_preloaded_language_models();
        }

        let detector = builder.build();
        Ok(Self { detector })
    }

    pub fn detect(&self, s: &str) -> Language {
        match self.detector.detect_language_of(s) {
            Some(lang) => lang.into(),
            None => Language::Simple,
        }
    }
}
