<!-- Code borrowed & adapted from Svelte's code playground:
  -- (https://svelte.dev/playground/modal?version=5.50.2) -->
<script>
	let { showModal = $bindable(false), title, children } = $props();

	let dialog = $state(); // HTMLDialogElement

	$effect(() => {
		if (showModal) {
			dialog.showModal();
		} else {
			dialog.close();
		}
	});
</script>

<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_noninteractive_element_interactions -->
<dialog
	bind:this={dialog}
	onclose={()=>(showModal=false)}
	onclick={(e)=>{ if(e.target===dialog) showModal=false }}
>
	<div>
		{#if title}
		<h1>{title}</h1>
		{/if}
		{@render children?.()}
	</div>
</dialog>

<style>
	dialog {
		border-radius: 1em;
		border: none;
		padding: 0;
	}
	dialog::backdrop {
		background: rgba(0, 0, 0, 0.4);
	}
	dialog h1 {
		margin: 0em;
	}
	dialog > div {
		padding: 1em;
	}
	dialog[open] {
		animation: zoom 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
	}
	@keyframes zoom {
		from {
			transform: scale(0.95);
		}
		to {
			transform: scale(1);
		}
	}
	dialog[open]::backdrop {
		animation: fade 0.2s ease-out;
	}
	@keyframes fade {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}
	button {
		display: block;
	}
</style>
