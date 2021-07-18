/*
This file defines the CLI via Clap and accepts the command line args, parses them with Clap, then
executes the relevant command.
*/
pub mod certificate;
pub mod command;
pub mod crypto;

use clap::{clap_app, crate_version, App};
use colored::*;
use command::create_ca_certificate;
use command::create_certificate;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    CreateAborted,
    CryptoFailed,
    StdError(std::io::Error),
    Crypto(openssl::error::ErrorStack),
}

impl From<std::io::Error> for Error {
    fn from(item: std::io::Error) -> Self {
        Error::StdError(item)
    }
}

impl From<openssl::error::ErrorStack> for Error {
    fn from(item: openssl::error::ErrorStack) -> Self {
        Error::Crypto(item)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Crypto(err) => err.fmt(f),
            Error::StdError(err) => err.fmt(f),
            _ => Ok(()),
        }
    }
}

fn main() -> Result<(), Error> {
    run_cli()
}

fn run_cli() -> Result<(), Error> {
    let app = get_app();
    let clap_matches = app.get_matches();

    let cli_return = match clap_matches.subcommand() {
        Some(("create-ca-certificate", subcommand_matches)) => {
            create_ca_certificate::run(subcommand_matches)
        }
        Some(("create-certificate", subcommand_matches)) => {
            create_certificate::run(subcommand_matches)
        }
        _ => print_help(),
    };

    match &cli_return {
        Ok(_) => (),
        Err(Error::StdError(error)) => {
            eprintln!("{}", format!("{}", error).red());
        }
        Err(error) => {
            eprintln!("{}", format!("{}", error).red());
        }
    };

    cli_return
}
fn print_help() -> Result<(), Error> {
    match get_app().print_help() {
        Ok(()) => Ok(()),
        Err(err) => Err(Error::StdError(err)),
    }
}

fn get_app() -> App<'static> {
    clap_app!("TLS Credential Helper" =>
        (bin_name: "tch")
        (version: crate_version!())
        (author: "Very")
        (about: "Creation of key pairs and certificates for use with TLS.")
        (@subcommand "create-ca-certificate" =>
            (about: "Create a CA certificate and a key pair.")
            (@arg ("output-format"): --("output-format") +takes_value default_value[pem] possible_value[der pem] "Sets the output file format.")
            (@arg ("key-type"): --("key-type") +takes_value default_value[ec] possible_value[ec rsa] "Sets the type of the created keys.")
            (@arg ("days-valid"): --("days-valid") +takes_value +required "How may days from today the created certificate will be valid for.")
            (@arg ("output-directory"): --("output-directory") +takes_value default_value["."] "Sets the output directory.")
            (@arg ("no-input"): --("no-input") "Runs the CLI in no-input mode.")
            (@group namee =>
                (@attributes +required)
                (@arg ("common-name"): --("common-name") +takes_value "Sets the created certificate's common name to the provided value.")
                (@arg ("random-common-name"): --("random-common-name") "Sets the created certificate's common name to a generated version 4 UUID.")
            )
            (@arg ("self-signed"): --("self-signed") conflicts_with_all(&["signer-certificate-path", "signer-private-key-path"]) "Sign the created certificate with the created private key pair as opposed to with an existing signer provided via --signer-certificate-path and --signer-private-key-path.")
            (@arg ("signer-certificate-path"): --("signer-certificate-path") required_unless_present("self-signed") +takes_value "A path to an existing pem or der encoded signer certificate to use.")
            (@arg ("signer-private-key-path"): --("signer-private-key-path") required_unless_present("self-signed") +takes_value "A path to an existing pem or der encoded signer private key to use.")
        )
        (@subcommand "create-certificate" =>
            (about: "Create a certificate and a key pair.")
            (@arg ("output-format"): --("output-format") +takes_value default_value[pem] possible_value[der pem] "Sets the output file format.")
            (@arg ("key-type"): --("key-type") +takes_value default_value[ec] possible_value[ec rsa] "Sets the type of the created keys.")
            (@arg ("days-valid"): --("days-valid") +takes_value +required "How may days from today the created certificate will be valid for.")
            (@arg ("output-directory"): --("output-directory") +takes_value default_value["."] "Sets the output directory.")
            (@arg ("no-input"): --("no-input") "Runs the CLI in no-input mode.")
            (@group name =>
                (@attributes +required)
                (@arg ("common-name"): --("common-name") +takes_value "Sets the created certificate's common name to the provided value.")
                (@arg ("random-common-name"): --("random-common-name") "Sets the created certificate's common name to a generated version 4 UUID.")
            )
            (@arg ("self-signed"): --("self-signed") conflicts_with_all(&["signer-certificate-path", "signer-private-key-path"]) "Sign the created certificate with the created private key pair as opposed to with an existing signer provided via --signer-certificate-path and --signer-private-key-path.")
            (@arg ("signer-certificate-path"): --("signer-certificate-path") required_unless_present("self-signed") +takes_value "A path to an existing pem or der encoded signer certificate to use.")
            (@arg ("signer-private-key-path"): --("signer-private-key-path") required_unless_present("self-signed") +takes_value "A path to an existing pem or der encoded signer private key to use.")
        )
    )
}
