<script lang='ts'>
	import { pretty_print_relative } from '$lib/time_fmt.ts';
	import Comment from './Comment.svelte';
	import Button from '$lib/Button.svelte';

	let {
		id,
		comments,
		replies,
		badges,
		reply_to = $bindable(),
		level = 0,
	} = $props();
	const self = comments.get(id);
	let user = $derived(badges?.get(self.user_id));
	let open = $state( level<=0 ); // By deafult, open root comments & show the second layer
</script>


<div class='cmt'>
	<label>{#if user}<a href='/u/{user.link}'>{user.username}</a>{:else}Unknown User{/if} - {pretty_print_relative(self.ts, navigator.language)}</label>
	<p>{self.text}</p>
	
	<div class='btngrp'>
		<Button img='/chat.svg' lbl='Reply' class={(reply_to===id?'tbtn sel':undefined)} onclick={()=>reply_to=id} />

		{#if replies.has(id)}
			{#if open}
				<Button lbl='Hide replies' onclick={()=>{open=false}} />
			{:else}
				<Button lbl='See replies ({replies.get(id).length})' onclick={()=>{open=true}} />
			{/if}

		{/if}
	</div>
	{#if open}
		{#each replies.get(id) as inner_id}
			<Comment id={inner_id} {comments} {replies} {badges} bind:reply_to level={level+1} />
		{/each}
	{/if}
</div>

<style>
	div.cmt {
		background-color: var(--cb3);
		margin-top: 1.5em;
		padding: 0.5em 0em 0.5em 1.5em;
		border-left: 4px solid var(--cb1);

		label, p {
			display: block;
		}
		label {
			color: var(--cticy);
		}
		div.btngrp {
			margin-top:1em;
		}

	}
	textarea {
		display: block;
		width: 100%;
		min-width: 300px;
		max-width: 800px;
	}
	input {
		margin-bottom: 20px;
	}
</style>