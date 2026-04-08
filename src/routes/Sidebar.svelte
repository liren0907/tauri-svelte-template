<script lang="ts">
	// @ts-ignore - SvelteKit built-in module, works at runtime
	import { page } from "$app/stores";

	let {
		isSidebarExpanded = true,
		toggleSidebar = () => {},
		width = 256,
	}: {
		isSidebarExpanded?: boolean;
		toggleSidebar?: () => void;
		width?: number;
	} = $props();

	const navItems = [
		{ href: "/", label: "Media Viewer", icon: "image" },
		{ href: "/showcase", label: "UI Showcase", icon: "palette" },
	];

	let isDark = $state(false);

	$effect(() => {
		isDark = document.documentElement.getAttribute("data-theme") === "dark";
	});

	function toggleTheme() {
		isDark = !isDark;
		const theme = isDark ? "dark" : "light";
		document.documentElement.setAttribute("data-theme", theme);
		localStorage.setItem("theme", theme);
	}
</script>

<ul
	class="menu bg-base-100 min-h-full p-4 gap-1 border-r border-base-200 text-base-content"
	style="width: {width}px"
>
	<!-- Logo / Title -->
	<li
		class="menu-title flex flex-row items-center justify-between gap-2 mb-6 px-2"
	>
		<div class="flex items-center gap-2.5">
			<div
				class="w-8 h-8 rounded-lg bg-primary/10 flex items-center justify-center text-primary border border-primary/20"
			>
				<span class="material-symbols-rounded">apps</span>
			</div>
			<span class="text-base font-bold text-base-content"
				>My App</span
			>
		</div>
		<button
			onclick={toggleSidebar}
			class="btn btn-ghost btn-xs btn-square text-base-content/40 hover:text-base-content"
		>
			<span class="material-symbols-rounded">first_page</span>
		</button>
	</li>

	<!-- Navigation Items -->
	{#each navItems as item (item.href)}
		{@const isActive =
			item.href === "/"
				? $page.url.pathname === "/"
				: $page.url.pathname === item.href ||
					$page.url.pathname.startsWith(item.href + "/")}
		<li>
			<a
				href={item.href}
				class="flex items-center gap-3 px-3 py-2 rounded-lg transition-colors duration-200
                {isActive
					? 'bg-primary/10 text-primary font-medium'
					: 'text-base-content/70 hover:bg-base-200 hover:text-base-content'}"
			>
				<span
					class="material-symbols-rounded {isActive
						? 'filled'
						: ''} text-[20px]">{item.icon}</span
				>
				<span class="text-sm">{item.label}</span>
			</a>
		</li>
	{/each}

	<!-- Spacer -->
	<div class="flex-1"></div>

	<!-- Theme Toggle (Pinned to Bottom) -->
	<li>
		<button
			onclick={toggleTheme}
			class="flex items-center gap-3 px-3 py-2 rounded-lg transition-colors duration-200
			text-base-content/70 hover:bg-base-200 hover:text-base-content"
		>
			<span class="material-symbols-rounded text-[20px]">
				{isDark ? "dark_mode" : "light_mode"}
			</span>
			<span class="text-sm">{isDark ? "Dark Mode" : "Light Mode"}</span>
		</button>
	</li>
</ul>

<style>
	.menu :global(.active) {
		@apply bg-primary text-primary-content;
	}
</style>
