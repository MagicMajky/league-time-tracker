<script>
    import { invoke } from '@tauri-apps/api/tauri'
    function printProccessInfo(){
        invoke("print_process_info")
    }
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>
<button on:click={printProccessInfo}>Refresh</button>
