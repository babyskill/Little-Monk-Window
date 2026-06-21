use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct DhammapadaVerse {
    pub id: String,
    #[serde(rename = "chapterNumber")]
    pub chapter_number: u32,
    #[serde(rename = "verseNumber")]
    pub verse_number: u32,
    pub translations: HashMap<String, Translation>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Translation {
    #[serde(rename = "chapterTitle")]
    pub chapter_title: String,
    pub text: String,
    #[serde(default)]
    pub translator: String,
    #[serde(default)]
    pub source: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct QuotePayload {
    pub id: String,
    #[serde(rename = "chapterNumber")]
    pub chapter_number: u32,
    #[serde(rename = "verseNumber")]
    pub verse_number: u32,
    #[serde(rename = "chapterTitle")]
    pub chapter_title: String,
    pub text: String,
    pub translator: String,
    pub source: String,
}

impl From<(DhammapadaVerse, Translation)> for QuotePayload {
    fn from((verse, translation): (DhammapadaVerse, Translation)) -> Self {
        Self {
            id: verse.id,
            chapter_number: verse.chapter_number,
            verse_number: verse.verse_number,
            chapter_title: translation.chapter_title,
            text: translation.text,
            translator: translation.translator,
            source: translation.source,
        }
    }
}

pub struct QuoteLibrary {
    verses: Vec<DhammapadaVerse>,
}

impl QuoteLibrary {
    pub fn load_from_bundle() -> anyhow::Result<Self> {
        let raw = include_str!("../resources/Dhammapada.json");
        let verses: Vec<DhammapadaVerse> = serde_json::from_str(raw)?;
        anyhow::ensure!(!verses.is_empty(), "Dhammapada library is empty");
        Ok(Self { verses })
    }

    pub fn quote_for_time(&self, unix_seconds: u64) -> QuotePayload {
        let slot = unix_seconds / 300;
        let index = (slot as usize) % self.verses.len();
        let verse = self.verses[index].clone();
        let translation = verse
            .translations
            .get("vi")
            .cloned()
            .or_else(|| verse.translations.get("en").cloned())
            .or_else(|| verse.translations.values().next().cloned())
            .unwrap_or(Translation {
                chapter_title: String::new(),
                text: String::new(),
                translator: String::new(),
                source: String::new(),
            });
        QuotePayload::from((verse, translation))
    }
}
