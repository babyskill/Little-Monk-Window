<script lang="ts">
  import { onMount } from 'svelte';
  import { getCurrentWindow, PhysicalPosition } from '@tauri-apps/api/window';
  import { invoke } from '@tauri-apps/api/core';
  import MonkSprite from './MonkSprite.svelte';
  import QuoteBubble from './QuoteBubble.svelte';
  import ReactionBubble from './ReactionBubble.svelte';
  import { playSelectedBellSound } from '../lib/audio';
  import type { QuotePayload } from '../lib/quote';
  import type { AppSettings } from '../lib/settings';

  export let quote: QuotePayload | null = null;
  export let reaction = '';
  export let settings: AppSettings;
  export let showQuote = true;

  $: petSize = settings?.pet_size ?? 145;
  $: quoteFontSize = settings?.font_size ?? 17;

  const currentWindow = getCurrentWindow();
  const dragThreshold = 4;
  let dragState:
    | {
        pointerId: number;
        startX: number;
        startY: number;
        originX: number;
        originY: number;
        scaleFactor: number;
        moved: boolean;
        ready: boolean;
      }
    | null = null;

  let showMenu = false;
  let menuX = 0;
  let menuY = 0;

  async function saveQuickSettings() {
    try {
      await invoke('update_settings', { next: settings });
    } catch (err) {
      console.error('Failed to save quick settings:', err);
    }
  }

  function toggleAlwaysOnTop() {
    settings.always_on_top = !settings.always_on_top;
    void saveQuickSettings();
  }

  function toggleSound() {
    settings.bell_sound_enabled = !settings.bell_sound_enabled;
    void saveQuickSettings();
  }

  function handleContextMenu(event: MouseEvent) {
    event.preventDefault();
    const menuWidth = 220;
    const menuHeight = 270;
    menuX = Math.min(event.clientX, 430 - menuWidth - 10);
    menuY = Math.min(event.clientY, 380 - menuHeight - 10);
    showMenu = true;
  }

  function closeMenu() {
    showMenu = false;
  }

  onMount(() => {
    const handleGlobalClick = (event: MouseEvent) => {
      if (showMenu) {
        const menuEl = document.querySelector('.context-menu');
        if (menuEl && !menuEl.contains(event.target as Node)) {
          closeMenu();
        }
      }
    };
    window.addEventListener('click', handleGlobalClick);
    return () => {
      window.removeEventListener('click', handleGlobalClick);
    };
  });

  async function bonk() {
    reaction = '';
    playSelectedBellSound(
      settings.bell_sound,
      settings.bell_volume,
      settings.bell_repeat_count,
      settings.bell_custom_sound_data
    );

    try {
      const next = await invoke<QuotePayload>('get_next_quote', { currentId: quote?.id || '' });
      quote = next;
    } catch (err) {
      console.error('Failed to rotate quote on click:', err);
    }
  }

  async function beginDrag(event: PointerEvent) {
    if (event.button !== 0) {
      return;
    }

    const target = event.currentTarget as HTMLElement;
    target.setPointerCapture(event.pointerId);
    dragState = {
      pointerId: event.pointerId,
      startX: event.screenX,
      startY: event.screenY,
      originX: 0,
      originY: 0,
      scaleFactor: 1,
      moved: false,
      ready: false,
    };

    const [position, scaleFactor] = await Promise.all([
      currentWindow.outerPosition(),
      currentWindow.scaleFactor().catch(() => 1),
    ]);

    if (!dragState || dragState.pointerId !== event.pointerId) {
      return;
    }

    dragState = {
      pointerId: event.pointerId,
      startX: event.screenX,
      startY: event.screenY,
      originX: position.x,
      originY: position.y,
      scaleFactor,
      moved: false,
      ready: true,
    };
  }

  function drag(event: PointerEvent) {
    if (!dragState || dragState.pointerId !== event.pointerId) {
      return;
    }
    if (!dragState.ready) {
      return;
    }

    const dx = event.screenX - dragState.startX;
    const dy = event.screenY - dragState.startY;
    if (!dragState.moved && Math.hypot(dx, dy) < dragThreshold) {
      return;
    }

    dragState.moved = true;
    const x = Math.round(dragState.originX + dx * dragState.scaleFactor);
    const y = Math.round(dragState.originY + dy * dragState.scaleFactor);
    void currentWindow.setPosition(new PhysicalPosition(x, y));
  }

  function endDrag(event: PointerEvent) {
    if (!dragState || dragState.pointerId !== event.pointerId) {
      return;
    }

    const wasDrag = dragState.moved;
    dragState = null;
    const target = event.currentTarget as HTMLElement;
    if (target.hasPointerCapture(event.pointerId)) {
      target.releasePointerCapture(event.pointerId);
    }

    if (!wasDrag) {
      bonk();
    }
  }
</script>

