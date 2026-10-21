<script lang="ts">
	import CircleCheckIcon from '@lucide/svelte/icons/circle-check';
	import InfoIcon from '@lucide/svelte/icons/info';
	import Loader2Icon from '@lucide/svelte/icons/loader-2';
	import MailIcon from '@lucide/svelte/icons/mail';
	import PlusIcon from '@lucide/svelte/icons/plus';
	import SettingsIcon from '@lucide/svelte/icons/settings';
	import TriangleAlertIcon from '@lucide/svelte/icons/triangle-alert';
	import { toast } from 'svelte-sonner';
	import * as Alert from '#lib/components/ui/alert/index.js';
	import { Badge } from '#lib/components/ui/badge/index.js';
	import { Button, buttonVariants } from '#lib/components/ui/button/index.js';
	import * as Card from '#lib/components/ui/card/index.js';
	import { Checkbox } from '#lib/components/ui/checkbox/index.js';
	import * as Dialog from '#lib/components/ui/dialog/index.js';
	import { Input } from '#lib/components/ui/input/index.js';
	import { Label } from '#lib/components/ui/label/index.js';
	import { Skeleton } from '#lib/components/ui/skeleton/index.js';
	import { Switch } from '#lib/components/ui/switch/index.js';
	import * as Tabs from '#lib/components/ui/tabs/index.js';
	import { Textarea } from '#lib/components/ui/textarea/index.js';
	import * as Tooltip from '#lib/components/ui/tooltip/index.js';

	let name = $state('');
	let notifications = $state(true);
	let acceptTerms = $state(false);
</script>

