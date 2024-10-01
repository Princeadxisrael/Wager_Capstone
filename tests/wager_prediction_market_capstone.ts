import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { WagerPredictionMarketCapstone } from "../target/types/wager_prediction_market_capstone";
import { Keypair, SystemProgram } from "@solana/web3.js";
import {assert} from "chai"

describe("wager_prediction_market_capstone", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider=anchor.getProvider();
  const connection=provider.connection;
  const program = anchor.workspace.WagerPredictionMarketCapstone as Program<WagerPredictionMarketCapstone>;
  const event = Keypair.generate();
  const housePool = Keypair.generate(); //pool account where liquidity or house funds are held.
  const oracleFeed = Keypair.generate();  // Assuming an oracle feed pubkey for now

  const eventId = new BN(1);
  const description = "Test Event Description";
  const possibleOutcomes = ["Team A Wins", "Team B Wins", "Draw"];
  const odds = [100, 200, 150];  // Example odds for outcomes
  const eventType = { match: {} };  // Example event type, assuming a `match` type
  const startTime = Math.floor(Date.now() / 1000); // Current time as start time (in seconds)
  const endTime = startTime + 3600;  // End time is 1 hour later

  const marketParams = {
    eventId: eventId,
    eventType: eventType,
    description: description,
    possibleOutcomes: possibleOutcomes,
    odds: odds.map(odd => new BN(odd)),
    startTime: new BN(startTime),
    endTime: new BN(endTime),
    oracleFeed: oracleFeed.publicKey,
    housePool: housePool.publicKey,
  };
  it("Create event", async () => {
    const tx= await program.methods.createevent(
      eventId,                        // event_id as u64
      housePool.publicKey,             // house_pool as Pubkey
      marketParams                     // market parameters
    )
    .accounts({
      event: event.publicKey,
      creator: provider.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .signers([event])
    .rpc();

    const eventAccount = await program.account.event.fetch(event.publicKey);
    assert.equal(eventAccount.id.toString(), eventId.toString());
    assert.isTrue(eventAccount.isActive);
    assert.equal(eventAccount.creator.toString(), provider.publicKey.toString());
    assert.equal(eventAccount.description, description);
    assert.deepEqual(eventAccount.possibleOutcomes, possibleOutcomes);
    assert.deepEqual(eventAccount.odds.map(o => o.toString()), odds.map(o => o.toString()));
    assert.equal(eventAccount.startTime.toString(), startTime.toString());
    assert.equal(eventAccount.endTime.toString(), endTime.toString());
  });

  it("Place a bet", async () => {
    const bettor = provider.wallet.publicKey;
    const outcome = new anchor.BN(1); // Bet on the second outcome
    const betAmount = new anchor.BN(1000); // Bet amount of 1000 tokens

    await program.methods.placebet(outcome, betAmount)
      .accounts({
        bettor: bettor,
        event: event.publicKey,
        housePool: housePool.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const eventAccount = await program.account.event.fetch(event.publicKey);
    assert.equal(eventAccount.totalBets[1].toString(), betAmount.toString());
  });
});
