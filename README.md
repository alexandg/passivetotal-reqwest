`passivetotal-reqwest`
-------------------------

A [`reqwest`](https://github.com/seanmonstar/reqwest)-based Rust library and 
app for querying the [PassiveTotal](https://www.passivetotal.org)
[API](https://api.passivetotal.org/api/docs/).

Requires Rust v1.19+.

### Installation

To use this crate as a library add it to your `Cargo.toml`:

```toml
[dependencies]
passivetotal-reqwest = { git = "https://github.com/alexandg/passivetotal-reqwest" }
```

Then include it in your Rust code:

```rust
extern crate passivetotal_reqwest;

use passivetotal_reqwest::PassiveTotal;
```

### Configuration

This crate requires a valid PassiveTotal API username and key. These can be
provided to the command line app by storing them in a toml configuration file
with the following format:

```toml
[passivetotal]
username = "USERNAME"
apikey = "API_KEY"
timeout = 60
```

This file can either be provided on the command line with the `--config` flag
or placed in `$HOME/.passivetotal.toml`.

### Usage

Access to the API is provided through the `PassiveTotal` struct. For example:

```rust
extern crate passivetotal_reqwest;

use passivetotal_reqwest::PassiveTotal;

let pt = PassiveTotal::new("USERNAME", "APIKEY", Duration::from_secs(30));
let resp = pt.passive_dns("www.passivetotal.org").send().unwrap();
println!("{}", resp);
```

### License

`passivetotal-reqwest` is licensed under the MIT License.
