<script lang="ts">
	let { logged_in	} = $props();
</script>

<nav><div id="l">
	<img src="/logo.png" alt="Welcome to ImgCat" fetchpriority="high"/>	
</div><div id="m">
	<a href="/home"><img src="/home.svg" alt="Home"/></a>
	<a href="/viral"><img src="/viral.svg" alt="Viral posts"/></a>
	{#if logged_in}
	<a href="/recent"><img src="/recent.svg" alt="Recent posts"/></a>
	
	{/if}
	<input type="search" placeholder="Search for #tags and @users"/>
	{#if logged_in}
	<a href="/post"><img src="/post.svg" height="42px" alt="Upload and post an image"/></a>
	<!--<a href="/meme"><img src="/meme.svg" height="42px" alt="Build a meme"/></a>-->
	{/if}
</div><div id="r">
	{#if logged_in}
	<!--<a href="/chat"><img src="/chat.svg" height="42px" alt="Chat"/></a>-->
	<a href="/myfavs"><img src="/myfavs.svg" alt="My favorites"/></a>
	<a href="/myposts"><img src="/myposts.svg" alt="My posts"/></a>
	<a href="/profile"><img src="/profile.svg" height="42px" alt="Profile"/></a>
	{:else}
	<a href="/login"><img src="/profile.svg" height="42px" alt="Log in"/></a>
	{/if}
</div></nav>

<style>
	nav {
		position: sticky;
		top: 0px;
		background-color: var(--cb4);

		display: grid;
		grid-template-columns: repeat(5, 1fr);

		a:hover {
			/* #B8D2E3 or --cacold */
			filter: brightness(0) saturate(100%) invert(68%) sepia(7%) saturate(7494%) hue-rotate(175deg) brightness(86%) contrast(91%);
		}

		div#m img,div#r img {
			/* #F5F5F5 or --cttxt */
			/*filter: brightness(0) saturate(100%) invert(98%) sepia(3%) saturate(85%) hue-rotate(114deg) brightness(117%) contrast(92%);*/
			/* #B8D2E3 or --cticy */
			/*filter: brightness(0) saturate(100%) invert(89%) sepia(9%) saturate(751%) hue-rotate(171deg) brightness(93%) contrast(90%);*/
			/* #87B3CF or --ctlnk */
			filter: brightness(0) saturate(100%) invert(69%) sepia(63%) saturate(183%) hue-rotate(160deg) brightness(88%) contrast(91%);
		}

		div#l, div#m, div#r {
			display: flex;
		}

		div#l {
			grid-column-start: 1;
			grid-column-end: 2;
			justify-content: left;

			img {
				height: 75px;
			}
		}
		div#m {
			grid-column-start: 2;
			grid-column-end: 5;
			justify-content: left;
		}
		div#r {
			grid-column-start: 5;
			grid-column-end: 6;
			justify-content: right;
		}

		div {
			* {
				display: inline-block;
				height: 48px;
				margin: auto 0px;
			}
			a {
				padding: 10px;
			}
			input {
				padding: 5px 10px;
				width: 400px;
			}
			img {
				vertical-align: middle;
			}
		}
	}
</style>