<script lang="ts">
	import NotebookTextIcon from '@lucide/svelte/icons/notebook-text';
	import PencilIcon from '@lucide/svelte/icons/pencil';
	import PlusIcon from '@lucide/svelte/icons/plus';
	import RefreshCwIcon from '@lucide/svelte/icons/refresh-cw';
	import ServerOffIcon from '@lucide/svelte/icons/server-off';
	import Trash2Icon from '@lucide/svelte/icons/trash-2';
	import { onMount } from 'svelte';
	import { toast } from 'svelte-sonner';
	import { ApiError } from '#lib/api/client.js';
	import { notesApi } from '#lib/api/notes.js';
	import type { Note } from '#lib/api/types.js';
	import * as Alert from '#lib/components/ui/alert/index.js';
	import * as AlertDialog from '#lib/components/ui/alert-dialog/index.js';
	import { Button } from '#lib/components/ui/button/index.js';
	import * as Card from '#lib/components/ui/card/index.js';
	import * as Dialog from '#lib/components/ui/dialog/index.js';
	import { Input } from '#lib/components/ui/input/index.js';
	import { Label } from '#lib/components/ui/label/index.js';
	import { Skeleton } from '#lib/components/ui/skeleton/index.js';
	import * as Table from '#lib/components/ui/table/index.js';
	import { Textarea } from '#lib/components/ui/textarea/index.js';

	// --- List -------------------------------------------------------------
	let notes: Note[] = $state([]);
	let loading = $state(true);
	let loadError: string | null = $state(null);

	async function loadNotes() {
		loading = true;
		loadError = null;
		try {
			notes = await notesApi.list();
		} catch (error) {
			loadError = errorMessage(error);
		} finally {
			loading = false;
		}
	}

	onMount(loadNotes);

	// --- Create / edit dialog -------------------------------------------
	let editorOpen = $state(false);
	let editing: Note | null = $state(null);
	let title = $state('');
	let content = $state('');
	let saving = $state(false);
	let formError: string | null = $state(null);

	function openEditor(note: Note | null) {
		editing = note;
		title = note?.title ?? '';
		content = note?.content ?? '';
		formError = null;
		editorOpen = true;
	}

	async function saveNote(event: SubmitEvent) {
		event.preventDefault();
		saving = true;
		formError = null;
		try {
			const input = { title, content };
			if (editing) {
				const updated = await notesApi.update(editing.id, input);
				notes = notes.map((n) => (n.id === updated.id ? updated : n));
				toast.success('Note updated');
			} else {
				const created = await notesApi.create(input);
				notes = [created, ...notes];
				toast.success('Note created');
			}
			editorOpen = false;
		} catch (error) {
			// Validation errors (422) are shown next to the form; anything else as a toast.
			if (error instanceof ApiError && error.code === 'validation_error') {
				formError = error.message;
			} else {
				toast.error(errorMessage(error));
			}
		} finally {
			saving = false;
		}
	}

	// --- Delete confirmation ----------------------------------------------
	let deleteTarget: Note | null = $state(null);
	let deleting = $state(false);

	async function confirmDelete() {
		if (!deleteTarget) return;
		deleting = true;
		try {
			await notesApi.remove(deleteTarget.id);
			notes = notes.filter((n) => n.id !== deleteTarget?.id);
			toast.success('Note deleted');
			deleteTarget = null;
		} catch (error) {
			toast.error(errorMessage(error));
		} finally {
			deleting = false;
		}
	}

	// --- Helpers ----------------------------------------------------------
	const dateFormat = new Intl.DateTimeFormat(undefined, { dateStyle: 'medium', timeStyle: 'short' });

	function formatDate(value: string) {
		return dateFormat.format(new Date(value));
	}

	function errorMessage(error: unknown) {
		return error instanceof Error ? error.message : String(error);
	}
</script>

