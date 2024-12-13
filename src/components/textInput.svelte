<script lang="ts">
  let {
    disabled = false,
    type,
    value = $bindable(),
  }: {
    disabled?: boolean;
    type: string;
    value: string | number;
  } = $props();
</script>

<input {type} {disabled} bind:value />

<style>
  input {
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

  input[disabled] {
    background-color: #646cff40;
    color: #646cff80;
    cursor: not-allowed;
  }
</style>
