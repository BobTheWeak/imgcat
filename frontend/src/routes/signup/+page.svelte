<script lang='ts'>
	import type { ActionData } from './$types';
	import { enhance } from '$app/forms';
	import { GetCountryCodes } from '$lib/country_codes.ts';
	import { IsUsernameFree } from '$lib/is_username_free.remote.ts';
	import Button from '$lib/Button.svelte';
	import { env } from '$env/dynamic/public';

	let { data, form } = $props();

	function unamefree(e) {
		const username = e.target.value;
		IsUsernameFree(username).run().then((x)=>{
			if(typeof x === 'boolean') {
				if(!x) {
					form = {success:false,message:'Username is not available', username: username};
				};
			}
			if(typeof x === 'string') {
				form = {success:false,message:x, username: username};
			}
		}).catch((e)=>{
			form = {success:false,message:'Could not check username', username: username}
		});
	}

</script>

<h2>Signup</h2>

<p>We are excited for you to join the ImgCat community!</p>

<form method="POST" use:enhance={() => {
		return async ({ update }) => {
			return await update({reset:false});
		};
	}}>

	{#if form?.success === false}
	<p style="color:red">{form?.message}</p>
	{/if}

	<label for='username'>Username</label>
	<input type='text' name='username' required value={form?.username || ''} minlength='4' maxlength='40' autocomplete='off' onchange={unamefree}/>

	<!-- TODO: Not supported, at the moment
	At the moment, we're asking for expanded perms on initial login. If we
	know we'll need it later, we might as well ask for everything, once.

	But we should change the flow to ask for the absolute minimum data
	first. Then check here, and see if we need an upgrade to perms. This
	is better transparency, and allows us a chance to explain why & how
	this is better than scanning gov ids or creeping on your selfies.

	{#if data.ck_age}
	<div id='agever'>
		<label for='age'>Age Verification</label>
		<input type='hidden' name='age' value={form?.age} />
		<Button href='/auth/av/google' lbl='Verify with Google' style='width:100%' />
	</div>
	{/if}
	-->

	<label>
		<input type='checkbox' name='toc' />
		I accept the <a href='/about/terms' target="_blank">Terms of Use</a> and the <a href='/about/privacy' target="_blank">Privacy Policy</a>
	</label>

	<div id='cmds'>
		<a href="/login">Back</a>
		<button type='submit'>Create account</button>
	</div>
</form>


<style>
	form {
		width: 300px;

		label, input, select {
			display:block;
			width: 100%;
			box-sizing: border-box;
		}

		input[type='checkbox'] {
			display:inline;
			width: unset;
		}

		label {
			margin-top: 1em;
		}

		input {
			margin-bottom: 1em;
		}
		select {
			margin-top: 0px;
		}

		

		div#agever {
			/* border:1px solid white; */
			/* padding: 1em 0.5em; */
			/* background-color: var(--cb3); */
		}

		div#cmds {
			display: grid;
			grid-template-columns: repeat(4, 75px);
			margin-top: 2em;
			
			a {
				text-align: center;
				grid-column-start: 1;
				grid-column-end: 2;
			}

			button {
				grid-column-start: 3;
				grid-column-end: 5;
				cursor: pointer;
			}
		}
	}
</style>