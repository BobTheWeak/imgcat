<!-- Code borrowed & adapted from Svelte's code playground:
  -- (https://svelte.dev/playground/modal?version=5.50.2) -->
<script>
	let {
		showModal = $bindable(false),
		id = null,
		title = null,
		children
	} = $props();

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
<dialog {id}
	bind:this={dialog}
	onclose={()=>(showModal=false)}
	onclick={(e)=>{ if(e.target===dialog) showModal=false }}
>
	{#if title}
	<h1>{title}</h1>
	{/if}
	<div id='content'>
		{@render children?.()}
	</div>
</dialog>

<style>
	dialog {
		top: -15%;
		border-radius: 1em;
		border: none;
		background-color: var(--cb2);
		border: 1px solid var(--cawarm);
		color: var(--cttxt);
		padding: 0px;
	}
	dialog::backdrop {
		background: rgba(72, 72, 72, 0.8);
	}
	dialog h1 {
		margin: 0em;
		padding: 0.25em;
		text-align: center;
		border-bottom: 1px solid var(--cawarm);
	}
	dialog div#content {
		padding: 1.25em;
		background-color: var(--cb4);
	}
	dialog[open] {
		animation: zoom 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
	}
	@keyframes zoom {
		from {
			transform: scale(0.90);
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
</style>
