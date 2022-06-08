import { createRouter, createWebHistory } from "vue-router";
import Home from "../pages/Home.vue";
import Explore from "../pages/Explore.vue";
import Offers from "../pages/Offers.vue";
import Trades from "../pages/Trades.vue";
import TradeDetail from "../pages/TradeDetail.vue";
import Staking from "../pages/Staking.vue";

const routes = [
    {
        path: "/",
        name: "Home",
        component: Home,
    },
    {
        path: "/explore",
        name: "Explore",
        component: Explore,
    },
    {
        path: "/offers",
        name: "Offers",
        component: Offers,
    },
    {
        path: "/trades",
        name: "Trades",
        component: Trades,
    },
    {
        path: "/trade/:addr",
        name: "TradeDetail",
        component: TradeDetail,
    },
    {
        path: "/staking",
        name: "Staking",
        component: Staking,
    },
];

const router = createRouter({
    history: createWebHistory(process.env.BASE_URL),
    routes,
});

export default router;
