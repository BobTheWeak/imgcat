<script lang='ts'>
	import Button from '$lib/Button.svelte';
	import ModBox from '$lib/ModBox.svelte';
	import { isFavPost, toggleFavPost, setPostVisibility } from '$lib/ActionBox.remote.ts';
	import { goto } from '$app/navigation';

	const {
		post,
		user_id
	} = $props();

	let mod_box = $state(false);
	let is_fav = $state(false);

	// Only check if we're authenticated
	if(user_id && user_id != post.user_id) {
		isFavPost([user_id, post.id]).then((onoff)=>{
			is_fav = onoff;
		});
	}

	function make_pub(e) {
		setPostVisibility([user_id, post.id, !post.is_public]).then(()=>{
			// Do a total reload of the page, and re-fetch all data
			window.location.reload();
		});
	}

	function copy_link(e) {
		const p = navigator.clipboard.writeText('https://www.imgcat.io/view/'+post.link);
		// This won't work b/c we changed to Button controls
		//p.then(()=>{
		//	e.target.classList.add('sb_cplk');
		//	setTimeout(()=>{
		//		e.target.classList.remove('sb_cplk');
		//	}, 700);
		//});
	}

	function toggleFav() {
		toggleFavPost([user_id, post.id, null]).then((onoff)=>{
			is_fav = onoff;
		});
	}

	function report_modal() {
		mod_box = !mod_box;
	}
</script>

<div>
	{#if user_id == post.user_id}
		<!-- Your post -->
		<Button img='/add.svg' lbl={post.is_public?'Make private':'Share with community'} onclick={make_pub} />
		<Button img='/share.svg' lbl='Copy link' onclick={copy_link} />
		<Button img='/download.svg' lbl='Download' onclick={()=>{window.location=post.img[0].link}} />
	{:else if user_id}
		<!-- Authenticated user actions -->
		<Button img='/star_{is_fav?'on':'off'}.svg' lbl='Favorite' onclick={toggleFav} />
		<Button img='/share.svg' lbl='Copy link' onclick={copy_link} />
		<Button img='/download.svg' lbl='Download' onclick={()=>{window.location=post.img[0].link}} />
		<Button img='/report.svg' lbl='Report' onclick={report_modal} />
	{:else}
		<!-- Anonymous user actions -->
		<Button img='/share.svg' lbl='Copy link' onclick={copy_link} />
		<Button img='/report.svg' lbl='Report' onclick={report_modal} />
	{/if}
	{#if mod_box}
		<ModBox {post} {user_id} />
	{/if}
</div>

<style>
	div{
		padding: 5px 0px;
	}
</style>