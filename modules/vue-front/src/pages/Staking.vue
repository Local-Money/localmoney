<template>
    <main>
        <h3>Staking</h3>
        <div class="top-info-wrap">
            <div class="card info-item">
                <p class="label">Total LOCAL Deposited</p>
                <p class="value">{{ stakingTotalDeposit }}</p>
            </div>
            <div class="card info-item">
                <p class="label">APY</p>
                <p class="value">??%</p>
            </div>
            <div class="card info-item">
                <p class="label">Total xLOCAL Minted</p>
                <p class="value">{{ stakingTotalShares }}</p>
            </div>
            <div class="card info-item">
                <p class="label">Total LOCAL Warming</p>
                <p class="value">{{ stakingTotalWarming }}</p>
            </div>
        </div>

        <h3>My LOCAL</h3>

        <div class=" card controls-wrap">
            <div class="my-local-wrap">
                <div class="local">
                    <p class="label">LOCAL</p>
                    <p>$9999.000</p>
                </div>
                <div class="separator"></div>
                <div class="xlocal">
                    <p class="label">xLOCAL</p>
                    <p>$9999.000</p>
                </div>
            </div>
            <div class="wrap-btns">
                <input
                    class="bg-gray100"
                    type="text"
                    placeholder="0.000"
                    v-model="stakingAmount"
                />
                <button
                    class="secondary bg-gray300"
                    @click="enterStaking(this.stakingAmount)"
                >
                    Stake
                </button>
                <button
                    class="secondary bg-gray300"
                    @click="leaveStaking(this.stakingAmount)"
                >
                    Unstake
                </button>
            </div>
        </div>

        <h3>My warming LOCAL</h3>
        <section class="trade-history-table card">
            <div class="table-header">
                <div class="col-1"><p>Amount</p></div>
                <div class="col-2"><p>Ready</p></div>
                <div class="col-3"><p>Claim</p></div>
            </div>
            <div
                class="wrap-table-item"
                v-for="claim in stakingClaims"
                :key="claim.id"
            >
                <div class="col-1">
                    <p>{{ claim.amount }}</p>
                </div>
                <div class="col-2">
                    <p>{{ new Date(claim.created_at * 1000 + 2 * 60) }}</p>
                </div>
                <div class="col-3">
                    <button
                        @click="claimStaking(claim.id)"
                        :disabled="
                            claim.created_at * 1000 + 2 * 60 * 1000 >=
                                Date.now()
                        "
                    >
                        Claim
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
            "fetchStakingClaims",
            "enterStaking",
            "leaveStaking",
            "claimStaking",
        ]),
        polling: async function() {
            if (this.isPolling) {
                console.log("polling");
                await this.fetchStakingTotalWarming();
                await this.fetchStakingTotalDeposit();
                await this.fetchStakingTotalShares();
                await this.fetchStakingClaims();
                await sleep(5000);
                this.polling();
            }
        },
    },
    computed: {
        ...mapGetters([
            "walletAddress",
            "stakingTotalDeposit",
            "stakingTotalShares",
            "stakingTotalWarming",
            "stakingClaims",
        ]),
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

        .switcher {
            display: inline-flex;
            justify-content: space-evenly;
            width: 100%;
            max-width: 200px;
            background-color: $surface;
            border: 1px solid $border;
            border-radius: 8px;
            overflow: hidden;

            .separator {
                width: 1px;
                height: 40px;
                background-color: $border;
                margin: 0px;
                z-index: 999;
            }

            button {
                width: 100%;
                justify-content: center;
                background-color: inherit;
                border-radius: none;
                color: $gray600;
            }

            .focus {
                background-color: $gray300;
                border-radius: 0px;
                color: $primary;
            }

            @media only screen and (max-width: 550px) {
                margin-top: 0;
                max-width: none;
                margin-bottom: 16px;

                button {
                    height: 48px;
                }
                .separator {
                    height: 48px;
                }
            }
        }
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
}

.col-1,
:deep(.col-1) {
    width: 17.5%;
}

.col-2,
:deep(.col-2) {
    width: 12.5%;
}
</style>
