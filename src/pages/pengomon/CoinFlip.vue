<template>
  <div class="content">
    <div class="row"></div>
    <div class="row">
      <div class="col-md-4 ml-auto mr-auto">
        <div class="canvas">
          <h1>Coin Flip!</h1>
          <div id="center">
            <div id="winner"></div>
            <div id="image" :style="{ backgroundImage: 'url(' + backImage + ')' }"></div>
          </div>
          <template v-if="playGame" >
            <div>
              <h2>Make a Selection</h2>
              <button id="heads" @click="playToss(0)">Heads</button>
              <button id="tails" @click="playToss(1)">Tails</button>
            </div>
          </template>
          <base-button slot="footer" type="success" class="animation-on-hover" @click="playDeposit" style="left: 0%;top:20%;"  v-else>
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
            <li class="list-group-item rules-list">- 300 messages fala dim khana</li>
            <li class="list-group-item rules-list">- 150 emails meko pyaar hai</li>
            <li class="list-group-item rules-list">- 24/7 Support kardo mere boobs ko</li>
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
      userScore : 0,
      computerScore : 0,
      backImage: 'url(\'./heads.png\')',
    }
  },

  methods: {
    playDeposit() {
      this.playGame = true
      // asd
    },
    displayRandom(random){
      const displayResult = document.querySelector('#image');
      console.log(random);

      if (random === 1){
        this.backImage = "url('./heads.png')";
        // displayResult.style.backgroundImage =  "url('./heads.png')";

      } else {
        this.backImage = "url('./heads.png')";
        // displayResult.style.backgroundImage =  "url('./tails.png')";
      }
    },
    playToss(choice){
      //Computer randomly select heads or tails
      const random = Math.round(Math.random());
      //Computer selects a random 'heads' or 'tails
      const computerPick = Math.round(Math.random());
      //Record computers selection
      let computerSelection;
      if (computerPick === 1){
        computerSelection = 'heads';
      } else {
        computerSelection = 'tails';
      }

      //spin the coin
      const spin = document.querySelector('#image');
      spin.classList.add('animate');

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
      console.log(computerSelection)
      this.displaySelections(userSelection, computerSelection);
      this.displayRandom(random);

      //Adds the score of the player and computer
      setTimeout(function(){
        document.querySelector('#image').classList.remove('animate');
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
  0% {background: url("./heads.png")}

  25% {background: url("./tails.png")}

  50% {background: url("./heads.png")}

  75% {background: url("./tails.png")}

  100% {background: url("./heads.png")}
}


#image{
  background: url("./heads.png");
  width: 227px;
  height: 337px;

}

.animate{
  background: url("./heads.png");
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
