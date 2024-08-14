import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RadarHackathonExperiments } from "../target/types/radar_hackathon_experiments";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from "@solana/spl-token";
import * as fs from "fs";
import { PublicKey } from "@solana/web3.js";

export async function createTokenMint(
  provider: anchor.AnchorProvider,
  wallet: anchor.web3.Keypair
) {
  const mint = await createMint(
    provider.connection,
    wallet, //payer
    wallet.publicKey, //mint authoritu
    null, //freeze authority
    6
  );
  return mint;
}

export async function getRequiredATA(
  provider: anchor.AnchorProvider,
  wallet: anchor.web3.Keypair
) {
  const mintAmount = 1000 * 1e6;
  const mint = await createTokenMint(provider, wallet);
  console.log("Token mint: ", mint.toString());

  const ata = (
    await getOrCreateAssociatedTokenAccount(
      provider.connection,
      wallet,
      mint,
      wallet.publicKey,
      false
    )
  ).address;
  await mintTo(
    provider.connection,
    wallet, //fee payer
    mint,
    ata,
    wallet, //mint authority
    mintAmount
  );
  return [mint, ata];
}

export async function loadWallet(
  secretKeyPath: string
): Promise<anchor.web3.Keypair> {
  const secretKey = JSON.parse(fs.readFileSync(secretKeyPath, "utf8"));
  return anchor.web3.Keypair.fromSecretKey(Uint8Array.from(secretKey));
}

describe("radar-hackathon-experiments", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .RadarHackathonExperiments as Program<RadarHackathonExperiments>;
  const secretKeyPath = "/home/jovian/.config/solana/id.json";
  let wallet: anchor.web3.Keypair;
  let provider: anchor.AnchorProvider;
  let mint: PublicKey;
  let ata: PublicKey;

  before(async () => {
    wallet = await loadWallet(secretKeyPath);
    provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const [_mint, _ata] = await getRequiredATA(provider, wallet);
    mint = _mint;
    ata = _ata;
  });

  it("Is initialized!", async () => {
    const tx = await program.methods.initializeVault(new anchor.BN(1)).accounts({
      mint: mint
    }).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Withdraw vault!", async () => {
    const tx = await program.methods
      .withdrawVault()
      .accounts({
        mint: mint,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
