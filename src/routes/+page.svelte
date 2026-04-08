<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { safeConvertFileSrc } from "$lib/utils/tauriUtils";
    import { Button, Card } from "$lib/components/ui";
    import { showSuccess, showError } from "$lib/stores/toastStore.svelte";

    let filePath: string = $state("");
    let fileName: string = $state("");
    let fileUrl: string = $state("");
    let fileType: "image" | "video" | "" = $state("");
    let isLoading: boolean = $state(false);

    const IMAGE_EXTENSIONS = ["png", "jpg", "jpeg", "gif", "webp", "bmp", "svg", "ico", "tiff"];
    const VIDEO_EXTENSIONS = ["mp4", "webm", "mov", "avi", "mkv"];

    function getFileType(path: string): "image" | "video" | "" {
        const ext = path.split(".").pop()?.toLowerCase() ?? "";
        if (IMAGE_EXTENSIONS.includes(ext)) return "image";
        if (VIDEO_EXTENSIONS.includes(ext)) return "video";
        return "";
    }

    async function openFile() {
        isLoading = true;
        try {
            const path = await invoke<string>("open_file_dialog");
            const type = getFileType(path);
            if (!type) {
                showError("Unsupported file type");
                return;
            }
            filePath = path;
            fileName = path.split("/").pop() ?? path.split("\\").pop() ?? path;
            fileType = type;
            fileUrl = safeConvertFileSrc(path);
            showSuccess(`${fileType === "image" ? "Image" : "Video"} loaded`);
        } catch (e) {
            const msg = e instanceof Error ? e.message : String(e);
            if (msg !== "No file selected") {
                showError(`Failed to open file: ${msg}`);
            }
        } finally {
            isLoading = false;
        }
    }
</script>

<svelte:head>
    <title>Media Viewer — Tauri Svelte Template</title>
</svelte:head>

<div class="flex flex-col h-full gap-4">
    <!-- Toolbar -->
    <div class="flex items-center gap-3">
        <Button onclick={openFile} disabled={isLoading}>
            <span class="material-symbols-rounded text-[18px] mr-1">folder_open</span>
            {isLoading ? "Opening..." : "Open File"}
        </Button>
        {#if fileName}
            <span class="text-sm text-base-content/50 font-mono truncate">{filePath}</span>
        {/if}
    </div>

    <!-- Media Display -->
    <div class="flex-1 flex items-center justify-center bg-base-200 rounded-xl border border-base-300 min-h-[400px] overflow-hidden">
        {#if fileType === "image" && fileUrl}
            <img
                src={fileUrl}
                alt={fileName}
                class="max-w-full max-h-full object-contain"
            />
        {:else if fileType === "video" && fileUrl}
            <!-- svelte-ignore a11y_media_has_caption -->
            <video
                src={fileUrl}
                controls
                class="max-w-full max-h-full"
            >
            </video>
        {:else}
            <div class="flex flex-col items-center gap-3 text-base-content/30">
                <span class="material-symbols-rounded text-[64px]">image</span>
                <p class="text-sm">Open an image or video to view</p>
            </div>
        {/if}
    </div>
</div>
