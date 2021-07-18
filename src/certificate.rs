/*
 */
use crate::crypto;
use crate::Error;
use clap::ArgMatches;
use colored::*;
use std::fs;
use std::io;
use std::io::Write;
use std::ops::Div;
use std::path::PathBuf;
use uuid::Uuid;

pub struct Config {
    certificate_path: PathBuf,
    crypto_config: crypto::Config,
    is_ca: bool,
    no_input: bool,
    output_directory: PathBuf,
    private_key_path: PathBuf,
}

pub fn run(clap_matches: &ArgMatches, is_ca: bool) -> Result<(), Error> {
    match create_config(clap_matches, is_ca) {
        Ok(config) => {
            let certificate = match &config.is_ca {
                true => crypto::create_ca_certificate(&config.crypto_config),
                false => crypto::create_certificate(&config.crypto_config),
            };

            match certificate {
                Ok((certificate_bytes, private_key_bytes)) => {
                    fs::write(&config.certificate_path, certificate_bytes)?;
                    fs::write(&config.private_key_path, private_key_bytes)?;

                    if config.no_input {
                        println!(
                            "{} {}",
                            "created".green(),
                            config.private_key_path.display()
                        );
                        println!(
                            "{} {}",
                            "created".green(),
                            config.certificate_path.display()
                        );
                    }

                    Ok(())
                }
                Err(err) => Err(Error::Crypto(err)),
            }
        }
        Err(Error::CreateAborted) => Ok(()),
        Err(err) => Err(err),
    }
}

fn create_config(clap_matches: &ArgMatches, is_ca: bool) -> Result<Config, Error> {
    let uuid = &Uuid::new_v4().to_hyphenated().to_string()[..];

    let common_name = clap_matches
        .value_of("common-name")
        .or(Some(uuid))
        .unwrap()
        .to_string();

    let days_valid: u32 = clap_matches.value_of_t("days-valid").unwrap();

    let output_directory = PathBuf::from(
        clap_matches
            .value_of("output-directory")
            .unwrap()
            .to_string(),
    )
    .canonicalize()?;

    let output_format = resolve_output_format(clap_matches.value_of("output-format"));

    let prefix = match is_ca {
        true => "-ca",
        false => "",
    };

    let cert_filename = format!(
        "{}{}-certificate.{}",
        common_name,
        prefix,
        output_format.to_string().to_lowercase()
    );

    let key_filename = format!(
        "{}{}-private-key.{}",
        common_name,
        prefix,
        output_format.to_string().to_lowercase()
    );

    let certificate_path = output_directory.join(cert_filename);
    let private_key_path = output_directory.join(key_filename);

    let signer_private_key_path: Option<PathBuf>;
    let signer_certificate_path: Option<PathBuf>;

    match clap_matches.value_of("signer-private-key-path") {
        Some(signer_private_key_path_str) => {
            signer_private_key_path =
                Some(PathBuf::from(signer_private_key_path_str).canonicalize()?);
            signer_certificate_path = Some(
                PathBuf::from(clap_matches.value_of("signer-certificate-path").unwrap())
                    .canonicalize()?,
            );
        }
        None => {
            signer_private_key_path = None;
            signer_certificate_path = None;
        }
    };

    let self_signed = signer_certificate_path.is_none() || signer_private_key_path.is_none();

    let config = Config {
        certificate_path,
        crypto_config: crypto::Config {
            self_signed,
            signer_certificate_path,
            signer_private_key_path,
            common_name,
            key_type: resolve_key_type(clap_matches.value_of("key-type")),
            days_valid,
            output_format,
        },
        is_ca,
        no_input: clap_matches.is_present("no-input"),
        output_directory,
        private_key_path,
    };

    if !config.no_input {
        verify_config(&config)?;
    }

    Ok(config)
}

fn verify_config(config: &Config) -> Result<(), Error> {
    let output_directory = config.output_directory.as_path().to_str().unwrap().cyan();

    print!(
        "\
A CA certificate and private key will be created using the following configuration.

key type: {}
common name: {}
days valid: {} (which is {} years)
self-signed: {}
signer certificate path: {}
signer private key path: {}
output directory: {}

{} Double check \"days valid\" above. Inconsiderate values can have devastating consequences.
",
        config
            .crypto_config
            .key_type
            .to_string()
            .to_lowercase()
            .cyan(),
        config.crypto_config.common_name.cyan(),
        config.crypto_config.days_valid.to_string().cyan(),
        (config.crypto_config.days_valid as f32)
            .div(365.0)
            .to_string()
            .cyan(),
        config.crypto_config.self_signed.to_string().cyan(),
        config
            .crypto_config
            .signer_certificate_path
            .as_ref()
            .unwrap_or(&PathBuf::from("not applicable"))
            .display()
            .to_string()
            .cyan(),
        config
            .crypto_config
            .signer_private_key_path
            .as_ref()
            .unwrap_or(&PathBuf::from("not applicable"))
            .display()
            .to_string()
            .cyan(),
        output_directory,
        "WARNING".yellow(),
    );

    if config.crypto_config.self_signed {
        println!(
            "{} You are creating a self-signed certificate.",
            "WARNING".yellow()
        );
    }

    println!();
    println!("{} {}", "create".green(), config.private_key_path.display());
    println!("{} {}", "create".green(), config.certificate_path.display());
    println!();
    print!("{} ", "execute (Y/n):".magenta());
    let mut guess = String::new();
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    if ["", "y", "Y"].contains(&guess.trim()) {
        Ok(())
    } else {
        Err(Error::CreateAborted)
    }
}

fn resolve_output_format(raw_output_format: Option<&str>) -> crypto::FileFormat {
    match raw_output_format {
        Some("der") => crypto::FileFormat::Der,
        Some("pem") => crypto::FileFormat::Pem,
        _ => panic!("invalid raw file format"),
    }
}

fn resolve_key_type(result: Option<&str>) -> crypto::KeyType {
    match result {
        Some("ec") => crypto::KeyType::Ec,
        Some("rsa") => crypto::KeyType::Rsa,
        _ => panic!("invalid raw key type"),
    }
}
