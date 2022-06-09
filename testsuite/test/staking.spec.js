import { expect } from "chai";

import { createLCDClient } from "../lib/createLCDClient.js";
import { createUser } from "../lib/createUser.js";
import { getFactoryCfg } from "../lib/getFactoryCfg.js";
import { enterStaking } from "../lib/enterStaking.js";
import { leaveStaking } from "../lib/leaveStaking.js";
import { queryClaims } from "../lib/queryClaims.js";
import { claimStaking } from "../lib/claimStaking.js";
import { queryStakingTotals } from "../lib/queryStakingTotals.js";
import { before } from "mocha";

(async () => {
  const terra = await createLCDClient();

  const staker = createUser(terra, process.env.STAKER_MNEMONIC);

  describe("Staking", function () {
    before(async function () {
      global.factoryCfg = await getFactoryCfg(terra, staker);
    });
    describe("Enter", function () {
      it("should deposit 1 LOCAL", async function () {
        const stakingResult = await enterStaking(terra, "1000000", staker);

        console.log("stakingResult :>> ", stakingResult);

        return stakingResult;
      });
      // TODO list yLOCAL balance
    });
    describe("Query Staking Totals", function () {
      it("should return totals", async function () {
        const resultStakingTotals = await queryStakingTotals(terra);

        console.log("resultStakingTotals :>> ", resultStakingTotals);

        return resultStakingTotals;
      });
    });
    describe("Enter", function () {
      it("should deposit 2 LOCAL", async function () {
        const stakingResult = await enterStaking(terra, "2000000", staker);

        console.log("stakingResult :>> ", stakingResult);

        return stakingResult;
      });
      // TODO list yLOCAL balance
    });
    describe("Query Staking Totals", function () {
      it("should return totals", async function () {
        const resultStakingTotals = await queryStakingTotals(terra);

        console.log("resultStakingTotals :>> ", resultStakingTotals);

        return resultStakingTotals;
      });
    });
    describe("Leave", function () {
      it("should withdraw 1 LOCAL", async function () {
        const leaveStakingResult = await leaveStaking(terra, "1000000", staker);

        console.log("leaveStakingResult :>> ", leaveStakingResult);

        return leaveStakingResult;
      });
    });
    describe("Query Staking Totals", function () {
      it("should return totals", async function () {
        const resultStakingTotals = await queryStakingTotals(terra);

        console.log("resultStakingTotals :>> ", resultStakingTotals);

        return resultStakingTotals;
      });
    });
    describe("Leave", function () {
      it("should withdraw 0.5 LOCAL", async function () {
        const leaveStakingResult = await leaveStaking(terra, "500000", staker);

        console.log("leaveStakingResult :>> ", leaveStakingResult);

        return leaveStakingResult;
      });
    });
    describe("Query Staking Totals", function () {
      it("should return totals", async function () {
        const resultStakingTotals = await queryStakingTotals(terra);

        console.log("resultStakingTotals :>> ", resultStakingTotals);

        return resultStakingTotals;
      });
    });
    describe("Query Claims", function () {
      it("should show 1 LOCAL maturing.", async function () {
        const queryClaimsResult = await queryClaims(terra);

        console.log("queryClaimsResult :>> ", queryClaimsResult);

        return queryClaimsResult;
      });
    });
    describe("Execute Claim", function () {
      it("transfer 1 matured LOCAL", async function () {
        const claimResult = await claimStaking(terra, 1, staker);

        console.log("claimResult :>> ", claimResult);

        return claimResult;
      });
      describe("Query Claims", function () {
        it("should show 1 LOCAL maturing.", async function () {
          const queryClaimsResult = await queryClaims(terra);

          console.log("queryClaimsResult :>> ", queryClaimsResult);

          return queryClaimsResult;
        });
      });
      describe("Query Staking Totals", function () {
        it("should return totals", async function () {
          const resultStakingTotals = await queryStakingTotals(terra);

          console.log("resultStakingTotals :>> ", resultStakingTotals);

          return resultStakingTotals;
        });
      });
    });
  });
})();
