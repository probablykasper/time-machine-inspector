<script lang="ts">
	import type { Backup, BackupInfo } from '../../bindings'
	import { page, backupInfos } from '../page/page'

	export let backups: Backup[]

	const enum LoadState {
		NothingToLoad,
		Loading,
		Loaded,
	}

	function getLoadState(backupInfos: BackupInfo[], path: unknown): LoadState {
		for (const info of backupInfos) {
			if (info.new === path) {
				if (info.loading) {
					return 1
				} else {
					return 2
				}
			}
		}
		return 0
	}
</script>

<div class="content">
	{#each backups as backup}
		<button
			type="button"
			class="item"
			data-load-state={getLoadState($backupInfos, backup.path)}
			class:selected={$page.backup?.path === backup.path}
			on:click={() => {
				$page = {
					backup,
					loading: false,
				}
			}}
		>
			{backup.name}
		</button>
	{/each}
</div>

<style lang="sass">
	.content
		overflow: auto
		height: 10px
		flex-grow: 1
	$ease-md: cubic-bezier(0.4, 0.0, 0.2, 1)
	.item
		display: block
		font-family: inherit
		text-align: left
		background-color: transparent
		border: 0px
		font-size: 14px
		color: hsla(216, 50%, 70%, 0.75)
		cursor: default
		user-select: none
		padding: 4px 15px
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
</style>
