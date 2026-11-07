//! Evora fork (patch EVORA-SIN-46, decision Q-EVO-1): on screen the user reads the app's name, never
//! the engine's. The catalogs and the source strings keep the engine's name, so they survive an
//! upgrade and what agents, tests and files read doesn't change; only what the interface displays
//! does: [`super::tr`] and its siblings pass their result through [`rename`], and the few strings
//! drawn without a translation call it themselves. Identifiers, file extensions, format signatures
//! and the licence files are never drawn through here.
//!
//! The name itself is replaced whatever the language. The grammar around it is listed in [`FORMS`]
//! and in [`shown_text`], from a survey of the 13 catalogs: English writes "an" before the old name;
//! Czech declines it, Hungarian adds a suffix, German makes compounds; French, Italian, Spanish,
//! Portuguese and the others write it bare ("de EvoVector", "di EvoVector", "do EvoVector"), which
//! reads right with the new one.

use std::cell::RefCell;
use std::collections::HashMap;

/// The name this build shows.
pub const SHOWN_NAME: &str = "Sinopia Pro";

/// The engines' names as the catalogs write them: this one's, and its siblings' in the suite.
const ENGINE_NAMES: [&str; 7] = ["EvoVector", "EvoFilm", "EvoPhoto", "EvoLight", "EvoEffect", "EvoPdf", "EvoDesign"];

/// Forms of the name to settle first, in this order: (as written, what reads right).
const FORMS: [(&str, &str); 8] = [
    // Czech: "ve" and the genitive or locative ending; the new name isn't declined, and takes "v".
    ("ve EvoVectoru", "v EvoVector"),
    ("EvoVectoru", "EvoVector"),
    // Hungarian: a suffix on a name that ends in a vowel takes a hyphen.
    ("EvoVectorból", "EvoVector-ból"),
    // German: a compound on a name of two words is hyphenated throughout.
    ("EvoVector-Datei", "Sinopia-Pro-Datei"),
    ("EvoVector-Dokumente", "Sinopia-Pro-Dokumente"),
    ("EvoVector-Hilfe", "Sinopia-Pro-Hilfe"),
    ("EvoVector-Tastaturbefehlssatz", "Sinopia-Pro-Tastaturbefehlssatz"),
    // The command-line tool, named in one message, in every language.
    ("evovector-cli", "EvoVector CLI"),
];

thread_local! {
    /// Displayed text → the same text with the app's name, made once per text (the interface asks
    /// for the same strings every frame).
    static RENAMED: RefCell<HashMap<String, &'static str>> = RefCell::new(HashMap::new());
}

/// Does `text` name an engine? ("Evora" and "Evolution" don't.)
fn names_an_engine(text: &str) -> bool {
    text.contains("Evo") && ENGINE_NAMES.iter().any(|name| text.contains(*name))
}

/// `text` with `from` replaced where it starts a word: "an X", not the end of "can X".
fn at_word_start(text: &str, from: &str, to: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some((before, after)) = rest.split_once(from) {
        out.push_str(before);
        let starts_word = !out.chars().next_back().is_some_and(char::is_alphanumeric);
        out.push_str(if starts_word { to } else { from });
        rest = after;
    }
    out.push_str(rest);
    out
}

fn shown_text(text: &str) -> String {
    let mut out = text.to_string();
    for (form, bare) in FORMS {
        out = at_word_start(&out, form, bare);
    }
    for name in ENGINE_NAMES {
        // The old names start with a vowel, the new one doesn't.
        out = at_word_start(&out, &format!("an {name}"), &format!("a {SHOWN_NAME}"));
        out = at_word_start(&out, &format!("An {name}"), &format!("A {SHOWN_NAME}"));
        out = out.replace(name, SHOWN_NAME);
    }
    out
}

/// `text` as this build shows it: with the app's name instead of an engine's. Text without one
/// (almost all) comes back as it is, without a lookup.
pub fn rename(text: &str) -> &str {
    if !names_an_engine(text) {
        return text;
    }
    RENAMED.with(|cache| -> &'static str {
        let mut cache = cache.borrow_mut();
        if let Some(&shown) = cache.get(text) {
            return shown;
        }
        let shown: &'static str = Box::leak(shown_text(text).into_boxed_str());
        cache.insert(text.to_string(), shown);
        shown
    })
}

/// [`rename`] for a text made on the fly (a message with a file name in it): nothing is kept.
pub(super) fn rename_owned(text: String) -> String {
    if names_an_engine(&text) { shown_text(&text) } else { text }
}

