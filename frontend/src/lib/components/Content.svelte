<script lang="ts">
	import { pushState } from "$app/navigation";
	import { wrap, writableContent } from "$lib/stores";
	import { onMount } from "svelte";

	let textArea = $state<HTMLTextAreaElement | null>(null);
	let contentContainer = $state<HTMLElement | null>(null);

	let { content = "", tokenLines = null, newPaste = false } = $props();
	let length = $state(0);

	function padIndex(index: number): string {
		let useLength = length + 1;

		let padding = Math.ceil(Math.log10(useLength));
		return index.toString().padStart(padding, " ");
	}

	function getLeftInputPadding(): string {
		let padding = Math.ceil(Math.log10(length + 1));

		if (padding < 1) {
			padding = 1;
		}

		return `${padding + 2}ch`;
	}

	function getWidth(): string {
		let characterWidth = 0;
		if (content) {
			characterWidth = Math.max(
				...content.split("\n").map((line: string) => line.length),
			);
		} else if (tokenLines) {
			let rawLines = tokenLines.map((line: any) =>
				line.map((token: any) => token.content).join(""),
			);

			characterWidth = Math.max(
				...rawLines.map((line: string) => line.length),
			);
		}

		return `calc(100% - 3.2rem - 5.6ch - ${getLeftInputPadding() + 3}ch)`;
	}

	function scrollElementToMiddleInContainer(
		container: HTMLElement,
		element: HTMLElement,
	): void {
		const containerRect = container.getBoundingClientRect();
		const elementRect = element.getBoundingClientRect();

		const offsetTop =
			elementRect.top - containerRect.top + container.scrollTop;
		const scrollTo =
			offsetTop - container.clientHeight / 2 + elementRect.height / 2;

		pushState(`#${element.id}`, {});

		container.scrollTo({
			top: scrollTo,
			behavior: "smooth",
		});
	}

	$effect(() => {
		if (textArea) {
			textArea.style.textWrap = $wrap ? "normal" : "nowrap";
		}
	});

	async function updateContent(newContent: string) {
		length = newContent.split("\n").length;

		content = newContent;
		writableContent.set(newContent);
	}

	onMount(async () => {
		const hash = window.location.hash;

		length = content
			? content.split("\n").length
			: tokenLines
				? tokenLines.length
				: 0;

		if (contentContainer && hash) {
			const id = hash.substring(1);
			const element = document.getElementById(id);
			if (element) {
				setTimeout(() => {
					scrollElementToMiddleInContainer(
						contentContainer as HTMLElement,
						element,
					);
				});
			} else {
				console.error(`Element with id ${id} not found.`);
			}
		}

		textArea?.addEventListener("keydown", (event) => {
			if (!textArea) return;

			if (event.key === "Tab") {
				event.preventDefault();

				textArea.setRangeText(
					"\t",
					textArea.selectionStart,
					textArea.selectionEnd,
					"end",
				);
			}
		});

		contentContainer?.addEventListener("keydown", (event) => {
			if ((event.metaKey || event.ctrlKey) && event.key == "a") {
				console.log("Select all triggered");
				event.preventDefault();
				contentContainer?.focus();
			}
		});
	});

	function selectAllContentInContainer(container: HTMLElement) {
		const range = document.createRange();
		range.selectNodeContents(container);

		const selection = window.getSelection();
		if (!selection) return;

		selection.removeAllRanges();
		selection.addRange(range);
	}

	let isFocused = false;

	onMount(() => {
		if (contentContainer) {
			contentContainer.addEventListener("click", () => {
				isFocused = true;
			});
		}

		document.addEventListener("click", (e) => {
			if (!contentContainer?.contains(e.target as Node)) {
				isFocused = false;
			}
		});

		document.addEventListener("keydown", (e) => {
			const isSelectAll =
				(e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "a";

			if (isFocused && isSelectAll) {
				e.preventDefault();
				selectAllContentInContainer(contentContainer as HTMLElement);
			}
		});
	});
</script>

<div
	id="content-container"
	onclick={() => {
		if (textArea && !isFocused) {
			textArea.focus();
		}
	}}
>
	<div id="content" bind:this={contentContainer}>
		{#if newPaste}
			<textarea
				oninput={(event) => {
					if (!textArea) return;
					const newContent = (event.target as HTMLTextAreaElement)
						.value;
					updateContent(newContent);
					textArea.style.width = getWidth();
				}}
				onkeydown={(event) => {
					if (event.key === "Enter") {
						window.scrollTo({
							left: 0,
							behavior: "smooth",
						});
					}
				}}
				id="input-textarea"
				placeholder="Paste your content here..."
				style="text-wrap: {$wrap
					? 'initial'
					: 'nowrap'}; left: {getLeftInputPadding()};"
				bind:this={textArea}
			></textarea>
		{/if}
		{#if tokenLines}
			{#each tokenLines as line, index}
				<div class="line-container">
					<span
						class="number"
						id={(index + 1).toString()}
						onclick={() => {
							const element = document.getElementById(
								(index + 1).toString(),
							);
							if (element) {
								scrollElementToMiddleInContainer(
									contentContainer as HTMLElement,
									element,
								);
							}
						}}
					>
						{padIndex(index + 1)}
					</span>
					<span
						id="line"
						style="text-wrap: {$wrap ? 'initial' : 'nowrap'}"
					>
						{#if line.length === 0}
							<span style="color: transparent;">{"\u200B"}</span>
						{:else}
							{#each line as token}
								<span
									class={$wrap ? "wrap" : ""}
									style="color: {token.color}; text-wrap: {$wrap
										? 'initial'
										: 'nowrap'}">{token.content}</span
								>
							{/each}
						{/if}
					</span>
				</div>
			{/each}
		{:else}
			{#each content.split("\n") as line, index}
				<div class="line-container">
					<span
						class="number"
						id={(index + 1).toString()}
						onclick={() => {
							const element = document.getElementById(
								(index + 1).toString(),
							);
							if (element) {
								scrollElementToMiddleInContainer(
									contentContainer as HTMLElement,
									element,
								);
							}
						}}
					>
						{padIndex(index + 1)}
					</span>
					<span
						id="line"
						class={$wrap ? "wrap" : ""}
						style="text-wrap: {$wrap ? 'initial' : 'nowrap'}"
					>
						{line == "" ? "\u200B" : line}
					</span>
				</div>
			{/each}
		{/if}
	</div>
</div>

<style lang="scss">
	#input-textarea {
		flex: 1;
		resize: none;
		border-radius: 0.5rem;
		color: var(--color-primary);
		background: transparent;
		font-size: 1rem;
		font-family: var(--font-family-mono);
		border: none;
		font-weight: 400;
		text-wrap: nowrap;
		font-variant-ligatures: none !important;
		-webkit-text-fill-color: transparent;
		z-index: 2;
		position: absolute;
		top: 0;
		left: 3ch;
		right: 3ch;
		bottom: 0;
		padding: 1.6rem;
		user-select: text;
		-webkit-user-select: text;
		height: calc(100% - 3.2rem);

		@media (max-width: 650px) {
			font-size: 0.9rem;
		}
	}

	#input-textarea::placeholder {
		color: var(--color-text-secondary);
		font-size: 1rem;
		font-family: var(--font-family);
		font-weight: 200;
	}

	#input-textarea:focus {
		outline: none;
	}

	#content {
		display: flex;
		flex-direction: column;
		padding: 1.6rem;
		background-color: var(--color-background);
		border-radius: 0.5rem;
		position: relative;
	}

	#content-container {
		width: 100%;
		height: 100%;
		overflow-y: auto;
		cursor: text;
	}

	#content #line {
		font-size: 1rem;
		color: var(--color-primary);
		font-family: var(--font-family-mono);
		font-weight: 400;
		margin: 0;
		display: inline-block;
		white-space: pre-wrap;
		animation: none;

		@media (max-width: 650px) {
			font-size: 0.9rem;
		}
	}

	#content #line::selection {
		background-color: var(--color-primary);
		color: var(--color-background);
	}

	.line-container {
		display: flex;
		flex-direction: row;
		gap: 2ch;
		font-family: var(--font-family-mono);
		padding-right: 1.6rem;
		animation: none;
		opacity: 1;
	}

	.number {
		font-size: 1rem;
		font-family: var(--font-family-mono);
		display: inline-block;
		margin: 0;
		color: var(--color-text-secondary);
		font-weight: 200;
		user-select: none;
		-webkit-user-select: none;
		cursor: pointer;
		animation: fadeIn 0.2s ease-in-out;

		@media (max-width: 650px) {
			font-size: 0.9rem;
		}

		@keyframes fadeIn {
			from {
				opacity: 0;
			}
			to {
				opacity: 1;
			}
		}
	}

	:global(span) {
		font-family: var(--font-family-mono);
		width: fit-content;
	}

	span {
		margin: 0;
		padding: 0;
		white-space: pre;
		position: relative;

		&.wrap {
			display: inline-block;
			overflow-wrap: anywhere;
			word-break: normal;
			white-space: pre-wrap;
		}
	}

	span::selection {
		background-color: var(--color-primary);
		color: var(--color-background);
	}
</style>
