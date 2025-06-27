use bs58;
use std::io::{self, BufRead};
use solana_client::rpc_client::RpcClient;
use solana_program::{pubkey::Pubkey, system_instruction::transfer};
use solana_sdk::{
    message::Message,
    signature::{read_keypair_file},
    transaction::Transaction,
    hash::hash,
};
use std::str::FromStr;

const RPC_URL: &str = "https://api.devnet.solana.com";

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::signature::{Keypair, Signer};

    #[test]
    fn keygen() {
        // Create a new keypair
        let kp = Keypair::new();
        println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string());
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn airdrop() {
        // Import our keypair
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        // We'll establish a connection to Solana devnet using the const we defined above
        let client = RpcClient::new(RPC_URL.to_string());
        // We're going to claim 2 devnet SOL tokens (2 billion lamports)
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(sig) => {
                println!("Success! Check your TX here:");
                println!(
                    "https://explorer.solana.com/tx/{}?cluster=devnet",
                    sig
                );
            }
            Err(err) => {
                println!("Airdrop failed: {}", err);
            }
        }
    }

    #[test]
    fn transfer_sol() {
        // Load your devnet keypair from file
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        // Generate a signature from the keypair
        let pubkey = keypair.pubkey();

        let message_bytes = b"I verify my Solana Keypair!";
        let sig = keypair.sign_message(message_bytes);
        let sig_hashed = hash(sig.as_ref());
        // Verify the signature using the public key
        match sig.verify(&pubkey.to_bytes(), &sig_hashed.to_bytes()) {
            true => println!("Signature verified"),
            false => println!("Verification failed"),
        }

        // Define the destination (Turbin3) address
        let to_pubkey = Pubkey::from_str("6KpVthY1cTceiHcsnuYm34kQvcSrzNBQ1PjaTkZ4FZzu").unwrap();

        // Connect to devnet
        let rpc_client = RpcClient::new(RPC_URL.to_string());

        // Fetch recent blockhash
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        // Create and sign the transaction
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, 1_000_000)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );

        // Send the transaction and print tx
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }

    #[test]
    fn empty_wallet() {
        // Load your devnet keypair from file
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        
        // Define the destination (Turbin3) address
        let to_pubkey = Pubkey::from_str("6KpVthY1cTceiHcsnuYm34kQvcSrzNBQ1PjaTkZ4FZzu").unwrap();

        // Connect to devnet
        let rpc_client = RpcClient::new(RPC_URL.to_string());

        // Get current balance
        let balance = rpc_client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get balance");

        // Fetch recent blockhash
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        // Build a mock transaction to calculate fee
        let message = Message::new_with_blockhash(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
            Some(&keypair.pubkey()),
            &recent_blockhash,
        );

        // Estimate transaction fee
        let fee = rpc_client
            .get_fee_for_message(&message)
            .expect("Failed to get fee calculator");

        // Create final transaction with balance minus fee
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );

        // Send transaction and verify
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send final transaction");
        println!(
            "Success! Entire balance transferred: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }

    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as a base58 string:");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        println!("Your wallet file format is:");
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}", wallet);
    }

    #[test]
    fn wallet_to_base58() {
        println!("Input your private key as a JSON byte array (e.g. [12,34,...]):");
        let stdin = io::stdin();
        let wallet = stdin
            .lock()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(',')
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        println!("Your Base58-encoded private key is:");
        let base58 = bs58::encode(wallet).into_string();
        println!("{:?}", base58);
    }

    #[test]
    fn check_turbin3_prerequisites() {
        // Define the Turbin3 program ID
        let program_id = Pubkey::from_str("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM").unwrap();
        let turbin3_pubkey = Pubkey::from_str("6KpVthY1cTceiHcsnuYm34kQvcSrzNBQ1PjaTkZ4FZzu").unwrap();
        
        // Create RPC client
        let rpc_client = RpcClient::new(RPC_URL.to_string());
        
        // Define the seed constants from the IDL
        let prereqs_seed = b"prereqs";
        
        // Derive the PDA account for the user
        let (account_pda, _account_bump) = Pubkey::find_program_address(
            &[prereqs_seed, turbin3_pubkey.as_ref()],
            &program_id
        );
        
        println!("🔍 Checking Turbin3 Prerequisites:");
        println!("Your Turbin3 wallet: {}", turbin3_pubkey);
        println!("Your PDA account: {}", account_pda);
        
        // Check if the PDA account exists
        match rpc_client.get_account(&account_pda) {
            Ok(account) => {
                println!("✅ PDA account found! Data length: {} bytes", account.data.len());
                println!("Account owner: {}", account.owner);
                
                // Try to find the collection by deriving it from program seeds
                println!("🔍 Trying to derive collection from program seeds...");
                
                // Try some common collection derivation patterns
                let collection_patterns = vec![
                    ("turbin3", None),
                    ("collection", None),
                    ("global", None),
                    ("turbin3", Some("collection")),
                ];
                
                for (seed1, seed2_opt) in collection_patterns {
                    let seeds: Vec<&[u8]> = if let Some(seed2) = seed2_opt {
                        vec![seed1.as_bytes(), seed2.as_bytes()]
                    } else {
                        vec![seed1.as_bytes()]
                    };
                    
                    let (derived_collection, _bump) = Pubkey::find_program_address(
                        &seeds,
                        &program_id
                    );
                    
                    println!("🔍 Checking derived collection with seeds {:?}: {}", 
                             seeds.iter().map(|s| String::from_utf8_lossy(s)).collect::<Vec<_>>(), 
                             derived_collection);
                    
                    match rpc_client.get_account(&derived_collection) {
                        Ok(account) => {
                            println!("✅ Derived collection found! Owner: {}", account.owner);
                            println!("✅ Data length: {} bytes", account.data.len());
                            if account.owner.to_string() == "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d" {
                                println!("🎉 This is an MPL Core collection! Use this address: {}", derived_collection);
                            }
                        }
                        Err(_) => println!("❌ Derived collection not found"),
                    }
                }
            }
            Err(err) => {
                println!("❌ PDA account not found: {}", err);
                println!("This suggests the TypeScript prerequisites haven't been completed yet.");
            }
        }
    }

    #[test] 
    fn find_your_collection_from_ts() {
        // Since you completed TypeScript prerequisites, let's find your submit_ts transaction
        let rpc_client = RpcClient::new(RPC_URL.to_string());
        let program_id = Pubkey::from_str("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM").unwrap();
        let your_pubkey = Pubkey::from_str("6KpVthY1cTceiHcsnuYm34kQvcSrzNBQ1PjaTkZ4FZzu").unwrap();
        
        println!("🔍 Looking for your previous submit_ts transaction...");
        println!("Your wallet: {}", your_pubkey);
        
        // Let's check if we can find program accounts that might contain collection info
        println!("🔍 Searching for MPL Core collections on devnet...");
        
        // The collection address from your TypeScript implementation
        let known_turbin3_collections = vec![
            "5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2", // Your actual collection from enroll.ts
            "2kqZsyj8YM1vvR6xC2weTwU7S3AFd1M7B6Z5RqQQjVKH", // Backup option
            "5ycSTEYHNHn8vutVhPaEHBZ5ygQ1BNj2qmKPbTqyLs2P", // Another backup
        ];
        
        for collection_str in known_turbin3_collections {
            match Pubkey::from_str(collection_str) {
                Ok(collection_pubkey) => {
                    println!("🔍 Checking known collection: {}", collection_pubkey);
                    
                    match rpc_client.get_account(&collection_pubkey) {
                        Ok(account) => {
                            println!("✅ Collection found!");
                            println!("  Owner: {}", account.owner);
                            println!("  Data length: {} bytes", account.data.len());
                            
                            // Check if this is an MPL Core collection
                            if account.owner.to_string() == "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d" {
                                println!("🎉 This is an MPL Core collection!");
                                
                                // Derive the authority PDA to verify
                                let (authority_pda, _) = Pubkey::find_program_address(
                                    &[b"collection", collection_pubkey.as_ref()],
                                    &program_id
                                );
                                println!("  Derived authority PDA: {}", authority_pda);
                                
                                println!("🎯 Use this collection address: {}", collection_pubkey);
                                println!("");
                                println!("📝 To complete your submission:");
                                println!("1. Update the submit_turbin3 function with this collection address");
                                println!("2. Create a keypair file for your Turbin3 wallet");
                                println!("3. Run the submission!");
                                return;
                            }
                        }
                        Err(_) => println!("❌ Collection not found at this address"),
                    }
                }
                Err(_) => println!("❌ Invalid collection address format"),
            }
        }
        
        println!("❌ Collection not found in known addresses.");
        println!("💡 You can also try:");
        println!("1. Check your TypeScript implementation for the collection address");
        println!("2. Look at the Solana Explorer for recent submit_ts transactions");
        println!("3. Ask in the Turbin3 Discord for the current collection address");
    }

    #[test]
    fn submit_turbin3() {
        // NOTE: You need to provide your Turbin3 wallet keypair file
        // Copy your Turbin3-wallet.json to this directory as "turbin3-wallet.json"
        
        // Try to load the Turbin3 wallet keypair
        let turbin3_keypair = match read_keypair_file("turbin3-wallet.json") {
            Ok(keypair) => keypair,
            Err(_) => {
                println!("❌ ERROR: Turbin3 wallet keypair file not found!");
                println!("");
                println!("📝 To complete the submission, you need to:");
                println!("1. Copy your Turbin3-wallet.json file to this directory");
                println!("2. Rename it to 'turbin3-wallet.json'");
                println!("3. Run this test again");
                println!("");
                println!("💡 Your Turbin3 wallet should contain the private key for: 6KpVthY1cTceiHcsnuYm34kQvcSrzNBQ1PjaTkZ4FZzu");
                return;
            }
        };
        
        // Verify this is the correct wallet
        let expected_pubkey = Pubkey::from_str("6KpVthY1cTceiHcsnuYm34kQvcSrzNBQ1PjaTkZ4FZzu").unwrap();
        if turbin3_keypair.pubkey() != expected_pubkey {
            println!("❌ ERROR: Wrong wallet keypair!");
            println!("Expected: {}", expected_pubkey);
            println!("Got: {}", turbin3_keypair.pubkey());
            return;
        }
        
        println!("✅ Turbin3 wallet loaded successfully!");
        println!("Wallet: {}", turbin3_keypair.pubkey());
        
        // Define all the required addresses from your TypeScript implementation
        let program_id = Pubkey::from_str("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM").unwrap();
        let collection_pubkey = Pubkey::from_str("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2").unwrap();
        let authority_pda = Pubkey::from_str("5xstXUdRJKxRrqbJuo5SAfKf68y7afoYwTeH1FXbsA3k").unwrap();
        let mpl_core_program = Pubkey::from_str("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d").unwrap();
        let system_program = Pubkey::from_str("11111111111111111111111111111111").unwrap();
        
        // Create RPC client
        let rpc_client = RpcClient::new(RPC_URL.to_string());
        
        // Derive the PDA account for the user (same as TypeScript)
        let prereqs_seed = b"prereqs";
        let (account_pda, _account_bump) = Pubkey::find_program_address(
            &[prereqs_seed, turbin3_keypair.pubkey().as_ref()],
            &program_id
        );
        
        println!("📋 Transaction Details:");
        println!("Program ID: {}", program_id);
        println!("Account PDA: {}", account_pda);
        println!("Collection: {}", collection_pubkey);
        println!("Authority: {}", authority_pda);
        
        // Create a new mint keypair for the NFT (like in TypeScript)
        let mint_keypair = Keypair::new();
        println!("New mint: {}", mint_keypair.pubkey());
        
        // Create instruction discriminator for submit_rs (from IDL)
        let discriminator: [u8; 8] = [77, 124, 82, 163, 21, 133, 181, 206];
        
        // Create the instruction exactly as specified in the IDL
        let instruction = solana_program::instruction::Instruction {
            program_id,
            accounts: vec![
                solana_program::instruction::AccountMeta::new(turbin3_keypair.pubkey(), true), // user
                solana_program::instruction::AccountMeta::new(account_pda, false), // account  
                solana_program::instruction::AccountMeta::new(mint_keypair.pubkey(), true), // mint
                solana_program::instruction::AccountMeta::new(collection_pubkey, false), // collection
                solana_program::instruction::AccountMeta::new_readonly(authority_pda, false), // authority
                solana_program::instruction::AccountMeta::new_readonly(mpl_core_program, false), // mpl_core_program
                solana_program::instruction::AccountMeta::new_readonly(system_program, false), // system_program
            ],
            data: discriminator.to_vec(),
        };
        
        // Get recent blockhash
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");
        
        // Create and sign the transaction
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&turbin3_keypair.pubkey()),
            &vec![&turbin3_keypair, &mint_keypair],
            recent_blockhash,
        );
        
        println!("🚀 Submitting Rust completion to Turbin3...");
        
        // Send the transaction
        match rpc_client.send_and_confirm_transaction(&transaction) {
            Ok(signature) => {
                println!("");
                println!("🎉 SUCCESS! Turbin3 Rust submission completed!");
                println!("📜 Transaction signature: {}", signature);
                println!("🔗 Explorer: https://explorer.solana.com/tx/{}?cluster=devnet", signature);
                println!("🎓 You have completed the Turbin3 prerequisites!");
            }
            Err(err) => {
                println!("❌ Submission failed: {}", err);
                if err.to_string().contains("PreReqRsAlreadyCompleted") {
                    println!("💡 You have already completed the Rust submission!");
                } else if err.to_string().contains("PreReqTsNotCompleted") {
                    println!("💡 Complete the TypeScript prerequisites first!");
                }
            }
        }
    }

    #[test]
    fn send_raw_instruction_turbin3() {
        println!("🔧 Section 5.2: Send Raw Instruction to Turbin3 Program");
        println!("");
        
        // Step 1: Create a Solana RPC client
        let rpc_client = RpcClient::new(RPC_URL.to_string());
        
        // Step 2: Load your signer keypair
        // Note: Using turbin3-wallet.json since we emptied dev-wallet.json
        let signer = read_keypair_file("turbin3-wallet.json")
            .expect("Couldn't find wallet file");
        
        println!("✅ Signer loaded: {}", signer.pubkey());
        
        // Step 3: Define program and account public keys
        let mint = Keypair::new();
        let turbin3_prereq_program = 
            Pubkey::from_str("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM").unwrap();
        let collection = 
            Pubkey::from_str("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2").unwrap();
        let mpl_core_program = 
            Pubkey::from_str("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d").unwrap();
        let system_program = solana_program::system_program::id();
        
        println!("✅ Program and account keys defined");
        println!("   New mint: {}", mint.pubkey());
        
        // Step 4: Get the PDA (Program Derived Address)
        let signer_pubkey = signer.pubkey();
        let seeds = &[b"prereqs", signer_pubkey.as_ref()];
        let (prereq_pda, _bump) = Pubkey::find_program_address(seeds, &turbin3_prereq_program);
        
        println!("✅ PDA derived: {}", prereq_pda);
        
        // Also derive the authority PDA (missing from the steps but required)
        let authority_seeds = &[b"collection", collection.as_ref()];
        let (authority, _authority_bump) = Pubkey::find_program_address(authority_seeds, &turbin3_prereq_program);
        
        println!("✅ Authority PDA: {}", authority);
        
        // Step 5: Prepare the instruction data (discriminator)
        let data = vec![77, 124, 82, 163, 21, 133, 181, 206];
        
        println!("✅ Instruction discriminator prepared");
        
        // Step 6: Define the accounts metadata
        let accounts = vec![
            solana_program::instruction::AccountMeta::new(signer.pubkey(), true),  // user signer
            solana_program::instruction::AccountMeta::new(prereq_pda, false),      // PDA account  
            solana_program::instruction::AccountMeta::new(mint.pubkey(), true),    // mint keypair
            solana_program::instruction::AccountMeta::new(collection, false),      // collection
            solana_program::instruction::AccountMeta::new_readonly(authority, false), // authority (PDA)
            solana_program::instruction::AccountMeta::new_readonly(mpl_core_program, false), // mpl core program
            solana_program::instruction::AccountMeta::new_readonly(system_program, false),   // system program
        ];
        
        println!("✅ Account metadata defined ({} accounts)", accounts.len());
        
        // Step 7: Get the recent blockhash
        let blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");
        
        println!("✅ Recent blockhash obtained");
        
        // Step 8: Build the instruction
        let instruction = solana_program::instruction::Instruction {
            program_id: turbin3_prereq_program,
            accounts,
            data,
        };
        
        println!("✅ Raw instruction constructed");
        
        // Step 9: Create and sign the transaction
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&signer.pubkey()),
            &[&signer, &mint],
            blockhash,
        );
        
        println!("✅ Transaction created and signed");
        println!("");
        println!("🚀 Sending raw instruction to Turbin3 program...");
        
        // Step 10: Send and confirm the transaction
        match rpc_client.send_and_confirm_transaction(&transaction) {
            Ok(signature) => {
                println!("🎉 Success! Check out your TX here:");
                println!("https://explorer.solana.com/tx/{}/?cluster=devnet", signature);
                println!("");
                println!("✅ Raw instruction sent successfully!");
                println!("📋 Transaction Details:");
                println!("   Signature: {}", signature);
                println!("   Program: {}", turbin3_prereq_program);
                println!("   User: {}", signer.pubkey());
                println!("   PDA: {}", prereq_pda);
                println!("   Mint: {}", mint.pubkey());
                println!("   Collection: {}", collection);
                println!("   Authority: {}", authority);
            }
            Err(err) => {
                println!("❌ Transaction failed: {}", err);
                if err.to_string().contains("PreReqRsAlreadyCompleted") {
                    println!("💡 Note: You have already completed the Rust submission!");
                    println!("🎉 This means your previous submission was successful!");
                }
            }
        }
    }
}
