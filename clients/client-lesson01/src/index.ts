import {
  Connection,
  Keypair,
  PublicKey,
  Transaction,
  TransactionInstruction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import dotenv from "dotenv";

dotenv.config();

const programId = new PublicKey("F5qD9dxkw8ZRZKGQJ6ekNPwGXzesjBKgekQCqf56bMch");
let connection = new Connection("http://127.0.0.1:8899", "confirmed");

function getKeyPair() {
  const secret = JSON.parse(process.env.PRIVATE_KEY ?? "") as number[];
  const secretKey = Uint8Array.from(secret);
  const keypairFromSecretKey = Keypair.fromSecretKey(secretKey);
  return keypairFromSecretKey;
}

async function main() {
  const payer = getKeyPair();
  const transaction = new Transaction();
  const instruction = new TransactionInstruction({
    keys: [],
    programId,
  });
  transaction.add(instruction);
  const transactionSignature = await sendAndConfirmTransaction(
    connection,
    transaction,
    [payer],
  );

  console.log(
    `Transaction: https://explorer.solana.com/tx/${transactionSignature}?cluster=custom`,
  );
}

main()
  .then(() => {
    console.log("Success!");
  })
  .catch((error) => {
    console.error(error);
  });
