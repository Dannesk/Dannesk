# Dannesk

Dannesk is a DeFi application for managing XRPL and Bitcoin wallets.

## Features

- Create and import wallets for **Bitcoin** and **XRP Ledger (XRPL)**.
- Trade stablecoins on XRPL's decentralized exchange (Dex), currently supporting RLUSD (Ripple) and EUROP (Schuman Financial).
- User keys are securely stored in device-specific key storage:  
  - Secure Enclave on macOS  
  - Windows Credential Manager on Windows  
  - Secret Service keyring on Linux  
- All cryptographic operations, including transaction signing, happen **client-side**; the server only forwards encrypted data blobs.
- Entire stack is built entirely in Rust for performance and security.
- Frontend built with [egui](https://github.com/emilk/egui), a Rust GUI library.

## Installation

Download and install Dannesk from [dannesk.com](https://dannesk.com) by selecting your platform.

## Security

- User keys are encrypted using AES-256 encryption and protected by the user’s passphrase.  
- Even if device key storage is compromised, the encrypted keys cannot be accessed without the passphrase.  
- All transaction signing is performed locally on the user's device to ensure privacy and security.
- You can delete keys from your device by clicking **Delete** in the application.

## License

Dannesk is licensed under the GNU General Public License v3 (GPLv3). See the [LICENSE](LICENSE) file for details.

