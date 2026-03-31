<script lang='ts'>
	import type { ClassValue } from 'svelte/elements';
	import { error } from '@sveltejs/kit';

	const {
		// Core (required) props:
		lbl,
		img,
		onclick = $bindable(),

		// HTML props:
		id = null,
		class: classes = null,
		title = null,
		name = null,
		value = null,
		style = null

		// Color props:
		// --bkg='#ABCDEF'
		// --bdr='#ABCDEF'
		// --txt='#ABCDEF'

		// Color props (if class='sel')
		// --bkg-sel='#ABCDEF'
		// --bdr-sel='#ABCDEF'

	} = $props();

	if(!lbl&&!img) {
		error(500, '<Button> must have either lbl or img');
	}

</script>

<button {id} class={classes} {onclick} {title} {name} {value} {style}>
	{#if img}
	<img src="{img}" alt='{lbl}'/>
	{/if}
	{#if lbl}
	<span>{lbl}</span>
	{/if}
</button>

<style>
	button {
		display: inline-block;
		padding: 5px 10px;
		border: 1px solid var(--bdr, var(--cawarm));
		border-radius: 10px;
		color: var(--txt, var(--cahot));
		background-color: var(--bkg, var(--cb3));
		cursor: pointer;
		margin-right: 5px;
	}
	button:hover {
		background-color: var(--bkg-sel, var(--cb4));
		border-color: var(--bdr-sel, var(--bdr, var(--cahot)));
		padding: 4px 9px;
		border-width: 2px;
	}
	button.sel {
		background-color: var(--bkg-sel, var(--cawarm));
		border-color: var(--bdr-sel, var(--bdr, var(--cahot)));
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

	}
</style>