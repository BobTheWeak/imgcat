<script lang='ts'>
	import { pretty_print_relative } from '$lib/time_fmt.ts';
	// NOTE: nested is a dict: {comment_id:[comment, ...]}
	// TODO: This only works one level deep, but 
	const { comment, nested, post_link, user_id } = $props();
	//const my_nested = nested[comment.id];
	//console.log(my_nested);

	let reply_here = $state(false);
	let is_collapsed = $state(true);

</script>

{#snippet comment_block(c)}
	<div class='block'>
		<p>{c.username} - {pretty_print_relative(c.time, navigator.language)}</p>
		<span>{c.comment}</span>
	</div>
{/snippet}

{#snippet reply_block(reply_to)}
	{#if user_id}
		<!-- <div class='block'> -->
		{#if reply_here}
			<!-- TODO: Strip this out into its own control, so CommentBox & Comment can have a common control & codebase, using remote functions. -->
			<button onclick={()=>{reply_here=false}}>Cancel</button>
			<form method='POST' action='/view/{post_link}?/comment'>
				<label for='comment'>Post a comment:</label>
				<textarea name='comment' rows=4 maxlength=255 ></textarea>
				<input type='hidden' name='reply_to' value='{reply_to}' />
				<input type='submit' value='Post' />
			</form>
		{:else}
			<button onclick={()=>{reply_here=true}}>Reply</button>
		{/if}
		<!-- </div> -->
	{/if}
{/snippet}

<div class='cmt'>
	{@render comment_block(comment)}

	{#if nested}
		<div class='block'>
			{#if is_collapsed}
				{#if nested.length==1}
				<button onclick={()=>{is_collapsed=false}}>Show 1 reply</button>
				{:else}
				<button onclick={()=>{is_collapsed=false}}>Show {nested.length} replies</button>
				{/if}
				{@render reply_block(comment.id)}
			{:else}
				<button onclick={()=>{is_collapsed=true}}>Hide replies</button>
				{@render reply_block(comment.id)}
				<div class='cmt' style='margin-left:10px'>
				{#each nested as c}
					{@render comment_block(c)}
				{/each}
				{@render reply_block(comment.id)}
				</div>
			{/if}
		</div>
	{:else}
		{@render reply_block(comment.id)}
	{/if}
</div>

<style>
	div.block {
		/*margin-top: 1em;*/
	}
	div.cmt {
		/*background-color: var(--cb4);*/
		border-left: 2px solid var(--cb1);
		border-radius: 10px 0px 0px 10px;
		margin: 5px 0px;
		padding: 5px 10px;

		p {
			color: var(--cticy);
		}

		span {
			margin: 10px 0px 0px 0px;
		}

		button {
			display: inline-block;
		}
	}
</style>