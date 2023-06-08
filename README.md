# CapsLock Modifier
This Rust script uses the rdev library to modify the behavior of the CapsLock key.

## Overview
This script changes the behavior of the CapsLock key on your keyboard. When CapsLock is held down, other keys will be sent as if MetaLeft (commonly the Windows key on Windows or Command key on Mac) was also held down. If CapsLock is pressed and released without any other key press, it sends an Escape key press and release.

## Prerequisites
This script uses the rdev library, which you need to add to your project. You can add it using the following command:

```sh
cargo add rdev
```

## Usage
To run this script, simply navigate to the directory containing it and use the following command:

```sh
cargo run
```

## Troubleshooting
If you encounter errors, ensure that:

- The rdev library is correctly installed.
- You have the necessary permissions to capture and simulate keyboard events.
- Your Rust environment is correctly set up.

**Note**: Be aware that changing key behaviors can have broad impacts on your system's usability. Use this script responsibly, and ensure you have another way to input text if needed. Also, note that you may need to adjust the code if your keyboard layout or operating system behaves differently.
