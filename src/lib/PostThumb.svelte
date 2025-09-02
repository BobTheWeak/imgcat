<script lang="ts">
	import { page } from '$app/stores';
	let { post } = $props();
	
	// Get imageBaseUrl from page data using $derived
	const imageBaseUrl = $derived($page.data.imageBaseUrl || 'http://localhost:9000/imgcat-dev');
	
	// Debug logging
	$effect(() => {
		console.log('PostThumb - imageBaseUrl:', imageBaseUrl);
		console.log('PostThumb - post["first_img"]:', post?.["first_img"]);
		console.log('PostThumb - full URL:', `${imageBaseUrl}/${post?.["first_img"]}`);
	});
</script>

{#if post}
<div class='post_thumb'>
	<a href='/view/{post["link"]}'>
	<!-- svelte-ignore a11y_missing_attribute (b/c it's a user-generated img) -->
	<div><img src='{imageBaseUrl}/{post["first_img"]}' /></div>
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