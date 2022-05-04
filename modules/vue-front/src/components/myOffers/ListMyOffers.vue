<template>
    <section>
        <!-- My Offers section-->
        <section v-if="hasOffers" class="offers-list">
            <!-- Offers for -->
            <ul>
                <li class="card" v-for="offer in offers" v-bind:key="offer.id">
                    <!-- Collapsed Offer -->
                    <CollapsedMyOffer
                        v-if="!offer.isExpanded"
                        :offer="offer"
                        v-on:select="expandOfferItem"
                    />
                    <!-- Expanded Offer Desktop -->
                    <ExpandedMyOffer
                        v-else
                        :offer="offer"
                        v-on:cancel="collapseOfferItem"
                    />
                </li>
            </ul>
            <div class="load-more">
                <button class="wallet" @click="loadMore()">
                    Load more offers
                </button>
            </div>
        </section>
        <section v-else class="card">
            <p>
                Nothing here yet
            </p>
        </section>
        <!-- End My Offers section -->
        <!-- Archived offers table -->
        <h3 v-if="hasArchivedOffers">Archived Offers</h3>
        <section v-if="hasArchivedOffers" class="archived-offers-table card">
            <div class="table-header">
                <div class="col-1"><p>Type</p></div>
                <div class="col-2"><p>Date</p></div>
                <div class="col-3"><p>Fiat</p></div>
                <div class="col-4"><p>Limits</p></div>
                <div class="col-5"><p>Price</p></div>
                <div class="col-6"></div>
            </div>
            <ArchivedOfferItem
                v-for="offer in archivedOffers"
                :offer="offer"
                :key="offer.id"
            />
        </section>
        <!--End Archived offers table-->
    </section>

    <!-- Expanded Offer Mobile -->
    <div class="expanded-mobile" v-if="false">
        <div class="owner">
            <p class="wallet">terra12242343</p>
            <p class="n-trades">352 trades</p>
        </div>

        <div class="payment-info">
            <p class="note">
                Nubank, Itaú, C6, Mercado Pago, Inter, Caixa, Bradesco
            </p>
        </div>

        <form action="">
            <div class="input">
                <label for="buy">I want to buy</label>
                <input type="text" placeholder="100.00" ref="buyAmountInput" />
            </div>
            <div class="input">
                <label for="sell">I will receive</label>
                <input type="text" placeholder="100.00" />
                <p>Min - 1 | Max - 50</p>
            </div>
        </form>

        <div class="receipt">
            <div class="price">
                <div class="wrap-price">
                    <p class="label">Price</p>
                    <p class="ticker">Will refresh in 47s</p>
                </div>
                <div class="wrap">
                    <p class="margin">4% above market</p>
                    <p class="value">COL$ 3.695,59</p>
                </div>
            </div>

            <div class="sumary">
                <p class="label">Transaction sumary</p>
                <div class="wrap">
                    <div class="item">
                        <p class="info">Trading Fee</p>
                        <p>COL$ 3.695,59</p>
                    </div>
                    <div class="item">
                        <p class="info">You will get</p>
                        <p class="price-get">100.00 UST</p>
                    </div>
                    <div class="item">
                        <p class="info">You will pay</p>
                        <p class="price-pay">COP$ 348.892,53</p>
                    </div>
                </div>
            </div>
        </div>

        <div class="wrap-btns">
            <button class="secondary">cancel</button>
            <button class="primary">open transaction</button>
        </div>
    </div>
    <!-- Expanded Offer Mobile -->
</template>

<script>
import { defineComponent } from "vue";
import { mapActions, mapGetters } from "vuex";
import { formatAddress, formatAmount } from "@/shared";
import ExpandedMyOffer from "@/components/myOffers/ExpandedMyOffer.vue";
import CollapsedMyOffer from "@/components/myOffers/CollapsedMyOffer.vue";
import ArchivedOfferItem from "@/components/myOffers/AchivedOfferItem.vue";

export default defineComponent({
    name: "ListMyOffers",
    components: {
        ExpandedMyOffer,
        CollapsedMyOffer,
        ArchivedOfferItem,
    },
    data() {
        return {
            ExpandedMyOffer: null,
        };
    },
    mounted: async function() {
        await this.fetchMyOffers({});
    },
    methods: {
        ...mapActions(["fetchMyOffers", "fetchUsdRates", "openTrade"]),
        formatAmount,
        formatAddress,
        expandOfferItem: function(offer) {
            if (this.ExpandedMyOffer !== offer) {
                if (this.ExpandedMyOffer != null) {
                    this.ExpandedMyOffer.isExpanded = false;
                }
                offer.isExpanded = true;
                this.ExpandedMyOffer = offer;
            }
        },
        collapseOfferItem: function(offer) {
            offer.isExpanded = false;
            this.ExpandedMyOffer = null;
        },
        loadMore: function() {
            this.$nextTick(() => {
                this.fetchMyOffers({
                    paginated: true,
                });
            });
        },
    },
    computed: {
        ...mapGetters(["myOffers", "getUsdRate"]),
        offers: function() {
            let offers = [];
            let myOffers = this.myOffers.filter(
                (offer) => offer.state !== "archive",
            );
            myOffers.forEach((offer) => {
                offer["isExpanded"] = false;
                offers.push(offer);
            });
            return offers;
        },
        hasOffers: function() {
            return this.offers.length > 0;
        },
        archivedOffers: function() {
            return this.myOffers.filter((offer) => offer.state === "archive");
        },
        hasArchivedOffers: function() {
            return this.archivedOffers.length > 0;
        },
    },
    created: function() {
        this.fetchUsdRates();
    },
});
</script>

