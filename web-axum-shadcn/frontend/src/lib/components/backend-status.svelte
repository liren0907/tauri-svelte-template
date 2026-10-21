<script lang="ts">
	import { isBackendHealthy } from '#lib/api/health.js';
	import { cn } from '#lib/utils.js';

	const POLL_INTERVAL_MS = 5000;

	let { class: className }: { class?: string } = $props();

	let status: 'checking' | 'online' | 'offline' = $state('checking');

	// Poll GET /healthz so the UI shows when the backend is (re)started.
	$effect(() => {
		let cancelled = false;
		const check = async () => {
			const healthy = await isBackendHealthy();
			if (!cancelled) status = healthy ? 'online' : 'offline';
		};
		check();
		const timer = setInterval(check, POLL_INTERVAL_MS);
		return () => {
			cancelled = true;
			clearInterval(timer);
		};
	});

	const label = $derived(
		{ checking: 'Checking backend…', online: 'Backend online', offline: 'Backend offline' }[status]
	);
</script>

<div class={cn('flex items-center gap-2 text-xs text-muted-foreground', className)} title={label}>
	<span class="relative flex size-2.5 shrink-0">
		{#if status === 'online'}
			<span class="absolute inline-flex size-full animate-ping rounded-full bg-emerald-500 opacity-60"
			></span>
		{/if}
		<span
			class={cn(
				'relative inline-flex size-2.5 rounded-full',
				status === 'online' && 'bg-emerald-500',
				status === 'offline' && 'bg-red-500',
				status === 'checking' && 'bg-amber-400'
			)}
		></span>
	</span>
	<span class="truncate group-data-[collapsible=icon]:hidden">{label}</span>
</div>
