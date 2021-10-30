import { createRouter, createWebHistory } from "vue-router";
import Home from "../pages/Home.vue";
import Explore from "../pages/Explore.vue";
import Offers from "../pages/Offers.vue";
import Trades from "../pages/Trades.vue";
import Offer from "../components/Offer.vue";
import Trade from "../components/Trade.vue";

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
    path: "/offer/:id",
    name: "Offer",
    component: Offer,
  },
  {
    path: "/trade/:addr",
    name: "Trade",
    component: Trade,
  },
];

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes,
});

export default router;
