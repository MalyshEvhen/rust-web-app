use anyhow::Result;
use rand::RngCore;

fn main() -> Result<()> {
  let mut key = [0u8; 64];
  rand::thread_rng().fill_bytes(&mut key);
  print!("\nGenerated key: \n{key:?}");

  let b64u = base64_url::encode(&key);
  println!("\nKey base 64 encoded:\n{b64u}");

  Ok(())
}
