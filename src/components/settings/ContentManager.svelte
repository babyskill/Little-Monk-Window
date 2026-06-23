<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import type { DhammapadaVerse, QuotePayload, Translation } from '../../lib/quote';
  import { pickPrimaryTranslation } from '../../lib/quote';

  type EditMode = 'add' | 'edit' | null;

  const defaultZipUrl = 'https://github.com/babyskill/dhammapada-data/archive/refs/heads/main.zip';

  let verses: DhammapadaVerse[] = [];
  let currentQuote: QuotePayload | null = null;
  let query = '';
  let updateUrl = defaultZipUrl;
  let isUpdating = false;
  let updateMessage = '';
  let error = '';
  let editMode: EditMode = null;
  let editing: DhammapadaVerse = blankVerse();
  let editLanguage = 'vi';

  $: matchingVerses = verses
    .filter(matchesQuery)
    .sort((left, right) => {
      if (left.chapterNumber !== right.chapterNumber) return left.chapterNumber - right.chapterNumber;
      if (left.verseNumber !== right.verseNumber) return left.verseNumber - right.verseNumber;
      return primary(left).chapterTitle.localeCompare(primary(right).chapterTitle);
    });

  $: activeTranslation = editing.translations[editLanguage] ?? blankTranslation();

  onMount(() => {
    let unlisten: (() => void) | undefined;
    void refresh();
    void listen('content:changed', refresh).then((cleanup) => {
      unlisten = cleanup;
    });
    return () => {
      unlisten?.();
    };
  });

  function blankTranslation(): Translation {
    return {
      chapterTitle: '',
      text: '',
      translator: '',
      source: '',
    };
  }

  function blankVerse(): DhammapadaVerse {
    return {
      id: '',
      chapterNumber: 1,
      verseNumber: 1,
      translations: {
        vi: {
          chapterTitle: 'Phẩm mới',
          text: '',
          translator: '',
          source: '',
        },
      },
    };
  }

  function primary(verse: DhammapadaVerse): Translation {
    return pickPrimaryTranslation(verse);
  }

  function matchesQuery(verse: DhammapadaVerse) {
    const trimmed = query.trim().toLowerCase();
    if (!trimmed) return true;

    const translations = Object.values(verse.translations);
    return (
      String(verse.chapterNumber).includes(trimmed) ||
      String(verse.verseNumber).includes(trimmed) ||
      translations.some((translation) =>
        [
          translation.chapterTitle,
          translation.text,
          translation.translator,
          translation.source,
        ].some((value) => value.toLowerCase().includes(trimmed)),
      )
    );
  }

  async function refresh() {
    try {
      verses = await invoke<DhammapadaVerse[]>('list_verses');
      currentQuote = await invoke<QuotePayload>('current_quote');
      error = '';
    } catch (err) {
      error = String(err);
    }
  }

  function startAdd() {
    editing = blankVerse();
    editLanguage = 'vi';
    editMode = 'add';
  }

  function startEdit(verse: DhammapadaVerse) {
    editing = JSON.parse(JSON.stringify(verse)) as DhammapadaVerse;
    editLanguage = editing.translations.vi ? 'vi' : Object.keys(editing.translations)[0] ?? 'vi';
    editMode = 'edit';
  }

  function updateTranslation(patch: Partial<Translation>) {
    editing.translations = {
      ...editing.translations,
      [editLanguage]: {
        ...blankTranslation(),
        ...activeTranslation,
        ...patch,
      },
    };
  }

  async function saveVerse() {
    try {
      const saved = await invoke<DhammapadaVerse>('upsert_verse', { verse: editing });
      const index = verses.findIndex((verse) => verse.id === saved.id);
      verses = index >= 0
        ? verses.map((verse) => (verse.id === saved.id ? saved : verse))
        : [...verses, saved];
      editMode = null;
      await refresh();
    } catch (err) {
      error = String(err);
    }
  }

  async function deleteVerse(verse: DhammapadaVerse) {
    if (!window.confirm(`Xóa Phẩm ${verse.chapterNumber} • Câu ${verse.verseNumber}?`)) {
      return;
    }

    try {
      await invoke('delete_verse', { id: verse.id });
      verses = verses.filter((item) => item.id !== verse.id);
      await refresh();
    } catch (err) {
      error = String(err);
    }
  }

  async function resetVerses() {
    if (!window.confirm('Khôi phục toàn bộ nội dung về bộ Dhammapada mặc định?')) {
      return;
    }

    try {
      verses = await invoke<DhammapadaVerse[]>('reset_verses');
      await refresh();
    } catch (err) {
      error = String(err);
    }
  }

  async function updateFromGithub() {
    const zipUrl = updateUrl.trim();
    if (!zipUrl) return;

    isUpdating = true;
    updateMessage = '';
    error = '';
    try {
      const count = await invoke<number>('update_verses_from_github', { zipUrl });
      updateMessage = `Đã cập nhật ${count} câu.`;
      await refresh();
    } catch (err) {
      error = String(err);
    } finally {
      isUpdating = false;
    }
  }
