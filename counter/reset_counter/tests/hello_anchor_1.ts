import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ResetCounter } from "../target/types/reset_counter";

describe("reset_counter", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.resetCounter as Program<ResetCounter>;

  const provider = anchor.getProvider();
  it("Init Count", async () => {
    const tx = await program.methods.initCount().rpc();
    console.log("Your transaction signature", tx);
  });

  it("Increment Count", async () => {
    const tx = await program.methods.increment().accounts({
      // @ts-ignore
      authority: provider.publicKey,
    }).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Decrement Count", async () => {
    const tx = await program.methods.decrement().accounts({
      // @ts-ignore
      authority: provider.publicKey,
    }).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Reset Count", async () => {
    const tx = await program.methods.reset().accounts({
      // @ts-ignore
      authority: provider.publicKey,
    }).rpc();
    console.log("Your transaction signature", tx);
  });
});
