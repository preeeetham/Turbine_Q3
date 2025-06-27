import { Connection, Keypair, PublicKey, SystemProgram } from "@solana/web3.js"
import { Program, Wallet, AnchorProvider } from "@coral-xyz/anchor"
import wallet from "./Turbin3-wallet.json"

const MPL_CORE_PROGRAM_ID = new PublicKey("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d");
const TURBIN3_PROGRAM_ID = new PublicKey("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM");
const SYSTEM_PROGRAM_ID = new PublicKey("11111111111111111111111111111111");

// Declare the address of the mint Collection (from section 5.4)
const mintCollection = new PublicKey("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2");

// Create the mint Account for the new asset (from section 5.4)
const mintTs = Keypair.generate();

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

// Create a devnet connection
const connection = new Connection("https://api.devnet.solana.com");

// Create our anchor provider
const provider = new AnchorProvider(connection, new Wallet(keypair), {
    commitment: "confirmed"
});

// Create the PDA for our enrollment account (as specified in section 5.3)
const account_seeds = [
    Buffer.from("prereqs"),
    keypair.publicKey.toBuffer(),
];
const [account_key, _account_bump] = PublicKey.findProgramAddressSync(account_seeds, TURBIN3_PROGRAM_ID);

// Fetch the program using the provider
async function createProgram() {
    const program = await Program.at(TURBIN3_PROGRAM_ID, provider);
    return program;
}

// Helper function to derive PDA
function deriveAccountPDA(user: PublicKey): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
        [Buffer.from("prereqs"), user.toBuffer()],
        TURBIN3_PROGRAM_ID
    );
}

function deriveAuthorityPDA(collection: PublicKey): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
        [Buffer.from("collection"), collection.toBuffer()],
        TURBIN3_PROGRAM_ID
    );
}

async function initialize(program: any, githubUsername: string) {
    try {
        console.log(`Initializing Turbin3 account for GitHub: ${githubUsername}`);
        console.log(`User wallet: ${keypair.publicKey.toBase58()}`);
        
        // Use the PDA created as specified in section 5.3
        console.log(`Account PDA: ${account_key.toBase58()}`);
        console.log(`Bump: ${_account_bump}`);
        
        // Execute the initialize transaction (as specified in section 5.4)
        const txhash = await program.methods
            .initialize(githubUsername)
            .accounts({
                user: keypair.publicKey,
                account: account_key,
                systemProgram: SystemProgram.programId,
            })
            .signers([keypair])
            .rpc();
            
        console.log(`Success! Check out your TX here:
https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
        
    } catch (e) {
        console.error(`Oops, something went wrong: ${e}`);
    }
}

async function submitTypeScript(program: any) {
    try {
        console.log("Submitting TypeScript work to Turbin3 program");
        
        // Use the predefined collection and mint from section 5.4
        console.log(`Mint address: ${mintTs.publicKey.toBase58()}`);
        console.log(`Using collection: ${mintCollection.toBase58()}`);
        
        // Derive the authority PDA for the collection (the missing piece!)
        const [authorityPDA] = deriveAuthorityPDA(mintCollection);
        console.log(`Authority PDA: ${authorityPDA.toBase58()}`);
        
        // Execute the submitTs transaction (as specified in section 5.4)
        const txhash = await program.methods
            .submitTs()
            .accounts({
                user: keypair.publicKey,
                account: account_key,
                mint: mintTs.publicKey,
                collection: mintCollection,
                authority: authorityPDA,
                mplCoreProgram: MPL_CORE_PROGRAM_ID,
                systemProgram: SystemProgram.programId,
            })
            .signers([keypair, mintTs])
            .rpc();
            
        console.log(`Success! Check out your TX here:
https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
        
    } catch (e) {
        console.error(`Oops, something went wrong: ${e}`);
    }
}

// Main function
(async () => {
    console.log("=== Turbin3 Program Interaction ===");
    console.log(`Program ID: ${TURBIN3_PROGRAM_ID.toBase58()}`);
    console.log(`Your Turbin3 Wallet: ${keypair.publicKey.toBase58()}`);
    console.log("");
    
    console.log("Executing Turbin3 program instructions...");
    console.log("");
    
    // Create program instance
    const program = await createProgram();
    
    // Execute the initialize transaction (ALREADY COMPLETED)
    console.log("Initialize transaction already completed in previous run");
    // await initialize(program, "preeeetham");
    console.log("");
    
    // Execute the submitTs transaction
    console.log("Executing submitTs transaction...");
    await submitTypeScript(program);
    
    console.log("");
    console.log("🎉 Turbin3 program interaction complete!");
})();