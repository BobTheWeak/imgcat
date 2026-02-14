<script lang="ts">
	import Navbar from "$lib/Navbar.svelte";

	let { children, data } = $props();
</script>

<Navbar logged_in="{data.logged_in}"/>

<!--
<div id="adpanel">
	<h3>Future Ad Banner</h3>
	<p>Help feed the server-hamsters</p>
</div>
-->
<div id="content">
{@render children()}
</div>

<footer>
	<span>&copy;2025</span>
	<a href="/about">About</a>
	<a href="/about/terms">Terms and Conditions</a>
	<a href="/about/privacy">Privacy Policy</a>
	<a href="/about/cookies">Cookie Policy</a>
</footer>

<style>
	footer {
		position: relative;
		bottom: 0px;
		background-color: var(--cbk1)/*#EEE*/;
		height: 40px;
		width: 100%;
		align-content: center;
		text-align: center;

		a,span {
			margin: 0em 0.5em;
			padding: 5px;
		}
	}
	#content {
		max-width: 950px;
		margin: 10px;
	}
	#adpanel {
		float:right;
		width: 300px;
		height: 600px;
		margin: 100px 10px 10px 10px;
		border: 1px solid black;
		padding: 10px;
	}
</style>