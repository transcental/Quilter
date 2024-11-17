<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { ask, message } from "@tauri-apps/plugin-dialog";
  import { check } from "@tauri-apps/plugin-updater";

  async function checkForAppUpdates(userAsked: boolean = false) {
    const update = await check();
    console.log(update);
    if (update === null && userAsked) {
      await message("You are on the latest version. Yay!", {
        title: "No Update Available",
        kind: "info",
        okLabel: "OK",
      });
      return;
    } else if (update?.available) {
      const yes = await ask(
        `Update to ${update.version} is available!\n\nRelease notes: ${update.body}`,
        {
          title: "Update Available",
          kind: "info",
          okLabel: "Update",
          cancelLabel: "Cancel",
        },
      );
      if (yes) {
        await update.downloadAndInstall();
        // Restart the app after the update is installed by calling the Tauri command that handles restart for your app
        // It is good practice to shut down any background processes gracefully before restarting
        // As an alternative, you could ask the user to restart the app manually
        await invoke("graceful_restart");
      }
    }
  }

  // run on boot
  checkForAppUpdates();
</script>

<button
  on:click={() => checkForAppUpdates(true)}
  class="updater"
  aria-label="Check for updates"
>
  <svg class="update">
    <use xlink:href="update.svg#update"></use>
  </svg>
</button>

<main class="container">
  <h1>Quilter</h1>
  <h3>Please select an option below.</h3>

  <div class="row">
    <a href="/sort-files">
      <!-- <img src="/sort.svg" class="logo sort-files" alt="Sort Files" /> -->
      <svg class="logo sort-files">
        <use xlink:href="sort.svg#sort"></use>
      </svg>
      <h4>Sort Files</h4>
    </a>
    <a href="/create-quilt">
      <!-- <img
        src="/sewing-machine.svg"
        class="logo create-quilt"
        alt="Create Quilt"
      /> -->
      <svg class="logo create-quilt">
        <use xlink:href="sewing-machine.svg#sewing-machine"></use>
      </svg>
      <h4>Create Quilt</h4>
    </a>
    <a href="/cast">
      <!-- <img src="/to-pip.svg" class="logo cast" alt="Cast to Glass" /> -->
      <svg class="logo cast">
        <use xlink:href="to-pip.svg#to-pip"></use>
      </svg>
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

  .update {
    width: 2em;
    height: 2em;
  }

  button.updater {
    position: fixed;
    bottom: 1em;
    right: 1em;
    background: none;
    border: none;
    cursor: pointer;
    z-index: 1000;
  }

  .updating {
    animation: spin 3s linear infinite;
  }

  @keyframes spin {
    0% {
      transform: rotate(0deg);
    }
    100% {
      transform: rotate(360deg);
    }
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

  svg {
    fill: #0f0f0f;
    width: 6em;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }

    a:hover {
      color: #24c8db;
    }

    svg {
      fill: #f6f6f6;
    }
  }

  footer {
    margin-top: 5em;
    text-align: center;
    font-size: 0.75em;
  }
</style>
