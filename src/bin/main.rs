//! Simple command line client using the `passivetotal-reqwest` crate to
//! query the `PassiveTotal` v2 API via the `reqwest` rust crate.
//!
//! Access to the API is via the `PassiveTotal` struct. This requires a valid
//! `PassiveTotal` username and api key. These can either be provided via a
//! configuration file. This is a toml file with the following format:
//!
//! ```
//! [passivetotal]
//! username = "USERNAME"
//! apikey = "SECRET_API_KEY"
//! timeout = 60
//! ```
//!
//! The username and apikey fields are required, while the other fields are
//! optional. This file can either be passed as a command line argument or
//! created as `$HOME/.passivetotal.toml`.
//!
//! # License
//!
//! `passivetotal-reqwest` is licensed under the MIT License. See LICENSE-MIT.
#![recursion_limit = "1024"]
#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json as json;
extern crate toml;

extern crate passivetotal_reqwest as passivetotal;

use std::io::{Read, Write};
use std::env;
use std::fs::File;
use std::time::Duration;
use std::path::{Path, PathBuf};

use clap::ArgMatches;
use passivetotal::PassiveTotal;

const ABOUT: &'static str = "Simple CLI for passivetotal-reqwest library.";

mod errors {
    use std::io;
    use json;
    use passivetotal;

    error_chain! {
        foreign_links {
            Pt(passivetotal::PassiveTotalError);
            Io(io::Error);
            Json(json::Error);
        }
    }
}

use errors::*;

#[derive(Debug, Deserialize)]
struct Config {
    passivetotal: PassiveTotalConfig,
}

#[derive(Debug, Deserialize)]
struct PassiveTotalConfig {
    username: String,
    apikey: String,
    timeout: Option<u64>,
}

fn parse_args() -> ArgMatches<'static> {
    clap_app!(passivetotal =>
        (version: crate_version!())
        (about: ABOUT)
        (@arg CONFIG: -c --config +takes_value "Choose a specific config file. \
                                                Default $HOME/.passivetotal.toml.")
        (@arg TIMEOUT: -t --timeout +takes_value "Timeout for all requests.")
        (@arg OUTPUT: -o --output +takes_value "File to write output to.")
        (@arg PRETTY: -p --pretty "Pretty print JSON results.")
        (@subcommand pdns =>
         (about: "Retrieve the passive DNS results from active sources.")
         (@arg UNIQUE: --unique "Query for unique passive dns results.")
         (@arg QUERY: +required "DNS Query to make."))
        (@subcommand whois =>
         (about: "Retrieve or search WHOIS data for a given query.")
         (@subcommand data =>
          (about: "Retrieve the WHOIS data for given query.")
          (@arg QUERY: +required "Domain to query."))
         (@subcommand search =>
          (about: "Search WHOIS data for a keyword.")
          (@arg FIELD: --field +takes_value "The field to query. [Email, Domain, Name, \
                                             Organization, Address, Phone, Nameserver]")
          (@arg QUERY: +required "Keyword to search for.")))
        (@subcommand ssl =>
         (about: "Retrieve information about an SSL certificate.")
         (@subcommand certificate =>
          (about: "Retrieve an SSL certificate by SHA1 hash")
          (@arg QUERY: +required "SHA1 hash of certificate."))
         (@subcommand search =>
          (about: "Retrieves SSL certificates for a given search.")
          (@arg FIELD: --field +takes_value "The field to query. See the Passivetotal \
                                             API for a full list of fields.")
          (@arg QUERY: +required "Keyword to search for."))
         (@subcommand history =>
          (about: "Retrieve the SSL certificate history of a given SHA1 or IP address.")
          (@arg QUERY: +required "SHA1 or IP address to retrieve certificate history for.")))
        (@subcommand enrichment =>
         (about: "Get additional enrichment information about a query.")
         (@subcommand data =>
          (about: "Get enrichment data for a query.")
          (@arg QUERY: +required "Domain or IP to query."))
         (@subcommand malware =>
          (about: "Get malware data for a query.")
          (@arg QUERY: +required "Domain or IP to query."))
         (@subcommand osint =>
          (about: "Get osint data for a query.")
          (@arg QUERY: +required "Domain or IP to query."))
         (@subcommand subdomains =>
          (about: "Get subdomains data for a query.")
          (@arg QUERY: +required "Domain or IP to query.")))
        (@subcommand actions =>
         (about: "Retrieve action status information for given query.")
         (@subcommand classification =>
          (about: "Retrieve classification status for a given domain.")
          (@arg QUERY: +required "Domain for which to retrieve classification status."))
         (@subcommand compromised =>
          (about: "Indicates whether or not a given domain has ever been compromised.")
          (@arg QUERY: +required "Domain for which to retrieve classification status."))
         (@subcommand ddns =>
          (about: "Indicates whether or not a domain's DNS records are updated via dynamic DNS.")
          (@arg QUERY: +required "Domain for which to retrieve dynamic DNS status."))
         (@subcommand monitor =>
          (about: "Indicates whether or not a domain is monitored.")
          (@arg QUERY: +required "Domain for which to check for monitoring."))
         (@subcommand sinkhole =>
          (about: "Indicates whether or not an IP address is a sinkhole.")
          (@arg QUERY: +required "IP address to check for sinkhole status"))
         (@subcommand tags =>
          (about: "Retrieves tags for a given artifact.")
          (@arg QUERY: +required "Artifact for which to retrieve tags")))
    ).get_matches()
}

