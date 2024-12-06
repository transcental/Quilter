<script lang="ts">
  import { open } from "@tauri-apps/plugin-dialog";

  const openFolder = async () => {
    const result = await open({
      directory: true,
      multiple: multiple,
    });
    if (result) {
      callback(result);
    }
  };

  let {
    multiple = false,
    callback,
    children,
  }: {
    multiple?: boolean;
    callback: (path: string) => void;
    children: any;
  } = $props();
</script>

<button onclick={openFolder}>
  {@render children?.()}
</button>

<style>
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
</style>
