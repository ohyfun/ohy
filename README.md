# ohy

"ohy" is derived from the Japanese word "おはよ" (good morning). It's a lightweight command-line tool written in Rust that converts web applications into desktop applications.

## Key Features

* Small single executable: Less than 5MB.
* Data privacy isolation: Isolates based on the connection address and application name.
* Multiple instances: Applications can be run concurrently with isolated session data by default.
* Easy deployment: Simply copy the binary file for use; no need for sudo privileges.
* Automatic icon retrieval: Automatically fetches the application icon.
* Centralized caching: Local cache data is stored in a fixed directory, avoiding scattered data writes that could clutter system directories.

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

## Installation

1. Install the application using Cargo
```bash
cargo install ohy
```
2. Linux Dependencies (Windows and macOS do not require this step)

Arch Linux / Manjaro:
```bash
sudo pacman -S webkit2gtk-4.1
```

Debian / Ubuntu:
```bash
sudo apt install libwebkit2gtk-4.1-dev
```

Fedora
```bash
sudo dnf install gtk3-devel webkit2gtk4.1-devel
```

Centos
```bash
sudo yum install gtk3-devel webkit2gtk4.1-devel
```

## Benefits

* Lightweight and efficient resource usage
* Enhanced privacy through session isolation
* Simple and straightforward configuration
* Cross-platform compatibility

## Developers
The program has only been tested on Linux and Windows (since I'm a Linux user and don't have a Mac). <br/>
In theory, it should work on macOS (as the underlying dependency, wry, supports all platforms).<br/>
although there might be some bugs with crate imports. Mac users are welcome to submit issues.

## screenshot

### linux
![qwen](https://github.com/user-attachments/assets/eec15bdd-f1fa-4364-bd5c-e6229b70a46b)
![qwen-dark](https://github.com/user-attachments/assets/02eeba87-62b8-420a-a727-b8033a5c1bc0)
![ohy-icon](https://github.com/user-attachments/assets/fc092cdd-005b-4d81-9525-c0e92dfedd7b)
> linux app cache data directory `$HOME/.config/ohy/*`

![app-context-dir](https://github.com/user-attachments/assets/c7297da3-9335-4d6b-b3f8-d27216557624)

### windows
![app1](https://github.com/user-attachments/assets/1a5d86a9-0408-4fe3-b380-87635d7d44cd)
![app2](https://github.com/user-attachments/assets/07fa2e21-d825-44d7-bd87-4009db4dd388)
![app3](https://github.com/user-attachments/assets/73c990ca-719b-4696-ae98-105e52012ef7)
![dark-mode](https://github.com/user-attachments/assets/8585f8ca-7e50-42c8-a520-df9bca5b3691)
![icon](https://github.com/user-attachments/assets/d6897c82-8df1-43e2-8c0c-6a0e2dd8ce21)
> windows app cache data directory `C:\Users\$USER_NAME\AppData\Roaming\ohy\*`

![web-content](https://github.com/user-attachments/assets/3c4a20d8-17d9-48c6-8259-36de75380772)
