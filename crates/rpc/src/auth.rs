use anyhow::Result;

pub struct PublicKey(Vec<u8>);

pub struct PrivateKey(Vec<u8>);

pub fn keypair() -> Result<(PublicKey, PrivateKey)> {
    Ok((PublicKey(vec![]), PrivateKey(vec![])))
}

pub fn random_token() -> String {
    String::new()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EncryptionFormat {
    V0,
    V1,
}

impl PublicKey {
    pub fn encrypt_string(&self, _: &str, _: EncryptionFormat) -> Result<String> {
        Ok(String::new())
    }
}

impl PrivateKey {
    pub fn decrypt_string(&self, _: &str) -> Result<String> {
        Ok(String::new())
    }
}

impl TryFrom<PublicKey> for String {
    type Error = anyhow::Error;

    fn try_from(_: PublicKey) -> Result<Self> {
        Ok(String::new())
    }
}
