// Solana Data Storage Test Program
// This demonstrates how data is stored in Solana accounts on devnet

const {
    Connection,
    PublicKey,
    Keypair,
    Transaction,
    SystemProgram,
    LAMPORTS_PER_SOL,
    sendAndConfirmTransaction,
  } = require('@solana/web3.js');
  
  const {
    createMint,
    createAccount,
    mintTo,
    getAccount,
    TOKEN_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID,
  } = require('@solana/spl-token');
  
  // Connect to devnet
  const connection = new Connection('https://api.devnet.solana.com', 'confirmed');
  
  class SolanaDataStorageDemo {
  private wallet: any = null;
  private customDataAccount: any = null;

  constructor() {
    this.wallet = null;
    this.customDataAccount = null;
  }
  
    // Generate a new wallet for testing
    async createWallet() {
      console.log('🔑 Creating new wallet...');
      this.wallet = Keypair.generate();
      console.log(`✅ Wallet created: ${this.wallet.publicKey.toString()}`);
      
      // Request airdrop on devnet
      console.log('💰 Requesting SOL airdrop...');
      const signature = await connection.requestAirdrop(
        this.wallet.publicKey,
        2 * LAMPORTS_PER_SOL
      );
      await connection.confirmTransaction(signature);
      console.log('✅ Airdrop confirmed!');
      
      return this.wallet.publicKey.toString();
    }
  
    // Check wallet account data
    async inspectWalletAccount() {
      console.log('\n📊 WALLET ACCOUNT INSPECTION');
      console.log('=' .repeat(50));
      
      const accountInfo = await connection.getAccountInfo(this.wallet.publicKey);
      const balance = await connection.getBalance(this.wallet.publicKey);
      
      console.log(`Address: ${this.wallet.publicKey.toString()}`);
      console.log(`Balance: ${balance / LAMPORTS_PER_SOL} SOL`);
      console.log(`Owner: ${accountInfo.owner.toString()}`);
      console.log(`Executable: ${accountInfo.executable}`);
      console.log(`Data Length: ${accountInfo.data.length} bytes`);
      console.log(`Rent Epoch: ${accountInfo.rentEpoch}`);
    }
  
    // Create a custom data account
    async createCustomDataAccount() {
      console.log('\n🏗️  CREATING CUSTOM DATA ACCOUNT');
      console.log('=' .repeat(50));
      
      // Generate a new account for storing custom data
      this.customDataAccount = Keypair.generate();
      const dataSize = 1024; // 1KB of data storage
      
      // Calculate rent exemption amount
      const rentExemption = await connection.getMinimumBalanceForRentExemption(dataSize);
      console.log(`Rent exemption required: ${rentExemption / LAMPORTS_PER_SOL} SOL`);
      
      // Create the account
      const transaction = new Transaction().add(
        SystemProgram.createAccount({
          fromPubkey: this.wallet.publicKey,
          newAccountPubkey: this.customDataAccount.publicKey,
          lamports: rentExemption,
          space: dataSize,
          programId: SystemProgram.programId,
        })
      );
      
      const signature = await sendAndConfirmTransaction(
        connection,
        transaction,
        [this.wallet, this.customDataAccount]
      );
      
      console.log(`✅ Custom data account created: ${this.customDataAccount.publicKey.toString()}`);
      console.log(`Transaction signature: ${signature}`);
      
      return this.customDataAccount.publicKey.toString();
    }
  
    // Inspect the custom data account
    async inspectCustomDataAccount() {
      console.log('\n📊 CUSTOM DATA ACCOUNT INSPECTION');
      console.log('=' .repeat(50));
      
      const accountInfo = await connection.getAccountInfo(this.customDataAccount.publicKey);
      const balance = await connection.getBalance(this.customDataAccount.publicKey);
      
      console.log(`Address: ${this.customDataAccount.publicKey.toString()}`);
      console.log(`Balance: ${balance / LAMPORTS_PER_SOL} SOL`);
      console.log(`Owner: ${accountInfo.owner.toString()}`);
      console.log(`Executable: ${accountInfo.executable}`);
      console.log(`Data Length: ${accountInfo.data.length} bytes`);
      console.log(`Data (first 32 bytes): ${accountInfo.data.slice(0, 32).toString('hex')}`);
      console.log(`Rent Epoch: ${accountInfo.rentEpoch}`);
    }
  
    // Create a token mint (like creating a new cryptocurrency)
    async createTokenMint() {
      console.log('\n🪙 CREATING TOKEN MINT');
      console.log('=' .repeat(50));
      
      // Create a new token mint
      const mint = await createMint(
        connection,
        this.wallet,
        this.wallet.publicKey, // mint authority
        this.wallet.publicKey, // freeze authority
        9 // decimals
      );
      
      console.log(`✅ Token mint created: ${mint.toString()}`);
      
      // Inspect mint account (simplified for new API)
      console.log(`Mint Address: ${mint.toString()}`);
      console.log(`Decimals: 9`);
      console.log(`Mint Authority: ${this.wallet.publicKey.toString()}`);
      console.log(`Freeze Authority: ${this.wallet.publicKey.toString()}`);
      
      return mint;
    }
  
    // Create token account and mint some tokens
    async createTokenAccount(mint) {
      console.log('\n💎 CREATING TOKEN ACCOUNT');
      console.log('=' .repeat(50));
      
      // Create token account
      const tokenAccount = await createAccount(
        connection,
        this.wallet,
        mint,
        this.wallet.publicKey
      );
      console.log(`✅ Token account created: ${tokenAccount.toString()}`);
      
      // Mint 1000 tokens to our account
      await mintTo(
        connection,
        this.wallet,
        mint,
        tokenAccount,
        this.wallet.publicKey,
        1000 * Math.pow(10, 9)
      );
      console.log('✅ Minted 1000 tokens');
      
      // Check token account info
      const tokenAccountInfo = await getAccount(connection, tokenAccount);
      console.log(`Token balance: ${Number(tokenAccountInfo.amount) / Math.pow(10, 9)}`);
      console.log(`Owner: ${tokenAccountInfo.owner.toString()}`);
      console.log(`Mint: ${tokenAccountInfo.mint.toString()}`);
      
      return tokenAccount;
    }
  
    // Inspect token account data structure
    async inspectTokenAccount(tokenAccount) {
      console.log('\n📊 TOKEN ACCOUNT INSPECTION');
      console.log('=' .repeat(50));
      
      const accountInfo = await connection.getAccountInfo(tokenAccount);
      const balance = await connection.getBalance(tokenAccount);
      
      console.log(`Address: ${tokenAccount.toString()}`);
      console.log(`Balance: ${balance / LAMPORTS_PER_SOL} SOL (rent)`);
      console.log(`Owner: ${accountInfo.owner.toString()} (Token Program)`);
      console.log(`Executable: ${accountInfo.executable}`);
      console.log(`Data Length: ${accountInfo.data.length} bytes`);
      
      // Token account data structure (first 165 bytes)
      const data = accountInfo.data;
      console.log('\n🔍 Token Account Data Structure:');
      console.log(`Mint (32 bytes): ${data.slice(0, 32).toString('hex')}`);
      console.log(`Owner (32 bytes): ${data.slice(32, 64).toString('hex')}`);
      console.log(`Amount (8 bytes): ${data.slice(64, 72).toString('hex')}`);
    }
  
    // Demonstrate Program Derived Address (PDA)
    async demonstratePDA() {
      console.log('\n🔗 PROGRAM DERIVED ADDRESS (PDA) DEMO');
      console.log('=' .repeat(50));
      
      // Find PDA for a hypothetical game program
      const gameProgram = new PublicKey('11111111111111111111111111111112'); // System program as example
      const playerSeed = 'player';
      
      const [pda, bump] = await PublicKey.findProgramAddress(
        [Buffer.from(playerSeed), this.wallet.publicKey.toBuffer()],
        gameProgram
      );
      
      console.log(`Player PDA: ${pda.toString()}`);
      console.log(`Bump seed: ${bump}`);
      console.log(`Seeds used: "${playerSeed}" + wallet public key`);
      console.log('✅ This PDA can store player game data deterministically');
    }
  
    // Show rent calculation
    async demonstrateRentCalculation() {
      console.log('\n💰 RENT CALCULATION DEMO');
      console.log('=' .repeat(50));
      
      const sizes = [0, 165, 1024, 10000, 1000000]; // Different data sizes
      
      for (const size of sizes) {
        const rent = await connection.getMinimumBalanceForRentExemption(size);
        console.log(`${size.toString().padStart(8)} bytes: ${(rent / LAMPORTS_PER_SOL).toFixed(8)} SOL`);
      }
    }
  
    // Run all demonstrations
    async runFullDemo() {
      try {
        console.log('🚀 SOLANA DATA STORAGE DEMONSTRATION');
        console.log('=' .repeat(60));
        
        // 1. Create wallet and get some SOL
        await this.createWallet();
        await this.inspectWalletAccount();
        
        // 2. Create and inspect custom data account
        await this.createCustomDataAccount();
        await this.inspectCustomDataAccount();
        
        // 3. Create token mint and account
        const mint = await this.createTokenMint();
        const tokenAccount = await this.createTokenAccount(mint);
        await this.inspectTokenAccount(tokenAccount);
        
        // 4. Demonstrate PDA
        await this.demonstratePDA();
        
        // 5. Show rent calculations
        await this.demonstrateRentCalculation();
        
        console.log('\n✅ DEMONSTRATION COMPLETE!');
        console.log('Check these accounts on Solana Explorer (devnet):');
        console.log(`- Wallet: https://explorer.solana.com/address/${this.wallet.publicKey.toString()}?cluster=devnet`);
        console.log(`- Custom Data Account: https://explorer.solana.com/address/${this.customDataAccount.publicKey.toString()}?cluster=devnet`);
        console.log(`- Token Account: https://explorer.solana.com/address/${tokenAccount.toString()}?cluster=devnet`);
        
      } catch (error) {
        console.error('❌ Error:', error.message);
      }
    }
  }
  
  // Usage instructions
  console.log(`
  SETUP INSTRUCTIONS:
  ==================
  
  1. Install dependencies:
     npm install @solana/web3.js @solana/spl-token
  
  2. Run the demo:
     node solana-demo.js
  
  3. Or run individual parts:
     const demo = new SolanaDataStorageDemo();
     await demo.runFullDemo();
  
  WHAT THIS DEMONSTRATES:
  ======================
  
  ✅ Wallet accounts (System Program owned)
  ✅ Custom data accounts with rent
  ✅ Token mint accounts
  ✅ Token holder accounts (SPL Token Program owned)
  ✅ Program Derived Addresses (PDAs)
  ✅ Rent calculation for different data sizes
  ✅ Account inspection and data structure analysis
  
  Each account type shows different ownership, data structures, and use cases!
  `);
  
  // Export for use
  module.exports = SolanaDataStorageDemo;
  
  // Auto-run if this file is executed directly
  if (require.main === module) {
    const demo = new SolanaDataStorageDemo();
    demo.runFullDemo();
  }