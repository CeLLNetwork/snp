import * as web3 from "@solana/web3.js";
import * as spl from '@solana/spl-token';
import * as anchor from "@coral-xyz/anchor";
import { Snp } from "../target/types/snp";

const program = anchor.workspace.Snp as anchor.Program<Snp>;
const provider = anchor.AnchorProvider.env()
const connection = provider.connection;


export function findPdaWithProgramId(seeds: Array<Buffer | Uint8Array>, programId: web3.PublicKey) {
	// return [publicKey, bump]
	return web3.PublicKey.findProgramAddressSync(seeds, programId)[0];
}


export function findPda(seeds: Array<Buffer | Uint8Array>) {
	// return [publicKey, bump]
	return web3.PublicKey.findProgramAddressSync(seeds, program.programId);
}


export async function execTx(txArr, signer) {
	const txs = new web3.Transaction()
	txArr.forEach(tx => {
		txs.add(tx)
	});

	const txHash = await provider.sendAndConfirm(txs, signer)
	printTx(txHash)
}


export function printTx(tx) {
	console.log(
		"\nTx: ",
		`https://explorer.solana.com/tx/${tx}?cluster=custom&customUrl=http%3A%2F%2Flocalhost%3A8899#ix-1`
	);
}

export async function fetchConfig() {
	return await program.account.config.all();
}

