<script lang="ts">
  import { PageMap, pageMap, selectedPath } from './page'

  export let path: string

  type Item = {
    name: string
    size: number
    path: string
    isFolder: boolean
    isOpen: boolean
  }

  function getChildPath(path: string, childPath: string) {
    if (path === '/') {
      return '/' + childPath
    } else {
      return path + '/' + childPath
    }
  }

  $: dir = getDir($pageMap, path)
  function getDir(map: PageMap | null, path: string): Item[] {
    if (map === null) {
      return []
    }
    const names = Object.keys(map[path]).sort()
    return names.map((name) => {
      const childPath = getChildPath(path, name)
      const item: Item = {
        name,
        size: map[path][name].size,
        path: childPath,
        isFolder: map[childPath] !== undefined,
        isOpen: !!map[path][name].isOpen,
      }
      return item
    })
  }

  function bodyKeydown(e: KeyboardEvent) {
    if (e.key === 'ArrowRight' || e.key === 'ArrowLeft') {
      for (const item of dir) {
        if (item.path === $selectedPath) {
          if (e.key === 'ArrowRight') {
            $pageMap[path][item.name].isOpen = true
          } else if (e.key === 'ArrowLeft') {
            $pageMap[path][item.name].isOpen = false
          }
        }
      }
      e.preventDefault()
    }
  }

  function openOrClose(name: string) {
    $pageMap[path][name].isOpen = !$pageMap[path][name].isOpen
  }

  export let indentLevel = 0
</script>

<svelte:body on:keydown={bodyKeydown} />

{#each dir as item}
  <div
    class="item"
    class:open={item.isOpen}
    class:selected={$selectedPath === item.path}
    style={`padding-left: ${14 * indentLevel + 2}px`}
    on:click={() => ($selectedPath = item.path)}
    on:dblclick={() => openOrClose(item.name)}
  >
    {#if $pageMap && $pageMap[item.path]}
      <div class="arrow" on:click|stopPropagation={() => openOrClose(item.name)}>
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"
          ><path d="M21 12l-18 12v-24z" /></svg
        >
      </div>
    {/if}
    {item.name}
    <div class="size">
      {#if item.size < 1000}
        {item.size}
      {:else if item.size < 1000000}
        {item.size / 1000} KB
      {:else if item.size < 1000000000}
        {item.size / 1000000} MB
      {:else if item.size < 1000000000000}
        {item.size / 1000000000} GB
      {:else if item.size < 1000000000000000}
        {item.size / 1000000000000} TB
      {/if}
    </div>
  </div>
  <div class="children">
    {#if item.isOpen}
      <svelte:self path={item.path} indentLevel={indentLevel + 1} />
    {/if}
  </div>
{/each}

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
  .arrow
    padding: 5px
    margin-right: 4px
    width: 10px
    height: 10px
    display: flex
    align-items: center
  svg
    fill: hsla(216, 50%, 70%, 0.75)
  .size
    display: inline-block
    margin-left: auto
    margin-right: 10px
</style>
