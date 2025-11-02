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
  getAccount,
  TOKEN_2022_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID
} from "@solana/spl-token";
import { expect } from "chai";
import { PublicKey } from "@solana/web3.js";

describe("stake-vault", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env(); 
  anchor.setProvider(provider);

  const program = anchor.workspace.StakeVault as Program<StakeVault>;

  const admin = provider.wallet
  const userKeypair = anchor.web3.Keypair.fromSeed(
    Uint8Array.from(Buffer.from("CQB35H179dvx8ADpLUiWkf45XWfydGMT"))
  );

  const rewardMintKeypair = new anchor.web3.Keypair();

  let depositMint: anchor.web3.PublicKey;
  let depositMintAta: anchor.web3.PublicKey;
  let rewardMintUserAta: anchor.web3.PublicKey;
  let stakeVault: anchor.web3.PublicKey;


  const stakeConfigPDA = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("stake_config"), admin.publicKey.toBuffer(),
    ],
    program.programId
  )[0];
  const stakePositionPDA = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("stake_position"), stakeConfigPDA.toBuffer(), userKeypair.publicKey.toBuffer(),
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

  const baseYieldRate = new BN(100);
  const lockDurations = [new BN(15), new BN(30), new BN(90)];
  const lockDuration = new BN(30);
  const stakeAmount = new BN(1_000_000_000);

  it("Request airdrop to taker.", async () => {

  });

  it("Preapre test env", async () => {
    const signature = await provider.connection.requestAirdrop(userKeypair.publicKey, 1_000_000_000);
    await provider.connection.confirmTransaction(signature);
    console.log("\nAirdrop to taker successful - Signature", signature);

    depositMint = await createMint(
      provider.connection,
      admin.payer,
      admin.publicKey,
      admin.publicKey,
      9,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID
    );
    console.log("\nDepositMint", depositMint.toBase58());

    stakeVault = getAssociatedTokenAddressSync(depositMint, stakePositionPDA, true, TOKEN_2022_PROGRAM_ID);

    depositMintAta = (await createAssociatedTokenAccount(
      provider.connection,
      userKeypair,
      depositMint,
      userKeypair.publicKey,
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

    await mintTo(
      provider.connection, userKeypair, depositMint, depositMintAta, admin.payer, mint_amount, undefined, undefined, TOKEN_2022_PROGRAM_ID
    );
    console.log("Tokens minted to user ata for deposit", depositMintAta.toBase58());
    
  });

  it('Initialize stake vault', async () => {
    const tx = await program.methods
      .initialize(metadata, baseYieldRate, lockDurations)
      .accountsPartial({ 
        payer: provider.publicKey,
        admin: provider.publicKey,
        depositMint: depositMint,
        rewardMint: rewardMintKeypair.publicKey,
        stakeConfig: stakeConfigPDA,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([rewardMintKeypair])
      .rpc();
    console.log('Your transaction signature', tx);

    await program.provider.connection.confirmTransaction(tx, 'confirmed');
    
    const stakeConfig = await program.account.stakeConfig.fetch(stakeConfigPDA);

    expect(stakeConfig.admin.toBase58()).to.equal(admin.publicKey.toBase58())
    expect(Number(stakeConfig.baseYieldRate)).to.equal(Number(baseYieldRate))
    expect(Number(stakeConfig.totalStaked)).to.equal(0)
    expect(Number(stakeConfig.totalPositions)).to.equal(0)
    expect(stakeConfig.depositMint.toBase58()).to.equal(depositMint.toBase58())
    expect(stakeConfig.rewardMint.toBase58()).to.equal(rewardMintKeypair.publicKey.toBase58())

    const rewardMintInfo = await getMint(
      program.provider.connection,
      rewardMintKeypair.publicKey,
      'confirmed',
      TOKEN_2022_PROGRAM_ID
    );

    expect(rewardMintInfo.address.toBase58()).to.equal(rewardMintKeypair.publicKey.toBase58())
    expect(rewardMintInfo.decimals).to.equal(mint_decimals)
    expect(Number(rewardMintInfo.supply)).to.equal(0)
    expect(rewardMintInfo.mintAuthority.toBase58()).to.equal(provider.publicKey.toBase58())
    expect(rewardMintInfo.freezeAuthority.toBase58()).to.equal(provider.publicKey.toBase58())
  });


  it('Stake tokens', async () => {
    const tx = await program.methods
      .stakeTokens(stakeAmount, lockDuration)
      .accountsPartial({ 
        user: userKeypair.publicKey,
        depositMint: depositMint,
        depositMintAta: depositMintAta,
        stakeConfig: stakeConfigPDA,
        stakePosition: stakePositionPDA,
        vault: stakeVault,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .transaction();

    const signature = await anchor.web3.sendAndConfirmTransaction(
      provider.connection,
      tx,
      [userKeypair]
    );
    console.log('Your transaction signature', signature);
    await program.provider.connection.confirmTransaction(signature, 'confirmed');

    const stakeConfig = await program.account.stakeConfig.fetch(stakeConfigPDA);
    const stakePosition = await program.account.stakingPosition.fetch(stakePositionPDA);
    const stakeVaultAccount = await getAccount(
      provider.connection,
      stakeVault,
      'confirmed',
      TOKEN_2022_PROGRAM_ID
    );

    expect(Number(stakeConfig.totalStaked)).to.equal(Number(stakeAmount))
    expect(Number(stakeConfig.totalPositions)).to.equal(1)
    expect(Number(stakePosition.amountStaked)).to.equal(Number(stakeAmount))
    expect(Number(stakeVaultAccount.amount)).to.equal(Number(stakeAmount));
  });
});
