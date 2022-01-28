<script lang="ts">
  import { runCmd } from '../general'
  import PageItems from './PageItems.svelte'
  import { backups, page, backupInfos, pageMap, PageMap } from './page'
  import Button from '../lib/Button.svelte'
  import ProgressBar from '../lib/ProgressBar.svelte'

  async function compare(autoLoad = false) {
    if ($page.loading || $backups === null) {
      return
    }
    const fullPathParent = $page.fullPath.substring(0, $page.fullPath.lastIndexOf('/'))
    const dir = $backups.dirs[fullPathParent][$page.name]
    if (dir !== undefined) {
      if (!autoLoad) {
        $page.loading = true
      }
      const result = (await runCmd('get_backup', {
        old: $page.prevPath,
        new: $page.fullPath,
        refresh: false,
      })) as { map: PageMap; cached_paths: [string, string][] }
      $pageMap = result.map
      backupInfos.load()
      console.log('Page items', result)
    }
  }

  $: autoLoad($page.prevPath, $page.fullPath)
  function autoLoad(oldPath: string, newPath: string) {
    for (const info of $backupInfos) {
      if (info.old === oldPath && info.new === newPath) {
        compare(true)
      }
    }
  }
</script>

{#if $page.fullPath === ''}
  <main class="empty">
    <p>You can open a backup from the sidebar when it's loaded</p>
  </main>
{:else}
  <main>
    <div class="bar">{$page.fullPath}</div>
    <div class="content">
      {#if $page.loading}
        <div class="absolute center-align">
          <ProgressBar />
        </div>
      {:else if $pageMap === null || $pageMap[$page.fullPath] === undefined}
        <div class="absolute center-align">
          <Button on:click={() => compare()}>Load</Button>
        </div>
      {:else}
        <PageItems path={$page.fullPath} />
      {/if}
    </div>
  </main>
{/if}

<style lang="sass">
  main
    width: 100%
    height: 100%
    box-sizing: border-box
    display: flex
    flex-direction: column
  main.empty
    display: flex
    align-items: center
    justify-content: center
    font-size: 14px
    padding: 30px
    text-align: center
  .content
    position: relative
    overflow: auto
    height: 100%
    width: 100%
  .absolute
    position: absolute
    top: 0px
    left: 0px
  .center-align
    height: 100%
    width: 100%
    display: flex
    align-items: center
    justify-content: center
  .bar
    background-color: hsla(230, 80%, 90%, 0.1)
    font-size: 13px
    padding: 5px 10px
</style>
