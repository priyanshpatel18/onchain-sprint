import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { FeesBank } from "../target/types/fees_bank";

describe("fees_bank", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.feesBank as Program<FeesBank>;

  const user = anchor.getProvider().wallet;

  const [vault] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), user.publicKey.toBuffer()],
    program.programId
  )

  const [bank] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("bank"), user.publicKey.toBuffer()],
    program.programId
  )

  const [treasury] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("treasury")],
    program.programId
  )

  const amount = new anchor.BN(2 * anchor.web3.LAMPORTS_PER_SOL);

  const accounts = {
    user: user.publicKey,
    vault,
    bank,
    treasury,
    systemProgram: anchor.web3.SystemProgram.programId
  }

  it("Init Bank!", async () => {
    const tx = await program.methods
    .initBank()
    .accounts(accounts)
    .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Deposit", async () => {
    const tx = await program.methods
    .deposit(amount)
    .accounts(accounts)
    .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Withdraw", async () => {
    const tx = await program.methods
    .withdraw(amount)
    .accounts(accounts)
    .rpc();
    console.log("Your transaction signature", tx);
  });
});
