#[cfg(test)]
mod tests {
  use solana_sdk::{
      signature::{Keypair, Signer, read_keypair_file},
      // pubkey::Pubkey
  };
  use solana_client::rpc_client::RpcClient;
  
  pub const RPC_URL: &str = "https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55aca5c9c80a";

  #[test]
  fn keygen() {
    let kp = Keypair::new();

    print!("You've generated a new Solana wallet: {}\n", kp.pubkey().to_string());
    print!("To save your wallet, copy and paste the following into a JSON file:");
    print!("{:?}", kp.to_bytes());
  }

  #[test]
  fn airdrop() {
    // Import KP, if not expect then type got Err in it and not Keypair type
    let keypair = read_keypair_file("src/wallets/dev-wallet.json").expect("Couldn't find wallet file");
    // Connect to Solana devnet
    let client = RpcClient::new(RPC_URL);
    // Claim 2 devnet SOL (2 billion lamports)
    match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
      Ok(sig) => {
        println!("Success! Check your TX here:");
        println!("https://explorer.solana.com/tx/{}?cluster=devnet", sig);
      }
      Err(err) => { println!("Airdrop failed: {}", err); }
    }
  }

  #[test]
  fn transfer_sol() {
  }
}
