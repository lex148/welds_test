
# performance testing WELDS

This repo holds a actix-web site using welds that can be used for performance testing

## steps to use:

1) boot up the databases using `docker compose up -d`
2) edit `./src/main.rs` and select the DB you would like to test
3) boot the server in release mode `cargo run --release`
4) run `ab -n 20000 -c 50 'http://127.0.0.1:3000/'`


