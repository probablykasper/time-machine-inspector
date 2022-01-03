<script lang="ts">
  import { runCmd } from './lib/general'

  let backups: string[] | null = []

  let loading = false
  async function loadBackups() {
    if (!loading) {
      loading = true
      const stdout: string | null = await runCmd('load_backups')
      if (stdout === null) {
        backups = null
      } else {
        backups = stdout.split('\n')
      }
      await new Promise((resolve) => setTimeout(resolve, 1000))
      loading = false
    }
  }
</script>

<div class="border" />
<div class="sidebar">
  <div class="button" on:click={loadBackups} class:disabled={loading} tabindex="0">
    {#if loading}
      Loading...
    {:else}
      Load Backups
    {/if}
  </div>
  {#if backups !== null}
    {#each backups as backup}
      <div class="backup">{backup}</div>
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
    width: 200px
    flex-shrink: 0
    height: 100%
    box-sizing: border-box
    overflow: auto
    padding: 15px
    backdrop-filter: brightness(80%)
    color: hsla(0, 0%, 0%, 0.7)
    border: 0px
    border-right: 1px solid hsla(230, 100%, 93%, 0.5)
    border-image: radial-gradient(ellipse at bottom right, hsla(230, 100%, 93%, 0.1), transparent) 1 / 1px
    border-image-outset: 1px
    display: flex
    flex-direction: column
    align-items: center
    .button
      margin-bottom: 15px
  .page
    width: 100%
    height: 100%
  .button
    user-select: none
    cursor: default
    outline: none
    width: 100%
    box-sizing: border-box
    padding: 9px 18px
    text-align: center
    font-size: 13px
    border: 1px solid hsla(230, 100%, 70%, 0.1)
    border-radius: 5px
    background-color: hsla(230, 80%, 70%, 0.1)
    color: hsla(216, 100%, 60%, 0.6)
    font-weight: 600
    text-shadow: 0 0.1rem 0.2rem hsl(270, 58%, 54%, 0.36)
    transition: all 360ms $easing
    &.disabled
      opacity: 0.75
      box-shadow: 0px 4px 30px 0px hsla(270, 30%, 35%, 0)
  .backup
    font-size: 13px
</style>
