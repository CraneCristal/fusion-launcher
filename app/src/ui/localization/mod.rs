use dioxus_sdk::i18n::{Language, use_init_i18n};
use std::str::FromStr;

pub fn init_localization() {
    use_init_i18n("en".parse().unwrap(), "en".parse().unwrap(), || {
        let en = Language::from_str(include_str!("messages/en.json")).unwrap();
        let fr = Language::from_str(include_str!("messages/fr.json")).unwrap();
        vec![en, fr]
    })
}