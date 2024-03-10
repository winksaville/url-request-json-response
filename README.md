# URL request with json response

This is a simple example of how to make a URL request and parse the JSON response.
I prompted both Gemini and ChatGPT-4 the following:

"Write a Rust app that takes a URL plus requests via the header
that json is to be returned. Optionally a "-d" or "--data" to
supply a body. The a "get" request is made and on success the
returned data is printed as json"

Gemini was a little simpler and I tried it first, but it was actually async
but hadn't imported/used an async runtime. So I decided to use the ChatGPT-4 version
which was intentailly async and uses tokio runtime.

## Examples

### Request to beaconcha.in to get the "performance" (rewards) of two validators

```sh
wink@3900x 24-03-10T18:45:46.381Z:~/prgs/rust/myrepos/url-request-json-response (main)
$ cargo run https://beaconcha.in/api/v1/validator/195766,198275/execution/performance -v
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/url-request-json-response 'https://beaconcha.in/api/v1/validator/195766,198275/execution/performance' -v`
response headers: {
  date: "Sun, 10 Mar 2024 18:45:47 GMT"
  content-type: "application/json"
  transfer-encoding: "chunked"
  connection: "keep-alive"
  access-control-allow-headers: "*, Authorization"
  access-control-allow-methods: "*"
  access-control-allow-origin: "*"
  ratelimit-limit: "120000"
  ratelimit-remaining: "119800"
  ratelimit-reset: "1833253"
  ratelimit-window: "month"
  vary: "Cookie"
  vary: "Accept-Encoding"
  x-ratelimit-limit-day: "120000"
  x-ratelimit-limit-hour: "120000"
  x-ratelimit-limit-minute: "120000"
  x-ratelimit-limit-month: "120000"
  x-ratelimit-limit-second: "5"
  x-ratelimit-remaining-day: "119800"
  x-ratelimit-remaining-hour: "119800"
  x-ratelimit-remaining-minute: "119800"
  x-ratelimit-remaining-month: "119800"
  x-ratelimit-remaining-second: "4"
  via: "1.1 google"
  last-modified: "Sun, 10 Mar 2024 18:44:09 GMT"
  cache-control: "max-age=12"
  cf-cache-status: "HIT"
  age: "1"
  permissions-policy: "interest-cohort=()"
  referrer-policy: "strict-origin-when-cross-origin"
  strict-transport-security: "max-age=7257600"
  x-content-type-options: "nosniff"
  x-frame-options: "DENY"
  x-xss-protection: "1; mode=block"
  server: "cloudflare"
  cf-ray: "862576ba587e67a8-SJC"
  alt-svc: "h3=\":443\"; ma=86400"
}
{
  "data": [
    {
      "performance1d": 0,
      "performance31d": 0,
      "performance365d": 103323191045454191,
      "performance7d": 0,
      "performanceTotal": 310027217653695454,
      "validatorindex": 195766
    },
    {
      "performance1d": 0,
      "performance31d": 34512074457665825,
      "performance365d": 187531625415343172,
      "performance7d": 0,
      "performanceTotal": 250318784306458495,
      "validatorindex": 198275
    }
  ],
  "status": "OK"
}
wink@3900x 24-03-10T18:45:47.278Z:~/prgs/rust/myrepos/url-request-json-response (main)
$ cargo run https://beaconcha.in/api/v1/validator/195766,198275/execution/performance -v
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/url-request-json-response 'https://beaconcha.in/api/v1/validator/195766,198275/execution/performance' -v`
response headers: {
  date: "Sun, 10 Mar 2024 18:46:37 GMT"
  content-type: "application/json"
  transfer-encoding: "chunked"
  connection: "keep-alive"
  access-control-allow-headers: "*, Authorization"
  access-control-allow-methods: "*"
  access-control-allow-origin: "*"
  ratelimit-limit: "120000"
  ratelimit-remaining: "119799"
  ratelimit-reset: "1833203"
  ratelimit-window: "month"
  vary: "Cookie"
  vary: "Accept-Encoding"
  x-ratelimit-limit-day: "120000"
  x-ratelimit-limit-hour: "120000"
  x-ratelimit-limit-minute: "120000"
  x-ratelimit-limit-month: "120000"
  x-ratelimit-limit-second: "5"
  x-ratelimit-remaining-day: "119799"
  x-ratelimit-remaining-hour: "119799"
  x-ratelimit-remaining-minute: "119799"
  x-ratelimit-remaining-month: "119799"
  x-ratelimit-remaining-second: "4"
  via: "1.1 google"
  last-modified: "Sun, 10 Mar 2024 18:45:46 GMT"
  cache-control: "max-age=12"
  cf-cache-status: "EXPIRED"
  permissions-policy: "interest-cohort=()"
  referrer-policy: "strict-origin-when-cross-origin"
  strict-transport-security: "max-age=7257600"
  x-content-type-options: "nosniff"
  x-frame-options: "DENY"
  x-xss-protection: "1; mode=block"
  server: "cloudflare"
  cf-ray: "862577ed5fce236e-SJC"
  alt-svc: "h3=\":443\"; ma=86400"
}
{
  "data": [
    {
      "performance1d": 0,
      "performance31d": 34512074457665825,
      "performance365d": 187531625415343172,
      "performance7d": 0,
      "performanceTotal": 250318784306458495,
      "validatorindex": 198275
    },
    {
      "performance1d": 0,
      "performance31d": 0,
      "performance365d": 103323191045454191,
      "performance7d": 0,
      "performanceTotal": 310027217653695454,
      "validatorindex": 195766
    }
  ],
  "status": "OK"
}
wink@3900x 24-03-10T18:46:37.147Z:~/prgs/rust/myrepos/url-request-json-response (main)
```

