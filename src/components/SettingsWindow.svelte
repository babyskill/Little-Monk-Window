<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import MonkSprite from './MonkSprite.svelte';
  import QuoteBubble from './QuoteBubble.svelte';
  import BellSettings from './settings/BellSettings.svelte';
  import ContentManager from './settings/ContentManager.svelte';
  import type { QuotePayload } from '../lib/quote';
  import type { AppSettings } from '../lib/settings';
  import { defaultSettings } from '../lib/settings';

  export let settings: AppSettings = defaultSettings;

  let tab: 'character' | 'bell' | 'content' = 'character';
  let draft: AppSettings = { ...defaultSettings };
  let saveTimer: number | null = null;
  let ready = false;

  const previewQuote: QuotePayload = {
    id: 'preview',
    chapterNumber: 1,
    verseNumber: 1,
    chapterTitle: 'Phẩm Song Yếu',
    text: 'Chiến thắng vạn quân\nkhông bằng chiến thắng chính mình.',
    translator: 'Kinh Pháp Cú',
    source: 'preview',
  };

  onMount(() => {
    draft = { ...settings };
    ready = true;
  });

  $: if (ready && JSON.stringify(settings) !== JSON.stringify(draft)) {
    draft = { ...settings };
  }

  function queueSave() {
    if (saveTimer) {
      window.clearTimeout(saveTimer);
    }
    saveTimer = window.setTimeout(() => {
      void invoke('update_settings', { next: draft });
    }, 180);
  }

  function applyLanguage(language: 'vi' | 'en') {
    draft.quote_language = language;
    queueSave();
  }

  function resetDefaults() {
    draft = { ...defaultSettings };
    queueSave();
  }
</script>

