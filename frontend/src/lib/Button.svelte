<script lang='ts'>
	import type { ClassValue } from 'svelte/elements';
	import { error } from '@sveltejs/kit';

	let {
		// Core (required) props:
		lbl,
		img,
		onclick = $bindable(),

		// Core (optional) props:
		href = null, // Wraps the button in a regular <a> link
		size = '1.5em', // Controls the size of the button

		// HTML props:
		class: ext_classes = $bindable(''),
		type = 'button',
		...others


		// Color props:
		// --bkg='#ABCDEF'
		// --bdr='#ABCDEF'
		// --txt='#ABCDEF'
		// Color props (if class='sel')
		// --bkg-sel='#ABCDEF'
		// --bdr-sel='#ABCDEF'

	} = $props();

	let self;
	let int_classes = $state(['btn']);
	let cmb_classes = $derived(ext_classes.split(' ').concat(int_classes).join(' '));

	if(!lbl&&!img) {
		error(500, '<Button> must have either lbl or img');
	}

</script>


{#snippet buttonblock()}
<button bind:this={self} {type} class={cmb_classes} {onclick} {...others}>
	{#if img}
	<img src="{img}" alt='{lbl}' style='height:{size}'/>
	{/if}
	{#if lbl}
	<span style='height:{size};line-height:{size}'>{lbl}</span>
	{/if}
</button>
{/snippet}


{#if href}
	<a href='{href}'>
		{@render buttonblock()}
	</a>
{:else}
	{@render buttonblock()}
{/if}


<style>
	/*
	button {
		display: inline-block;
		padding: 0.5em 1em;
		border: 1px solid var(--bdr, var(--cawarm));
		color: var(--txt, var(--cahot));
		background-color: var(--bkg, var(--cb3));
		cursor: pointer;
		margin-right: 5px;
	}
	button:hover:enabled:not(.sel) {
		background-color: var(--bkg-sel, var(--cb4));
		border-color: var(--bdr-sel, var(--bdr, var(--cahot)));
		padding: calc(0.5em - 1px) calc(1em - 1px);
		border-width: 2px;
	}
	button.sel {
		background-color: var(--bkg-sel, var(--cawarm));
		border-color: var(--bdr-sel, var(--bdr, var(--cahot)));
		color: var(--txt-sel, var(--cb3));
	}
	img {
		margin-right: 5px;
		vertical-align: middle;
		filter: brightness(0) saturate(100%) invert(74%) sepia(12%) saturate(6987%) hue-rotate(327deg) brightness(88%) contrast(86%);
	}
	button.sel img {
		filter: brightness(0) saturate(100%) invert(36%) sepia(0%) saturate(0%) hue-rotate(284deg) brightness(95%) contrast(96%);
	}
	span {
		vertical-align:middle;
		display:inline-block;
	}
	*/
</style>