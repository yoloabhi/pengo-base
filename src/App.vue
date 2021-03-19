<template>
  <router-view></router-view>
</template>

<script>
import getConfig from "./configNear";
import { login,logout } from "./utils";

const nearConfig = getConfig(process.env.NODE_ENV || "development");
console.log(
  `networkId:${nearConfig.networkId} CONTRACT_NAME:${nearConfig.contractName}`
);
window.networkId = nearConfig.networkId;

export default {
  created() {
    document.title = "PengoMon";
  },
  methods: {
    initializeLayout() {
      if (!this.$rtl.isRTL) {
        // Just make sure rtl css is off when we are not on rtl
        this.$rtl.disableRTL();
      }
    }
  },
  mounted() {
    this.initializeLayout();
  }
};
</script>
