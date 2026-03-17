<script lang="ts">
	let { post } = $props();
</script>

{#if post}
<div class='post_thumb'>
	<a href='/view/{post["link"]}'>
	<!-- svelte-ignore a11y_missing_attribute (b/c it's a user-generated img) -->
	<div><img src='/api/img/{post["first_img"]}' /></div>
	{#if post["title"]}
	<p>{post["title"]}</p>
	{/if}
	{#if post.is_public}
		<p>{post["views"]} views, {post["votes"]} points</p>
	{:else}
		<p>- Private -</p>
	{/if}
	</a>
</div>
{:else}
<p>Oops, we can't render this post</p>
{/if}

<style>
	div.post_thumb {
		display: inline-block;
		border: 1px solid black;
		width: 300px;
		vertical-align: top;
		margin: 3px;
		
		a {
			text-decoration:none;
			color: #000;
		}

		div {
			max-height: 300px;
			overflow-y: hidden;

			img {
				width: 100%;
			}
		}

		p {
			padding: 0px 5px;
			margin: 0px;
		}
	}
</style>