<script lang="ts">
	//import { hydratable } from 'svelte';
	import { pretty_print_relative } from '$lib/time_fmt.ts';
	//import Comment from '$lib/Comment.svelte';
	//import VoteBox from '$lib/VoteBox.svelte';
	//import ActionBox from '$lib/ActionBox.svelte';
	import SingleActionBar from './SingleActionBar.svelte';
	import CommentBox from '$lib/CommentBox.svelte';
	import { liveViews } from './actions.remote.ts';

	const { data, form } = $props();
	const post = $derived(data.post);

	// Lazy-load various types of data
	const views = $derived(await liveViews(post.id));
</script>

{#if post}
	<div id="header">
		{#if post.title}
			<h1>{post.title}</h1>
		{/if}
		<p>{post.username} - {pretty_print_relative(post.time, navigator.language)}
		{#await views then v}
			- {v} {#if v > 1}views{:else}view{/if}
		{/await}
		</p>
	</div>
	<div id="content">
		{#each post.img as item, i}
		<div class="imgbox">
			{#if item.type == 'image'}
				<img src="{item.link}" />
			{:else if item.type == 'svg'}
				<img src="{item.link}" />
			{:else if item.type == 'video'}
				<video controls>
					<source src='{item.link}' type='{item.mime_type}' />
				</video>
			{/if}
			{#if item.description}
			<p>{item.description}</p>
			{/if}
		</div>
		{/each}
	</div>
	<SingleActionBar {post} {form} user_id={data.user_id} />
	<!--
	<CommentBox {post} user_id={data.user_id} />
	-->
{:else}
	<p>There was an error loading this post</p>
{/if}

<style>
	div#header {
		p {color: var(--cticy)}
	}
	div#content {
		display: inline-block;
		max-width: 800px;

		div.imgbox {
			border: 1px solid black;

			img,video {
				width: 100%;
			}
			p {
				margin: 10px 20px;
			}
		}
	}
</style>