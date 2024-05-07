#![cfg_attr(feature = "guest", no_std)]
#![no_main]

use ed25519_dalek::{Signature, VerifyingKey};

#[jolt::provable]
fn farcaster_message(message_hash: &[u8], signature: &[u8], public_key: &[u8]) -> bool {
    let verifying_key: VerifyingKey = VerifyingKey::try_from(public_key).unwrap();
    return verifying_key.verify_strict(&message_hash, &Signature::try_from(signature).unwrap()).is_ok();
}
