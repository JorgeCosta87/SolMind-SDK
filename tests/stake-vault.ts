import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { StakeVault } from "../target/types/stake_vault";
import {
  TOKEN_PROGRAM_ID,
  getAssociatedTokenAddressSync,
  createAssociatedTokenAccount,
  mintTo,
  createMint,
  getMint, 
  TOKEN_2022_PROGRAM_ID
} from "@solana/spl-token";
import { expect } from "chai";
import { PublicKey } from "@solana/web3.js";

describe("stake-vault", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env(); 
  anchor.setProvider(provider);

  const program = anchor.workspace.StakeVault as Program<StakeVault>;

  const rewardMintKeypair = new anchor.web3.Keypair();

  const admin = provider.wallet


  let depositMint: anchor.web3.PublicKey;

  let depositMintAta: anchor.web3.PublicKey;
  let rewardMintUserAta: anchor.web3.PublicKey;


  const stakeConfig = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("stake_config"), admin.publicKey.toBuffer(),
    ],
    program.programId
  )[0];

  const mint_decimals = 9;
  const mint_amount = 1_000_000_000;

  const metadata = {
    "name": "TANGAROO",
    "symbol": "TAGOO",
    uri: '',
    decimals: mint_decimals,
  };

  const base_yield_rate = new BN(100);
  const lock_durations = [15,30,90];


  it("Preapre test env", async () => {
    depositMint = await createMint(
      provider.connection,
      admin.payer,
      provider.publicKey,
      provider.publicKey,
      9,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID
    );
    console.log("\nDepositMint", depositMint.toBase58());

    depositMintAta = (await createAssociatedTokenAccount(
      provider.connection,
      admin.payer,
      depositMint,
      provider.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID
    ));
    /*
    rewardMintUserAta = (await  createAssociatedTokenAccount(
      provider.connection,
      admin.payer,
      rewardMintKeypair.publicKey,
      provider.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID
    )); 
    */
  });

  it('Initialize stake vault', async () => {
    const tx = await program.methods
      .initialize(metadata, base_yield_rate, lock_durations)
      .accountsPartial({ 
        payer: provider.publicKey,
        admin: provider.publicKey,
        depositMint: depositMint,
        rewardMint: rewardMintKeypair.publicKey,
        stakeConfig: stakeConfig,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([rewardMintKeypair])
      .rpc();
    console.log('Your transaction signature', tx);

    await program.provider.connection.confirmTransaction(tx, 'confirmed');

    const rewardMintInfo = await getMint(
      program.provider.connection,
      rewardMintKeypair.publicKey,
      'confirmed',
      TOKEN_2022_PROGRAM_ID
    );

    console.log("mint: ", rewardMintInfo);

    expect(rewardMintInfo.address.toBase58()).to.equal(rewardMintKeypair.publicKey.toBase58())
    expect(rewardMintInfo.decimals).to.equal(mint_decimals)
    expect(Number(rewardMintInfo.supply)).to.equal(0)
    expect(rewardMintInfo.mintAuthority.toBase58()).to.equal(provider.publicKey.toBase58())
    expect(rewardMintInfo.freezeAuthority.toBase58()).to.equal(provider.publicKey.toBase58())



  });

});
