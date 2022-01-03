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
      // await new Promise((resolve) => setTimeout(resolve, 1000))
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
  $c1: #F3A3AB
  $c2: #dd83cb
  $c3: #FFFFFF
  :global(html)
    height: 100%
    background-color: #F7FAFE
    font-family: Karla
    // , Arial, Helvetica, sans-serif
    font-size: 18px
    color: #f2f2f2
  :global(body)
    height: 100%
    margin: 0px
    display: flex
    box-sizing: border-box
    border-top: 0px
    background: radial-gradient(ellipse at 10% 10%, hsla(9, 72%, 75%, 1) 0%, rgba(255,255,255,0) 100%) 0px 0px, radial-gradient(ellipse at 65% 0%, hsla(278, 89%, 76%, 1) 0%, rgba(69,168,255,0) 95%) 0px 0px, linear-gradient(0deg, hsla(0, 0%, 100%, 1) 0%, hsla(0, 0%, 100%, 1) 100%) 0px 0px
  .sidebar
    width: 200px
    flex-shrink: 0
    height: 100%
    box-sizing: border-box
    overflow: auto
    padding: 15px
    background-color: hsla(9, 100%, 95%, 0.25)
    color: hsla(0, 0%, 0%, 0.7)
    border: 0px
    border-right: 1px solid hsla(9, 100%, 93%, 0.5)
    border-image: radial-gradient(ellipse at top right, hsla(9, 100%, 93%, 0.8), transparent) 1 / 1px
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
    -webkit-user-select: none
    cursor: default
    outline: none
    width: 100%
    box-sizing: border-box
    padding: 7px 14px
    text-align: center
    font-size: 13px
    // border: 1px solid hsla(9, 100%, 93%, 0.8)
    // border-radius: 5px
    background-color: hsla(9, 100%, 95%, 0.35)
    color: #ffffff
    font-weight: 600
    text-shadow: 0 0.1rem 0.2rem hsl(270, 58%, 54%, 0.36)
    box-shadow: 0px 4px 10px 0px hsla(270, 30%, 35%, 0.2)
    transition: all 360ms $easing

    @function svg-gradient-border($angle, $border-width, $border-radius, $stops)
      $length: 20%
      @return url('data:image/svg+xml;utf8,<svg xmlns="http://www.w3.org/2000/svg"><defs><linearGradient gradientTransform="rotate(#{$angle})" id="Gradient" x1="0" x2="#{$length}" y1="0" y2="0" gradientUnits="userSpaceOnUse">#{$stops}</linearGradient></defs><rect x="#{calc($border-width / 2)}" y="#{calc($border-width / 2)}" width="100%" height="100%" style="height:calc(100% - #{$border-width}px);width:calc(100% - #{$border-width}px)" rx="#{$border-radius}" ry="#{$border-radius}" stroke-width="#{$border-width}" fill="transparent" stroke="url(%23Gradient)"/></svg>')

    $stop1: '<stop stop-color="hsla(9, 100%, 94%, 0.8)" offset="0%"/>'
    $stop2: '<stop stop-color="hsla(9, 100%, 94%, 0)" offset="100%"/>'
    border-radius: 5px
    background-image: svg-gradient-border(90, 1, 5, $stop1+$stop2)
    &.disabled
      opacity: 0.75
      box-shadow: 0px 4px 30px 0px hsla(270, 30%, 35%, 0)
  .backup
    font-size: 13px
</style>
