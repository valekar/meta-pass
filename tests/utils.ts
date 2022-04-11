import { Provider } from "@project-serum/anchor";
import { PublicKey, LAMPORTS_PER_SOL } from "@solana/web3.js";

export const addSols = async (
  provider: Provider,
  wallet: PublicKey,
  amount = 1 * LAMPORTS_PER_SOL
) => {
  await provider.connection.confirmTransaction(
    await provider.connection.requestAirdrop(wallet, amount),
    "confirmed"
  );
};
