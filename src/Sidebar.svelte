<script lang="ts">
  import Item, { ItemClickEvent } from './Item.svelte'
  import { page, Backups } from './page'

  export let backups: Backups

  function sidebarClick(e: ItemClickEvent) {
    if (e.detail.isFolder) {
      e.detail.toggleChildren()
    } else {
      const paths = Object.keys(backups.dirs[e.detail.dir]).sort()
      const backupIndex = paths.indexOf(e.detail.name)
      if (backupIndex >= 1) {
        let prevPathSeparator = e.detail.dir === '/' ? '' : '/'
        $page = {
          fullPath: e.detail.fullPath,
          name: e.detail.name,
          prevPath: e.detail.dir + prevPathSeparator + paths[backupIndex - 1],
        }
      }
    }
  }
</script>

<div class="content">
  {#each Object.keys(backups.dirs[backups.rootPath]).sort() as child}
    {#if backups.rootPath === '/'}
      <Item
        map={backups.dirs}
        selectedPath={$page.fullPath + '/' + $page.name}
        name={child}
        dir="/"
        fullPath={'/' + child}
        on:click={sidebarClick} />
    {:else}
      <Item
        map={backups.dirs}
        name={child}
        dir={backups.rootPath}
        fullPath={backups.rootPath + '/' + child}
        selectedPath={$page.fullPath + '/' + $page.name}
        on:click={sidebarClick} />
    {/if}
  {/each}
</div>

<style lang="sass">
  .content
    overflow: auto
    height: 10px
    flex-grow: 1
</style>
