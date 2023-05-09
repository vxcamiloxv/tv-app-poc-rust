<script>
  import ListVideo from './ListVideo.svelte';
  export let videos;
  export let title;
</script>

<div class="videos-container">
  {#if videos}
    <ListVideo title={title} videos={videos} />
  {/if}
</div>

<style>
  .videos-container {
    background-color: #353535;
    display: flex;
    flex: 1;
    padding: 10px;
  }
  @media (min-width : 1280px) {
    .videos-container {
      background-color: #353535;
      display: flex;
      flex: 1;
      padding: 50px;
    }
  }
</style>