<style lang="scss" scoped>
@import "@/style/tokens.scss";

/* ----------- BUY SELL ROW */
.separator {
    margin: 0 auto 42px;
    display: flex;
    height: 1px;
    background-color: $border;
}

.offers-filter {
    display: flex;

    @media only screen and (max-width: 550px) {
        display: block;
    }
}

.filter {
    display: inline-flex;
    flex-direction: column;
    width: 100%;
    max-width: 200px;
    margin-left: 24px;

    label {
        font-size: 12px;
        color: $gray600;
        margin-bottom: 8px;
    }

    select {
        width: 100%;
        max-width: 200px;
        background-color: $surface;
        border-radius: 8px;
        border: 1px solid $border;
        font-family: "Poppins", sans-serif;
        font-size: 14px;
        font-weight: 600;
        color: $base-text;
        padding: 10px 16px;
        appearance: none;
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

/* ----------- OFFER LIST */
.offers-list {
    h3 {
        color: $base-text;
        font-size: 18px;
        font-weight: 600;
        margin-bottom: 32px;
    }

    li {
        list-style: none;
        margin-bottom: 24px;
    }

    .load-more {
        display: flex;
        justify-content: center;
        margin-top: 32px;

        button {
            padding: 0 48px;
        }
    }
}

/* -------------- Expanded Mobile */
.expanded-mobile {
    position: absolute;
    width: 100%;
    height: 100vh;
    display: grid;
    grid-template-columns: 1fr;
    background-color: $white;

    .owner {
        grid-column: 1/1;
        grid-row: 1;
        padding: 16px 24px 0;

        .wallet {
            font-size: 18px;
            font-weight: 600;
            color: $base-text;
        }

        .n-trades {
            font-size: 14px;
            color: $gray600;
        }
    }

    .payment-info {
        grid-column: 1/1;
        grid-row: 2;
        padding: 0px 24px;

        .note {
            font-size: 14px;
            color: $gray600;
        }
    }

    form {
        grid-column: 1/1;
        grid-row: 3;
        margin-top: 16px;
        padding: 0px 24px;

        .input:first-child {
            margin-bottom: 24px;
        }

        label {
            font-size: 14px;
            color: $gray600;
            display: block;
        }

        input {
            width: 100%;
            font-family: "Poppins", sans-serif;
            font-size: 16px;
            font-weight: 800;
            line-height: 24px;
            color: $base-text;
            padding: 10px 16px;
            border: 1px solid $border;
            border-radius: 8px;
            margin-top: 8px;
            text-align: right;
        }

        p {
            font-size: 12px;
            color: $gray600;
            text-align: right;
            margin-top: 8px;
        }
    }

    .receipt {
        grid-column: 1/1;
        grid-row: 4;
        margin-top: 16px;
        background-color: $background;
        border-top: 1px solid $border;
        border-bottom: 1px solid $border;
        padding: 16px 24px;

        .price {
            margin-bottom: 24px;

            .wrap-price {
                display: flex;
                justify-content: space-between;
            }

            .label {
                display: inline-block;
                font-size: 14px;
                color: $gray600;
            }

            .ticker {
                display: inline-block;
                font-size: 12px;
                color: $primary;
            }

            .wrap {
                width: 100%;
                display: inline-flex;
                justify-content: space-between;
                border: 1px solid $border;
                background-color: $white;
                border-radius: 8px;
                padding: 10px 24px;
                margin-top: 8px;
                align-items: center;
                gap: 16px;

                .margin {
                    font-size: 14px;
                    max-width: 100px;
                    color: $gray600;
                }

                .value {
                    font-size: 16px;
                    font-weight: 600;
                    color: $base-text;
                }
            }
        }

        .sumary {
            margin-bottom: 24px;

            .label {
                font-size: 14px;
                color: $gray600;
            }

            .wrap {
                width: 100%;
                display: flex;
                flex-direction: column;
                justify-content: space-between;
                border: 1px solid $border;
                background-color: $white;
                border-radius: 8px;
                padding: 16px 24px;
                margin-top: 8px;

                gap: 8px;

                .item {
                    display: inline-flex;
                    justify-content: space-between;

                    .price-get {
                        font-weight: 800;
                    }

                    .price-pay {
                        font-weight: 800;
                        color: $primary;
                    }
                }
            }
        }
    }
}

.wrap-btns {
    grid-template-columns: 1/1;
    grid-row: 5;
    display: flex;
    justify-content: space-around;
    padding: 24px 0px;

    .primary {
        background-color: #ef6100;
        color: $white;
        border: none;
        font-family: inherit;
        font-weight: 700;
        font-size: 16px;
        text-transform: lowercase;
        padding: 8px 24px;
    }

    .secondary {
        color: $primary;
        border: none;
        font-family: inherit;
        font-weight: 700;
        font-size: 16px;
        text-transform: lowercase;
        padding: 8px 24px;
    }
}

/* ----------- ARCHIVED OFFERS TABLE */
.archived-offers-table {
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
    width: 12.5%;
}

.col-2,
:deep(.col-2) {
    width: 16%;
}

.col-3,
:deep(.col-3) {
    width: 12.5%;
}

.col-4,
:deep(.col-4) {
    width: 25%;
}

.col-5,
:deep(.col-5) {
    width: 24%;
}

.col-6,
:deep(.col-6) {
    width: 10%;
}

/* ----------- END ARCHIVED OFFERS TABLE */
</style>
