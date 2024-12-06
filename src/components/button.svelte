<script lang="ts">
  export let disabled: boolean = false;
  export let onClick: () => void;
  export let type: "button" | "submit" | "reset" = "button";
  export let ariaLabel: string;
  export let close: boolean = false;
</script>

<button {type} {disabled} aria-label={ariaLabel} onclick={onClick} class:close>
  <slot></slot>
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
