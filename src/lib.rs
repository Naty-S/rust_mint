use solana_sdk::{
    signature::{Keypair, Signer},
    pubkey::Pubkey
};

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn keygen() {
    let kp = Keypair::new();

    print!("You've generated a new Solana wallet: {}\n", kp.pubkey().to_string());
    print!("To save your wallet, copy and paste the following into a JSON file:");
    print!("{:?}", kp.to_bytes());
  }

  #[test]
  fn airdrop() {
  }

  #[test]
  fn transfer_sol() {
    }
}
