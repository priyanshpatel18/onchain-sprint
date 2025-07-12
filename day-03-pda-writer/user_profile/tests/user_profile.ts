import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { UserProfile } from "../target/types/user_profile";

describe("user_profile", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.userProfile as Program<UserProfile>;

  const [profilePda, _] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("profile"), program.provider.wallet.publicKey.toBuffer()],
    program.programId
  )

  const accounts = {
    user: program.provider.wallet.publicKey,
    authority: program.provider.wallet.publicKey,
    profile: profilePda
  }

  it("Init Profile!", async () => {
    const tx = await program.methods
    .initProfile("Priyansh","Solana")
    .accounts({ ...accounts})
    .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Update Profile!", async () => {
    const tx = await program.methods
    .updateProfile("Priyansh","Solana")
    .accounts({ ...accounts})
    .rpc();
    console.log("Your transaction signature", tx);
  });
});
