<template>
  <Login v-if="!authManager.authService.currentUser"></Login>
  <layout v-if="authManager.authService.currentUser" :links="routes"></layout>
</template>

<script lang="ts" setup>

import { useGlobalState } from "./GlobalStates";
import Layout from "./components/shared/Layout.vue";
import { LinkProps } from "./components/shared/interfaces";
import Login from "./pages/Login.vue";


const routes: Array<LinkProps> = [
  {
    link: "/",
    text: "Ana Sayfa",
  },
];


var authManager = useGlobalState();
</script>
