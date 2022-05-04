import {
    Coin,
    Coins,
    Extension,
    LCDClient,
    MsgExecuteContract,
    StdSignature,
    StdSignMsg,
    StdTx,
} from "@terra-money/terra.js";
import { FACTORY_CONTRACT } from "@/constants";
import router from "@/router";
import { updateTrade } from "@/store/firebase";
import { newTrade } from "../firebase";

const lcdOptions = {
    URL: "https://bombay-lcd.terra.dev", // URL: "http://143.244.190.1:3060",
    chainID: "bombay-12", // chainID: "localterra"
};
let terra = new LCDClient(lcdOptions);
const ext = new Extension();

const state = {
    loading: {
        isLoading: false,
        label: "Loading...",
        transaction: "Follow the transaction",
    },
    walletAddress: "",
    showLoadingOffers: false,
    offers: [],
    offersFilter: {
        type: "",
        fiatCurrency: "",
    },
    myOffers: [],
    tradeInfos: [],
    lunaUstPrice: 0,
    ustUsdPrice: 0,
    factoryConfig: {
        trade_code_id: 0,
        token_addr: "",
        local_ust_pool_addr: "",
        gov_addr: "",
        offers_addr: "",
        fee_collector_addr: "",
        trading_incentives_addr: "",
    },
};

// eslint-disable-next-line no-unused-vars
function prepareTransaction(signedMsg) {
    const { public_key, signature, stdSignMsgData } = signedMsg;
    const sig = StdSignature.fromData({
        signature,
        pub_key: {
            type: "tendermint/PubKeySecp256k1",
            value: public_key,
        },
    });

    const stdSignMsg = StdSignMsg.fromData(stdSignMsgData);
    return new StdTx(stdSignMsg.msgs, stdSignMsg.fee, [sig], stdSignMsg.memo);
}

const getters = {
    walletAddress: (state) => state.walletAddress,
    showLoadingOffers: (state) => state.showLoadingOffers,
    offers: (state) => state.offers,
    offersFilter: (state) => state.offersFilter,
    myOffers: (state) => state.myOffers,
    getOfferById: (state) => (id) => {
        return state.offers.find((offer) => offer.id === id);
    },
    trades: (state) => state.tradeInfos,
    getTradeInfo: (state) => (tradeAddr) => {
        return state.tradeInfos.find(
            (tradeInfo) => tradeInfo.trade.addr === tradeAddr,
        );
    },
    lunaUstPrice: (state) => state.lunaUstPrice,
    ustUsdPrice: (state) => state.ustUsdPrice,
    loading: (state) => state.loading,
};

