<div align="center">

# Babu AI
### Intelligent Offline Voice Assistant

<img src="poster.jpg" alt="Babu Banner" width="100%">

<p>
An intelligent, privacy-first desktop voice assistant built with modern AI technologies.<br>
Designed to run completely offline while delivering fast, natural and secure voice interactions.
</p>

<p>

![Rust](https://img.shields.io/badge/Rust-Backend-orange?style=for-the-badge&logo=rust)
![Tauri](https://img.shields.io/badge/Tauri-Desktop-blue?style=for-the-badge&logo=tauri)
![Svelte](https://img.shields.io/badge/Svelte-Frontend-ff3e00?style=for-the-badge&logo=svelte)
![Vite](https://img.shields.io/badge/Vite-Build-purple?style=for-the-badge&logo=vite)
![License](https://img.shields.io/badge/Open%20Source-Yes-success?style=for-the-badge)

</p>

---

### 🚀 Fast • 🔒 Private • ⚡ Offline • 🧠 AI Powered

</div>

# Overview

**Babu AI** is a next-generation desktop voice assistant focused on **privacy**, **performance**, and **complete local execution**.

Unlike traditional assistants that constantly communicate with cloud servers, Babu performs speech recognition, wake-word detection and command processing directly on your computer.

No telemetry.

No hidden analytics.

No personal data collection.

Your voice never leaves your device.

---

# Vision

Our goal is to build one of the most advanced open-source desktop AI assistants with the following principles:

- 🔒 100% Offline Processing
- 🧠 AI-Powered Voice Recognition
- ⚡ Extremely Low Latency
- 🛡 Privacy by Design
- 🌍 Open Source & Transparent
- 📦 Cross-Platform Architecture
- 🔌 Easily Extendable
- 💻 Native Desktop Performance

---

# Core Features

### 🎙 Speech Recognition (STT)

Fast offline speech recognition powered by neural networks.

- Continuous Listening
- Streaming Recognition
- High Accuracy
- Local Processing
- No Internet Required

---

### 🔊 Natural Speech Synthesis (TTS)

Generate natural voice responses directly on-device.

Supported engines include:

- Silero TTS
- Coqui TTS
- Windows TTS
- SAM
- gTTS (legacy)

Future versions will include expressive neural voices.

---

### 👂 Wake Word Detection

Activate the assistant hands-free using custom wake words.

Current implementations:

- Rustpotter
- Picovoice Porcupine
- Vosk Wake Detection

Future improvements include:

- Personal voice adaptation
- Custom wake-word training
- Lower CPU usage
- Faster activation

---

### 🧠 Natural Language Understanding (NLU)

Upcoming AI-powered language understanding module capable of:

- Intent Recognition
- Context Awareness
- Multi-step Commands
- Smart Dialogs
- Personalized Responses

---

### 🤖 AI Chat

Future versions will support:

- Local Large Language Models
- AI Conversations
- Knowledge Base
- Context Memory
- Productivity Assistant

---

# Privacy First

Privacy is the foundation of Babu.

Unlike cloud assistants, Babu never uploads your voice recordings.

✔ No Cloud Processing

✔ No Telemetry

✔ No Tracking

✔ No User Profiling

✔ No Data Collection

Everything stays on your own computer.

---

# Technology Stack

## Backend

- Rust
- Tauri
- Tokio
- Serde
- Crossterm
- Rodio
- CPAL

---

## Frontend

- Svelte
- TypeScript
- Vite
- HTML5
- CSS3

---

## AI Stack

### Speech Recognition

- Vosk
- Vosk-rs

### Wake Word

- Rustpotter
- Picovoice Porcupine

### Speech Synthesis

- Silero
- Coqui
- Windows TTS
- SAM

---

# Supported Languages

Current:

- 🇷🇺 Russian

Planned:

- 🇺🇸 English
- 🇺🇦 Ukrainian
- 🇹🇯 Tajik
- 🇩🇪 German
- 🇫🇷 French
- 🇪🇸 Spanish

---

# Smart Command Learning

Babu can automatically learn new aliases for existing commands.

Examples:

```
remember command open explorer as open files
```

```
learn command start writing as open notepad
```

```
запомни команду открой документы как открой проводник
```

Learned commands are stored locally inside:

```
learned_commands.toml
```

No cloud synchronization is used.

---

# Project Structure

```
Babu
│
├── frontend/
├── crates/
│   ├── assistant/
│   ├── gui/
│   ├── speech/
│   ├── wakeword/
│   └── core/
│
├── models/
├── resources/
├── scripts/
├── docs/
└── assets/
```

---

# Installation

## Requirements

- Rust (latest stable)
- Cargo
- Node.js
- npm

Linux dependencies:

```bash
sudo apt update

sudo apt install \
libasound2-dev \
libglib2.0-dev \
libgtk-3-dev \
libwebkit2gtk-4.1-dev \
libappindicator3-dev \
librsvg2-dev \
patchelf
```

---

# Development

Install frontend dependencies:

```bash
cd frontend

npm install

npm run check

npm run build
```

Run the desktop application:

```bash
cd ../crates/babu-gui

cargo tauri dev
```

Create production build:

```bash
./scripts/build-installer.sh
```

---

# Roadmap

- AI Conversation Engine
- Plugin Marketplace
- Local LLM Support
- Vision Recognition
- OCR
- Smart Automation
- Voice Macros
- Smart Home Integration
- Calendar Assistant
- Notes Assistant
- Browser Automation
- Multi-language Recognition
- Better Wake Word Detection
- GPU Acceleration

---

# Contributing

Contributions are always welcome.

If you have ideas, improvements, or bug fixes, feel free to open an Issue or submit a Pull Request.

Let's build the future of privacy-focused AI together.

---

# Performance Goals

- ⚡ Startup < 1 second
- 🎤 Recognition latency < 200ms
- 💾 Low memory usage
- 🖥 Native desktop performance
- 🔋 Minimal CPU consumption

---

# Author

## Abdullo Ashurov

**Full Stack Software Engineer**

Specialized in:

- AI Applications
- Desktop Software
- Backend Engineering
- Rust Development
- Go Development
- React Ecosystem
- Distributed Systems
- System Architecture

GitHub Portfolio:

```
Designed and developed with ❤️ by Abdullo Ashurov
```

---

# License

This project is distributed under the

**Attribution-NonCommercial-ShareAlike 4.0 International**

See the **LICENSE** file for details.

---

<div align="center">

### ⭐ If you like this project, don't forget to star the repository.

**Building the future of private AI.**

</div>
