<template>
    <div class="collapsed" :key="`${offer.id}-collapsed`">
        <div class="owner">
            <div class="wrap">
                <p class="wallet">
                    {{ formatAddress(offer.owner) }}
                </p>
                <p class="n-trades">0 trades</p>
            </div>
        </div>

        <div class="info">
            <div class="divider"></div>
            <p class="min-max">
                Min ${{ formatAmount(offer.min_amount) }} - Max ${{
                    formatAmount(offer.max_amount)
                }}
            </p>
        </div>

        <div class="price">
            <div class="wrap-value">
                <p class="value">
                    {{ offer.fiat_currency }} {{ formatAmount(usdRate, false) }}
                </p>
                <p class="margin">0% above market</p>
            </div>
            <div>
                <p class="value">
                    {{ offer.state }}
                </p>
            </div>
            <div>
                <p class="value">
                    {{ offer.offer_type }}
                </p>
            </div>

            <button type="button" v-on:click="$emit('select', offer)">
                View
            </button>
        </div>
    </div>
</template>

<script>
import { defineComponent } from "vue";
import { formatAddress, formatAmount } from "@/shared";
import { mapGetters } from "vuex";

export default defineComponent({
    name: "CollapsedOffer",
    props: ["offer"],
    setup() {},
    data() {
        return {};
    },
    methods: {
        formatAddress,
        formatAmount,
    },
    computed: {
        ...mapGetters(["getUsdRate"]),
        usdRate: function() {
            return this.getUsdRate(this.$props.offer.fiat_currency);
        },
    },
});
</script>

<style lang="scss" scoped>
@import "@/style/tokens.scss";

.owner {
    grid-column: 1/2;
    grid-row: 1;
    display: flex;
    flex-direction: row;

    .avatar {
        width: 48px;
        height: 48px;
        margin-right: 24px;

        img {
            display: block;
            width: 100%;
            height: 100%;
            border-radius: 100px;
        }
    }

    .wrap {
        display: flex;
        flex-direction: column;
    }

    .wallet {
        font-size: 18px;
        font-weight: 600;
        color: $base-text;
    }

    .n-trades {
        font-size: 14px;
        color: $gray600;
    }

    @media only screen and (max-width: 550px) {
        .owner {
            display: inline-flex;
        }
    }
}

.info {
    grid-column: 2/5;
    grid-row: 1;
    display: flex;
    align-items: center;
    gap: 24px;

    .currency {
        text-align: center;
        font-size: 12px;
        font-weight: 600;
        color: $gray600;
        text-transform: uppercase;

        img {
            width: 24px;
        }
    }

    .divider {
        height: 40px;
        width: 1px;
        background-color: $border;
    }

    .note {
        font-size: 14px;
        color: $gray600;
        max-width: 200px;
    }

    .min-max {
        font-size: 14px;
        color: $gray600;
    }

    @media only screen and (max-width: 550px) {
        grid-column: 1/7;
        grid-row: 2;

        .note {
            grid-row: 3;
        }
        .divider {
            display: none;
        }
        .min-max {
            padding: 8px 16px;
            margin-bottom: 4px;
            border-radius: 8px;
            background-color: $gray150;
        }
    }
}

.collapsed {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr 1fr 1fr;
    gap: 16px;

    @media only screen and (max-width: 1050px) {
        grid-template-columns: 1fr 1fr 1fr 1fr 1fr;
    }

    @media only screen and (max-width: 550px) {
        grid-template-columns: 1fr 1fr;
        padding: 24px 24px;
    }

    .price {
        grid-column: 1/7;
        grid-row: 1;
        justify-self: end;
        text-align: right;
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: 24px;

        .value {
            font-size: 20px;
            font-weight: 800;
            color: $base-text;
        }

        .margin {
            font-size: 14px;
            color: $gray600;
        }

        button {
            background-color: $gray300;
            color: $primary;
            border: none;
            font-family: inherit;
            font-weight: 700;
            font-size: 16px;
            text-transform: lowercase;
            padding: 8px 24px;
        }

        @media only screen and (max-width: 550px) {
            grid-column: 1/7;
            grid-row: 3;
            text-align: left;
            justify-content: space-between;
            gap: none;
            width: 100%;
        }
    }
}
</style>
