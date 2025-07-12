import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { P2pTransfer } from "../target/types/p2p_transfer";

describe("p2p_transfer", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.p2pTransfer as Program<P2pTransfer>;

  const initiator = anchor.getProvider().wallet.publicKey;
  const receiver = anchor.web3.Keypair.generate();

  const [escrow] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("escrow"), initiator.toBuffer(), receiver.publicKey.toBuffer()],
    program.programId
  );

  const [vault] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), initiator.toBuffer(), receiver.publicKey.toBuffer()],
    program.programId
  )

  const amount = new anchor.BN(2 * anchor.web3.LAMPORTS_PER_SOL);

  const accounts = {
    initiator,
    receiver: receiver.publicKey,
    escrow,
    vault,
    systemProgram: anchor.web3.SystemProgram.programId
  }

  it("Init Escrow!", async () => {
    const tx = await program.methods
      .initEscrow(amount)
      .accounts(accounts)
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Failed Accept!", async () => {
    const fakeReceiver = anchor.web3.Keypair.generate().publicKey;

    try {
      const tx = await program.methods
        .accept()
        .accounts({
          ...accounts,
          receiver: fakeReceiver
        })
        .rpc();
      throw new Error("Accept should have failed but didn't");
    } catch (_) {
      console.log("Failed Accept!");
    }
  })

  it("Accept Escrow!", async () => {
    const balanceBefore = await program.provider.connection.getBalance(receiver.publicKey);

    const tx = await program.methods
      .accept()
      .accounts(accounts)
      .signers([receiver])
      .rpc()
    console.log("Your transaction signature", tx);
    const balanceAfter = await program.provider.connection.getBalance(receiver.publicKey);
    console.log("Received:", balanceAfter - balanceBefore);
  });
});
