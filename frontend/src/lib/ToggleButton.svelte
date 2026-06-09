<script lang='ts'>
	import Button from '$lib/Button.svelte';
	import type { ClassValue } from 'svelte/elements';
	import { error } from '@sveltejs/kit';
	import { onMount } from 'svelte';

	let {
		// Core props:
		lbl,
		img,
		onclick = $bindable(),

		// Add a form element to track the value
		name = null, // If null, don't track it
		value = 'true',

		// HTML props:
		class: ext_classes = $bindable(''),
		...others
	} = $props();

	let self;
	let cmb_classes = $derived.by(()=>{
		const cl = ext_classes.split(' ');
		cl.push('tbtn');
		if(selected){cl.push('sel')}
		return cl.join(' ');
	});

	let input;
	let selected = $state(false);

	export function select(val) {
		if(val===undefined) {
			// Toggle
			selected=!selected;
		} else {
			// Set
			selected=val;	
		}
		return select;
	}

	export function is_selected() {
		return selected;
	}
	
	// If the toggle button is .sel on creation, move it to internal
	const IS_SELECTED = /\bsel\b/g
	if(IS_SELECTED.test(ext_classes)){
		ext_classes = ext_classes.replaceAll(IS_SELECTED, '');
		selected = true;
	}

	function local_onclick(event) {
		selected = !selected;
		if(onclick){onclick(event)}
	}

</script>


<Button bind:this={self} class={cmb_classes} {img} {lbl} onclick={local_onclick} {...others} />
{#if name && selected}
<input bind:this={input} type='hidden' {name} {value} />
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