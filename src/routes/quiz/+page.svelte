<script lang="ts">
    import { getContext, onMount } from "svelte";
    import type { Quiz, Answer, Context } from "$lib/index";
    import { goto } from "$app/navigation";

    const context = getContext<Context>("quiz");

    const quiz = $state<Quiz>({
        questions: [],
        title: "",
    });
    let current = $state(0);
    let showExp = $state(false);
    let selected: Answer | null = $state(null);
    let score = $state(0);
    let finished = $state(false);
    let loading = $state(true);
    let question = $derived.by(() => quiz.questions[current]);
    let autoNextTimeout: ReturnType<typeof setTimeout> | null = null;

    async function saveQuizResult() {
        if (!context.db) {
            console.error(
                "Database connection not available. Cannot save result."
            );
            return;
        }

        const successPercentage = (score / quiz.questions.length) * 100;

        console.log(JSON.parse(JSON.stringify(quiz)));
        try {
            if (quiz.id) {
                // ID EXISTS: This is an existing quiz. Update it.
                console.log(`Updating quiz with ID: ${quiz.id}`);
                await context.db.execute(
                    "UPDATE quizzes SET success_percentage = $1 WHERE id = $2",
                    [successPercentage, quiz.id]
                );
            } else {
                // NO ID: This is a new quiz from JSON. Insert it.
                console.log("Saving new quiz.");
                await context.db.execute(
                    "INSERT INTO quizzes (title, questions_json, success_percentage) VALUES ($1, $2, $3)",
                    [
                        quiz.title,
                        JSON.stringify(quiz.questions),
                        successPercentage,
                    ]
                );
            }
        } catch (e) {
            console.error("Failed to save or update quiz result:", e);
        }
    }

    function select(ans: Answer) {
        if (showExp) return;
        selected = ans;
        showExp = true;
        if (ans.isCorrect) {
            score++;
            autoNextTimeout = setTimeout(() => next(), 5000);
        }
    }

    async function next() {
        if (autoNextTimeout) {
            clearTimeout(autoNextTimeout);
            autoNextTimeout = null;
        }
        if (current < quiz.questions.length - 1) {
            current++;
            reset();
        } else {
            await saveQuizResult();
            finished = true;
            context.quiz = {
                questions: [],
                title: "",
            };
        }
    }

    function back() {
        if (current > 0) {
            current--;
            reset();
        }
    }

    function reset() {
        showExp = false;
        selected = null;
    }

    onMount(() => {
        if (!context) {
            console.error("Quiz context not found");
            return;
        }
        quiz.questions = context.quiz.questions;
        quiz.title = context.quiz.title;
        quiz.createdAt = context.quiz.createdAt;
        quiz.id = context.quiz.id;
        quiz.successPercentage = context.quiz.successPercentage;
        loading = false;
    });
</script>

{#if loading}
    <div
        class="p-4 max-w-3xl mx-auto flex flex-col items-center justify-center min-h-[300px]">
        <svg
            class="animate-spin h-16 w-16 text-blue-500 mb-4"
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24">
            <circle
                class="opacity-25"
                cx="12"
                cy="12"
                r="10"
                stroke="currentColor"
                stroke-width="4"></circle>
            <path
                class="opacity-75"
                fill="currentColor"
                d="M4 12a8 8 0 018-8v4a4 4 0 00-4 4H4z"></path>
        </svg>
        <p>Loading...</p>
    </div>
{:else if !finished}
    <div class="p-4 max-w-3xl mx-auto flex flex-col gap-6">
        <div class="flex items-center justify-between">
            <h1 class="text-2xl font-bold">{quiz.title}</h1>
            <span class="text-lg font-semibold"
                >{current + 1}/{quiz.questions.length}</span>
        </div>
        <div>
            <p class="mb-4">
                <span class="font-semibold">Question {current + 1}:</span>
                {question.question}
            </p>
            <div class="flex flex-col gap-2">
                {#each question.answers as ans}
                    <button
                        onclick={() => select(ans)}
                        class="px-4 py-2 border rounded text-left {showExp &&
                        ans === selected
                            ? ans.isCorrect
                                ? 'bg-green-100'
                                : 'bg-red-100'
                            : 'bg-white hover:bg-gray-100'}">
                        {ans.text}
                    </button>
                {/each}
            </div>
            {#if showExp && selected}
                <div class="mt-4 p-4 bg-gray-100 rounded">
                    {selected.explanation}
                </div>
            {/if}
        </div>
        <div class="flex justify-end gap-4">
            <button
                onclick={back}
                disabled={current === 0}
                class="px-4 py-2 bg-gray-300 hover:bg-gray-400 rounded disabled:opacity-50">
                Back
            </button>
            <button
                onclick={next}
                disabled={!showExp}
                class="px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded disabled:opacity-50">
                {current === quiz.questions.length - 1 ? "Done" : "Continue"}
            </button>
        </div>
    </div>
{:else}
    <div class="p-8 flex flex-col items-center justify-center gap-4">
        <h2 class="text-4xl font-bold">
            Score: {score}/{quiz.questions.length}
        </h2>
        <h3 class="text-3xl">
            Accuracy: {Math.round((score / quiz.questions.length) * 100)}%
        </h3>
        <button
            onclick={async () => await goto("/")}
            class="mt-6 px-6 py-3 bg-blue-500 hover:bg-blue-600 text-white text-lg rounded">
            Go to Main Page
        </button>
    </div>
{/if}
