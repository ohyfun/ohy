# ohy

"ohy" is derived from the Japanese word "おはよ" (good morning). It's a lightweight command-line tool written in Rust that converts web applications into desktop applications.

## Key Features

* Compact executable size (under 5MB)
* Privacy-focused data isolation based on URL and application name
* Multiple instance support with isolated sessions by default
* Automatic application icon fetching
* Centralized local cache storage

## Usage

```bash
Usage: ohy --url <url> [-n <name>] [-w <width>] [-h <height>] [-a <user-agent>]

Options:
  --url             url example https://www.github.com
  -n, --name        name
  -w, --width       width default 1200
  -h, --height      height default 780
  -a, --user-agent  user agent
  --help, help      display usage information
```

## Example

To create a desktop application for qwen chat:

```bash
ohy --url https://chat.qwenlm.ai -n qwen
```

## Benefits

* Lightweight and efficient resource usage
* Enhanced privacy through session isolation
* Simple and straightforward configuration
* Cross-platform compatibility

## screenshot

![app-context-dir](https://github.com/user-attachments/assets/c7297da3-9335-4d6b-b3f8-d27216557624)
![ohy-icon](https://github.com/user-attachments/assets/fc092cdd-005b-4d81-9525-c0e92dfedd7b)
![qwen-dark](https://github.com/user-attachments/assets/02eeba87-62b8-420a-a727-b8033a5c1bc0)
![qwen](https://github.com/user-attachments/assets/eec15bdd-f1fa-4364-bd5c-e6229b70a46b)