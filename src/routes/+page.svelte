<main class="container">
  <h1>Quilter</h1>
  <h3>Please select an option below.</h3>

  <div class="row">
    <a href="/sort-files">
      <img src="/sort.svg" class="logo sort-files" alt="Sort Files" />
      <h4>Sort Files</h4>
    </a>
    <a href="/create-quilt">
      <img
        src="/sewing-machine.svg"
        class="logo create-quilt"
        alt="Create Quilt"
      />
      <h4>Create Quilt</h4>
    </a>
    <a href="/cast">
      <img src="/to-pip.svg" class="logo cast" alt="Cast to Glass" />
      <h4>Cast to Glass</h4>
    </a>
  </div>
</main>

<footer>
  <div class="row">
    Created by&nbsp;<a href="https://transcental.dev" target="_blank">Amber</a
    >&nbsp;© 2024
  </div>
</footer>

<style>
  .logo.sort-files:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.cast:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }

  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: #f6f6f6;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  .container {
    margin: 0;
    padding-top: 10vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
  }

  .logo {
    height: 6em;
    padding: 1.5em;
    padding-bottom: 0;
    will-change: filter;
    transition: 0.75s;
  }

  .logo.create-quilt:hover {
    filter: drop-shadow(0 0 2em #24c8db);
  }

  .row {
    display: flex;
    justify-content: center;
  }

  a {
    font-weight: 500;
    color: #646cff;
    text-decoration: inherit;
  }

  a:hover {
    color: #535bf2;
  }

  h1 {
    text-align: center;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }

    a:hover {
      color: #24c8db;
    }
  }

  footer {
    margin-top: 5em;
    text-align: center;
    font-size: 0.75em;
  }
</style>
