import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AuthorityCounter } from "../target/types/authority_counter";

describe("authority_counter", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.authorityCounter as Program<AuthorityCounter>;

  const provider = anchor.getProvider() as anchor.AnchorProvider; 

  it("Init Count!", async () => {
    const tx = await program.methods.initCount().rpc();
    console.log("Your transaction signature", tx);
  });

  it("Increment Count!", async () => {
    const tx = await program.methods.increment().accounts({
      // @ts-ignore
      authority: provider.publicKey,
    }).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Fail to Increment Count!", async () => {
    let not_authority = new anchor.web3.Keypair();
    try {
      const tx = await program.methods.increment().accounts({
        // @ts-ignore
        authority: not_authority.publicKey,
      }).rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
    }
  });

  it("Decrement Count!", async () => {
    const tx = await program.methods.decrement().accounts({
      // @ts-ignore
      authority: provider.publicKey,
    }).rpc();
    console.log("Your transaction signature", tx);
  });
});
