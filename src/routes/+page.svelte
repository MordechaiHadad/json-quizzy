<script lang="ts">
    import { goto } from "$app/navigation";
    import type { Context, Quiz } from "$lib";
    import { getContext, onMount } from "svelte";
    import FileUpload from "@tabler/icons-svelte/icons/file-upload";

    let context: Context = getContext<Context>("quiz");
    let quizzes: Quiz[] = $state([]);

    let jsonInput = $state("");

    async function startQuiz() {
        console.log("Starting quiz");
        try {
            context.quiz = JSON.parse(jsonInput);
            if (context.quiz.questions.length === 0) {
                console.error("No questions found in the quiz context");
                return;
            }
            await goto("/quiz");
        } catch (error) {
            console.error("Invalid JSON input", error);
            return;
        }
    }
    $effect(() => {
        if (context?.db) loadQuizzes();
    });

    async function loadQuizzes() {
        if (!context.db) {
            console.error("Database connection is not established.");
            return;
        }

        try {
            const result = await context.db.select<any[]>(
                "SELECT * FROM quizzes ORDER BY created_at DESC"
            );

            quizzes = result.map((q) => ({
                id: q.id,
                title: q.title,
                questions: JSON.parse(q.questions_json),
                successPercentage: q.success_percentage,
                createdAt: q.created_at,
            }));
        } catch (e) {
            console.error("Failed to load quizzes:", e);
        }
    }
</script>

<div class="flex @container flex-col items-center min-h-dvh p-8 bg-gray-50">
    <div class="flex flex-col items-center mb-10">
        <img src="/logo.svg" alt="JSONQuizzy Logo" class="size-16">
        <h1 class="text-2xl font-bold tracking-tight mt-2 mb-2">JSONQuizzy</h1>
    </div>
    <div
        class="flex flex-col @3xl:flex-row gap-8 justify-center items-center w-full">
        <div
            class="bg-white rounded-lg shadow-lg p-8 flex flex-col gap-4 w-full max-w-md">
            <h2 class="text-xl font-bold mb-2">Start a New Quiz</h2>
            <textarea
                bind:value={jsonInput}
                class="w-full h-40 p-2 border rounded"
                placeholder="Enter JSON here"></textarea>
            <div class="flex flex-row gap-2 items-center">
                <button
                    onclick={startQuiz}
                    class="px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white font-semibold rounded flex-1">
                    Start Quiz
                </button>
                <button
                    class="px-4 py-2 bg-gray-200 hover:bg-gray-300 text-gray-700 rounded flex items-center gap-2 border border-gray-300">
                    <FileUpload class="size-5" />
                    <span>Load JSON</span>
                </button>
            </div>
        </div>
        <div
            class="bg-white rounded-lg shadow-lg p-8 flex flex-col gap-4 w-full max-w-md min-h-[340px] max-h-[340px]">
            <h2 class="text-xl font-bold mb-2">Recent Quizzes</h2>
            <ul class="flex flex-col gap-3 overflow-y-auto max-h-64">
                {#if quizzes.length === 0}
                    <li class="p-3 bg-gray-100 rounded text-gray-500 italic">
                        No recent quizzes
                    </li>
                {:else}
                    {#each quizzes as quiz}
                        <li
                            class="bg-gray-50 rounded p-3 flex flex-col gap-2 border border-gray-100 shadow-sm">
                            <span class="font-semibold text-base"
                                >{quiz.title}</span>
                            <div class="flex items-center w-full justify-between gap-2 mt-1">
                                <div class="flex gap-0.5 w-full place-items-center">
                                <div
                                    class="flex-1 max-w-1/2 h-2 bg-gray-200 rounded overflow-hidden">
                                    <div
                                        class="h-2 bg-blue-400"
                                        style={`width: ${Math.round(quiz.successPercentage ?? 0)}%`}>
                                    </div>
                                </div>
                                <span
                                    class="ml-2 text-xs text-gray-600 w-10 text-right"
                                    >{Math.round(
                                        quiz.successPercentage ?? 0
                                    )}%</span>

                                </div>
                                <button
                                    class="ml-4 px-3 py-1 bg-blue-500 hover:bg-blue-600 text-white text-sm rounded"
                                    onclick={async () => {
                                        context.quiz = quiz;
                                        await goto('/quiz');
                                    }}>
                                    Start
                                </button>
                            </div>
                        </li>
                    {/each}
                {/if}
            </ul>
        </div>
    </div>
</div>
