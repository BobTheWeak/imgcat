<script lang="ts">
	import Comment from '$lib/Comment.svelte';
	import VoteBox from '$lib/VoteBox.svelte';
	import ActionBox from '$lib/ActionBox.svelte';
	import CommentBox from '$lib/CommentBox.svelte';
	const { data } = $props();

	let post = data?.post;
	console.log(post);
</script>

{#if post}
	<div id="header">
		{#if post.title}
		<h1>{post.title}</h1>
		{/if}
		<p>{post.username} - {post.time}</p>
	</div>
	<div id="content">
		{#each post.img as item}
		<div class="imgbox">
			{#if item.type == 'image'}
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
	<div id="panel">
		<VoteBox {post} user_id={data.user_id} />
		<ActionBox {post} user_id={data.user_id} />
		<CommentBox {post} user_id={data.user_id} />
	</div>
{:else}
	<p>There was an error loading this post</p>
{/if}

<style>
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