<script lang='ts'>
	import { pretty_print_relative } from '$lib/time_fmt.ts';
	import Comment from './Comment.svelte';
	import Button from '$lib/Button.svelte';
	import Modal from '$lib/Modal.svelte';
	import { enhance } from '$app/forms';

	import { get_badges } from './actions.remote.ts';

	let {
		post,
		form,
		user_id,
		comment_replies,
		reply_to = $bindable(null),
	} = $props();

	const {comments, replies} = $derived(await comment_replies);
	const badges = $derived(await get_badges(Array.from(comments.keys())));

	let showCommentModal = $derived(Number.isInteger(reply_to));

	function submit_success(e){
		return async({result, update}) => {
			console.log(e);
			console.log(result);
			await update();
			// Do post-processing
			if(form?.success) {
				reply_to = null;
			}
		}
	}
</script>

<!-- <div>
	<label>Reply to this post:
		<textarea bind:this={message_element} name='message' rows=4 maxlength=255 ></textarea>
	</label>
	{#if reply_to !== 0}
	<Button img='/chat.svg' lbl='Comment' onclick={()=>reply_to=0} />
	{:else}
	<Button img='/chat.svg' lbl='Cancel' onclick={()=>reply_to=null} />
	{/if}
</div> -->
{#each replies.get(0) as id}
<div class='root-cmt'>
<Comment {id} {comments} {replies} {badges} bind:reply_to />
</div>
{/each}

<Modal id='comment' bind:showModal={showCommentModal} title='Connect with the community'>
	<form method='POST' action='?/comment' use:enhance={submit_success} >
		{#if reply_to>0}
		<label style='color:var(--cticy)'>TBD Username - {pretty_print_relative(comments.get(reply_to).ts, navigator.language)}</label>
		<p>{comments.get(reply_to).text}</p>
		<input type='hidden' name='reply_to' value={reply_to} />
		<br />
		{/if}
		<textarea name='comment' style='width:30em;height:5em'></textarea>
		<br /><br />
		<Button img='/chat.svg' lbl={(reply_to===0?'Comment':'Reply')} type='submit' />
	</form>
</Modal>

<style>
	div.root-cmt {
		background-color: var(--cb3);
		margin-top: 1em;

		label, textarea {
			display: block;
		}
	}
</style>