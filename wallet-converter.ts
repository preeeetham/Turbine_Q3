import bs58 from 'bs58';
import prompt from 'prompt-sync';

const promptSync = prompt();

// Convert base58 string (Phantom format) to byte array (Solana wallet format)
function base58ToWallet(): void {
    console.log("Enter your base58 private key (Phantom format):");
    const base58 = promptSync('');
    
    try {
        const wallet = bs58.decode(base58);
        console.log("Solana wallet format (byte array):");
        console.log(JSON.stringify(Array.from(wallet)));
    } catch (error) {
        console.error("Error decoding base58 string:", error);
    }
}

// Convert byte array (Solana wallet format) to base58 string (Phantom format)
function walletToBase58(): void {
    console.log("Enter your wallet byte array (Solana format):");
    const walletInput = promptSync('');
    
    try {
        const wallet = JSON.parse(walletInput);
        const base58 = bs58.encode(Buffer.from(wallet));
        console.log("Phantom wallet format (base58):");
        console.log(base58);
    } catch (error) {
        console.error("Error encoding to base58:", error);
    }
}

// Main function to choose conversion direction
function main(): void {
    console.log("Solana Wallet Format Converter");
    console.log("1. Convert base58 (Phantom) to wallet (Solana)");
    console.log("2. Convert wallet (Solana) to base58 (Phantom)");
    console.log("Enter your choice (1 or 2):");
    
    const choice = promptSync('');
    
    switch (choice) {
        case '1':
            base58ToWallet();
            break;
        case '2':
            walletToBase58();
            break;
        default:
            console.log("Invalid choice. Please enter 1 or 2.");
    }
}

main(); 