import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BasicCounter } from "../target/types/basic_counter";
import { PublicKey, SystemProgram } from "@solana/web3.js";

describe("basic_counter", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.basicCounter as Program<BasicCounter>;

  const [count] = PublicKey.findProgramAddressSync([Buffer.from("count")], program.programId);

  it("Init Counter!", async () => {
    const tx = await program.methods
    .initCount()
    .accounts({
      // @ts-ignore
      count: count,
      systemProgram: SystemProgram.programId
    })
    .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Increment!", async () => {
    const tx = await program.methods
    .increment()
    .accounts({
      // @ts-ignore
      count: count,
      systemProgram: SystemProgram.programId
    })
    .rpc();
    console.log("Your transaction signature", tx);
  });
  it("Init Counter!", async () => {
    const tx = await program.methods
    .decrement()
    .accounts({
      // @ts-ignore
      count: count,
      systemProgram: SystemProgram.programId
    })
    .rpc();
    console.log("Your transaction signature", tx);
  });
});
