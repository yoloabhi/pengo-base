<template>
  <div class="content">
    <div class="row">


    </div>
    <div class="row">
      <div class="col-md-4 ml-auto mr-auto">
        <div class="canvas">
          <h1>First Player to 5 Points Win!</h1>
          <div class="selection">Player Selected: <span id="player-selection"></span></div><div class="selection">Computer Selected: <span id="computer-selection"></span></div>
          <div id="center">
            <div id="winner"></div>
            <div id="image"></div>
          </div>

          <div>
            <div id="player">Player: <span id="player-score">0</span></div>
            <div id="computer">Computer: <span id="computer-score">0</span></div>
          </div>
          <div>
            <h2>Make a Selection</h2>
            <button id="heads" @click="playToss">Heads</button>
            <button id="tails" @click="playToss">Tails</button>
          </div>
        </div>


<!--        <template v-if="playGame" >-->
<!--          <div style="margin-top: 20%;margin-left: 25%">-->
<!--            <base-button slot="footer" round type="primary" class="btn-just-icon" @click="play" style="margin-right:10%;">-->
<!--              UP-->
<!--            </base-button>-->
<!--            <base-button slot="footer" round type="primary" class="btn-just-icon" @click="play" style="margin-right:10%;">-->
<!--              7-->
<!--            </base-button>-->
<!--            <base-button slot="footer" round type="primary" class="btn-just-icon" @click="play" >-->
<!--              DOWN-->
<!--            </base-button>-->
<!--          </div>-->

<!--        </template>-->
<!--        <base-button slot="footer" type="success" class="animation-on-hover" @click="playToss" style="left: 40%;top:20%;" >-->
<!--          <img src="public/img/purple.png" style="width:40%;padding-top:5%; " />-->
<!--          Play!-->
<!--        </base-button>-->



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
            <li class="list-group-item rules-list">- 300 messages fala dim khana</li>
            <li class="list-group-item rules-list">- 150 emails meko pyaar hai</li>
            <li class="list-group-item rules-list">- 24/7 Support kardo mere boobs ko</li>
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

// import PengoCard from "@/components/Cards/PengoCard";
export default {
  components: {
    // PengoCard,
  },
  data() {
    return {
      playGame:false,
      buttons : document.querySelectorAll('button'),
      heads : 1,
      tails : 0,
      userScore : 0,
      computerScore : 0


    }
  },

  methods: {
    displaySelections(user, computer){
  const playerSelection = document.querySelector('#player-selection');
  const computerSelection = document.querySelector('#computer-selection');
  if (user === 'heads'){
    playerSelection.style.color = 'green';
  }
  if (user === 'tails'){
    playerSelection.style.color = 'blue';
  }
  if (computer === 'heads'){
    computerSelection.style.color = 'green';
  }
  if (computer === 'tails'){
    computerSelection.style.color = 'blue';
  }
  playerSelection.innerHTML = `${user}`;
  computerSelection.innerHTML = `${computer}`
},

    displayRandom(random){
  const displayResult = document.querySelector('#image');
  console.log(random);

  if (random === 1){
    displayResult.style.backgroundImage =  "url('./heads.png')";

  } else {
    displayResult.style.backgroundImage =  "url('./tails.png')";
  }
},

    tallyScore(random, userPick, computerPick){
  //select scoreboard from DOM
  const playerDisplay = document.querySelector('#player-score');
  const computerDisplay = document.querySelector('#computer-score');
  const winner = document.querySelector('#winner');

  if (userPick === random){
    this.userScore++;
  }
  if (computerPick === random){
    this.computerScore++;
  }
  playerDisplay.textContent = `${this.userScore}`;
  computerDisplay.textContent = `${this.computerScore}`;

  if (this.userScore === 5 && this.computerScore === 5){
    winner.innerHTML = `<h1>It's a Tie</h1>`;
  } else if (this.userScore === 5){
    winner.innerHTML = `<h1>You Win!!!</h1>`;
  } else if (this.computerScore === 5){
    winner.innerHTML = `<h1>Computer Wins!!!</h1>`;
  }
},
    playToss(){
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

      //Record users selection
      const userSelection = e.target.id;
      let userPick;

      if (userSelection === 'heads'){
        userPick = 1;
      } else if (userSelection === 'tails'){
        userPick = 0;
      }

      //displays the player and computer's selection
      //in the Selected portion of DOM
      this.displaySelections(userSelection, computerSelection);
      this. displayRandom(random);



      //Adds the score of the player and computer
      setTimeout(function(){
        this.tallyScore(random, userPick, computerPick);
        //resets animations
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
