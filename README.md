# Transcribot Whisper wrapper
Simple wrapper for Whisper which:
- Takes 2 args: language code and audio file path
- Launches whisper as a child process
- Reads result from txt file and removes it
- Prints transcription result to stdout

## Installation
```sh
cargo install --git https://github.com/Kozlov-TranscriBot/transcribot-whisper-wrapper
```

## Usage
```sh
transcribot-whisper-wrapper <lang_code> <file_path>
```

See all lang codes [here](https://github.com/openai/whisper/blob/main/whisper/tokenizer.py)

