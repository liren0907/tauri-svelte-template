<script lang="ts">
	import './layout.css';
	import { page } from '$app/state';
	import { ModeWatcher } from 'mode-watcher';
	import AppSidebar from '#lib/components/app-sidebar.svelte';
	import { Separator } from '#lib/components/ui/separator/index.js';
	import * as Sidebar from '#lib/components/ui/sidebar/index.js';
	import { Toaster } from '#lib/components/ui/sonner/index.js';
	import { isActive, navItems } from '#lib/nav.js';
	import type { LayoutProps } from './$types';

	let { children }: LayoutProps = $props();

	const current = $derived(navItems.find((item) => isActive(item.href, page.url.pathname)));
</script>

<svelte:head>
	<title>{current ? `${current.title} · ` : ''}Axum + shadcn-svelte</title>
</svelte:head>

<ModeWatcher />
<Toaster position="top-right" />

<Sidebar.Provider>
	<AppSidebar />
	<Sidebar.Inset>
		<header class="flex h-14 shrink-0 items-center gap-2 border-b px-4">
			<Sidebar.Trigger class="-ml-1" />
			<Separator orientation="vertical" class="mr-2 data-vertical:h-4 data-vertical:self-center" />
			<span class="text-sm font-medium">{current?.title ?? ''}</span>
		</header>
		<main class="flex-1 p-4 md:p-6">
			{@render children()}
		</main>
	</Sidebar.Inset>
</Sidebar.Provider>
