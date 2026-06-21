const bonkUrl = new URL('../assets/bonk_1.mp3', import.meta.url).href;
const bellUrl = new URL('../assets/bell.mp3', import.meta.url).href;

function play(url: string, volume = 0.9) {
  const audio = new Audio(url);
  audio.volume = volume;
  void audio.play().catch(() => undefined);
}

export function playBonk() {
  play(bonkUrl, 0.85);
}

export function playBell() {
  play(bellUrl, 0.4);
}
