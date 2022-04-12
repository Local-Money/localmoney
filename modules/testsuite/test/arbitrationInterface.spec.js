import { expect } from "chai";

import { createLCDClient } from "../lib/createLCDClient.js";
import { createUser } from "../lib/createUser.js";
import { sleep } from "../lib/sleep.js";
import { queryOffers } from "../lib/queryOffers.js";
import { createArbitrator } from "../lib/createArbitrator.js";
import { deleteArbitrator } from "../lib/deleteArbitrator.js";
import { getFactoryCfg } from "../lib/getFactoryCfg.js";
import { before } from "mocha";

(async () => {
  const terra = await createLCDClient();

  const quorum = createUser(terra, process.env.QUORUM_MNEMONIC);

  const attacker = createUser(terra, process.env.MAKER_MNEMONIC2);

  describe("LocalTerra", function () {
    describe("Arbitration Interface", function () {
      before(async function () {
        global.factoryCfg = await getFactoryCfg(terra, quorum);
      });
      describe("New arbitrator with asset", function () {
        it("should create a new arbitrator `jr` with fiat currency COP", async function () {
          const arbitrator = {
            arbitrator: "terra1a29pt4q0vmf3qzjl3nu5fc0wuqj02qsfkvqyjr",
            asset: "COP",
          };

          const arbitratorResult = await createArbitrator(
            terra,
            arbitrator,
            quorum
          );

          return arbitratorResult;
        });
        it("should create a new arbitrator `jr` with fiat currency BRL", async function () {
          await sleep(2000);
          const arbitrator = {
            arbitrator: "terra1a29pt4q0vmf3qzjl3nu5fc0wuqj02qsfkvqyjr",
            asset: "BRL",
          };

          const arbitratorResult = await createArbitrator(
            terra,
            arbitrator,
            quorum
          );

          return arbitratorResult;
        });
        it("should create a new arbitrator `jr` with fiat currency KWD", async function () {
          await sleep(2000);
          const arbitrator = {
            arbitrator: "terra1a29pt4q0vmf3qzjl3nu5fc0wuqj02qsfkvqyjr",
            asset: "KWD",
          };

          const arbitratorResult = await createArbitrator(
            terra,
            arbitrator,
            quorum
          );

          return arbitratorResult;
        });
        it("should DELETE the arbitrator `jr` with fiat currency KWD", async function () {
          await sleep(2000);
          const arbitrator = {
            arbitrator: "terra1a29pt4q0vmf3qzjl3nu5fc0wuqj02qsfkvqyjr",
            asset: "KWD",
          };

          const arbitratorResult = await deleteArbitrator(
            terra,
            arbitrator,
            quorum
          );

          return arbitratorResult;
        });
        it("should create a new arbitrator `sk` with fiat currency COP", async function () {
          await sleep(2000);
          const arbitrator = {
            arbitrator: "terra10ms2n6uqzgrz4gtkcyslqx0gysfvwlg6n2tusk",
            asset: "COP",
          };

          const arbitratorResult = await createArbitrator(
            terra,
            arbitrator,
            quorum
          );

          return arbitratorResult;
        });
        it("should throw Unauthorized when an attacker tries to create an arbitrator", async function () {
          await sleep(2000);
          const arbitrator = {
            arbitrator: "terra10ms2n6uqzgrz4gtkcyslqx0gysfvwlg6n2tusk",
            asset: "COP",
          };

          let error;

          try {
            const arbitratorResult = await createArbitrator(
              terra,
              arbitrator,
              attacker
            );
          } catch (e) {
            error = e;
          }

          expect(error.response.data.code).to.equal(3);
          expect(error.response.data.message).to.include("Unauthorized");
        });
        it("should throw Unauthorized when an attacker tries to DELETE an arbitrator", async function () {
          await sleep(2000);
          const arbitrator = {
            arbitrator: "terra10ms2n6uqzgrz4gtkcyslqx0gysfvwlg6n2tusk",
            asset: "COP",
          };

          let error;

          try {
            const arbitratorResult = await createArbitrator(
              terra,
              arbitrator,
              attacker
            );
          } catch (e) {
            error = e;
          }

          expect(error.response.data.code).to.equal(3);
          expect(error.response.data.message).to.include("Unauthorized");
        });
      });
      describe("Query arbitrator interface", function () {
        it("should return all arbitrator / fiat pairs with pagination", async function () {
          const query = {
            arbitrators: {
              last_value: "terra1a29pt4q0vmf3qzjl3nu5fc0wuqj02qsfkvqyjrBRL", // last_value = addr + asset
              limit: 10,
            },
          };

          const result = await queryOffers(terra, query);

          console.log("query all result :>> ", result);

          expect(result.length).to.equal(1);
        });
        it("should return the fiat currencies for an arbitrator `jr`", async function () {
          const query = {
            arbitrator: {
              arbitrator: "terra1a29pt4q0vmf3qzjl3nu5fc0wuqj02qsfkvqyjr",
            },
          };

          const result = await queryOffers(terra, query);

          expect(result.length).to.equal(2);
        });
        it("should return the fiat currencies for an arbitrator `sk`", async function () {
          const query = {
            arbitrator: {
              arbitrator: "terra10ms2n6uqzgrz4gtkcyslqx0gysfvwlg6n2tusk",
            },
          };

          const result = await queryOffers(terra, query);

          console.log("result :>> ", result);

          expect(result.length).to.equal(1);
        });
        it("should return the arbitrators for a fiat currency COP", async function () {
          const query = {
            arbitrator_asset: {
              asset: "COP",
            },
          };

          const result = await queryOffers(terra, query);

          console.log("result :>> ", result);

          expect(result.length).to.equal(2);
        });
        it("should return the arbitrators for a fiat currency COP", async function () {
          const query = {
            arbitrator_asset: {
              asset: "COP",
            },
          };

          const result = await queryOffers(terra, query);

          console.log("cop arbitrators result :>> ", result);

          expect(result.length).to.equal(3);
        });
        it("should return a random arbitrator", async function () {
          const query = {
            arbitrator_random: { random_value: 23, asset: "COP" },
          };

          const result = await queryOffers(terra, query);

          console.log("arbitrator_random result :>> ", result);

          // expect(result.length).to.equal(1);
        });
      });
    });
  });
})();
