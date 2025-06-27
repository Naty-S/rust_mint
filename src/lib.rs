#[cfg(test)]
mod tests {
  use solana_sdk::{
    signature::{Keypair, Signer, read_keypair_file},
    transaction::Transaction,
    message::Message,
    // hash::hash
  };
  use solana_program::{
    pubkey::Pubkey,
    system_instruction::transfer,
    // hash::hash
  };
  use solana_client::rpc_client::RpcClient;
  use std::str::FromStr;

  
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
    let keypair = read_keypair_file("src/wallets/dev-wallet.json").expect("Couldn't find wallet file");
    // Generate a signature from the keypair
    let pubkey = keypair.pubkey();
    let message_bytes = b"I verify my Solana Keypair!";
    let sig = keypair.sign_message(message_bytes);
    // let sig_hashed = hash(sig.as_ref());

    // Verify the signature using the public key
    match sig.verify(&pubkey.to_bytes(), message_bytes) {
      true => println!("Signature verified"),
      false => println!("Verification failed"),
    };

    let to_pubkey = Pubkey::from_str("FzozXYEUfyuSPckCmyh5Y73b9KZe1ZjAXzfsuRCD3Ct").unwrap();
    let rpc_client = RpcClient::new(RPC_URL);

    let balance = rpc_client
      .get_balance(&keypair.pubkey())
      .expect("Failed to get balance");

    let recent_blockhash = rpc_client
      .get_latest_blockhash()
      .expect("Failed to get recent blockhash");

    // Mock TX
    let message = Message::new_with_blockhash(
      &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
      Some(&keypair.pubkey()),
      &recent_blockhash,
    );

    let fee = rpc_client
      .get_fee_for_message(&message)
      .expect("Failed to get fee calculator");

    let transaction = Transaction::new_signed_with_payer(
      &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
      Some(&keypair.pubkey()),
      &vec![&keypair],
      recent_blockhash,
    );

    let signature = rpc_client
      .send_and_confirm_transaction(&transaction)
      .expect("Failed to send transaction");
    
    println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",signature);
  }
}
