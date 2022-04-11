import { PublicKey } from "@solana/web3.js";

const META_PASS_PROGRAM_ID = new PublicKey(
  "5TzpQbN8Mec9c3aiX2KbK5jq6dYUnseDjuzzdjhcFZi4"
);

export const getMetaPassAccount = async () => {
  return await PublicKey.findProgramAddress(
    [Buffer.from("meta-pass")],
    META_PASS_PROGRAM_ID
  );
};
