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

To make a valid query through the proxy, run the following:
```
curl -x localhost:8888 "https://api.giphy.com/v1/gifs/search?api_key=$giphy_api_key&q=happy&limit=1" | jq .
```

To test an invalid query, just this works (this sends a GET directly to the proxy, instead of a CONNECT through it for GIPHY):
```
curl localhost:8888
```

You can see a successful interaction here:
```
# Client makes request
curl -x localhost:8888 "https://api.giphy.com/v1/gifs/search?api_key=$giphy_api_key&q=happy&limit=2" | jq .

# Server logs
[2021-05-20T21:27:57Z INFO  signal_take_home] Listening on 127.0.0.1:8888
[2021-05-20T21:27:58Z DEBUG signal_take_home] Client wrote 954 bytes and received 21946 bytes

# Client response is too long but it is valid json from GIPHY
```

Here you can see the server performing correct validation on where the client is trying to connect:
```
# Client makes request to invalid URL
$ curl -x localhost:8888 "https://google.com/"
curl: (56) Received HTTP code 400 from proxy after CONNECT

# Server logs
[2021-05-20T21:33:35Z WARN  signal_take_home] Rejected request for google.com which is not in the list of allowed providers
```
