<script lang="ts">
import { defineComponent } from 'vue'
import { LinkProps } from './interfaces';
import { WalletMultiButton, useWallet } from 'solana-wallets-vue';

export default defineComponent({
    setup() {
    },
    props:{
        links:Array<LinkProps>
    },
    methods:{
        disconect:async function () {
            const {disconnect,wallet,connected}= useWallet();

            await wallet.value?.adapter.disconnect();
            await disconnect();

        }
    }
})
</script>

<template>
    <v-app id="inspire">
    <v-app-bar
      class="px-3"
      density="compact"
      flat
    >
      <v-avatar
        class="hidden-md-and-up"
        color="grey-darken-1"
        size="32"
      ></v-avatar>

      <v-spacer></v-spacer>

      <v-tabs
        color="grey-darken-2"
        centered
      >
        <v-tab
          v-for="link in links"
          :key="link.link"
          :text="link.text"
        ></v-tab>
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

    <v-main class="bg-grey-lighten-3">
      <v-container>
      <router-view></router-view>
      </v-container>
    </v-main>
  </v-app>
</template>
