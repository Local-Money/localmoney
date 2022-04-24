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
import { fiatDeposited } from "../lib/fiatDeposited.js";
import { sleep } from "../lib/sleep.js";
import { before } from "mocha";

(async () => {
  const terra = await createLCDClient();
  const maker = createUser(terra, process.env.MAKER_MNEMONIC3);
  const maker_contact = process.env.MAKER_CONTACT3;

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
      it("should create a SELL offer in BRL", async function () {
        const offer = {
          offer_type: "sell",
          fiat_currency: "BRL",
          rate: "48000",
          min_amount,
          max_amount,
          maker_contact,
        };

        return await createOffer(terra, offer, maker);
      });
      it("should create a BUY offer in COP", async function () {
        const offer = {
          offer_type: "buy",
          fiat_currency: "COP",
          rate: "37842000",
          min_amount,
          max_amount,
          maker_contact,
        };

        return await createOffer(terra, offer, maker);
      });
      it("should create a BUY offer in COP", async function () {
        const offer = {
          offer_type: "buy",
          fiat_currency: "COP",
          rate: "37842007",
          min_amount,
          max_amount,
          maker_contact,
        };

        return await createOffer(terra, offer, maker);
      });
      it("should create a BUY offer in COP", async function () {
        const offer = {
          offer_type: "buy",
          fiat_currency: "COP",
          rate: "37842001",
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
          rate: "10000",
          min_amount,
          max_amount,
          maker_contact,
        };

        const result = await createOffer(terra, offer, maker);
        return result;
      });
      it("should create a BUY offer in COP", async function () {
        const offer = {
          offer_type: "buy",
          fiat_currency: "COP",
          rate: "37842000",
          min_amount,
          max_amount,
          maker_contact,
        };

        return await createOffer(terra, offer, maker);
      });
      it("should create a BUY offer in COP", async function () {
        const offer = {
          offer_type: "buy",
          fiat_currency: "COP",
          rate: "37842000",
          min_amount,
          max_amount,
          maker_contact,
        };

        return await createOffer(terra, offer, maker);
      });
      it("should create a BUY offer in COP", async function () {
        const offer = {
          offer_type: "buy",
          fiat_currency: "COP",
          rate: "37842000",
          min_amount,
          max_amount,
          maker_contact,
        };

        return await createOffer(terra, offer, maker);
      });
      it("should create a BUY offer in COP", async function () {
        const offer = {
          offer_type: "buy",
          fiat_currency: "COP",
          rate: "37842000",
          min_amount,
          max_amount,
          maker_contact,
        };

        return await createOffer(terra, offer, maker);
      });
      it("should create a BUY offer in COP", async function () {
        const offer = {
          offer_type: "buy",
          fiat_currency: "COP",
          rate: "37842000",
          min_amount,
          max_amount,
          maker_contact,
        };

        return await createOffer(terra, offer, maker);
      });
      it("should create a BUY offer in COP", async function () {
        const offer = {
          offer_type: "buy",
          fiat_currency: "COP",
          rate: "37842001",
          min_amount,
          max_amount,
          maker_contact,
        };

        return await createOffer(terra, offer, maker);
      });
      it("should create a BUY offer in COP", async function () {
        const offer = {
          offer_type: "buy",
          fiat_currency: "COP",
          rate: "37842002",
          min_amount,
          max_amount,
          maker_contact,
        };

        return await createOffer(terra, offer, maker);
      });
      it("should create a BUY offer in COP", async function () {
        const offer = {
          offer_type: "buy",
          fiat_currency: "COP",
          rate: "37842006",
          min_amount,
          max_amount,
          maker_contact,
        };

        return await createOffer(terra, offer, maker);
      });
      it("should create a BUY offer in COP", async function () {
        const offer = {
          offer_type: "buy",
          fiat_currency: "COP",
          rate: "37842005",
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
      it("with pagination should return 14 offers sorted by rate", async function () {
        const query = {
          offers_query: {
            limit: 15,
            last_value: 1,
          },
        };
        const result = await queryOffers(terra, query);
        console.log("sorted by rate result :>> ", result);
        expect(result.length).to.equal(14);
        return result;
      });
    });

    describe("Trade -> SELL.EscrowDisputed", function () {
      before(async function () {
        global.factoryCfg = await getFactoryCfg(terra, maker);

        global.tradeFlow = { offerId: undefined };
      });
      it("Maker creates a SELL offer in COP", async function () {
        const offer = {
          offer_type: "sell",
          fiat_currency: "COP",
          rate: "37842001",
          min_amount,
          max_amount,
          maker_contact,
        };

        const offerResult = await createOffer(terra, offer, maker); // TODO set offerId directly from the offerResult and remove factory instantiation / taker listing offer

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
      it("Taker should disputes the trade (TadeState::EscrowDisputed)", async function () {
        await disputeTrade(terra, this.tradeAddr, maker);
      });
    });
    describe("Query trades", async function () {
      it("by index 'arbitrator' and state 'created' should return 1 trade", async function () {
        const query = {
          trades_query: {
            user: arbitrator.address,
            index: "arbitrator_state",
            state: "request_created",
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
            user: taker.address,
            index: "buyer",
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
            user: maker.address,
            index: "seller",
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
