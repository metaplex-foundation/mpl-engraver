# Mpl Engraver

Metaplex Engravings provide an additional step in the direction of immutability, beyond the scope of standard Solana programs, for any creators unsatisfied with the regular, immutable NFTs. The Solana programming model allows Solana programs to modify any accounts they own, meaning that users of a program must trust the authors of the Solana program as long as the authors maintain upgrade authority over the program. There are many good reasons to keep programs mutable for some time including new features, bug/security fixes and maintaining ecosystem compatibility; the vast majority of programs on Solana are mutable for these reasons.  All program deployers should maintain security best practices such as keeping private keys in a multi-sig to prevent deploy key theft and malicious program changes that could result in unwanted account changes, which in Token Metadata’s case could be changes to people’s NFTs (note: Token Metadata can not change the ownership of the underlying token, which is owned by the SPL Token program and is already immutable, or Token22 mints, which are owned by the Token22 program. The ownership of Token Records, which track token ownership and delegation, must remain with Token Metadata to properly allow pNFT transfers).

While Metaplex uses extremely thorough security practices for the Token Metadata program, some users have expressed concern over the malicious program upgrade attack vector. To mitigate this concern, Metaplex has developed an additional program, the Engraver, that can assume ownership of any Solana account, preventing the originating program, such as Token Metadata from modifying the account contents. The Engraver would be an immutable program itself (i.e. upgrade authority is burned) and therefore not prone to the issues of Solana program upgrade authority.

Engraved accounts are therefore immune to any modifications while still preserving the account address and ability of the originating program to sign using the PDA. Pending Token Metadata immutability, engraving NFTs provides users with a way to ensure their NFTs are completely immutable.

The Metaplex Engraver program is queued up for a full audit and immutability as soon as possible.


## Programs

This project contains the following programs:

- [Mpl Engraver](./programs/mpl-engraver/README.md) `ENGRVY4DL6uKDnNS91hCkJMwzTfcofYpkZH8zsgJfzA3`

You will need a Rust version compatible with BPF to compile the program, currently we recommend using Rust 1.68.0.

## Clients

This project contains the following clients:

- [JavaScript](./clients/js/README.md)
- [Rust](./clients/rust/README.md)

## Contributing

Check out the [Contributing Guide](./CONTRIBUTING.md) the learn more about how to contribute to this project.
