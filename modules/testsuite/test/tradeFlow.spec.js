import { expect } from "chai";

import { createLCDClient } from "../lib/createLCDClient.js";
import { createUser } from "../lib/createUser.js";
import { createOffer } from "../lib/createOffer.js";
import { queryOffers } from "../lib/queryOffers.js";
import { getFactoryCfg } from "../lib/getFactoryCfg.js";
import { createTrade } from "../lib/createTrade.js";
import { acceptTradeRequest } from "../lib/acceptTradeRequest.js";
import { disputeTrade } from "../lib/disputeTrade.js";
import { fiatDeposited } from "../lib/fiatDeposited.js";
import { releaseTradeEscrow } from "../lib/releaseTradeEscrow.js";
import { refundTradeEscrow } from "../lib/refundTradeEscrow.js";
import { fundTradeEscrow } from "../lib/fundTradeEscrow.js";
import { cancelTradeRequest } from "../lib/cancelTradeRequest.js";
import { sleep } from "../lib/sleep.js";
import { before } from "mocha";

(async () => {
  const min_amount = process.env.MIN_AMOUNT;

  const max_amount = process.env.MAX_AMOUNT;

  const terra = await createLCDClient();

  const maker = createUser(terra, process.env.MAKER_MNEMONIC3);
  const maker_contact = process.env.MAKER_CONTACT3;

  const taker = createUser(terra, process.env.TAKER_MNEMONIC7);
  const taker_contact = process.env.TAKER_CONTACT7;

  const arbitrator = createUser(terra, process.env.ARBITRATOR_MNEMONIC);

  describe("Trade LifeCycle Endpoints", function () {
    describe.only("SELL.EscrowReleased", function () {
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
          offer_id: parseInt(global.tradeFlow.offerId),
          ust_amount: process.env.MIN_AMOUNT,
          taker: taker.address,
          taker_contact,
          // arbitrator:,
        };

        this.tradeAddr = await createTrade(terra, new_trade, taker);

        return this.tradeAddr;
      });

      it("Maker funds the trade escrow (TradeState::EscrowFunded)", async function () {
        await sleep(5000);
        await fundTradeEscrow(
          terra,
          {
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
      it("Maker releases the escrow (TradeState::EscrowReleased)", async function () {
        await releaseTradeEscrow(terra, this.tradeAddr, maker);
      });
    });
    describe("SELL.RequestCanceled: Maker cancels", function () {
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
          offer_id: parseInt(global.tradeFlow.offerId),
          ust_amount: process.env.MIN_AMOUNT,
          taker: taker.address,
          taker_contact,
          // arbitrator:,
        };

        this.tradeAddr = await createTrade(terra, new_trade, taker);

        return this.tradeAddr;
      });

      it("Maker cancels the trade request (TradeState::RequestCanceled)", async function () {
        await cancelTradeRequest(terra, this.tradeAddr, maker);
      });
    });
    describe("SELL.RequestCanceled: Taker cancels", function () {
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
          offer_id: parseInt(global.tradeFlow.offerId),
          ust_amount: process.env.MIN_AMOUNT,
          taker: taker.address,
          taker_contact,
          // arbitrator:,
        };

        this.tradeAddr = await createTrade(terra, new_trade, taker);

        return this.tradeAddr;
      });

      it("Taker cancels the trade request (TradeState::RequestCanceled)", async function () {
        await cancelTradeRequest(terra, this.tradeAddr, taker);
      });
    });
    describe.skip("SELL.EscrowRefunded", function () {
      // TODO test after waiting for 2 hours?
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
          offer_id: parseInt(global.tradeFlow.offerId),
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
            offerId: global.tradeFlow.offerId,
            tradeAddr: this.tradeAddr,
          },
          maker
        );
      });
      it("The funding expires; Maker refunds the escrow (TradeState::EscrowRefunded)", async function () {
        await refundTradeEscrow(terra, this.tradeAddr, maker);
      });
    });
    describe.only("BUY.EscrowReleased", function () {
      before(async function () {
        global.factoryCfg = await getFactoryCfg(terra, maker);
        global.tradeFlow = { offerId: undefined };
      });
      it("Maker creates a BUY offer in BRL", async function () {
        const offer = {
          offer_type: "buy",
          fiat_currency: "BRL",
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
          offer_id: parseInt(global.tradeFlow.offerId),
          ust_amount: process.env.MIN_AMOUNT,
          taker: taker.address,
          taker_contact,
          // arbitrator:,
        };

        this.tradeAddr = await createTrade(terra, new_trade, taker);
        return this.tradeAddr;
      });
      it("Maker accepts the trade (TadeState::RequestAccepted)", async function () {
        await acceptTradeRequest(terra, this.tradeAddr, maker);
        return;
      });
      it("Taker funds the trade escrow (TradeState::EscrowFunded)", async function () {
        await sleep(5000);
        await fundTradeEscrow(
          terra,
          {
            offerId: global.tradeFlow.offerId,
            tradeAddr: this.tradeAddr,
          },
          taker
        );
      });
      it("Maker clicks `mark paid` (TradeState::FiatDeposited)", async function () {
        await fiatDeposited(
          terra,
          {
            tradeAddr: this.tradeAddr,
          },
          maker
        );
      });
      it("Taker releases the escrow (TradeState::EscrowReleased)", async function () {
        await releaseTradeEscrow(terra, this.tradeAddr, taker);
      });
    });
    describe("Trade Flow", function () {
      before(async function () {
        global.factoryCfg = await getFactoryCfg(terra, maker);

        global.tradeFlow = { offerId: undefined };
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
        it("Maker creates a buy offer in COP", async function () {
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
      describe("released BUY offer: Taker creates trade, funds escrow and releases.", function () {
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

        it("Taker creates a trade", async function () {
          const new_trade = {
            offer_id: parseInt(global.tradeFlow.offerId),
            ust_amount: process.env.MIN_AMOUNT,
            taker: taker.address,
            taker_contact,
            arbitrator: arbitrator.address,
          };

          this.tradeAddr = await createTrade(terra, new_trade, taker);

          return this.tradeAddr;
        });

        it("Taker funds the trade escrow.", async function () {
          await fundTradeEscrow(
            terra,
            {
              offerId: global.tradeFlow.offerId,
              tradeAddr: this.tradeAddr,
            },
            taker
          );
        });

        it("Taker releases the trade escrow.", async function () {
          await releaseTradeEscrow(terra, this.tradeAddr, taker);
        });
        it("Taker lists his trades", async function () {
          const trades_query = {
            user: taker.address,
            index: "seller",
            limit: 10,
          };

          const offers = await queryOffers(terra, { trades_query });

          // console.log(offers);

          expect(offers.length).to.equal(1);
          expect(offers[0].trade.state).to.equal("released");
        });
      });
      describe("released SELL offer: Taker creates trade. Maker funds escrow and releases.", function () {
        // Suit variables
        this.tradeAddr = undefined;

        it("Taker lists a SELL offer", async function () {
          // TODO use query filters
          const query = {
            offers_query: {
              limit: 1,
              last_value: 1,
            },
          };

          const offers = await queryOffers(terra, query);

          if (offers.length === 0) throw Error("No offers found.");

          global.tradeFlow.offerId = offers[0].id;

          return;
        });

        it("Taker creates a trade", async function () {
          const new_trade = {
            offer_id: parseInt(global.tradeFlow.offerId),
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
              offerId: global.tradeFlow.offerId,
              tradeAddr: this.tradeAddr,
            },
            maker
          );
        });

        it("Maker releases the trade escrow.", async function () {
          await releaseTradeEscrow(terra, this.tradeAddr, taker);
        });
        it("Taker lists his trades", async function () {
          const trades_query = {
            user: taker.address,
            index: "seller",
            limit: 10,
          };

          const offers = await queryOffers(terra, { trades_query });

          // console.log(offers);

          expect(offers.length).to.equal(2);
          expect(offers[0].trade.state).to.equal("released");
          expect(offers[1].trade.state).to.equal("released");
        });
      });
      // TODO enabled cancellation in smart contract
      describe.skip("cancelled SELL offer: Taker creates trade. Maker funds escrow. Taker refunds.", function () {
        // Suit variables
        this.tradeAddr = undefined;

        it("Taker lists a SELL offer", async function () {
          // TODO use query filters
          const query = {
            offers_query: {
              limit: 1,
              last_value: 1,
            },
          };

          const offers = await queryOffers(terra, query);

          if (offers.length === 0) throw Error("No offers found.");

          global.tradeFlow.offerId = offers[0].id;

          return;
        });

        it("Taker creates a trade", async function () {
          const new_trade = {
            offer_id: parseInt(global.tradeFlow.offerId),
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
              offerId: global.tradeFlow.offerId,
              tradeAddr: this.tradeAddr,
            },
            maker
          );
        });

        it("Taker refunds the trade escrow.", async function () {
          await refundTradeEscrow(terra, this.tradeAddr, maker);
        });
        it("Taker lists his trades", async function () {
          const trades_query = {
            user: taker.address,
            index: "seller",
            limit: 10,
          };

          const trades = await queryOffers(terra, { trades_query });

          // console.log(trades);

          // expect(trades.length).to.equal(2);
          // expect(trades[0].trade.state).to.equal("released");
          // expect(trades[1].trade.state).to.equal("released");
        });
      });
    });
  });
})();
