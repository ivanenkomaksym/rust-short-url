use sha2::{Sha256, Digest};

pub fn hash(value_to_hash: &str) -> String {
    let mut sha256 = Sha256::new();
    sha256.update(value_to_hash);    
    let hash_result = sha256.finalize();

    // Take the first 4 bytes (32 bits) of the hash and convert them to u32
    let hash_value = u32::from_be_bytes([hash_result[0], hash_result[1], hash_result[2], hash_result[3]]);

    // Format the u32 as an 8-digit string
    return format!("{:X}", hash_value)
}