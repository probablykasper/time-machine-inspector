<script lang="ts">
  import Page from './page/Page.svelte'
  import Sidebar from './sidebar/Sidebar.svelte'
  import { close as closePage } from './page/page'
  import { fade } from 'svelte/transition'
  import { cubicInOut } from 'svelte/easing'
  import ProgressBar from './lib/ProgressBar.svelte'
  import Button from './lib/Button.svelte'
  import commands from './lib/commands'
  import type { Backup, DestinationDetail } from '../bindings'

  let destinations: DestinationDetail[] | null = null
  let selectedDestination: DestinationDetail | null = null
  let backups: Backup[] | null = null
  let loading = false
  async function refresh(refresh = false) {
    let minEndTime = Date.now() + 250
    function timeRemaining() {
      return Math.max(minEndTime - Date.now(), 0)
    }
    if (loading) {
      return
    }
    loading = true
    selectedDestination = null
    closePage()

    destinations = await commands.destinationinfo()

    if (destinations[0]) {
      setTimeout(() => {
        selectedDestination = destinations[0]
      }, timeRemaining())

      backups = await commands.loadBackupList(destinations[0].id, refresh)
      console.log('Loaded backups', backups)
    }
    await new Promise((resolve) => {
      setTimeout(resolve, timeRemaining())
    })
    loading = false
  }
  refresh()
</script>

<div class="sidebar">
  {#if loading}
    <div class="loading" transition:fade={{ duration: 500, easing: cubicInOut }}>
      <ProgressBar />
    </div>
  {/if}
  <Button disabled={loading} on:click={() => refresh(true)}>Refresh</Button>
  <div class="mount-point">
    {#if selectedDestination}
      <div transition:fade={{ duration: 300, easing: cubicInOut }}>
        {#if destinations.length >= 2}
          <select
            value={selectedDestination.id}
            disabled={loading}
            on:change={async (e) => {
              loading = true
              selectedDestination = destinations.find((d) => d.id === e.currentTarget.value) || null
              backups = await commands.loadBackupList(destinations[0].id, false)
              console.log('Loaded backups', backups)
              loading = false
            }}
          >
            {#each destinations as destination}
              <option value={destination.id}>{destination.mount_point_name}</option>
            {/each}
          </select>
        {:else}
          <span>{selectedDestination.mount_point_name}</span>
          <!-- <select value={destination}>
            {#each destinations as destination}
              <option value={destination}>{destination.mount_point_name}</option>
            {/each}
          </select> -->
        {/if}
      </div>
    {/if}
  </div>
  {#if !loading}
    <div class="sidebar-stuff" transition:fade={{ duration: 300, easing: cubicInOut }}>
      {#if backups}
        <Sidebar {backups} />
      {/if}
    </div>
  {/if}
</div>
<Page destination={selectedDestination} />

<style lang="sass">
  @font-face
    font-family: 'Open Sans'
    src: url("/OpenSans-VariableFont_wdth,wght.ttf")
    font-weight: 300 800
    font-stretch: 75% 125
    font-style: normal
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
    background-image: radial-gradient(circle at 0% 0%, #0e3930, #090b10 100%)
  :global(p)
    color: hsla(216, 50%, 85%, 0.8)
    font-size: 15px
  .sidebar
    position: relative
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
  .sidebar-stuff
    display: flex
    flex-direction: column
    flex-grow: 1
  .mount-point
    font-size: 15px
    font-weight: 600
    margin-left: auto
    margin-right: auto
    color: hsla(216, 50%, 85%, 1)
    height: 25px
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
