<script lang='ts'>
	let { data } = $props();
</script>

<h1>Profile</h1>

<form>
<div id='account' class='section'>
	<h2>Account</h2>

	<div>
		<label for='username'>Username</label>
		<input name='username' type='text' value='{data.username}' autocomplete='off' />
	</div>
	<div>
		<label for='location'>Location</label>
		<input name='location' type='text' value='{data.location}' autocomplete='off' />
	</div>
	<div>
		<label for='age'>Age</label>
		<input name='age' type='number' value='{data.age}' autocomplete='off' />
	</div>
</div>

<div id='security' class='section'>
	<h2>Security</h2>

	<div>
		<label for='email'>Email address</label>
		<input name='email' type='text' value='{data.email}' />
	</div>
	<div>
		<label for='password'>Change password</label>
		<input name='password' type='password' value='{data.email}' autocomplete='new-password' />
	</div><div>
		
	</div>
</div>

<div id='content' class='section'>
	<h2>Content Settings</h2>
	<div>
		<label for='content-category'>Allowed content</label>
		<select name='content-category'>
			<option value='1'>Kid-friendly</option>
			<option value='2'>Standard</option>
			<option value='3' selected>Lewd or suggestive</option>
			<option value='4' disabled>NSFW content</option>
		</select>
	</div>
	<div>
		<label for='content-messaging'>Enable direct messages</label>
		<select name='content-messaging'>
			<option value='Y' selected>Enabled</option>
			<option value='N'>Disabled</option>
		</select>
	</div>

	<h4>On my viral page, I want to see more or less of the following:</h4>
	<div>
		<label for='content-politics'>Politics</label>
		<select name='content-politics'>
			<option value='more'>More</option>
			<option value='normal' selected>Normal</option>
			<option value='less'>Less</option>
			<option value='none'>Block all</option>
		</select>
	</div>
	<div>
		<label for='content-news'>News</label>
		<select name='content-news'>
			<option value='more'>More</option>
			<option value='normal' selected>Normal</option>
			<option value='less'>Less</option>
			<option value='none'>Block all</option>
		</select>
	</div>
	<div>
		<label for='content-creators'>Creator content</label>
		<select name='content-creators'>
			<option value='more'>More</option>
			<option value='normal' selected>Normal</option>
			<option value='less'>Less</option>
			<option value='none'>Block all</option>
		</select>
	</div>
	<div>
		<label for='content-ai'>AI generated</label>
		<select name='content-ai'>
			<option value='more'>More</option>
			<option value='normal' selected>Normal</option>
			<option value='less'>Less</option>
			<option value='none'>Block all</option>
		</select>
	</div>
	<div>
		<label for='content-ai'>Low quality</label>
		<select name='content-ai'>
			<option value='more'>More</option>
			<option value='normal' selected>Normal</option>
			<option value='less'>Less</option>
			<option value='none'>Block all</option>
		</select>
	</div>
</div>

<div id='blocked' class='section'>
	<h2>Blocked</h2>
	<div>
		<h4>Users</h4>
		<div style='display:flex'>
			<span class='tag'>Testing1<button>X</button></span>
			<span class='tag'>Testing2<button>X</button></span>
			<span class='tag'>Testing3<button>X</button></span>
			{#each data?.blocked_users as user}
			<span class='tag'>{user}<button>X</button></span>
			{/each}
		</div>
		<input /><button>Add</button>
	</div>
	<div>
		<h4>Tags</h4>
		<div style='display:flex'>
			{#each data?.blocked_tags as tag}
			<span class='tag'>{tag}<button>X</button></span>
			{/each}
		</div>
		<input /><button>Add</button>
	</div>
</div>

</form>

<style>
	div.section {
		margin-top: 10px;
		border: 1px solid var(--cb1);
		padding: 10px;

		label {
			display: inline-block;
			width: 250px;
		}
		select {
			width: 200px;
		}
	}

	div#blocked span.tag {
		background-color: var(--cb2);
		padding: 5px 10px;
		margin: 5px 5px 5px 0px;
		border: 1px solid var(--cb1);
		border-radius: 20px;

		button {
			margin-left: 5px;
		}
	}
</style>