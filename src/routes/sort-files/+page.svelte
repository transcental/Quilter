<script lang="ts">
  import { open } from "@tauri-apps/plugin-dialog";

  type Folder = {
    path: string;
    status: "checking" | "ok" | "error";
  };

  let folders: Folder[] = $state([]);
  let finalFolder: string = $state("");
  let views: number = $state(66);

  const openFolderSelect = async () => {
    const result = await open({
      directory: true,
    });
    if (result) {
      folders.push({
        path: result,
        status: "checking",
      });
    }
  };

  const openFinalFolderSelect = async () => {
    const result = await open({
      directory: true,
    });
    if (result) {
      finalFolder = result;
    }
  };

  const removeFolder = (folder: string) => {
    folders = folders.filter((f) => f.path !== folder);
  };

  const submitForm = async (event: Event) => {
    event.preventDefault();
  };

  const validateNumberInput = (event: Event) => {
    const input = (event.target as HTMLInputElement).value;
    if (isNaN(parseInt(input))) {
      (event.target as HTMLInputElement).value = "";
    }
  };
</script>

<main class="container">
  <h1>Sort Files</h1>
  <form onsubmit={submitForm}>
    <div class="column">
      <div class="column form-input">
        <h4>Frame Folders</h4>
        <small
          >These should be selected in the order you want the animation built in</small
        >
        <button onclick={openFolderSelect} id="folders" name="folders"
          >Select Folder</button
        >
        {#if folders.length > 0}
          <ul>
            {#each folders as folder (folder)}
              <li>
                <div class="loader"></div>
                <div class="ok hidden">
                  <img src="/check.svg" class="icon" alt="Folder Selected" />
                </div>
                <div class="error hidden">
                  <img src="/cancel.svg" class="icon" alt="Folder Selected" />
                </div>
                <small>
                  <code>{folder.path}</code>
                </small>
                <button class="close" onclick={() => removeFolder(folder.path)}>
                  <img
                    src="/trash.svg"
                    class="icon close"
                    alt="Remove Folder"
                  />
                </button>
              </li>
            {/each}
          </ul>
        {/if}
      </div>
      <div class="column form-input">
        <h4>Amount of Views</h4>
        <small>How many images are there per frame</small>
        <input
          type="number"
          oninput={validateNumberInput}
          onchange={(e) =>
            (views = parseInt((e.target as HTMLInputElement).value))}
        />
      </div>
      <div class="column form-input">
        <h4>Final Frame Folder</h4>
        <small>Where should the final frame sequence be saved</small>
        <button onclick={openFinalFolderSelect} id="final" name="final"
          >Select Folder</button
        >
        {#if finalFolder}
          <small><code>{finalFolder}</code></small>
        {/if}
      </div>
      <div class="column form-input">
        <button type="submit"><strong>Sort Files</strong></button>
      </div>
    </div>
  </form>
</main>

<style>
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
    padding: 10vw;
  }

  .column {
    display: flex;
    flex-direction: column;
    flex: 50%;
    padding: 10px;
  }

  .loader {
    width: 25px;
    aspect-ratio: 1;
    border-radius: 50%;
    padding: 3px;
    background:
      radial-gradient(farthest-side, gray 95%, #0000) 50% 0/10px 10px no-repeat,
      radial-gradient(
          farthest-side,
          #0000 calc(100% - 5px),
          gray calc(100% - 4px)
        )
        content-box;
    animation: l6 2s infinite;
  }

  @keyframes l6 {
    to {
      transform: rotate(1turn);
    }
  }

  .column.form-input {
    gap: 5px;
    h4 {
      margin: 0;
    }
  }

  .ok,
  .error {
    width: 25px;
    height: 25px;
  }

  .hidden {
    display: none;
  }

  ul {
    list-style-type: none;
    padding: 0;
    margin: 0;
  }

  li {
    display: flex;
    justify-content: space-evenly;
    align-items: center;
    padding: 5px;
    border: 1px solid #646cff;
    border-radius: 5px;
    margin: 5px 0;
  }

  button,
  input {
    padding: 5px;
    margin: 5px;
    border: 1px solid #646cff;
    border-radius: 5px;
    background-color: #646cff;
    color: white;
  }

  button {
    cursor: pointer;
  }

  button.close {
    background-color: #ff3e00;
    border-color: #ff3e00;
  }

  .icon.close {
    display: inline;
    width: 16px;
    height: 16px;
  }

  h1 {
    text-align: center;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }
  }
</style>
