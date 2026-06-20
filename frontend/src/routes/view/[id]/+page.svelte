<script lang="ts">
	import { pretty_print_relative } from '$lib/time_fmt.ts';
	import SingleActionBar from './SingleActionBar.svelte';
	import CommentPanel from './CommentPanel.svelte';

	const { data, form } = $props();
	const post = $derived(data.post);

	// The selected comment we're replying to.
	// Null closes the modal, 0 replies to the post, and any number replies to that comment_id
	let reply_to = $state(null);
</script>

{#if post}
	<div id="header">
		{#if post.title}
			<h1>{post.title}</h1>
		{/if}
		<p>
		{#if post.user}
		<a href='/u/{post.user.link}'>{post.user.username}</a>
		{:else}
		Unknown User
		{/if}
		 - {pretty_print_relative(post.time, navigator.language)}

		{#await data.views then v}
		{#if v} - {v} {#if v > 1}views{:else}view{/if}{/if}
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
	<SingleActionBar {data} bind:reply_to />
	<CommentPanel {post} {form} user_id={data.user_id} comment_replies={data.comment_replies} bind:reply_to />
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