</script>

<div class="content-grid">
  <section class="section">
    <div class="section-title">Cập nhật dữ liệu từ GitHub</div>
    <div class="zip-row">
      <input bind:value={updateUrl} placeholder="URL ZIP GitHub" disabled={isUpdating} />
      <button class="primary" disabled={isUpdating || !updateUrl.trim()} on:click={updateFromGithub}>
        {isUpdating ? 'Đang cập nhật...' : 'Cập nhật ngay'}
      </button>
    </div>
    {#if updateMessage}<p class="success">{updateMessage}</p>{/if}
    {#if error}<p class="error">{error}</p>{/if}
  </section>

  <section class="section">
    <div class="toolbar">
      <input bind:value={query} placeholder="Tìm phẩm / câu" />
      <button class="primary" on:click={startAdd}>Thêm câu</button>
      <button class="ghost" on:click={resetVerses}>Khôi phục bộ mặc định</button>
    </div>

    <div class="list">
      {#if matchingVerses.length === 0}
        <p class="empty">Không có câu phù hợp.</p>
      {:else}
        {#each matchingVerses.slice(0, 80) as verse (verse.id)}
          <article class="verse-row">
            <div>
              <div class="meta">Phẩm {verse.chapterNumber} • Câu {verse.verseNumber}</div>
              <h3>{primary(verse).chapterTitle}</h3>
              <p>{primary(verse).text}</p>
            </div>
            <div class="actions">
              <button on:click={() => startEdit(verse)}>Sửa</button>
              <button class="danger" on:click={() => deleteVerse(verse)}>Xóa</button>
            </div>
          </article>
        {/each}
      {/if}
    </div>
  </section>

  <section class="section current">
    <div class="section-title">Câu hiện tại</div>
    {#if currentQuote}
      <p>{currentQuote.text}</p>
      <span>{currentQuote.chapterTitle} • {currentQuote.translator || 'Kinh Pháp Cú'}</span>
    {/if}
  </section>
</div>

{#if editMode}
  <div class="modal-backdrop" role="presentation">
    <form class="editor" on:submit|preventDefault={saveVerse}>
      <div class="editor-head">
        <h2>{editMode === 'add' ? 'Thêm câu' : 'Sửa câu'}</h2>
        <button type="button" class="icon" on:click={() => (editMode = null)}>×</button>
      </div>

      <div class="fields two">
        <label>
          <span>Số phẩm</span>
          <input type="number" min="1" bind:value={editing.chapterNumber} required />
        </label>
        <label>
          <span>Số câu</span>
          <input type="number" min="1" bind:value={editing.verseNumber} required />
        </label>
      </div>

      <div class="fields">
        <label>
          <span>Ngôn ngữ</span>
          <select bind:value={editLanguage}>
            <option value="vi">Tiếng Việt</option>
            <option value="en">Tiếng Anh</option>
          </select>
        </label>
        <label>
          <span>Tên phẩm</span>
          <input value={activeTranslation.chapterTitle} on:input={(event) => updateTranslation({ chapterTitle: event.currentTarget.value })} required />
        </label>
        <label>
          <span>Nội dung</span>
          <textarea value={activeTranslation.text} rows="5" on:input={(event) => updateTranslation({ text: event.currentTarget.value })} required></textarea>
        </label>
        <label>
          <span>Dịch giả</span>
          <input value={activeTranslation.translator} on:input={(event) => updateTranslation({ translator: event.currentTarget.value })} />
        </label>
        <label>
          <span>Nguồn</span>
          <input value={activeTranslation.source} on:input={(event) => updateTranslation({ source: event.currentTarget.value })} />
        </label>
      </div>

      <div class="editor-actions">
        <button type="button" class="ghost" on:click={() => (editMode = null)}>Hủy</button>
        <button class="primary" type="submit">Lưu</button>
      </div>
    </form>
  </div>
{/if}

<style>
  .content-grid,
  .section,
  .fields {
    display: grid;
    gap: 14px;
  }

  .section-title {
    font-size: 18px;
    font-weight: 700;
  }

  .zip-row,
  .toolbar {
    display: grid;
    grid-template-columns: 1fr auto auto;
    gap: 10px;
  }

  input,
  select,
  textarea {
    width: 100%;
    box-sizing: border-box;
    border: 1px solid rgba(255, 255, 255, 0.12);
    border-radius: 9px;
    padding: 9px 10px;
    color: rgba(255, 255, 255, 0.92);
    background: rgba(255, 255, 255, 0.07);
    font: inherit;
  }

  textarea {
    resize: vertical;
  }

  button {
    border: 0;
    border-radius: 9px;
    padding: 9px 12px;
    cursor: pointer;
    font: inherit;
  }

  button:disabled {
    cursor: default;
    opacity: 0.55;
  }

  .primary {
    color: white;
    background: #596dff;
  }

  .ghost {
    color: rgba(255, 255, 255, 0.84);
    background: rgba(255, 255, 255, 0.09);
  }

  .danger {
    color: #ffb3b3;
    background: rgba(255, 80, 80, 0.13);
  }

  .success {
    margin: 0;
    color: #96f0b4;
  }

  .error {
    margin: 0;
    color: #ff9a9a;
  }

  .list {
    max-height: 300px;
    overflow: auto;
    border-radius: 12px;
    border: 1px solid rgba(255, 255, 255, 0.08);
  }

  .verse-row {
    display: grid;
    grid-template-columns: 1fr auto;
    gap: 14px;
    padding: 12px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
  }

  .verse-row:last-child {
    border-bottom: 0;
  }

  .meta,
  .current span {
    color: rgba(255, 255, 255, 0.58);
    font-size: 12px;
  }

  h3,
  p {
    margin: 0;
  }

  h3 {
    margin-top: 2px;
    font-size: 15px;
  }

  .verse-row p {
    margin-top: 5px;
    color: rgba(255, 255, 255, 0.68);
    display: -webkit-box;
    -webkit-line-clamp: 2;
    line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  .actions {
    display: flex;
    gap: 8px;
    align-items: start;
  }

  .empty {
    padding: 16px;
    color: rgba(255, 255, 255, 0.62);
  }

  .current {
    padding-top: 14px;
    border-top: 1px solid rgba(255, 255, 255, 0.08);
  }

  .modal-backdrop {
    position: fixed;
    inset: 0;
    z-index: 20;
    display: grid;
    place-items: center;
    padding: 20px;
    background: rgba(0, 0, 0, 0.54);
  }

  .editor {
    width: min(560px, 100%);
    max-height: calc(100vh - 40px);
    overflow: auto;
    padding: 18px;
    border-radius: 16px;
    color: rgba(255, 255, 255, 0.92);
    background: #191b27;
    border: 1px solid rgba(255, 255, 255, 0.12);
    box-shadow: 0 22px 70px rgba(0, 0, 0, 0.45);
  }

  .editor-head,
  .editor-actions {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
  }

  .editor h2 {
    margin: 0 0 12px;
  }

  .icon {
    width: 34px;
    height: 34px;
    padding: 0;
    color: rgba(255, 255, 255, 0.82);
    background: rgba(255, 255, 255, 0.08);
    font-size: 24px;
    line-height: 1;
  }

  .fields.two {
    grid-template-columns: 1fr 1fr;
  }

  .fields label {
    display: grid;
    gap: 6px;
  }
</style>
