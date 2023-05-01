<script lang="ts">
  import { page, backupInfos, type Backups, type BackupInfo } from '../page/page'

  export let path: string
  export let backups: Backups
  export let indentLevel = 0

  function getChildPath(path: string, childPath: string) {
    if (path === '/') {
      return '/' + childPath
    } else {
      return path + '/' + childPath
    }
  }

  type Item = {
    name: string
    path: string
    prevPath: string | null
    isFolder: boolean
    show: boolean
  }

  $: dir = getDir(backups, path)
  function getDir(backups: Backups, path: string): Item[] {
    const names = Object.keys(backups.dirs[path]).sort()
    let prevChildPath = null as string | null
    return names.map((name) => {
      const childPath = getChildPath(path, name)
      const item: Item = {
        name,
        path: childPath,
        prevPath: prevChildPath,
        isFolder: backups.dirs[childPath] !== undefined,
        show: true,
      }
      prevChildPath = childPath
      return item
    })
  }

  function getLoadState(backupInfos: BackupInfo[], prevPath: unknown, path: unknown) {
    for (const info of backupInfos) {
      if (info.old === prevPath && info.new === path) {
        if (info.loading) {
          return 1
        } else {
          return 2
        }
      }
    }
    return 0
  }

  function itemClick(item: Item) {
    if (item.isFolder) {
      item.show = !item.show
    } else if (item.prevPath && $page.fullPath !== item.path) {
      console.log('Open', item)

      $page = {
        fullPath: item.path,
        name: item.name,
        prevPath: item.prevPath,
        loading: false,
      }
    }
  }
</script>

{#each dir as item}
  <div
    class="item"
    data-load-state={getLoadState($backupInfos, item.prevPath, item.path)}
    class:selected={$page.fullPath === item.path}
    style={`padding-left: ${14 * indentLevel + 10}px`}
    on:click={() => itemClick(item)}
  >
    {#if item.isFolder}
      <svg
        class:open={item.show}
        xmlns="http://www.w3.org/2000/svg"
        width="24"
        height="24"
        viewBox="0 0 24 24"><path d="M21 12l-18 12v-24z" /></svg
      >
    {/if}
    {item.name}
  </div>
  {#if item.isFolder && item.show}
    <svelte:self {backups} path={item.path} indentLevel={indentLevel + 1} />
  {/if}
{/each}

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
  [data-load-state="1"]
    animation: flash 1s $ease-md infinite alternate
  [data-load-state="2"]
    font-weight: 600
    color: hsla(216, 50%, 80%, 0.9)
  .selected
    background-color: hsla(216, 70%, 70%, 0.2)
    animation: none

  @keyframes flash
    0%
      color: hsla(216, 50%, 70%, 0.5)
    100%
      color: hsla(216, 50%, 70%, 0.9)

  svg
    fill: hsla(216, 50%, 70%, 0.75)
    width: 10px
    height: 10px
    margin-right: 1px
  svg.open
    transform: rotate(90deg)
</style>
