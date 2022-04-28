<template>
    <nav>
        <ul>
            <li class="item">
                <router-link to="/offers">
                    <svg class="icon-24" viewBox="0 0 24 24" fill="none">
                        <path
                            d="M12 2L2 7L12 12L22 7L12 2Z"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                        />
                        <path
                            d="M2 17L12 22L22 17"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                        />
                        <path
                            d="M2 12L12 17L22 12"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                        />
                    </svg>
                    <p>My Offers</p>
                </router-link>
            </li>

            <li class="item">
                <router-link to="/trades">
                    <svg class="icon-24" viewBox="0 0 24 24" fill="none">
                        <path
                            d="M2 3H8C9.06087 3 10.0783 3.42143 10.8284 4.17157C11.5786 4.92172 12 5.93913 12 7V21C12 20.2044 11.6839 19.4413 11.1213 18.8787C10.5587 18.3161 9.79565 18 9 18H2V3Z"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                        />
                        <path
                            d="M22 3H16C14.9391 3 13.9217 3.42143 13.1716 4.17157C12.4214 4.92172 12 5.93913 12 7V21C12 20.2044 12.3161 19.4413 12.8787 18.8787C13.4413 18.3161 14.2044 18 15 18H22V3Z"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                        />
                    </svg>
                    <p>My Trades</p>
                </router-link>
            </li>
            <li class="item">
                <router-link to="/staking">
                    <svg class="icon-24" viewBox="0 0 24 24" fill="none">
                        <path
                            fill="none"
                            d="M12.173,16.086c0.72,0,1.304-0.584,1.304-1.305V6.089c0-0.72-0.584-1.304-1.304-1.304c-0.721,0-1.305,0.584-1.305,1.304v8.692C10.868,15.502,11.452,16.086,12.173,16.086z M11.738,6.089c0-0.24,0.194-0.435,0.435-0.435s0.435,0.194,0.435,0.435v8.692c0,0.24-0.194,0.436-0.435,0.436s-0.435-0.195-0.435-0.436V6.089zM16.52,16.086c0.72,0,1.304-0.584,1.304-1.305v-11.3c0-0.72-0.584-1.304-1.304-1.304c-0.721,0-1.305,0.584-1.305,1.304v11.3C15.215,15.502,15.799,16.086,16.52,16.086z M16.085,3.481c0-0.24,0.194-0.435,0.435-0.435s0.435,0.195,0.435,0.435v11.3c0,0.24-0.194,0.436-0.435,0.436s-0.435-0.195-0.435-0.436V3.481z M3.48,16.086c0.72,0,1.304-0.584,1.304-1.305v-3.477c0-0.72-0.584-1.304-1.304-1.304c-0.72,0-1.304,0.584-1.304,1.304v3.477C2.176,15.502,2.76,16.086,3.48,16.086z M3.045,11.305c0-0.24,0.194-0.435,0.435-0.435c0.24,0,0.435,0.194,0.435,0.435v3.477c0,0.24-0.195,0.436-0.435,0.436c-0.24,0-0.435-0.195-0.435-0.436V11.305z M18.258,16.955H1.741c-0.24,0-0.435,0.194-0.435,0.435s0.194,0.435,0.435,0.435h16.517c0.24,0,0.435-0.194,0.435-0.435S18.498,16.955,18.258,16.955z M7.826,16.086c0.72,0,1.304-0.584,1.304-1.305V8.696c0-0.72-0.584-1.304-1.304-1.304S6.522,7.977,6.522,8.696v6.085C6.522,15.502,7.106,16.086,7.826,16.086z M7.392,8.696c0-0.239,0.194-0.435,0.435-0.435s0.435,0.195,0.435,0.435v6.085c0,0.24-0.194,0.436-0.435,0.436s-0.435-0.195-0.435-0.436V8.696z"
                        ></path>
                    </svg>
                    <p>Staking</p>
                </router-link>
            </li>
        </ul>
    </nav>
</template>

<script>
import { defineComponent } from "vue";

export default defineComponent({
    name: "NavDesktop",
});
</script>

<style lang="scss" scoped>
@import "../style/tokens.scss";

nav {
    display: flex;
    justify-content: space-between;
    margin-left: auto;

    ul {
        display: flex;
        margin-right: 32px;

        li {
            padding: 0 20px;
        }

        a {
            display: inline-block;
            height: 40px;
            font-size: 14px;
            color: $base-text;

            &:hover {
                color: $gray900;
                .icon-24 {
                    stroke: $primary;
                }
            }

            .icon-24 {
                vertical-align: middle;
            }

            p {
                display: inline-block;
                vertical-align: middle;
                line-height: 40px;
                margin-left: 16px;
            }
        }
    }
}
</style>
