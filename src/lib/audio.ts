const bonkUrl = new URL('../assets/bonk_1.mp3', import.meta.url).href;
const bellUrl = new URL('../assets/bell.mp3', import.meta.url).href;

function play(url: string, volume = 0.9) {
  const audio = new Audio(url);
  audio.volume = volume;
  void audio.play().catch(() => undefined);
}

export function playBonk(volume = 0.85) {
  play(bonkUrl, volume);
}

export function playBell(volume = 0.4, repeatCount = 1) {
  const repeats = Math.max(1, Math.min(10, Math.round(repeatCount)));
  for (let index = 0; index < repeats; index += 1) {
    window.setTimeout(() => play(bellUrl, volume), index * 650);
  }
}
