<template>
    <div class="collapsed" :key="`${offer.id}-collapsed`">
        <div class="offer-type">
            <p class="state">
                {{ offer.state }}
            </p>
            <p class="type">{{ offer.offer_type }}ing</p>
        </div>

        <div class="info">
            <!-- <p class="state">
{{ offer.state }}
</p>
<div class="divider"></div> -->
            <div class="wrap">
                <p class="label">Limits</p>
                <p class="limit">
                    ${{ formatAmount(offer.min_amount) }} - ${{
                        formatAmount(offer.max_amount)
                    }}
                </p>
            </div>
            <div class="divider"></div>
            <div class="wrap">
                <p class="label">Price</p>
                <p class="margin">
                    {{ marginRate.marginOffset }}%
                    {{ marginRate.margin }} market
                </p>
            </div>
        </div>

        <div class="price">
            <p class="value">{{ offerPrice }}</p>
            <button
                type="button"
                v-on:click="$emit('select', offer)"
                class="tertiary"
            >
                edit
            </button>
        </div>
    </div>
</template>

<script>
import { defineComponent } from "vue";
import {
    calculateFiatPriceByRate,
    convertOfferRateToMarginRate,
    formatAddress,
    formatAmount,
} from "@/shared";
import { mapGetters } from "vuex";

export default defineComponent({
    name: "CollapsedMyOffer",
    props: ["offer"],
    setup() {},
    data() {
        return {
            marginRate: convertOfferRateToMarginRate(this.offer.rate),
        };
    },
    methods: {
        formatAddress,
        formatAmount,
    },
    computed: {
        ...mapGetters(["getUsdRate"]),
        usdRate: function() {
            return this.getUsdRate(this.offer.fiat_currency);
        },
        offerPrice: function() {
            const fiatPrice = calculateFiatPriceByRate(
                this.usdRate,
                this.offer.rate,
            );
            return `${this.offer.fiat_currency} ${formatAmount(
                fiatPrice,
                false,
            )}`;
        },
    },
});
</script>

<style lang="scss" scoped>
@import "@/style/tokens.scss";

.collapsed {
    display: flex;
    justify-content: space-between;

    .offer-type {
        display: flex;
        align-items: center;

        .type {
            font-size: 18px;
            font-weight: $semi-bold;
            color: $base-text;
            text-transform: capitalize;
        }

        .state {
            margin-right: 24px;
            padding: 8px 16px;
            background-color: $gray150;
            border-radius: 8px;
            font-size: 14px;
            text-transform: capitalize;
            color: $gray900;
        }
    }

    .info {
        display: flex;
        align-items: center;
        gap: 40px;

        .divider {
            height: 40px;
            width: 1px;
            background-color: $border;
        }

        .wrap {
            .label {
                margin-bottom: 4px;
                font-size: 12px;
                color: $gray600;
            }
        }

        .limit,
        .margin {
            font-size: 15px;
            color: $gray700;
        }

        @media only screen and (max-width: 550px) {
            .divider {
                display: none;
            }
            .limit {
                padding: 8px 16px;
                margin-bottom: 4px;
                border-radius: 8px;
                background-color: $gray150;
            }
        }
    }

    .price {
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: 40px;

        .value {
            font-size: 16px;
            font-weight: 600;
            color: $base-text;
            font-weight: $semi-bold;
        }

        button {
            background-color: $gray300;
            color: $base-text;
            border: none;
            font-family: inherit;
            font-weight: 600;
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
