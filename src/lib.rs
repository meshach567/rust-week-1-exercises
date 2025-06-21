#[allow(unused)]

// Implement extract_tx_version function below

pub fn extract_tx_version(_raw_tx_hex: &str) -> Result<u32, String> {
    return match hex::decode(_raw_tx_hex) {
        Ok(decoded) => {
            if decoded.len() < 4 {
                Err("Transaction hex is too short to contain a version".to_string())
            } else {
                // The version is stored in the first 4 bytes
                let version_bytes = &decoded[0..4];
                let version = u32::from_le_bytes([version_bytes[0], version_bytes[1], version_bytes[2], version_bytes[3]]);
                Ok(version)
            }
        },
        Err(e) => Err(format!("Failed to decode hex: {}", e)),
    };
}

#[allow(unused)]
fn main() {
    // Example usage of the extract_tx_version function
    let raw_tx_hex = "0100000001abcdef"; // Example hex string
    match extract_tx_version(raw_tx_hex) {
        Ok(version) => println!("Transaction version: {}", version),
        Err(e) => println!("Error extracting transaction version: {}", e),
    }
}
