use std::iter::repeat;
use std::time::{SystemTime, UNIX_EPOCH};

use crypto::digest::Digest;
use ring::digest::{Context, SHA256};
use ring::signature::{EcdsaKeyPair, ECDSA_P256_SHA256_FIXED, ECDSA_P256_SHA256_FIXED_SIGNING};
use ring::rand::SystemRandom;

pub fn new_key_pair() -> Result<Vec<u8>, ring::error::Unspecified> {
    let rng = SystemRandom::new();
    let pkcs = EcdsaKeyPair::generate_pkcs8(
        &ring::signature::ECDSA_P256_SHA256_FIXED_SIGNING,
        &rng,
    )?;
    Ok(pkcs.as_ref().to_vec())
}

pub fn sha256_digest(data: &[u8]) -> Vec<u8>{
    let mut context = Context::new(&SHA256);
    context.update(data);
    let digest = context.finish();
    digest.as_ref().to_vec()
}

pub fn ripemd160_digest(data: &[u8]) -> Vec<u8> {
    let mut ripemd160 = crypto::ripemd160::Ripemd160::new();
    ripemd160.input(data);
    let mut buf: Vec<u8> = repeat(0).take(ripemd160.output_bytes()).collect();
    ripemd160.result(&mut buf);
    buf
}

pub fn base58_encode(data: &[u8]) -> String {
    bs58::encode(data).into_string()
}

pub fn base58_decode(data: &str) -> Result<Vec<u8>, bs58::decode::Error> {
    bs58::decode(data).into_vec()
}


pub fn ecdsa_p256_sha256_sign_digest(pkcs8: &[u8], message: &[u8]) -> Result<Vec<u8>, ring::error::KeyRejected> {
    let key_pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, pkcs8)?;
    let rng = ring::rand::SystemRandom::new();
    Ok(key_pair.sign(&rng, message).unwrap().as_ref().to_vec())
}


pub fn ecdsa_p256_sha256_sign_verify(public_key: &[u8], signature: &[u8], message: &[u8]) -> bool {
    let peer_public_key =
        ring::signature::UnparsedPublicKey::new(&ECDSA_P256_SHA256_FIXED, public_key);
    let result = peer_public_key.verify(message, signature.as_ref());
    result.is_ok()
}

pub fn current_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as i64
}

