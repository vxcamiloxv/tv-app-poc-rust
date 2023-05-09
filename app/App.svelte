<script>
  import { onMount } from "svelte";
  import { menuStore, videoToPlay } from './store';
  import LeftMenu from './components/LeftMenu/LeftMenu.svelte';
  import Modal from './components/Modal/Modal.svelte';
  import VideoContainer from './components/VideoContainer/VideoContainer.svelte';

  const categories =  [
    {
      "id": 1,
      "title": "Cats"
    },
    {
      "id": 2,
      "title": "Dogs"
    },
    {
      "id": 3,
      "title": "Hamster"
    },
    {
      "id": 4,
      "title": "Birds"
    },
    {
      "id": 5,
      "title": "Horse"
    },
    {
      "id": 6,
      "title": "Fish"
    }
  ];

  let wasmRust;
  let chosenCategory = categories[0].title;
  let videoToReproduce;
  let categoriesContent = [];

  const updateList = async (category) => {
    const items = await wasmRust.fetch_videos(category);
    categoriesContent = items;
  };
  onMount(async () => {
    wasmRust = await import("./wasm_modules/wasm_rust/pkg");
    updateList(chosenCategory);
  });

  videoToPlay.subscribe(value => videoToReproduce = value);
</script>

<main>
  <LeftMenu data={categories} onChange={updateList} />
  <VideoContainer title={chosenCategory} videos={categoriesContent} />
  {#if videoToReproduce}
    <Modal video={videoToReproduce} />
  {/if}
</main>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    overflow: hidden;
  }
  main {
    position: relative;
    display: flex;
  }
</style>
