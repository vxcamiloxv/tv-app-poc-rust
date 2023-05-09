<script>
  import { videoToPlay } from '../../store';

  export let video;
  let show = false;

  const toggleShow = () => {
    show = !show
  };
  const handleClick = (id) => {
    videoToPlay.update(value => id);
  };
</script>

<div class="cards"
  on:mouseenter={toggleShow}
  on:mouseleave={toggleShow}
  on:click={handleClick.bind(this, { src: video.video_files[0].link, image: video.image })}>
  {#if show}
    <div class="card-author">
      <div><b>Author: </b>{video.user.name}</div>
      <div><b>Duration: </b>{video.duration} sg</div>
    </div>
  {/if}
  <img alt={video.user.name} class="cards-img" src={video.image} />
</div>

<style>
  .card-author {
    background: linear-gradient(to top, black, transparent);
    bottom: 0;
    color: white;
    display: flex;
    flex-direction: column;
    font-size: 0.875rem;
    height: 50px;
    justify-content: center;
    padding-bottom: 10px;
    position: absolute;
    width: 100%;
    z-index: 1;
  }
  .card-author > div {
    padding-left: 10px;
  }
  .cards {
    background-color: white;
    border-radius: 10px;
    cursor: pointer;
    height: 150px;
    margin-left: 20px;
    margin-top: 10px;
    overflow: hidden;
    position: relative;
    transition: all 0.5s;
    width: 200px;
  }
  .cards:hover {
    transform: scale(1.1);
  }
  .cards::before {
    align-items: center;
    border-radius: 100%;
    border: 2px solid;
    box-shadow: 1px 1px 1px rgba(0,0,0,.3);
    color: #fff;
    content: "▶";
    display: flex;
    font-size: 0.875rem;
    height: 22px;
    justify-content: center;
    left: 45%;
    position: absolute;
    text-align: center;
    text-indent: 4px;
    text-shadow: 1px 1px 1px rgba(0,0,0,.3);
    top: 45%;
    width: 22px;
    z-index: 1;
  }
  .cards-img {
    height: 100%;
    position: absolute;
    top: 0;
    width: 100%;
  }

  @media (min-width : 1280px) {
    .cards {
      height: 300px;
      margin-left: 50px;
      margin-top: 30px;
      width: 400px;
    }
    .cards::before {
      border: 4px solid;
      font-size: 1.5rem;
      height: 44px;
      left: calc(50% - 22px);
      top: calc(50% - 22px);
      width: 44px;
    }
    .card-author {
      font-size: 1.5rem;
    }
  }
</style>
