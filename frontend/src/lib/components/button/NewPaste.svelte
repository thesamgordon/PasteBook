<div id="new-paste">
    <a class="new-paste-button" href="/">
        <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path
                d="M4 12H20M12 4V20"
                stroke="#000000"
                stroke-width="4"
                stroke-linecap="round"
                stroke-linejoin="round"
            />
        </svg>
        New Paste
    </a>
</div>

<style lang="scss">
    #new-paste {
        display: flex;
        justify-content: center;
        align-items: center;
        height: 45px;
        width: 100%;
        background-color: var(--color-primary);
        border-radius: 15px;

        transition:
            background-color 0.3s ease,
            transform 0.2s ease,
            filter 0.2s ease;

        @media (max-width: 650px) {
            width: 100%;
            height: 38px;
        }
    }

    svg {
        width: 0.75rem;
        height: 0.75rem;
    }

    .new-paste-button {
        background-color: var(--color-button);
        color: var(--color-text);
        border: none;
        cursor: pointer;
        height: 2rem;
        font-size: 1.05rem;
        padding: 0.5rem 1.2rem;
        font-family: var(--font-family);
        font-weight: 600;
        display: flex;
        align-items: center;
        gap: 0.8rem;
        border-radius: 15px;
        text-decoration: none;

        @media (max-width: 650px) {
            font-size: 0.9rem;
            padding: 0.3rem 1rem;
            height: 35px;
            border-radius: 10px;
            gap: 0.3rem;
        }
    }

    #new-paste:hover {
        filter: brightness(0.9);
    }

    #new-paste:active {
        transform: scale(0.98);
    }
</style>
