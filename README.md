# Rusty Craby

Discord bot for the WKClass Citizens

## Setup

This bot requires rust to compile and run, the easiest way to get this setup is though rustup and cargo.

1. Install rustup and cargo, [Instructions](https://rustup.rs/)
2. Fill in .envrc, See Testing below (I use [direnv](https://github.com/direnv/direnv) to manage env variables easily)
3. Build and run Debug `cargo run`

> Remember to install and link openssl
>
> ```
> brew install openssl
>
> ln -s /usr/local/opt/openssl/include/openssl /usr/local/include
> ```

## Testing

For testing your build locally you can use a test bot.

1. Head to the [Discord Developer Portal](https://discordapp.com/developers/) and create a new app. Name can be whatever you want. I named mine Rusty Craby Test
2. Under the Bot section create your bot and copy the token.
3. Add your token to the .envrc `TEST_DISCORD_TOKEN`
4. You can now add your bot to a server, under the oatuh2 section. under scopes select bot then administrator then copy the generated url to a new browser tab to authroize and connect it to a test server
5. Building with cargo in debug mode (without the --release option) will automatically use the `TEST_DISCORD_TOKEN` over the `DISCORD_TOKEN`.
6. Building with cargo in release mode will use the `DISCORD_TOKEN` instead

## Cross Compiling

This project uses [Cross](https://github.com/rust-embedded/cross) to cross compile to raspberry pi. First install it by running

```
cargo install cross
```

Once installed you can call the helper build scrip to produce a binary compatable for rpi3/4

```
./scripts/cross-build.sh
```
