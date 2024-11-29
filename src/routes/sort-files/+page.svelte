<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import { DISPLAYS, type DisplayInfo } from "../../utils";
  import Dropdown from "../../components/dropdown.svelte";
  import FolderInput from "../../components/folderInput.svelte";

  type Folder =
    | { path: string; status: "checking" | "ok"; unwantedFiles?: never }
    | { path: string; status: "error"; unwantedFiles: string[] };

  type FilesMap = {
    files: string[];
    folder: string;
  };

  let sortStatus: "unsorted" | "sorting" | "sorted" = $state("unsorted");

  let folders: Folder[] = $state([]);
  let finalFolder: string = $state("");
  let views: DisplayInfo = $state(DISPLAYS[0]);
  let dropdownOptions = $state(
    DISPLAYS.map((display: DisplayInfo) => ({
      value: display,
      display: display.name,
    })),
  );

  const frameFolderCallback = async (result: string) => {
    folders.push({
      path: result,
      status: "checking",
    });
    checkFolder(result);
  };

  const checkFolder = (path: string) => {
    invoke("check_folder", { path }).then((res: unknown) => {
      const filesMap = res as FilesMap;
      if (filesMap.files.length > 0) {
        const folder = folders.find((f) => f.path === filesMap.folder);
        if (folder) {
          folder.status = "error";
          folder.unwantedFiles = filesMap.files;
        }
      } else {
        const folder = folders.find((f) => f.path === filesMap.folder);
        if (folder) {
          folder.status = "ok";
        }
      }
    });
  };

  const finalFolderCallback = async (result: string) => {
    finalFolder = result;
  };

  const removeFolder = (folder: string) => {
    folders = folders.filter((f) => f.path !== folder);
  };

  const deleteFile = (folder: string, file: string) => {
    invoke("delete_file", { folder, filename: file }).then((res: unknown) => {
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
    sortStatus = "sorting";
    invoke("sort_files", {
      frameFolders: folders.map((f) => f.path),
      finalFolder,
      views: views.layout[0] * views.layout[1],
    }).then((res: unknown) => {
      sortStatus = "sorted";
    });
  };
</script>

<a href="/" class="home" aria-label="Home">
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
      <FolderInput callback={frameFolderCallback}>Select Folder</FolderInput>
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
                <svg class="icon">
                  <use xlink:href="check.svg#check"></use>
                </svg>
                <small>
                  <code>{folder.path}</code>
                </small>
                <button
                  class="close"
                  onclick={() => removeFolder(folder.path)}
                  aria-label="Remove folder"
                >
                  <svg class="icon close">
                    <use xlink:href="trash.svg#trash"></use>
                  </svg>
                </button>
              </li>
            {:else if folder.status === "error"}
              <li class="bg-error">
                <div class="row">
                  <svg class="icon">
                    <use xlink:href="cancel.svg#cancel"></use>
                  </svg>
                  <small>
                    <code>{folder.path}</code>
                  </small>
                  <button
                    class="close"
                    onclick={() => removeFolder(folder.path)}
                    aria-label="Delete all unwanted items in folder"
                  >
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
                          aria-label="Delete file"
                        >
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
      <Dropdown bind:options={dropdownOptions} bind:selected={views} />
    </div>
    <div class="column form-input">
      <h4>Final Frame Folder</h4>
      <small>Where should the final frame sequence be saved</small>
      <FolderInput callback={finalFolderCallback}>Select Folder</FolderInput>
      {#if finalFolder}
        <small><code>{finalFolder}</code></small>
      {/if}
    </div>
    <div class="column form-input">
      <button
        type="submit"
        class={`row ${
          folders.length === 0 ||
          folders.some((folder) => folder.status !== "ok") ||
          !finalFolder ||
          !views ||
          sortStatus !== "unsorted"
            ? "disabled"
            : ""
        }`}
        disabled={folders.length === 0 ||
          folders.some((folder) => folder.status !== "ok") ||
          !finalFolder ||
          !views ||
          sortStatus !== "unsorted"}
        onclick={sortFiles}
      >
        {#if sortStatus === "unsorted"}
          <strong>Sort Files</strong>
        {:else if sortStatus === "sorting"}
          <div class="loader"></div>
          <strong>Sorting Files</strong>
        {:else if sortStatus === "sorted"}
          <strong>Files Sorted</strong>
        {/if}
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
      radial-gradient(farthest-side, gray 95%, #0000) 50% 0/12px 12px no-repeat,
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

  button {
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
