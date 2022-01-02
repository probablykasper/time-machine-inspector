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
  $c1: #822b38
  $c2: #ac209c
  $c3: #f19e8e

  $easing: cubic-bezier(0.4, 0.0, 0.2, 1)

  $c1: hsl(242, 50%, 21%)
  $c2: hsl(234, 63%, 10%)
  $c1: hsl(195, 60%, 20%)
  $c2: hsl(207, 90%, 8%)
  $c1: hsl(0, 0%, 8%)
  $c2: hsl(0, 0%, 0%)
  :global(html)
    height: 100%
    background: radial-gradient(circle at 50% 0%, $c1, $c2)
    // background: $c2
    font-family: Arial, Helvetica, sans-serif
    font-size: 18px
    color: #f2f2f2
  :global(body)
    height: 100%
    margin: 0px
    display: flex
    box-sizing: border-box
    border-top: 0px
  .sidebar
    width: 180px
    flex-shrink: 0
    height: 100%
    box-sizing: border-box
    overflow: auto
    padding: 15px
    background-color: hsla(0, 0%, 100%, 0.05)
    border-right: 1px solid hsla(0, 0%, 100%, 0.05)
    display: flex
    flex-direction: column
    align-items: center
    .button
      margin-bottom: 15px
  .page
    width: 100%
    height: 100%
  .button
    display: inline-block
    position: relative
    user-select: none
    -webkit-user-select: none
    cursor: default
    outline: none
    width: 100%
    box-sizing: border-box
    padding: 7px 14px
    color: #ffffff
    text-align: center
    font-size: 13px
    background-image: radial-gradient(circle at 50% 0%, hsla(165, 100%, 35%, 0.5), hsla(165, 100%, 28%, 0.5))
    border: 1px solid hsla(0, 0%, 100%, 0.08)
    border-radius: 4px
    box-shadow: 0px 0px 10px hsla(165, 80%, 35%, 0.5)
    transition: all 120ms $easing
    &::before
      content: ''
      z-index: -1
      position: absolute
      top: 0px
      left: 0px
      width: 100%
      height: 100%
      background-image: radial-gradient(circle at 50% 0%, hsla(165, 100%, 35%, 1), transparent)
      border-radius: 4px
      opacity: 0
      transition: all 180ms $easing
    &:hover
      box-shadow: 0px 0px 10px 2px hsla(165, 80%, 30%, 0.8)
      border: 1px solid hsla(0, 0%, 100%, 0.1)
      &::before
        opacity: 0.5
    &.disabled
      opacity: 0.75
  .backup
    color: #ffffff
    font-size: 13px
</style>
