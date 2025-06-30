import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TimeBasedCounter } from "../target/types/time_based_counter";

describe("time_based_counter", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.TimeBasedCounter as Program<TimeBasedCounter>;

  const provider = anchor.getProvider();
  const wallet = provider.wallet;

  // Derive PDA (same seed as in Rust: [b"count"])
  const [countPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("count")],
    program.programId
  );

  it("Init count!", async () => {
    try {
      await program.methods
        .initCount()
        .accounts({
          // @ts-ignore
          count: countPda,
          signer: wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
      console.log("✅ Count initialized.");
    } catch (err: any) {
      if (err.message.includes("already in use")) {
        console.log("ℹ️ Count already initialized, skipping.");
      } else {
        throw err;
      }
    }
  });

  it("First increment should succeed", async () => {
    console.log("⏳ Waiting 10s to start fresh...");
    await new Promise((res) => setTimeout(res, 10000)); // ensure cooldown is met

    const tx = await program.methods
      .increment()
      .accounts({
        // @ts-ignore
        count: countPda,
        signer: wallet.publicKey
      })
      .rpc();
    console.log("✅ Increment #1 TX:", tx);
  });

  it("Second increment immediately should fail (cooldown)", async () => {
    try {
      await program.methods
        .increment()
        .accounts({
          // @ts-ignore
          count: countPda,
          signer: wallet.publicKey
        })
        .rpc();
      throw new Error("❌ Expected cooldown error, but succeeded");
    } catch (err: any) {
      const errMsg = err?.message || "";
      console.log("⚠️ Cooldown error caught as expected.");
      if (!errMsg.includes("CooldownNotMet")) {
        console.error("Unexpected error:", errMsg);
        throw err;
      }
    }
  });

  it("Third increment after 10s should succeed", async () => {
    console.log("⏳ Waiting 10 seconds...");
    await new Promise((res) => setTimeout(res, 10000));

    const tx = await program.methods
      .increment()
      .accounts({
        // @ts-ignore
        count: countPda,
        signer: wallet.publicKey
      })
      .rpc();
    console.log("✅ Increment #2 TX:", tx);
  });

  it("Decrement (after cooldown) should succeed", async () => {
    console.log("⏳ Waiting 10 seconds for decrement cooldown...");
    await new Promise((res) => setTimeout(res, 10000));

    const tx = await program.methods
      .decrement()
      .accounts({
        // @ts-ignore
        count: countPda,
        signer: wallet.publicKey
      })
      .rpc();
    console.log("✅ Decrement TX:", tx);
  });
});
