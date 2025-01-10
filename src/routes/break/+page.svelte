<script lang="ts">
  import { onMount } from 'svelte';
  import { fade } from 'svelte/transition';
  import { invoke } from '@tauri-apps/api/core';
  import NumberFlow from '@number-flow/svelte';

  const MAX_ESC_HOLD_MS = 1000;
  let seconds = 20;
  let interval: number;
  let mounted = false;
  let escHoldStart: number | null = null;

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape' && !escHoldStart) {
      escHoldStart = Date.now();
    }
  }

  function handleKeyup(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      escHoldStart = null;
    }
  }

  onMount(() => {
    mounted = true;

    // Add keyboard event listeners
    document.addEventListener('keydown', handleKeydown);
    document.addEventListener('keyup', handleKeyup);

    // Start the countdown interval
    interval = setInterval(() => {
      if (seconds > 0) {
        seconds--;

        // Check if ESC has been held
        if (escHoldStart && Date.now() - escHoldStart >= MAX_ESC_HOLD_MS) {
          clearInterval(interval);
          mounted = false;
          setTimeout(() => {
            invoke('timer_done');
          }, 1000);
        }
      } else {
        clearInterval(interval);
        mounted = false;
        setTimeout(() => {
          invoke('timer_done');
        }, 1000);
      }
    }, 1000);

    // Cleanup function
    return () => {
      clearInterval(interval);
      document.removeEventListener('keydown', handleKeydown);
      document.removeEventListener('keyup', handleKeyup);
    };
  });
</script>

{#if mounted}
  <main
    class="container"
    in:fade={{ duration: 500 }}
    out:fade={{ duration: 250 }}
  >
    <h1>Break</h1>
    <small id="message">Rest your eyes for a bit</small>
    {#if escHoldStart === null}
      <small id="howToQuit">Hold ESC to skip</small>
    {:else}
      <small id="howToQuit"> Skipping... </small>
    {/if}
    <div id="counter">
      <NumberFlow
        value={seconds}
        format={{ minimumIntegerDigits: 2 }}
        trend={0}
      />
    </div>
  </main>
{/if}

<style>
  :root {
    font-family:
      system-ui,
      -apple-system,
      sans-serif;
    font-synthesis: none;
    text-rendering: optimizeLegibility;
  }
  .container {
    position: relative;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    height: 100%;
    margin: 0;
    color: #a3a3a3;
    background: #000;
  }
  h1 {
    margin: 0.3rem 0;
    cursor: default;
    font-size: 6rem;
    font-weight: 900;
    background: linear-gradient(90deg, #a3a3a3 0%, #242424 90%);
    color: transparent;
    -webkit-background-clip: text;
    background-clip: text;
    -webkit-text-fill-color: transparent;
    text-fill-color: transparent;
    z-index: 10;
  }
  #message {
    cursor: default;
    font-size: 1.25rem;
    font-weight: 600;
    z-index: 10;
  }
  #counter {
    margin-top: 1.5rem;
    font-size: 3rem;
    font-family: monospace;
    font-weight: 800;
    z-index: 10;
  }
  #howToQuit {
    margin-top: 0.6rem;
    color: #4e4e4e;
    font-size: 0.9rem;
    z-index: 10;
  }
</style>
