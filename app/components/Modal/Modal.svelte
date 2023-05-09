<script>
  import { onMount } from "svelte";
  import { videoToPlay } from '../../store';

  export let video = {};

  const handleClick = () => {
    videoToPlay.update(value => {});
  };
  onMount(() => {
    if (typeof window.Module !== 'undefined' && window.Module.runPlayer) {
      window.Module.runPlayer();
    }
  });
</script>

<div class="modal">
  <!-- svelte-ignore a11y-media-has-caption -->
  <video id="video-element" width="800" height="600" preload="auto" poster={video.image} autoplay loop controls>
    <source src={video.src}>
  </video>
  <div class="modal-close" on:click={handleClick}>Close</div>
</div>

<style>
  .modal{
    align-items: center;
    background-color: black;
    display: flex;
    height: 100vh;
    justify-content: center;
    position: fixed;
    width: 100vw;
    z-index: 5;
  }
  .modal-close {
    color: white;
    cursor: pointer;
    position: absolute;
    right: 5%;
    top: 5%;;
  }
</style>
