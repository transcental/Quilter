<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import { DISPLAYS, type DisplayInfo } from "../../utils";

  type Folder =
    | { path: string; status: "checking" | "ok"; unwantedFiles?: never }
    | { path: string; status: "error"; unwantedFiles: string[] };

  type FilesMap = {
    files: string[];
    folder: string;
  };

  let folders: Folder[] = $state([]);
  let finalFolder: string = $state("");
  let views: DisplayInfo = $state(DISPLAYS[0]);

  const openFolderSelect = async () => {
    const result = await open({
      directory: true,
    });
    if (result) {
      folders.push({
        path: result,
        status: "checking",
      });
      checkFolder(result);
    }
  };

  const checkFolder = (path: string) => {
    invoke("check_folder", { path }).then((res: unknown) => {
      const filesMap = res as FilesMap;
      console.log("Folder Checked");
      if (filesMap.files.length > 0) {
        const folder = folders.find((f) => f.path === filesMap.folder);
        if (folder) {
          folder.status = "error";
          folder.unwantedFiles = filesMap.files;
          console.log("Folder has unwanted files", filesMap.files);
        }
      } else {
        const folder = folders.find((f) => f.path === filesMap.folder);
        if (folder) {
          folder.status = "ok";
          console.log("Folder is good");
        }
      }
    });
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

  const deleteFile = (folder: string, file: string) => {
    invoke("delete_file", { folder, filename: file }).then((res: unknown) => {
      console.log("File Deleted", res);
      checkFolder(folder);
    });
  };

  const deleteOrMoveAllUnwantedFiles = (
    folder: Folder,
    shouldDelete: boolean,
  ) => {
    invoke("delete_move_files_in_folder", {
      folder: folder.path,
      files: folder.unwantedFiles,
      delete: shouldDelete,
    }).then((res: unknown) => {
      checkFolder(folder.path);
    });
  };

  const sortFiles = () => {
    invoke("sort_files", {
      frameFolders: folders.map((f) => f.path),
      finalFolder,
      views: views.layout[0] * views.layout[1],
    }).then((res: unknown) => {
      console.log("Files Sorted", res);
    });
  };
</script>

<a href="/" class="home">
  <!-- <img src="/home.svg" alt="Home" />  -->
  <svg>
    <use xlink:href="home.svg#home"></use>
  </svg>
</a>
<main class="container">
  <h1>Sort Files</h1>
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
            {#if folder.status === "checking"}
              <li class="row">
                <div class="loader"></div>
                <small>
                  <code>{folder.path}</code>
                </small>
              </li>
            {:else if folder.status === "ok"}
              <li class="row bg-success">
                <!-- <img src="/check.svg" class="icon" alt="Folder Selected" /> -->
                <svg class="icon">
                  <use xlink:href="check.svg#check"></use>
                </svg>
                <small>
                  <code>{folder.path}</code>
                </small>
                <button class="close" onclick={() => removeFolder(folder.path)}>
                  <!-- <img
                    src="/trash.svg"
                    class="icon close"
                    alt="Remove Folder"
                  /> -->
                  <svg class="icon close">
                    <use xlink:href="trash.svg#trash"></use>
                  </svg>
                </button>
              </li>
            {:else if folder.status === "error"}
              <li class="bg-error">
                <div class="row">
                  <!-- <img src="/cancel.svg" class="icon" alt="Folder Selected" /> -->
                  <svg class="icon">
                    <use xlink:href="cancel.svg#cancel"></use>
                  </svg>
                  <small>
                    <code>{folder.path}</code>
                  </small>
                  <button
                    class="close"
                    onclick={() => removeFolder(folder.path)}
                  >
                    <!-- <img
                      src="/trash.svg"
                      class="icon close"
                      alt="Remove Folder"
                    /> -->
                    <svg class="icon close">
                      <use xlink:href="trash.svg#trash"></use>
                    </svg>
                  </button>
                </div>
                <div class="column">
                  <div class="row">
                    <small>These files need to be deleted first</small>
                    <button
                      onclick={() =>
                        deleteOrMoveAllUnwantedFiles(folder, false)}
                    >
                      Move All
                    </button>
                    <button
                      onclick={() => deleteOrMoveAllUnwantedFiles(folder, true)}
                    >
                      Delete All
                    </button>
                  </div>
                  <ul>
                    {#each folder.unwantedFiles as file (file)}
                      <li class="elevated row">
                        <small><code>{file}</code></small>
                        <button
                          class="close"
                          onclick={() => deleteFile(folder.path, file)}
                        >
                          <!-- <img
                            src="/trash.svg"
                            class="icon"
                            alt="Remove File"
                          /> -->
                          <svg class="icon">
                            <use xlink:href="trash.svg#trash"></use>
                          </svg>
                        </button>
                      </li>
                    {/each}
                  </ul>
                </div>
              </li>
            {/if}
          {/each}
        </ul>
      {/if}
    </div>
    <div class="column form-input">
      <h4>What device?</h4>
      <small>Which device is this Quilt being built for?</small>
      <div class="select-wrapper">
        <select bind:value={views}>
          {#each DISPLAYS as display}
            <option value={display}>{display.name}</option>
          {/each}
        </select>
        <svg class="icon dropdown-icon">
          <use xlink:href="dropdown.svg#dropdown"></use>
        </svg>
      </div>
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
      <button
        type="submit"
        class={folders.length === 0 ||
        folders.some((folder) => folder.status !== "ok") ||
        !finalFolder ||
        !views
          ? "disabled"
          : ""}
        disabled={folders.length === 0 ||
          folders.some((folder) => folder.status !== "ok") ||
          !finalFolder ||
          !views}
        onclick={sortFiles}
      >
        <strong>Sort Files</strong>
      </button>
    </div>
  </div>
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

  .home svg {
    width: 25px;
    aspect-ratio: 1;
    height: 25px;
    position: fixed;
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
      radial-gradient(farthest-side, gray 95%, #0000) 50% 0/25px 25px no-repeat,
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

  ul {
    list-style-type: none;
    padding: 0;
    margin: 0;
  }

  li {
    border: 1px solid #646cff;
    border-radius: 5px;
    margin: 5px 0;
    overflow: scroll;
  }

  li svg {
    width: 25px;
    height: 25px;
  }

  .row {
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 10px;
    padding: 5px;
  }

  .bg-error {
    background-color: #ff400040;
    /* icon should be at the right */
  }

  .bg-error .elevated {
    background-color: #ff400040;
    color: white;
    padding: 0px;
    margin: 5px;
    border-radius: 5px;
    justify-content: space-between;
  }

  small {
    font-size: 0.8em;
    margin: 5px;
  }

  .bg-success {
    background-color: #00ff0040;
  }

  button,
  select {
    -webkit-appearance: none;
    -moz-appearance: none;
    appearance: none;
    padding: 5px;
    margin: 5px;
    border: 1px solid #646cff;
    border-radius: 5px;
    background-color: #646cff;
    color: white;
  }

  .select-wrapper {
    position: relative;
    margin: 0;
    padding: 0;
  }

  select {
    width: 100%;
  }

  .select-wrapper svg.dropdown-icon {
    position: absolute;
    right: 5px;
    top: 50%;
    transform: translateY(-50%);
    fill: #f6f6f6;
  }

  button {
    cursor: pointer;
  }

  button.disabled {
    background-color: #646cff40;
    color: #646cff80;
    cursor: not-allowed;
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

  svg {
    fill: #0f0f0f;
    width: 16px;
    height: 16px;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }

    svg {
      fill: #f6f6f6;
    }
  }
</style>
