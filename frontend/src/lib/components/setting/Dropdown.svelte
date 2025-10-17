<script lang="ts">
    import InlineSetting from "./InlineSetting.svelte";

    let { title, description, options, onChange } = $props();

    let listContainer: HTMLUListElement | null = $state(null);

    let selected = $state("");
    let filteredOptions = $state<string[]>([]);
    let showSuggestions = $state(false);
    let hoveredIndex = $state(0);

    function handleInput(event: Event) {
        const input = (event.target as HTMLInputElement).value;
        selected = input;

        filteredOptions = options.map((option: any) => option.name).filter((option: string) =>
            option.toLowerCase().includes(input.toLowerCase())
        );

        filteredOptions.sort((a, b) => {
            if (input.toLowerCase() == a.toLowerCase()) return -1;
            if (input.toLowerCase() == b.toLowerCase()) return 1;

            const optionA = options.find((option: any) => option.name === a);
            const optionB = options.find((option: any) => option.name === b);

            return (optionB?.priority ?? 0) - (optionA?.priority ?? 0);
        });

        showSuggestions = filteredOptions.length > 0;

        hoveredIndex = 0;
    }

    function selectOption(option: string) {
        selected = option;
        showSuggestions = false;

        onChange?.(option);
    }

    function handleBlur() {
        setTimeout(() => showSuggestions = false, 100);

        if (!filteredOptions.includes(selected)) {
            selected = "";
            onChange?.("");
        }
    }

    function fillOptions() {
        filteredOptions = options.map((option: any) => option.name);
        if (selected !== "") {
            filteredOptions = filteredOptions.filter((option: string) =>
                option.toLowerCase().includes(selected.toLowerCase())
            );
        }

        filteredOptions.sort((a, b) => {
            const optionA = options.find((option: any) => option.name === a);
            const optionB = options.find((option: any) => option.name === b);

            return (optionB?.priority ?? 0) - (optionA?.priority ?? 0);
        });

        showSuggestions = true;
    }
</script>

<InlineSetting {title} {description}>
    <div id="autocomplete-container">
        <input
            type="text"
            bind:value={selected}
            oninput={handleInput}
            onfocus={() => {
                fillOptions();
                showSuggestions = true
            }}
            onblur={handleBlur}
            onkeydown={(event: KeyboardEvent) => {
                if (event.key === "Enter" && filteredOptions.length > 0) {
                    selectOption(filteredOptions[hoveredIndex]);
                    (event.target as HTMLInputElement).blur();
                } else if (event.key === "ArrowDown") {
                    hoveredIndex = Math.min(hoveredIndex + 1, filteredOptions.length - 1);
                    event.preventDefault();

                    if (listContainer) {
                        const hoveredElement = listContainer.children[hoveredIndex] as HTMLElement;
                        if (hoveredElement) {
                            hoveredElement.scrollIntoView({ block: "nearest", behavior: "smooth" });
                        }
                    }

                } else if (event.key === "ArrowUp") {
                    hoveredIndex = Math.max(hoveredIndex - 1, 0);
                    event.preventDefault();
                }
            }}
            placeholder="Search..."
            autocomplete="off"
            autocorrect="off"
            autocapitalize="off"
            spellcheck="false"
        />
        {#if showSuggestions}
        <div id="suggestions-container" class={showSuggestions ? "visible" : "hidden"}>
            <ul class="suggestions" bind:this={listContainer}>
                {#each filteredOptions as option, index}
                    <li onmousedown={() => selectOption(option)} onmouseover={() => {hoveredIndex = index}} class="hovered-{hoveredIndex == index}">{option}</li>
                {/each}
            </ul>
        </div>
        {/if}
    </div>
</InlineSetting>

<style lang="scss">
    #autocomplete-container {
        position: relative;
        width: 150px;
    }

    input {
        height: 40px;
        padding: 0 14px;
        width: calc(100% - 28px);
        line-height: 1;
        border-radius: 10px;
        border: none;
        color: var(--color-primary);
        font-family: var(--font-family), sans-serif;
        font-size: 0.9rem;
        font-weight: 500;
        background-color: var(--color-background);
        z-index: 10;

        &:hover {
            cursor: text;
        }

        &:focus {
            outline: none;
        }
    }

    #suggestions-container {
        position: absolute;
        top: 100%;
        left: 0;
        right: 0;
        z-index: 1;
        transition: opacity 0.3s ease-in-out, background-color 0.3s ease-in-out;

        &.hidden {
            opacity: 0 !important;
            display: none;
        }

        &.visible {
            opacity: 1 !important;
        }
    }

    .suggestions {
        background-color: var(--color-background);
        border-radius: 8px;
        margin-top: .5rem;
        padding: 0;
        list-style: none;
        box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
        max-height: 125px;
        overflow-y: auto;
        z-index: 1;
    }

    .suggestions li {
        height: 40px;
        display: flex;
        align-items: center;
        padding: 0 14px;
        cursor: pointer;
        color: var(--color-primary);
        font-family: var(--font-family);
        font-size: 0.9rem;
        font-weight: 500;

        transition: filter 0.1s ease-in-out, color 0.1s ease-in-out, background-color 0.1s ease-in-out;
    }

    .suggestions .hovered-true {
        background-color: var(--color-primary);
        color: var(--color-background);
    }
</style>
