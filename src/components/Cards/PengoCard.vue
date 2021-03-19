<template>
  <div>
    <card
        type="pricing"
        class="card-primary"
        footer-classes="text-center mb-3 mt-3"
    >
      <h1 class="card-title">{{ this.name }}</h1>
      <img class="card-img" src="img/card-primary.png" alt="Image" />
      <img :src="image" style="display: block; position: relative!important; width: 80%; margin: auto;" alt="Image" />
      <template v-if="type === 'market'">
        <div class="card-prices" style="padding-top: 40px">
          <h3 class="text-on-front"><span>$</span>95</h3>
          <h5 class="text-on-back">95</h5>
          <p class="plan">Professional plan</p>
        </div>
        <base-button slot="footer" round type="primary" class="btn-just-icon">
          Get started
        </base-button>
      </template>
      <template v-else>


        <template v-if="sell">
          <div class="col-md-4 ml-auto mr-auto">
            <base-input
              required
              v-model="sellPrice"
              icon-url="img/icons/near_icon.png"
            >
            </base-input>
          </div>

          <base-button slot="footer" round type="primary" class="btn-just-icon" @click="sell=!sell">
            Confirm
          </base-button>
        </template>
        <base-button slot="footer" round type="primary" class="btn-just-icon" @click="sell=!sell" v-else>
          List
        </base-button>

      </template>
    </card>
  </div>
</template>
<script>

export default {
  name: 'pengo-card',
  // props: ['name', 'image', 'cost', 'grade'],
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
    },
    grade: {
      type: String,
      description: 'Card type'
    },
    type: {
      type: String,
      description: 'Card type',
      default: '',
    },

  },
  data() {
    return {
      sell: false,
      sellPrice:null,
    }
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
<style>

</style>
