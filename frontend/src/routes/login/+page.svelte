<script lang='ts'>
	import type { ActionData, PageProps } from './$types';
	import { enhance } from '$app/forms';
	import { invalidateAll } from '$app/navigation';
	import Button from '$lib/Button.svelte';
	import { env } from '$env/dynamic/public';

	let { data, form }: PageProps = $props();


	//const error_msg = $derived.by(()=>{
	//	if(document) {
	//		new URL(document?.URL).searchParams().get('error_msg')
	//	}
	//});

	function show_answer(e) {
		let e2 = e.target.nextElementSibling;
		if(e2.classList.contains("hide")) {
			e2.classList.remove("hide");
		} else {
			e2.classList.add("hide");
		}
	}

	async function submit_private_key(e) {
		const k = document.getElementById('private_server_key');
		if(k?.value) {
			let cval = await cookieStore.get('ic_private');
			if(cval) {
				cval = cval.value + '|' + k?.value;
			} else {
				cval = k?.value;
			}
			console.log(cval);
			cookieStore.set({
				name: 'ic_private',
				value: cval,
				maxAge: 31536000
			});
		}
		invalidateAll();
	}

</script>

<h2>Log In &amp; Sign up</h2>
<p>ImgCat is an image-board community, run by a nonprofit. We don't sell your data, and don't use toxic engagement models. We're trying to create a better, friendlier, more social web.</p>
<!--
<form method="POST" action='?/user_pass' use:enhance={() => {
		return async ({ update }) => {
			return await update({reset:false});
		};
	}}>

	<label for='email'>Email or Username</label>
	{#if form?.email}
	<input type='text' name='email' required placeholder='user@example.com' value={form?.email ?? ''}/>
	{:else}
	<input type='text' name='email' required placeholder='user@example.com'/>
	{/if}

	<label for='password'>Password</label>
	<input type='password' name='password' required />

	{#if form?.success === false}
	<p style="color:red">{form?.message}</p>
	{/if}
	<a href="/signup">Create an account</a>
	<button type='submit'>Login</button>
</form>
-->

<!--<form method='POST' use:enhance >
	<button formaction="?/oauth_google">Sign in with Google</button>
	<button formaction="?/oauth_github">Sign in with GitHub</button>
	
</form>-->
{#if data.error_msg}
<p style='color:red'>Error: {data.error_msg}</p>
{/if}
<div id='providers'>
{#if data.login_allowed}
	<Button href='/auth/p/google' lbl='Sign in with Google' />
	<Button href='/auth/p/microsoft' lbl='Sign in with Microsoft' />
{:else}
<p>This is a private server, and logins are disabled without the special key</p>
<input id='private_server_key' />
<Button lbl='Submit' onclick={submit_private_key} />
{/if}
</div>

<br/>
<div id='faqs'>
	<h3>FAQs</h3>
	<ul>
		<li>
			<p class="q" onclick={show_answer}>What is the vibe of this site?</p>
			<p class="a hide">We're an image-sharing community. To keep it light, every post requires an image or a meme. Our main page is shared and generated from the community's votes, not engagement algorithms. Our content controls are designed to slow or stop the spread of toxic or viral content. We're a non-profit, and we're trying to be different.</p>
		</li><li>
			<p class="q" onclick={show_answer}>How do I log in?</p>
			<p class="a hide">We use a secure login system called OpenID Connect. The provider you choose handles the security, and sends us back an ID number. That's all we need to sign you in, providing better data security and privacy than an email/password.</p>
		</li><li>
			<p class="q" onclick={show_answer}>How do I create a new account?</p>
			<p class="a hide">Choose any provider to sign in, we'll notice you're new, and create a new account for you.</p>
		</li><li>
			<p class="q" onclick={show_answer}>Do you perform age verification?</p>
			<p class="a hide">
				The law requires it in some countries. With your permission, we pull the birthday associated with your login. Adults probably filled it out. Kids probably have an age-restricted family account. Asking your login allows us to verify ages more accurately and privately than alternative methods. We don't believe scanning government IDs or analyzing selfies is safe, reasonable, or appropriate.
			</p>
		</li>
	</ul>
</div>

<style>
	form button {
		padding: 10px 20px;
		cursor: pointer;
	}
	#providers {
		margin: 1em;
		:global(button) {
			padding: 14px 24px;
		}
		:global(button:hover) {
			padding: 13px 23px;
		}
	}
	div#faqs {
		li {
			color: var(--cawarm);

			p.q {
				cursor: pointer;
				color: var(--ctlnk);
			}
			p.a {
				color: var(--cttxt);
				margin-bottom: 1em;
			}
			p.a em {
				color: var(--cahot);
				font-variant: small-caps;
			}
			p.a.hide {
				display: none;
			}
		}
	}
	/*form {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		width: 300px;
		
		* {	
			margin-top: 15px;
		}

		label,p {
			
			grid-column-start: 1;
  			grid-column-end: 4;
		}

		input {
			margin-top: 0px;
			grid-column-start: 1;
  			grid-column-end: 4;
		}

		a {
			grid-column-start: 1;
  			grid-column-end: 3;
		}

		button {
			grid-column-start: 3;
  			grid-column-end: 4;
		}
	}*/
</style>