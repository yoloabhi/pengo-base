<template>
  <base-nav
    v-model="showMenu"
    class="navbar-absolute top-navbar"
    type="white"
    :transparent="true"
  >
    <div slot="brand" class="navbar-wrapper">
      <div class="navbar-minimize d-inline"><sidebar-toggle-button /></div>
      <div
        class="navbar-toggle d-inline"
        :class="{ toggled: $sidebar.showSidebar }"
      >
        <button type="button" class="navbar-toggler" @click="toggleSidebar">
          <span class="navbar-toggler-bar bar1"></span>
          <span class="navbar-toggler-bar bar2"></span>
          <span class="navbar-toggler-bar bar3"></span>
        </button>
      </div>
      <a class="navbar-brand" href="#pablo">{{ routeName }}</a>
    </div>

    <ul class="navbar-nav" :class="$rtl.isRTL ? 'mr-auto' : 'ml-auto'">
        <template v-if="!isSignedIn">
          <button @click="login" type="button" class="btn btn-info btn-outline" acon="tim-icons icon-single-02">
            <i class="tim-icons icon-single-02"></i>
            Login
          </button>
        </template>
        <template v-else>
          <base-input v-model="getAccountBalance" :disabled="true" style="color: #fff!important;" />
          <base-dropdown
            tag="li"
            :menu-on-right="!$rtl.isRTL"
            title-tag="a"
            class="nav-item"
            title-classes="nav-link"
            menu-classes="dropdown-navbar"
          >
        <template
          slot="title"
        >
          <div class="photo"><img src="img/mike.jpg" /></div>
          <b class="caret d-none d-lg-block d-xl-block"></b>
          <p class="d-lg-none">Log out</p>
        </template>
        <li class="nav-link">
          <a href="#" @click="logout" class="nav-item dropdown-item">Log out</a>
        </li>
      </base-dropdown>
        </template>
    </ul>
  </base-nav>
</template>
<script>
import { CollapseTransition } from 'vue2-transitions';
import { BaseNav, Modal } from '@/components';
import SidebarToggleButton from './SidebarToggleButton';
import { login,logout } from "@/utils";

export default {
  components: {
    SidebarToggleButton,
    BaseNav,
    Modal
  },
  props:{
    iconUrl: {
      type: String,
    }
  },
  computed: {
    getAccountBalance() {
      if (this.accountBalance === null) {
        return "Balance: Fetching"
      }
      return "Balance: " + this.accountBalance + " Near";
    },
    isSignedIn() {
      this.getBalance()
      return window.walletConnection.isSignedIn();
    },
    routeName() {
      const { name } = this.$route;
      return this.capitalizeFirstLetter(name);
    },
    isRTL() {
      return this.$rtl.isRTL;
    }
  },
  data() {
    return {
      dat: 'rrr',
      activeNotifications: false,
      showMenu: false,
      searchModalVisible: false,
      searchQuery: '',
      accountBalance: null,
    };
  },
  methods: {
    async getBalance() {
      const accountBalance = await window.walletConnection.account().getAccountBalance();
      this.accountBalance = (accountBalance['available'] / 1000000000000000000000000).toFixed(3);
    },
    login() {
      console.log("calling utils.login")
      login()
    },
    logout: logout,
    capitalizeFirstLetter(string) {
      return string.charAt(0).toUpperCase() + string.slice(1);
    },
    toggleNotificationDropDown() {
      this.activeNotifications = !this.activeNotifications;
    },
    closeDropDown() {
      this.activeNotifications = false;
    },
    toggleSidebar() {
      this.$sidebar.displaySidebar(!this.$sidebar.showSidebar);
    },
    hideSidebar() {
      this.$sidebar.displaySidebar(false);
    },
    toggleMenu() {
      this.showMenu = !this.showMenu;
    }
  }
};
</script>
<style scoped>
.top-navbar {
  top: 0px;
}
</style>
