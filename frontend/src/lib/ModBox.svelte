<script lang='ts'>
	import type { PageLoad } from './$types';
	import Button from '$lib/Button.svelte';
	import Modal from '$lib/Modal.svelte';
	import { voteMature, voteTag, pingMod } from '$lib/ModBox.remote.ts';
	//import { query } from '$app/server';

	const {
		post,
		user_id
	} = $props();

	let hidePostModal = $state(false);
	let blockUserModal = $state(false);
	let tagModal = $state(false);
	let matureModal = $state(false);
	let pingModsModal = $state(false);
</script>

<div id='modbox'>
	{#if user_id == post.user_id}
		<!-- Your post -->
	{:else if user_id}
		<!-- Authenticated user actions -->
		<Button lbl='Hide post' img='/visibility_off.svg' onclick={()=>{hidePostModal=true}} />
		<Button lbl='Block user' img='/block_user.svg' onclick={()=>{blockUserModal=true}} />
		<Button lbl='Add filter/tag' img='/politics.svg' onclick={()=>{tagModal=true}}/>
		<Button lbl='Mark mature' img='/content_r.svg' onclick={()=>{matureModal=true}}/>
		<Button lbl='Flag for moderators' img='/report.svg' onclick={()=>{pingModsModal=true}} />
	{:else}
		<!-- Anonymous user actions -->
		<Button lbl='Flag for moderators' img='/report.svg' onclick={()=>{pingModsModal=true}} />
	{/if}
</div>

<!-- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -->
<!--  TODO: Move these modals into their own files.  -->
<!-- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -->
<Modal bind:showModal={hidePostModal} title='Hide this post'>
	<form>
		<p>Under Construction</p>
		<input type='button' value='Close' onclick={(e)=>{hidePostModal=false}}/>
	</form>
</Modal>


<Modal bind:showModal={blockUserModal} title='Block this user'>
	<form>
		<p>Under Construction</p>
		<label>
			<input type='radio' name='duration' value='1mo' checked />
			Snooze this user for 1 month
		</label>

		<br />

		<label>
			<input type='radio' name='duration' value='perm' />
			Block this user permenantly
		</label>

		<br /><br />
		<input type='button' value='Close' onclick={(e)=>{blockUserModal=false}}/>
		<input type='submit' value='Submit' />
	</form>
</Modal>


<Modal bind:showModal={matureModal} title='Mark as mature'>
	<form {...voteMature} onsubmit={(e)=>{matureModal=false}}>
		<input type='hidden' name='post_id' value='{post.id}' />
		
		<!-- Replace these with themed controls -->
		<div>
			<label>
				<input {...voteMature.fields.mature.as('radio', 'kid')} autofocus />
				<img src='/content_g.svg'/>
				Kid-safe
			</label>
			<label>
				<input {...voteMature.fields.mature.as('radio', 'std')} />
				<img src='/content_std.svg'/>
				Normal
			</label>
			<label>
				<input {...voteMature.fields.mature.as('radio', '18+')} />
				<img src='/content_r.svg'/>
				Spicy
			</label>
			<label>
				<input {...voteMature.fields.mature.as('radio', 'xxx')} />
				<img src='/content_x.svg'/>
				Not safe for work
			</label>
			<label>
				<input {...voteMature.fields.mature.as('radio', 'ill')} />
				<img src='/content_illegal.svg'/>
				Illegal
			</label>
		</div>

		{#if ['18+', 'xxx', 'ill'].includes(voteMature.fields.mature.content.value())}
			<!-- Replace these with themed controls -->
			<div>
				<label>
					<input {...voteMature.fields.is_sexual.as('checkbox')} />
					Sexual Content
				</label>
				<label>
					<input {...voteMature.fields.is_gore.as('checkbox')} />
					Gross/gore
				</label>
				<label>
					<input {...voteMature.fields.is_trauma.as('checkbox')} />
					Emotional/triggering
				</label>
			</div>
		{/if}

		<br />
		<input type='button' value='Cancel' onclick={(e)=>{matureModal=false}}/>
		<input type='submit' value='Submit' />
	</form>
</Modal>


<Modal bind:showModal={tagModal} title='Add missing tags'>
	<form {...voteTag} onsubmit={(e)=>{tagModal=false}}>
		<input type='hidden' name='post_id' value='{post.id}' />
		
		<!-- Replace these with themed controls -->
		<div>
			<label>
				<input {...voteTag.fields.is_politics.as('checkbox')} />
				<img src='/politics.svg' />
				News & Politics
			</label>
			<label>
				<input {...voteTag.fields.is_ttrap.as('checkbox')} />
				<img src='/lips.svg' />
				Thirst Traps & Selfies
			</label>
			<label>
				<input {...voteTag.fields.is_creator.as('checkbox')} />
				<img src='/artist.svg' />
				Creator Content
			</label>

		</div>

		<br />

		<p>Under Construction</p>
		<div id='tags'>
			<label>Tags</label>
			<Button lbl='science' />
			<Button lbl='space' />
			<Button lbl='black holes' />
			<br />
			<input />
			<Button lbl='add' />
		</div>
		<input type='text' name='science' value='science' />
		<input type='text' name='space' value='space' />

		<br />
		<input type='button' value='Cancel' onclick={(e)=>{tagModal=false}}/>
		<input type='submit' value='Submit' />
	</form>
</Modal>


<Modal bind:showModal={pingModsModal} title='Flag for moderators'>
	<form {...pingMod} onsubmit={(e)=>{pingModsModal=false}}>
		<input type='hidden' name='post_id' value='{post.id}' />
		<label>
			Comments:
			<textarea name='comment' minlength='10' maxlength='200' required autofocus style='width:100%;box-sizing:border-box;'></textarea>
		</label>

		<br /><br />
		<input type='button' value='Cancel' onclick={(e)=>{pingModsModal=false}}/>
		<input type='submit' value='Submit' />
	</form>
</Modal>


<style>
	div#modbox {
		margin: 1em 0em;
	}
</style>