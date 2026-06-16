<script module>
	const TAG_REGEX = /^[\w\s\-,]+$/
</script>
<script lang='ts'>
	import Button from '$lib/Button.svelte';
	import ToggleButton from '$lib/ToggleButton.svelte';
	import Modal from '$lib/Modal.svelte';
	import { enhance } from '$app/forms';
	import { refreshAll } from '$app/navigation';

	let {
		data,
		reply_to = $bindable(),
	} = $props();

	// 
	let post = $derived(data.post);
	let form = $derived(data.form);
	let user_id = $derived(data.user_id);
	let votes = $derived(data.votes);
	let my_vote = $derived(data.my_vote);
	let is_fav = $derived(data.is_fav);

	// svelte-ignore non_reactive_update
	let user_type; // (m)e, (u)ser, (a)nonymous
	// svelte-ignore state_referenced_locally
	if(user_id){if(user_id==post.user_id){user_type='m'}else{user_type='u'}}else{user_type='a'}

	let showTagModal = $state(false);
	let showTagModal_types = $state(false);
	let showReportModal = $state(false);
	let showReportAnonModal = $state(false);
	let showThanksModal = $state(false);

	function copy_link(e) {
		const p = navigator.clipboard.writeText('https://www.imgcat.io/view/'+post.link);
		// TODO: Some kind of animation showing we copied it
	}

	// Bind controls to Svelte objects
	let m_kid, m_std, m_sfw, m_nsfw;
	function toggleMaturity(btn) {
		if(btn!==m_kid){m_kid.select(false)}
		if(btn!==m_std){m_std.select(false)}
		if(btn!==m_sfw){m_sfw.select(false)}
		if(btn!==m_nsfw){m_nsfw.select(false)}
		showTagModal_types=(m_sfw.is_selected() || m_nsfw.is_selected());
	}

	function submit_success(e){
		return async({result, update}) => {
			await update();
			// Do post-processing
			if(form?.success) {
				showTagModal=false;
				showTagModal_types=false;
				showReportModal=false;
				showReportAnonModal=false;

				showThanksModal=true;
				new Promise(f=>setTimeout(f,5000)).then(()=>{showThanksModal=false});
			}
		}
	}
</script>

