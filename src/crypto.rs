use std::fs;
use std::path::PathBuf;
use strum_macros::Display;

use openssl::asn1::Asn1Time;
use openssl::bn::{BigNum, MsbOption};
use openssl::ec::*;
use openssl::error::ErrorStack;
use openssl::hash::MessageDigest;
use openssl::pkey::{PKey, Private};
use openssl::rsa::Rsa;

use openssl::x509::extension::{
    AuthorityKeyIdentifier, BasicConstraints, KeyUsage, SubjectKeyIdentifier,
};
use openssl::x509::{X509NameBuilder, X509Req, X509ReqBuilder, X509};

#[derive(Display, Debug)]
pub enum FileFormat {
    Der,
    Pem,
}

#[derive(Display, Debug)]
pub enum KeyType {
    Ec,
    Rsa,
}

pub struct Config {
    pub common_name: String,
    pub days_valid: u32,
    pub key_type: KeyType,
    pub output_format: FileFormat,
    pub signer_certificate_path: Option<PathBuf>,
    pub signer_private_key_path: Option<PathBuf>,
    pub self_signed: bool,
}

/// Make a CA certificate and private key
pub fn create_ca_certificate(config: &Config) -> Result<(Vec<u8>, Vec<u8>), ErrorStack> {
    let private_key = match config.key_type {
        KeyType::Ec => {
            let ec_group = EcGroup::from_curve_name(openssl::nid::Nid::X9_62_PRIME256V1)?;
            let ec = EcKey::generate(&ec_group)?;
            PKey::from_ec_key(ec)?
        }
        KeyType::Rsa => {
            let rsa = Rsa::generate(2048)?;
            PKey::from_rsa(rsa)?
        }
    };

    let mut x509_name = X509NameBuilder::new()?;
    x509_name.append_entry_by_text("CN", &config.common_name)?;
    let x509_name = x509_name.build();
    let mut cert_builder = X509::builder()?;
    cert_builder.set_version(2)?;

    let serial_number = {
        let mut serial = BigNum::new()?;
        serial.rand(159, MsbOption::MAYBE_ZERO, false)?;
        serial.to_asn1_integer()?
    };

    cert_builder.set_serial_number(&serial_number)?;
    cert_builder.set_subject_name(&x509_name)?;
    cert_builder.set_pubkey(&private_key)?;
    let not_before = Asn1Time::days_from_now(0)?;
    cert_builder.set_not_before(&not_before)?;
    let not_after = Asn1Time::days_from_now(config.days_valid)?;
    cert_builder.set_not_after(&not_after)?;
    cert_builder.append_extension(BasicConstraints::new().critical().ca().build()?)?;

    cert_builder.append_extension(
        KeyUsage::new()
            .critical()
            .key_cert_sign()
            .crl_sign()
            .build()?,
    )?;

    let subject_key_identifier =
        SubjectKeyIdentifier::new().build(&cert_builder.x509v3_context(None, None))?;

    cert_builder.append_extension(subject_key_identifier)?;

    if config.self_signed {
        cert_builder.set_issuer_name(&x509_name)?;

        let auth_key_identifier = AuthorityKeyIdentifier::new()
            .keyid(false)
            .issuer(false)
            .build(&cert_builder.x509v3_context(None, None))?;

        cert_builder.append_extension(auth_key_identifier)?;
        cert_builder.sign(&private_key, MessageDigest::sha384())?;
    } else {
        let (signer_certificate, signer_private_key) = load_ca(config).unwrap();
        cert_builder.set_issuer_name(signer_certificate.subject_name())?;

        let auth_key_identifier = AuthorityKeyIdentifier::new()
            .keyid(false)
            .issuer(false)
            .build(&cert_builder.x509v3_context(Some(&signer_certificate), None))?;

        cert_builder.append_extension(auth_key_identifier)?;
        cert_builder.sign(&signer_private_key, MessageDigest::sha384())?;
    }

    let certificate = cert_builder.build();
    let certificate_bytes: Vec<u8>;
    let private_key_bytes: Vec<u8>;

    match config.output_format {
        FileFormat::Pem => {
            certificate_bytes = certificate.to_pem().unwrap();
            private_key_bytes = private_key.private_key_to_pem_pkcs8().unwrap();
        }
        FileFormat::Der => {
            certificate_bytes = certificate.to_der().unwrap();
            private_key_bytes = private_key.private_key_to_der().unwrap();
        }
    }

    Ok((certificate_bytes, private_key_bytes))
}

