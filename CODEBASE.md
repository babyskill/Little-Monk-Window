# LittleMonkWindow — Codebase Map

> **Project Type:** macOS desktop companion widget  
> **Primary Stack:** Rust + Tauri v2 + Svelte  
> **Last Synced:** 2026-06-21

Read this file before searching the repo. Start with the target area, then search inside that scope.

## Directory Structure

```text
src/
├── App.svelte
├── app.css
├── app.d.ts
├── assets/
├── components/
└── lib/

src-tauri/
├── Cargo.toml
├── build.rs
├── capabilities/
├── icons/
├── resources/
└── src/
```

## File Index

### Root

| Path | Purpose |
|------|---------|
| `package.json` | Frontend scripts and JS dependencies |
| `vite.config.ts` | Vite build/dev config |
| `tsconfig.json` | TypeScript compiler config |
| `index.html` | Vite entry HTML |
| `svelte.config.js` | Svelte preprocessing config |
| `implementation_plan.md` | Current architecture and rollout plan |

### Frontend

| Path | Purpose |
|------|---------|
| `src/main.ts` | App bootstrap |
| `src/App.svelte` | Main monk/bubble UI |
| `src/app.css` | Global transparent window styling |
| `src/app.d.ts` | Asset module declarations |
| `src/components/PetWindow.svelte` | Composite monk window shell |
| `src/components/MonkSprite.svelte` | Monk placeholder art wrapper |
| `src/components/QuoteBubble.svelte` | Quote bubble renderer |
| `src/components/ReactionBubble.svelte` | Tap reaction capsule |
| `src/lib/audio.ts` | Bonk/bell playback helpers |
| `src/lib/quote.ts` | Shared Dhammapada quote payload types |
| `src/lib/spriteSlicer.ts` | Canvas slicing helper for future sprite packs |
| `src/lib/geometry.ts` | Hit-test helpers for future click-through logic |
| `src/lib/tauriApi.ts` | Thin Tauri API re-export |
| `src/assets/monk.svg` | Local monk placeholder art |
| `src/assets/Lotus.svg` | Reused Mac project resource |
| `src/assets/bell.mp3` | Reused Mac project resource |
| `src/assets/bonk_1.mp3` | Reused Mac project resource |

### Tauri / Rust

| Path | Purpose |
|------|---------|
| `src-tauri/Cargo.toml` | Rust dependencies and Tauri features |
| `src-tauri/build.rs` | Tauri build script |
| `src-tauri/tauri.conf.json` | Window, bundle, and capability config |
| `src-tauri/capabilities/main.json` | Default desktop capability set |
| `src-tauri/icons/icon.png` | Tray/bundle icon source |
| `src-tauri/resources/Dhammapada.json` | Bundled verse library |
| `src-tauri/src/main.rs` | App startup, tray, scheduler wiring |
| `src-tauri/src/quote.rs` | Quote library loading and payload mapping |
| `src-tauri/src/settings.rs` | App settings persistence |
| `src-tauri/src/scheduler.rs` | Quote interval loop |
| `src-tauri/src/tray.rs` | Tray menu and event handling |
| `src-tauri/src/window.rs` | Window visibility and click-through helpers |

## Primary Flows

- Startup: Tauri boots, loads settings, loads the Dhammapada library, configures the floating window, and starts the quote loop.
- Quote rotation: Rust picks the verse for the current 5-minute slot and emits `monk:quote`; frontend shows the bubble and plays `bell.mp3`.
- Tap interaction: Clicking the monk plays `bonk_1.mp3` and shows a short reaction.
- Tray controls: The tray menu can show/hide the monk and toggle always-on-top.

## Build And Test

```bash
npm install
npm run check
npm run build
cd src-tauri && cargo check
cd src-tauri && cargo test
```

## Notes

- The macOS source project `/Users/appdexter/Dev/Little-Monk-Mac` is the behavior reference.
- `implementation_plan.md` is the current source of truth for next phases.
- The monk image is currently a local placeholder and can be replaced with a cleaner sprite pack later.
