<template>
    <div class="main-wrap">
        <p>Create Offer</p>
        <div class="buy-sell">
            <button
                v-on:click="offerType = 0"
                v-bind:class="{ focus: offerType == 0 }"
            >
                Buy
            </button>
            <div class="separator"></div>
            <button
                v-on:click="offerType = 1"
                v-bind:class="{ focus: offerType == 1 }"
            >
                Sell
            </button>
        </div>
        <div class="card">
            <div class="currency">
                <div class="filter">
                    <label for="crypto">Crypto</label>
                    <select class="bg-gray300" name="crypto" id="crypto">
                        <option value="UST">UST</option>
                    </select>
                </div>
                <div class="filter">
                    <label for="currency">Currency (FIAT)</label>
                    <select
                        class="bg-gray300"
                        name="currency"
                        id="currency"
                        v-model="fiatCurrency"
                    >
                        <option value="ARS">ARS</option>
                        <option value="BRL">BRL</option>
                        <option value="COP">COP</option>
                    </select>
                </div>
            </div>
            <div class="divider"></div>
            <div class="wrap-price">
                <div class="margin">
                    <label for="">Margin</label>
                    <select class="bg-gray300">
                        <option value="ARS">Above</option>
                        <option value="BRL">Below</option>
                    </select>
                </div>
                <div class="margin-offset">
                    <label for="currency">Margin Offset</label>
                    <input type="text" placeholder="0%" />
                </div>
            </div>

            <div class="min-max">
                <div class="wrap">
                    <label>Min amount:</label>
                    <CurrencyInput
                        v-model="minAmount"
                        :placeholder="this.cryptoPlaceholder"
                        :options="{
                            currency: 'UST',
                            currencyDisplay: 'code',
                            hideCurrencySymbolOnFocus: false,
                            hideGroupingSeparatorOnFocus: false,
                            precision: 2,
                        }"
                        ref="minAmount"
                    />
                </div>
                <div class="wrap">
                    <label>Max amount:</label>
                    <CurrencyInput
                        v-model="maxAmount"
                        :placeholder="this.cryptoPlaceholder"
                        :options="{
                            currency: 'UST',
                            currencyDisplay: 'code',
                            hideCurrencySymbolOnFocus: false,
                            hideGroupingSeparatorOnFocus: false,
                            precision: 2,
                        }"
                        ref="maxAmount"
                    />
                </div>
            </div>
        </div>
        <div class="btns">
            <button class="secondary" @click="$emit('cancel')">Cancel</button>
            <button class="primary" @click="createOffer()" :disabled="!valid">
                Create
            </button>
        </div>
    </div>
</template>

<script>
import { defineComponent } from "vue";
import CurrencyInput from "../CurrencyInput.vue";
import { formatAddress, formatAmount } from "@/shared";
import { mapActions, mapGetters } from "vuex";

export default defineComponent({
    name: "CreateOffer",
    components: {
        CurrencyInput,
    },
    data() {
        return {
            minAmount: 0,
            maxAmount: 0,
            offerType: 0,
            fiatCurrency: "ARS",
        };
    },
    methods: {
        ...mapActions(["initWallet", "newOffer"]),
        formatAmount,
        formatAddress,
        createOffer() {
            let offerType = this.offerType === 0 ? "buy" : "sell";
            const newOffer = {
                create: {
                    offer: {
                        offer_type: offerType,
                        fiat_currency: this.fiatCurrency,
                        rate: "1", // TODO add real rate
                        min_amount: parseInt(this.minAmount * 1000000) + "",
                        max_amount: parseInt(this.maxAmount * 1000000) + "",
                        maker_contact: "TODO",
                    },
                },
            };
            this.newOffer({ offer: newOffer });
        },
    },
    computed: {
        ...mapGetters(["walletAddress"]),
        valid: function() {
            return this.maxAmount > this.minAmount;
        },
    },
    created() {
        this.initWallet();
    },
});
</script>

<style lang="scss" scoped>
@import "../../style/tokens.scss";
@import "../../style/elements.scss";

.main-wrap {
    display: inline-flex;
    flex-direction: column;
}

.buy-sell {
    display: flex;
    margin: 24px 0 24px;
}

.divider {
    width: 100%;
    height: 1px;
    background-color: $border;
    margin: 32px 0;
}

.wrap-price {
    display: flex;
    justify-items: center;
    align-content: center;
    gap: 24px;
    margin-bottom: 24px;

    .margin,
    .margin-offset {
        width: 100%;
        display: flex;
        flex-direction: column;
        gap: 8px;

        label {
            font-size: 14px;
            font-weight: 400;
            color: $gray900;
        }
    }

    input {
        width: 100%;
        background-color: $background;
    }
}

.min-max {
    display: inline-flex;
    flex-basis: content;

    .wrap {
        display: flex;
        flex-direction: column;

        &:last-child {
            margin-left: 24px;
        }

        label {
            font-size: 14px;
            font-weight: 400;
            color: $gray900;
            margin-bottom: 8px;
        }
    }

    input {
        width: 100%;
        background-color: $background;
    }
}

.btns {
    display: flex;
    justify-content: flex-end;
    gap: 24px;
    margin-top: 24px;
}

.currency {
    display: flex;
    .filter {
        display: flex;
        flex-direction: column;
        width: 100%;

        &:last-child {
            margin-left: 24px;
        }

        label {
            font-size: 14px;
            font-weight: 400;
            color: $gray900;
            margin-bottom: 8px;
        }

        @media only screen and (max-width: 550px) {
            margin-left: 0;
            max-width: none;

            select {
                max-width: none;
                height: 48px;
            }
        }
    }
}
</style>
