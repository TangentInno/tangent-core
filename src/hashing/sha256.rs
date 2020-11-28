/*
*
*	Author: Austin Mullins
*	Copyright: Tangent
*
*/

use sha2::{Sha256, Digest};

pub fn generate_sha256_hash(seed: &str) -> String {
  let mut result = Sha256::new();
  result.update(seed);

  return format!("{:x}", &result.finalize())
}