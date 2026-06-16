<script lang='ts'>
	import Comment from '$lib/Comment.svelte';
	import Button from '$lib/Button.svelte';
	import { getPostComments, createComment } from '$lib/CommentBox.remote.ts';

	const { post, user_id } = $props();
	let comments = [];
	if(user_id && post.is_public) {
		comments = getPostComments(post.id);
		comments.then((c)=>{
			//console.log(c);
		})
	}

	// Form data & posting comments
	let message_element;

	function post_comment() {
		createComment([
			user_id, post.id, null, null, message_element.value
		]).then((id)=>{
			if(id) {
				message_element.value = '';
				window.location.reload();	
			}
		}).catch((e)=>{
			// TODO:
		});
	}
</script>

<!-- TODO: Strip this out into its own control, so Comment can reuse it -->
<!-- Right now, replies are broken -->
{#if user_id && post.is_public}
<div>
	<label for='message'>Post a comment:</label>
	<textarea bind:this={message_element} name='message' rows=4 maxlength=255 ></textarea>
	<Button lbl='Post' onclick={post_comment} />
</div>
{/if}

{#await comments then {root, nested} }
	{#each root as c}
		<!-- TODO: This only handles a single layer, not a nested chain -->
		<!-- NOTE: Be careful about performance. Copy-by-ref, not value. -->
		{#if c.id in nested}
			<Comment post_link={post.link} {user_id} comment={c} nested={nested[c.id]} />
		{:else}
			<Comment post_link={post.link} {user_id} comment={c} />
		{/if}
	{/each}
{:catch}
	<p>There was an error loading comments</p>
{/await}

<style>
	label {
		display: block;
	}
	textarea {
		width: 100%;
		min-width: 300px;
		max-width: 800px;
	}
	input {
		margin-bottom: 20px;
	}
</style>