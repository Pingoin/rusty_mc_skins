use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use dioxus_i18n::unic_langid::{LanguageIdentifier, langid};

pub fn german() -> LanguageIdentifier {
    langid!("de")
}

pub fn english() -> LanguageIdentifier {
    langid!("en")
}

/// Initialize the Fluent-based i18n provider. Must be called once in the root component.
pub fn init_i18n() -> I18n {
    use_init_i18n(|| {
        I18nConfig::new(german())
            .with_locale((german(), include_str!("../locales/de.ftl")))
            .with_locale((english(), include_str!("../locales/en.ftl")))
            .with_fallback(english())
    })
}

#[component]
pub fn LanguageSwitcher() -> Element {
    let mut i18n = i18n();
    let current = i18n.language();
    rsx! {
        div { class: "join",
            button {
                class: if current == german() { "join-item btn btn-sm btn-active" } else { "join-item btn btn-sm" },
                onclick: move |_| i18n.set_language(german()),
                title: "Deutsch",
                aria_label: "Deutsch",
                // Flag als Asset-Symbol statt Text
                crate::components::FlagDeIcon {}
            }
            button {
                class: if current == english() { "join-item btn btn-sm btn-active" } else { "join-item btn btn-sm" },
                onclick: move |_| i18n.set_language(english()),
                title: "English",
                aria_label: "English",
                crate::components::FlagGbIcon {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    fn message_ids(ftl: &str) -> BTreeSet<String> {
        let mut ids = BTreeSet::new();
        let mut current: Option<String> = None;
        for line in ftl.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            if line.starts_with([' ', '\t']) {
                if let Some(attr) = trimmed.strip_prefix('.') {
                    if let Some(name) = attr.split('=').next() {
                        if let Some(parent) = &current {
                            ids.insert(format!("{parent}.{}", name.trim()));
                        }
                    }
                }
                continue;
            }
            if let Some((key, _)) = trimmed.split_once('=') {
                let key = key.trim().to_string();
                current = Some(key.clone());
                ids.insert(key);
            }
        }
        ids
    }

    #[test]
    fn locales_define_the_same_message_ids() {
        let de = message_ids(include_str!("../locales/de.ftl"));
        let en = message_ids(include_str!("../locales/en.ftl"));
        assert!(!de.is_empty(), "de.ftl contains no messages");
        assert_eq!(de, en, "de.ftl and en.ftl define different message ids");
    }
}
