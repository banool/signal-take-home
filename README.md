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

## Production
Build the image like this:
```
docker build -t signal-take-home .
```

Run it like this:
```
docker run -p 8888:8888 -it --init signal-take-home
```

## Deployment

I have deployed this for real on my local server using Github Actions on this repo to get the image to build continuously and my existing automated server management setup at [banool/server-setup](https://github.com/banool/server-setup) (ansible, systemd, etc). See [this commit](https://github.com/banool/server-setup/commit/82ba95e514de34948245016bd74c4c8bf514cc23). I ran the following command to execute the setup:

```
ansible-playbook -i hosts everything.yaml --extra-vars "@vars.json" --extra-vars='ansible_become_pass=<fakepassword>' --tags signal,nginx,https
```

This just all runs on my own little friend at home, running CentOS 8.

You can query it for real like this:
```
curl -x dport.me "https://api.giphy.com/v1/gifs/search?api_key=$giphy_api_key&q=happy&limit=2" | jq .
```

## Extension ideas

- A neato extension to work on would be the padding stuff Signal does for real as described here: https://signal.org/blog/signal-and-giphy-update/. This isn't really in the domain of this proxy service however, but more on the client side, so it is a bit out of scope for here.
- It'd be cool to try and spin up a bunch of these with Kubernetes or something similar. It'd be pretty straightforward since the service is stateless.
- It'd be good to serve the actual functionality out of a specific endpoint and return a "Sorry, you're looking in the wrong spot" page for the root endpoint.
