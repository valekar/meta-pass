import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MetaPass } from "../target/types/meta_pass";
import { getMetaPassAccount } from "./pda";
import { addSols } from "./utils";

describe("meta-pass", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.MetaPass as Program<MetaPass>;

  const dummyAuthority = anchor.web3.Keypair.generate();

  before(async () => {
    addSols(program.provider, dummyAuthority.publicKey);
  });

  it("Is initialized!", async () => {
    // Add your test here.

    const [metaPass, _] = await getMetaPassAccount();

    const initializeInstruction = await program.methods
      .initialize({
        convenienceFee: 10,
      })
      .accounts({
        metaPass: metaPass,
        authority: dummyAuthority.publicKey,
        treasury: dummyAuthority.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .instruction();

    const trans = new anchor.web3.Transaction();
    trans.add(initializeInstruction);

    const tx = program.provider.send(trans, [dummyAuthority]);

    console.log("Your transaction signature", tx);
  });
});
