import { expect } from "chai";

import { createLCDClient } from "../lib/createLCDClient.js";
import { createUser } from "../lib/createUser.js";
import { createOffer } from "../lib/createOffer.js";
import { queryOffers } from "../lib/queryOffers.js";
import { getFactoryCfg } from "../lib/getFactoryCfg.js";
import { createTrade } from "../lib/createTrade.js";
import { disputeTrade } from "../lib/disputeTrade.js";
import { releaseTradeEscrow } from "../lib/releaseTradeEscrow.js";
import { refundTradeEscrow } from "../lib/refundTradeEscrow.js";
import { fundTradeEscrow } from "../lib/fundTradeEscrow.js";
import { fiatDeposited } from "../lib/fiatDeposited.js";
import { before } from "mocha";
(async () => {
  const min_amount = process.env.MIN_AMOUNT;

  const max_amount = process.env.MAX_AMOUNT;

  const terra = await createLCDClient();

  const maker = createUser(terra, process.env.MAKER_MNEMONIC);
  const maker_contact = process.env.MAKER_CONTACT;

  const taker = createUser(terra, process.env.TAKER_MNEMONIC5);
  const taker_contact = process.env.TAKER_CONTACT5;

  const arbitrator = createUser(terra, process.env.ARBITRATOR_MNEMONIC);

  describe("LocalTerra", function () {
    describe("Dispute Resolution", function () {
      describe.only("SELL.SettledForTaker", function () {
        describe("Maker disputes Buy trade -> Arbitrator settles for Taker", function () {
          before(async function () {
            global.factoryCfg = await getFactoryCfg(terra, maker);

            global.tradeFlow = { offerId: undefined };
          });
          it("Maker creates a SELL offer in COP", async function () {
            const offer = {
              offer_type: "sell",
              fiat_currency: "COP",
              min_amount,
              max_amount,
              maker_contact,
            };

            const offerResult = await createOffer(terra, offer, maker);

            return offerResult;
          });

          // Suit variables
          this.tradeAddr = undefined;

          it("Taker lists an offer", async function () {
            const query = {
              offers_query: {
                limit: 1,
                last_value: 0,
              },
            };

            const offers = await queryOffers(terra, query);

            if (offers.length === 0) throw Error("No offers found.");

            global.tradeFlow.offerId = offers[0].id;

            return;
          });

          it("Taker requests a trade (TadeState::RequestCreated)", async function () {
            const new_trade = {
              offer_id: global.tradeFlow.offerId,
              ust_amount: process.env.MIN_AMOUNT,
              taker: taker.address,
              taker_contact,
              // arbitrator:,
            };

            this.tradeAddr = await createTrade(terra, new_trade, taker);

            return this.tradeAddr;
          });

          it("Maker funds the trade escrow (TradeState::EscrowFunded)", async function () {
            await fundTradeEscrow(
              terra,
              {
                amount: process.env.MIN_AMOUNT,
                offerId: global.tradeFlow.offerId,
                tradeAddr: this.tradeAddr,
              },
              maker
            );
          });
          it("Taker clicks `mark paid` (TradeState::FiatDeposited)", async function () {
            await fiatDeposited(
              terra,
              {
                tradeAddr: this.tradeAddr,
              },
              taker
            );
          });

          it("Maker disputes the trade (TadeState::EscrowDisputed)", async function () {
            await disputeTrade(terra, this.tradeAddr, maker);
          });

          it("Arbitrator RELEASES trade escrow (TradeState::SettledForTaker)", async function () {
            await releaseTradeEscrow(terra, this.tradeAddr, arbitrator);
          });

          it("Arbitrator lists assigned trades", async function () {
            const trades_query = {
              user: arbitrator.address,
              index: "arbitrator_state",
              limit: 10,
            };

            const offers = await queryOffers(terra, { trades_query });

            expect(offers.length).to.equal(1);
            expect(offers[0].trade.state).to.equal("settled_for_taker");
          });
        });
      });
      describe("SELL.SettledForMaker", function () {
        describe("Taker disputes Buy trade -> Arbitrator settles for Maker", function () {
          before(async function () {
            global.factoryCfg = await getFactoryCfg(terra, maker);

            global.tradeFlow = { offerId: undefined };
          });
          it("Maker creates a SELL offer in COP", async function () {
            const offer = {
              offer_type: "sell",
              fiat_currency: "COP",
              min_amount,
              max_amount,
              maker_contact,
            };

            const offerResult = await createOffer(terra, offer, maker);

            return offerResult;
          });

          // Suit variables
          this.tradeAddr = undefined;

          it("Taker lists an offer", async function () {
            const query = {
              offers_query: {
                limit: 1,
                last_value: 0,
              },
            };

            const offers = await queryOffers(terra, query);

            if (offers.length === 0) throw Error("No offers found.");

            global.tradeFlow.offerId = offers[0].id;

            return;
          });

          it("Taker requests a trade (TadeState::RequestCreated)", async function () {
            const new_trade = {
              offer_id: global.tradeFlow.offerId,
              ust_amount: process.env.MIN_AMOUNT,
              taker: taker.address,
              taker_contact,
              // arbitrator:,
            };

            this.tradeAddr = await createTrade(terra, new_trade, taker);

            return this.tradeAddr;
          });

          it("Maker funds the trade escrow (TradeState::EscrowFunded)", async function () {
            await fundTradeEscrow(
              terra,
              {
                amount: process.env.MIN_AMOUNT,
                offerId: global.tradeFlow.offerId,
                tradeAddr: this.tradeAddr,
              },
              maker
            );
          });
          it("Taker clicks `mark paid` (TradeState::FiatDeposited)", async function () {
            await fiatDeposited(
              terra,
              {
                tradeAddr: this.tradeAddr,
              },
              taker
            );
          });

          it("Taker disputes the trade (TadeState::EscrowDisputed)", async function () {
            await disputeTrade(terra, this.tradeAddr, taker);
          });

          it("Arbitrator REFUNDS trade escrow (TradeState::SettledForMaker)", async function () {
            await refundTradeEscrow(terra, this.tradeAddr, arbitrator);
          });

          it("Arbitrator lists assigned trades", async function () {
            const trades_query = {
              user: arbitrator.address,
              index: "arbitrator_state",
              limit: 10,
            };

            const offers = await queryOffers(terra, { trades_query });

            expect(offers.length).to.equal(1);
            expect(offers[0].trade.state).to.equal("settled_for_maker");
          });
        });
      });
    });
  });
})();
