import Home from '~/ui/pages/Home.vue'
import Offers from '~/ui/pages/Offers.vue'
import Trades from '~/ui/pages/Trades.vue'
import TradeDetail from '~/ui/pages/TradeDetail.vue'
import Arbitration from '~/ui/pages/Arbitration.vue'
import Maker from '~/ui/pages/Maker.vue'

const routes = [
  {
    path: '/:type?/:token?/:fiat?',
    name: 'Home',
    component: Home,
  },
  {
    path: '/maker/:addr',
    name: 'Maker',
    component: Maker,
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
