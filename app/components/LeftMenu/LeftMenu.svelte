<script>
  import Arrow from './Arrow.svelte';

  export let data;
  export let onChange;
  let toggle = false;

  const handleClick = (event) => {
    event.preventDefault();
    toggle = !toggle;
  };
  const handleClickMenu = (category) => {
    onChange(category);
  };
</script>

<div class="left-menu" class:left-menu-active={toggle}>
  <Arrow toggle={toggle} onClick={handleClick} />
  {#each data as item}
    <div
      class:left-menu-items-active={toggle}
      on:click={handleClickMenu.bind(this, item.title)}
      class="left-menu-items">
        {item.title}
    </div>
  {/each}
</div>

<style>
  .left-menu {
    background-color: #0EBFE9;
    color: white;
    display: flex;
    flex-direction: column;
    font-size: 2rem;
    height:100vh;
    justify-content: center;
    opacity: 1;
    position: relative;
    transition: width 1s;
    width: 20vw;
    overflow: hidden;
  }
  .left-menu-active {
    width: 80px;
  }
  .left-menu-items {
    cursor: pointer;
    margin-top: 15px;
    transition: all 0.5s;
    text-align: center;
    width: 100%;
  }
  .left-menu-items-active {
    opacity: 0;
  }
  @media (min-width : 1280px) {
    .left-menu  {
      font-size: 3.125rem;
    }
  }
</style>