<div class="mx-auto flex max-w-5xl flex-col gap-6">
	<div class="flex flex-wrap items-end justify-between gap-4">
		<div class="space-y-1">
			<h1 class="text-2xl font-semibold tracking-tight">Notes</h1>
			<p class="text-sm text-muted-foreground">
				Stored by the Axum backend and loaded over HTTP —
				<code class="rounded bg-muted px-1 py-0.5 text-xs">GET / POST / PUT / DELETE /api/notes</code>
			</p>
		</div>
		<div class="flex gap-2">
			<Button variant="outline" onclick={loadNotes} disabled={loading}>
				<RefreshCwIcon class={loading ? 'animate-spin' : ''} />
				Refresh
			</Button>
			<Button onclick={() => openEditor(null)} disabled={loadError !== null}>
				<PlusIcon />
				New note
			</Button>
		</div>
	</div>

	{#if loadError}
		<Alert.Root variant="destructive">
			<ServerOffIcon />
			<Alert.Title>Cannot load notes</Alert.Title>
			<Alert.Description>
				<p>{loadError}</p>
				<p>
					Start the API with <code class="font-mono">cargo run</code> in
					<code class="font-mono">backend/</code>, then retry.
				</p>
			</Alert.Description>
		</Alert.Root>
		<div>
			<Button variant="outline" onclick={loadNotes}>
				<RefreshCwIcon />
				Retry
			</Button>
		</div>
	{:else if loading && notes.length === 0}
		<Card.Root>
			<Card.Content class="space-y-3">
				{#each [0, 1, 2] as row (row)}
					<div class="flex items-center gap-4">
						<Skeleton class="h-5 w-1/3" />
						<Skeleton class="h-5 flex-1" />
						<Skeleton class="h-5 w-24" />
					</div>
				{/each}
			</Card.Content>
		</Card.Root>
	{:else if notes.length === 0}
		<Card.Root>
			<Card.Content class="flex flex-col items-center gap-3 py-10 text-center">
				<NotebookTextIcon class="size-10 text-muted-foreground" />
				<div class="space-y-1">
					<p class="font-medium">No notes yet</p>
					<p class="text-sm text-muted-foreground">Create your first note to see the API in action.</p>
				</div>
				<Button onclick={() => openEditor(null)}>
					<PlusIcon />
					New note
				</Button>
			</Card.Content>
		</Card.Root>
	{:else}
		<Card.Root class="py-0">
			<Table.Root>
				<Table.Header>
					<Table.Row>
						<Table.Head class="w-[30%] pl-4">Title</Table.Head>
						<Table.Head>Content</Table.Head>
						<Table.Head class="w-44">Updated</Table.Head>
						<Table.Head class="w-24 pr-4 text-right">Actions</Table.Head>
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#each notes as note (note.id)}
						<Table.Row>
							<Table.Cell class="pl-4 font-medium">{note.title}</Table.Cell>
							<Table.Cell class="max-w-0 truncate text-muted-foreground">
								{note.content || '—'}
							</Table.Cell>
							<Table.Cell class="text-muted-foreground">{formatDate(note.updatedAt)}</Table.Cell>
							<Table.Cell class="pr-4 text-right">
								<Button
									variant="ghost"
									size="icon-sm"
									aria-label="Edit {note.title}"
									onclick={() => openEditor(note)}
								>
									<PencilIcon />
								</Button>
								<Button
									variant="ghost"
									size="icon-sm"
									class="text-destructive hover:text-destructive"
									aria-label="Delete {note.title}"
									onclick={() => (deleteTarget = note)}
								>
									<Trash2Icon />
								</Button>
							</Table.Cell>
						</Table.Row>
					{/each}
				</Table.Body>
			</Table.Root>
		</Card.Root>
	{/if}
</div>

<Dialog.Root bind:open={editorOpen}>
	<Dialog.Content class="sm:max-w-lg">
		<form class="grid gap-4" onsubmit={saveNote}>
			<Dialog.Header>
				<Dialog.Title>{editing ? 'Edit note' : 'New note'}</Dialog.Title>
				<Dialog.Description>
					{editing ? 'Changes are sent with PUT /api/notes/{id}.' : 'The note is sent with POST /api/notes.'}
				</Dialog.Description>
			</Dialog.Header>
			<div class="grid gap-2">
				<Label for="note-title">Title</Label>
				<Input
					id="note-title"
					bind:value={title}
					placeholder="What is this note about?"
					aria-invalid={formError !== null}
				/>
				{#if formError}
					<p class="text-sm text-destructive">{formError}</p>
				{/if}
			</div>
			<div class="grid gap-2">
				<Label for="note-content">Content</Label>
				<Textarea id="note-content" bind:value={content} rows={5} placeholder="Write something…" />
			</div>
			<Dialog.Footer>
				<Button type="button" variant="outline" onclick={() => (editorOpen = false)}>Cancel</Button>
				<Button type="submit" disabled={saving}>{saving ? 'Saving…' : 'Save'}</Button>
			</Dialog.Footer>
		</form>
	</Dialog.Content>
</Dialog.Root>

<AlertDialog.Root
	open={deleteTarget !== null}
	onOpenChange={(open) => {
		if (!open) deleteTarget = null;
	}}
>
	<AlertDialog.Content>
		<AlertDialog.Header>
			<AlertDialog.Title>Delete this note?</AlertDialog.Title>
			<AlertDialog.Description>
				“{deleteTarget?.title}” will be removed from the backend. This cannot be undone.
			</AlertDialog.Description>
		</AlertDialog.Header>
		<AlertDialog.Footer>
			<AlertDialog.Cancel disabled={deleting}>Cancel</AlertDialog.Cancel>
			<AlertDialog.Action variant="destructive" disabled={deleting} onclick={confirmDelete}>
				{deleting ? 'Deleting…' : 'Delete'}
			</AlertDialog.Action>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>
