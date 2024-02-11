[![](https://dcbadge.vercel.app/api/server/Rh7VgCtmFN)](https://discord.gg/Rh7VgCtmFN)
# A simple and **_fast_** Self-host Audiobook server and player written fully in rust

this is a simple hobby project built with leptos + axum + sea-orm + tailwind. It's simple but fast

## compare to [AudiobookShelf](https://github.com/advplyr/audiobookshelf)

- very **_fast_** (that the reason I decide to write this for my own use): the AudiobookShelf would take 1-3s to change the page. while in audiobookroom. every operation are finished in **_10ms_**.
- very poor functionality:***most time you should use AudiobookShelf if you want a full-featured AUDIO BOOK PLAYER*** This is just an audio server and audio player, nothing else (like metadata fetch. Audio analysis)

## features

- record reading history and progress.
- user and password protected.
- the server and client are very lightweight, you could run it on your very old PC.

## install and serve

1. clone this repo
2. Prepare the perquisites:
   1. pnpm: this project use tailwindcss, so pnpm is needed (or NPM, yarn as you like)
   2. cargo-leptos: `cargo install cargo-leptos`
   3. sea-orm-cli: `cargo install sea-orm-cli`
   4. just: `cargo install just`
3. install the node_modules: `pnpm -i`
4. fresh the database (**_only do this for the first time to run. This will erase all previous data_**): `just fresh`
5. start the server:`just serve`
6. read the output, the site should be served at http://127.0.0.1:3003

## notes

1. when you run with `cargo leptos serve`, it will read the config in Config.toml leptos config. Feel free to change
2. when you run manually (read justfile (deploy) to know more how to run it manually). You should write your own config in .env file(please read [leptos doc](https://github.com/leptos-rs/cargo-leptos?tab=readme-ov-file#environment-variables) to know more about leptos config)
3. Generally you should not listen to 127.0.0.1. But if you listen on 0.0.0.0. Make sure to use a reverse proxy to provide https connection: **\*this is very important**

## screenshots

- login
  ![login](/markdown/login.png)
- player
  ![player](/markdown/player.png)
- addbook
  ![addbook](/markdown/addbook.png)


## todo
- there are a lot of things unfinished...
- manage current user
- implement profiles for authors and books.
- prettify the UI
- ...
