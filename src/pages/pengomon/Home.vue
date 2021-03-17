<template>
  <div class="content">
    <div class="row">
      <div class="col-lg-3 col-md-6">
        <pengo-card name="Ishan Randi" grade="general" image="img\UltraPengo.jpg"/>
      </div>
      <div class="col-lg-3 col-md-6">
        <pengo-card name="Ishan Randi" grade="general" image="img\UltraPengo.jpg" type="market"/>
      </div>
    </div>
  </div>
</template>
<script>
import PengoCard from "@/components/Cards/PengoCard";
export default {
  components: {
    PengoCard,
  }
};
</script>
<style></style>
