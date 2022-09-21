import Home from '~/ui/pages/Home.vue'
import Offers from '~/ui/pages/Offers.vue'
import Trades from '~/ui/pages/Trades.vue'
import TradeDetail from '~/ui/pages/TradeDetail.vue'
import Arbitration from '~/ui/pages/Arbitration.vue'

const routes = [
  {
    path: '/',
    name: 'Home',
    component: Home,
  },
  {
    path: '/offers',
    name: 'Offers',
    component: Offers,
  },
  {
    path: '/trades',
    name: 'Trades',
    component: Trades,
  },
  {
    path: '/trade/:id',
    name: 'TradeDetail',
    component: TradeDetail,
  },
  {
    path: '/staking',
    name: 'Staking',
    component: [],
  },
  {
    path: '/arbitration',
    name: 'Arbitration',
    component: Arbitration,
  },
]

export default routes
