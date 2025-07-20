use anyhow::{Context, Result};
use log::info;
use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::pkey::Private;
use openssl::sign::Signer;
use std::fs;

const GENERIC_DIGEST_EXT: &str = ".dgst";

pub fn read_private_key(path: &str) -> Result<PKey<Private>> {
    info!("Attempting to read private key");
    let private_key_bytes =
        fs::read(&path).with_context(|| "Failed to read private key from PEM bytes")?;
    let private_key = PKey::private_key_from_pem(&private_key_bytes)?;

    Ok(private_key)
}

pub fn sign_file(private_key: &PKey<Private>, file_path: &str) -> Result<()> {
    info!("Signing file '{}'", &file_path);

    let file_bytes = fs::read(&file_path).with_context(|| "Failed to read file to sign")?;
    let mut signer = Signer::new(MessageDigest::sha256(), &private_key)?;
    signer.update(&file_bytes)?;

    let signature = signer.sign_to_vec()?;
    fs::write(
        &format!("{}{}", &file_path, &GENERIC_DIGEST_EXT),
        &signature,
    )?;

    Ok(())
}
