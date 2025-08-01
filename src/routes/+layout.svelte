<script lang="ts">
    import type { Context } from "$lib";
    import { onMount, setContext } from "svelte";
    let { children } = $props();
    import "../app.css";
    import Database from "@tauri-apps/plugin-sql";
    import { invoke } from "@tauri-apps/api/core";
    import { GoogleGenAI } from "@google/genai";

    let context: Context = $state({
        quiz: {
            questions: [],
            title: "",
        },
        db: undefined,
        gemini: undefined

    });

    setContext("quiz", context);

    onMount(async () => {
        let database = await Database.load("sqlite:jsonquizzy.db");
        context.db = database;
        const geminiKey: string = await invoke("get_env_var", { name: "GEMINI_KEY"});
        if (!geminiKey) return;
        const genAi = new GoogleGenAI({
            apiKey: geminiKey,
        });
        context.gemini = genAi;
    });
</script>

{@render children()}
