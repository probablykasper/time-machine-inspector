<script lang="ts">
  import Item, { ItemClickEvent } from './Item.svelte'
  import { page, backups, close as closePage, loadBackups } from './page'
  import Page from './Page.svelte'

  let loading = false
  async function refreshBackups() {
    if (loading) {
      return
    }
    loading = true
    closePage()
    await loadBackups()
    loading = false
  }

  function sidebarClick(e: ItemClickEvent) {
    if (e.detail.isFolder) {
      e.detail.toggleChildren()
    } else {
      $page = {
        name: e.detail.name,
        fullPath: e.detail.fullPath,
      }
    }
  }
  refreshBackups()
</script>

<div class="sidebar">
  <button on:click={refreshBackups} class:disabled={loading} tabindex="0">
    {#if loading}
      Loading...
    {:else}
      Refresh
    {/if}
  </button>
  <div class="content">
    {#if $backups !== null}
      {#each $backups.dirs[$backups.rootPath] as child}
        {#if $backups.rootPath === '/'}
          <Item
            map={$backups.dirs}
            selectedPath={$page.fullPath + '/' + $page.name}
            name={child}
            fullPath={'/' + child}
            on:click={sidebarClick} />
        {:else}
          <Item
            map={$backups.dirs}
            selectedPath={$page.fullPath + '/' + $page.name}
            name={child}
            fullPath={$backups.rootPath + '/' + child}
            on:click={sidebarClick} />
        {/if}
      {/each}
    {/if}
  </div>
</div>
<Page />

<style lang="sass">
  $easing: cubic-bezier(0.4, 0.0, 0.2, 1)
  :global(html)
    height: 100%
    background-color: hsl(230, 72%, 7%)
    font-family: 'Open Sans', Arial, Helvetica, sans-serif
    font-size: 18px
    color: #f2f2f2
    color-scheme: dark
  :global(body)
    height: 100%
    margin: 0px
    display: flex
    box-sizing: border-box
    border-top: 0px
    background-image: radial-gradient(circle at 50% 27%, hsl(230, 65%, 33%), transparent 100%),radial-gradient(circle at 0% 90%, hsl(230, 53%, 38%), transparent 40%),radial-gradient(circle at 5% 1%, hsl(230, 71%, 4%), transparent 100%),radial-gradient(circle at 50% 50%, #000000, #000000 100%)
    background-image: radial-gradient(circle at -10% -10%, hsl(230, 65%, 30%), hsl(230, 72%, 9%) 110%)
    background-image: radial-gradient(circle at -10% -10%, #25282F, #14101C 110%)
  :global(p)
    color: hsla(216, 50%, 85%, 0.8)
    font-size: 15px
  .sidebar
    flex-shrink: 0
    flex-grow: 1
    min-width: 220px
    max-width: 35%
    height: 100%
    box-sizing: border-box
    overflow: auto
    backdrop-filter: brightness(80%)
    border: 0px
    border-right: 1px solid hsla(230, 100%, 85%, 0.12)
    display: flex
    flex-direction: column
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
  .content
    overflow: auto
    height: 10px
    flex-grow: 1
</style>
