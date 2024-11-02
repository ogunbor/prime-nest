use anchor_lang::prelude::*;
use hex;

use crate::errors::VaultError;

// Helper function for parsing hex feed ID
pub fn get_feed_id_from_hex(hex_str: &str) -> Result<[u8; 32]> {
    let mut bytes = [0u8; 32];
    hex::decode_to_slice(hex_str, &mut bytes).map_err(|_| error!(VaultError::InvalidHex))?;
    Ok(bytes)
}
