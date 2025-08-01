<script lang="ts">
    import { getContext, onMount } from "svelte";
    import type { Quiz, Answer, Context, GeminiResponse } from "$lib/index";
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
    let startTime = $state<number | null>(null);
    let elapsed = $state(0);
    let wrongCollector = $state<
        Array<{ question: string; chosen: string; correct: string }>
    >([]);
    let geminiExplanations = $state<
        Array<{ question: string; answer: string; explanation: string }>
    >([]);
    let geminiLoading = $state(false);

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
        } else {
            const correctAns = question.answers.find((a) => a.isCorrect);
            wrongCollector.push({
                question: question.question,
                chosen: ans.text,
                correct: correctAns ? correctAns.text : "",
            });
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

            if (startTime)
                elapsed = Math.floor((Date.now() - startTime) / 1000);
            finished = true;
            geminiLoading = true;
            let geminiResp = await context.gemini?.models.generateContent({
                model: "gemini-2.5-flash",
                contents: `Here are a few I answered incorrectly please provide corrections for these questions, a few rules for you to follow:
1. Provide a concise answer.
2. Return the answer in this JSON format: { "question": "The question", "answer": "The answer", "explanation": "Your generated explanation" }.

The questions I answered incorrectly are as follows:\n\n${JSON.stringify(wrongCollector, null, 2)}`,
            });
            const modifiedText = geminiResp?.text
                ?.replaceAll("json", "")
                .replaceAll("`", "");
            let parsed: Array<{
                question: string;
                answer: string;
                explanation: string;
            }> = [];
            try {
                parsed = JSON.parse(modifiedText ?? "[]");
            } catch (e) {
                console.error(
                    "Failed to parse Gemini response:",
                    modifiedText,
                    e
                );
            }
            geminiExplanations = parsed;
            geminiLoading = false;
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
        startTime = Date.now();
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
                        class="px-4 py-2 border rounded text-left
                        {showExp && ans === selected
                            ? ans.isCorrect
                                ? 'bg-green-100'
                                : 'bg-red-100'
                            : showExp && !selected?.isCorrect && ans.isCorrect
                              ? 'border-2 border-green-500'
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
    <div class="p-8 flex flex-col items-center justify-center gap-4 @container">
        <h2 class="text-4xl font-bold">
            Score: {score}/{quiz.questions.length}
        </h2>
        <h3 class="text-3xl">
            Accuracy: {Math.round((score / quiz.questions.length) * 100)}%
        </h3>
        <div class="text-xl font-semibold text-gray-700">
            Time: {Math.floor(elapsed / 60)}:{(elapsed % 60)
                .toString()
                .padStart(2, "0")}
        </div>
        {#if geminiLoading}
            <div class="flex flex-col items-center justify-center">
                <svg
                    class="animate-spin h-12 w-12 text-blue-500 mb-4"
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    ><circle
                        class="opacity-25"
                        cx="12"
                        cy="12"
                        r="10"
                        stroke="currentColor"
                        stroke-width="4"></circle
                    ><path
                        class="opacity-75"
                        fill="currentColor"
                        d="M4 12a8 8 0 018-8v4a4 4 0 00-4 4H4z"></path
                    ></svg>
                <div class="text-lg">Generating explanations...</div>
            </div>
        {:else if geminiExplanations.length > 0}
            <div class="w-full max-w-2xl">
                <h4 class="text-2xl font-bold mb-4 text-center @3xl:text-left">
                    Corrections & Explanations
                </h4>
                <ul class="space-y-4 overflow-y-auto max-h-96 @3xl:max-h-[40rem]">
                    {#each geminiExplanations as item}
                        <li class="p-4 bg-gray-100 rounded">
                            <div class="font-semibold">Q: {item.question}</div>
                            <div class="text-blue-700">A: {item.answer}</div>
                            <div class="mt-2 text-gray-700">
                                {item.explanation}
                            </div>
                        </li>
                    {/each}
                </ul>
            </div>
        {/if}
        <button
            onclick={async () => await goto("/")}
            class="mt-6 px-6 py-3 bg-blue-500 hover:bg-blue-600 text-white text-lg rounded">
            Go to Main Page
        </button>
    </div>
{/if}
