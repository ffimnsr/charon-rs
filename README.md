# Haya (Doorkeeper)

> Hold my hand in yours, and we will not fear what hands like ours can do.
> - The Epic Of Gilgamesh

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Generating EC Private And Public Keys

In order to generate EC private key:

```bash
openssl ecparam -genkey -noout -name prime256v1 |
    openssl pkcs8 -topk8 -nocrypt -out ./certs/priv-key.pem
```

To generate the counterpart public key:

```
openssl ec -in ./certs/priv-key.pem -pubout -out ./certs/pub.pem
```

### Endpoints

- GET /settings
- POST, PUT /admin/users/<user_id>
- POST /admin/generate_link
- POST /signup
- POST /resend
- POST /invite
- GET, POST /verify
- POST /otp
- POST /recover
- POST /token
- GET, PUT /user
- GET /reauthenticate
- POST /logout
- GET /authorize
- GET /callback

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
