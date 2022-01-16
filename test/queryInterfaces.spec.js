import { expect } from "chai";

import { createLCDClient } from "../lib/createLCDClient.js";
import { createUser } from "../lib/createUser.js";
import { createOffer } from "../lib/createOffer.js";
import { queryOffers } from "../lib/queryOffers.js";
import { queryTrades } from "../lib/queryTrades.js";
import { getFactoryCfg } from "../lib/getFactoryCfg.js";
import { createTrade } from "../lib/createTrade.js";
import { disputeTrade } from "../lib/disputeTrade.js";
import { releaseTradeEscrow } from "../lib/releaseTradeEscrow.js";
import { fundTradeEscrow } from "../lib/fundTradeEscrow.js";
import { sleep } from "../lib/sleep.js";
import { before } from "mocha";

(async () => {
  const terra = await createLCDClient();
  const maker = createUser(terra, process.env.MAKER_MNEMONIC2);
  const maker_contact = process.env.MAKER_CONTACT2;

  const taker = createUser(terra, process.env.TAKER_MNEMONIC6);
  const taker_contact = process.env.TAKER_CONTACT6;

  const arbitrator = createUser(terra, process.env.ARBITRATOR_MNEMONIC);

  const min_amount = process.env.MIN_AMOUNT;

  const max_amount = process.env.MAX_AMOUNT;

  describe("Query Interfaces", function () {
    before(async function () {
      try {
        global.factoryCfg = await getFactoryCfg(terra, maker);
      } catch (err) {
        console.error(err);
        throw err;
      }
    });
    describe("Maker", function () {
      it("should create a BUY offer in COP", async function () {
        const offer = {
          offer_type: "buy",
          fiat_currency: "COP",
          min_amount,
          max_amount,
          maker_contact,
        };

        return await createOffer(terra, offer, maker);
      });

      it("should create a SELL offer in BRL", async function () {
        const offer = {
          offer_type: "sell",
          fiat_currency: "BRL",
          min_amount,
          max_amount,
          maker_contact,
        };

        return await createOffer(terra, offer, maker);
      });

      it("should create a BUY offer in USD", async function () {
        const offer = {
          offer_type: "buy",
          fiat_currency: "USD",
          min_amount,
          max_amount,
          maker_contact,
        };

        return await createOffer(terra, offer, maker);
      });
    });
    describe("Query Offers", async function () {
      it("by owner should return 1 offer", async function () {
        const query = {
          offers_query: {
            limit: 1,
            last_value: 0,
            owner: maker.address,
          },
        };
        const result = await queryOffers(terra, query);
        expect(result.length).to.equal(1);
        return result;
      });
      it("with pagination should return 2 offers", async function () {
        const query = {
          offers_query: {
            limit: 5,
            last_value: 1,
          },
        };
        const result = await queryOffers(terra, query);
        expect(result.length).to.equal(2);
        return result;
      });
    });
    describe("Taker", async function () {
      it("should create a trade", async function () {
        const new_trade = {
          offer_id: 1,
          ust_amount: process.env.MIN_AMOUNT,
          taker: taker.address,
          taker_contact,
          arbitrator: arbitrator.address,
        };
        const tradeAddr = await createTrade(terra, new_trade, taker);
        // console.log("tradeAddr :>> ", tradeAddr);
        return tradeAddr;
      });
    });
    describe("Query trades", async function () {
      it("by index 'arbitrator' and state 'created' should return 1 trade", async function () {
        const query = {
          trades_query: {
            user: arbitrator.address,
            index: "arbitrator_state",
            state: "created",
            limit: 10,
          },
        };
        const result = await queryOffers(terra, query);
        expect(result.length).to.equal(1);
        return result;
      });
      it("by index 'seller' should return 1 trade", async function () {
        const query = {
          trades_query: {
            user: taker.address,
            index: "seller",
            limit: 10,
          },
        };
        const result = await queryOffers(terra, query);
        expect(result.length).to.equal(1);
        return result;
      });
      it("by index 'buyer' should return 1 trade", async function () {
        const query = {
          trades_query: {
            user: maker.address,
            index: "buyer",
            limit: 10,
          },
        };
        const result = await queryOffers(terra, query);
        expect(result.length).to.equal(1);
        return result;
      });
    });
  });
})();
