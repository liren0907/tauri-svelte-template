<script lang="ts">
	import MoonIcon from '@lucide/svelte/icons/moon';
	import SunIcon from '@lucide/svelte/icons/sun';
	import { mode, toggleMode } from 'mode-watcher';
	import * as Sidebar from '#lib/components/ui/sidebar/index.js';

	const isDark = $derived(mode.current === 'dark');
</script>

<Sidebar.MenuButton onclick={toggleMode} tooltipContent="Toggle theme">
	{#if isDark}
		<MoonIcon />
	{:else}
		<SunIcon />
	{/if}
	<span>{isDark ? 'Dark mode' : 'Light mode'}</span>
</Sidebar.MenuButton>
