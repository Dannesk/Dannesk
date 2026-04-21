# Dannesk v0.4.0

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-orange)](https://www.rust-lang.org/)
[![UI: Dioxus](https://img.shields.io/badge/UI-Dioxus-6f42c1)](https://dioxuslabs.com/)
[![Graphics: wgpu](https://img.shields.io/badge/Graphics-wgpu-1f425f)](https://wgpu.rs/)
[![App: Dannesk](https://img.shields.io/badge/App-Dannesk-334155)](https://dannesk.com)

Dannesk is a non-custodial DeFi wallet for **Bitcoin** and the **XRP Ledger**. Built in Rust for security and reliability, the app gives users complete control over their private keys, while enabling powerful trading capabilities on **XRPL’s native CLOB**. 

<p align="center">
  <img src="./assets/dashboard.png" width="800"/>
</p>

---

### Multi-Chain Wallet

Users may create a new wallet or import an existing one for:

- **Bitcoin**
- **XRP**

Additionally, users may enable trustlines and trade currencies for a fraction of a cent on the **XRPL native order book (CLOB)**.
Swaps are atomic, and occur **directly on-chain** with no centralized intermediary.

Supported assets include:

- **XRP**
- **RLUSD**
- **EUROP**
- **XSGD**
- **BTC**

---

### Security

Dannesk supports the optional BIP39 passphrase (sometimes called the 25th word). The 25th word allows for enhanced wallet security and the deterministic generation of multiple wallets from the same seed. 

Upon import/create, users are prompted to choose their own encryption passphrase and bip39 passphrase. The private keys are then encrypted locally on the user’s device using AES-256 encryption and Argon2id for key derivation. 

Transactions are **signed locally**. The signed transaction blob is then **broadcast to the network**. For added security, memory is also **zeroized** after signing operations. At no time does any sensitive data leave the user's device. 
  
---

### Encryption

Dannesk uses AES-256 Encryption, along with Argon2id for password derivation. 
---


### Key Management

Dannesk uses AES-256 Encryption, along with Argon2id for password derivation. 
---

### Desktop v0.4.0 



Because we use dioxus native, a hyper-modern engine for the desktop app, which uses stylo/taffy/and the wgpu for rendering, older devices with outdated drivers may fail to render the UI correctly. 


---


### Android v0.4.0 

The Android app is available for direct download as an APK, but is not yet available on the playstore. 
The app uses a rust core for the communication layer and state management, and a very thin kotlin layer for rendering. 
The app should work ony most devices; however, because we use Argon2id for key derivation, you may find older devices to be rather slow when importing/creating or sending a transaction. Newer devices should not have a problem. 

---


### Roadmap 

1. Replace by Fee for Bitcoin
2. Converting dust
3. Loans for Bitcoin
4. Federated Bridge between XRP and BTC

We aim to release v1.0.0 for all platforms by the second quarter of 2027. 


---

## Installation

Download the latest release:

https://dannesk.com

Supported platforms:

- Linux (.deb) 
- Windows (.exe) 
- Android (.apk)

---

## License

Dannesk is licensed under the **GNU General Public License v3 (GPLv3)**.