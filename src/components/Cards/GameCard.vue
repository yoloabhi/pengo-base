<template>
  <div>
    <card
        type="pricing"
        class="card-primary"
        footer-classes="text-center mb-3 mt-3"
    >
<!--      <h1 class="card-title">{{ this.name }}</h1>-->
      <img class="card-img" src="img/card-primary.png" alt="Image" />
      <img :src="image" style="display: block; position: relative!important; width: 95%; margin: auto;margin-top: 3%;" alt="Image" />

<!--        <div class="card-prices" style="padding-top: 40px">-->
<!--          <h3 class="text-on-front"><span>$</span>95</h3>-->
<!--          <h5 class="text-on-back">95</h5>-->
<!--          <p class="plan">Professional plan</p>-->
<!--        </div>-->
      <div class="col-md-6 ml-auto mr-auto">
        <card
          type="pricing"
          class="card-primary card-white"
          footer-classes="text-center mb-3 mt-3"
          id="rule-card"
          style="margin-top: 2%;"
        >
          <h1 class="card-title" style="text-align:left; font-size:3em; margin-left: 1%;" >Rules</h1>
          <!--          <img class="card-img" alt="Image" />-->
          <ul class="list-group" style="text-align:left; /">
            <li class="list-group-item rules-list">Press Play which would deduct the Near</li>
            <li class="list-group-item rules-list">Then Choose 7 , up or Down</li>
            <li class="list-group-item rules-list">Then depending on the result you get your prize</li>
          </ul>
          <!--          <div class="card-prices">-->
          <!--            <h3 class="text-on-front"><span>$</span>95</h3>-->
          <!--            <h5 class="text-on-back">95</h5>-->
          <!--            <p class="plan">Professional plan</p>-->
          <!--          </div>-->
          <!--          <base-button slot="footer" round type="primary" class="btn-just-icon">-->
          <!--            Get started-->
          <!--          </base-button>-->
        </card>

      </div>
        <base-button slot="footer" round type="primary" class="btn-just-icon">
          Play!
        </base-button>


    </card>
  </div>
</template>
<script>

export default {
  name: 'game-card',
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
<style scoped>
.card-pricing.card-white ul li {
  color: white;
  text-align: left;
}
.card-pricing ul {
  max-width: 420px;
}
#rule-card{
  background: rgba( 0, 0, 0, 0.35 );
  /*box-shadow: 0 8px 32px 0 rgba( 31, 38, 135, 0.37 );*/
  /*backdrop-filter: blur( 7.0px );*/
  /*-webkit-backdrop-filter: blur( 7.0px );*/
  /*border-radius: 10px;*/
  /*border: 1px solid rgba( 255, 255, 255, 0.18 );*/

}
@media (min-width: 768px){
  .col-md-6 {
    flex: 0 0 50%;
    max-width: 90%;
  }
}

</style>
