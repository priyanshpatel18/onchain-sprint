import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BasicCounter2 } from "../target/types/basic_counter_2";
import { PublicKey } from "@solana/web3.js";

describe("basic_counter_2", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.basicCounter2 as Program<BasicCounter2>;

  const [count] = PublicKey.findProgramAddressSync([Buffer.from("count")], program.programId);

  it("Init Count!", async () => {
    const tx = await program.methods.initCount().rpc();
    console.log("Your transaction signature", tx);
  });

  it("Increment Count!", async () => {
    const tx = await program.methods.increment().rpc();
    console.log("Your transaction signature", tx);
  });

  it("Decrement Count!", async () => {
    const tx = await program.methods.decrement().rpc();
    console.log("Your transaction signature", tx);
  });
});
