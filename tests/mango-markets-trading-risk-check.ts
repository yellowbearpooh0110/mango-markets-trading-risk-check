import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MangoMarketsTradingRiskCheck } from "../target/types/mango_markets_trading_risk_check";

describe("mango-markets-trading-risk-check", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.MangoMarketsTradingRiskCheck as Program<MangoMarketsTradingRiskCheck>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
