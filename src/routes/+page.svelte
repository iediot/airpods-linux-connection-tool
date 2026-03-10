<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let visible = true;

  async function connect() {
    await invoke("connect_airpods");
  }

  function ignore() {
    visible = false;
  }
</script>

{#if visible}
  <div class="card">
    <h2>AirPods detected</h2>
    <p>Connect your AirPods?</p>

    <div class="buttons">
      <button class="connect" on:click={connect}>Connect</button>
      <button class="ignore" on:click={ignore}>Ignore</button>
    </div>
  </div>
{/if}

<style>

  :global(html, body) {
    margin: 0;
    padding: 0;
    width: 100%;
    height: 100%;
    background: transparent;
    overflow: hidden;
    font-family: system-ui;
  }

  .card {
    width: 100%;
    height: 100%;

    background: white;
    border-radius: 16px;

    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;

    box-shadow: 0 10px 40px rgba(0,0,0,0.25);
    text-align: center;
  }

  h2 {
    margin-bottom: 4px;
  }

  p {
    font-size: 13px;
    color: #666;
  }

  .buttons {
    margin-top: 14px;
    display: flex;
    gap: 10px;
  }

  button {
    border: none;
    padding: 8px 14px;
    border-radius: 10px;
    cursor: pointer;
    font-weight: 500;
  }

  .connect {
    background: #007aff;
    color: white;
  }

  .ignore {
    background: #e5e5e5;
  }

</style>