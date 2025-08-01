import type { GoogleGenAI } from "@google/genai";
import type Database from "@tauri-apps/plugin-sql";

export type Answer = {
    text: string;
    isCorrect: boolean;
    explanation: string;
};

export type Question = {
    question: string;
    answers: Answer[];
    hint: string;
};

export type Quiz = {
    title: string;
    questions: Question[];
    successPercentage?: number;
    createdAt?: string;
    id?: number;
};


export type Context = {
    quiz: Quiz;
    db?: Database;
    gemini?: GoogleGenAI;
}

export type GeminiResponse = {
    question: string;
    answer: string;
    explanation: string;
}