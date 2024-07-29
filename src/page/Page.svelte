<script lang="ts">
	import PageItems from './PageItems.svelte'
	import { page, backupInfos, pageMap, selectedPath } from './page'
	import Button from '../lib/Button.svelte'
	import ProgressBar from '../lib/ProgressBar.svelte'
	import commands from '../lib/commands'
	import type { DestinationDetail } from '../../bindings'
	import { tick } from 'svelte'

	let content_el: HTMLDivElement
	export let destination: DestinationDetail | null = null

	async function compare(autoLoad = false) {
		if ($page.loading || $page.backup === null || !destination) {
			return
		}
		if (!autoLoad) {
			$page.loading = true
		}
		const result = await commands.getBackup(destination.id, $page.backup.path, false)
		$pageMap = result.map
		backupInfos.load()
	}

	$: if ($page.backup) {
		autoLoad($page.backup.path)
	}
	function autoLoad(newPath: string) {
		for (const info of $backupInfos) {
			if (info.new === newPath) {
				compare(true)
			}
		}
	}

	$: if ($selectedPath) {
		tick().then(() => {
			console.log('.selected', document.querySelector('.selected'))
			content_el
				.querySelector('.selected')
				?.scrollIntoView({ behavior: 'instant', block: 'nearest', inline: 'nearest' })
		})
	}
</script>

{#if !destination}
	<main class="empty">
		<p>You can open a backup from the sidebar when it's loaded</p>
	</main>
{:else if !$page.backup}
	<main class="empty">
		<p>You can open a backup from the sidebar</p>
	</main>
{:else}
	<main>
		<div class="bar">{$page.backup.path}</div>
		<div class="content" bind:this={content_el}>
			{#if $page.loading}
				<div class="absolute center-align">
					<ProgressBar />
				</div>
			{:else if $pageMap === null || $pageMap[$page.backup.path] === undefined}
				<div class="absolute center-align">
					<Button on:click={() => compare()}>Load</Button>
				</div>
			{:else}
				<PageItems path={$page.backup.path} />
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
