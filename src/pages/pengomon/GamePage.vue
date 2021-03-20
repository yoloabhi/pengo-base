<template>
  <div class="content">
    <div class="row">


    </div>
    <div class="row">
      <div class="col-md-4 ml-auto mr-auto">
        <div class="dice">
          <ol class="die-list even-roll" data-roll="1" id="die-1">
            <li class="die-item" data-side="1">
              <span class="dot"></span>
            </li>
            <li class="die-item" data-side="2">
              <span class="dot"></span>
              <span class="dot"></span>
            </li>
            <li class="die-item" data-side="3">
              <span class="dot"></span>
              <span class="dot"></span>
              <span class="dot"></span>
            </li>
            <li class="die-item" data-side="4">
              <span class="dot"></span>
              <span class="dot"></span>
              <span class="dot"></span>
              <span class="dot"></span>
            </li>
            <li class="die-item" data-side="5">
              <span class="dot"></span>
              <span class="dot"></span>
              <span class="dot"></span>
              <span class="dot"></span>
              <span class="dot"></span>
            </li>
            <li class="die-item" data-side="6">
              <span class="dot"></span>
              <span class="dot"></span>
              <span class="dot"></span>
              <span class="dot"></span>
              <span class="dot"></span>
              <span class="dot"></span>
            </li>
          </ol>
          <ol class="die-list odd-roll" data-roll="1" id="die-2">
            <li class="die-item" data-side="1">
              <span class="dot"></span>
            </li>
            <li class="die-item" data-side="2">
              <span class="dot"></span>
              <span class="dot"></span>
            </li>
            <li class="die-item" data-side="3">
              <span class="dot"></span>
              <span class="dot"></span>
              <span class="dot"></span>
            </li>
            <li class="die-item" data-side="4">
              <span class="dot"></span>
              <span class="dot"></span>
              <span class="dot"></span>
              <span class="dot"></span>
            </li>
            <li class="die-item" data-side="5">
              <span class="dot"></span>
              <span class="dot"></span>
              <span class="dot"></span>
              <span class="dot"></span>
              <span class="dot"></span>
            </li>
            <li class="die-item" data-side="6">
              <span class="dot"></span>
              <span class="dot"></span>
              <span class="dot"></span>
              <span class="dot"></span>
              <span class="dot"></span>
              <span class="dot"></span>
            </li>
          </ol>
        </div>

        <template v-if="playGame" >
          <div style="margin-top: 20%;margin-left: 25%">
            <base-button slot="footer" round type="primary" class="btn-just-icon" @click="play" style="margin-right:10%;">
              UP
            </base-button>
            <base-button slot="footer" round type="primary" class="btn-just-icon" @click="play" style="margin-right:10%;">
              7
            </base-button>
            <base-button slot="footer" round type="primary" class="btn-just-icon" @click="play" >
              DOWN
            </base-button>
          </div>

        </template>
        <base-button slot="footer" type="success" class="animation-on-hover" @click="playGame=!playGame" style="left: 15%;top:20%;"  v-else>
          <img src="img/icons/near_icon2.png" style="width:15%;" />
          Play!
        </base-button>



      </div>
      <div class="col-md-4 ml-auto mr-auto">
        <card
          type="pricing"
          class="card-primary card-white"
          footer-classes="text-center mb-3 mt-3"
          id="rule-card"
        >
          <h1 class="card-title" style="text-align:center; font-size:4em;" >Rules</h1>
<!--          <img class="card-img" alt="Image" />-->
          <ul class="list-group">
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
    </div>

  </div>
