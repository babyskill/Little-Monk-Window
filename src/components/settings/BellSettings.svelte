<script lang="ts">
  import type { AppSettings } from '../../lib/settings';
  import { playSelectedBellSound } from '../../lib/audio';

  export let draft: AppSettings;
  export let queueSave: () => void;

  const sounds = [
    { value: 'bell', label: 'Bell' },
    { value: 'bonk', label: 'Mõ' },
    { value: 'custom', label: 'Custom file' },
  ];

  let customSoundInput: HTMLInputElement | null = null;
  let hasCustomSound = false;
  let isCustomSelected = false;

  function minutesToTime(minutes: number) {
    const hour = Math.floor(minutes / 60).toString().padStart(2, '0');
    const minute = Math.floor(minutes % 60).toString().padStart(2, '0');
    return `${hour}:${minute}`;
  }

  function timeToMinutes(value: string) {
    const [hour, minute] = value.split(':').map((part) => Number(part));
    return Math.max(0, Math.min(1439, hour * 60 + minute));
  }

  function previewSound() {
    playSelectedBellSound(
      draft.bell_sound,
      draft.bell_volume,
      draft.bell_repeat_count,
      draft.bell_custom_sound_data
    );
  }

  async function handleCustomSoundChange(event: Event) {
    const input = event.currentTarget as HTMLInputElement;
    const file = input.files?.[0];

    if (!file) {
      return;
    }

    const dataUrl = await new Promise<string>((resolve, reject) => {
      const reader = new FileReader();
      reader.onload = () => resolve(String(reader.result ?? ''));
      reader.onerror = () => reject(reader.error ?? new Error('Failed to read custom audio file'));
      reader.readAsDataURL(file);
    });

    draft.bell_sound = 'custom';
    draft.bell_custom_sound_data = dataUrl;
    draft.bell_custom_sound_name = file.name;
    queueSave();
  }

  function chooseCustomSound() {
    customSoundInput?.click();
  }

  function handleSoundChange() {
    queueSave();
    if (draft.bell_sound === 'custom' && !draft.bell_custom_sound_data) {
      chooseCustomSound();
    }
  }

  function removeCustomSound() {
    draft.bell_custom_sound_data = '';
    draft.bell_custom_sound_name = '';
    if (draft.bell_sound === 'custom') {
      draft.bell_sound = 'bell';
    }
    queueSave();
  }

  $: hasCustomSound = Boolean(draft.bell_custom_sound_data);
  $: isCustomSelected = draft.bell_sound === 'custom';

</script>

<div class="section">
  <div class="section-title">Quote flow</div>
  <label class="toggle">
    <span>Show quote on pet</span>
    <input type="checkbox" bind:checked={draft.show_quote} on:change={queueSave} />
  </label>
  <label class="toggle">
    <span>Show quote while idle</span>
    <input
      type="checkbox"
      bind:checked={draft.show_idle_message}
      disabled={!draft.show_quote}
      on:change={queueSave}
    />
  </label>
  <label>
    <span>Quote interval</span>
    <input type="range" min="60" max="10800" step="60" bind:value={draft.quote_interval_secs} on:input={queueSave} />
    <strong>{Math.round(draft.quote_interval_secs / 60)} min</strong>
  </label>
</div>

