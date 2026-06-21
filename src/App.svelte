<script lang="ts">
  import { onMount } from 'svelte';
  import { listen } from '@tauri-apps/api/event';
  import PetWindow from './components/PetWindow.svelte';
  import { playBell } from './lib/audio';
  import type { QuotePayload } from './lib/quote';

  let quote: QuotePayload | null = null;
  let bubbleVisible = true;
  let reaction = '';
  let ready = false;

  onMount(() => {
    const unlistenTasks: Array<() => void> = [];

    void (async () => {
      const unlistenQuote = await listen<QuotePayload>('monk:quote', (event) => {
        quote = event.payload;
        bubbleVisible = true;
        ready = true;
        playBell();
      });

      const unlistenClear = await listen('monk:clear', () => {
        bubbleVisible = false;
      });

      const unlistenReaction = await listen<string>('monk:reaction', (event) => {
        reaction = event.payload;
        window.setTimeout(() => {
          reaction = '';
        }, 1800);
      });

      unlistenTasks.push(unlistenQuote, unlistenClear, unlistenReaction);
    })();

    return () => {
      for (const unlisten of unlistenTasks) {
        unlisten();
      }
    };
  });
</script>

<svelte:head>
  <title>Little Monk Window</title>
</svelte:head>

<div class="app-shell" class:ready>
  <PetWindow {quote} {bubbleVisible} bind:reaction />
</div>

<style>
  .app-shell {
    width: 100vw;
    height: 100vh;
    overflow: hidden;
    background: transparent;
  }

  .app-shell.ready {
    animation: settle 420ms ease-out both;
  }

  @keyframes settle {
    from {
      transform: translateY(8px) scale(0.98);
      opacity: 0.2;
    }
    to {
      transform: translateY(0) scale(1);
      opacity: 1;
    }
  }
</style>
