<template>
    <main>
        <h3>Staking</h3>
        <p>Total LOCAL Deposited: {{ stakingTotalDeposit }}</p>
        <p>Total xLOCAL Minted: {{ stakingTotalShares }}</p>
        <p>Total LOCAL Warming: {{ stakingTotalWarming }}</p>
        <div class="buy-sell">
            <button @click="enterStaking('10000000')">
                Stake
            </button>
            <button @click="leaveStaking('5000000')">
                Unstake
            </button>
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
