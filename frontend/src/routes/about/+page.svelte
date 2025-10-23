<script lang="ts">
    import Navbar from "$lib/components/Navbar.svelte";

    let { data }: { data: any } = $props();
    const commitHash = import.meta.env.VITE_COMMIT_HASH || 'unknown';
</script>

<div id="fixed">
    <Navbar />
</div>
<div id="about">
    <h1>{data.title}</h1>
    <p id="description">
      {@html data.description}
    </p>
</div>

<p id="footer">
    <a href="https://github.com/thesamgordon/PasteBook" target="_blank" aria-label="button">pastebook</a>@<a href="https://github.com/thesamgordon/PasteBook/commit/{commitHash}" target="_blank" aria-label="button">{commitHash}</a>
</p>

<style lang="scss">
    #fixed {
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        z-index: 1000;
    }

    #about {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        height: 100vh;
        background-color: var(--color-background);
        color: var(--color-primary);
        font-family: var(--font-family);
    }

    h1 {
        font-size: 10vw;
        font-weight: 800;
        margin: 0;
    }

    #description {
        font-size: 1.5vw;
        margin: 0.5rem 0;
        text-align: center;
        color: var(--color-text-secondary);
        font-weight: 200;
        margin: 0;

        @media (max-width: 650px) {
            font-size: 15px;
            width: 80%;
        }
    }

    #footer {
        position: fixed;
        bottom: 0;
        left: 0;
        right: 0;
        text-align: center;
        font-size: .8rem;
        color: var(--color-secondary);
        font-family: var(--font-family-mono);
        font-weight: 100;
        margin: 0.5rem 0;

        @media (max-width: 650px) {
            font-size: 10px;
        }
    }

    a {
        color: var(--color-secondary);
    }
</style>
