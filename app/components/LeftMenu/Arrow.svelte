<script>
  export let onClick;
  export let toggle;
  $: arrow = toggle ? '›' : '‹';
</script>

<div class="arrow-menu" on:click="{onClick}">{arrow}</div>

<style>
  .arrow-menu {
    align-items: flex-end;
    background-color: white;
    border-radius: 50%;
    color: #0EBFE9;
    cursor: pointer;
    display: flex;
    font-size: 50px;
    height: 50px;
    justify-content: center;
    margin: 15px;
    position: absolute;
    right: 0;
    top: 0;
    width: 50px;
  }
</style>