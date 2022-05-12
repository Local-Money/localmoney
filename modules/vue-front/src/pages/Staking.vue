<template>
    <main>
        <h3>Staking</h3>
        <div class="top-info-wrap">
            <div class="card info-item">
                <p class="label">Total LOCAL Deposited</p>
                <p class="value">{{ prettyStakingTotalDeposit }}</p>
            </div>
            <div class="card info-item">
                <p class="label">xLOCAL:LOCAL</p>
                <p class="value">
                    1:{{
                        prettyRatio(stakingTotalDeposit / stakingTotalShares)
                    }}
                </p>
            </div>
            <div class="card info-item">
                <p class="label">Total xLOCAL Minted</p>
                <p class="value">{{ prettyStakingTotalShares }}</p>
            </div>
            <div class="card info-item">
                <p class="label">Total LOCAL Warming</p>
                <p class="value">{{ prettyStakingTotalWarming }}</p>
            </div>
        </div>

        <h3>My LOCAL</h3>

        <div class=" card controls-wrap">
            <div class="my-local-wrap">
                <div
                    class="local pointer"
                    @click="stakingAmount = parseInt(myLocalBalance) / 1e6"
                >
                    <p class="label">LOCAL</p>
                    <p>{{ prettyBalance(myLocalBalance) }}</p>
                </div>
                <div class="separator"></div>
                <div
                    class="xlocal pointer"
                    @click="stakingAmount = parseInt(myxLocalBalance) / 1e6"
                >
                    <p class="label">xLOCAL</p>
                    <p>{{ prettyBalance(myxLocalBalance) }}</p>
                </div>
            </div>
            <div class="wrap-btns">
                <input class="bg-gray100" v-model="stakingAmount" />
                <button
                    class="secondary bg-gray300 primary-action"
                    @click="enter(stakingAmount)"
                    :disabled="!isStakingAmountValid(stakingAmount)"
                >
                    stake
                </button>
                <button
                    class="secondary bg-gray300"
                    @click="leave(stakingAmount)"
                    :disabled="!isStakingAmountValid(stakingAmount)"
                >
                    unstake
                </button>
            </div>
        </div>

        <h3>Unstaking</h3>
        <section class="trade-history-table card">
            <div class="table-header">
                <div class="col-1"><p>Amount</p></div>
                <div class="col-2"><p>Release On</p></div>
                <div class="col-3"><p></p></div>
            </div>
            <div
                class="wrap-table-item"
                v-for="claim in sortedStakingClaims"
                :key="claim.id"
            >
                <div class="col-1">
                    <p>{{ prettyBalance(claim.amount) }} LOCAL</p>
                </div>
                <div class="col-2">
                    <p>
                        {{ prettyReleaseOn(claim.released_at) }}
                    </p>
                </div>
                <div class="col-3">
                    <button
                        class="secondary claim"
                        :class="disabled"
                        @click="claimStaking(claim.id)"
                        :disabled="!isClaimReady(claim.released_at)"
                    >
                        claim
                    </button>
                </div>
            </div>
        </section>
    </main>
</template>

<script>
import { defineComponent } from "vue";
import { mapActions, mapGetters } from "vuex";
import { sleep } from "../shared.js";
import numeral from "numeral";
import * as timeago from "timeago.js";

