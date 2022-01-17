<script lang="ts">
  import Page from './page/Page.svelte'
  import Sidebar from './sidebar/Sidebar.svelte'
  import { backups, close as closePage, loadBackups } from './page/page'
  import { fade } from 'svelte/transition'
  import { cubicInOut } from 'svelte/easing'
  import ProgressBar from './lib/ProgressBar.svelte'
  import Button from './lib/Button.svelte'

  let loading = false
  async function load(refresh = false) {
    let minEndTime = Date.now() + 250
    if (loading) {
      return
    }
    loading = true
    closePage()
    await loadBackups(refresh)
    let timeRemaining = Math.max(minEndTime - Date.now())
    await new Promise((resolve) => {
      setTimeout(resolve, timeRemaining)
    })
    loading = false
  }
  load()
</script>

<div class="sidebar">
  {#if loading}
    <div class="loading" transition:fade={{ duration: 500, easing: cubicInOut }}>
      <ProgressBar />
    </div>
  {:else}
    <div class="sidebar-stuff" transition:fade={{ duration: 300, easing: cubicInOut }}>
      <Button disabled={loading} on:click={() => load(true)}>Refresh</Button>
      {#if $backups !== null}
        <Sidebar backups={$backups} />
      {/if}
    </div>
  {/if}
</div>
<Page />

<style lang="sass">
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
    background-image: radial-gradient(circle at -10% -10%, #080A16, #14101C 110%)
    background-image: radial-gradient(circle at -10% -10%, #133e34, #0b0d13 70%)
  :global(p)
    color: hsla(216, 50%, 85%, 0.8)
    font-size: 15px
  .sidebar
    position: relative
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
  .sidebar-stuff
    height: 100%
    display: flex
    flex-direction: column
  .loading
    height: 100%
    position: absolute // for transition
    top: 0px
    left: 0px
    width: 100%
    display: flex
    align-items: center
    justify-content: center
</style>
