<script lang="ts">
  type Option = {
    value: any;
    display: string;
  };
  let {
    options = $bindable(),
    selected = $bindable(),
  }: { options: Option[]; selected: any } = $props();
</script>

<div class="select-wrapper">
  <select bind:value={selected}>
    {#each options as option}
      <option value={option.value}>{option.display}</option>
    {/each}
  </select>
  <svg class="icon dropdown-icon">
    <use xlink:href="dropdown.svg#dropdown" />
  </svg>
</div>

<style>
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

  svg {
    width: 1em;
    height: 1em;
    fill: currentColor;
    display: inline-block;
    font-size: inherit;
    user-select: none;
    flex-shrink: 0;
    position: absolute;
    right: 5px;
    top: 50%;
    transform: translateY(-50%);
    fill: #f6f6f6;
  }
</style>
