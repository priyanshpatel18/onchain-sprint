import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  createInitializeMintInstruction,
  getAccount,
  getOrCreateAssociatedTokenAccount
} from "@solana/spl-token";
import { assert } from "chai";
import { TokenVault } from "../target/types/token_vault";

describe("token_vault", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.tokenVault as Program<TokenVault>;

  const user = anchor.web3.Keypair.generate();
  const mintKeypair = anchor.web3.Keypair.generate();
  let mint = mintKeypair.publicKey;
  let vault: anchor.web3.PublicKey;
  let vaultAta: any;

  const VAULT_SEED = Buffer.from("vault");
  const DECIMALS = 6;
  const INIT_AMOUNT = new anchor.BN(100 * 10 ** DECIMALS);
  const SEND_AMOUNT = new anchor.BN(50 * 10 ** DECIMALS);

  before("Setup user, mint, vault", async () => {
    // Airdrop some SOL to user
    const sig = await provider.connection.requestAirdrop(
      user.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(sig);

    // Derive PDA for vault
    [vault] = anchor.web3.PublicKey.findProgramAddressSync(
      [VAULT_SEED, user.publicKey.toBuffer()],
      program.programId
    );

    // Create mint where authority is vault PDA
    const tx = new anchor.web3.Transaction().add(
      anchor.web3.SystemProgram.createAccount({
        fromPubkey: provider.wallet.publicKey,
        newAccountPubkey: mint,
        space: 82,
        lamports: await provider.connection.getMinimumBalanceForRentExemption(82),
        programId: anchor.utils.token.TOKEN_PROGRAM_ID,
      }),
      createInitializeMintInstruction(
        mint,
        DECIMALS,
        vault, // mint authority
        null
      )
    );

    await provider.sendAndConfirm(tx, [mintKeypair]);

    // Create vault ATA (owned by vault PDA)
    vaultAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      user,
      mint,
      vault,
      true
    );
  });

  it("Initializes the vault and mints tokens", async () => {
    const tx = await program.methods
      .initVault(INIT_AMOUNT)
      .accounts({
        user: user.publicKey,
        vault,
        mint,
        vaultAta: vaultAta.address,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([user])
      .rpc();

    console.log("✅ Vault initialized:", tx);

    const vaultAtaAccount = await getAccount(provider.connection, vaultAta.address);
    assert.strictEqual(Number(vaultAtaAccount.amount), INIT_AMOUNT.toNumber(), "Vault should be funded");
  });

  it("Withdraws tokens from vault to user ATA", async () => {
    const userAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      user,
      mint,
      user.publicKey
    );

    const tx = await program.methods
      .sendTokens(SEND_AMOUNT)
      .accounts({
        user: user.publicKey,
        vault,
        mint,
        vaultAta: vaultAta.address,
        userAta: userAta.address,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([user])
      .rpc();

    console.log("✅ Tokens withdrawn:", tx);

    const userAtaAccount = await getAccount(provider.connection, userAta.address);
    assert.strictEqual(Number(userAtaAccount.amount), SEND_AMOUNT.toNumber(), "User should receive tokens");

    const vaultAtaAccount = await getAccount(provider.connection, vaultAta.address);
    assert.strictEqual(
      Number(vaultAtaAccount.amount),
      INIT_AMOUNT.toNumber() - SEND_AMOUNT.toNumber(),
      "Vault should be debited"
    );
  });
});
