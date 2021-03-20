<template>
  <div class="content">
    <div class="row"></div>
    <div class="row">
      <div class="col-md-4 ml-auto mr-auto">
        <div class="canvas">
          <h1>Coin Flip!</h1>
          <div id="center">
            <div id="winner"></div>
            <img :src="backImage"/>
<!--            <div id="image" :style="{ backgroundImage: 'url(' + backImage + ')!important;' }"></div>-->
          </div>
          <template v-if="display" >
            <div>User Choice: {{ this.userChoice.toUpperCase() }} <br/> Computer Choice: {{ this.computerChoice.toUpperCase() }}</div>
            <div v-if="userChoice === computerChoice">You Win!</div>
            <div v-else>You Lose!</div>
          </template>
          <template v-if="playGame" >
            <div>
              <h2>Make a Selection</h2>
              <button id="heads" @click="playToss(1)">Heads</button>
              <button id="tails" @click="playToss(0)">Tails</button>
            </div>
          </template>
          <base-button slot="footer" type="success" class="animation-on-hover" @click="playDeposit" style="left: 0%;top:12%;"  v-else>
            <img src="img/icons/near_icon2.png" style="width:15%;" />
            Play!
          </base-button>
        </div>



      </div>
      <div class="col-md-4 ml-auto mr-auto">
        <card
          type="pricing"
          class="card-primary card-white"
          footer-classes="text-center mb-3 mt-3"
          id="rule-card"
        >
          <h1 class="card-title" style="text-align:center; font-size:4em;" >Rules</h1>
          <ul class="list-group">
            <li class="list-group-item rules-list">Press Play which would deduct the Near</li>
            <li class="list-group-item rules-list">Then Choose Head and Tails</li>
            <li class="list-group-item rules-list">Then depending on the result you get your prize</li>
          </ul>
        </card>

      </div>
    </div>

  </div>
</template>
<script>

// import PengoCard from "@/components/Cards/PengoCard";
export default {
  components: {
    // PengoCard,
  },
  data() {
    return {
      playGame: false,
      buttons : document.querySelectorAll('button'),
      heads : 1,
      tails : 0,
      userChoice : 0,
      computerChoice : 0,
      display: false,
      backImage: 'img\\tails.png',
    }
  },

  methods: {
    playDeposit() {
      this.display = false;
      this.playGame = true;
      // call contract logic
    },
    displayRandom(random){
      if (random === 1){
        this.backImage = "img/heads.png";

      } else {
        this.backImage = "img/tails.png";
      }
      this.playGame = false;
    },
    playToss(choice){
      //spin the coin
      // const spin = document.querySelector('#image');
      // spin.classList.add('animate');
      let vm = this;
      let animate = setInterval(() => {
        if (vm.backImage === "img/tails.png") {
          vm.backImage = "img/heads.png";
        } else {
          vm.backImage = "img/tails.png";
        }
        console.log(vm.backImage)
      }, 150)
      //Computer selects a random 'heads' or 'tails
      const computerPick = Math.round(Math.random()) === 1? 1 : 0;
      //Record computers selection
      let computerSelection;
      if (computerPick === 1){
        computerSelection = 'heads';
      } else {
        computerSelection = 'tails';
      }

      // //Record users selection
      let userSelection = null;
      let userPick = choice;

      if (userPick === 1){
        userSelection = 'heads';
      } else if (userPick === 0){
        userSelection = 'tails';
      }

      // displays the player and computer's selection
      // in the Selected portion of DOM
      console.log(computerPick)
      // this.displaySelections(userSelection, computerSelection);
      // this.displayRandom(computerPick);

      //Adds the score of the player and computer
      setTimeout(function(){
        clearInterval(animate);
        vm.displayRandom(computerPick);
        vm.userChoice = userSelection;
        vm.computerChoice = computerSelection;
        vm.display = true;
      }, 2000);
    },
},

};
</script>
<style scoped>

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
@keyframes spin {
  0% {background: url("../../../public/img/heads.png")}

  25% {background: url("../../../public/img/tails.png")}

  50% {background: url("../../../public/img/heads.png")}

  75% {background: url("../../../public/img/tails.png")}

  100% {background: url("../../../public/img/heads.png")}
}


#image{
  /*background-image: url("../../../public/img/heads.png");*/
  width: 227px;
  height: 337px;

}

.animate{
  /*background: url("../../../public/img/heads.png");*/
  width: 227px;
  height: 337px;
  animation-name: spin;
  animation-iteration-count: 2;
  animation-duration: 1s;
}

.canvas{
  width: 800px;
  height: 500px;
  margin: 50px auto;
  text-align: center;
  font-family: Arial, Helvetica, sans-serif;
  font-size: 1.5em;
}

#center{
  width: 227px;
  margin: 15px auto;
}

.selection, #player, #computer{
  display: inline;
  margin: 25px;
  font-size: 1.3em;
}

button {
  font-size: 1.2em;
  border-radius: 5px;
}


</style>