<div class="mx-auto flex max-w-5xl flex-col gap-6">
	<div class="space-y-1">
		<h1 class="text-2xl font-semibold tracking-tight">UI Showcase</h1>
		<p class="text-sm text-muted-foreground">
			shadcn-svelte components with Lucide icons. Add more with
			<code class="rounded bg-muted px-1 py-0.5 text-xs">npx shadcn-svelte add &lt;component&gt;</code>.
		</p>
	</div>

	<div class="grid gap-6 lg:grid-cols-2">
		<Card.Root>
			<Card.Header>
				<Card.Title>Buttons</Card.Title>
				<Card.Description>Variants, sizes and states.</Card.Description>
			</Card.Header>
			<Card.Content class="flex flex-col gap-4">
				<div class="flex flex-wrap gap-2">
					<Button>Default</Button>
					<Button variant="secondary">Secondary</Button>
					<Button variant="outline">Outline</Button>
					<Button variant="ghost">Ghost</Button>
					<Button variant="destructive">Destructive</Button>
					<Button variant="link">Link</Button>
				</div>
				<div class="flex flex-wrap items-center gap-2">
					<Button size="sm">Small</Button>
					<Button>Default</Button>
					<Button size="lg">Large</Button>
					<Button size="icon" aria-label="Add"><PlusIcon /></Button>
					<Button><MailIcon /> With icon</Button>
					<Button disabled><Loader2Icon class="animate-spin" /> Loading</Button>
				</div>
			</Card.Content>
		</Card.Root>

		<Card.Root>
			<Card.Header>
				<Card.Title>Badges & Tooltip</Card.Title>
				<Card.Description>Small labels and hover hints.</Card.Description>
			</Card.Header>
			<Card.Content class="flex flex-col gap-4">
				<div class="flex flex-wrap gap-2">
					<Badge>Default</Badge>
					<Badge variant="secondary">Secondary</Badge>
					<Badge variant="outline">Outline</Badge>
					<Badge variant="destructive">Destructive</Badge>
					<Badge variant="ghost">Ghost</Badge>
				</div>
				<div>
					<Tooltip.Root>
						<Tooltip.Trigger class={buttonVariants({ variant: 'outline' })}>
							<SettingsIcon /> Hover me
						</Tooltip.Trigger>
						<Tooltip.Content>Tooltips work anywhere inside the layout.</Tooltip.Content>
					</Tooltip.Root>
				</div>
			</Card.Content>
		</Card.Root>

		<Card.Root>
			<Card.Header>
				<Card.Title>Form controls</Card.Title>
				<Card.Description>Inputs bound to Svelte state.</Card.Description>
			</Card.Header>
			<Card.Content class="flex flex-col gap-4">
				<div class="grid gap-2">
					<Label for="showcase-name">Name</Label>
					<Input id="showcase-name" bind:value={name} placeholder="Type your name" />
				</div>
				<div class="grid gap-2">
					<Label for="showcase-bio">Bio</Label>
					<Textarea id="showcase-bio" placeholder="A few words about you" />
				</div>
				<div class="flex items-center gap-3">
					<Switch id="showcase-notifications" bind:checked={notifications} />
					<Label for="showcase-notifications">Email notifications</Label>
				</div>
				<div class="flex items-center gap-3">
					<Checkbox id="showcase-terms" bind:checked={acceptTerms} />
					<Label for="showcase-terms">Accept terms and conditions</Label>
				</div>
				<p class="text-xs text-muted-foreground">
					name = "{name}", notifications = {notifications}, acceptTerms = {acceptTerms}
				</p>
			</Card.Content>
		</Card.Root>

		<Card.Root>
			<Card.Header>
				<Card.Title>Feedback</Card.Title>
				<Card.Description>Toasts, dialogs and alerts.</Card.Description>
			</Card.Header>
			<Card.Content class="flex flex-col gap-4">
				<div class="flex flex-wrap gap-2">
					<Button variant="outline" onclick={() => toast.success('Saved successfully')}>Success toast</Button>
					<Button variant="outline" onclick={() => toast.error('Something went wrong')}>Error toast</Button>
					<Button variant="outline" onclick={() => toast.info('Here is some information')}>Info toast</Button>
					<Dialog.Root>
						<Dialog.Trigger class={buttonVariants({ variant: 'outline' })}>Open dialog</Dialog.Trigger>
						<Dialog.Content>
							<Dialog.Header>
								<Dialog.Title>Dialog title</Dialog.Title>
								<Dialog.Description>Dialogs trap focus and close on Escape.</Dialog.Description>
							</Dialog.Header>
							<Dialog.Footer>
								<Dialog.Close class={buttonVariants()}>Got it</Dialog.Close>
							</Dialog.Footer>
						</Dialog.Content>
					</Dialog.Root>
				</div>
				<Alert.Root>
					<InfoIcon />
					<Alert.Title>Heads up</Alert.Title>
					<Alert.Description>Alerts call attention to something important.</Alert.Description>
				</Alert.Root>
				<Alert.Root variant="destructive">
					<TriangleAlertIcon />
					<Alert.Title>Error</Alert.Title>
					<Alert.Description>Destructive alerts are for failures.</Alert.Description>
				</Alert.Root>
			</Card.Content>
		</Card.Root>

		<Card.Root class="lg:col-span-2">
			<Card.Header>
				<Card.Title>Tabs & Skeleton</Card.Title>
				<Card.Description>Switch between views; show placeholders while loading.</Card.Description>
			</Card.Header>
			<Card.Content>
				<Tabs.Root value="overview">
					<Tabs.List>
						<Tabs.Trigger value="overview">Overview</Tabs.Trigger>
						<Tabs.Trigger value="loading">Loading state</Tabs.Trigger>
					</Tabs.List>
					<Tabs.Content value="overview" class="pt-3">
						<div class="flex items-start gap-3 text-sm">
							<CircleCheckIcon class="mt-0.5 size-4 text-emerald-500" />
							<p class="text-muted-foreground">
								Every component lives in <code class="font-mono">src/lib/components/ui</code> as plain
								Svelte source — edit them freely.
							</p>
						</div>
					</Tabs.Content>
					<Tabs.Content value="loading" class="space-y-2 pt-3">
						<Skeleton class="h-4 w-3/4" />
						<Skeleton class="h-4 w-1/2" />
						<Skeleton class="h-4 w-2/3" />
					</Tabs.Content>
				</Tabs.Root>
			</Card.Content>
		</Card.Root>
	</div>
</div>
