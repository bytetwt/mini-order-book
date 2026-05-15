import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MiniOrderBook } from "../target/types/mini_order_book";
import { Keypair, PublicKey } from "@solana/web3.js";
import { it } from "mocha";

describe("mini-order-book", async () => {
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.miniOrderBook as Program<MiniOrderBook>;

  let [marketPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("market")],
    program.programId
  );

  it("Is initialized market!", async () => {
    const tx = await program.methods
      .initializeMarket()
      .accountsPartial({
        signer: provider.wallet.publicKey,
        market: marketPda,
      })
      .rpc();

    console.log("Your transaction signature", tx);
  });

  it("Creates an ask order", async () => {
    let market = await program.account.market.fetch(marketPda);
    let askId = market.askCount;
    let [askPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("ask"),
        provider.wallet.publicKey.toBuffer(),
        askId.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    let price = new anchor.BN(100);
    let size = new anchor.BN(100);

    let tx = await program.methods
      .createAsk(price, size)
      .accountsPartial({
        seller: provider.wallet.publicKey,
        market: marketPda,
        ask: askPda,
      })
      .rpc();

    console.log("Your create ask transaction signature", tx);
  });

  it("Creates an bid order", async () => {
    let market = await program.account.market.fetch(marketPda);
    let bidId = market.bidCount;
    let [bidPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("bid"),
        provider.wallet.publicKey.toBuffer(),
        bidId.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    let price = new anchor.BN(100);
    let size = new anchor.BN(100);

    let tx = await program.methods
      .createBid(price, size)
      .accountsPartial({
        buyer: provider.wallet.publicKey,
        market: marketPda,
        bid: bidPda,
      })
      .rpc();

    console.log("Your create bid transaction signature", tx);
  });

  it("Resolves the market", async () => {
    let market = await program.account.market.fetch(marketPda);

    let askId = market.askCount.sub(new anchor.BN(1));
    let [askPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("ask"),
        provider.wallet.publicKey.toBuffer(),
        askId.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    let bidId = market.bidCount.sub(new anchor.BN(1));
    let [bidPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("bid"),
        provider.wallet.publicKey.toBuffer(),
        bidId.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    let tx = await program.methods
      .resolve()
      .accountsPartial({
        market: marketPda,
        ask: askPda,
        bid: bidPda,
        resolver: provider.wallet.publicKey,
      })
      .rpc();

    console.log("Market got solved", tx);
  });
});
