set windows-shell := ["nu" , "-c"]


up:
    sea-orm-cli migrate up
fresh:
    touch audiobookroom.db
    sea-orm-cli migrate fresh
pull:
    git pull
serve:
    cargo leptos serve --release
deploy_db target_addr:
    rsync -au --progress ./audiobookroom.db {{target_addr}}:~/
generate:
    sea-orm-cli generate entity -o src/entities --with-serde=both
watch:
    cargo leptos watch
deploy target_addr:pull
    cargo leptos build --release
    cargo build --release --bin add_book --bin add_user --bin modify_user
    rsync -au --progress ./target/release/audiobookroom ./target/site \
      ./target/release/add_book \
       ./target/release/add_user \
        ./target/release/modify_user \
         {{target_addr}}:~/
add_book author name:
    cargo run --bin add_book --features=ssr -- --db sqlite://audiobookroom.db --new-book-name {{name}} --author-name {{author}} --source-dir ./testbooks --book-dir fetchbook  
