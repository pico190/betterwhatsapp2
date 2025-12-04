# BetterWhatsApp

A lightweight WhatsApp client for Linux built with Tauri and WebKit.

## Installation

```bash
snap install betterwhatsapp
```

## Features

- **System Tray Icon**: Quick access from the system tray with menu options
- **Minimize to Tray**: Close the window to minimize to tray instead of fully closing
- **Native Notifications**: Get notified of unread messages
- **Unread Badge**: Window title shows unread message count
- **Low Resource Usage**: Optimized for Linux, minimal memory footprint
- **Always on Top**: Optional always-on-top mode for multitasking

## Development

### Prerequisites

- Rust 1.91+
- Node.js 16+

### Build from Source

```bash
# Install dependencies
pnpm install

# Development
pnpm run tauri dev

# Production build (generates .snap file)
pnpm run tauri build
```

The production build creates a distributable `.snap` package in `src-tauri/target/release/bundle/snap/`.

## Notes

- Uses WhatsApp Web (https://web.whatsapp.com) - authentication handled by WhatsApp
- Requires internet connection
- Notifications use native Linux notification system (libnotify)
