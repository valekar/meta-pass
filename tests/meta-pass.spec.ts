import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { assert } from "chai";
import { MetaPass } from "../target/types/meta_pass";
import {
  getEventAccount,
  getMetaPassAccount,
  getOrganizerAccount,
} from "./pda";
import { addSols, CreateEventArgs, EventType } from "./utils";

describe("meta-pass", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.MetaPass as Program<MetaPass>;

  const dummyAuthority = anchor.web3.Keypair.generate();
  const eventAuthority = anchor.web3.Keypair.generate();

  before(async () => {
    await addSols(program.provider, dummyAuthority.publicKey);
    await addSols(program.provider, eventAuthority.publicKey);
  });

  it("should initialize meta-pass!", async () => {
    // Add your test here.

    const [metaPass, _] = await getMetaPassAccount();

    const initializeInstruction = await program.methods
      .initialize({
        convenienceFee: 10,
      })
      .accounts({
        metaPass: metaPass,
        authority: dummyAuthority.publicKey,
        //treasury: dummyAuthority.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .instruction();

    const trans = new anchor.web3.Transaction();
    trans.add(initializeInstruction);

    const tx = await program.provider.send(trans, [dummyAuthority]);

    console.log("Your transaction signature", tx);
  });

  it("should initialize organizer", async () => {
    const seq = new anchor.BN(new Date().getTime() / 1000);

    const [organizerKey, _] = await getOrganizerAccount(
      eventAuthority.publicKey
    );

    const instruction = await program.methods
      .initializeOrganizer({
        seq: seq,
      })
      .accounts({
        organizer: organizerKey,
        eventAuthority: eventAuthority.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .instruction();

    const transaction = new anchor.web3.Transaction();
    transaction.add(instruction);

    await program.provider.send(transaction, [eventAuthority]);
  });

  it("should create event", async () => {
    const seq = new anchor.BN(new Date().getTime() / 1000);

    const [eventAccountKey, _] = await getEventAccount(
      eventAuthority.publicKey,
      seq
    );

    const [organizerKey, _1] = await getOrganizerAccount(
      eventAuthority.publicKey
    );

    const args: CreateEventArgs = {
      name: "fun-event",
      description: "a fun event for fun people",
      startTimestamp: new anchor.BN((new Date().getTime() + 1000000) / 1000),
      totalTickets: 100,
      durationHours: 2,
      eventType: EventType.Online,
      ticketPrice: 1,
      tokenMint: new anchor.web3.PublicKey(
        "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"
      ), // usdc
    };
    const instruction = await program.methods
      .createEvent(args)
      .accounts({
        event: eventAccountKey,
        organizer: organizerKey,
        eventAuthority: eventAuthority.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .instruction();

    const transaction = new anchor.web3.Transaction();
    transaction.add(instruction);

    await program.provider.send(transaction, [eventAuthority]);

    //console.log("Your transaction signature", tx);

    const eventData = await program.account.event.fetch(eventAccountKey);

    assert.isOk(eventData.config.name === "fun-event");
  });
});
