<template>
  <div class="content">

    <div class="row">
      <div class="col-md-6 ml-auto mr-auto">
        <game-card name="Most Important Pengo" grade="general" image="img\7up7down.png"/>
      </div>
    </div>
  </div>
</template>
<script>
import PengoCard from "@/components/Cards/PengoCard";
import GameCard from "@/components/Cards/GameCard";
export default {
  components: {
    PengoCard,
    GameCard

  }
};
</script>
<style></style>
