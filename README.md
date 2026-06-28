# Babu (БАБУ) Voice Assistant

![We are NOT limited by the technology of our time!](poster.jpg)

`Babu` (по-русски — **БАБУ**) is a voice assistant made as an experiment using neural networks for things like **STT/TTS/Wake Word/NLU** etc.

The main project challenges we try to achieve is:
 - 100% offline *(no cloud)*
 - Open source *(full transparency)*
 - No data collection *(we respect your privacy)*

Our backend stack is 🦀 **[Rust](https://www.rust-lang.org/)** with ❤️ **[Tauri](https://tauri.app/)**.<br>
For the frontend we use ⚡️ **[Vite](https://vitejs.dev/)** + 🛠️ **[Svelte](https://svelte.dev/)**.

*Other libraries, tools and packages can be found in source code.*

## Neural Networks

This are the neural networks we are currently using:

 - Speech-To-Text
	 - [Vosk Speech Recognition Toolkit](https://github.com/alphacep/vosk-api) via [Vosk-rs](https://github.com/Bear-03/vosk-rs)
 - Text-To-Speech
	 - [~~Silero TTS~~](https://github.com/snakers4/silero-models) *(currently not used)*
	 - [~~Coqui TTS~~](https://github.com/coqui-ai/TTS) *(currently not used)*
	 - [~~WinRT~~](https://github.com/ndarilek/tts-rs) *(currently not used)*
	 - [~gTTS~](https://github.com/nightlyistaken/tts_rust) *(currently not used)*
	 - [~~SAM~~](https://github.com/s-macke/SAM) *(currently not used)*
 - Wake Word
	 - [Rustpotter](https://github.com/GiviMAD/rustpotter) *(Partially implemented, still WIP)*
	 - [Picovoice Porcupine](https://github.com/Picovoice/porcupine) via [official SDK](https://github.com/Picovoice/porcupine#rust) *(requires API key)*
	 - [Vosk Speech Recognition Toolkit](https://github.com/alphacep/vosk-api) via [Vosk-rs](https://github.com/Bear-03/vosk-rs) *(very slow)*
	 - [~~Snowboy~~](https://github.com/Kitt-AI/snowboy) *(currently not used)*
 - NLU
	 - Nothing yet.
- Chat
	- [~~ChatGPT~~](https://chat.openai.com/) (coming soon)

## Supported Languages

Currently, only Russian language is supported.<br>
But soon, Ukranian and English will be added for the interface, wake-word detection and speech recognition.

## Self-learning command aliases

Babu can learn local aliases for existing commands. Say or type phrases like:

- `запомни команду открой мои файлы как открой проводник`
- `когда я скажу новая заметка выполняй открой блокнот`
- `learn command start writing as open notepad`

Learned aliases are stored in the user config directory in `learned_commands.toml` and are matched before built-in fuzzy command matching.

## How to build and run?

You need Rust and Node.js installed. On Linux, install the native audio/GUI libraries before running Rust builds:

```bash
sudo apt-get update
sudo apt-get install -y libasound2-dev libglib2.0-dev libgtk-3-dev libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf
```

Install frontend dependencies, then run the Tauri GUI from the GUI crate:

```bash
cd frontend
npm install
npm run check
npm run build
cd ../crates/babu-gui
cargo tauri dev
```

For a release installer, run `./scripts/build-installer.sh`. The script builds the frontend, the background assistant binary, copies runtime resources, and then runs the Tauri bundler.


<br><br>
*Thought you might need some of the platform specific libraries for [PvRecorder](https://github.com/Picovoice/pvrecorder) and [Vosk](https://github.com/alphacep/vosk-api).*

## Author

Abraham Tugalov

## Python version?
Old version of Babu was built with Python.<br>
The last Python version commit can be found [here](https://github.com/Priler/babu/tree/943efbfbdb8aeb5889fa5e2dc7348ca4ea0b81df).

## License

[Attribution-NonCommercial-ShareAlike 4.0 International](https://creativecommons.org/licenses/by-nc-sa/4.0/)<br>
See LICENSE.txt file for more details.