<div class="section">
  <div class="section-title">Chuông chánh niệm</div>
  <label class="toggle">
    <span>Mindfulness bell</span>
    <input type="checkbox" bind:checked={draft.bell_enabled} on:change={queueSave} />
  </label>
  <label>
    <span>Bell interval</span>
    <input
      type="range"
      min="1"
      max="180"
      step="1"
      bind:value={draft.bell_interval_minutes}
      disabled={!draft.bell_enabled}
      on:input={queueSave}
    />
    <strong>{draft.bell_interval_minutes} min</strong>
  </label>
  <label class="toggle">
    <span>Sync message with bell</span>
    <input type="checkbox" bind:checked={draft.bell_sync_message} disabled={!draft.bell_enabled} on:change={queueSave} />
  </label>
  <label class="toggle">
    <span>Bell sound</span>
    <input type="checkbox" bind:checked={draft.bell_sound_enabled} disabled={!draft.bell_enabled} on:change={queueSave} />
  </label>
  <label>
    <span>Sound</span>
    <select bind:value={draft.bell_sound} disabled={!draft.bell_enabled || !draft.bell_sound_enabled} on:change={handleSoundChange}>
      {#each sounds as sound}
        <option value={sound.value}>{sound.label}</option>
      {/each}
    </select>
    <button type="button" class="small" disabled={!draft.bell_sound_enabled} on:click={previewSound}>Preview</button>
  </label>
  {#if isCustomSelected}
    <div class="custom-sound-row">
      <input
        bind:this={customSoundInput}
        type="file"
        accept="audio/*"
        hidden
        on:change={handleCustomSoundChange}
      />
      <div class="custom-sound-meta">
        <span>{hasCustomSound ? draft.bell_custom_sound_name : 'No custom audio selected yet'}</span>
        <small>Choose an external audio file to use for this sound.</small>
      </div>
      <div class="custom-sound-actions">
        <button type="button" class="small" disabled={!draft.bell_sound_enabled} on:click={chooseCustomSound}>Choose file</button>
        <button type="button" class="small secondary" disabled={!hasCustomSound} on:click={removeCustomSound}>Remove</button>
      </div>
    </div>
  {/if}
  <label>
    <span>Volume</span>
    <input
      type="range"
      min="0"
      max="1"
      step="0.01"
      bind:value={draft.bell_volume}
      disabled={!draft.bell_enabled || !draft.bell_sound_enabled}
      on:input={queueSave}
    />
    <strong>{Math.round(draft.bell_volume * 100)}%</strong>
  </label>
  <label>
    <span>Repeat</span>
    <input
      type="number"
      min="1"
      max="10"
      bind:value={draft.bell_repeat_count}
      disabled={!draft.bell_enabled || !draft.bell_sound_enabled}
      on:change={queueSave}
    />
    <strong>{draft.bell_repeat_count}x</strong>
  </label>
  <label class="toggle">
    <span>Quiet hours</span>
    <input type="checkbox" bind:checked={draft.quiet_hours_enabled} disabled={!draft.bell_enabled} on:change={queueSave} />
  </label>
  {#if draft.quiet_hours_enabled}
    <div class="time-row">
      <label>
        <span>Start</span>
        <input
          type="time"
          value={minutesToTime(draft.quiet_hours_start_minutes)}
          on:change={(event) => {
            draft.quiet_hours_start_minutes = timeToMinutes(event.currentTarget.value);
            queueSave();
          }}
        />
      </label>
      <label>
        <span>End</span>
        <input
          type="time"
          value={minutesToTime(draft.quiet_hours_end_minutes)}
          on:change={(event) => {
            draft.quiet_hours_end_minutes = timeToMinutes(event.currentTarget.value);
            queueSave();
          }}
        />
      </label>
    </div>
  {/if}
</div>

<style>
  .section {
    display: grid;
    gap: 16px;
  }

  .section + .section {
    margin-top: 22px;
    padding-top: 18px;
    border-top: 1px solid rgba(255, 255, 255, 0.08);
  }

  .section-title {
    font-size: 18px;
    font-weight: 700;
  }

  label {
    display: grid;
    grid-template-columns: 170px 1fr auto;
    gap: 12px;
    align-items: center;
    color: rgba(236, 236, 245, 0.84);
  }

  label span {
    font-size: 14px;
  }

  input,
  select {
    min-height: 30px;
  }

  input[type='range'] {
    width: 100%;
    accent-color: #7f90ff;
  }

  strong {
    font-size: 13px;
    color: rgba(255, 255, 255, 0.68);
    min-width: 62px;
    text-align: right;
  }

  .toggle {
    grid-template-columns: 170px auto 1fr;
  }

  .small {
    border: 0;
    border-radius: 8px;
    padding: 7px 10px;
    cursor: pointer;
    color: #12131a;
    background: rgba(255, 255, 255, 0.88);
  }

  .small:disabled {
    cursor: default;
    opacity: 0.45;
  }

  .small.secondary {
    background: rgba(255, 255, 255, 0.16);
    color: rgba(255, 255, 255, 0.9);
  }

  .custom-sound-row {
    display: grid;
    grid-template-columns: 1fr auto;
    gap: 12px;
    align-items: center;
    padding: 12px 14px;
    border-radius: 12px;
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.08);
  }

  .custom-sound-meta {
    display: grid;
    gap: 4px;
  }

  .custom-sound-meta span {
    font-size: 14px;
    color: rgba(255, 255, 255, 0.92);
  }

  .custom-sound-meta small {
    font-size: 12px;
    color: rgba(255, 255, 255, 0.58);
  }

  .custom-sound-actions {
    display: inline-flex;
    gap: 8px;
    flex-wrap: wrap;
    justify-content: flex-end;
  }

  .time-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 12px;
  }

  .time-row label {
    grid-template-columns: 70px 1fr;
  }
</style>
