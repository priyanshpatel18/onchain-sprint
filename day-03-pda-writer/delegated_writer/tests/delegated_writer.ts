import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DelegatedWriter } from "../target/types/delegated_writer";

describe("delegated_writer", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.delegatedWriter as Program<DelegatedWriter>;

  const user = provider.wallet.publicKey;
  const writer = anchor.web3.Keypair.generate();

  const [profilePda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("delegated_writer"), user.toBuffer()],
    program.programId
  );

  const baseAccounts = {
    user,
    profile: profilePda,
    systemProgram: anchor.web3.SystemProgram.programId,
  };

  it.skip("Initialize!", async () => {
    const tx = await program.methods
      .initProfile("PATEL")
      .accounts({
        ...baseAccounts,
      })
      .rpc();

    console.log("Your transaction signature", tx);
  });

  it("Delegate!", async () => {
    const tx = await program.methods
      .delegateWriter(writer.publicKey)
      .accounts({
        ...baseAccounts,
        authority: user, // original authority
      })
      .rpc();

    console.log("Your transaction signature", tx);
  });

  it("Write on Behalf", async () => {
    const tx = await program.methods
      .writeOnBehalf("SOLANA")
      .accounts({
        user,                     // still the original user's public key for PDA seed
        profile: profilePda,
        authority: writer.publicKey, // delegate acts as authority
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([writer])
      .rpc();

    console.log("Your transaction signature", tx);
  });
});