export default defineComponent({
    name: "Staking",
    components: {},
    data: function() {
        return {
            isPolling: true,
            isStake: true,
            stakingAmount: 0,
        };
    },
    methods: {
        ...mapActions([
            "fetchStakingTotalWarming",
            "fetchStakingTotalDeposit",
            "fetchStakingTotalShares",
            "fetchMyLocalBalance",
            "fetchMyxLocalBalance",
            "fetchStakingClaims",
            "enterStaking",
            "leaveStaking",
            "claimStaking",
        ]),
        enter: async function(amount) {
            await this.enterStaking(parseInt(amount * 1e6).toString());
            this.stakingAmount = "";
        },
        leave: async function(amount) {
            await this.leaveStaking(parseInt(amount * 1e6).toString());
            this.stakingAmount = "";
        },
        polling: async function() {
            if (this.isPolling) {
                console.log("polling");
                await this.fetchStakingTotalWarming();
                await this.fetchStakingTotalDeposit();
                await this.fetchStakingTotalShares();
                await this.fetchStakingClaims();
                await this.fetchMyLocalBalance();
                await this.fetchMyxLocalBalance();
                await this.fetchStakingClaims();
                await sleep(5000);
                this.polling();
            }
        },
        fuzzy: function(time) {
            return timeago.format(time);
        },
    },
    computed: {
        ...mapGetters([
            "walletAddress",
            "stakingTotalDeposit",
            "stakingTotalShares",
            "stakingTotalWarming",
            "stakingClaims",
            "myLocalBalance",
            "myxLocalBalance",
        ]),
        prettyStakingTotalDeposit: function() {
            return numeral(this.stakingTotalDeposit / 1e6).format("0,0.0");
        },
        prettyStakingTotalShares: function() {
            return numeral(this.stakingTotalShares / 1e6).format("0,0.0");
        },
        prettyStakingTotalWarming: function() {
            return numeral(this.stakingTotalWarming / 1e6).format("0,0.0");
        },
        prettyStakingClaims: function() {
            return numeral(this.stakingClaims / 1e6).format("0,0");
        },
        isClaimReady: () => (claim_released_at) =>
            new Date(claim_released_at * 1000) < new Date(),
        isStakingAmountValid: () => (stakingAmount) =>
            parseInt(stakingAmount * 1e6) > 0,
        prettyBalance: () => (balance) =>
            numeral(balance / 1e6).format("0,0.00"),
        prettyRatio: () => (ratio) => numeral(ratio).format("0.00"),
        prettyReleaseOn() {
            return (timestamp) => {
                const date = new Date(timestamp * 1000);
                if (date < new Date()) {
                    return "ready";
                } else {
                    return this.fuzzy(date);
                }
            };
        },
        sortedStakingClaims: function() {
            return [...this.stakingClaims].sort((a, b) => a.id - b.id);
        },
    },
    async mounted() {
        this.polling();
    },
    unmounted() {
        this.isPolling = false;
    },
});
</script>

<style lang="scss" scoped>
@import "../style/pages.scss";
.pointer {
    cursor: pointer;
}
h3 {
    margin: 32px 0;
    font-size: 18px;
    font-weight: $semi-bold;
}

.top-info-wrap {
    margin-bottom: 32px;
    display: flex;
    justify-content: space-between;
    gap: 24px;

    .info-item {
        width: 100%;
        text-align: center;

        .label {
            font-size: 14px;
            color: $gray700;
        }

        .value {
            font-size: 24px;
        }
    }
}

.controls-wrap {
    display: flex;
    justify-content: space-between;

    .my-local-wrap {
        display: flex;
        gap: 32px;

        .label {
            font-size: 12px;
            color: $gray700;
        }

        .separator {
            width: 1px;
            height: 100%;
            border: 1px solid $border;
        }
    }

    .wrap-btns {
        display: flex;
        align-items: center;
        gap: 24px;
    }
    .primary-action {
        color: $primary;
    }
}

.trade-history-table {
    .table-header {
        display: flex;
        flex-direction: row;
        border-bottom: 1px solid $border;
        padding: 16px;
        margin-bottom: 16px;

        p {
            font-size: 14px;
            font-weight: $semi-bold;
            color: $gray700;
        }
    }

    .col-1,
    :deep(.col-1) {
        width: 20%;
    }

    .col-2,
    :deep(.col-2) {
        width: 70%;
    }

    .col-3,
    :deep(.col-3) {
        width: 10%;
    }

    .wrap-table-item {
        display: flex;
        flex-direction: row;
        padding: 16px;

        p {
            font-size: 14px;
            font-weight: $regular;
        }

        .claim {
            font-size: 14px;
            color: $primary;
        }
        :disabled {
            color: $gray700;
        }
    }
}
</style>
