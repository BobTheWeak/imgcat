<script lang="ts">
	import { enhance } from '$app/forms';
	let { data, form } = $props();
</script>

<h1>Post</h1>
<form method="POST" enctype="multipart/form-data" use:enhance>
<div class="box">
	<h2>Upload Image</h2>
	{#if form?.err_msg}<p class="error">{form.err_msg}</p>{/if}
	<input type='file' name="upload-file"/>
	<button type='submit' formaction="?/upload">Next</button>
</div>
<div class="box">
	<h2>Meme Builder</h2>
	<ul>
		<li><a href='/meme'>From template</a></li>
		<li><a href='/meme'>From favorites</a></li>
	</ul>
</div>
<!--<div class="box">
	<h2>Repost from ImgCat</h2>
	<input type='url' name="repost-url" placeholder="ImgCat URL"/>
	<button type='submit' formaction="?/repost">Next</button>
</div>-->
</form>

<style>
	div.box {
		display: inline-block;
		margin: 10px;
		padding: 10px;
		border: 1px solid var(--cb1);
		border-radius: 10px;
		background-color: var(--cb3);
	}
	p.error {
		color: red;
	}
	ul {
		color: var(--cahot);
	}
</style>