</template>
<script>
var diceSound= new Audio('../../assets/sound/dicesound.mp3');
// import PengoCard from "@/components/Cards/PengoCard";
export default {
  components: {
    // PengoCard,
  },
  data() {
    return {
      playGame:false,
      playSound:false,

    }
  },

  methods: {

     rollDice() {
      const dice = [...document.querySelectorAll(".die-list")];
      dice.forEach(die => {
        this.toggleClasses(die);
        die.dataset.roll = this.getRandomNumber(1, 6);
      });
    },
    play(){
       this.rollDice();
       this.playGame=!this.playGame;
       diceSound.play();
    },
    toggleClasses(die) {
      die.classList.toggle("odd-roll");
      die.classList.toggle("even-roll");
    },

    getRandomNumber(min, max) {
      min = Math.ceil(min);
      max = Math.floor(max);
      return Math.floor(Math.random() * (max - min + 1)) + min;
    }

},

};
</script>
<style scoped>
* {
  margin: 0;
  padding: 0;
  vertical-align: baseline;
}
html {
  font-family: system-ui, sans-serif;
}
body {
  background: linear-gradient(#545454, #454545, #676767);
  display: grid;
  grid-template-columns: 1fr;
  height: 100vh;
  overflow: hidden;
  width: 100%;
}
.card-pricing.card-white ul li {
  color: white;
  text-align: justify;
}
.card-pricing ul {
  max-width: 320px;

}
#rule-card{
  background: rgba( 0, 0, 0, 0.35 );
  box-shadow: 0 8px 32px 0 rgba( 31, 38, 135, 0.37 );
  backdrop-filter: blur( 7.0px );
  -webkit-backdrop-filter: blur( 7.0px );
  border-radius: 10px;
  border: 1px solid rgba( 255, 255, 255, 0.18 );

}
.dice {
  /*align-items: center;*/
  display: grid;
  grid-gap: 2rem;
  grid-template-columns: repeat(auto-fit, minmax(8rem, 1fr));
  grid-template-rows: auto;
  justify-items: center;
  padding: 2rem;
  perspective: 600px;
}
.die-list {
  display: grid;
  grid-template-columns: 1fr;
  grid-template-rows: 1fr;
  height: 6rem;
  list-style-type: none;
  transform-style: preserve-3d;
  width: 6rem;
}
.even-roll {
  transition: transform 1.5s ease-out;
}
.odd-roll {
  transition: transform 1.25s ease-out;
}
.die-item {
  background-color: #fefefe;
  box-shadow: inset -0.35rem 0.35rem 0.75rem rgba(0, 0, 0, 0.3),
  inset 0.5rem -0.25rem 0.5rem rgba(0, 0, 0, 0.15);
  display: grid;
  grid-column: 1;
  grid-row: 1;
  grid-template-areas:
    "one two three"
    "four five six"
    "seven eight nine";
  grid-template-columns: repeat(3, 1fr);
  grid-template-rows: repeat(3, 1fr);
  height: 100%;
  padding: 1rem;
  width: 100%;

}
.dot {
  align-self: center;
  background-color: #676767;
  border-radius: 50%;
  box-shadow: inset -0.15rem 0.15rem 0.25rem rgba(0, 0, 0, 0.5);
  display: block;
  height: 1.25rem;
  justify-self: center;
  width: 1.25rem;
}
.even-roll[data-roll="1"] {
  transform: rotateX(360deg) rotateY(720deg) rotateZ(360deg);
}
.even-roll[data-roll="2"] {
  transform: rotateX(450deg) rotateY(720deg) rotateZ(360deg);
}
.even-roll[data-roll="3"] {
  transform: rotateX(360deg) rotateY(630deg) rotateZ(360deg);
}
.even-roll[data-roll="4"] {
  transform: rotateX(360deg) rotateY(810deg) rotateZ(360deg);
}
.even-roll[data-roll="5"] {
  transform: rotateX(270deg) rotateY(720deg) rotateZ(360deg);
}
.even-roll[data-roll="6"] {
  transform: rotateX(360deg) rotateY(900deg) rotateZ(360deg);
}
.odd-roll[data-roll="1"] {
  transform: rotateX(-360deg) rotateY(-720deg) rotateZ(-360deg);
}
.odd-roll[data-roll="2"] {
  transform: rotateX(-270deg) rotateY(-720deg) rotateZ(-360deg);
}
.odd-roll[data-roll="3"] {
  transform: rotateX(-360deg) rotateY(-810deg) rotateZ(-360deg);
}
.odd-roll[data-roll="4"] {
  transform: rotateX(-360deg) rotateY(-630deg) rotateZ(-360deg);
}
.odd-roll[data-roll="5"] {
  transform: rotateX(-450deg) rotateY(-720deg) rotateZ(-360deg);
}
.odd-roll[data-roll="6"] {
  transform: rotateX(-360deg) rotateY(-900deg) rotateZ(-360deg);
}
[data-side="1"] {
  transform: rotate3d(0, 0, 0, 90deg) translateZ(2.9rem);
}
[data-side="2"] {
  transform: rotate3d(-1, 0, 0, 90deg) translateZ(2.9rem);
}
[data-side="3"] {
  transform: rotate3d(0, 1, 0, 90deg) translateZ(2.9rem);
}
[data-side="4"] {
  transform: rotate3d(0, -1, 0, 90deg) translateZ(2.9rem);
}
[data-side="5"] {
  transform: rotate3d(1, 0, 0, 90deg) translateZ(2.9rem);
}
[data-side="6"] {
  transform: rotate3d(1, 0, 0, 180deg) translateZ(2.9rem);
}
[data-side="1"] .dot:nth-of-type(1) {
  grid-area: five;
}
[data-side="2"] .dot:nth-of-type(1) {
  grid-area: one;
}
[data-side="2"] .dot:nth-of-type(2) {
  grid-area: nine;
}
[data-side="3"] .dot:nth-of-type(1) {
  grid-area: one;
}
[data-side="3"] .dot:nth-of-type(2) {
  grid-area: five;
}
[data-side="3"] .dot:nth-of-type(3) {
  grid-area: nine;
}
[data-side="4"] .dot:nth-of-type(1) {
  grid-area: one;
}
[data-side="4"] .dot:nth-of-type(2) {
  grid-area: three;
}
[data-side="4"] .dot:nth-of-type(3) {
  grid-area: seven;
}
[data-side="4"] .dot:nth-of-type(4) {
  grid-area: nine;
}
[data-side="5"] .dot:nth-of-type(1) {
  grid-area: one;
}
[data-side="5"] .dot:nth-of-type(2) {
  grid-area: three;
}
[data-side="5"] .dot:nth-of-type(3) {
  grid-area: five;
}
[data-side="5"] .dot:nth-of-type(4) {
  grid-area: seven;
}
[data-side="5"] .dot:nth-of-type(5) {
  grid-area: nine;
}
[data-side="6"] .dot:nth-of-type(1) {
  grid-area: one;
}
[data-side="6"] .dot:nth-of-type(2) {
  grid-area: three;
}
[data-side="6"] .dot:nth-of-type(3) {
  grid-area: four;
}
[data-side="6"] .dot:nth-of-type(4) {
  grid-area: six;
}
[data-side="6"] .dot:nth-of-type(5) {
  grid-area: seven;
}
[data-side="6"] .dot:nth-of-type(6) {
  grid-area: nine;
}

button {
  align-self: center;
  background-color: #efefef;
  border: none;
  color: #333;
  font-size: 1.25rem;
  font-weight: 700;
  justify-self: center;
  padding: 0.5rem 1rem;
}
button:hover {
  cursor: pointer;
}

@media (min-width: 900px) {
  .dice {
    perspective: 1300px;
  }
}

</style>
