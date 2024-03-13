
# A simple and **_fast_** Self-host Audiobook server and player written fully in rust

The frontend and backend are both written in ***RUST***! this is a simple hobby project built with leptos + axum + sea-orm + tailwind. It's simple but fast

## compare to [AudiobookShelf](https://github.com/advplyr/audiobookshelf)

- very **_fast_** (that the reason I decide to write this for my own use): the AudiobookShelf would take 1-3s to change the page. while in audiobookroom. every operation are finished in **_10ms_**.
- very poor functionality:***most time you should use AudiobookShelf if you want a full-featured AUDIO BOOK PLAYER*** This is just an audio server and audio player, nothing else (like metadata fetch. Audio analysis)

## features

- keep reading history and progress for each user.
- user and password protected.
- the server and client are very lightweight, you could run it on your very old PC.

## quick start
1. create dir for store the data:`mkdir ./fetchbook`,`mkdir ./db`
2. prepare your download dir: in this example, it's `./(path_to_you_data)`
3. the docker-compose.yml file:
```yml
version: '2.1'
services:
  app:
    container_name: audiobookroom
    image: 'jiangqiu/audiobookroom:1.1'
    volumes:
      #!! don't delete this, you can chage ./fetchbook. in container: /app/fetchbook is the data directory which stores the book files
      - ./fetchbook:/app/fetchbook
      #!! mount the book library where you store the downloaded books. in container, use /test_book/some_book, to add book
      - ./(path_to_you_data):/test_book
    environment:
      - PUID=1000
      - PGID=1000
    ports:
      - '3000:3000'
    depends_on:
      - db
  db:
    image: 'mariadb:latest'
    container_name: mysql
    volumes:
      #!! the directory to store the database, don't delete this
      - ./db:/var/lib/mysql:Z
    environment:
      # !!!don't change these settings!!!, because currently it's hardcoded in the app
      - MARIADB_USER=audiobookroom
      - MARIADB_PASSWORD=audiobookroom
      - MARIADB_ROOT_PASSWORD=audiobookroom
  # delete this as you like
  adminer:
    image: adminer
    restart: always
    ports:
      - 8081:8080


```
4. now login into [localhost:3000](http://localhost:3000), setup the user and password. the signup page are only valid at the first install.
5. go to setting, add a book. if your book is located at `./(path_to_you_data)/mybook`, set the path as `/test_book/mybook`
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
- index
  ![index](/markdown/index.png)
- books
  ![player](/markdown/books.png)
- addbook
  ![addbook](/markdown/addbook.png)
- settings
  ![settings](/markdown/settings.png)

## todo
- there are a lot of things unfinished...
- manage current user
- implement profiles for authors and books.
- prettify the UI
- ...
