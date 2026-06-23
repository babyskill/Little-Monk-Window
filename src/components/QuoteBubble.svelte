<script lang="ts">
  import type { QuotePayload } from '../lib/quote';

  export let quote: QuotePayload | null = null;
  export let visible = true;
  export let fontSize = 17;

  const fallbackText = 'Chiến thắng vạn quân\nkhông bằng chiến thắng chính mình.';
</script>

{#if visible}
  <section class="bubble" aria-live="polite" style={`--quote-font-size:${fontSize}px;`}>
    <span class="quote-mark quote-mark-open" aria-hidden="true">“</span>
    <p class="text">{quote?.text ?? fallbackText}</p>
    <p class="source">- Kinh Pháp Cú</p>
    <span class="quote-mark quote-mark-close" aria-hidden="true">”</span>
  </section>
{/if}

<style>
  .bubble {
    position: relative;
    max-width: 380px;
    width: fit-content;
    min-width: 260px;
    min-height: 90px;
    padding: 22px 36px 18px;
    border-radius: 22px;
    box-sizing: border-box;
    background: rgba(255, 250, 242, 0.82);
    border: 1px solid rgba(255, 255, 255, 0.72);
    box-shadow:
      0 18px 38px rgba(58, 39, 24, 0.2),
      inset 0 1px 0 rgba(255, 255, 255, 0.62);
    color: rgba(28, 23, 20, 0.9);
    text-align: center;
    backdrop-filter: blur(18px) saturate(1.12);
    -webkit-backdrop-filter: blur(18px) saturate(1.12);
    margin-bottom: 8px;
  }

  .bubble::before,
  .bubble::after {
    content: "";
    position: absolute;
    left: 50%;
    width: 0;
    height: 0;
    pointer-events: none;
    transform: translateX(-50%);
  }

  .bubble::before {
    bottom: -11px;
    border-left: 10px solid transparent;
    border-right: 10px solid transparent;
    border-top: 11px solid rgba(255, 255, 255, 0.72);
  }

  .bubble::after {
    bottom: -9px;
    border-left: 8px solid transparent;
    border-right: 8px solid transparent;
    border-top: 9px solid rgba(255, 250, 242, 0.82);
  }

  .text {
    margin: 0;
    white-space: pre-wrap;
    line-height: 1.36;
    font-family: -apple-system, BlinkMacSystemFont, "SF Pro Text", Arial, sans-serif;
    font-size: var(--quote-font-size);
    font-weight: 500;
    letter-spacing: 0;
  }

  .source {
    margin: 13px 0 0;
    color: rgba(71, 58, 50, 0.64);
    font-family: -apple-system, BlinkMacSystemFont, "SF Pro Text", Arial, sans-serif;
    font-size: calc(var(--quote-font-size) * 0.88);
    font-weight: 450;
    line-height: 1.2;
  }

  .quote-mark {
    position: absolute;
    color: rgba(127, 107, 93, 0.14);
    font-family: Georgia, "Times New Roman", serif;
    font-size: 58px;
    font-weight: 700;
    line-height: 1;
    pointer-events: none;
  }

  .quote-mark-open {
    top: 8px;
    left: 14px;
  }

  .quote-mark-close {
    right: 14px;
    bottom: 2px;
  }
</style>
