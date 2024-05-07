use std::env;
use hex::FromHex; 
use blake3::hash;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    let (prove_farcaster_message, verify_farcaster_message) = guest::build_farcaster_message();

    let hex_public_key = &args[1];
    let hex_message = &args[2];
    let hex_signature = &args[3];

    let mut public_key_bytes: Vec<u8> = FromHex::from_hex(&hex_public_key).unwrap();
    public_key_bytes = public_key_bytes[..32].try_into().unwrap();
    let message_bytes: Vec<u8> = FromHex::from_hex(&hex_message).unwrap();
    let signature_bytes: Vec<u8> = FromHex::from_hex(&hex_signature).unwrap();
    let message_hash_bytes = *hash(&message_bytes).as_bytes();

    let (output, proof) = prove_farcaster_message(&message_hash_bytes[..20], &signature_bytes[..64], &public_key_bytes);
    let is_valid = verify_farcaster_message(proof);

    println!("output: {}", output);
    println!("valid: {}", is_valid);
}
