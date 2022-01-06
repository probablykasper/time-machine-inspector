<script lang="ts">
  import { runCmd } from './general'
  import PageItem, { ItemClickEvent } from './PageItem.svelte'
  import { backups, page } from './page'

  let selectedPath = ''

  type DirItem = {
    [key: string]: number
  }
  type DirMap = {
    [key: string]: DirItem
  }
  let dirMap: DirMap | null = null

  let loading = false
  async function compare() {
    if (loading || $backups === null) {
      return
    }
    const fullPathParent = $page.fullPath.substring(0, $page.fullPath.lastIndexOf('/'))
    const dir = $backups.dirs[fullPathParent]
    const selectedBackupIndex = dir.indexOf($page.name)
    if (selectedBackupIndex >= 1) {
      const prevBackup = fullPathParent + '/' + dir[selectedBackupIndex - 1]
      const args = { old: prevBackup, new: $page.fullPath }
      loading = true
      dirMap = (await runCmd('compare_backups', args)) as DirMap
      console.log(dirMap)
    }
    loading = false
  }

  function itemClick(e: ItemClickEvent) {
    if (e.detail.isFolder) {
      e.detail.toggleChildren()
    }
    selectedPath = e.detail.fullPath + '/' + e.detail.name
  }
</script>

{#if $page.fullPath === ''}
  <main class="empty">
    <p>You can open a backup from the sidebar when it's loaded</p>
    <button on:click={compare}>Load</button>
  </main>
{:else}
  <main>
    <div class="bar">{$page.fullPath}</div>
    <button on:click={compare}>
      {#if loading}
        Loading...
      {:else}
        Load
      {/if}
    </button>
    {#if dirMap !== null}
      {#each Object.entries(dirMap['/']) as [childName, size]}
        <PageItem
          map={dirMap}
          {selectedPath}
          name={childName}
          fullPath={'/' + childName}
          on:click={itemClick} />
      {/each}
    {/if}
  </main>
{/if}

<style lang="sass">
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
</style>
