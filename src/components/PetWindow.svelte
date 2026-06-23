<script lang="ts">
  import { getCurrentWindow, PhysicalPosition } from '@tauri-apps/api/window';
  import { invoke } from '@tauri-apps/api/core';
  import MonkSprite from './MonkSprite.svelte';
  import QuoteBubble from './QuoteBubble.svelte';
  import ReactionBubble from './ReactionBubble.svelte';
  import { playBonk } from '../lib/audio';
  import type { QuotePayload } from '../lib/quote';

  export let quote: QuotePayload | null = null;
  export let reaction = '';
  export let petSize = 145;
  export let quoteFontSize = 17;
  export let showQuote = true;
  export let showTapMessage = true;

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

  async function bonk() {
    if (showTapMessage) {
      reaction = 'A-di-đà Phật.';
      window.setTimeout(() => {
        reaction = '';
      }, 1800);
    }
    playBonk();

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

<main class="frame">
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
</main>

<style>
  .frame {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: flex-start;
    gap: 8px;
    padding: 18px 18px 12px;
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
</style>
