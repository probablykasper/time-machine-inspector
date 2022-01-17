<script lang="ts">
  import type { Backups } from '../page/page'
  import SidebarItems from './SidebarItems.svelte'

  export let backups: Backups
</script>

<div class="content">
  <SidebarItems {backups} path={backups.rootPath} />
</div>

<style lang="sass">
  .content
    overflow: auto
    height: 10px
    flex-grow: 1
</style>
