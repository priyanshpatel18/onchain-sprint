import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { HelloAnchor6 } from "../target/types/hello_anchor_6";

describe("hello_anchor_6", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.helloAnchor6 as Program<HelloAnchor6>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.sayHello().rpc();
    console.log("Your transaction signature", tx);
  });
});
