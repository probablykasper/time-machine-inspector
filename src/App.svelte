<script lang="ts">
  import { runCmd } from './general'
  import Item, { ItemClickEvent } from './Item.svelte'
  import { page } from './page'

  let rootPath = '/'
  let backups: DirMap | null = null

  type DirMap = {
    '/': string[]
    [name: string]: string[]
  }

  function parseStdout(stdout: string) {
    const paths = stdout.split('\n').filter((path) => path !== '')
    const dirmap: DirMap = { '/': [] }

    paths.forEach((path) => {
      let base = path.substring(path.lastIndexOf('/') + 1)
      let parent = path.substring(0, path.lastIndexOf('/'))
      do {
        if (parent === '') {
          dirmap['/'].push(base)
          break
        } else if (dirmap[parent] === undefined) {
          dirmap[parent] = [base]
        } else {
          dirmap[parent].push(base)
          break
        }
        base = parent.substring(parent.lastIndexOf('/') + 1)
        parent = parent.substring(0, parent.lastIndexOf('/'))
      } while (dirmap[parent] === undefined)
    })

    console.log(dirmap)
    return dirmap
  }

  let stdout: string | null = null
  $: stdoutUpdate(stdout)
  function stdoutUpdate(stdout: string | null) {
    if (stdout === null) {
      backups = null
    } else {
      backups = parseStdout(stdout)
      if (backups['/'].length === 1 && backups['/'][0] === 'Volumes') {
        rootPath = '/Volumes'
      } else {
        rootPath = '/'
      }
    }
  }

  let loading = false
  async function loadBackups() {
    if (loading) {
      return
    }
    loading = true
    stdout = (await runCmd('load_backups')) as string | null
    await new Promise((resolve) => setTimeout(resolve, 1000))
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
</script>

<div class="sidebar">
  <button on:click={loadBackups} class:disabled={loading} tabindex="0">
    {#if loading}
      Loading...
    {:else}
      Load Backups
    {/if}
  </button>
  {#if backups !== null}
    {#each backups[rootPath] as child}
      {#if rootPath === '/'}
        <Item
          map={backups}
          selectedPath={$page.fullPath + '/' + $page.name}
          name={child}
          fullPath={'/' + child}
          on:click={sidebarClick} />
      {:else}
        <Item
          map={backups}
          selectedPath={$page.fullPath + '/' + $page.name}
          name={child}
          fullPath={rootPath + '/' + child}
          on:click={sidebarClick} />
      {/if}
    {/each}
  {/if}
</div>
<div class="page" />

<style lang="sass">
  $easing: cubic-bezier(0.4, 0.0, 0.2, 1)
  :global(html)
    height: 100%
    background-color: hsl(230, 72%, 7%)
    font-family: Karla, Arial, Helvetica, sans-serif
    font-size: 18px
    color: #f2f2f2
  :global(body)
    height: 100%
    margin: 0px
    display: flex
    box-sizing: border-box
    border-top: 0px
    background-image: radial-gradient(circle at 50% 27%, hsl(230, 65%, 33%), transparent 100%),radial-gradient(circle at 0% 90%, hsl(230, 53%, 38%), transparent 40%),radial-gradient(circle at 5% 1%, hsl(230, 71%, 4%), transparent 100%),radial-gradient(circle at 50% 50%, #000000, #000000 100%)
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
  .page
    width: 100%
    height: 100%
  button
    user-select: none
    cursor: default
    outline: none
    box-sizing: border-box
    padding: 9px 18px
    text-align: center
    font-size: 13px
    margin: 15px
    border: 1px solid hsla(230, 100%, 80%, 0.1)
    border-radius: 5px
    background-color: hsla(230, 80%, 75%, 0.2)
    color: hsla(230, 100%, 90%, 0.8)
    font-weight: 500
    text-shadow: 0px 0.1em 0.8em hsla(230, 30%, 7%, 1)
    transition: all 240ms $easing
    &:focus
      box-shadow: 0px 0px 0px 2px hsla(230, 100%, 80%, 0.5)
    &.disabled, &:hover
      font-weight: 700
      letter-spacing: 0.02em
    &.disabled, &:active
      opacity: 0.75
</style>