fn default_config() -> Option<PathBuf> {
    env::home_dir().map(|mut home| {
        home.push(".passivetotal.toml");
        home
    })
}

fn load_config<P: AsRef<Path>>(config: P) -> Result<Config> {
    let mut s = String::new();
    File::open(config)
        .and_then(|mut f| f.read_to_string(&mut s))
        .chain_err(|| "Failed to open config file.")?;

    toml::from_str(&s).chain_err(|| "Unable to parse configuration file.")
}

fn config(args: &ArgMatches) -> Result<Config> {
    args.value_of("CONFIG")
        .map(PathBuf::from)
        .or_else(default_config)
        .ok_or_else(|| "Unable to find valid configuration filepath!".into())
        .and_then(load_config)
}

fn handle_ssl_command(pt: &PassiveTotal, cmd: &ArgMatches) -> Result<json::Value> {
    match cmd.subcommand() {
        ("certificate", Some(c)) => {
            let query = c.value_of("QUERY").unwrap();
            pt.sslcert(query).chain_err(
                || "Failed running 'ssl certificate' command!",
            )
        },
        ("history", Some(c)) => {
            let query = c.value_of("QUERY").unwrap();
            pt.sslcert_history(query).chain_err(
                || "Failed running 'ssl history' command!",
            )
        },
        ("search", Some(c)) => {
            let query = c.value_of("QUERY").unwrap();
            if let Some(f) = c.value_of("FIELD") {
                match f.parse() {
                    Ok(field) => {
                        pt.sslcert_search_by_field(query, field).chain_err(
                            || "Failed to run `ssl search` command!",
                        )
                    },
                    Err(err) => {
                        Err(err).chain_err(|| "Failed to create valid field from given argument.")
                    },
                }
            } else {
                pt.sslcert_search(query).chain_err(
                    || "Failed to run `ssl search` command!",
                )
            }
        },
        _ => Err("No valid subcommand provided to `ssl` command!".into()),
    }
}

fn handle_enrichment_command(pt: &PassiveTotal, cmd: &ArgMatches) -> Result<json::Value> {
    match cmd.subcommand() {
        ("data", Some(c)) => {
            let query = c.value_of("QUERY").unwrap();
            pt.enrichment_data(query).chain_err(
                || "Failed running `enrichment data` command!",
            )
        },
        ("malware", Some(c)) => {
            let query = c.value_of("QUERY").unwrap();
            pt.malware(query).chain_err(
                || "Failed running `enrichment malware` command!",
            )
        },
        ("osint", Some(c)) => {
            let query = c.value_of("QUERY").unwrap();
            pt.osint(query).chain_err(
                || "Failed running `enrichment osint` command!",
            )
        },
        ("subdomains", Some(c)) => {
            let query = c.value_of("QUERY").unwrap();
            pt.subdomains(query).chain_err(
                || "Failed running `enrichment subdomains` command!",
            )
        },
        _ => Err(
            "No valid subcommand provided to `enrichment` command!".into(),
        ),
    }
}

