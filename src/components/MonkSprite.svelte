<script lang="ts">
  import monkUrl from '../assets/monk.svg?url';

  export let size = 156;
</script>

<div class="monk" style={`width:${size}px;height:${size}px;`}>
  <img src={monkUrl} alt="Little monk" draggable="false" />
</div>

<style>
  .monk {
    display: grid;
    place-items: center;
    border-radius: 32px;
    background:
      radial-gradient(circle at 30% 20%, rgba(255, 255, 255, 0.42), transparent 45%),
      linear-gradient(180deg, rgba(255, 248, 236, 0.88), rgba(236, 222, 192, 0.8));
    box-shadow:
      0 16px 30px rgba(23, 26, 23, 0.22),
      inset 0 1px 0 rgba(255, 255, 255, 0.45);
    border: 1px solid rgba(112, 86, 42, 0.12);
  }

  img {
    width: 82%;
    height: 82%;
    object-fit: contain;
    user-select: none;
    pointer-events: none;
    filter: drop-shadow(0 10px 16px rgba(26, 24, 18, 0.18));
  }
</style>
