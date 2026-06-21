export type Translation = {
  chapterTitle: string;
  text: string;
  translator: string;
  source: string;
};

export type DhammapadaVerse = {
  id: string;
  chapterNumber: number;
  verseNumber: number;
  translations: Record<string, Translation>;
};

export type QuotePayload = {
  id: string;
  chapterNumber: number;
  verseNumber: number;
  chapterTitle: string;
  text: string;
  translator: string;
  source: string;
};

export function pickPrimaryTranslation(verse: DhammapadaVerse): Translation {
  return verse.translations.vi ?? verse.translations.en ?? Object.values(verse.translations)[0] ?? {
    chapterTitle: '',
    text: '',
    translator: '',
    source: '',
  };
}
