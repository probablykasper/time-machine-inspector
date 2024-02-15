<script lang="ts">
  import { createEventDispatcher, SvelteComponent } from 'svelte'

  import { pageMap, type PageMap, selectedPath } from './page'

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

  const dispatch = createEventDispatcher()

  let itemChildren: SvelteComponent[] = []

  export function selectFirst() {
    $selectedPath = dir[0].path
  }
  export function selectLast() {
    if (dir[dir.length - 1].isOpen) {
      itemChildren[dir.length - 1].selectLast()
    } else {
      $selectedPath = dir[dir.length - 1].path
    }
  }
  function selectUp(i: number) {
    if (i === 0) {
      dispatch('selectUp')
    } else if (dir[i - 1] && dir[i - 1].isOpen) {
      itemChildren[i - 1].selectLast()
    } else if (dir[i - 1]) {
      $selectedPath = dir[i - 1].path
    }
  }
  function selectDown(i: number) {
    if (dir[i].isOpen) {
      itemChildren[i].selectFirst()
    } else if (i < dir.length - 1) {
      $selectedPath = dir[i + 1].path
    } else {
      dispatch('selectDown')
    }
  }
  function onSelectDown(i: number) {
    if (i < dir.length - 1) {
      $selectedPath = dir[i + 1].path
    } else {
      dispatch('selectDown')
    }
  }

  async function handleArrowKey(e: KeyboardEvent) {
    for (let i = 0; i < dir.length; i++) {
      const item = dir[i]
      if (item.path === $selectedPath) {
        e.stopImmediatePropagation()
        if (e.key === 'ArrowRight') {
          $pageMap[path][item.name].isOpen = true
        } else if (e.key === 'ArrowLeft' && $pageMap[path][item.name].isOpen) {
          $pageMap[path][item.name].isOpen = false
        } else if (e.key === 'ArrowLeft' && !$pageMap[path][item.name].isOpen) {
          selectUp(0)
        } else if (e.key === 'ArrowUp') {
          selectUp(i)
        } else if (e.key === 'ArrowDown') {
          selectDown(i)
        }
        break
      }
    }
  }
  function bodyKeydown(e: KeyboardEvent) {
    if (
      e.key === 'ArrowLeft' ||
      e.key === 'ArrowRight' ||
      e.key === 'ArrowUp' ||
      e.key === 'ArrowDown'
    ) {
      handleArrowKey(e)
      e.preventDefault()
    }
  }

  function openOrClose(item: Item) {
    if (item.isFolder) {
      $pageMap[path][item.name].isOpen = !$pageMap[path][item.name].isOpen
    }
  }

  export let indentLevel = 0
</script>

<svelte:body on:keydown={bodyKeydown} />

{#each dir as item, i}
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <!-- svelte-ignore a11y-interactive-supports-focus -->
  <div
    class="item"
    role="button"
    class:open={item.isOpen}
    class:selected={$selectedPath === item.path}
    style={`padding-left: ${14 * indentLevel + 2}px`}
    on:mousedown={() => ($selectedPath = item.path)}
    on:click={() => openOrClose(item)}
  >
    <!-- svelte-ignore a11y-interactive-supports-focus -->
    <div
      class="arrow"
      role="button"
      on:mousedown|stopPropagation={() => openOrClose(item)}
      on:click|stopPropagation
    >
      {#if $pageMap && $pageMap[item.path]}
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"
          ><path d="M21 12l-18 12v-24z" /></svg
        >
      {/if}
    </div>
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
      <svelte:self
        bind:this={itemChildren[i]}
        path={item.path}
        indentLevel={indentLevel + 1}
        on:selectUp={() => ($selectedPath = item.path)}
        on:selectDown={() => onSelectDown(i)}
      />
    {/if}
  </div>
{/each}

<style lang="sass">
  .item
    font-size: 14px
    display: flex
    align-items: center
    box-sizing: border-box
    color: hsla(216, 80%, 90%, 0.8)
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
    width: 10px
    height: 10px
    display: flex
    align-items: center
  svg
    fill: hsla(216, 80%, 90%, 0.6)
  .size
    display: inline-block
    margin-left: auto
    margin-right: 10px
</style>
