<script>
  import Card from './Card.svelte';

  export let title;
  export let videos;
</script>

<div id={title} class="list-videos-container">
  <div class="list-video-title">{title}</div>
  <div class="list-video-cards">
    {#each videos as video}
      <Card video={video} />
    {/each}
  </div>

</div>

<style>
  .list-video-cards {
    display: flex;
    flex-wrap: wrap;
    max-height: 75vh;
    overflow-y: auto;
    padding-bottom: 80px;
    padding-top: 20px;
  }
  .list-video-title {
    color: white;
    font-size: 2.5rem;
    padding-left: 20px;
  }
  .list-videos-container {
    display: flex;
    flex-direction: column;
  }
  @media (min-width : 1280px) {
    .list-video-title {
    font-size: 3.125rem;
  }
  }
</style>