fn load_ca(config: &Config) -> Result<(X509, PKey<Private>), ErrorStack> {
    let cert_bytes: Vec<u8> = fs::read(&config.signer_certificate_path.as_ref().unwrap()).unwrap();
    let pkey_bytes: Vec<u8> = fs::read(&config.signer_private_key_path.as_ref().unwrap()).unwrap();

    let pkey: PKey<Private>;

    match config
        .signer_private_key_path
        .as_ref()
        .unwrap()
        .extension()
        .unwrap()
        .to_str()
        .unwrap()
    {
        "der" => pkey = PKey::private_key_from_der(&pkey_bytes).unwrap(),
        "pem" => pkey = PKey::private_key_from_pem(&pkey_bytes).unwrap(),
        _ => panic!("invalid signer private key file extension"),
    }

    let cert;

    match config
        .signer_certificate_path
        .as_ref()
        .unwrap()
        .extension()
        .unwrap()
        .to_str()
        .unwrap()
    {
        "der" => cert = X509::from_der(&cert_bytes).unwrap(),
        "pem" => cert = X509::from_pem(&cert_bytes).unwrap(),
        _ => panic!("invalid signer certificate file extension"),
    };

    Ok((cert, pkey))
}

// /// Make a certificate and private key signed by the given CA cert and private key
pub fn create_certificate(config: &Config) -> Result<(Vec<u8>, Vec<u8>), ErrorStack> {
    let private_key = match config.key_type {
        KeyType::Ec => {
            let ec_group = EcGroup::from_curve_name(openssl::nid::Nid::X9_62_PRIME256V1)?;
            let ec = EcKey::generate(&ec_group)?;
            PKey::from_ec_key(ec)?
        }
        KeyType::Rsa => {
            let rsa = Rsa::generate(2048)?;
            PKey::from_rsa(rsa)?
        }
    };

    let req = mk_request(config, &private_key)?;
    let mut cert_builder = X509::builder()?;
    cert_builder.set_version(2)?;
    let serial_number = {
        let mut serial = BigNum::new()?;
        serial.rand(159, MsbOption::MAYBE_ZERO, false)?;
        serial.to_asn1_integer()?
    };
    cert_builder.set_serial_number(&serial_number)?;
    cert_builder.set_subject_name(req.subject_name())?;

    cert_builder.set_pubkey(&private_key)?;
    let not_before = Asn1Time::days_from_now(0)?;
    cert_builder.set_not_before(&not_before)?;
    let not_after = Asn1Time::days_from_now(config.days_valid)?;
    cert_builder.set_not_after(&not_after)?;

    cert_builder.append_extension(BasicConstraints::new().build()?)?;

    cert_builder.append_extension(
        KeyUsage::new()
            .critical()
            .non_repudiation()
            .digital_signature()
            .key_encipherment()
            .build()?,
    )?;

    if config.self_signed {
        let issuer_subject_name = req.subject_name();
        let subject_key_identifier =
            SubjectKeyIdentifier::new().build(&cert_builder.x509v3_context(None, None))?;
        cert_builder.set_issuer_name(issuer_subject_name)?;
        cert_builder.append_extension(subject_key_identifier)?;
        let auth_key_identifier = AuthorityKeyIdentifier::new()
            .keyid(false)
            .issuer(false)
            .build(&cert_builder.x509v3_context(None, None))?;
        cert_builder.append_extension(auth_key_identifier)?;
        cert_builder.sign(&private_key, MessageDigest::sha384())?;
    } else {
        let (signer_certificate, signer_private_key) = load_ca(config).unwrap();
        let issuer_subject_name = signer_certificate.subject_name();
        let subject_key_identifier = SubjectKeyIdentifier::new()
            .build(&cert_builder.x509v3_context(Some(&signer_certificate), None))?;

        cert_builder.set_issuer_name(issuer_subject_name)?;
        let auth_key_identifier = AuthorityKeyIdentifier::new()
            .keyid(false)
            .issuer(false)
            .build(&cert_builder.x509v3_context(Some(&signer_certificate), None))?;
        cert_builder.append_extension(subject_key_identifier)?;
        cert_builder.append_extension(auth_key_identifier)?;
        cert_builder.sign(&signer_private_key, MessageDigest::sha384())?;
    }

    let certificate = cert_builder.build();

    let certificate_bytes: Vec<u8>;
    let private_key_bytes: Vec<u8>;
    match config.output_format {
        FileFormat::Pem => {
            certificate_bytes = certificate.to_pem().unwrap();
            private_key_bytes = private_key.private_key_to_pem_pkcs8().unwrap();
        }
        FileFormat::Der => {
            certificate_bytes = certificate.to_der().unwrap();
            private_key_bytes = private_key.private_key_to_der().unwrap();
        }
    }

    Ok((certificate_bytes, private_key_bytes))
}

/// Make a X509 request with the given private key
fn mk_request(config: &Config, private_key: &PKey<Private>) -> Result<X509Req, ErrorStack> {
    let mut req_builder = X509ReqBuilder::new()?;
    req_builder.set_pubkey(private_key)?;

    let mut x509_name = X509NameBuilder::new()?;
    x509_name.append_entry_by_text("CN", &config.common_name)?;
    let x509_name = x509_name.build();
    req_builder.set_subject_name(&x509_name)?;

    req_builder.sign(private_key, MessageDigest::sha256())?;
    let req = req_builder.build();
    Ok(req)
}
