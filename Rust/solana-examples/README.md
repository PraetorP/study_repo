# solana-examples

 Examples of Solana programs written in Anchor

## Description

 The repo includes 3 programs written in Anchor and a prototype of the client-side written in Rust. For development tasks, testing has been implemented via `solana-program-test`  crate.

### Proxy Token Emission

 This program mints the token that it owns. The program has an owner who must carry out the initial initialization of the program by passing it the key of the new token. The program will perform the initialization of the token on its own.

## Quick Launch

* Follow the instructions [here](https://project-serum.github.io/anchor/getting-started/installation.html) to install dependencies for Anchor.
* Run tests to illustrate how programs work:

 ```
 make test
 ```

It looks like the solana-program-test (v 1.8.5) crate is not compatible with the current version of Rust (1.61.0), so you need to change the version to work correctly:

```
rustup override set 1.59
```
