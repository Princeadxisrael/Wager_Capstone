import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { WagerPredictionMarketCapstone } from "../target/types/wager_prediction_market_capstone";

describe("wager_prediction_market_capstone", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.WagerPredictionMarketCapstone as Program<WagerPredictionMarketCapstone>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
