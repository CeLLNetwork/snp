import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Snp } from "../target/types/snp";
import {
  createAllocTreeIx,
  SPL_ACCOUNT_COMPRESSION_PROGRAM_ID,
  SPL_NOOP_PROGRAM_ID,
} from "@solana/spl-account-compression";
import {
  PROGRAM_ID as BUBBLEGUM_PROGRAM_ID,
} from "@metaplex-foundation/mpl-bubblegum";
import * as web3 from "@solana/web3.js";
import * as spl from "@solana/spl-token";
import {
  execTx,
  findPda,
} from "./util";


describe("snp", () => {
  const provider = anchor.AnchorProvider.env();
  const program = anchor.workspace.Snp as Program<Snp>;
  const authority = provider.wallet as anchor.Wallet;

  // tree init size ************************************
  // data from direct url:  https://compressed.app/
  const merkleTree = web3.Keypair.generate();
  let config;
  let config_bump;

  const [treeConfig, treeConfigBump] = web3.PublicKey.findProgramAddressSync(
    [merkleTree.publicKey.toBuffer()],
    BUBBLEGUM_PROGRAM_ID
  );

  [config, config_bump] = findPda([
    Buffer.from("cNFT-config"),
    treeConfig.toBuffer(),
  ]);
  console.log("config: ", config.toBase58());
  console.log("config_bump: ", config_bump);

  it("Initialize", async () => {
    let initializeIx = await program.methods.initialize()
      .accounts({
        verifier: authority.publicKey,
        config: config,
        treeConfig: treeConfig,
        systemProgram: web3.SystemProgram.programId
      })
      .instruction()
    await execTx([initializeIx], [authority.payer]);
  });


  it("Add a tree", async () => {
    const allocTreeIx = await createAllocTreeIx(
      provider.connection,
      merkleTree.publicKey,
      authority.publicKey,// signerPubkey,
      { maxDepth: 24, maxBufferSize: 64 },
      14,
    );
    await execTx([allocTreeIx], [authority.payer, merkleTree]);

    const addTreeIx = await program.methods.addTree()
      .accounts({
        config: config,
        merkleTree: merkleTree.publicKey,
        treeConfig: treeConfig,
        bubblegumProgram: BUBBLEGUM_PROGRAM_ID,
        systemProgram: web3.SystemProgram.programId,
        logWrapper: SPL_NOOP_PROGRAM_ID,
        compressionProgram: SPL_ACCOUNT_COMPRESSION_PROGRAM_ID
      })
      .instruction();
    await execTx([addTreeIx], [authority.payer, merkleTree]);

  });

  it("Mint cMFT", async () => {
    let mintcNftTx = await program.methods.mintCnft(
      { uri: "" }
    )
      .accounts({
        config: config,
        treeConfig: treeConfig,
        merkleTree: merkleTree.publicKey,
        mplBubblegumProgram: BUBBLEGUM_PROGRAM_ID,
        logWrapper: SPL_NOOP_PROGRAM_ID,
        compressionProgram: SPL_ACCOUNT_COMPRESSION_PROGRAM_ID,
        systemProgram: web3.SystemProgram.programId
      })
      .instruction();
    await execTx([mintcNftTx], [authority.payer]);

  });



});

