import { wallet } from "~/.config/solana/agent_wallet.json"
import bs58 from "bs58"

const bs58PrivateKey = wallet; 
console.log(bs58PrivateKey);
const secretKey = bs58.decode(bs58PrivateKey);

console.log(secretKey)