### Determine the epoch of the current ethereum finalized data

Answer was `"epoch": "268940"`

```sh
wink@3900x 24-03-10T19:14:50.775Z:~/prgs/rust/myrepos/url-request-json-response (main)
$ cargo run http://hazel:5052/eth/v1/beacon/states/head/finality_checkpoints -v
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/url-request-json-response 'http://hazel:5052/eth/v1/beacon/states/head/finality_checkpoints' -v`
response headers: {
  content-type: "application/json"
  server: "Lighthouse/v5.0.0-b5bae6e/x86_64-linux"
  content-length: "395"
  date: "Sun, 10 Mar 2024 19:15:01 GMT"
}
{
  "data": {
    "current_justified": {
      "epoch": "268941",
      "root": "0x19731621d7b855b74f5a74b435b0ad9d1aa5cfcd799cd0e76583779586f38bc5"
    },
    "finalized": {
      "epoch": "268940",
      "root": "0x619cfe1164d1637cfd604c63e2fb4c0b62891f2b55dd5b777e3cc5819fef1cc6"
    },
    "previous_justified": {
      "epoch": "268940",
      "root": "0x619cfe1164d1637cfd604c63e2fb4c0b62891f2b55dd5b777e3cc5819fef1cc6"
    }
  },
  "execution_optimistic": false,
  "finalized": false
}
wink@3900x 24-03-10T19:15:01.830Z:~/prgs/rust/myrepos/url-request-json-response (main)
```

### Determine the rewards of two validators for a specific epoch

In this case we're going to use the most "current" finalized epoch from above: `268940`
and two validators `195766` and `198275`:

```sh
wink@3900x 24-03-10T19:15:01.830Z:~/prgs/rust/myrepos/url-request-json-response (main)
$ cargo run http://hazel:5052/eth/v1/beacon/rewards/attestations/268940 -d '[  "195766","198275" ]'
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/url-request-json-response 'http://hazel:5052/eth/v1/beacon/rewards/attestations/268940' -d '[  "195766","198275" ]'
{
  "data": {
    "ideal_rewards": [
      {
        "effective_balance": "1000000000",
        "head": "78",
        "inactivity": "0",
        "source": "78",
        "target": "146"
      },
      {
        "effective_balance": "2000000000",
        "head": "156",
        "inactivity": "0",
        "source": "157",
        "target": "292"
      },

 ...

      {
        "effective_balance": "31000000000",
        "head": "2432",
        "inactivity": "0",
        "source": "2440",
        "target": "4528"
      },
      {
        "effective_balance": "32000000000",
        "head": "2511",
        "inactivity": "0",
        "source": "2518",
        "target": "4675"
      }
    ],
    "total_rewards": [
      {
        "head": "2511",
        "inactivity": "0",
        "source": "2518",
        "target": "4675",
        "validator_index": "195766"
      },
      {
        "head": "2511",
        "inactivity": "0",
        "source": "2518",
        "target": "4675",
        "validator_index": "198275"
      }
    ]
  },
  "execution_optimistic": false
}
wink@3900x 24-03-10T19:15:26.836Z:~/prgs/rust/myrepos/url-request-json-response (main)
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
