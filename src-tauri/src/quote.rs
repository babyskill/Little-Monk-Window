use crate::settings;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io::Read};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DhammapadaVerse {
    #[serde(default)]
    pub id: String,
    pub chapter_number: u32,
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LegacyDhammapadaVerse {
    #[serde(default)]
    id: String,
    chapter_number: u32,
    chapter_title: String,
    verse_number: u32,
    text: String,
    #[serde(default)]
    translator: String,
    #[serde(default)]
    source: String,
}

impl From<LegacyDhammapadaVerse> for DhammapadaVerse {
    fn from(value: LegacyDhammapadaVerse) -> Self {
        let mut translations = HashMap::new();
        translations.insert(
            "vi".to_string(),
            Translation {
                chapter_title: value.chapter_title,
                text: value.text,
                translator: value.translator,
                source: value.source,
            },
        );

        Self {
            id: value.id,
            chapter_number: value.chapter_number,
            verse_number: value.verse_number,
            translations,
        }
    }
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
    pub fn load() -> anyhow::Result<Self> {
        if let Some(verses) = Self::load_custom()? {
            if !verses.is_empty() {
                return Ok(Self { verses });
            }
        }

        Self::load_from_bundle()
    }

    pub fn load_from_bundle() -> anyhow::Result<Self> {
        let raw = include_str!("../resources/Dhammapada.json");
        let verses = Self::parse_verses(raw.as_bytes())?;
        anyhow::ensure!(!verses.is_empty(), "Dhammapada library is empty");
        Ok(Self { verses })
    }

    pub fn list(&self) -> Vec<DhammapadaVerse> {
        let mut verses = self.verses.clone();
        verses.sort_by(|left, right| {
            left.chapter_number
                .cmp(&right.chapter_number)
                .then(left.verse_number.cmp(&right.verse_number))
                .then(left.id.cmp(&right.id))
        });
        verses
    }

    pub fn upsert(&mut self, verse: DhammapadaVerse) -> anyhow::Result<DhammapadaVerse> {
        let fixed = Self::normalized(verse);
        anyhow::ensure!(
            !fixed.translations.is_empty(),
            "Verse must have at least one non-empty translation"
        );
        if let Some(index) = self.verses.iter().position(|item| item.id == fixed.id) {
            self.verses[index] = fixed.clone();
        } else {
            self.verses.push(fixed.clone());
        }

        self.persist()?;
        Ok(fixed)
    }

    pub fn remove(&mut self, id: &str) -> anyhow::Result<bool> {
        anyhow::ensure!(self.verses.len() > 1, "Cannot delete the last verse");
        let before = self.verses.len();
        self.verses.retain(|verse| verse.id != id);
        let changed = before != self.verses.len();
        if changed {
            self.persist()?;
        }
        Ok(changed)
    }

    pub fn reset_to_bundle(&mut self) -> anyhow::Result<Vec<DhammapadaVerse>> {
        let bundled = Self::load_from_bundle()?.verses;
        self.verses = bundled;
        self.persist()?;
        Ok(self.list())
    }

    pub fn update_from_github_zip(&mut self, zip_url: &str) -> anyhow::Result<usize> {
        let response = reqwest::blocking::get(zip_url)?.error_for_status()?;
        let bytes = response.bytes()?;
        let reader = std::io::Cursor::new(bytes);
        let mut archive = zip::ZipArchive::new(reader)?;
        let mut translations_by_lang: HashMap<String, HashMap<(u32, u32), Translation>> =
            HashMap::new();

        for index in 0..archive.len() {
            let mut file = archive.by_index(index)?;
            if file.is_dir() || !file.name().to_ascii_lowercase().ends_with(".json") {
                continue;
            }

            let stem = std::path::Path::new(file.name())
                .file_stem()
                .and_then(|value| value.to_str())
                .unwrap_or_default()
                .to_ascii_lowercase();
            if stem.len() > 8
                || ["package", "tsconfig", "package-lock", "manifest"].contains(&stem.as_str())
            {
                continue;
            }

            let mut raw = String::new();
            file.read_to_string(&mut raw)?;
            Self::collect_translations(&raw, &stem, &mut translations_by_lang);
        }

        anyhow::ensure!(
            !translations_by_lang.is_empty(),
            "No valid translation JSON files found in ZIP"
        );

        let mut keys: Vec<(u32, u32)> = translations_by_lang
            .values()
            .flat_map(|verses| verses.keys().copied())
            .collect();
        keys.sort_unstable();
        keys.dedup();

        let mut merged = Vec::with_capacity(keys.len());
        for (chapter_number, verse_number) in keys {
            let mut translations = HashMap::new();
            for (lang, verses) in &translations_by_lang {
                if let Some(translation) = verses.get(&(chapter_number, verse_number)) {
                    translations.insert(lang.clone(), translation.clone());
                }
            }

            merged.push(DhammapadaVerse {
                id: Self::verse_id(chapter_number, verse_number),
                chapter_number,
                verse_number,
                translations,
            });
        }

        anyhow::ensure!(!merged.is_empty(), "No verses were imported from ZIP");
        self.verses = merged.into_iter().map(Self::normalized).collect();
        let count = self.verses.len();
        self.persist()?;
        Ok(count)
    }

    pub fn quote_for_time(&self, unix_seconds: u64, language: &str) -> QuotePayload {
        let slot = unix_seconds / 300;
        let index = (slot as usize) % self.verses.len();
        let verse = self.verses[index].clone();
        let translation = verse
            .translations
            .get(language)
            .cloned()
            .or_else(|| verse.translations.get("vi").cloned())
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

    pub fn next_quote(&self, current_id: &str, language: &str) -> QuotePayload {
        if self.verses.is_empty() {
            return QuotePayload {
                id: String::new(),
                chapter_number: 0,
                verse_number: 0,
                chapter_title: String::new(),
                text: String::new(),
                translator: String::new(),
                source: String::new(),
            };
        }
        let index = self
            .verses
            .iter()
            .position(|v| v.id == current_id)
            .map(|pos| (pos + 1) % self.verses.len())
            .unwrap_or(0);
        let verse = self.verses[index].clone();
        let translation = verse
            .translations
            .get(language)
            .cloned()
            .or_else(|| verse.translations.get("vi").cloned())
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

    fn parse_verses(raw: &[u8]) -> anyhow::Result<Vec<DhammapadaVerse>> {
        if let Ok(verses) = serde_json::from_slice::<Vec<DhammapadaVerse>>(raw) {
            return Ok(verses.into_iter().map(Self::normalized).collect());
        }

        let legacy = serde_json::from_slice::<Vec<LegacyDhammapadaVerse>>(raw)?;
        Ok(legacy
            .into_iter()
            .map(DhammapadaVerse::from)
            .map(Self::normalized)
            .collect())
    }

    fn collect_translations(
        raw: &str,
        fallback_lang: &str,
        output: &mut HashMap<String, HashMap<(u32, u32), Translation>>,
    ) {
        if let Ok(legacy) = serde_json::from_str::<Vec<LegacyDhammapadaVerse>>(raw) {
            let verses = output.entry(fallback_lang.to_string()).or_default();
            for verse in legacy {
                verses.insert(
                    (verse.chapter_number, verse.verse_number),
                    Translation {
                        chapter_title: verse.chapter_title,
                        text: verse.text,
                        translator: verse.translator,
                        source: verse.source,
                    },
                );
            }
            return;
        }

        if let Ok(multilingual) = serde_json::from_str::<Vec<DhammapadaVerse>>(raw) {
            for verse in multilingual {
                for (lang, translation) in verse.translations {
                    output
                        .entry(lang)
                        .or_default()
                        .insert((verse.chapter_number, verse.verse_number), translation);
                }
            }
        }
    }

    fn load_custom() -> anyhow::Result<Option<Vec<DhammapadaVerse>>> {
        let path = Self::custom_path()?;
        if path.exists() {
            let data = std::fs::read(&path)?;
            return Ok(Some(Self::parse_verses(&data)?));
        }

        let legacy_path = settings::app_config_dir()?.join("dhammapada-custom.vi.json");
        if legacy_path.exists() {
            let data = std::fs::read(&legacy_path)?;
            let verses = Self::parse_verses(&data)?;
            Self::persist_to_path(&path, &verses)?;
            let _ = std::fs::remove_file(legacy_path);
            return Ok(Some(verses));
        }

        Ok(None)
    }

    fn persist(&self) -> anyhow::Result<()> {
        Self::persist_to_path(&Self::custom_path()?, &self.list())
    }

    fn persist_to_path(path: &std::path::Path, verses: &[DhammapadaVerse]) -> anyhow::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let data = serde_json::to_string_pretty(verses)?;
        std::fs::write(path, data)?;
        Ok(())
    }

    fn custom_path() -> anyhow::Result<std::path::PathBuf> {
        Ok(settings::app_config_dir()?.join("dhammapada-custom.json"))
    }

    fn normalized(mut verse: DhammapadaVerse) -> DhammapadaVerse {
        if verse.id.trim().is_empty() {
            verse.id = Self::verse_id(verse.chapter_number, verse.verse_number);
        }

        verse.translations = verse
            .translations
            .into_iter()
            .map(|(lang, translation)| {
                (
                    lang.trim().to_ascii_lowercase(),
                    Translation {
                        chapter_title: translation.chapter_title.trim().to_string(),
                        text: translation.text.trim().to_string(),
                        translator: translation.translator.trim().to_string(),
                        source: translation.source.trim().to_string(),
                    },
                )
            })
            .filter(|(lang, translation)| !lang.is_empty() && !translation.text.is_empty())
            .collect();
        verse
    }

    fn verse_id(chapter_number: u32, verse_number: u32) -> String {
        format!("verse-{chapter_number}-{verse_number}")
    }
}
