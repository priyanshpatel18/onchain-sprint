import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NestedPda } from "../target/types/nested_pda";

describe("nested_pda", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.nestedPda as Program<NestedPda>;

  const index = new anchor.BN(1);

  const [nestedPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("nested"), anchor.getProvider().wallet.publicKey.toBuffer()],
    program.programId
  )

  const [post] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("post"), nestedPda.toBuffer(), index.toArrayLike(Buffer, "le", 8)],
    program.programId
  )

  const accounts = {
    user: anchor.getProvider().wallet.publicKey,
    profile: nestedPda,
    post: post,
    authority: anchor.getProvider().wallet.publicKey,
    systemProgram: anchor.web3.SystemProgram.programId
  }

  it("Init!", async () => {
    const tx = await program.methods
      .initProfile("PATEL")
      .accounts(accounts)
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Post", async () => {
    const tx = await program.methods
      .post(index, "Hello World")
      .accounts(accounts)
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
