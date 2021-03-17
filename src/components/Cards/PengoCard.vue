<template>
  <div>
    <card
      type="pricing"
      class="card-primary card-white"
      footer-classes="text-center mb-3 mt-3"
    >
      <h1 class="card-title">pro</h1>
      <img class="card-img" src="img/card-primary.png" alt="Image" />
      <ul class="list-group">
        <li class="list-group-item">300 messages</li>
        <li class="list-group-item">150 emails</li>
        <li class="list-group-item">24/7 Support</li>
      </ul>
      <div class="card-prices">
        <h3 class="text-on-front"><span>$</span>95</h3>
        <h5 class="text-on-back">95</h5>
        <p class="plan">Professional plan</p>
      </div>
      <base-button slot="footer" round type="primary" class="btn-just-icon">
        Get started
      </base-button>
    </card>
    <card
      type="pricing"
      class="card-primary card-white"
      footer-classes="text-center mb-3 mt-3">
<!--      :style="{ border: '2px solid ' + this.getCardOutlineColor }"-->
<!--    >-->
<!--      <img class="card-img" :src="image" alt="Image" />-->
      <h1 class="card-title">{{ this.name }}</h1>
      <ul class="list-group">
        <li class="list-group-item">300 messages</li>
        <li class="list-group-item">150 emails</li>
        <li class="list-group-item">24/7 Support</li>
      </ul>
      <div class="card-prices">
        <h3 class="text-on-front"><span>$</span>95</h3>
        <h5 class="text-on-back">95</h5>
        <p class="plan">Professional plan</p>
      </div>
      <base-button slot="footer" round type="primary" class="btn-just-icon">
        Get started
      </base-button>
    </card>
  </div>
</template>
<script>
import Card from "@/components/Cards/Card";
export default {
  name: 'card',
  props: {
    name: {
      type: String,
      description: 'Card title'
    },
    image: {
      type: String,
      description: 'Card Image'
    },
    cost: {
      type: String,
      description: 'Listed price',
      default: '0',
    },
    grade: {
      type: String,
      description: 'Card type'
    },
  },
  // computed: {
  //   getCardOutlineColor () {
  //     if (this.grade === 'general') {
  //       return 'gold';
  //     } else if (this.grade === 'lieutenant') {
  //       return 'red';
  //     } else
  //     if (this.grade === 'captain') {
  //       return 'violet';
  //     }
  //     if (this.grade === 'major') {
  //       return 'blue';
  //     }
  //     else {
  //       return 'black';
  //     }
  //   }
  // },
  mounted() {
    console.log('YOII')
  }
};
</script>
<style></style>
