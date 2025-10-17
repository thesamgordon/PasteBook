import { error } from '@sveltejs/kit';

export async function load({ }) {
    if (process.env.DISABLE_NEW?.toLowerCase() === 'true') {
        error(403, {
            code: 403,
            message: 'New posts are disabled',
        } satisfies App.Error);
    }

    let loadedLanguages = [
        { name: "Batch", priority: 6 },
        { name: "C", priority: 6 },
        { name: "C#", priority: 7 },
        { name: "C++", priority: 7 },
        { name: "Dockerfile", priority: 6 },
        { name: "Dotenv", priority: 5 },
        { name: "CSS", priority: 9 },
        { name: "F#", priority: 5 },
        { name: "Go", priority: 7 },
        { name: "HTML", priority: 10 },
        { name: "Haskell", priority: 5 },
        { name: "Java", priority: 8 },
        { name: "JavaScript", priority: 10 },
        { name: "JSX", priority: 8 },
        { name: "Kotlin", priority: 6 },
        { name: "Lua", priority: 4 },
        { name: "Markdown", priority: 6 },
        { name: "PHP", priority: 5 },
        { name: "Python", priority: 10 },
        { name: "R", priority: 5 },
        { name: "Rust", priority: 7 },
        { name: "Ruby", priority: 5 },
        { name: "Sass", priority: 6 },
        { name: "SCSS", priority: 6 },
        { name: "Shell", priority: 6 },
        { name: "Svelte", priority: 6 },
        { name: "Swift", priority: 7 },
        { name: "SQL", priority: 9 },
        { name: "TSX", priority: 8 },
        { name: "TypeScript", priority: 9 },
        { name: "Vue", priority: 7 },
        { name: "XML", priority: 5 },
        { name: "YAML", priority: 6 },
        { name: "JSON", priority: 9 },
        { name: "Handlebars", priority: 4 },
        { name: "Less", priority: 4 },
        { name: "Log", priority: 5 },
        { name: "Zig", priority: 3 }
    ]

    return {
        loadedLanguages: loadedLanguages,
    }
}