<main class="settings">
  <section class="hero">
    <div class="hero-copy">
      <div class="eyebrow">Little Monk</div>
      <h1>Cài đặt</h1>
      <p>Điều chỉnh chú tiểu, nhịp hiển thị và kiểu trình bày.</p>
    </div>
    <div class="hero-preview">
      <QuoteBubble quote={previewQuote} visible={true} />
      <div class="hero-monk">
        <MonkSprite size={draft.pet_size} />
      </div>
    </div>
  </section>

  <nav class="tabs" aria-label="Các tab cài đặt">
    <button class:active={tab === 'character'} on:click={() => (tab = 'character')}>Nhân vật</button>
    <button class:active={tab === 'bell'} on:click={() => (tab = 'bell')}>Chuông</button>
    <button class:active={tab === 'content'} on:click={() => (tab = 'content')}>Nội dung</button>
  </nav>

  <section class="panel">
    {#if tab === 'character'}
      <div class="section">
        <div class="section-title">Nhân vật</div>
        <label>
          <span>Kích thước chú tiểu</span>
          <input type="range" min="110" max="168" step="1" bind:value={draft.pet_size} on:input={queueSave} />
          <strong>{Math.round(draft.pet_size)} px</strong>
        </label>
        <label>
          <span>Cỡ chữ</span>
          <input type="range" min="14" max="20" step="1" bind:value={draft.font_size} on:input={queueSave} />
          <strong>{Math.round(draft.font_size)} px</strong>
        </label>
        <label class="toggle">
          <span>Luôn ở trên cùng</span>
          <input type="checkbox" bind:checked={draft.always_on_top} on:change={queueSave} />
        </label>
        <label class="toggle">
          <span>Hiện câu trích trên chú tiểu</span>
          <input type="checkbox" bind:checked={draft.show_quote} on:change={queueSave} />
        </label>
        <label class="toggle">
          <span>Hiện câu trích khi nhàn rỗi</span>
          <input
            type="checkbox"
            bind:checked={draft.show_idle_message}
            disabled={!draft.show_quote}
            on:change={queueSave}
          />
        </label>
      </div>
    {:else if tab === 'bell'}
      <BellSettings bind:draft {queueSave} />
    {:else}
      <div class="section">
        <div class="section-title">Nội dung</div>
        <div class="segmented">
          <button class:active={draft.quote_language === 'vi'} on:click={() => applyLanguage('vi')}>Tiếng Việt</button>
          <button class:active={draft.quote_language === 'en'} on:click={() => applyLanguage('en')}>Tiếng Anh</button>
        </div>
        <ContentManager />
      </div>
    {/if}
  </section>

  <footer class="footer">
    <button class="ghost" on:click={resetDefaults}>Khôi phục mặc định</button>
    <button class="primary" on:click={() => void invoke('update_settings', { next: draft })}>Lưu</button>
  </footer>
</main>

<style>
  .settings {
    height: 100vh;
    box-sizing: border-box;
    display: grid;
    grid-template-rows: auto auto minmax(0, 1fr) auto;
    gap: 12px;
    overflow: hidden;
    padding: 16px;
    font-family: -apple-system, BlinkMacSystemFont, "SF Pro Text", "Helvetica Neue", Arial, sans-serif;
    color: rgba(243, 242, 247, 0.95);
    background:
      radial-gradient(circle at top right, rgba(89, 109, 255, 0.28), transparent 30%),
      linear-gradient(145deg, #121420 0%, #1a1830 56%, #11131b 100%);
  }

  .hero {
    display: grid;
    grid-template-columns: 1fr 0.9fr;
    gap: 18px;
    align-items: center;
    margin-bottom: 0;
  }

  .hero-copy h1 {
    margin: 2px 0 8px;
    font-size: 28px;
    line-height: 1;
  }

  .hero-copy p {
    margin: 0;
    color: rgba(235, 235, 242, 0.72);
    font-size: 14px;
  }

  .eyebrow {
    color: rgba(152, 166, 255, 0.9);
    font-size: 12px;
    font-weight: 700;
    letter-spacing: 0.12em;
    text-transform: uppercase;
  }

  .hero-preview {
    position: relative;
    min-height: 190px;
    display: grid;
    justify-items: center;
    align-items: end;
  }

  .hero-preview :global(.bubble) {
    transform: scale(0.72);
    transform-origin: center bottom;
  }

  .hero-monk {
    position: absolute;
    bottom: 0;
    transform: translateY(6px) scale(0.9);
    pointer-events: none;
  }

  .tabs {
    display: inline-flex;
    gap: 6px;
    padding: 5px;
    border-radius: 12px;
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.06);
  }

  .tabs button,
  .segmented button,
  .footer button {
    border: 0;
    cursor: pointer;
    font: inherit;
  }

  .tabs button {
    padding: 8px 14px;
    border-radius: 9px;
    color: rgba(235, 235, 242, 0.78);
    background: transparent;
  }

  .tabs button.active {
    color: #ffffff;
    background: rgba(86, 113, 255, 0.26);
  }

  .panel {
    min-height: 0;
    margin-top: 0;
    border-radius: 18px;
    padding: 18px;
    overflow: auto;
    background: rgba(255, 255, 255, 0.045);
    border: 1px solid rgba(255, 255, 255, 0.08);
  }

  .section {
    display: grid;
    gap: 16px;
  }

  .section-title {
    font-size: 18px;
    font-weight: 700;
  }

  label {
    display: grid;
    grid-template-columns: 130px 1fr auto;
    gap: 12px;
    align-items: center;
    color: rgba(236, 236, 245, 0.84);
  }

  label span {
    font-size: 14px;
  }

  input[type='range'] {
    width: 100%;
    accent-color: #7f90ff;
  }

  strong {
    font-size: 13px;
    color: rgba(255, 255, 255, 0.68);
    min-width: 62px;
    text-align: right;
  }

  .toggle {
    grid-template-columns: 130px auto 1fr;
  }

  .toggle input {
    justify-self: start;
    width: 18px;
    height: 18px;
  }

  .segmented {
    display: inline-flex;
    gap: 6px;
    width: fit-content;
    padding: 4px;
    border-radius: 12px;
    background: rgba(255, 255, 255, 0.04);
  }

  .segmented button {
    padding: 8px 14px;
    border-radius: 9px;
    color: rgba(235, 235, 242, 0.78);
    background: transparent;
  }

  .segmented button.active {
    color: #ffffff;
    background: rgba(86, 113, 255, 0.26);
  }

  .footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-top: 0;
  }

  .footer .ghost,
  .footer .primary {
    padding: 10px 14px;
    border-radius: 10px;
  }

  .ghost {
    color: rgba(238, 238, 244, 0.82);
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.08);
  }

  .primary {
    color: white;
    background: linear-gradient(180deg, #6f83ff 0%, #5366eb 100%);
  }
</style>
