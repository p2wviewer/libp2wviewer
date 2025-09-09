# p2wviewer

A command-line tool written in Rust to encrypt and decrypt images into self-contained noise images.

## Features
  - **Encrypt:** Encrypts an image into a new image that appears as random noise.
  - **Decrypt:** Decrypts a previously encrypted image back to its original state.
  - **Password-based encryption:** Use a password to en- and decrypt your images.
  - **File support:** Use a file as a key for en- and decryption.
  - **Image splitting:** Encrypt a single image into multiple blocks for security and avoid detection.

## Installation

You can download precompiled binaries from the [releases page](https://github.com/p2wviewer/p2wviewer/releases) or automatic github actions artifacts.

## Build and install from source

Ensure you have the Rust programming language and Cargo, its package manager, installed. If not, you can get them from the [official Rust website](https://www.rust-lang.org/tools/install).

Install the project in release (default) or debug mode:
    ```sh
    cargo install --git https://github.com/p2wviewer/p2wviewer
    ```

## Usage

### Encrypting an image

To encrypt a file, use the `encrypt` subcommand. You must specify the input file, the output file, and one of the authentication methods (`--password` or `--password-file`).

```sh
p2wviewer encrypt --input <INPUT_FILE> --output <OUTPUT_FILE> --password <YOUR_PASSWORD>
```

  - `-i, --input <INPUT_FILE>`: The path to the image you want to encrypt.
  - `-o, --output <OUTPUT_FILE>`: The path where the encrypted image will be saved.
  - `-p, --password <PASSWORD>`: A password to use for encryption.
  - `--password-file <PASSWORD_FILE>`: The path to a file to be used as the encryption key.
  - `-s, --split <SPLIT>`: The number of blocks to split the image into during encryption (default is 1).

**Example:**

```sh
p2wviewer encrypt --input path/to/my_photo.png --output encrypted.png --password "paytowin" --split 4
```

### Decrypting an image

To decrypt a file, use the `decrypt` subcommand. You must specify the input encrypted file, the desired output file, and the same authentication method used for encryption.

```sh
p2wviewer decrypt --input <INPUT_FILE> --output <OUTPUT_FILE> --password <YOUR_PASSWORD>
```

  - `-i, --input <INPUT_FILE>`: The path to the encrypted noise image.
  - `-o, --output <OUTPUT_FILE>`: The path where the decrypted original image will be saved.
  - `-p, --password <PASSWORD>`: The password used for encryption.
  - `--password-file <PASSWORD_FILE>`: The path to the key file used for encryption.

**Example:**

```sh
p2wviewer decrypt --input path/to/encrypted.png --output decrypted.png --password "freetoplay"
p2wviewer decrypt --input path/to/encrypted/file/dir/ --output decrypted.png --password-file keyfile.txt
```

## Logging

You can control the verbosity of the output using the `-v` or `--verbose` flag. Each additional `v` increases the log level.

  - `-v`: Log level `INFO`.
  - `-vv`: Log level `DEBUG`.
  - `-vvv`: Log level `TRACE`.

**Example:**

```sh
p2wviewer -vv encrypt --input image.jpg --output encrypted.png --password "imf2p"
```

## Contributing

Contributions are welcome\! If you find a bug or have a feature request, please open an issue.

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=p2wviewer/p2wviewer&type=Date)](https://www.star-history.com/#p2wviewer/p2wviewer&Date)

## License

This project is licensed under the Apache 2.0 License. See the `LICENSE` file for details.
