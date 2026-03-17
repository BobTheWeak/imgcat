<script lang="ts">
	import TemplateSelector from './TemplateSelector.svelte';
	import { get_width, split_text } from './MemeMaker.ts';

	const PADDING=20;
	const data = $props();
	let selected_meme = $state(undefined);
	let text_blocks = $state([]);
	let text_split = $state([]);
	let font_size = $state(28);
	let dl_link = $state(undefined);

	const update_meme=(values)=>{
		//console.log('Updating meme template...');
		text_blocks = [];
		if(values.text){
			for(const box of values.text){
				text_blocks.push(box.text||'');
			}
			update_all();
		}
	}

	const update_all=(e)=>{
		//console.log('Updating meme text ALL...');
		if(selected_meme?.text){
			for(const i in selected_meme.text){
				update_one(e,i);
			}
		}
	}

	const update_one=(e,idx)=>{
		//console.log('Updating meme text #'+idx+'...');
		// Req: y, dir
		// Opt: x, height, width, text
		const box = selected_meme.text[idx];
		const w=(box.right&&box.left)?(box.right-box.left):(selected_meme.width-2*PADDING);
		const h=get_width('X',font_size).h;
		const x=box.x||'50%';

		let ls=split_text(text_blocks[idx], w, font_size+'px');
		if(!box.dir){ls = ls.reverse()}
		text_split[idx] = ls;
	}

	const getFileAsBase64=async(url)=>{
		const r=await fetch(url);
		const d=await r.arrayBuffer();
		return {
			d: new Uint8Array(d).toBase64(),
			t: r.headers.get('content-type')
		}
	};

	const save = async(e)=>{
		const d=await getFileAsBase64('/api/img/'+selected_meme.image);
		const clone=document.getElementById('svg').cloneNode(true);
		clone.getElementsByTagName('image')[0].setAttribute('href', 'data:'+d.t+';base64,'+d.d);
		const data = (new XMLSerializer()).serializeToString(clone);
		const blob = new Blob([data], {type: 'image/svg+xml;charset=utf-8'});
		const url = URL.createObjectURL(blob);
		
		const btnlink=document.getElementById('dl_link');
		const hr=btnlink.getAttribute('href');
		if(hr){URL.revokeObjectURL(hr)}
		btnlink.setAttribute('download', (selected_meme.name.replaceAll(' ',''))+'.svg');
		btnlink.setAttribute('target', '_blank');
		btnlink.setAttribute('href', url);
		btnlink.click();
		// Let the event continue to propigate, and it'll DL
	}

	//$inspect(text_blocks);
	//$inspect(selected_meme);
</script>

<h1>Meme Maker</h1>

<TemplateSelector bind:values={selected_meme} onchange={update_meme} />
{#if selected_meme}
	<br/>
	{#each selected_meme.text as block, idx}
	<textarea cols='50' rows='3' bind:value={text_blocks[idx]} oninput={(e)=>{update_one(e,idx)}}>{block.text||''}</textarea>
	{/each}
	<label>
		<input type='number' min='14' max='128' step='2' bind:value={font_size} onchange={update_all}/>
		Font size
	</label>
	<button onclick={save} style='margin-left:30px'>Save meme as...</button>
	<a id='dl_link' style='visibility:hidden'></a>
	<br/><br/>

	<svg id='svg' height={selected_meme.height} width={selected_meme.width}>
		<style>text{font-family:sans-serif;font-weight:bold;fill:#FFF;stroke:#000;stroke-width:0.1em;paint-order:stroke;stroke-linejoin:round;text-anchor:middle;dominant-baseline:middle}</style>
		<image href='/api/img/{selected_meme.image}' width='100%' />
		{#each selected_meme.text as tb, i}
		<text x={tb.x||'50%'} y={tb.y+font_size*(tb.dir?-0.5:0.5)} style='font-size:{font_size}px'>
			{#each text_split[i] as ts, j}
			<tspan x='50%' dy={(tb.dir?font_size:-font_size)}>{ts}</tspan>
			{/each}
		</text>
		{/each}
	</svg>
{/if}

<style>
	svg {
		border:1px solid black
	}
	textarea {
		display: block;
	}
</style>