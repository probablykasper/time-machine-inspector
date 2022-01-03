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

  type DirMap = {
    '/': string[]
    [name: string]: string[]
  }

  export let map: DirMap

  export let name: string
  export let fullPath: string
  $: isFolder = map[fullPath] !== undefined

  export let selectedPath: string
  export let open = true

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
</div>
<div class="children">
  {#if open && isFolder}
    {#each map[fullPath] as child}
      <svelte:self
        {map}
        {selectedPath}
        name={child}
        fullPath={fullPath + '/' + child}
        on:click
        indentLevel={indentLevel + 1} />
    {/each}
  {/if}
</div>

<style lang="sass">
  .item
    font-size: 14px
    color: hsla(216, 50%, 70%, 0.75)
    cursor: default
    user-select: none
    padding: 4px 0px
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
    margin-right: 1px
</style>
