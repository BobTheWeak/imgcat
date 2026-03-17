<script lang="ts">
	import { tick } from 'svelte';
	import {getTemplates, getTemplateTags} from './TemplateSelector.remote.ts';

	// We bind two properties: id:number, values:Object
	let {
		id=$bindable(undefined),
		values=$bindable(undefined),
		onload=$bindable(()=>{}),
		onchange=$bindable(()=>{})
	} = $props();
	
	// Local data
	let all_data = $state(undefined);

	// This is the primary load function
	getTemplates().then((d)=>{
		// As long as at least a single item is returned, select data[0]
		if(d.length>0){
			id=d[0].id;
			values=d[0];
		}

		// There are "cleaner" ways to do this. But the automagic juggling
		// between: Svelte states, promises, remote functions, JSON data,
		// all stored in the same object... gee wiz... it gets wonky.
		all_data = d;

		/////////////////////////////////
		//onload(values);
		changed();
	});

	const changed=()=>{
		values=all_data.find((e)=>e.id==id);
		onchange(values);
	}
</script>

<!-- <select bind:value={ (e)=>{console.log(e)} }> -->
{#if all_data}
<div>
	<label>
	<select bind:value={id} onchange={changed}>
	{#each all_data as item(item.id)}
		<option value='{item.id}'>{item.name}</option>
	{/each}
	</select>
	Select a meme template
	</label>
</div>
{/if}