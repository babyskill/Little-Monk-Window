<script lang="ts">
  import type { QuotePayload } from '../lib/quote';

  export let quote: QuotePayload | null = null;
  export let visible = true;
</script>

{#if visible && quote}
  <section class="bubble" aria-live="polite">
    <p class="text">{quote.text}</p>
    <div class="meta">
      <span>{quote.chapterTitle} · {quote.chapterNumber}:{quote.verseNumber}</span>
      {#if quote.translator}
        <span>{quote.translator}</span>
      {/if}
    </div>
  </section>
{/if}

<style>
  .bubble {
    max-width: 360px;
    padding: 14px 16px 12px;
    border-radius: 22px;
    background: linear-gradient(180deg, var(--paper-strong), var(--paper));
    box-shadow:
      0 18px 34px var(--shadow),
      inset 0 1px 0 rgba(255, 255, 255, 0.6);
    border: 1px solid rgba(127, 104, 54, 0.18);
    color: var(--ink);
    backdrop-filter: blur(18px);
  }

  .text {
    margin: 0;
    white-space: pre-wrap;
    line-height: 1.35;
    font-size: 1rem;
    letter-spacing: 0;
  }

  .meta {
    margin-top: 10px;
    display: flex;
    flex-direction: column;
    gap: 2px;
    font-size: 0.72rem;
    color: var(--ink-soft);
  }
</style>