#[cfg(test)]
mod tests {
    use super::super::catalog::parse_entries;
    use super::super::{LANGUAGES, Lang, message, tr, tr_ctx, tr_id, trn};
    use super::{ENGINE_NAMES, SHOWN_NAME, rename};

    fn lang(code: &str) -> Lang {
        Lang::from_code(code).expect("registered")
    }

    /// Q-EVO-1: every lookup shows the app's name, in English (the source) as in a catalog.
    #[test]
    fn the_app_is_called_sinopia_pro_on_screen() {
        assert_eq!(tr(Lang::EN, "About EvoVector"), "About Sinopia Pro");
        assert_eq!(tr(Lang::EN, "Welcome to EvoVector"), "Welcome to Sinopia Pro");
        assert_eq!(tr_id(Lang::EN, "help.about", "About EvoVector"), "About Sinopia Pro");
        assert_eq!(tr_ctx(Lang::EN, "file-filter", "EvoVector Template"), "Sinopia Pro Template");
        assert_eq!(tr(lang("fr"), "Welcome to EvoVector"), "Bienvenue dans Sinopia Pro");
        assert_eq!(tr_ctx(lang("cs"), "file-filter", "EvoVector Template"), "Šablona Sinopia Pro");
        assert_eq!(trn(Lang::EN, 2, "{n} EvoVector file", "{n} EvoVector files"), "2 Sinopia Pro files");
        // The source stays the key: the message is found in the catalog, then renamed.
        assert_eq!(message(Lang::EN, "not an EvoVector file: poster"), "not a Sinopia Pro file: poster");
        let french = message(lang("fr"), "not an EvoVector file: poster");
        assert!(french.contains(SHOWN_NAME) && french.contains("fichier") && french.contains("poster"), "{french}");
    }

    #[test]
    fn the_grammar_follows_the_new_name() {
        assert_eq!(rename("not an EvoVector file"), "not a Sinopia Pro file");
        assert_eq!(rename("An EvoVector document"), "A Sinopia Pro document");
        // "an" as a word only.
        assert_eq!(rename("Can EvoVector read it?"), "Can Sinopia Pro read it?");
        assert_eq!(rename("Vítejte ve EvoVectoru"), "Vítejte v Sinopia Pro");
        assert_eq!(rename("Načítání EvoVectoru…"), "Načítání Sinopia Pro…");
        assert_eq!(rename("Kilépés a EvoVectorból"), "Kilépés a Sinopia Pro-ból");
        assert_eq!(rename("keine EvoVector-Datei: poster"), "keine Sinopia-Pro-Datei: poster");
        assert_eq!(tr(lang("de"), "EvoVector Help…"), "Sinopia-Pro-Hilfe…");
        // Hungarian keeps its one hyphen.
        assert_eq!(rename("nem EvoVector-fájl: poster"), "nem Sinopia Pro-fájl: poster");
        assert_eq!(rename("a tool: EvoVector and evovector-cli"), "a tool: Sinopia Pro and Sinopia Pro CLI");
        // A sibling engine's name too.
        assert_eq!(rename("EvoPhoto"), SHOWN_NAME);
        assert_eq!(rename("an EvoPhoto profile"), "a Sinopia Pro profile");
    }

    #[test]
    fn what_is_not_the_name_stays() {
        for text in ["File", "Poster.evovector", "evovector-shortcuts", "Evora", "Evolution"] {
            assert!(std::ptr::eq(rename(text), text), "{text}");
        }
        // The extension of a format is not the name.
        assert_eq!(rename("Saved EvoVector logo.evovector"), "Saved Sinopia Pro logo.evovector");
        // Renamed text is made once.
        assert!(std::ptr::eq(rename("EvoVector Project"), rename("EvoVector Project")));
    }

    /// An upgrade that brings a new way of saying the name, or the team's, shows here.
    #[test]
    fn no_catalog_shows_an_engine_or_its_team() {
        const NEVER: [&str; 6] = ["Evo Apps", "Upstream", "evorules", "evovector-cli", " an Sinopia", "ve Sinopia"];
        let mut shown = Vec::new();
        for l in &LANGUAGES {
            for (ctx, src, text) in parse_entries(l.source).0 {
                for t in [src.as_str(), text.as_str()] {
                    let r = rename(t);
                    if ENGINE_NAMES.iter().chain(&NEVER).any(|n| r.contains(*n)) {
                        shown.push(format!("{} [{ctx}]: {t:?} → {r:?}", l.code));
                    }
                }
            }
        }
        assert!(shown.is_empty(), "shown: {shown:#?}");
    }
}
