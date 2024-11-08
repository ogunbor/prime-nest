import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PrimeNest } from "../target/types/prime_nest";

describe("prime_nest", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.PrimeNest as Program<PrimeNest>;

 
  const lockDuration = 30 * 24 * 60 * 60; 
  const minDeposit = new anchor.BN(0.2 * 1e9); 

  it("Initializes the vault and makes a deposit", async () => {
    try {
      const tx = await program.methods
        .vaultInitDeposit(new anchor.BN(lockDuration), minDeposit)
        .rpc();
      console.log("Vault initialization and deposit transaction signature:", tx);
    } catch (error) {
      console.error("Error during atomic vault initialization and deposit:", error);
    }
  });
});
