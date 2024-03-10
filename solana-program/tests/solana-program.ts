import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaProgram } from "../target/types/solana_program";

describe("solana-program", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaProgram as Program<SolanaProgram>;

  it("Should create a new campaign", async () => {
    // Add your test here.
    const donator = anchor.web3.Keypair.fromSecretKey(new Uint8Array([238,50,8,9,1,20,196,156,131,137,224,113,135,199,28,251,64,222,32,85,23,224,37,161,4,102,32,79,157,41,208,118,173,175,164,45,165,145,1,152,125,87,183,100,33,135,192,48,111,9,219,153,7,181,30,124,82,175,55,106,93,138,39,187]
      ));
    const celeb = anchor.web3.Keypair.generate();


    const tx = await program.methods.transferLamports(new anchor.BN(1000000000)).accounts({from: donator.publicKey, to: celeb.publicKey}).signers([donator]).rpc();
    console.log("Your transaction signature", tx);
  });
});