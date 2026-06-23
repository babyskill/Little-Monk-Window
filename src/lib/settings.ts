export type AppSettings = {
  always_on_top: boolean;
  show_quote: boolean;
  show_idle_message: boolean;
  quote_interval_secs: number;
  quote_visible_secs: number;
  pet_size: number;
  font_size: number;
  quote_language: 'vi' | 'en' | string;
  bell_enabled: boolean;
  bell_interval_minutes: number;
  bell_sync_message: boolean;
  bell_sound_enabled: boolean;
  bell_sound: 'bell' | 'bonk' | string;
  bell_volume: number;
  bell_repeat_count: number;
  quiet_hours_enabled: boolean;
  quiet_hours_start_minutes: number;
  quiet_hours_end_minutes: number;
};

export const defaultSettings: AppSettings = {
  always_on_top: true,
  show_quote: true,
  show_idle_message: true,
  quote_interval_secs: 300,
  quote_visible_secs: 20,
  pet_size: 145,
  font_size: 17,
  quote_language: 'vi',
  bell_enabled: false,
  bell_interval_minutes: 30,
  bell_sync_message: true,
  bell_sound_enabled: true,
  bell_sound: 'bell',
  bell_volume: 0.75,
  bell_repeat_count: 1,
  quiet_hours_enabled: false,
  quiet_hours_start_minutes: 22 * 60,
  quiet_hours_end_minutes: 7 * 60,
};
