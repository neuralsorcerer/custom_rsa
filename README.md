# RSA Encryption System

This project implements a basic RSA encryption system in Rust. It demonstrates the generation of RSA keys, encrypting and decrypting messages, and ensuring that the encrypted data can be safely converted to and from a string format using base64 encoding.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine.

### Installing

A step-by-step series of examples that tell you how to get a development environment running:

Clone the repository:

```bash
git clone https://github.com/papaaannn/custom_rsa
cd custom_rsa
```

### Build the project:

```bash
cargo build
```

### Run the project:

```bash
cargo run
```

## Usage

To use the RSA encryption system, follow the prompts in the command line:

1. Run the program.
2. Enter a message to encrypt when prompted.
3. View the encrypted and decrypted output in the terminal.

### Example

```text
Enter a message to encrypt:
meow
Original: meow
Encrypted (Base64): CtoxhoVeIV9bLiuU2vqixDl9gSivvER2xlouzUYlVAftB1IsGPpe6SHMZnggqiK5KvS3v+w34md4sIsckWHkYHla0OjNI+544uGqE6u/jcXXorcxGrGpaX16nq+Cu2eIXcwAyfN1pdRZeVqZQ1fHrDu0cMc/hosidEdRs9WylLxuphyojWiHXvNZHAq3d+sjmM7xhSV3l6eHlJzIL/ce1AhVSdLZ+BEck78JC4xAXNjzlY88OLzV9z/JZzE9c7CPaFJsdTxM5p8ePhaytf5xaLZa+DNWGnwBHu3HCL5pdHyeELfRwWvRYQrxUHuFy2+23finrUiR1VIJ6Yj2bLcfOA==
Decrypted: meow
```

## License

This project is licensed under the MIT License - see the ![LICENSE](LICENSE) file for details.
