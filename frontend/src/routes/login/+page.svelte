<script lang='ts'>
	import type { ActionData, PageProps } from './$types';
	import { enhance } from '$app/forms';
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
	<Button href='/auth/p/google' lbl='Sign in with Google' />
</div>

<br/>
<div id='faqs'>
	<h3>FAQs</h3>
	<ul>
		<li>
			<p class="q" onclick={show_answer}>What is the vibe of this site?</p>
			<p class="a hide">We're an image-board community, so every post requires an image. We have a meme-maker if you can't think of one. Our main page is shared among all users, and sorted by your votes. The goal is to build a more friendly version of social media, built by users, without any toxic engagement models.
			</p>
		</li><li>
			<p class="q" onclick={show_answer}>How do I log in?</p>
			<p class="a hide">We use a secure login system called OpenID. We don't store your email/password, the provider you choose handles the security, and we just handle the memes.</p>
		</li><li>
			<p class="q" onclick={show_answer}>How do I create a new account?</p>
			<p class="a hide">Choose a provider to sign in, and we'll create a new account for you.</p>
		</li><li>
			<p class="q" onclick={show_answer}>How does OpenID work?</p>
			<p class="a hide">OpenID is a business technology allowing your login to work across multiple sites (ie. to access your HR or IT portal). We use it here too. The provider handles the security, then sends us a note: "I promise this user is ID# 987654". That's all we need to sign you in, providing better data security and privacy than an email and password. Click if you'd like to <a href='https://openid.net/developers/how-connect-works/'>learn more</a>.</p>
		</li><li>
			<p class="q" onclick={show_answer}>Do you perform age verification?</p>
			<p class="a hide">
				The law requires it in some countries. In those cases, we use OpenID to ask for both the user-id and age, because the account knows if it was set up with age-restrictions. This is more accurate, harder to defeat, and protects everyone's privacy better than scanning government IDs or selfies.
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