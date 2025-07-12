import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BasicBank } from "../target/types/basic_bank";

describe("basic_bank", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.basicBank as Program<BasicBank>;

  let tempUser: anchor.web3.Keypair;
  let vault: anchor.web3.PublicKey;
  let bank: anchor.web3.PublicKey;
  let accounts: any;

  const systemProgram = anchor.web3.SystemProgram.programId;

  let amount = new anchor.BN(2 * anchor.web3.LAMPORTS_PER_SOL);
  let withdrawAmount = new anchor.BN(1 * anchor.web3.LAMPORTS_PER_SOL);

  before(async () => {
    // Create a temporary user keypair
    tempUser = anchor.web3.Keypair.generate();

    // Transfer some SOL from provider to tempUser
    const tx = await provider.connection.requestAirdrop(tempUser.publicKey, 5 * anchor.web3.LAMPORTS_PER_SOL);
    await provider.connection.confirmTransaction(tx);

    // Derive PDAs using tempUser
    vault = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), tempUser.publicKey.toBuffer()],
      program.programId
    )[0];

    bank = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("bank"), tempUser.publicKey.toBuffer()],
      program.programId
    )[0];

    accounts = {
      user: tempUser.publicKey,
      vault,
      bank,
      systemProgram,
    };
  });

  it("Init Bank!", async () => {
    const tx = await program.methods
      .initBank()
      .accounts(accounts)
      .signers([tempUser])
      .rpc();
    console.log("Init transaction:", tx);
  });

  it("Deposit!", async () => {
    const tx = await program.methods
      .deposit(amount)
      .accounts(accounts)
      .signers([tempUser])
      .rpc();
    console.log("Deposit transaction:", tx);
  });

  it("Withdraw - 1!", async () => {
    const tx = await program.methods
      .withdraw(withdrawAmount)
      .accounts(accounts)
      .signers([tempUser])
      .rpc();
    console.log("Withdraw transaction:", tx);
  });
  it("Withdraw - 2!", async () => {
    const tx = await program.methods
      .withdraw(withdrawAmount)
      .accounts(accounts)
      .signers([tempUser])
      .rpc();
    console.log("Withdraw transaction:", tx);
  });
  it("Withdraw - 3!", async () => {
    const tx = await program.methods
      .withdraw(withdrawAmount)
      .accounts(accounts)
      .signers([tempUser])
      .rpc();
    console.log("Withdraw transaction:", tx);
  });
});