fn handle_actions_command(pt: &PassiveTotal, cmd: &ArgMatches) -> Result<json::Value> {
    match cmd.subcommand() {
        ("classification", Some(c)) => {
            let query = c.value_of("QUERY").unwrap();
            pt.classification(query).chain_err(
                || "Failed running `actions classification` command!",
            )
        },
        ("compromised", Some(c)) => {
            let query = c.value_of("QUERY").unwrap();
            pt.compromised(query).chain_err(
                || "Failed running `actions compromised` command!",
            )
        },
        ("ddns", Some(c)) => {
            let query = c.value_of("QUERY").unwrap();
            pt.ddns(query).chain_err(
                || "Failed running `actions ddns` command!",
            )
        },
        ("monitor", Some(c)) => {
            let query = c.value_of("QUERY").unwrap();
            pt.monitor(query).chain_err(
                || "Failed running `actions monitor` command!",
            )
        },
        ("sinkhole", Some(c)) => {
            let query = c.value_of("QUERY").unwrap();
            pt.sinkhole(query).chain_err(
                || "Failed running `actions sinkhole` command!",
            )
        },
        ("tags", Some(c)) => {
            let query = c.value_of("QUERY").unwrap();
            pt.tags(query).chain_err(
                || "Failed running `actions tags` command!",
            )
        },
        _ => Err("No valid subcommand provided to `actions` command!".into()),
    }
}

fn handle_whois_command(pt: &PassiveTotal, cmd: &ArgMatches) -> Result<json::Value> {
    match cmd.subcommand() {
        ("search", Some(c)) => {
            let query = c.value_of("QUERY").unwrap();
            if let Some(f) = c.value_of("FIELD") {
                match f.parse() {
                    Ok(field) => {
                        pt.whois_search(query, field).chain_err(
                            || "Failed to run `whois search` command!",
                        )
                    },
                    Err(err) => {
                        Err(err).chain_err(|| "Failed to create valid field from given argument.")
                    },
                }
            } else {
                pt.whois_search_keyword(query).chain_err(
                    || "Failed to run `whois search` command!",
                )
            }
        },
        ("data", Some(c)) => {
            let query = c.value_of("QUERY").unwrap();
            pt.whois(query).chain_err(
                || "Failed running `whois` command!",
            )
        },
        _ => Err("No valid subcommand provided to `whois` command!".into()),
    }
}

fn run(pt: &PassiveTotal, args: &ArgMatches) -> Result<()> {
    // Calling unwrap on all these checks for 'QUERY' are ok because 'QUERY' is
    // always required in each case so a value MUST exist.
    let resp = match args.subcommand() {
        ("pdns", Some(cmd)) => {
            let query = cmd.value_of("QUERY").unwrap();
            if cmd.is_present("UNIQUE") {
                pt.unique_passive_dns(query).chain_err(
                    || "Failed running `pdns --unique` command!",
                )?
            } else {
                pt.passive_dns(query).chain_err(
                    || "Failed running `pdns` command!",
                )?
            }
        },
        ("whois", Some(cmd)) => handle_whois_command(pt, cmd)?,
        ("ssl", Some(cmd)) => handle_ssl_command(pt, cmd)?,
        ("enrichment", Some(cmd)) => handle_enrichment_command(pt, cmd)?,
        ("actions", Some(cmd)) => handle_actions_command(pt, cmd)?,
        _ => return Err("No valid command provided!".into()),
    };

    let pretty = args.is_present("PRETTY");
    Ok(match args.value_of("OUTPUT") {
        Some(file) => {
            let f = File::create(file)?;
            print_response(f, &resp, pretty)?
        },
        _ => print_response(std::io::stdout(), &resp, pretty)?,
    })
}

fn print_response<W>(writer: W, resp: &json::Value, pretty_print: bool) -> Result<()>
where
    W: Write,
{
    Ok(if pretty_print {
        json::to_writer_pretty(writer, resp)?
    } else {
        json::to_writer(writer, resp)?
    })
}

fn print_errors(e: &errors::Error) {
    eprintln!("Error: {}", e);

    for e in e.iter().skip(1) {
        eprintln!("  {}", e);
    }
}

fn main() {
    let args = parse_args();

    let config = match config(&args) {
        Ok(cfg) => cfg,
        Err(ref e) => {
            print_errors(e);
            return;
        },
    };

    let timeout = value_t!(args, "TIMEOUT", u64)
        .ok()
        .or(config.passivetotal.timeout)
        .unwrap_or(60);

    let pt = PassiveTotal::new(
        config.passivetotal.username,
        config.passivetotal.apikey,
        Duration::from_secs(timeout),
    );

    if let Err(ref e) = run(&pt, &args) {
        print_errors(e);
    }
}
