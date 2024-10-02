# Phishing website example exercise
A phishing website demo using basic HTML, CSS, JS and a Rust web server.

## Usage
```
$ phishweb --help
Usage: phishweb [OPTIONS]

Options:
      --ip <IP>         IP address to bind to
      --port <PORT>     Port to bind to
      --webroot <PATH>  Path to webroot directory
      --output <PATH>   File to store captured credentials in
  -h, --help            Print help
  -V, --version         Print version
```

## Running
Download the precompiled binary from here, or compile it yourself using `cargo`. For instructions on how to install the Rust toolchain see [this](rustlang.org).

You will also need a basic website that will call `/api/login` on a form submit. The form must be sent as `application/x-www-form-urlencoded` and **must** contain a `username` and `password` field. Optionally, you can also add an `email` field. A pre-built website is included in this repo ([`www`](www/)). You must place this folder in the same directory as the `phishweb` binary, otherwise you must specify the path to this directory using the `--webroot` switch.

Credentials are stored in `capture.csv`, which is automatically created in the same directory from which you run the `phishweb` binary. To specify an alternative location, use the `--output` switch.

By default the port is 8080 and the server is bound to all IPs.

## Allowing access from WAN
1. Use [NGROK](https://ngrok.com/) (reverse proxy)
    - Start with `ngrok http 8080`
2. Use [Cloudflare Tunnel (`cloudflared`)](https://developers.cloudflare.com/cloudflare-one/connections/connect-networks/)
3. Manual port forwarding.