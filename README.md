# URL request with json response

This is a simple example of how to make a URL request and parse the JSON response.
I prompted both Gemini and ChatGPT-4 the following:

"Write a Rust app that takes a URL plus requests via the header
that json is to be returned. Optionally a "-d" or "--data" to
supply a body. The a "get" request is made and on success the
returned data is printed as json"

Gemini was a little simpler and I tried it at first but it was async and
but hadn't include a async runtime. So I decided to use the ChatGPT-4 version
which uses tokio.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