<div>
	<!-- Vote Bar -->
	<!-- Only logged-in users get to vote on posts -->
	<div id='vote' class='btngrp'>
		{#if user_type==='u'}
			<form method='POST' use:enhance style='display:flex'>
				{#await my_vote}
					<Button img='/vote_up.svg' type='submit' formaction='?/upvote' />
				{:then my_vote}
					<Button img='/vote_up.svg' type='submit' formaction='?/upvote' class={my_vote===1?'tbtn sel':''} />
				{/await}

				<span id='count'>
					{#await votes}&nbsp;{:then votes}<span id='count'>{votes}</span>{/await}
				</span>

				{#await my_vote}
					<Button img='/vote_dn.svg' type='submit' formaction='?/dnvote' />
				{:then my_vote}
					<Button img='/vote_dn.svg' type='submit' formaction='?/dnvote' class={my_vote===5?'tbtn sel':''} />
				{/await}
			</form>
		
		{:else if user_type==='a'}
			<Button img='/vote_up.svg' disabled title='Log in to vote' />
			<span id='count'>
				{#await votes}&nbsp;{:then votes}<span id='count'>{votes}</span>{/await}
			</span>
			<Button img='/vote_dn.svg' disabled title='Log in to vote' />
		
		{:else if user_type==='m' && post.is_public }
			<!-- TODO: Show fancier stats, b/c its your post -->
			<Button img='/vote_up.svg' disabled />
			<span id='count'>
				{#await votes}&nbsp;{:then votes}<span id='count'>{votes}</span>{/await}
			</span>
			<Button img='/vote_dn.svg' disabled />
		{/if}
	</div>

	<!-- Action Bar -->
	<!-- If User is original poster -->
	{#if user_type === 'm'}
		<div class='btngrp'>
			<form method='POST' action='?/public' use:enhance style='display:flex'>
				{#if post.is_public}
					<Button img='/chat.svg' lbl='Comment' onclick={()=>{reply_to=0}} />
					<Button img='/remove.svg' lbl='Remove post' type='submit' />
				{:else}
					<Button img='/add.svg' lbl='Make public' type='submit' />
				{/if}
				<Button img='/share.svg' lbl='Link' onclick={copy_link} />
				<Button img='/politics.svg' lbl='Tag' onclick={()=>{showTagModal=true}} />
			</form>
		</div>

	<!-- If User is ImgCat user -->
	{:else if user_type==='u'}
		<div class='btngrp'>
			<form method='POST' action='?/fav' use:enhance style='display:flex'>
				<Button img='/chat.svg' lbl='Comment' onclick={()=>{reply_to=0}} />
				{#await is_fav}
				<Button img='/star_off.svg' lbl='Fav' type='submit' />
				{:then is_fav}
				<Button img='/star_{is_fav?'on':'off'}.svg' lbl='Fav' type='submit' class={is_fav?'tbtn sel':''} />
				{/await}
				<Button img='/share.svg' lbl='Link' onclick={copy_link} />
				<Button img='/politics.svg' lbl='Tag' onclick={()=>{showTagModal=true}} />
			</form>
		</div>
		<!-- Separated button group to visually indicate that Report actions are different -->
		<Button img='/report.svg' lbl='Report' onclick={()=>{showReportModal=true}} />
	
	<!-- If User is random user from the interwebs -->
	{:else}
		<span id='actions'>
			<Button img='/share.svg' lbl='Link' onclick={copy_link} />
			<Button img='/report.svg' lbl='Report' onclick={()=>{showReportAnonModal=true}} />
		</span>
	{/if}
</div>


<Modal id='tag' bind:showModal={showTagModal} title='Help us understand this post'>
	<form method='POST' action='?/tag' use:enhance={submit_success} >
		<h4>Maturity</h4>
		<div id='tag_maturity' class='btngrp'>
			<ToggleButton bind:this={m_kid} img='/content_g.svg' lbl='Kid-safe' name='maturity' value='kid' onclick={()=>toggleMaturity(m_kid)} />
			<ToggleButton bind:this={m_std} img='/content_std.svg' lbl='Normal' name='maturity' value='std' onclick={()=>toggleMaturity(m_std)} />
			<ToggleButton bind:this={m_sfw} img='/content_r.svg' lbl='Mature' name='maturity' value='sfw' onclick={()=>toggleMaturity(m_sfw)} />
			<ToggleButton bind:this={m_nsfw} img='/content_x.svg' lbl='NSFW' name='maturity' value='nsfw' onclick={()=>toggleMaturity(m_nsfw)} style={ showTagModal_types?'border-bottom-right-radius:0px':null } />
		</div>
		{#if showTagModal_types}
		<br/>
		<div id='tag_type' class='btngrp'>
			<ToggleButton img='/content_x.svg' lbl='Is sexual' name='is_sexual' />
			<ToggleButton img='/vote_troll.svg' lbl='Is violence' name='is_gore' />
			<ToggleButton img='/vote_heart.svg' lbl='Is trauma' name='is_trauma' />
		</div>
		{/if}

		<h4>Categories</h4>
		<div id='tag_category' class='btngrp'>
			<ToggleButton img='/pawprint.svg' lbl='Animals' name='cat_animals' />
			<ToggleButton img='/artist.svg' lbl='Artists' name='cat_artists' />
			<ToggleButton img='/lips.svg' lbl='Selfies' name='cat_selfies' />
			<ToggleButton img='/news.svg' lbl='News' name='cat_news' />
			<ToggleButton img='/politics.svg' lbl='Politics' name='cat_politics' />
			<ToggleButton img='/robot.svg' lbl='AI' name='cat_ai' />
		</div>

		<h4>Suggest Tags</h4>
		<div id='tag_tags' class='btngrp'>
			<!-- TODO: Use TAG_REGEX to do live validation & immediate feedback -->
			<input type='text' name='tags' placeholder='ie. Cats, Orange-energy, Cat fail video'>
			<Button lbl='Submit' type='submit' />
		</div>
		<br />

		<!-- A spacer, so expanding the mature/nsfw types doesn't mess with modal height -->
		{#if !showTagModal_types}<br style='line-height:calc(2.5em - 5px)'/>{/if}

		{#if form?.form==='tag' && !form.success}
			<br />
			<div class='error'>
				{#if form.message}
					{form.message}
				{:else}
					We're sorry, but there was a system problem
				{/if}
			</div>
		{/if}

		<br />
		<Button lbl='Cancel' onclick={(e)=>{showTagModal=false}} style='color:var(--cttxt)' />
	</form>
</Modal>

<Modal id='report' bind:showModal={showReportModal} title='Mute, block, or report'>
	<form method='POST' action='?/report' use:enhance={submit_success} >
		<h4>Self-service actions</h4>
		<Button img='/visibility_off.svg' lbl='Hide this post' formaction='?/mute' type='submit' />
		<Button img='/recent.svg' lbl='Snooze this user (2wks)' formaction='?/snooze' type='submit' />
		<Button img='/block_user.svg' lbl='Block this user (permanent)' formaction='?/block' type='submit' />

		<br/><br/>
		<h4>Report to a Moderator</h4>
		<textarea name='comment' placeholder='Comments'></textarea>
		<Button img='/content_illegal.svg' lbl='Report' type='submit' />
		<br />

		{#if form?.form==='report' && !form.success}
			<br />
			<div class='error'>
				{#if form.message}
					{form.message}
				{:else}
					We're sorry, but there was a system problem
				{/if}
			</div>
		{/if}

		<br />
		<Button lbl='Cancel' onclick={(e)=>{showReportModal=false}} style='color:var(--cttxt)' />
	</form>
</Modal>

<Modal id='report_anon' bind:showModal={showReportAnonModal} title='Report to a Moderator'>
	<form method='POST' action='?/report_anon' use:enhance={submit_success} >
		<textarea name='comment' placeholder='Comments'></textarea>
		<Button img='/content_illegal.svg' lbl='Report' type='submit' />
		<br />

		{#if form?.form==='report_anon' && !form.success}
			<br />
			<div class='error'>
				{#if form.message}
					{form.message}
				{:else}
					We're sorry, but there was a system problem
				{/if}
			</div>
		{/if}

		<br />
		<Button lbl='Cancel' onclick={(e)=>{showReportAnonModal=false}} style='color:var(--cttxt)' />
	</form>
</Modal>

<Modal bind:showModal={showThanksModal} title='Thank you!' >
	<p>Thank you! Tagging and reporting content helps the ImgCat community</p>

	<br /><br />
	<Button lbl='Close' onclick={(e)=>{showThanksModal=false}} style='color:var(--cttxt)' />
</Modal>


<style>
	div#vote span#count {
		display: inline-block;
		min-width: 3em;
		height: 1em;
		line-height: 1em;
		text-align: center;
		margin: auto;
	}
	div.error {
		color: red;
		work-break: break-word;
	}
	:global {
		dialog button,input {
			height: 3em;
		}
		dialog#tag {
			div#tag_maturity button {
				width: 12em;
			}
			div#tag_type {
				/* TODO: Do a flex grid */
				margin-left: calc(12em - 22px);
				button {
					height: 2.5em;
					width: 12em;
					&:first-of-type {
						border-top-left-radius: 0px;
						border-bottom-left-radius: 3em;
					}
					&:last-of-type {
						border-top-right-radius: 0px;
						border-bottom-right-radius: 3em;
					}
				}	
			}
			div#tag_category button {
				width: 8em;
			}
			div#tag_tags {
				width: 100%;
				input {
					display: inline-block;
					width: calc(100% - 105px);
					margin-right:0px;
					border-top-right-radius: 0px;
					border-bottom-right-radius: 0px;
				}
				button {
					display: inline-block;
					width: 100px;
					margin-left:0px;
					border-top-left-radius: 0px;
					border-bottom-left-radius: 0px;
				}
			}
		}
		dialog#report, dialog#report_anon {
			button,input,textarea {
				display: block !important;
				width: 20em;
			}
			div.error {
				/* TODO: Weird padding thing */
				width: 17em;
			}
		}
	}
</style>