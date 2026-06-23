const bonkUrl = new URL('../assets/bonk_1.mp3', import.meta.url).href;
const bellUrl = new URL('../assets/bell.mp3', import.meta.url).href;

export type BellSoundName = 'bell' | 'bonk' | 'custom' | string;

function play(url: string, volume = 0.9) {
  const audio = new Audio(url);
  audio.volume = volume;
  void audio.play().catch(() => undefined);
}

function playRepeated(url: string, volume = 0.4, repeatCount = 1) {
  const repeats = Math.max(1, Math.min(10, Math.round(repeatCount)));
  for (let index = 0; index < repeats; index += 1) {
    window.setTimeout(() => play(url, volume), index * 650);
  }
}

export function playBell(volume = 0.4, repeatCount = 1) {
  playRepeated(bellUrl, volume, repeatCount);
}

export function playMo(volume = 0.85, repeatCount = 1) {
  playRepeated(bonkUrl, volume, repeatCount);
}

export function playSelectedBellSound(
  sound: BellSoundName,
  volume = 0.4,
  repeatCount = 1,
  customSoundData = ''
) {
  if (sound === 'bonk' || sound === 'mo') {
    playMo(volume, repeatCount);
    return;
  }

  if (sound === 'custom' && customSoundData) {
    playRepeated(customSoundData, volume, repeatCount);
    return;
  }

  playBell(volume, repeatCount);
}
