<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";

  let connecting = false;
  let error = "";

  onMount(() => {
    listen("airpods-found", () => {
      connecting = false;
      error = "";
    });
  });

  async function connect() {
    connecting = true;
    error = "";
    try {
      await invoke("connect_airpods");
    } catch (e) {
      error = String(e);
      connecting = false;
    }
  }

  async function ignore() {
    await invoke("ignore_airpods");
  }
</script>

<div class="card">
  <div class="text">
    <h2>AirPods detected</h2>

    {#if error}
      <p class="error">{error}</p>
    {:else if connecting}
      <p>Connecting...</p>
    {:else}
      <p>Connect your AirPods?</p>
    {/if}
  </div>

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
    /* Ensure Inter is loaded in app.html as discussed earlier */
    font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'SF Pro Display', 'SF Pro Text', system-ui, sans-serif;
  }

  /* Compact card with symmetrical borders */
  .card {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;

    background: white;
    border-radius: 16px;
    border: 1px solid #d1d1d1;

    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;

    /* Reduced padding for a more squished look */
    padding: 6px 14px;
    gap: 6px;

    box-shadow: 0 10px 40px rgba(0,0,0,0.25);
    text-align: center;
    overflow: hidden;
  }

  /* Group the title + prompt tightly */
  .text {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0; /* almost touching */
  }

  h2 {
    font-size: 18px;
    font-weight: 600;
    margin-bottom: 2px; /* tiny gap to the prompt */
  }

  p {
    font-size: 15px;
    color: #666;
    margin-top: 0;
    margin-bottom: 0;
  }

  .error {
    color: #ff3b30;
    font-size: 12px;
    line-height: 1.2;
    overflow: hidden;
    display: -webkit-box;
    -webkit-line-clamp: 3;
    -webkit-box-orient: vertical;
    margin-top: 2px;
  }

  /* Buttons sit close under the text */
  .buttons {
    display: flex;
    gap: 10px;
    margin-top: 6px; /* tight to the text */
  }

  button {
    border: none;
    padding: 8px 14px; /* unchanged size per request */
    border-radius: 10px;
    cursor: pointer;
    font-weight: 500;
    font-family: inherit;
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
    color: #333;
  }
</style>