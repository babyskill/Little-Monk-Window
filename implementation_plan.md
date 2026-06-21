# Implementation Plan — LittleMonkWindow (Desktop Widget for Windows)

This plan outlines the architecture, technology stack, and implementation steps to build the **LittleMonkWindow** desktop widget for Windows using Rust and Tauri, heavily inspired by the design and behavior of [LiteMonk (AgentPet)](file:///Users/trungkientn/Dev2/MacOS/agentpet) on macOS.

---

## Technical Stack & Architecture

- **Backend**: Rust + Tauri v2
  - Lightweight system tray management.
  - Native window control (transparency, position saving, window styling, click-through toggling).
  - File system CRUD for custom pet assets and configuration persistence.
- **Frontend**: Vite + Vanilla TypeScript (or Svelte for structured component state) + TailwindCSS/Vanilla CSS.
  - Transparent overlay rendering the animated monk sprite and quote bubble.
  - Sprite slicing via HTML5 Canvas API (matching macOS's alpha gutter-based slicing algorithm).
  - Audio playback via Web Audio API (low latency bonk sound).
- **Assets**: Imported from LiteMonk macOS (`Dhammapada.json`, `bonk_1.mp3`, `bell.mp3`, and pet spritesheets).

---

## User Review Required

> [!IMPORTANT]
> **Windows Click-Through (Mouse Passthrough) Behavior**
> To make the background of the widget transparent and click-through (so users can interact with folders/apps behind the widget, while still clicking the monk/bubble), we must dynamically toggle the cursor events:
> - **Default State**: Mouse ignores window events (`set_ignore_cursor_events(true)`).
> - **Active State (Hovering Monk or Bubble)**: Mouse captures window events (`set_ignore_cursor_events(false)`).
> We will implement this by detecting mouse coordinates relative to the DOM elements in Frontend, then invoking a Rust command to toggle click-through.
> Please review if this approach meets your expectations or if you prefer a different mechanism.

> [!NOTE]
> **Tech Stack Choice for Frontend**
> We recommend using **Vite + Svelte + Vanilla CSS** for the frontend because it yields a very compact binary, has minimal memory footprint, and provides excellent reactive data binding for sprite animation loops, settings, and custom quote additions.

---

## Open Questions

> [!WARNING]
> 1. **Pet Assets Loading**: Should we bundle a default Little Monk asset pack inside the binary (or App resources), or do we download it on the first launch similar to the macOS version?
> 2. **Mindfulness Bell Service**: On Windows, do we want to support native Windows notifications alongside the audio chime when the Mindfulness Bell triggers?

---

## Proposed Changes

We will bootstrap the Tauri project inside `/Users/trungkientn/Dev2/MacOS/little-monk-window`.

### Core Project Bootstrapping

#### [NEW] [tauri.conf.json](file:///Users/trungkientn/Dev2/MacOS/little-monk-window/src-tauri/tauri.conf.json)
Configure Tauri settings:
- Transparency enabled (`"transparent": true`).
- Window decorations disabled (`"decorations": false`).
- Always on top (`"alwaysOnTop": true`).
- Skip taskbar (`"skipTaskbar": true`).
- Configure two windows: `main` (floating pet widget) and `settings` (hidden by default).
- Configure System Tray settings.

#### [NEW] [Cargo.toml](file:///Users/trungkientn/Dev2/MacOS/little-monk-window/src-tauri/Cargo.toml)
Add necessary dependencies:
- `tauri` (v2 with tray-icon and window APIs).
- `serde` and `serde_json` (for manifest parsing).
- `window-shadows` (for the Settings window).

#### [NEW] [main.rs](file:///Users/trungkientn/Dev2/MacOS/little-monk-window/src-tauri/src/main.rs)
- Implement Tauri commands:
  - `toggle_click_through(ignore: bool)`: calls `window.set_ignore_cursor_events(ignore)`.
  - `start_dragging()`: calls `window.start_dragging()`.
  - `load_dhammapada_quotes()`: returns quote database.
  - `save_settings()` / `load_settings()`.
- System Tray setup: Add context menu (Show Settings, Toggle Mute, Exit).

---

### Frontend Components (Vite + Svelte/TS)

#### [NEW] [App.svelte](file:///Users/trungkientn/Dev2/MacOS/little-monk-window/src/App.svelte)
- High-level layout of the widget:
  - Speech bubble (QuoteBubble).
  - Tap button containing the monk sprite (PetView).
  - Reaction message text capsule.
- Monitors mouse position to dynamically toggle `toggle_click_through`.

#### [NEW] [SpriteSlicer.ts](file:///Users/trungkientn/Dev2/MacOS/little-monk-window/src/lib/SpriteSlicer.ts)
Port the Swift `SpriteSlicer` logic to JavaScript using Canvas context pixel manipulation:
- Detect rows and columns based on alpha transparent channels.
- Generate image data frames for animation rendering.

#### [NEW] [PetController.ts](file:///Users/trungkientn/Dev2/MacOS/little-monk-window/src/lib/PetController.ts)
Implement the core logic:
- Handle clicking (bonk sound playback, play hit animation with elastic scale bounce).
- Timer to rotate Dhammapada quotes.
- Trigger consecutive click tiers (reactions: `consecutivePets >= 6 ? tier 2 ...`).

---

### Assets Migration

#### [NEW] [Dhammapada.json](file:///Users/trungkientn/Dev2/MacOS/little-monk-window/src/assets/Dhammapada.json)
Copy multilingual Dhammapada text from [Dhammapada.json](file:///Users/trungkientn/Dev2/MacOS/agentpet/Sources/App/Resources/Dhammapada.json).

#### [NEW] [Audio Assets](file:///Users/trungkientn/Dev2/MacOS/little-monk-window/src/assets/)
Copy `bonk_1.mp3` and `bell.mp3` sound files.

---

## Verification Plan

### Automated Tests
- Setup unit tests for `SpriteSlicer` in JS/TS to ensure spritesheet coordinates are sliced correctly.
- Test JSON loading in Rust.

### Manual Verification
1. **Window Transparency & Hovering**: Verify the background of the widget is fully transparent. Check that clicking on transparent areas clicks *through* to underlying desktop elements.
2. **Interaction**: Hover over the Monk, click to bonk, observe the elastic scale effect, bonk audio, and reaction speech bubbles.
3. **Tray Icon**: Check that the app shows up in the System Tray and can open the Settings screen or close the app.
