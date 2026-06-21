<script lang="ts">
  import MonkSprite from './MonkSprite.svelte';
  import QuoteBubble from './QuoteBubble.svelte';
  import ReactionBubble from './ReactionBubble.svelte';
  import { playBonk } from '../lib/audio';
  import type { QuotePayload } from '../lib/quote';
  import lotusUrl from '../assets/Lotus.svg?url';

  export let quote: QuotePayload | null = null;
  export let bubbleVisible = true;
  export let reaction = '';

  function bonk() {
    reaction = 'A-di-đà Phật.';
    playBonk();
    window.setTimeout(() => {
      reaction = '';
    }, 1800);
  }
</script>

<main class="frame">
  <img class="lotus" src={lotusUrl} alt="" aria-hidden="true" />
  <div class="halo"></div>
  <QuoteBubble {quote} visible={bubbleVisible} />

  <button class="monk-button" aria-label="Tap the monk" on:click={bonk}>
    <MonkSprite size={168} />
  </button>

  <ReactionBubble message={reaction} />
</main>

<style>
  .frame {
    position: relative;
    width: 100%;
    height: 100%;
    display: grid;
    place-items: center;
    grid-template-rows: auto auto auto;
    gap: 12px;
    padding: 18px 14px 20px;
    box-sizing: border-box;
    background:
      radial-gradient(circle at 50% 15%, rgba(255, 244, 214, 0.25), transparent 36%),
      radial-gradient(circle at 40% 85%, rgba(104, 138, 92, 0.15), transparent 38%),
      linear-gradient(180deg, rgba(255, 255, 255, 0.02), rgba(255, 255, 255, 0));
  }

  .halo {
    position: absolute;
    inset: 10px 24px auto;
    height: 120px;
    border-radius: 999px;
    background: radial-gradient(circle, var(--glow), transparent 70%);
    filter: blur(14px);
    pointer-events: none;
  }

  .lotus {
    position: absolute;
    bottom: 10px;
    right: 12px;
    width: 64px;
    opacity: 0.22;
    pointer-events: none;
    filter: drop-shadow(0 8px 14px rgba(39, 31, 14, 0.12));
  }

  .monk-button {
    border: 0;
    background: transparent;
    padding: 0;
    cursor: pointer;
    transition: transform 140ms cubic-bezier(.2,.75,.2,1);
  }

  .monk-button:hover {
    transform: translateY(-2px) scale(1.02);
  }

  .monk-button:active {
    transform: translateY(2px) scale(0.97);
  }
</style>
