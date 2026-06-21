# Implementation Plan — LittleMonkWindow

Desktop companion app for macOS built with Rust + Tauri. The app shows a small monk floating on screen and periodically displays a Buddhist verse bubble from a bundled scripture library.

Source reference: `/Users/appdexter/Dev/Little-Monk-Mac`

## Current Findings

- The source macOS app is a SwiftPM/AppKit + SwiftUI app named `AgentPet`.
- The active source already has the core behavior we need:
  - floating transparent `NSPanel` in `Sources/App/PetWindowController.swift`
  - quote timer and tap reactions in `Sources/App/PetController.swift`
  - Dhammapada loading/custom persistence in `Sources/App/DhammapadaStore.swift`
  - deterministic 5-minute quote rotation in `Sources/App/IdleBoost.swift`
  - sprite pack slicing in `Sources/App/SpriteSlicer.swift`
- Reusable bundled resources:
  - `Sources/App/Resources/Dhammapada.json`
  - `Sources/App/Resources/Lotus.svg`
  - `Sources/App/Resources/bell.mp3`
  - `Sources/App/Resources/bonk_1.mp3`
  - `assets/icon.png`
- The current `Little-Monk-Window` repo is not scaffolded yet. It only has project metadata and planning docs.
- `CODEBASE.md` is currently a generated placeholder and should be refreshed after scaffold.

## Product Direction

MVP should be calm and small:

- A transparent, borderless, always-on-top floating monk window.
- A speech bubble that shows one Dhammapada verse every interval.
- Default interval: 5 minutes.
- Default display duration: 20 seconds.
- Tapping the monk shows a short reaction and can play `bonk_1.mp3`.
- Tray/menu controls: Show/Hide, Toggle Always On Top, Quit.
- Settings window is Phase 2 unless needed for testing.

## Key Technical Decision

Use Tauri v2 + Svelte + Rust.

Why:

- Tauri gives native window control and packaging with a small footprint.
- Rust is the right place for quote scheduling, settings persistence, and native window commands.
- Svelte keeps the transparent overlay UI simple and reactive.

Trade-off:

- Tauri cannot perfectly match AppKit `NSPanel` behavior out of the box. We may need a small macOS-native helper later for `canJoinAllSpaces`, non-activating panel behavior, or full-screen Space behavior.
- Transparent webview on macOS may require `macOSPrivateApi`, which is acceptable for direct DMG/notarized distribution but risky for Mac App Store distribution.

## Proposed Architecture

```text
src-tauri/
├── Cargo.toml
├── tauri.conf.json
├── resources/
│   ├── Dhammapada.json
│   ├── bell.mp3
│   └── bonk_1.mp3
└── src/
    ├── main.rs
    ├── app_state.rs
    ├── commands.rs
    ├── quotes.rs
    ├── scheduler.rs
    ├── settings.rs
    ├── tray.rs
    └── window.rs

src/
├── App.svelte
├── components/
│   ├── MonkSprite.svelte
│   ├── PetWindow.svelte
│   ├── QuoteBubble.svelte
│   └── ReactionBubble.svelte
├── lib/
│   ├── audio.ts
│   ├── geometry.ts
│   ├── spriteSlicer.ts
│   └── tauriApi.ts
└── assets/
    ├── lotus.svg
    └── monk/
```

## Window Behavior

Tauri window config target:

```json
{
  "label": "pet",
  "width": 320,
  "height": 380,
  "transparent": true,
  "decorations": false,
  "alwaysOnTop": true,
  "skipTaskbar": true,
  "resizable": false,
  "shadow": false,
  "focus": false,
  "visible": true
}
```

Click-through strategy:

- MVP: fail-safe interactive window, with transparent background visually clear.
- Next: hover/hit-rect toggling via Rust command `set_ignore_cursor_events(ignore: bool)`.
- Avoid per-pixel click-through assumptions. Tauri toggles the whole window, so geometry must be carefully tested on Retina displays.

## Quote Logic

Port from Swift:

- Load bundled `Dhammapada.json`.
- Use Vietnamese as primary because source has 423 Vietnamese verses and only a few English translations.
- Select verse by time slot:

```text
slot = unix_timestamp / (5 * 60)
verse = verses[slot % verses.length]
```

- Store custom quotes later in app config dir, not inside the bundle.
- Keep attribution fields visible in settings/about before any public release.

## Asset Strategy

Use immediately:

- `Dhammapada.json`
- `Lotus.svg`
- `bell.mp3`
- `bonk_1.mp3`
- `assets/icon.png`

Needs decision:

- Default monk character asset.

The Swift app does not bundle a monk spritesheet in active resources. It supports imported/downloaded pet packs through `pet.json + spritesheetPath`. For this Tauri app, choose one:

1. Bundle a new original monk sprite with clear license.
2. Generate a simple original monk image/spritesheet for MVP.
3. Reuse remote pet catalog only as optional import, not as bundled default.

Recommended: generate or draw an original monk asset for MVP, then keep the existing pet pack importer for Phase 2.

## Phases

### Phase A — Infrastructure

- Scaffold Tauri v2 + Svelte.
- Configure transparent `pet` window.
- Add tray menu.
- Copy bundled quote/audio/icon resources.
- Add Rust modules: quotes, settings, window commands.
- Verification: `npm run build`, `cargo fmt --check`, `cargo test`.

### Phase B — UI Shell

- Render monk placeholder or generated original monk asset.
- Render quote bubble with mock/current quote.
- Render tap reaction capsule.
- Keep fixed 320x380 window to avoid resize drift.
- Manual checkpoint: user visually checks transparent floating UI on macOS.

### Phase C — Logic

- Rust scheduler emits quote updates.
- Frontend receives quote events.
- Tap interaction plays sound and reaction.
- Add persistence for size, visibility, always-on-top, interval.
- Add click-through hit-rect after UI shell is approved.

## Risks

- `macOSPrivateApi` can affect Mac App Store eligibility.
- Tauri always-on-top may not match AppKit `NSPanel` across all Spaces/full-screen apps.
- Whole-window click-through can make the monk unclickable if hover geometry is wrong.
- Retina coordinate conversion can break hit-rect detection.
- Dhammapada translations and sound assets need license/attribution review before commercial/public release.
- Pet catalog artwork should not be bundled without explicit license verification.

## Verification Plan

Automated:

```bash
npm run build
cd src-tauri && cargo fmt --check
cd src-tauri && cargo clippy --all-targets --all-features -- -D warnings
cd src-tauri && cargo test
```

Manual:

- `npm run tauri dev`
- Window is transparent and borderless.
- Monk stays visible above normal desktop windows.
- Quote appears and hides on schedule.
- Tap reaction works and sound plays.
- Tray menu can show/hide and quit.
- Later: transparent areas click through while monk/bubble remain interactive.

## Immediate Next Step

Start Phase A by scaffolding Tauri v2 + Svelte in this repo, then copy `Dhammapada.json`, `Lotus.svg`, `bell.mp3`, `bonk_1.mp3`, and `icon.png` from `/Users/appdexter/Dev/Little-Monk-Mac`.
