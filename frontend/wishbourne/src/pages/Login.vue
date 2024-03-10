<template>
  <v-app style="background-color: black;" >
    <div class="d-flex align-center justify-center w-100 h-100">
        <div style="max-width: 50vw;min-width: 50vw; min-height: 50vh; background-color: white;">
        <v-tabs
          v-model="tab"
          show-arrows
          background-color="deep-purple accent-4"
          icons-and-text
          dark
          grow
        >
          <v-tab v-for="i in tabs" :key="i.name">
            <v-icon large>{{ i.icon }}</v-icon>
            <div class="caption py-1">{{ i.name }}</div>
          </v-tab>
        </v-tabs>
        <v-window v-model="tab">
          <v-window-item :value="tabs[0].name">
            <v-card class="px-4">
              <v-card-text>
                <v-row>
                  <v-col class="d-flex" cols="12" sm="3" xsm="12" align-end>
                    <v-text-field v-model="publicKey" v-bind:disabled="false">
                      <template v-slot:append>
                        <v-btn
                          v-if="publicKey"
                          :loading="loading"
                          class="mt-2"
                          text="Submit"
                          type="submit"
                          block
                        ></v-btn>
                        <template v-if="!publicKey">
                          <wallet-multi-button> </wallet-multi-button>
                        </template>
                      </template>
                    </v-text-field>
                  </v-col>
                </v-row>
              </v-card-text>
            </v-card>
          </v-window-item>
          <v-window-item :value="tabs[1].name">
            <v-card class="px-4">
              <v-card-text>
                <v-form ref="registerForm" v-model="valid" lazy-validation>
                  <v-row>
                    <v-col cols="12" sm="6" md="6">
                      <v-text-field
                        v-model="name"
                        label="First Name"
                        maxlength="20"
                        required
                      ></v-text-field>
                    </v-col>
                    <v-col cols="12" sm="6" md="6">
                      <v-text-field
                        v-model="lastname"
                        label="Last Name"
                        maxlength="20"
                        required
                      ></v-text-field>
                    </v-col>
                    <v-col cols="12">
                      <v-text-field
                        v-model="email"
                        label="E-mail"
                        required
                      ></v-text-field>
                    </v-col>
                    <v-col class="d-flex" cols="12" sm="3" xsm="12" align-end>
                      <v-text-field v-model="publicKey" v-bind:disabled="false">
                        <template v-slot:append>
                          <v-btn
                            v-if="publicKey"
                            :loading="loading"
                            class="mt-2"
                            text="Submit"
                            type="submit"
                            block
                          ></v-btn>
                          <template v-if="!publicKey">
                            <wallet-multi-button></wallet-multi-button>
                          </template>
                        </template>
                      </v-text-field>
                    </v-col>
                    <v-spacer></v-spacer>
                    <v-col class="d-flex ml-auto" cols="12" sm="3" xsm="12">
                      <v-btn x-large block :disabled="!valid" color="success"
                        >Register</v-btn
                      >
                    </v-col>
                  </v-row>
                </v-form>
              </v-card-text>
            </v-card>
          </v-window-item>
        </v-window>
      </div>
    </div>
    
    <!-- <v-dialog v-model="dialog" persistent max-width="600px" min-width="360px">
      
    </v-dialog> -->
  </v-app>
  <!-- <v-sheet class="mx-auto" max-width="300">
    <v-form validate-on="submit lazy" @submit.prevent="submit">
      <v-text-field v-model="email" label="User name"></v-text-field>
      <v-text-field v-model="email" label="User name"></v-text-field>

      <v-btn
        v-if="publicKey"
        :loading="loading"
        class="mt-2"
        text="Submit"
        type="submit"
        block
      ></v-btn>
      <template v-if="!publicKey">
        <wallet-multi-button></wallet-multi-button>
      </template>
    </v-form>
  </v-sheet> -->
</template>

<script lang="ts">
import { useGlobalState } from "@/GlobalStates";
import { useWallet } from "solana-wallets-vue";
import { defineComponent, ref, watchEffect } from "vue";
import { useRouter } from "vue-router";
import {
  PhantomWalletAdapter,
  SolflareWalletAdapter,
} from "@solana/wallet-adapter-wallets";
import { initWallet } from "solana-wallets-vue";
import { WalletAdapterNetwork } from "@solana/wallet-adapter-base";
import { WalletMultiButton } from "solana-wallets-vue";

const formRef = ref<HTMLFormElement | null>(null);

export default defineComponent({
  components: {
    WalletMultiButton,
  },
  data: () => ({
    loading: false,
    email: "",
    name: "",
    lastname: "",

    publicKey: undefined,
    dialog: true,
    tab: 0,
    tabs: [
      { name: "Login", icon: "mdi-account" },
      { name: "Register", icon: "mdi-account-outline" },
    ],
    valid: true,
  }),
  setup() {
    const walletOptions = {
      wallets: [
        new PhantomWalletAdapter(),
        new SolflareWalletAdapter({ network: WalletAdapterNetwork.Devnet }),
      ],
      autoConnect: true,
    };
    initWallet(walletOptions);

    const { publicKey, sendTransaction, connected,wallet } = useWallet();
    const router = useRouter();
    const authService = useGlobalState();

    var stopWalletListen = watchEffect(() => {
        console.log(connected.value);
        console.log(wallet.value);
      if (connected.value || wallet.value) {
        const { publicKey } = useWallet();

        if (publicKey.value) {
          authService.authService.signUp(publicKey.value.toString());
          stopWalletListen();
        }
      }
    });

    watchEffect(() => {
      if (authService.authService.currentUser) {
        router.push({
          path: "/",
        });
      }
    });
    if (authService.authService.currentUser) {
      router.push({
        path: "/",
      });
    }
  },
  watch: {},
  methods: {
    reset() {
      formRef.value?.reset();
      //.form.reset();
    },
    async submit(event: Event) {
      this.loading = true;

      const results = await event;

      this.loading = false;

      alert(JSON.stringify(results, null, 2));
    },
  },
});
</script>
