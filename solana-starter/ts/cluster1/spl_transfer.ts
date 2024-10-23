import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "../Turbin3-wallet.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("5MENh514BfFXjgRDVWPTk4Kk5uUhCev8ASqbsCEJxniC");

// Recipient address
const to = new PublicKey("5gAoKd7LyZ6mvHoJ2cBNwsL871NE6kKgYMSYEdRs4j33");

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        const fromWalletAta = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            keypair.publicKey
        )
          
        // Get the token account of the toWallet address, and if it does not exist, create it
        const toWalletAta = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            to
        )
        // Transfer the new token to the "toTokenAccount" we just created
        const transferTx = await transfer(
            connection, 
            keypair, 
            fromWalletAta.address, 
            toWalletAta.address, 
            keypair.publicKey, 
            100n * 1_000_000n
        )

        console.log(`Your transfer txid: ${transferTx}`);
        
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();
