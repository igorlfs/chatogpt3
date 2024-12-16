# Trabalho Prático - Engenharia de Software II

## Integrantes

- Igor Lacerda Faria

## Explicação do Sistema

O ChatoGPT3 é a segunda iteração do ChatoGPT, um programa que simula um _chat_ com uma LLM, mas com um _catch_: de vez em quando ele dá respostas inesperadas!

## Tecnologias Utilizadas

O [Tauri](https://tauri.app/), o sistema para desenvolver aplicações nativas usando tecnologias web (e Rust) foi utilizado, tal como no seu predecessor. No entanto, essa iteração conta com uso do _framework_ [Svelte](https://svelte.dev/) no _frontend_. Além disso, foi usado um sistema de ORM de Rust, o [diesel](https://diesel.rs/). Os testes foram feitos utilizando-se o próprio _framework_ _built-in_ de Rust, via `cargo test`.
