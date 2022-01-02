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
      loading = false
    }
  }
</script>

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

<style lang="sass">
  :global(html)
    height: 100%
    background-color: #0f1114
  :global(body)
    font-family: Arial, Helvetica, sans-serif
    font-size: 18px
    color: #f2f2f2
  .button
    display: inline-block
    position: relative
    user-select: none
    -webkit-user-select: none
    cursor: default
    outline: none
    width: 90px
    padding: 5px 12px
    margin: 10px
    color: #f2f2f2
    border-radius: 3px
    font-size: 14px
    background-color: rgba(255,255,255,0.15)
    box-shadow: 0px 0px 0px 1px hsla(0, 0%, 100%, 0.3)
    transition: all 0.2s ease-in-out
    &.disabled
      opacity: 0.75
  .backup
    color: #ffffff
    font-size: 13px
</style>
