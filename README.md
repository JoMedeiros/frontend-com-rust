# Frontend com Rust

Esse projeto tem apenas finalidade de estudos da linguagem Rust.

## Uso

Esse projeto utiliza [yew](https://yew.rs/) e precisa de [trunk](https://trunkrs.dev/) para fazer build para WASM.
Para instalar o trunk:
```bash
cargo install trunk
```
Também é necessário adicionar o target:
```bash
rustup target add wasm32-unknown-unknown
```

Para executar o servidor:
```bash
trunk serve --open
```

## Alternativas

Alternativas para frontend com Rust:

- [smithy](https://www.smithy.rs/)
- [dioxus](https://dioxuslabs.com/)

