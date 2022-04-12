import { Provider } from "@project-serum/anchor";
import { PublicKey, LAMPORTS_PER_SOL } from "@solana/web3.js";
import * as anchor from "@project-serum/anchor";
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

export interface CreateEventArgs {
  name: String;
  description: String;
  startTimestamp: anchor.BN;
  totalTickets: number;
  durationHours: number;
  eventType: any;
  ticketPrice: number;
  tokenMint: PublicKey;
}

// export enum EventType {
//   Online,
//   Offline,
// }

export const EventType = {
  Online: { online: {} },
  Offline: { offline: {} },
};
