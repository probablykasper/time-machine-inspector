<script lang="ts" context="module">
  export type ItemClickEventDetail = {
    name: string
    dir: string
    fullPath: string
    prevPath: string | null
    isFolder: boolean
    toggleChildren: () => void
  }
  export type ItemClickEvent = CustomEvent<ItemClickEventDetail>
</script>

<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  import { backupInfos } from './page'
  import type { DirMap, BackupInfo } from './page'

  export let map: DirMap

  export let name: string
  export let dir: string
  export let fullPath: string
  export let prevPath: string | null
  $: isFolder = map[fullPath] !== undefined

  let loadState = 0
  function updateLoadState(backupInfos: BackupInfo[]) {
    for (const info of backupInfos) {
      if (info.old === prevPath && info.new === fullPath) {
        if (info.loading) {
          loadState = 1
        } else {
          loadState = 2
        }
      }
    }
  }
  $: updateLoadState($backupInfos)

  let children: string[] = []
  $: if (isFolder) {
    children = Object.keys(map[fullPath]).sort()
  } else {
    children = []
  }

  export let selectedPath: string
  export let open = true

  export let indentLevel = 0

  const dispatch = createEventDispatcher<{ click: ItemClickEventDetail }>()

  function itemClick() {
    dispatch('click', {
      name,
      fullPath,
      prevPath,
      dir,
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
  class:loading={loadState === 1}
  class:loaded={loadState === 2}
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
    {#each children as child, i}
      <svelte:self
        {map}
        name={child}
        dir={fullPath}
        fullPath={fullPath + '/' + child}
        prevPath={i === 0 ? null : fullPath + '/' + children[i - 1]}
        {selectedPath}
        on:click
        indentLevel={indentLevel + 1} />
    {/each}
  {/if}
</div>

<style lang="sass">
  $ease-md: cubic-bezier(0.4, 0.0, 0.2, 1)
  .item
    font-size: 14px
    color: hsla(216, 50%, 70%, 0.75)
    cursor: default
    user-select: none
    padding: 4px 0px
    box-sizing: border-box
    width: 100%

  .loading:not(.selected)
    animation: flash 1s $ease-md infinite alternate

  @keyframes flash
    0%
      color: hsla(216, 50%, 70%, 0.5)
    100%
      color: hsla(216, 50%, 70%, 0.9)

  .selected
    background-color: hsla(216, 70%, 70%, 0.2)
  .open svg
    transform: rotate(90deg)
  .loaded
    font-weight: 600
    color: hsla(216, 50%, 80%, 0.9)
  svg
    fill: hsla(216, 50%, 70%, 0.75)
    width: 10px
    height: 10px
    margin-right: 1px
</style>
