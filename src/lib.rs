#[cfg(test)]
mod tests {
  use solana_sdk::{
    instruction::AccountMeta,
    message::Message,
    signature::{read_keypair_file, Keypair, Signer},
    transaction::Transaction,
    instruction::Instruction,
    // hash::hash
  };
  use solana_program::{
    pubkey::Pubkey,
    system_instruction::transfer,
    // hash::hash
  };
  use solana_client::rpc_client::RpcClient;
  use solana_sdk_ids::system_program;
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

  #[test]
  fn enroll() {
    let rpc_client = RpcClient::new(RPC_URL);
    let turbin3_prereq_program = Pubkey::from_str("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM").unwrap();
    let collection = Pubkey::from_str("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2").unwrap();
    let mpl_core_program = Pubkey::from_str("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d").unwrap();
    let system_program = system_program::id();
    
    let keypair = read_keypair_file("src/wallets/Turbin3-wallet.json").expect("Couldn't find Turbin3 wallet file");
    let signer_pubkey = keypair.pubkey();
    let seeds = &[b"prereqs", signer_pubkey.as_ref()];
    let (prereq_pda, _bump) = Pubkey::find_program_address(
      seeds,
      &turbin3_prereq_program
    );
    
    let mint = read_keypair_file("src/wallets/mint-wallet.json").expect("Couldn't find mint wallet file");
    // println!("mint public: {:?}", &mint.pubkey());
    // println!("mint secret: {:?}", &mint.to_bytes());

    let (authority, _bump_auth) = Pubkey::find_program_address(
      &[b"collection", collection.as_ref()],
      &turbin3_prereq_program
    );
    // println!("authority: {}", &authority);

    // The discriminator uniquely identifies the instruction your program expects.
    let data = vec![77, 124, 82, 163, 21, 133, 181, 206];

    // Use new for accounts that the instruction writes to and new_readonly for accounts that are read-only.
    // The 'true' flag indicates the account must sign the transaction.
    let accounts = vec![
      AccountMeta::new(keypair.pubkey(), true), // user signer
      AccountMeta::new(prereq_pda, false), // PDA account
      AccountMeta::new(mint.pubkey(), true), // mint keypair
      AccountMeta::new(collection, false), // collection
      AccountMeta::new_readonly(authority, false), // authority (PDA)
      AccountMeta::new_readonly(mpl_core_program, false), // mpl core program
      AccountMeta::new_readonly(system_program, false), // system program
    ];

    let recent_blockhash = rpc_client
      .get_latest_blockhash()
      .expect("Failed to get recent blockhash");

    let instruction = Instruction {
      program_id: turbin3_prereq_program,
      accounts,
      data,
    };

    let transaction = Transaction::new_signed_with_payer(
      &[instruction],
      Some(&keypair.pubkey()),
      &[&keypair, &mint],
      recent_blockhash,
    );

    let signature = rpc_client
    .send_and_confirm_transaction(&transaction)
    .expect("Failed to send transaction");

    println!("Success! Check out your TX here:\nhttps://explorer.solana.com/tx/{}/?cluster=devnet", signature);
  }
}
