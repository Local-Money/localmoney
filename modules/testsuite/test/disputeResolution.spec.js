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
      before(async function () {
        global.factoryCfg = await getFactoryCfg(terra, maker);

        global.disputeResolution = { offerId: undefined };
      });
      describe("Setup", function () {
        it("Maker creates a BUY offer in COP", async function () {
          const offer = {
            offer_type: "buy",
            fiat_currency: "COP",
            min_amount,
            max_amount,
            maker_contact,
          };

          const offerResult = await createOffer(terra, offer, maker);

          return offerResult;
        });
      });
      describe("Refund Escrow: Taker disputes Buy trade, Arbitrator settles for Taker", function () {
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

          global.disputeResolution.offerId = offers[0].id;

          return;
        });

        it("Taker creates a trade", async function () {
          const new_trade = {
            offer_id: parseInt(global.disputeResolution.offerId),
            ust_amount: process.env.MIN_AMOUNT,
            taker: taker.address,
            taker_contact,
            arbitrator: arbitrator.address,
          };

          this.tradeAddr = await createTrade(terra, new_trade, taker);

          return this.tradeAddr;
        });

        it("Maker funds the trade escrow.", async function () {
          await fundTradeEscrow(
            terra,
            {
              offerId: global.disputeResolution.offerId,
              tradeAddr: this.tradeAddr,
            },
            maker
          );
        });

        it("Taker disputes the trade", async function () {
          await disputeTrade(terra, this.tradeAddr, taker);
        });

        it("Arbitrator REFUNDS trade escrow", async function () {
          await refundTradeEscrow(terra, this.tradeAddr, arbitrator);
        });

        it("Arbitrator lists assigned trades", async function () {
          const trades_query = {
            user: arbitrator.address,
            index: "arbitrator_state",
            limit: 10,
          };

          const offers = await queryOffers(terra, { trades_query });

          // console.log(offers);

          expect(offers.length).to.equal(1);
          expect(offers[0].trade.state).to.equal("settled_for_taker");
        });
        it("Arbitrator lists assigned trades", async function () {
          const trades_query = {
            user: arbitrator.address,
            unistate: "created",
            index: "arbitrator_state",
            limit: 10,
          };

          const offers = await queryOffers(terra, { trades_query });

          // console.log(offers);

          expect(offers.length).to.equal(1);
          expect(offers[0].trade.state).to.equal("settled_for_taker");
        });
      });
      describe("Release Escrow: Taker disputes Buy trade, Arbitrator settles for Maker", function () {
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

          // console.log("result offers query :>> ", offers);

          if (offers.length === 0) throw Error("No offers found.");

          global.disputeResolution.offerId = offers[0].id;
        });

        it("Taker creates a trade", async function () {
          const new_trade = {
            offer_id: parseInt(global.disputeResolution.offerId),
            ust_amount: process.env.MIN_AMOUNT,
            taker: taker.address,
            taker_contact,
            arbitrator: arbitrator.address,
          };

          this.tradeAddr = await createTrade(terra, new_trade, taker);
          return this.tradeAddr;
        });

        it("Maker funds the trade escrow.", async function () {
          await fundTradeEscrow(
            terra,
            {
              offerId: global.disputeResolution.offerId,
              tradeAddr: this.tradeAddr,
            },
            maker
          );
        });

        it("Taker disputes the trade", async function () {
          await disputeTrade(terra, this.tradeAddr, taker);
        });

        it("Arbitrator RELEASES trade escrow", async function () {
          await releaseTradeEscrow(terra, this.tradeAddr, arbitrator);
        });

        it("Arbitrator lists assigned trades", async function () {
          const trades_query = {
            user: arbitrator.address,
            index: "arbitrator_state",
            limit: 10,
          };

          const offers = await queryOffers(terra, { trades_query });

          // console.log(offers);

          expect(offers.length).to.equal(2);
          expect(offers[0].trade.state).to.equal("settled_for_maker");
        });
      });
    });
  });
})();
