<script lang="ts">
	import { page } from '$app/state';
	import NotebookPenIcon from '@lucide/svelte/icons/notebook-pen';
	import BackendStatus from '#lib/components/backend-status.svelte';
	import ModeToggle from '#lib/components/mode-toggle.svelte';
	import * as Sidebar from '#lib/components/ui/sidebar/index.js';
	import { isActive, navItems } from '#lib/nav.js';
</script>

<Sidebar.Root collapsible="icon">
	<Sidebar.Header>
		<Sidebar.Menu>
			<Sidebar.MenuItem>
				<Sidebar.MenuButton size="lg">
					{#snippet child({ props })}
						<a href="/" {...props}>
							<div
								class="flex aspect-square size-8 items-center justify-center rounded-lg bg-primary text-primary-foreground"
							>
								<NotebookPenIcon class="size-4" />
							</div>
							<div class="grid flex-1 text-left text-sm leading-tight">
								<span class="truncate font-semibold">Notes</span>
								<span class="truncate text-xs text-muted-foreground">Axum + shadcn-svelte</span>
							</div>
						</a>
					{/snippet}
				</Sidebar.MenuButton>
			</Sidebar.MenuItem>
		</Sidebar.Menu>
	</Sidebar.Header>

	<Sidebar.Content>
		<Sidebar.Group>
			<Sidebar.GroupLabel>Navigation</Sidebar.GroupLabel>
			<Sidebar.GroupContent>
				<Sidebar.Menu>
					{#each navItems as item (item.href)}
						<Sidebar.MenuItem>
							<Sidebar.MenuButton
								isActive={isActive(item.href, page.url.pathname)}
								tooltipContent={item.title}
							>
								{#snippet child({ props })}
									<a href={item.href} {...props}>
										<item.icon />
										<span>{item.title}</span>
									</a>
								{/snippet}
							</Sidebar.MenuButton>
						</Sidebar.MenuItem>
					{/each}
				</Sidebar.Menu>
			</Sidebar.GroupContent>
		</Sidebar.Group>
	</Sidebar.Content>

	<Sidebar.Footer>
		<Sidebar.Menu>
			<Sidebar.MenuItem>
				<ModeToggle />
			</Sidebar.MenuItem>
		</Sidebar.Menu>
		<BackendStatus class="px-2 pb-1 group-data-[collapsible=icon]:justify-center" />
	</Sidebar.Footer>

	<Sidebar.Rail />
</Sidebar.Root>
