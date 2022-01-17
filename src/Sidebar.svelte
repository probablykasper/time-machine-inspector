<script lang="ts">
  import Item, { ItemClickEvent } from './Item.svelte'
  import { page, Backups } from './page'

  export let backups: Backups
  $: dir = Object.keys(backups.dirs[backups.rootPath]).sort()

  function sidebarClick(e: ItemClickEvent) {
    if (e.detail.isFolder) {
      e.detail.toggleChildren()
    } else if (e.detail.prevPath) {
      $page = {
        fullPath: e.detail.fullPath,
        name: e.detail.name,
        prevPath: e.detail.prevPath,
        loading: false,
      }
    }
  }
</script>

<div class="content">
  {#each dir as child, i}
    {#if backups.rootPath === '/'}
      <Item
        map={backups.dirs}
        name={child}
        dir="/"
        fullPath={'/' + child}
        prevPath={i === 0 ? null : '/' + dir[i - 1]}
        selectedPath={$page.fullPath + '/' + $page.name}
        on:click={sidebarClick} />
    {:else}
      <Item
        map={backups.dirs}
        name={child}
        dir={backups.rootPath}
        fullPath={backups.rootPath + '/' + child}
        prevPath={i === 0 ? null : backups.rootPath + '/' + dir[i - 1]}
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
