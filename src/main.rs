use log::*;
use structopt::StructOpt;
use url::Url;
use urlencoding::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "url", author = "Sebastien Q. <sebastien.quioc@protonmail.com>", version = "0.1", about = "A tool to manipulate uris")]
struct Opt {
    /// Silence all output
    #[structopt(short = "q", long = "quiet")]
    quiet: bool,
    /// Verbose mode (-v, -vv, -vvv, etc)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: usize,
    /// Timestamp (sec, ms, ns, none)
    #[structopt(subcommand)]
    cmd: Command
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(about="Decodes a percent-encoding url")]
    Decode {
        // The uri to decode
        encoded_uri: String
    },

    #[structopt(about="Splits and prints parts of the url")]
    Dissect {
        // The uri to dissect
        uri: String
    },

    #[structopt(about="Encodes an url with percent-encoding")]
    Encode {
        // The uri to encode
        uri: String
    },

    #[structopt(about="Joins an url with a path")]
    Join {
        // The base of the uri
        uri: String,
        // The complement to the base
        complement: String
    },

    #[structopt(name = "idna-decode", about="Decodes a translated internationalized domain name")]
    IdnaDecode {
        // The uri to decode
        encoded_uri: String
    },

    #[structopt(name = "idna-encode", about="Encodes an internationalized domain name with the punycode translation")]
    IdnaEncode {
        // The uri to encode
        uri: String
    },
}


fn main() {
    let opt = Opt::from_args();

    stderrlog::new()
        .module(module_path!())
        .quiet(opt.quiet)
        .verbosity(opt.verbose)
        .init()
        .unwrap();
    
    match opt.cmd {
        Command::Decode { encoded_uri } => {
            match decode(&encoded_uri) {
                Ok(uri) => {
                    println!("{}", uri)
                }
                Err(error) => {
                    error!("Unable to decode the uri: '{}' reason: '{}'", encoded_uri, error)
                }
            }
        }
        Command::Dissect { uri } => {
            let url = Url::parse(uri.as_str());

            match url {
                Ok(uri) => {
                    println!("scheme: {}", uri.scheme());
                    println!("username: {}", uri.username());
                    println!("password: {}", uri.password().unwrap_or(""));
                    println!("host: {}", uri.host_str().unwrap_or(""));
                    match uri.port_or_known_default() {
                        Some(port) => { println!("port: {}", port)}
                        None => { println!("port: ")}
                    }
                    println!("path: {}", uri.path());
                    println!("query: {}", uri.query().unwrap_or(""));
                    println!("fragment: {}", uri.fragment().unwrap_or(""))
                }
                Err(error) => {
                    error!("Unable to parse the uri: '{}' reason: '{}'", uri, error)
                }
            }
        }
        Command::Encode { uri } => {
            println!("{}", encode(&uri))
        }
        Command::Join { uri, complement } => {
            let url = Url::parse(uri.as_str());

            match url {
                Ok(base) => {
                    let joined = base.join(complement.as_str());
                    println!("{}", joined.unwrap().as_str())
                }
                Err(error) => {
                    error!("Unable to parse the uri: '{}' reason: '{}'", uri, error)
                }
            }
        }
        Command::IdnaDecode { encoded_uri } => {
            let url = Url::parse(encoded_uri.as_str());

            match url {
                Ok(mut uri) => {
                    match uri.domain() {
                        Some(domain) => {
                            let mut domain_str = String::from(domain);
                            domain_str.push('-');
                            match idna::punycode::decode_to_string(domain_str.as_str()) {
                                Some(decoded) => {
                                    match uri.set_host(Some(decoded.as_str())) {
                                        Ok(()) => {
                                            println!("{}", uri.as_str())
                                        }
                                        Err(error) => {
                                            error!("Unable to update the uri: '{}' reason: '{}'", uri, error)
                                        }
                                    }
                                }
                                _ => ()
                            }
                        }
                        _ => ()
                    }
                }
                Err(error) => {
                    error!("Unable to parse the uri: '{}' reason: '{}'", encoded_uri, error)
                }
            }
        }
        Command::IdnaEncode { uri } => {
            let url = Url::parse(uri.as_str());

            match url {
                Ok(mut uri) => {
                    match uri.domain() {
                        Some(domain) => {
                            match idna::punycode::encode_str(domain) {
                                Some(encoded) => {
                                    match uri.set_host(Some(encoded.trim_end_matches("-"))) {
                                        Ok(()) => {
                                            println!("{}", uri.as_str())
                                        }
                                        Err(error) => {
                                            error!("Unable to update the uri: '{}' reason: '{}'", uri, error)
                                        }
                                    }
                                }
                                _ => ()
                            }
                        }
                        _ => ()
                    }
                }
                Err(error) => {
                    error!("Unable to parse the uri: '{}' reason: '{}'", uri, error)
                }
            }
        }
    }
}
