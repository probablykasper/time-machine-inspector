<script lang="ts" context="module">
  export type ItemClickEventDetail = {
    name: string
    fullPath: string
    isFolder: boolean
    toggleChildren: () => void
  }
  export type ItemClickEvent = CustomEvent<ItemClickEventDetail>
</script>

<script lang="ts">
  import { createEventDispatcher } from 'svelte'

  type DirItem = {
    [key: string]: number
  }
  type DirMap = {
    [key: string]: DirItem
  }

  export let map: DirMap

  export let name: string
  export let fullPath: string
  export let size: number
  $: isFolder = map[fullPath] !== undefined

  export let selectedPath: string
  export let open = false

  export let indentLevel = 0

  const dispatch = createEventDispatcher<{ click: ItemClickEventDetail }>()

  function itemClick() {
    dispatch('click', {
      name,
      fullPath,
      isFolder,
      toggleChildren: () => {
        open = !open
      },
    })
  }
</script>

<div
  class="item"
  class:open
  on:click={itemClick}
  style={`padding-left: ${14 * indentLevel + 10}px`}
  class:selected={selectedPath === fullPath + '/' + name}>
  {#if isFolder}
    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"
      ><path d="M21 12l-18 12v-24z" /></svg>
  {/if}
  {name}
  <div class="size">
    {#if size < 1000}
      {size}
    {:else if size < 1000000}
      {size / 1000} KB
    {:else if size < 1000000000}
      {size / 1000000} MB
    {:else if size < 1000000000000}
      {size / 1000000000} GB
    {:else if size < 1000000000000000}
      {size / 1000000000000} TB
    {/if}
  </div>
</div>
<div class="children">
  {#if open && isFolder}
    {#each Object.entries(map[fullPath]) as [childName, size]}
      <svelte:self
        {map}
        {selectedPath}
        name={childName}
        {size}
        fullPath={fullPath + '/' + childName}
        on:click
        indentLevel={indentLevel + 1} />
    {/each}
  {/if}
</div>

<style lang="sass">
  .item
    font-size: 14px
    display: flex
    align-items: center
    box-sizing: border-box
    color: hsla(216, 50%, 70%, 0.75)
    cursor: default
    user-select: none
    padding-top: 4px
    padding-bottom: 4px
    box-sizing: border-box
    width: 100%
    &.selected
      background-color: hsla(216, 70%, 70%, 0.2)
    &.open svg
      transform: rotate(90deg)
  svg
    fill: hsla(216, 50%, 70%, 0.75)
    width: 10px
    height: 10px
    margin-right: 4px
  .size
    display: inline-block
    margin-left: auto
    margin-right: 10px
</style>
