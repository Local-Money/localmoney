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
import { createFeeGrant } from "../lib/createFeeGrant.js";
import { revokeFeeGrant } from "../lib/revokeFeeGrant.js";
import { sleep } from "../lib/sleep.js";
import { before } from "mocha";

(async () => {
  const min_amount = process.env.MIN_AMOUNT;

  const max_amount = process.env.MAX_AMOUNT;

  const terra = await createLCDClient();

  const maker = createUser(terra, process.env.MAKER_MNEMONIC3);
  const maker_contact = process.env.MAKER_CONTACT3;

  const taker = createUser(terra, process.env.TAKER_CRYPTOLESS);
  const taker_contact = process.env.TAKER_CONTACT7;

  const arbitrator = createUser(terra, process.env.ARBITRATOR_MNEMONIC);

  describe("Cryptoless Taker Trade LifeCycle Endpoints", function () {
    describe.only("Cryptoless Taker SELL.EscrowReleased", function () {
      before(async function () {
        global.factoryCfg = await getFactoryCfg(terra, maker);

        global.tradeFlow = { offerId: undefined };
      });
      it("Maker creates a SELL offer in COP", async function () {
        const offer = {
          offer_type: "sell",
          fiat_currency: "COP",
          rate: "37842000",
          min_amount,
          max_amount,
          maker_contact,
        };

        const offerResult = await createOffer(terra, offer, maker);

        return offerResult;
      });
      it("Maker gives a feeGrant to the taker", async function () {
        const feeGrantResult = await createFeeGrant(terra, maker, taker);

        console.log("feeGrantResult :>> ", feeGrantResult);

        return feeGrantResult;
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
          offer_id: "37842000_1", //global.tradeFlow.offerId,
          ust_amount: process.env.MIN_AMOUNT,
          taker: taker.address,
          taker_contact,
        };

        this.tradeAddr = await createTrade(
          terra,
          new_trade,
          taker,
          maker.address
        );

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
          taker,
          maker.address
        );
      });
      it("Maker revokes a feeGrant from the taker", async function () {
        const feeGrantResult = await revokeFeeGrant(terra, maker, taker);

        console.log("revoke feeGrantResult :>> ", feeGrantResult);

        return feeGrantResult;
      });
    });
  });
})();
