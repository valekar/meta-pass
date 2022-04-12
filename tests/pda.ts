import { utils } from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js";
import * as anchor from "@project-serum/anchor";

const META_PASS_PROGRAM_ID = new PublicKey(
  "5TzpQbN8Mec9c3aiX2KbK5jq6dYUnseDjuzzdjhcFZi4"
);

export const getMetaPassAccount = async () => {
  return await PublicKey.findProgramAddress(
    [Buffer.from(utils.bytes.utf8.encode("meta-pass"))],
    META_PASS_PROGRAM_ID
  );
};

export const getEventAccount = async (
  organizer: PublicKey,
  sequence: anchor.BN
) => {
  return await PublicKey.findProgramAddress(
    [
      Buffer.from("meta-pass"),
      organizer.toBuffer(),
      sequence.toBuffer("le", 8),
    ],
    META_PASS_PROGRAM_ID
  );
};

export const getOrganizerAccount = async (organizer: PublicKey) => {
  return await PublicKey.findProgramAddress(
    [Buffer.from("meta-pass"), organizer.toBuffer()],
    META_PASS_PROGRAM_ID
  );
};
