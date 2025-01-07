<script lang="ts">
</script>

<main class="container">
  <h1>Break</h1>
  <small>Rest your eyes for a bit</small>
</main>

<style>
:root {
  font-family: system-ui, -apple-system, sans-serif;

  color: #a3a3a3;
  background: #000;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
  animation: fadein 1s;
  -webkit-user-select: none; /* Safari */
  -ms-user-select: none; /* IE 10 and IE 11 */
  user-select: none; /* Standard syntax */
}

@keyframes fadein {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}

h1 {
  margin: 0.4rem 0;
  cursor: default;
  font-size: 5rem;
  font-weight: 900;
  background: linear-gradient(90deg, #242424 0%, #a3a3a3 50%);
  color: transparent;
  -webkit-background-clip: text;
  background-clip: text;
  -webkit-text-fill-color: transparent;
  text-fill-color: transparent;
}
small {
  cursor: default;
  font-size: 1rem;
  font-weight: 600;
}
</style>
