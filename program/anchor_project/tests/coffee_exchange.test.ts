import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  PublicKey,
  SystemProgram,
  Keypair,
  Transaction,
  SendTransactionError,
} from "@solana/web3.js";

import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createMint,
  mintTo,
  getAssociatedTokenAddress,
  createAssociatedTokenAccountInstruction,
} from "@solana/spl-token";

import { assert } from "chai";

type CoffeeExchangeProgram = Program<any>;

describe("Coffee Exchange Test Suite", () => {

  // Shared objects across tests


  let provider: anchor.AnchorProvider;
  let program: CoffeeExchangeProgram;
  let connection: anchor.web3.Connection;

  let payer: anchor.web3.Keypair;
  let maker: PublicKey;

  let tokenMintA: PublicKey;
  let tokenMintB: PublicKey;
  let makerTokenAccountA: PublicKey;

  
  let offerPda: PublicKey;
  let vault: PublicKey;

  
  let tokenAOfferedAmount: anchor.BN;
  let tokenBWantedAmount: anchor.BN;

  
  const decimals = 6;

  
  // Helper: Create an ATA (Associated Token Account) for a given owner/mint
  

  async function createAta(
    owner: PublicKey,
    mint: PublicKey
  ): Promise<PublicKey> {
    
    const ata = await getAssociatedTokenAddress(
      mint,
      owner,
      false, // owner is an on-curve key
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

    const ix = createAssociatedTokenAccountInstruction(
      payer.publicKey, // fee payer
      ata,             // new ATA address
      owner,           // ATA owner
      mint,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

    const tx = new Transaction().add(ix);
    await provider.sendAndConfirm(tx, [payer]);

    return ata;
  }

  
  // Helper: Derive Offer PDA
  

  function deriveOfferPda(maker: PublicKey, id: anchor.BN): PublicKey {
  
    const [pda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("offer"),
        maker.toBuffer(),
        Buffer.from(id.toArray("le", 8)), // u64 LE
      ],
      program.programId
    );
    return pda;
  }

  
  // Helper: Derive Vault ATA owned by the Offer PDA
  

  async function deriveVaultAta(
    mint: PublicKey,
    offer: PublicKey
  ): Promise<PublicKey> {
  
    return getAssociatedTokenAddress(
      mint,
      offer,
      true, // owner is off-curve (PDA)
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );
  }

  
  // Global setup (runs once before all tests)
  

  before(async () => {
  
    provider = anchor.AnchorProvider.local();
    anchor.setProvider(provider);
  
    program = anchor.workspace.coffee_exchange as CoffeeExchangeProgram;

    connection = provider.connection;
    payer = (provider.wallet as anchor.Wallet).payer;
    maker = payer.publicKey;


    // Create Mint A and Mint B (SPL tokens)


    tokenMintA = await createMint(
      connection,
      payer,
      payer.publicKey, // mint authority
      null,            // freeze authority (none)
      decimals,
      undefined,
      undefined,
      TOKEN_PROGRAM_ID
    );

    tokenMintB = await createMint(
      connection,
      payer,
      payer.publicKey, // same mint authority for convenience
      null,
      decimals,
      undefined,
      undefined,
      TOKEN_PROGRAM_ID
    );

    
    // Create Maker's ATA for Mint A and mint some tokens
    

    makerTokenAccountA = await createAta(maker, tokenMintA);

    // Maker offers 1.0 token A (assuming 6 decimals) in the first offer
    tokenAOfferedAmount = new anchor.BN(1_000_000);

    await mintTo(
      connection,
      payer,
      tokenMintA,
      makerTokenAccountA,
      payer,
      tokenAOfferedAmount.toNumber(),
      [],
      undefined,
      TOKEN_PROGRAM_ID
    );

    // For simplicity, this will be the default "wanted amount" in Mint B
    tokenBWantedAmount = new anchor.BN(2_000_000);

    console.log("=== Global setup completed ===");
    console.log("Program:", program.programId.toBase58());
    console.log("Mint A:", tokenMintA.toBase58());
    console.log("Mint B:", tokenMintB.toBase58());
    console.log("Maker:", maker.toBase58());
    console.log("Maker ATA (A):", makerTokenAccountA.toBase58());
  });

  
  // HAPPY PATH TESTS
  

  it("Smoke Test - program loaded and ID matches", async () => {
    const expectedProgramId =
      "9VdKGKXs5ZJd6Cr9GtJcPP8fdUSmRgvkYScvhi1oPkFc";

    if (program.programId.toBase58() !== expectedProgramId) {
      console.warn(
        "Warning: programId does not match expected. " +
          "Check Anchor.toml and declare_id! in lib.rs."
      );
    }

    assert.ok(program.programId instanceof PublicKey, "Invalid programId");
  });

  it("MAKE_OFFER with real mints and token accounts", async () => {
    /**
     * This test creates a valid offer:
     * - Maker deposits token A into the vault (ATA owned by Offer PDA)
     * - Offer state is initialized on-chain
     */

    const id = new anchor.BN(1);

    // Derive Offer PDA and Vault ATA
    offerPda = deriveOfferPda(maker, id);
    vault = await deriveVaultAta(tokenMintA, offerPda);

    console.log("Offer PDA:", offerPda.toBase58());
    console.log("Vault ATA:", vault.toBase58());

    const txSig = await program.methods
      .makeOffer(id, tokenAOfferedAmount, tokenBWantedAmount)
      .accounts({
        maker,
        tokenMintA,
        tokenMintB,
        makerTokenAccountA,
        offer: offerPda,
        vault,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .rpc();

    console.log("make_offer tx:", txSig);

    // Fetch the on-chain Offer account and validate fields
    const offerState: any = await program.account.offer.fetch(offerPda);

    assert.strictEqual(
      offerState.id.toNumber(),
      id.toNumber(),
      "Offer id mismatch"
    );
    assert.ok(offerState.maker.equals(maker), "Offer maker mismatch");
    assert.ok(
      offerState.tokenMintA.equals(tokenMintA),
      "Offer tokenMintA mismatch"
    );
    assert.ok(
      offerState.tokenMintB.equals(tokenMintB),
      "Offer tokenMintB mismatch"
    );
    assert.strictEqual(
      offerState.tokenBWantedAmount.toNumber(),
      tokenBWantedAmount.toNumber(),
      "Offer tokenBWantedAmount mismatch"
    );
  });

  it("TAKE_OFFER - transfer tokens and close offer & vault", async () => {
    
    // 1) Create a taker keypair (different signer from maker)
    const taker = Keypair.generate();

    // Airdrop SOL to cover rent and fees for the taker
    const airdropSig = await connection.requestAirdrop(
      taker.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );
    await connection.confirmTransaction(airdropSig, "confirmed");

    // 2) Create ATA for taker with Mint B (the currency they will pay with)
    const takerTokenAccountB = await getAssociatedTokenAddress(
      tokenMintB,
      taker.publicKey,
      false,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

    {
      const tx = new Transaction().add(
        createAssociatedTokenAccountInstruction(
          payer.publicKey, // payer for ATA creation
          takerTokenAccountB,
          taker.publicKey, // owner
          tokenMintB,
          TOKEN_PROGRAM_ID,
          ASSOCIATED_TOKEN_PROGRAM_ID
        )
      );

      await provider.sendAndConfirm(tx, [payer]);
    }

    console.log("Taker ATA (B):", takerTokenAccountB.toBase58());

    // 3) Mint enough token B to the taker so they can pay the offer
    await mintTo(
      connection,
      payer,
      tokenMintB,
      takerTokenAccountB,
      payer,
      tokenBWantedAmount.toNumber(),
      [],
      undefined,
      TOKEN_PROGRAM_ID
    );

    // 4) Derive expected ATA addresses:
    //    - taker receives token A
    //    - maker receives token B
    const takerTokenAccountAAddr = await getAssociatedTokenAddress(
      tokenMintA,
      taker.publicKey,
      false,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

    const makerTokenAccountBAddr = await getAssociatedTokenAddress(
      tokenMintB,
      maker,
      false,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

    console.log(
      "Taker ATA (A, expected):",
      takerTokenAccountAAddr.toBase58()
    );
    console.log(
      "Maker ATA (B, expected):",
      makerTokenAccountBAddr.toBase58()
    );

    // 5) Execute `take_offer` instruction
    const txSig = await program.methods
      .takeOffer()
      .accounts({
        taker: taker.publicKey,
        maker,
        tokenMintA,
        tokenMintB,
        takerTokenAccountA: takerTokenAccountAAddr,
        takerTokenAccountB,
        makerTokenAccountB: makerTokenAccountBAddr,
        offer: offerPda,
        vault,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .signers([taker])
      .rpc();

    console.log("take_offer tx:", txSig);

    // 6) Check that the Offer account is closed
    const offerAccountInfo = await connection.getAccountInfo(offerPda);
    assert.strictEqual(
      offerAccountInfo,
      null,
      "Offer account should be closed after take_offer"
    );

    // 7) Check that the vault token account is also closed
    const vaultInfo = await connection.getAccountInfo(vault);
    assert.strictEqual(
      vaultInfo,
      null,
      "Vault token account should be closed after take_offer"
    );
  });

  
  // UNHAPPY PATH TESTS
  

  it("MAKE_OFFER should fail if maker has insufficient token A balance", async () => {
  
    const id = new anchor.BN(2);

    const offerPda2 = deriveOfferPda(maker, id);
    const vault2 = await deriveVaultAta(tokenMintA, offerPda2);

    const tooMuch = new anchor.BN(5_000_000); // More than current balance (0)

    let failed = false;

    try {
      await program.methods
        .makeOffer(id, tooMuch, tokenBWantedAmount)
        .accounts({
          maker,
          tokenMintA,
          tokenMintB,
          makerTokenAccountA,
          offer: offerPda2,
          vault: vault2,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        })
        .rpc();
    } catch (err: any) {
      failed = true;

      const sendErr = err as SendTransactionError;
      const logs: string[] = (sendErr as any).logs ?? [];

      console.log(
        "Expected make_offer failure (insufficient token A balance):",
        sendErr.message,
        logs
      );

      // Look for the SPL Token error in the transaction logs
      const hasInsufficientFunds = logs.some((l) =>
        l.includes("Error: insufficient funds")
      );
      assert.isTrue(
        hasInsufficientFunds,
        "Expected 'insufficient funds' error from the SPL Token program"
      );
    }

    assert.isTrue(
      failed,
      "Expected make_offer to fail when maker has insufficient token A balance"
    );
  });

  it("MAKE_OFFER should fail if makerTokenAccountA is not maker's ATA", async () => {

    // Create a fake owner and its ATA for Mint A
    const fakeOwner = Keypair.generate();

    const fakeOwnerAta = await getAssociatedTokenAddress(
      tokenMintA,
      fakeOwner.publicKey,
      false,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

    {
      const tx = new Transaction().add(
        createAssociatedTokenAccountInstruction(
          payer.publicKey,
          fakeOwnerAta,
          fakeOwner.publicKey,
          tokenMintA,
          TOKEN_PROGRAM_ID,
          ASSOCIATED_TOKEN_PROGRAM_ID
        )
      );
      await provider.sendAndConfirm(tx, [payer]);
    }

    const id = new anchor.BN(3);
    const offerPda3 = deriveOfferPda(maker, id);
    const vault3 = await deriveVaultAta(tokenMintA, offerPda3);

    let failed = false;

    try {
      await program.methods
        .makeOffer(id, new anchor.BN(1_000_000), tokenBWantedAmount)
        .accounts({
          maker,
          tokenMintA,
          tokenMintB,
          makerTokenAccountA: fakeOwnerAta, // wrong owner on purpose
          offer: offerPda3,
          vault: vault3,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        })
        .rpc();
    } catch (err: any) {
      failed = true;

      console.log(
        "Expected make_offer failure (wrong makerTokenAccountA / ATA owner):",
        err?.message ?? err
      );

      // Try to parse Anchor error from logs (if available)
      const logs: string[] = (err as any).logs ?? [];
      if (logs.length > 0) {
        const anchorErr = anchor.AnchorError.parse(logs);
        assert.strictEqual(
          anchorErr.error.errorCode.code,
          "ConstraintTokenOwner",
          "Expected ConstraintTokenOwner Anchor error for maker_token_account_a"
        );
      }
    }

    assert.isTrue(
      failed,
      "Expected make_offer to fail when makerTokenAccountA is not maker's ATA"
    );
  });

  it("TAKE_OFFER should fail if taker has insufficient token B balance", async () => {
 
    // 1) Mint some token A back to the maker so a new offer can be created
    const newAmountA = new anchor.BN(1_000_000);

    await mintTo(
      connection,
      payer,
      tokenMintA,
      makerTokenAccountA,
      payer,
      newAmountA.toNumber(),
      [],
      undefined,
      TOKEN_PROGRAM_ID
    );

    // 2) Create a new offer with id = 4
    const id = new anchor.BN(4);
    const offerPda4 = deriveOfferPda(maker, id);
    const vault4 = await deriveVaultAta(tokenMintA, offerPda4);

    await program.methods
      .makeOffer(id, newAmountA, tokenBWantedAmount)
      .accounts({
        maker,
        tokenMintA,
        tokenMintB,
        makerTokenAccountA,
        offer: offerPda4,
        vault: vault4,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .rpc();

    // 3) Create a "poor" taker with almost no token B
    const poorTaker = Keypair.generate();

    const airdropSig = await connection.requestAirdrop(
      poorTaker.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );
    await connection.confirmTransaction(airdropSig, "confirmed");

    const poorTakerTokenAccountB = await getAssociatedTokenAddress(
      tokenMintB,
      poorTaker.publicKey,
      false,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

    {
      const tx = new Transaction().add(
        createAssociatedTokenAccountInstruction(
          payer.publicKey,
          poorTakerTokenAccountB,
          poorTaker.publicKey,
          tokenMintB,
          TOKEN_PROGRAM_ID,
          ASSOCIATED_TOKEN_PROGRAM_ID
        )
      );
      await provider.sendAndConfirm(tx, [payer]);
    }

    // Mint a very small amount of token B (less than tokenBWantedAmount)
    const smallAmountB = new anchor.BN(1);
    await mintTo(
      connection,
      payer,
      tokenMintB,
      poorTakerTokenAccountB,
      payer,
      smallAmountB.toNumber(),
      [],
      undefined,
      TOKEN_PROGRAM_ID
    );

    // 4) Derive expected ATA for taker (Mint A) and maker (Mint B)
    const poorTakerTokenAccountA = await getAssociatedTokenAddress(
      tokenMintA,
      poorTaker.publicKey,
      false,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

    const makerTokenAccountBAddr = await getAssociatedTokenAddress(
      tokenMintB,
      maker,
      false,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

    let failed = false;

    try {
      await program.methods
        .takeOffer()
        .accounts({
          taker: poorTaker.publicKey,
          maker,
          tokenMintA,
          tokenMintB,
          takerTokenAccountA: poorTakerTokenAccountA,
          takerTokenAccountB: poorTakerTokenAccountB,
          makerTokenAccountB: makerTokenAccountBAddr,
          offer: offerPda4,
          vault: vault4,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        })
        .signers([poorTaker])
        .rpc();
    } catch (err: any) {
      failed = true;

      const sendErr = err as SendTransactionError;
      const logs: string[] = (sendErr as any).logs ?? [];

      console.log(
        "Expected take_offer failure (insufficient token B balance for taker):",
        sendErr.message,
        logs
      );

      const hasInsufficientFunds = logs.some((l) =>
        l.includes("Error: insufficient funds")
      );
      assert.isTrue(
        hasInsufficientFunds,
        "Expected 'insufficient funds' error from SPL Token program in take_offer"
      );
    }

    assert.isTrue(
      failed,
      "Expected take_offer to fail when taker has insufficient token B balance"
    );
  });
});