const actions = {
    async initWallet({ commit, dispatch }) {
        const ext = new Extension();
        const res = await ext.request("connect");
        const info = await ext.request("info");
        terra = new LCDClient({
            URL: info.payload.lcd,
            chainID: info.payload.chainID,
        });
        const walletAddress = res.payload.address;
        commit("setWalletAddress", walletAddress);
        dispatch("fetchFactoryConfig");
    },
    /**
     * Fetch Factory Contract config
     */
    async fetchFactoryConfig({ commit, dispatch }) {
        const cfgQuery = { config: {} };
        const factoryConfig = await terra.wasm.contractQuery(
            FACTORY_CONTRACT,
            cfgQuery,
        );
        commit("setFactoryConfig", factoryConfig);
        dispatch("fetchTradeInfos");
    },
    /**
     * Fetch Offer by Id
     */
    async fetchOffer({ commit }, { id }) {
        const offerQuery = { offer: { id } };
        const offer = await terra.wasm.contractQuery(
            state.factoryConfig.offers_addr,
            offerQuery,
        );
        commit("addOffer", offer);
    },
    /**
     * Fetch Offers.
     */
    async fetchMyOffers(
        { commit, getters },
        { paginated = false, order = "desc" },
    ) {
        const offers = paginated ? getters.myOffers : [];

        const last_offer_id =
            offers.length > 0 && paginated
                ? offers[offers.length - 1].id
                : undefined;

        // For correct pagination we max set the Bound depending on order direction
        let min, max;
        if (order === "asc") {
            min = last_offer_id;
        } else {
            // order === "desc"
            max = last_offer_id;
        }

        const offersQuery = {
            offers_query: {
                owner: getters.walletAddress,
                limit: 10,
                min,
                max,
                order,
            },
        };

        const loadedOffers = await terra.wasm.contractQuery(
            state.factoryConfig.offers_addr,
            offersQuery,
        );
        commit("setMyOffers", offers.concat(loadedOffers));
    },
    /**
     * Fetch Offers.
     */
    async fetchOffers(
        { commit, getters, dispatch },
        { fiatCurrency, offerType, paginated = false, order = "desc" },
    ) {
        commit("setLoadingOffers", true);
        if (
            getters.offersFilter.type !== offerType ||
            getters.offersFilter.fiatCurrency !== fiatCurrency
        ) {
            commit("setOffers", []);
        }
        // fetchOffers depends on the fetchFactoryConfig
        // TODO we should call the fetchFactoryConfig on the start of the application,
        //  but we need to fix the initWallet first.
        await dispatch("fetchFactoryConfig");
        const offers = paginated ? getters.offers : [];
        const last_offer_id =
            offers.length > 0 && paginated
                ? offers[offers.length - 1].id
                : undefined;

        // For correct pagination we max set the Bound depending on order direction
        let min, max;
        if (order === "asc") {
            min = last_offer_id;
        } else {
            // order === "desc"
            max = last_offer_id;
        }

        const offersQuery = {
            offers_by_type_fiat: {
                fiat_currency: fiatCurrency,
                offer_type: offerType,
                min,
                max,
                limit: 10,
                order,
            },
        };
        const loadedOffers = await terra.wasm.contractQuery(
            state.factoryConfig.offers_addr,
            offersQuery,
        );
        commit("setOffers", offers.concat(loadedOffers));
        commit("setOffersFilter", {
            type: offerType,
            fiatCurrency: fiatCurrency,
        });
        commit("setLoadingOffers", false);
    },
    /**
     * Create Offer
     */
    async newOffer({ commit, getters, dispatch }, { offer }) {
        const offerMsg = new MsgExecuteContract(
            getters.walletAddress,
            state.factoryConfig.offers_addr,
            offer,
        );

        await executeMsg(commit, getters, dispatch, offerMsg);
        router.push(`/`);
    },
    /**
     * Update Offer
     */
    async updateOffer({ commit, getters, dispatch }, { updatedOffer }) {
        const { id, rate, min_amount, max_amount } = updatedOffer;

        /** @type {OfferUpdateMsg} */
        const offerUpdateMsg = {
            id,
            rate,
            min_amount: min_amount * 1000000 + "",
            max_amount: max_amount * 1000000 + "",
            state: updatedOffer.state, // state is already in scope
        };

        /** @type {ExecuteUpdateMsg} */
        const update_offer = {
            offer_update: offerUpdateMsg,
        };

        console.log("update_offer :>> ", update_offer);

        const msg = new MsgExecuteContract(
            getters.walletAddress,
            state.factoryConfig.offers_addr,
            {
                update_offer,
            },
        );
        console.log("offerMsg msg:>> ", msg);
        const result = await executeMsg(commit, getters, dispatch, msg);
        console.log("result :>> ", result);
        router.push(`/`);
    },
    /**
     * Fetch a specific Trade
     */
    async fetchTradeInfo({ commit, getters, dispatch }, { addr, tradeData }) {
        const tradeInfo = {};
        const trade = await terra.wasm.contractQuery(addr, { state: {} });
        tradeInfo.trade = trade;

        tradeInfo.offer = getters.getOfferById(trade.offer_id);
        if (!tradeInfo.offer) {
            await dispatch("fetchOffer", { id: trade.offer_id });
            tradeInfo.offer = getters.getOfferById(trade.offer_id);
        }

        if (tradeData) {
            Object.assign(tradeInfo.trade, tradeData);
        }

        commit("addTradeInfo", tradeInfo);

        return tradeInfo;
    },
    /**
     * Fetches all trades for given Trader (maker or taker) address.
     */
    async fetchTradeInfos({ commit, getters }, redirect = false) {
        const wallet = getters.walletAddress;
        // TODO Add pagination
        const trades_as_seller = await terra.wasm.contractQuery(
            state.factoryConfig.offers_addr,
            { trades_query: { user: wallet, index: "seller", limit: 100 } },
        );
        // TODO Add pagination
        const trades_as_buyer = await terra.wasm.contractQuery(
            state.factoryConfig.offers_addr,
            { trades_query: { user: wallet, index: "buyer", limit: 100 } },
        );

        const trades = trades_as_buyer.concat(trades_as_seller);

        commit("setTradeInfos", trades);
        if (redirect) {
            router.push("/trades");
        }
    },
    /**
     * Sends a transaction to instantiate a Trade contract.
     * @param {*} offerId Id of the Offer provided by the Offers Smart Contract.
     * @param {*} amount Amount of UST to be traded.
     */
    async openTrade({ commit, getters, dispatch }, { offer, ustAmount }) {
        let sender = getters.walletAddress;
        const amount = ustAmount * 1000000;
        const newTradeMsg = {
            new_trade: {
                offer_id: offer.id,
                ust_amount: amount + "",
                counterparty: sender,
                taker: sender, //TODO
                taker_contact: "@TODO",
                //arbitrator: TODO,
            },
        };
        const createTradeMsg = new MsgExecuteContract(
            sender,
            state.factoryConfig.offers_addr,
            newTradeMsg,
        );
        //TODO: Error handling.
        await executeMsg(commit, getters, dispatch, createTradeMsg);

        dispatch("fetchTradeInfos", true);
        newTrade(offer.owner, newTradeMsg);
    },
    async acceptTradeRequest({ commit, getters, dispatch }, tradeAddr) {
        const fiatDeposited = new MsgExecuteContract(
            getters.walletAddress,
            tradeAddr,
            {
                accept_request: {},
            },
        );
        await executeMsg(commit, getters, dispatch, fiatDeposited);

        let tradeInfo = await dispatch("fetchTradeInfo", { addr: tradeAddr });
        await updateTrade(tradeInfo.trade);
    },
    async cancelTradeRequest({ commit, getters, dispatch }, tradeAddr) {
        const fiatDeposited = new MsgExecuteContract(
            getters.walletAddress,
            tradeAddr,
            {
                cancel_request: {},
            },
        );
        await executeMsg(commit, getters, dispatch, fiatDeposited);

        let tradeInfo = await dispatch("fetchTradeInfo", { addr: tradeAddr });
        await updateTrade(tradeInfo.trade);
    },
    async cancelTradeAfterFunds({ commit, getters, dispatch }, tradeAddr) {
        const fiatDeposited = new MsgExecuteContract(
            getters.walletAddress,
            tradeAddr,
            {
                cancel_trade: {},
            },
        );
        await executeMsg(commit, getters, dispatch, fiatDeposited);

        let tradeInfo = await dispatch("fetchTradeInfo", { addr: tradeAddr });
        await updateTrade(tradeInfo.trade);
    },
    async fundEscrow({ commit, getters, dispatch }, tradeAddr) {
        let tradeInfo = getters.getTradeInfo(tradeAddr);
        const ustAmount = tradeInfo.trade.ust_amount;
        const ust = Coin.fromData({ denom: "uusd", amount: ustAmount });

        const localTerraFee = Coin.fromData({
            denom: "uusd",
            amount: ustAmount * 0.01,
        });
        let ltFeeTax = await terra.utils.calculateTax(localTerraFee);
        let releaseTax = await terra.utils.calculateTax(ust);
        ltFeeTax = parseInt(ltFeeTax.toData().amount);
        releaseTax = parseInt(releaseTax.toData().amount);

        let fundEscrowAmount =
            parseInt(ustAmount) +
            parseInt(localTerraFee.amount) +
            ltFeeTax +
            releaseTax;
        fundEscrowAmount = Coin.fromData({
            denom: "uusd",
            amount: fundEscrowAmount,
        });
        const coins = new Coins([fundEscrowAmount]);
        const fundMsg = { fund_escrow: {} };
        const fundEscrowMsg = new MsgExecuteContract(
            getters.walletAddress,
            tradeAddr,
            fundMsg,
            coins,
        );
        await executeMsg(commit, getters, dispatch, fundEscrowMsg);

        tradeInfo = await dispatch("fetchTradeInfo", { addr: tradeAddr });
        await updateTrade(tradeInfo.trade);
    },
    async setFiatDeposited({ commit, getters, dispatch }, tradeAddr) {
        const fiatDeposited = new MsgExecuteContract(
            getters.walletAddress,
            tradeAddr,
            {
                fiat_deposited: {},
            },
        );
        await executeMsg(commit, getters, dispatch, fiatDeposited);

        let tradeInfo = await dispatch("fetchTradeInfo", { addr: tradeAddr });
        await updateTrade(tradeInfo.trade);
    },
    async releaseEscrow({ commit, getters, dispatch }, tradeAddr) {
        const releaseMsg = new MsgExecuteContract(
            getters.walletAddress,
            tradeAddr,
            {
                release_escrow: {},
            },
        );
        //TODO: Error handling
        await executeMsg(commit, getters, dispatch, releaseMsg);

        let tradeInfo = await dispatch("fetchTradeInfo", { addr: tradeAddr });
        await updateTrade(tradeInfo.trade);
    },
    async refundEscrow({ commit, getters, dispatch }, tradeAddr) {
        const refundMsg = new MsgExecuteContract(
            getters.walletAddress,
            tradeAddr,
            {
                refund_escrow: {},
            },
        );
        await executeMsg(commit, getters, dispatch, refundMsg);
        let tradeInfo = await dispatch("fetchTradeInfo", { addr: tradeAddr });
        await updateTrade(tradeInfo.trade);
    },
    async openDispute({ commit, getters, dispatch }, tradeAddr) {
        // TODO open dispute does not work.
        const disputeEscrowMSg = new MsgExecuteContract(
            getters.walletAddress,
            tradeAddr,
            {
                dispute_escrow: {},
            },
        );
        await executeMsg(commit, getters, dispatch, disputeEscrowMSg);
        let tradeInfo = await dispatch("fetchTradeInfo", { addr: tradeAddr });
        await updateTrade(tradeInfo.trade);
    },
    async fetchLunaPrice({ commit }) {
        const res = await fetch(`${lcdOptions.URL}/v1/market/swaprate/uluna`);
        const priceData = await res.json();
        const lunaUstPrice = priceData.find((p) => p.denom === "uusd").swaprate;
        commit("setLunaUstPrice", parseFloat(lunaUstPrice).toFixed(2));
    },
    async fetchUstUsdPrice({ commit }) {
        const res = await fetch(
            "https://api.coinpaprika.com/v1/tickers/ust-terrausd?quotes=USD",
        );
        const ustPriceData = await res.json();
        const ustUsdPrice = ustPriceData.quotes["USD"].price;
        commit("setUstUsdPrice", ustUsdPrice.toFixed(2));
    },
};

