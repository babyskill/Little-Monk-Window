<script lang="ts">
  import { onMount } from 'svelte';
  import { listen } from '@tauri-apps/api/event';
  import { invoke } from '@tauri-apps/api/core';
  import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
  import PetWindow from './components/PetWindow.svelte';
  import SettingsWindow from './components/SettingsWindow.svelte';
  import { playSelectedBellSound } from './lib/audio';
  import type { QuotePayload } from './lib/quote';
  import { defaultSettings, type AppSettings } from './lib/settings';

  let quote: QuotePayload | null = {
    id: 'startup',
    chapterNumber: 1,
    verseNumber: 1,
    chapterTitle: 'Phẩm Song Yếu',
    text: 'Ý dẫn đầu các pháp,\nÝ làm chủ, ý tạo.',
    translator: 'HT. Thích Minh Châu',
    source: 'startup',
  };
  let settings: AppSettings = { ...defaultSettings };
  let reaction = '';
  let ready = false;
  const isSettingsWindow = getCurrentWebviewWindow().label === 'settings';

  async function loadSettings() {
    try {
      settings = await invoke<AppSettings>('get_settings');
    } catch {
      settings = { ...defaultSettings };
    }
  }

  onMount(() => {
    const unlistenTasks: Array<() => void> = [];

    void (async () => {
      await loadSettings();

      let hasInitialized = false;
      const unlistenQuote = await listen<QuotePayload>('monk:quote', (event) => {
        quote = event.payload;
        ready = true;

        if (hasInitialized) {
          if (settings.bell_sound_enabled) {
            playSelectedBellSound(settings.bell_sound, settings.bell_volume, settings.bell_repeat_count);
          }
        } else {
          hasInitialized = true;
        }
      });

      const unlistenBell = await listen('monk:bell', () => {
        if (settings.bell_enabled && settings.bell_sound_enabled) {
          playSelectedBellSound(settings.bell_sound, settings.bell_volume, settings.bell_repeat_count);
        }
      });

      const unlistenSettings = await listen<AppSettings>('settings:changed', (event) => {
        settings = event.payload;
      });

      const unlistenReaction = await listen<string>('monk:reaction', (event) => {
        reaction = event.payload;
        window.setTimeout(() => {
          reaction = '';
        }, 1800);
      });

      unlistenTasks.push(unlistenQuote, unlistenBell, unlistenSettings, unlistenReaction);
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
  {#if isSettingsWindow}
    <SettingsWindow {settings} />
  {:else}
    <PetWindow
      {quote}
      bind:reaction
      bind:settings
      showQuote={settings.show_quote}
    />
  {/if}
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
