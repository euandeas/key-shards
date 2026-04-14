# Key Shards

This repository contains 3 projects that where developed as part of my dissertation project at the University of Nottingham on the topic of "Making Secure Key Management More Accessible Using Secret Sharing". The project was completed in April 2024 and supervised by Dr. Tim Muller. The paper is available upon request, at my sole discretion.

## Abstract

With the increase in online privacy concerns, comes the need for secure key/password management. Although people are told to physically back up their keys, this is often not done and the risk of losing them is high.

This project explores Shamir’s Secret Sharing and its implementations, in order to inform the design and implementation of an application that makes secure key management more accessible. This resulted in a library implementation that provides the core functionality of the secret sharing algorithm, including an authenticated encryption scheme and a BIP-39 mnemonic scheme. An application was also developed to provide a user-friendly interface for the library, allowing users to interact with the secret-sharing algorithm without needing to understand the underlying implementation details or programming. The library and application both include a predefined shares feature, that was developed to allow the user to create shorter memorable shares with secrets of any length.

The security of the algorithm, application, and particularly the shorter shares feature was carefully analyzed to ensure that usage of the application would not introduce any risks to secure key management. This determined that with the right education and correct usage, it is likely to not cause any risk, and will greatly benefit the users’ key management. The application was designed to be cross-platform and is open-source-ready, intending to be freely available to use, modify,
and review.

## Projects

- [Key Shards](./key-shards/README.md) — A desktop application (Tauri + SvelteKit) that provides a UI to create and combine secret shares.

- [shami_rs](./shami_rs/README.md) — A pure Rust library implementing Shamir's Secret Sharing with optional AEAD wrapping, BIP‑39 compression helpers, padding, and an experimental predefined‑shares feature.

- [rust-bip39](./rust-bip39/README.md) — A Rust implementation of BIP‑39 mnemonic codes forked from [rust-bitcoin/rust-bip39](https://github.com/rust-bitcoin/rust-bip39) to allow for optional low‑entropy features.

## Licenses

Both Key Shards and shami_rs are dual‑licensed MIT / Apache‑2.0). rust-bip39 is licensed under the MIT license. See the LICENSE files in each subproject directory for details.

## Contact

If you have any questions, suggestions, or would like to request access to the paper, please feel free to contact me at [contact@euandeas.com](mailto:contact@euandeas.com).
