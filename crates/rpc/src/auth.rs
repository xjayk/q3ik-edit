use anyhow::Result;

pub enum EncryptionFormat { V0, V1 }
pub struct PublicKey;
pub struct PrivateKey;

pub fn keypair() -> Result<(PublicKey, PrivateKey)> {
    Ok((PublicKey, PrivateKey))
}

pub fn random_token() -> String {
    "token".to_string()
}
