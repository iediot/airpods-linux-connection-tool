<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let connecting = false;
  let error = "";

  async function connect() {
    connecting = true;
    error = "";
    try {
      await invoke("connect_airpods");
      // App will close itself from Rust side
    } catch (e) {
      error = String(e);
      connecting = false;
    }
  }

  async function ignore() {
    await invoke("ignore_airpods");
    // App will close itself from Rust side
  }
</script>

<div class="card">
  <h2>AirPods detected</h2>

  {#if error}
    <p class="error">{error}</p>
  {:else if connecting}
    <p>Connecting...</p>
  {:else}
    <p>Connect your AirPods?</p>
  {/if}

  <div class="buttons">
    <button class="connect" on:click={connect} disabled={connecting}>
      {connecting ? "..." : "Connect"}
    </button>
    <button class="ignore" on:click={ignore} disabled={connecting}>Ignore</button>
  </div>
</div>

<style>

  :global(*, *::before, *::after) {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
  }

  :global(html, body) {
    width: 100%;
    height: 100%;
    background: transparent;
    overflow: hidden;
    font-family: system-ui;
  }

  .card {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;

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

  .error {
    color: #ff3b30;
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

  button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .connect {
    background: #007aff;
    color: white;
  }

  .ignore {
    background: #e5e5e5;
  }

</style>