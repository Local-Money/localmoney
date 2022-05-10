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
import { FACTORY_CFG } from "../../constants/factoryCfg.js";
import router from "@/router";
import { updateTrade } from "@/store/firebase";
// import { sleep } from "../../shared.js";
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
    showLoadingMyOffers: false,
    myOffers: [],
    tradeInfos: [],
    lunaUstPrice: 0,
    ustUsdPrice: 0,
    stakingTotalDeposit: "0",
    stakingTotalShares: "0",
    stakingTotalWarming: "0",
    stakingClaims: [],
    factoryCfg: FACTORY_CFG.factoryCfg,
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
    showLoadingMyOffers: (state) => state.showLoadingMyOffers,
    myOffers: (state) => state.myOffers,
    stakingTotalDeposit: (state) => state.stakingTotalDeposit,
    stakingTotalShares: (state) => state.stakingTotalShares,
    stakingTotalWarming: (state) => state.stakingTotalWarming,
    stakingClaims: (state) => state.stakingClaims,
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
    factoryCfg: (state) => state.factoryCfg,
};

const actions = {
    async initWallet({ commit }) {
        console.log("initWallet");
        const ext = new Extension();
        const res = await ext.request("connect");
        const info = await ext.request("info");
        terra = new LCDClient({
            URL: info.payload.lcd,
            chainID: info.payload.chainID,
        });
        const walletAddress = res.payload.address;
        commit("setWalletAddress", walletAddress);
        // dispatch("fetchfactoryCfg");
    },
    /**
     * Fetch Factory Contract config
     */
    // async fetchfactoryCfg({ commit, dispatch }) {
    //     const cfgQuery = { config: {} };
    //     const factoryCfg = await terra.wasm.contractQuery(
    //         FACTORY_CONTRACT,
    //         cfgQuery,
    //     );
    //     console.log("factoryCfg :>> ", factoryCfg);
    //     commit("setfactoryCfg", factoryCfg);
    //     dispatch("fetchTradeInfos");
    // },
    /**
     *
     * @param {*} VuexContext
     * @param {String} amount - The amount to stake in uLOCAL
     * @returns
     */
    async enterStaking({ commit, getters, dispatch }, amount) {
        const enterStakingMsg = new MsgExecuteContract(
            getters.walletAddress,
            getters.factoryCfg.local_token_addr, // LOCAL_TOKEN_ADDR
            {
                send: {
                    contract: state.factoryCfg.staking_addr,
                    amount,
                    msg: "ewogICJlbnRlciI6IHt9Cn0=", // { Enter: {}}
                },
            },
        );

        console.log("enterStakingMsg :>> ", enterStakingMsg);
        const result = await executeMsg(
            commit,
            getters,
            dispatch,
            enterStakingMsg,
        );
        console.log("result :>> ", result);

        return result;
    },
    /**
     *
     * @param {*} VuexContext
     * @param {String} amount - The amount to stake in uLOCAL
     * @returns
     */
    async leaveStaking({ commit, getters, dispatch }, amount) {
        const leaveStakingMsg = new MsgExecuteContract(
            getters.walletAddress,
            getters.factoryCfg.xlocal_addr,
            {
                send: {
                    contract: state.factoryCfg.staking_addr,
                    amount,
                    msg: "ewogICJsZWF2ZSI6IHt9Cn0=", // {Leave:{}}
                },
            },
        );

        console.log("leaveStakingMsg :>> ", leaveStakingMsg);
        const result = await executeMsg(
            commit,
            getters,
            dispatch,
            leaveStakingMsg,
        );
        console.log("result :>> ", result);
        return result;
    },

    /**
     *
     * @param {*} VuexContext
     * @param {Number} claim_id - The id of the claim
     * @returns
     */
    async claimStaking({ commit, getters, dispatch }, claim_id) {
        const claimStakingMsg = new MsgExecuteContract(
            getters.walletAddress,
            state.factoryCfg.staking_addr,
            {
                claim: {
                    claim_id,
                },
            },
        );

        console.log("claimStakingMsg :>> ", claimStakingMsg);
        const result = await executeMsg(
            commit,
            getters,
            dispatch,
            claimStakingMsg,
        );
        console.log("result :>> ", result);
        return result;
    },
    /**
     * Fetch Staking Total Deposit
     */
    async fetchStakingTotalDeposit({ commit }) {
        const msg = {
            total_deposit: {},
        };

        const stakingTotalDeposit = await terra.wasm.contractQuery(
            state.factoryCfg.staking_addr,
            msg,
        );
        console.log("stakingTotalDeposit :>> ", stakingTotalDeposit);
        commit("setStakingTotalDeposit", stakingTotalDeposit);
    },
    /**
     * Fetch Staking Total Shares
     */
    async fetchStakingTotalShares({ commit }) {
        const msg = {
            total_shares: {},
        };

        const stakingTotalShares = await terra.wasm.contractQuery(
            state.factoryCfg.staking_addr,
            msg,
        );
        console.log("state :>> ", state);
        commit("setStakingTotalShares", stakingTotalShares);
    },
    /**
     * Fetch Staking Total Warming
     */
    async fetchStakingTotalWarming({ commit }) {
        const msg = {
            total_warming: {},
        };

        const stakingTotalWarming = await terra.wasm.contractQuery(
            state.factoryCfg.staking_addr,
            msg,
        );
        commit("setStakingTotalWarming", stakingTotalWarming);
    },
    /**
     * Fetch Staking Claims
     */
    async fetchStakingClaims({ commit, getters }) {
        const msg = {
            claims: {
                recipient: getters.walletAddress,
            },
        };

        const stakingClaims = await terra.wasm.contractQuery(
            state.factoryCfg.staking_addr,
            msg,
        );
        commit("setStakingClaims", stakingClaims);
    },
    /**
     * Fetch Offer by Id
     */
    async fetchOffer({ commit }, { id }) {
        const offerQuery = { offer: { id } };
        const offer = await terra.wasm.contractQuery(
            state.factoryCfg.offers_addr,
            offerQuery,
        );
        commit("addOffer", offer);
    },
    /**
     * Fetch My Offers.
     */
    async fetchMyOffers(
        { commit, getters },
        { paginated = false, order = "desc" },
    ) {
        commit("setLoadingMyOffers", true);
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
            state.factoryCfg.offers_addr,
            offersQuery,
        );
        commit("setMyOffers", offers.concat(loadedOffers));
        commit("setLoadingMyOffers", false);
    },
    /**
     * Fetch Offers.
     */
    async fetchOffers(
        { commit, getters },
        { fiatCurrency, offerType, paginated = false, order = "desc" },
    ) {
        commit("setLoadingOffers", true);
        if (
            getters.offersFilter.type !== offerType ||
            getters.offersFilter.fiatCurrency !== fiatCurrency
        ) {
            commit("setOffers", []);
        }
        // fetchOffers depends on the fetchfactoryCfg
        // TODO we should call the fetchfactoryCfg on the start of the application,
        //  but we need to fix the initWallet first.
        // await dispatch("fetchfactoryCfg");
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
            state.factoryCfg.offers_addr,
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
        commit("setLoadingTransaction", "");
        commit("setLoadingLabel", "Please wait for the wallet...");
        commit("setIsLoading", true);
        const offerMsg = new MsgExecuteContract(
            getters.walletAddress,
            state.factoryCfg.offers_addr,
            offer,
        );

        await executeMsg(commit, getters, dispatch, offerMsg);
        await dispatch("fetchMyOffers", { paginated: false, order: "desc" });
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
            state.factoryCfg.offers_addr,
            {
                update_offer,
            },
        );

        await executeMsg(commit, getters, dispatch, msg);
        await dispatch("fetchMyOffers", { paginated: false, order: "desc" });
    },
    /**
     * Unachive Offer
     */
    async unarchiveOffer({ commit, getters, dispatch }, offer) {
        const { id, rate, min_amount, max_amount } = offer;
        /** @type {OfferUpdateMsg} */
        const offerUpdateMsg = {
            id,
            rate,
            min_amount,
            max_amount,
            state: "paused",
        };

        /** @type {ExecuteUpdateMsg} */
        const update_offer = {
            offer_update: offerUpdateMsg,
        };

        const msg = new MsgExecuteContract(
            getters.walletAddress,
            state.factoryCfg.offers_addr,
            {
                update_offer,
            },
        );
        console.log(msg);
        await executeMsg(commit, getters, dispatch, msg);
        await dispatch("fetchMyOffers", { paginated: false, order: "desc" });
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
            state.factoryCfg.offers_addr,
            { trades_query: { user: wallet, index: "seller", limit: 100 } },
        );
        // TODO Add pagination
        const trades_as_buyer = await terra.wasm.contractQuery(
            state.factoryCfg.offers_addr,
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
            state.factoryCfg.offers_addr,
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
                commit("setLoadingLabel", "");
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
    setfactoryCfg: (state, factoryCfg) => (state.factoryCfg = factoryCfg),
    setStakingTotalDeposit: (state, stakingTotalDeposit) =>
        (state.stakingTotalDeposit = stakingTotalDeposit),
    setStakingTotalShares: (state, stakingTotalShares) =>
        (state.stakingTotalShares = stakingTotalShares),
    setStakingTotalWarming: (state, stakingTotalWarming) =>
        (state.stakingTotalWarming = stakingTotalWarming),
    setStakingClaims: (state, stakingClaims) =>
        (state.stakingClaims = stakingClaims),
    addOffer: (state, offer) => state.offers.push(offer),
    setLoadingOffers: (state, showLoadingOffers) =>
        (state.showLoadingOffers = showLoadingOffers),
    setLoadingMyOffers: (state, showLoadingMyOffers) =>
        (state.showLoadingMyOffers = showLoadingMyOffers),
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