<main class="frame" on:contextmenu|preventDefault={handleContextMenu} role="presentation">
  <QuoteBubble {quote} visible={showQuote} fontSize={quoteFontSize} />

  <button
    class="monk-button"
    class:dragging={dragState?.moved}
    aria-label="Tap or drag the monk"
    on:pointerdown={beginDrag}
    on:pointermove={drag}
    on:pointerup={endDrag}
    on:pointercancel={endDrag}
  >
    <MonkSprite size={petSize} />
  </button>

  <ReactionBubble message={reaction} />

  {#if showMenu}
    <div
      class="context-menu"
      style={`left:${menuX}px;top:${menuY}px;`}
      on:contextmenu|preventDefault
      role="menu"
      tabindex="-1"
    >
      <div class="menu-section">
        <span class="menu-label">⚙️ Quick Settings</span>
        <div class="menu-slider">
          <label>
            <span>Size:</span>
            <input
              type="range"
              min="110"
              max="168"
              step="1"
              bind:value={settings.pet_size}
              on:input={saveQuickSettings}
            />
          </label>
        </div>
        <div class="menu-slider">
          <label>
            <span>Text:</span>
            <input
              type="range"
              min="14"
              max="20"
              step="1"
              bind:value={settings.font_size}
              on:input={saveQuickSettings}
            />
          </label>
        </div>
        <div class="menu-slider">
          <label>
            <span>Vol:</span>
            <input
              type="range"
              min="0"
              max="1"
              step="0.05"
              bind:value={settings.bell_volume}
              on:input={saveQuickSettings}
            />
          </label>
        </div>
      </div>

      <div class="menu-divider"></div>

      <button class="menu-item" on:click={toggleAlwaysOnTop}>
        <span>📌 Always on top</span>
        <input type="checkbox" checked={settings.always_on_top} readonly />
      </button>
      <button class="menu-item" on:click={toggleSound}>
        <span>🔊 Sound enabled</span>
        <input type="checkbox" checked={settings.bell_sound_enabled} readonly />
      </button>
      <button class="menu-item" on:click={() => { invoke('show_settings_window'); closeMenu(); }}>
        <span>🛠️ Settings...</span>
      </button>

      <div class="menu-divider"></div>

      <button class="menu-item quit" on:click={() => invoke('quit_app')}>
        <span>❌ Quit</span>
      </button>
    </div>
  {/if}
</main>

<style>
  .frame {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: flex-end;
    gap: 8px;
    padding: 12px 18px 16px;
    box-sizing: border-box;
    background: transparent;
  }

  .monk-button {
    position: relative;
    z-index: 2;
    border: 0;
    background: transparent;
    padding: 0;
    margin: -2px 0 0;
    cursor: grab;
    touch-action: none;
    user-select: none;
    transition: transform 140ms cubic-bezier(.2,.75,.2,1);
  }

  .monk-button:hover {
    transform: translateY(-2px) scale(1.02);
  }

  .monk-button:active {
    cursor: grabbing;
    transform: translateY(2px) scale(0.97);
  }

  .monk-button.dragging {
    cursor: grabbing;
    transform: scale(1.01);
  }

  .context-menu {
    position: fixed;
    z-index: 100;
    width: 220px;
    background: rgba(25, 27, 39, 0.94);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 12px;
    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.4);
    padding: 8px;
    display: flex;
    flex-direction: column;
    gap: 4px;
    backdrop-filter: blur(12px);
    -webkit-backdrop-filter: blur(12px);
    color: #f3f3f7;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
  }

  .menu-section {
    padding: 4px 8px;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .menu-label {
    font-size: 11px;
    font-weight: 700;
    text-transform: uppercase;
    color: rgba(255, 255, 255, 0.4);
    letter-spacing: 0.05em;
  }

  .menu-slider {
    display: flex;
    flex-direction: column;
  }

  .menu-slider label {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 12px;
    color: rgba(255, 255, 255, 0.8);
  }

  .menu-slider label span {
    min-width: 32px;
  }

  .menu-slider input[type="range"] {
    flex: 1;
    height: 4px;
    accent-color: #7f90ff;
    background: rgba(255, 255, 255, 0.1);
    border-radius: 2px;
    cursor: pointer;
  }

  .menu-divider {
    height: 1px;
    background: rgba(255, 255, 255, 0.08);
    margin: 4px 0;
  }

  .menu-item {
    background: transparent;
    border: 0;
    padding: 8px;
    border-radius: 6px;
    color: rgba(255, 255, 255, 0.85);
    font-size: 12px;
    text-align: left;
    display: flex;
    align-items: center;
    justify-content: space-between;
    cursor: pointer;
    transition: background 120ms;
  }

  .menu-item:hover {
    background: rgba(255, 255, 255, 0.08);
    color: #ffffff;
  }

  .menu-item input[type="checkbox"] {
    accent-color: #7f90ff;
    width: 14px;
    height: 14px;
    pointer-events: none;
  }

  .menu-item.quit {
    color: #ff9a9a;
  }

  .menu-item.quit:hover {
    background: rgba(255, 80, 80, 0.15);
  }
</style>
