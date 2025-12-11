<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { DISPLAYS, type DisplayInfo } from "../../utils";
  import Dropdown from "../../components/dropdown.svelte";
  import FolderInput from "../../components/folderInput.svelte";
  import Button from "../../components/button.svelte";
  import TextInput from "../../components/textInput.svelte";

  type Folder =
    | {
        path: string;
        status: "empty" | "checking" | "ok";
        unwantedFiles?: never;
      }
    | { path: string; status: "error"; unwantedFiles: string[] };

  type FilesMap = {
    files: string[];
    folder: string;
  };

  type QuiltStatusState =
    | "NotStarted"
    | "InProgress"
    | "Finished"
    | "CreatingAnimation"
    | "CreatedAnimation";

  type QuiltStatus = {
    status: QuiltStatusState;
    amount: number;
    index: number;
  };

  let quiltStatus: QuiltStatus = $state({
    status: "NotStarted",
    amount: 0,
    index: 0,
  });

  let sortedFolder: Folder = $state({
    path: "",
    status: "empty",
  });

  let quiltFolder: string = $state("");
  let framerate = $state(24);

  let views: DisplayInfo = $state(DISPLAYS[0]);
  let dropdownOptions = $state(
    DISPLAYS.map((display: DisplayInfo) => ({
      value: display,
      display: display.name,
    })),
  );

  const checkFolder = (path: string) => {
    invoke("check_folder", { path }).then((res: unknown) => {
      const filesMap = res as FilesMap;
      if (filesMap.files.length > 0) {
        sortedFolder = {
          status: "error",
          path: filesMap.folder,
          unwantedFiles: filesMap.files,
        };
      } else {
        sortedFolder = {
          ...sortedFolder,
          status: "ok",
          unwantedFiles: undefined,
        };
      }
    });
  };

  const sortedFolderCallback = async (result: string) => {
    sortedFolder = { path: result, status: "checking" };
    checkFolder(result);
  };

  const quiltFolderCallback = async (result: string) => {
    quiltFolder = result;
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

  const makeQuilt = () => {
    invoke("make_quilt", {
      sortedFolder: sortedFolder.path,
      outputFolder: quiltFolder,
      columns: views.layout[0],
      rows: views.layout[1],
      framerate,
    });
  };

  listen<QuiltStatus>("quilt_status", (event) => {
    quiltStatus = event.payload;
  });
</script>

<a href="/" class="home" aria-label="Home">
  <svg>
    <use xlink:href="home.svg#home"></use>
  </svg>
</a>
<main class="container">
  <h1>Create Quilt</h1>
  <div class="column">
    <div class="column form-input">
      <h4>Sorted Folder</h4>
      <small>
        This should be the sorted folder that has all your frames in that you
        want combining into quilts.
      </small>
      <FolderInput callback={sortedFolderCallback}>Select Folder</FolderInput>
      {#if sortedFolder.status === "checking"}
        <li class="row">
          <div class="loader"></div>
          <small>
            <code>{sortedFolder.path}</code>
          </small>
        </li>
      {:else if sortedFolder.status === "ok"}
        <li class="row bg-success">
          <svg class="icon">
            <use xlink:href="check.svg#check"></use>
          </svg>
          <small>
            <code>{sortedFolder.path}</code>
          </small>
        </li>
      {:else if sortedFolder.status === "error"}
        <li class="bg-error">
          <div class="row">
            <svg class="icon">
              <use xlink:href="cancel.svg#cancel"></use>
            </svg>
            <small>
              <code>{sortedFolder.path}</code>
            </small>
          </div>
          <div class="column">
            <small>These files need to be deleted first</small>
            <div class="row">
              <Button
                ariaLabel="Move all unwanted files"
                onClick={() =>
                  deleteOrMoveAllUnwantedFiles(sortedFolder, false)}
              >
                Move All
              </Button>
              <Button
                ariaLabel="Delete all unwanted files"
                onClick={() => deleteOrMoveAllUnwantedFiles(sortedFolder, true)}
              >
                Delete All
              </Button>
            </div>
            <ul>
              {#each sortedFolder.unwantedFiles as file (file)}
                <li class="elevated row">
                  <small><code>{file}</code></small>
                  <Button
                    close
                    onClick={() => deleteFile(sortedFolder.path, file)}
                    ariaLabel="Delete file"
                  >
                    <svg class="icon">
                      <use xlink:href="trash.svg#trash"></use>
                    </svg>
                  </Button>
                </li>
              {/each}
            </ul>
          </div>
        </li>
      {/if}
    </div>
    <div class="column form-input">
      <h4>What device?</h4>
      <Dropdown bind:options={dropdownOptions} bind:selected={views} />
    </div>
    <div class="column form-input">
      <h4>What framerate?</h4>
      <small>Set to -1 if not rendering an animation</small>
      <TextInput type="number" bind:value={framerate} />
    </div>
    <div class="column form-input">
      <h4>Quilt Folder</h4>
      <small>Where should the final quilt(s) be saved</small>
      <FolderInput callback={quiltFolderCallback}>Select Folder</FolderInput>
      {#if quiltFolder}
        <small><code>{quiltFolder}</code></small>
      {/if}
    </div>
    <div class="column form-input">
      <Button
        type="submit"
        disabled={sortedFolder.status !== "ok" ||
          !quiltFolder ||
          !views ||
          !framerate ||
          quiltStatus.status !== "NotStarted"}
        onClick={makeQuilt}
        ariaLabel="Create Quilts"
      >
        {#if quiltStatus.status === "NotStarted"}
          <strong>Create Quilts</strong>
        {:else if quiltStatus.status === "InProgress"}
          <div class="loader"></div>
          <strong
            >Creating Quilts ({quiltStatus.index}/{quiltStatus.amount})</strong
          >
        {:else if quiltStatus.status === "Finished"}
          <strong>Quilts Created</strong>
        {:else if quiltStatus.status === "CreatingAnimation"}
          <div class="loader"></div>
          <strong>Rendering Animation</strong>
        {:else if quiltStatus.status === "CreatedAnimation"}
          <strong>Animation Created</strong>
        {/if}
      </Button>
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
