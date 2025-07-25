<script lang="ts">
    import type { Context } from "$lib";
    import { onMount, setContext } from "svelte";
    let { children } = $props();
    import "../app.css";
    import Database from "@tauri-apps/plugin-sql";


    let context: Context = $state({
        quiz: {
            questions: [],
            title: ""
        },
        db: null
    });

    setContext("quiz", context);

    onMount(async () => {
        let database = await Database.load("sqlite:jsonquizzy.db");
        context.db = database;
    });
</script>

{@render children()}
