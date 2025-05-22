<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { getCurrentWindow } from "@tauri-apps/api/window";
    import { onMount } from "svelte";
    import "./global.css";

    let appName: string = $state("");
    let apps: string[] = $state([]);
    let selectedIndex: number = $state(0);

    onMount(async () => {
        apps = await invoke<string[]>("list_apps");
    });

    function filteredApps() {
        return apps
            .filter((app) => app.toLowerCase().includes(appName.toLowerCase()))
            .slice(0, 5);
    }

    async function launchSelected(): Promise<void> {
        const current = filteredApps()[selectedIndex];
        if (current) {
            await invoke("launch_app", { appName: current });
            await getCurrentWindow().close();
        }
    }

    function handleKey(event: KeyboardEvent): void {
        const results = filteredApps();

        switch (event.key) {
            case "Tab":
                event.preventDefault();
                if (event.shiftKey) {
                    selectedIndex =
                        (selectedIndex - 1 + results.length) % results.length;
                } else {
                    selectedIndex = (selectedIndex + 1) % results.length;
                }
                break;
            case "ArrowDown":
                event.preventDefault();
                selectedIndex = (selectedIndex + 1) % results.length;
                break;
            case "ArrowUp":
                event.preventDefault();
                selectedIndex =
                    (selectedIndex - 1 + results.length) % results.length;
                break;
            case "Enter":
                event.preventDefault();
                launchSelected();
                break;
            case "Escape":
                getCurrentWindow().close();
                break;
        }
    }
</script>

<!-- svelte-ignore a11y_autofocus -->
<input
    type="text"
    id="app_name"
    bind:value={appName}
    autofocus
    onkeydown={handleKey}
/>

<ul id="app_list">
    {#each filteredApps() as app, i}
        <li class:selected={i === selectedIndex}>
            {app}
        </li>
    {/each}
</ul>

<style>
    #app_name {
        position: absolute;
        left: 50%;
        transform: translateX(-50%);
        width: 94%;
        height: 30px;
        margin-top: 0%;
        background-color: #1e1e2e;
        border: 1px solid #fab387;
        border-radius: 0.5em;
        color: #fab387;
        padding: 0 10px;
        font-family: "Hack", monospace;
    }

    #app_name::placeholder {
        color: #fab387;
        opacity: 0.5;
    }

    #app_list {
        position: absolute;
        top: 40px;
        left: 50%;
        transform: translateX(-50%);
        width: 96%;
        margin: 0;
        padding: 0;
        list-style: none;
        z-index: 10;
    }

    #app_list li {
        background-color: #1e1e2e;
        border: 1px solid #fab387;
        border-radius: 0.5em;
        margin-top: 0.77em;
        padding: 8px 10px;
        color: #fab387;
        font-family: "Hack", monospace;
        transition:
            background-color 0.2s ease,
            color 0.2s ease;
        pointer-events: none; /* disable mouse interaction */
    }

    #app_list li.selected {
        background-color: #fab387;
        color: #1e1e2e;
    }
</style>
