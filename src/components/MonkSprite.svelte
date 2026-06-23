<script lang="ts">
  import { onDestroy, onMount } from 'svelte';
  import spriteUrl from '../assets/pets/an-mo/spritesheet.png?url';
  import { sliceAlphaGrid, type SliceRect } from '../lib/spriteSlicer';

  export let size = 156;

  let canvas: HTMLCanvasElement;
  let frameRects: SliceRect[] = [];
  let image: HTMLImageElement | null = null;
  let raf = 0;
  let startedAt = 0;

  onMount(() => {
    let disposed = false;
    const img = new Image();
    img.decoding = 'async';
    img.src = spriteUrl;
    img.onload = () => {
      if (disposed) return;
      image = img;
      frameRects = detectFrames(img);
      startedAt = performance.now();
      raf = requestAnimationFrame(draw);
    };

    return () => {
      disposed = true;
      cancelAnimationFrame(raf);
    };
  });

  onDestroy(() => {
    cancelAnimationFrame(raf);
  });

  function detectFrames(img: HTMLImageElement) {
    const probe = document.createElement('canvas');
    probe.width = img.naturalWidth;
    probe.height = img.naturalHeight;
    const context = probe.getContext('2d', { willReadFrequently: true });
    if (!context) return uniformFirstRow(img);

    context.clearRect(0, 0, probe.width, probe.height);
    context.drawImage(img, 0, 0);
    const rects = sliceAlphaGrid(context.getImageData(0, 0, probe.width, probe.height));

    if (rects.length > 0) {
      const firstRowTop = rects[0].y;
      return rects.filter((rect) => Math.abs(rect.y - firstRowTop) < 4);
    }

    return uniformFirstRow(img);
  }

  function uniformFirstRow(img: HTMLImageElement): SliceRect[] {
    const columns = 8;
    const rows = 9;
    const width = img.naturalWidth / columns;
    const height = img.naturalHeight / rows;

    return Array.from({ length: columns }, (_, index) => ({
      x: index * width,
      y: 0,
      width,
      height,
    }));
  }

  function draw(now: number) {
    const ctx = canvas.getContext('2d');
    if (!ctx || !image || frameRects.length === 0) {
      raf = requestAnimationFrame(draw);
      return;
    }

    const index = Math.floor(((now - startedAt) / 1000) * 6) % frameRects.length;
    const frame = frameRects[index];
    const dpr = window.devicePixelRatio || 1;
    const targetWidth = size * dpr;
    const targetHeight = size * dpr;

    if (canvas.width !== targetWidth || canvas.height !== targetHeight) {
      canvas.width = targetWidth;
      canvas.height = targetHeight;
    }

    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.imageSmoothingEnabled = false;

    const scale = Math.min(canvas.width / frame.width, canvas.height / frame.height);
    const width = frame.width * scale;
    const height = frame.height * scale;
    const x = (canvas.width - width) / 2;
    const y = canvas.height - height;

    ctx.drawImage(image, frame.x, frame.y, frame.width, frame.height, x, y, width, height);
    raf = requestAnimationFrame(draw);
  }
</script>

<div class="monk" style={`width:${size}px;height:${size}px;`}>
  <canvas bind:this={canvas} aria-label="Little monk companion"></canvas>
</div>

<style>
  .monk {
    display: grid;
    place-items: center;
    background: transparent;
  }

  canvas {
    width: 100%;
    height: 100%;
    user-select: none;
    pointer-events: none;
    filter: drop-shadow(0 10px 12px rgba(0, 0, 0, 0.28));
  }
</style>
