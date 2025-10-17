<script lang="ts">
	import NewPaste from "./button/NewPaste.svelte";
	import { disableNew, writableTitle } from "$lib/stores";
	import { onMount } from "svelte";

	let titleElement: HTMLInputElement | null = $state(null);

	let {
		title = null,
		editable = false,
		createdAt = null,
		expiresAt = null,
		burn = null,
		language = null,
		loadingPromise = null,
	} = $props();

	onMount(() => {
		titleElement?.addEventListener("input", updateWritableTitle);
	});

	function updateWritableTitle() {
		if (titleElement) {
			writableTitle.set(titleElement.value);
		}
	}
</script>

<div id="navbar" class={editable ? "" : "display"}>
	{#if editable}
		<input
			type="text"
			id="title"
			bind:value={title}
			bind:this={titleElement}
			placeholder="Enter a title..."
		/>
	{:else}
		<div id="content">
			{#if title}
				<div id="title-container">
					<h1 id="title">{title}</h1>
					{#await loadingPromise}
						<p>Loading...</p>
					{:then}
						{#if burn == true}
							{#if window.innerWidth < 650}
								<p>
									Created {createdAt} • {expiresAt != "0 seconds"
										? `Expires in ${expiresAt}`
										: "Never Expires"}{language
										? " • " + language
										: ""}
								</p>
								<p><strong>Burn Enabled</strong></p>
							{:else}
								<p>
									Created {createdAt} • {expiresAt != "0 seconds"
										? `Expires in ${expiresAt}`
										: "Never Expires"}
									•
									<strong>Burn Enabled</strong>{language
										? " • " + language
										: ""}
								</p>
							{/if}
						{:else}
							<p>
								Created {createdAt} • {expiresAt !=
								"0 seconds"
									? `Expires in ${expiresAt}`
									: "Never Expires"}{language
									? " • " + language
									: ""}
							</p>
						{/if}
					{/await}
				</div>
			{:else}
				<h1 id="title">PasteBook</h1>
			{/if}
		</div>
	{/if}
	{#if !$disableNew}
		<div id="buttons">
			<a href="/about">About</a>
			<div id="new">
				<NewPaste />
			</div>
		</div>
	{/if}
</div>

<style lang="scss">
	#navbar {
		display: flex;
		justify-content: space-between;
		align-items: center;
		justify-content: center;
		padding: 1.4rem;
		width: calc(100% - 2.8rem);
		padding-bottom: 1.6rem;
		background-color: var(--color-background);
		background-image: url("data:image/svg+xml,%3Csvg width='100%25' height='2' xmlns='http://www.w3.org/2000/svg'%3E%3Cline x1='0' y1='0' x2='100%25' y2='0' stroke='%235F5F5FFF' stroke-width='4' stroke-dasharray='5%2C12' stroke-linecap='square' /%3E%3C/svg%3E");
		background-repeat: no-repeat;
		background-position: bottom;
		background-size: 100% 1px;

		@media (max-width: 650px) {
			padding: 1rem;
			width: calc(100% - 2rem);
		}
	}

	input {
		padding: 0;
	}

	#content {
		flex: 1 1 auto;
		min-width: 0;
		overflow: hidden;
	}

	#title-container {
		display: flex;
		flex-direction: column;
		overflow: hidden;
	}

	#title {
		font-size: 3rem;
		color: var(--color-primary);
		margin: 0;
		font-family: var(--font-family);
		font-weight: 800;
		background-color: transparent;
		border: none;
		outline: none;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
		width: calc(100% - 2.8rem);
		align-content: center;

		@media (max-width: 650px) {
			font-size: 2.1rem;
		}
	}

	#title-container p {
		font-size: 1rem;
		color: var(--color-text-secondary);
		margin: 0;
		font-family: var(--font-family);
		font-weight: 200;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;

		@media (max-width: 650px) {
			font-size: 0.8rem;
		}
	}

	#buttons {
		display: flex;
		gap: 1.6rem;
		flex-shrink: 0;
		justify-content: flex-end;
		align-items: center;
		padding-left: 1.6rem;

		@media (max-width: 650px) {
			gap: 1.2rem;
			padding-left: 1.2rem;
		}
	}

	a {
		color: var(--color-primary);
		font-size: 1rem;
		font-family: var(--font-family);
		font-weight: 200;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;

		@media (max-width: 650px) {
			font-size: 0.8rem;
		}
	}
</style>
