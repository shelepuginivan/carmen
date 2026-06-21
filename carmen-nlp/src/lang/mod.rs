mod detect;

pub use detect::*;

/// Languages that are supported by both tsvector and lingua.
///
/// ```sql
/// SELECT oid, * FROM pg_ts_config;
/// ```
///
/// Since other languages are not supported, they are not detected and cannot be enabled in
/// configuration.
#[derive(strum::Display, strum::EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Language {
    #[strum(disabled)]
    Simple,
    #[strum(serialize = "ar", serialize = "ara", serialize = "Arabic")]
    Arabic,
    #[strum(
        serialize = "hy",
        serialize = "hye",
        serialize = "arm",
        serialize = "Armenian"
    )]
    Armenian,
    #[strum(
        serialize = "eu",
        serialize = "eus",
        serialize = "baq",
        serialize = "Basque"
    )]
    Basque,
    #[strum(serialize = "ca", serialize = "cat", serialize = "Catalan")]
    Catalan,
    #[strum(serialize = "da", serialize = "dan", serialize = "Danish")]
    Danish,
    #[strum(
        serialize = "nl",
        serialize = "nld",
        serialize = "dut",
        serialize = "Dutch"
    )]
    Dutch,
    #[strum(serialize = "en", serialize = "eng", serialize = "English")]
    English,
    #[strum(serialize = "et", serialize = "est", serialize = "Estonian")]
    Estonian,
    #[strum(serialize = "fi", serialize = "fin", serialize = "Finnish")]
    Finnish,
    #[strum(
        serialize = "fr",
        serialize = "fra",
        serialize = "fre",
        serialize = "French"
    )]
    French,
    #[strum(
        serialize = "de",
        serialize = "deu",
        serialize = "ger",
        serialize = "German"
    )]
    German,
    #[strum(
        serialize = "el",
        serialize = "ell",
        serialize = "gre",
        serialize = "Greek"
    )]
    Greek,
    #[strum(serialize = "hi", serialize = "hin", serialize = "Hindi")]
    Hindi,
    #[strum(serialize = "hu", serialize = "hun", serialize = "Hungarian")]
    Hungarian,
    #[strum(serialize = "id", serialize = "ind", serialize = "Indonesian")]
    Indonesian,
    #[strum(serialize = "ga", serialize = "gle", serialize = "Irish")]
    Irish,
    #[strum(serialize = "it", serialize = "ita", serialize = "Italian")]
    Italian,
    #[strum(serialize = "lt", serialize = "lit", serialize = "Lithuanian")]
    Lithuanian,
    #[strum(serialize = "pt", serialize = "por", serialize = "Portuguese")]
    Portuguese,
    #[strum(
        serialize = "ro",
        serialize = "ron",
        serialize = "rum",
        serialize = "Romanian"
    )]
    Romanian,
    #[strum(serialize = "ru", serialize = "rus", serialize = "Russian")]
    Russian,
    #[strum(serialize = "sr", serialize = "srp", serialize = "Serbian")]
    Serbian,
    #[strum(serialize = "es", serialize = "spa", serialize = "Spanish")]
    Spanish,
    #[strum(serialize = "sv", serialize = "swe", serialize = "Swedish")]
    Swedish,
    #[strum(serialize = "ta", serialize = "tam", serialize = "Tamil")]
    Tamil,
    #[strum(serialize = "tr", serialize = "tur", serialize = "Turkish")]
    Turkish,
}

impl From<lingua::Language> for Language {
    fn from(value: lingua::Language) -> Self {
        match value {
            lingua::Language::Arabic => Self::Arabic,
            lingua::Language::Armenian => Self::Armenian,
            lingua::Language::Basque => Self::Basque,
            lingua::Language::Catalan => Self::Catalan,
            lingua::Language::Danish => Self::Danish,
            lingua::Language::Dutch => Self::Dutch,
            lingua::Language::English => Self::English,
            lingua::Language::Estonian => Self::Estonian,
            lingua::Language::Finnish => Self::Finnish,
            lingua::Language::French => Self::French,
            lingua::Language::German => Self::German,
            lingua::Language::Greek => Self::Greek,
            lingua::Language::Hindi => Self::Hindi,
            lingua::Language::Hungarian => Self::Hungarian,
            lingua::Language::Indonesian => Self::Indonesian,
            lingua::Language::Irish => Self::Irish,
            lingua::Language::Italian => Self::Italian,
            lingua::Language::Lithuanian => Self::Lithuanian,
            lingua::Language::Portuguese => Self::Portuguese,
            lingua::Language::Romanian => Self::Romanian,
            lingua::Language::Russian => Self::Russian,
            lingua::Language::Serbian => Self::Serbian,
            lingua::Language::Spanish => Self::Spanish,
            lingua::Language::Swedish => Self::Swedish,
            lingua::Language::Tamil => Self::Tamil,
            lingua::Language::Turkish => Self::Turkish,

            // For some reason rust-analyzer complains here, even though all enum members are
            // guarded behind lingua features.
            #[allow(unreachable_patterns)]
            _ => Self::Simple,
        }
    }
}

impl From<Language> for lingua::Language {
    fn from(value: Language) -> Self {
        match value {
            Language::Arabic => Self::Arabic,
            Language::Armenian => Self::Armenian,
            Language::Basque => Self::Basque,
            Language::Catalan => Self::Catalan,
            Language::Danish => Self::Danish,
            Language::Dutch => Self::Dutch,
            Language::English => Self::English,
            Language::Estonian => Self::Estonian,
            Language::Finnish => Self::Finnish,
            Language::French => Self::French,
            Language::German => Self::German,
            Language::Greek => Self::Greek,
            Language::Hindi => Self::Hindi,
            Language::Hungarian => Self::Hungarian,
            Language::Indonesian => Self::Indonesian,
            Language::Irish => Self::Irish,
            Language::Italian => Self::Italian,
            Language::Lithuanian => Self::Lithuanian,
            Language::Portuguese => Self::Portuguese,
            Language::Romanian => Self::Romanian,
            Language::Russian => Self::Russian,
            Language::Serbian => Self::Serbian,
            Language::Spanish => Self::Spanish,
            Language::Swedish => Self::Swedish,
            Language::Tamil => Self::Tamil,
            Language::Turkish => Self::Turkish,

            // Unreachable since this conversion is only used when parsing env vars.
            // Language::Simple is disabled for strum::EnumString.
            Language::Simple => unreachable!(),
        }
    }
}