async function executeMsg(commit, getters, dispatch, msg) {
    if (getters.walletAddress === "") {
        dispatch("initWallet");
        return;
    }
    return new Promise((resolve) => {
        ext.once("onPost", async (res) => {
            if (res.success) {
                commit("setLoadingTransaction", res.result.txhash);
                commit("setIsLoading", true);
            }
            let interval = setInterval(async () => {
                console.log("res", res);
                let txInfo = await terra.tx.txInfo(res.result.txhash);
                if (txInfo) {
                    resolve(txInfo);
                    clearInterval(interval);
                    commit("setIsLoading", false);
                }
            }, 1000);
        });
        ext.post({
            msgs: [msg],
        });
        /*
    //Suddenly stopped working (at least on Terrarium, needs to be tested on TestNet, MainNet.
    //Or hopefully we'll be able to use the "auto" fee option.
    terra.tx.estimateFee(getters.walletAddress, [msg]).then((stdFee) => {
      ext.post({
        fee: stdFee,
        msgs: [msg]
      })
    })
     */
    });
}

const mutations = {
    setWalletAddress: (state, walletAddress) =>
        (state.walletAddress = walletAddress),
    setFactoryConfig: (state, factoryConfig) =>
        (state.factoryConfig = factoryConfig),
    addOffer: (state, offer) => state.offers.push(offer),
    setLoadingOffers: (state, showLoadingOffers) =>
        (state.showLoadingOffers = showLoadingOffers),
    setOffers: (state, offers) => (state.offers = offers),
    setOffersFilter: (state, offersFilter) =>
        (state.offersFilter = offersFilter),
    setMyOffers: (state, offers) => (state.myOffers = offers),
    addTradeInfo: (state, tradeInfo) => {
        const addedTradeInfo = state.tradeInfos.find(
            (t) => t.trade.addr === tradeInfo.trade.addr,
        );
        if (addedTradeInfo) {
            Object.assign(addedTradeInfo, tradeInfo);
        } else {
            state.tradeInfos.push(tradeInfo);
        }
        state.tradeInfos = [...state.tradeInfos];
    },
    setIsLoading: (state, isLoading) => {
        state.loading.isLoading = isLoading;
    },
    setLoadingLabel: (state, label) => {
        state.loading.label = label;
    },
    setLoadingTransaction: (state, transaction) => {
        state.loading.transaction = transaction;
    },
    setTradeInfos: (state, tradeInfos) => {
        state.tradeInfos = [...tradeInfos];
    },
    setLunaUstPrice: (state, price) => (state.lunaUstPrice = price),
    setUstUsdPrice: (state, price) => (state.ustUsdPrice = price),
};

export default {
    state,
    getters,
    actions,
    mutations,
};

/**
 * @typedef OfferMsg
 * @type {object}
 * @property {string} offer_type - buy/sell.
 * @property {string} fiat_currency - ARS/BRL/COP/..
 * @property {string} rate - Exchange rate Fiat / Crypto e.g. 43500
 * @property {string} min_amount - Minimum Amount in uusd
 * @property {string} max_amount - Maximum Amount in uusd
 * @property {string} maker_contact - Contact information for Maker
 */

/**
 * @typedef OfferUpdateMsg
 * @type {object}
 * @property {string} id - The id is the rate concatenated with a auto inc number, e.g. 43500_1
 * @property {string} rate - Exchange rate Fiat / Crypto e.g. 43500
 * @property {string} min_amount - Minimum Amount in uusd
 * @property {string} max_amount - Maximum Amount in uusd
 * @property {string} state -  Amount in uusd
 */

/**
 * @typedef ExecuteUpdateMsg
 * @type {object}
 * @property {OfferUpdateMsg} offer_update - The OfferUpdateMsg payload
 */
