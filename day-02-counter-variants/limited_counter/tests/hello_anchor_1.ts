import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LimitedCounter } from "../target/types/limited_counter";

describe("limited_counter", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.limitedCounter as Program<LimitedCounter>;

  it("init count", async () => {
    const tx = await program.methods.initCount().rpc();
    console.log("Your transaction signature", tx);
  });

  it("increment count 1", async () => {
    const tx = await program.methods.increment().rpc();
    console.log("Your transaction signature", tx);
  });

  it("increment count 2", async () => {
    const tx = await program.methods.increment().rpc();
    console.log("Your transaction signature", tx);
  });

  it("increment count 3", async () => {
    try {
      const tx = await program.methods.increment().rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
    }
  });

  it("decrement count", async () => {
    const tx = await program.methods.decrement().rpc();
    console.log("Your transaction signature", tx);
  });
});
