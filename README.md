
# performance testing WELDS

This repo holds a actix-web site using welds that can be used for performance testing

## steps to use:

1) boot up the databases using `docker compose up -d`
2) edit `./src/main.rs` and select the DB you would like to test
3) boot the server in release mode `cargo run --release`
4) run `ab -n 2000000 -c 800 -k 'http://127.0.0.1:3000/ab' > test_results.log`
5) run `ab -n 2000000 -c 800 -k 'http://127.0.0.1:3000/ab2' > test_results2.log`


NOTE: ab2 does NOT use the ORM. It is calling the underlying DB directly


