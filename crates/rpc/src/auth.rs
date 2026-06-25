//! Stub auth module — replaces the real keypair-based auth used in upstream Zed.
//! The real implementation generates asymmetric encryption keys for the browser
//! sign-in flow; this fork removes that flow so we only need the types to compile.

use anyhow::Result;

/// A stub public key that satisfies callers expecting to serialize one to a String.
pub struct PublicKey;

/// A stub private key that satisfies callers expecting to decrypt an access token.
pub struct PrivateKey;

impl TryFrom<PublicKey> for String {
    type Error = anyhow::Error;
    fn try_from(_: PublicKey) -> Result<String> {
        Ok(String::new())
    }
}

impl PrivateKey {
    pub fn decrypt_string(&self, _ciphertext: &str) -> Result<String> {
        anyhow::bail!("auth stub: decrypt_string not implemented")
    }
}

/// Generates a stub keypair. Always succeeds and returns no-op keys.
pub fn keypair() -> Result<(PublicKey, PrivateKey)> {
    Ok((PublicKey, PrivateKey))
}
