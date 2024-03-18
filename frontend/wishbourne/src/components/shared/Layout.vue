<script lang="ts">
import { defineComponent } from "vue";
import { LinkProps } from "./interfaces";
import { WalletMultiButton, useWallet } from "solana-wallets-vue";
import { useRouter } from "vue-router";
import {
  PhantomWalletAdapter,
  SolflareWalletAdapter,
} from "@solana/wallet-adapter-wallets";
import { initWallet } from "solana-wallets-vue";
import { WalletAdapterNetwork } from "@solana/wallet-adapter-base";
let router = undefined;
export default defineComponent({
  components: {
    WalletMultiButton,
  },
  setup() {
    router = useRouter();
    const walletOptions = {
      wallets: [
        new PhantomWalletAdapter(),
        new SolflareWalletAdapter({ network: WalletAdapterNetwork.Devnet }),
      ],
      autoConnect: true,
    };
    initWallet(walletOptions);
  },
  data: () => {
    return {};
  },
  props: {
    links: Array<LinkProps>,
  },
  computed: {
    publicKey: () => {
      const { publicKey, sendTransaction, connected, wallet } = useWallet();
      return publicKey;
    },
  },
  methods: {
    disconect: async function () {
      try{
        const { disconnect, wallet, connected } = useWallet();

      await wallet.value?.adapter.disconnect();
      // await disconnect();
      router.push({
        name: "/",
      });
      }
      catch{
        router.push({
        name: "/",
      });
      }
    },
  },
});
</script>

<template>
  <v-app id="inspire">
    <v-app-bar class="px-3" density="compact" flat>
      <v-avatar
        class="hidden-md-and-up"
        color="grey-darken-1"
        size="32"
      ></v-avatar>

      <v-spacer></v-spacer>

      <v-tabs color="grey-darken-2" centered>
        <v-tab v-for="link in links" :key="link.link" :text="link.text"></v-tab>
      </v-tabs>
      <v-spacer></v-spacer>

      <v-btn @click="disconect()">
        <v-avatar
          class="hidden-sm-and-down"
          color="grey-darken-1"
          size="32"
        ></v-avatar>
      </v-btn>
    </v-app-bar>
    <v-footer color="grey" height="44" app>
      <template v-if="publicKey">
        <div style="position: fixed">
          <wallet-multi-button></wallet-multi-button>
        </div>
      </template>
    </v-footer>
    <v-main class="bg-grey-lighten-3">
      <v-container>
        <router-view></router-view>
      </v-container>
    </v-main>
  </v-app>
</template>
