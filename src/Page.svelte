<script lang="ts">
  import { runCmd } from './general'
  import PageItem, { ItemClickEvent } from './PageItem.svelte'
  import { backups, page, loadCachedBackups, cachedBackups } from './page'

  let selectedPath = ''

  type DirItem = {
    [key: string]: number
  }
  type DirMap = {
    [key: string]: DirItem
  }
  let dirMap: DirMap | null = null

  let loading = false
  async function compare(autoLoad = false) {
    if (loading || $backups === null) {
      return
    }
    const fullPathParent = $page.fullPath.substring(0, $page.fullPath.lastIndexOf('/'))
    const dir = $backups.dirs[fullPathParent][$page.name]
    if (dir !== undefined) {
      if (!autoLoad) {
        loading = true
      }
      const result = (await runCmd('compare_backups', {
        old: $page.prevPath,
        new: $page.fullPath,
        refresh: false,
      })) as { map: DirMap; cached_paths: [string, string][] }
      dirMap = result.map
      loadCachedBackups()
      console.log(result)
    }
    loading = false
  }

  // auto load
  $: if ($cachedBackups.includes($page.fullPath)) {
    compare(true)
  }

  function itemClick(e: ItemClickEvent) {
    if (e.detail.isFolder) {
      e.detail.toggleChildren()
    }
    selectedPath = e.detail.fullPath + '/' + e.detail.name
  }

  $: rootPath = $page.fullPath
</script>

{#if $page.fullPath === ''}
  <main class="empty">
    <p>You can open a backup from the sidebar when it's loaded</p>
  </main>
{:else}
  <main>
    <div class="bar">{$page.fullPath}</div>
    {#if dirMap === null || dirMap[rootPath] === undefined}
      <button on:click={() => compare()} class:disabled={loading}>
        {#if loading}
          Loading...
        {:else}
          Load
        {/if}
      </button>
    {:else}
      {#each Object.entries(dirMap[rootPath]) as [childName, size]}
        <PageItem
          map={dirMap}
          {selectedPath}
          name={childName}
          {size}
          fullPath={rootPath + '/' + childName}
          on:click={itemClick} />
      {/each}
    {/if}
  </main>
{/if}

<style lang="sass">
  $easing: cubic-bezier(0.4, 0.0, 0.2, 1)
  main
    width: 100%
    height: 100%
    box-sizing: border-box
    overflow: auto
  main.empty
    display: flex
    align-items: center
    justify-content: center
    font-size: 14px
    padding: 30px
    text-align: center
  .bar
    background-color: hsla(230, 80%, 90%, 0.1)
    font-size: 13px
    padding: 5px 10px
  button
    font-family: inherit
    user-select: none
    cursor: default
    outline: none
    box-sizing: border-box
    padding: 7px 18px
    text-align: center
    font-size: 13px
    margin: 15px
    border: 1px solid hsla(230, 100%, 80%, 0.1)
    border-radius: 5px
    background-color: hsla(230, 80%, 75%, 0.2)
    color: hsla(230, 100%, 90%, 0.8)
    font-weight: 530
    text-shadow: 0px 0.1em 0.8em hsla(230, 30%, 7%, 1)
    transition: all 240ms $easing
    &:focus
      box-shadow: 0px 0px 0px 2px hsla(230, 100%, 80%, 0.5)
    &.disabled, &:hover
      font-weight: 600
      letter-spacing: 0.05em
    &.disabled, &:active
      opacity: 0.75
</style>
