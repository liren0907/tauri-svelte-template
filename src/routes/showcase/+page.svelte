<script lang="ts">
    import { Button, Card, Badge, Alert, IconButton, TextInput, ToggleButtonGroup } from "$lib/components/ui";
    import { showSuccess, showError, showToast } from "$lib/stores/toastStore.svelte";

    let textValue: string = $state("");
    let toggleValue: string = $state("option1");
</script>

<svelte:head>
    <title>UI Showcase — Tauri Svelte Template</title>
</svelte:head>

<div class="container mx-auto max-w-4xl space-y-8">
    <div>
        <h1 class="text-2xl font-bold text-base-content mb-1">UI Components</h1>
        <p class="text-sm text-base-content/60">Pre-built DaisyUI wrapper components included in this template.</p>
    </div>

    <!-- Buttons -->
    <Card variant="base-200" shadow="sm" icon="smart_button" title="Buttons">
        <div class="flex flex-wrap gap-2">
            <Button>Default</Button>
            <Button variant="ghost">Ghost</Button>
            <Button size="sm">Small</Button>
            <Button size="md">Medium</Button>
            <Button disabled={true}>Disabled</Button>
        </div>
    </Card>

    <!-- Badges -->
    <Card variant="base-200" shadow="sm" icon="label" title="Badges">
        <div class="flex flex-wrap gap-2">
            <Badge variant="primary">Primary</Badge>
            <Badge variant="secondary">Secondary</Badge>
            <Badge variant="accent">Accent</Badge>
            <Badge variant="success">Success</Badge>
            <Badge variant="warning">Warning</Badge>
            <Badge variant="info">Info</Badge>
            <Badge variant="ghost">Ghost</Badge>
        </div>
    </Card>

    <!-- Alerts -->
    <Card variant="base-200" shadow="sm" icon="info" title="Alerts">
        <div class="space-y-2">
            <Alert variant="info">This is an informational alert.</Alert>
            <Alert variant="success">This is a success alert.</Alert>
            <Alert variant="warning">This is a warning alert.</Alert>
            <Alert variant="error">This is an error alert.</Alert>
        </div>
    </Card>

    <!-- Toast -->
    <Card variant="base-200" shadow="sm" icon="notifications" title="Toast Notifications">
        <p class="text-sm text-base-content/60 mb-3">Trigger toast notifications from anywhere in the app.</p>
        <div class="flex flex-wrap gap-2">
            <Button onclick={() => showSuccess("Operation completed!")}>Success</Button>
            <Button onclick={() => showError("Something went wrong!")}>Error</Button>
            <Button onclick={() => showToast("Here is some information.", "info")}>Info</Button>
            <Button onclick={() => showToast("Careful with this action.", "warning")}>Warning</Button>
        </div>
    </Card>

    <!-- Inputs -->
    <Card variant="base-200" shadow="sm" icon="text_fields" title="Inputs">
        <div class="space-y-3 max-w-sm">
            <TextInput placeholder="Type something..." bind:value={textValue} />
            {#if textValue}
                <p class="text-xs text-base-content/50">Value: {textValue}</p>
            {/if}
        </div>
    </Card>

    <!-- Icon Buttons & Toggle Group -->
    <Card variant="base-200" shadow="sm" icon="toggle_on" title="Icon Buttons & Toggle Group">
        <div class="space-y-4">
            <div>
                <p class="text-xs font-semibold text-base-content/50 uppercase tracking-wider mb-2">Icon Buttons</p>
                <div class="flex gap-2">
                    <IconButton icon="home" tooltip="Home" />
                    <IconButton icon="settings" tooltip="Settings" />
                    <IconButton icon="delete" tooltip="Delete" />
                    <IconButton icon="favorite" tooltip="Favorite" />
                </div>
            </div>
            <div>
                <p class="text-xs font-semibold text-base-content/50 uppercase tracking-wider mb-2">Toggle Button Group</p>
                <ToggleButtonGroup
                    options={[
                        { value: "option1", label: "Option 1" },
                        { value: "option2", label: "Option 2" },
                        { value: "option3", label: "Option 3" },
                    ]}
                    bind:value={toggleValue}
                />
                <p class="text-xs text-base-content/50 mt-1">Selected: {toggleValue}</p>
            </div>
        </div>
    </Card>
</div>
