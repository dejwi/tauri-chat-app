# Chat app created with Tauri
Contains frontend, and client+server backend

Based on handling raw bytes streamed on a TCP socket + handles 6 different payloades types encoded within the stream

# Features
- handled in raw bytes
- persistent chat log - server sends saved chat to the connected user
- connected/disconnected announcements
- username/avatar

## Stack
Tauri

Rust, tokio, serde

Typescript, react, zustand, tailwindcss, daisyui, framer-motion

## How to run
Download already compiled version from [Releases](https://github.com/dejwi/tauri-chat-app/releases/tag/v1.0) (All platforms)

## How to run in dev environment
1. Make sure you have installed `Rust` and `Node`
2. run `pnpm run tauri dev`