<script lang="ts">
  let {
    disabled = false,
    onClick,
    type,
    ariaLabel,
    close = false,
    children,
  }: {
    disabled?: boolean;
    onClick: () => void;
    type?: "button" | "submit" | "reset";
    ariaLabel: string;
    close?: boolean;
    children: any;
  } = $props();
</script>

<button {type} {disabled} aria-label={ariaLabel} onclick={onClick} class:close>
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

  button[disabled] {
    background-color: #646cff40;
    color: #646cff80;
    cursor: not-allowed;
  }

  button.close {
    background-color: #ff3e00;
    border-color: #ff3e00;
  }
</style>
