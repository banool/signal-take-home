# Signal Take Home

## Development
You should have the following available locally:
- Cargo 1.52 (rustup is the easiest way to get this).

You should also make sure to source the environment containing the GIPHY API key and other secrets:
```
export `cat .env | xargs`
```
This is not checked in, but you can look at `.env.example` to see what it should look like.

To check the build as you develop, run this:
```
cargo check
```

To run the server, try something like this:
```
RUST_LOG=debug cargo run -- --address 127.0.0.1 -p 8888 --allowed-providers api.giphy.com
```
