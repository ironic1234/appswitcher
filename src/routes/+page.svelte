<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { getCurrentWindow } from "@tauri-apps/api/window";

    let appName = $state("");

    async function open_app(event: Event) {
        event.preventDefault();
        invoke("launch_app", { appName: appName }).then(() => getCurrentWindow().close());
    }
</script>

<form onsubmit={open_app}>
    <input type="text" id="app_name" bind:value={appName} />
    <button type="submit">Open App</button>
</form>
