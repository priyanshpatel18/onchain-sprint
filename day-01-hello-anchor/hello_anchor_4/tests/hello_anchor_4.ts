import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { HelloAnchor4 } from "../target/types/hello_anchor_4";

describe("hello_anchor_4", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.helloAnchor4 as Program<HelloAnchor4>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.sayHello().rpc();
    console.log("Your transaction signature", tx);
  });
});
