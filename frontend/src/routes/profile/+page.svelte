<script lang='ts'>
	import type { PageProps } from './$types';
	import Button from '$lib/Button.svelte';

	let { data, form }: PageProps = $props();

	let selected_maturity = $state(data.prefs.content_level[0]);
</script>

{#snippet secured_option(data, label, value, disabled)}
	{#if data[1] >= value}
		{#if !disabled}
			{#if data[0] == value}
				<option value={value} selected>{label}</option>
			{:else}
				<option value={value}>{label}</option>
			{/if}
		{:else}
			{#if data[0] == value}
				<option value={value} disabled selected>{label}</option>
			{:else}
				<option value={value} disabled>{label}</option>
			{/if}
		{/if}
	{/if}
{/snippet}

{#snippet weight_option(data, label, value)}
	{#if data == value}
		<option value={value} selected>{label}</option>
	{:else}
		<option value={value}>{label}</option>
	{/if}
{/snippet}

{#snippet weight_dd(data)}
	{@render weight_option(data, 'Much more', 8)}
	{@render weight_option(data, 'More', 7)}
	<!-- {@render weight_option(data, 'Little more', 6)} Removed to simplify UI/UX -->
	{@render weight_option(data, 'Normal', 5)}
	<!-- {@render weight_option(data, 'Little less', 4)} Removed to simplify UI/UX -->
	{@render weight_option(data, 'Less', 3)}
	{@render weight_option(data, 'Much less', 2)}
	{@render weight_option(data, '- None -', 0)}
{/snippet}

<h1>Profile</h1>

<div id='about' class='section'>
	<h2>About Me</h2>
	<p>Introduce yourself to the community</p>

	<form method="POST" action='?/save'>
		<div>
			<label>
				Username
				<input name='username' type='text' value={data.prefs.username} autocomplete='off' disabled />
			</label>
		</div><div>
			<label>
				About me
				<textarea name='about_me' placeholder='Tell the ImgCat community about you!'>{data.prefs.about_me}</textarea>
			</label>
		</div>

		<Button class='sbutton' lbl='Save changes' />
	</form>
</div>

<div id='content' class='section'>
	<h2>Content Settings</h2>
	<p>How much "spiciness" do you want?</p>

	<form method="POST" action='?/save'>
		<div>
			<label>Maturity level
				<select name='content_level' bind:value={selected_maturity}>
					{@render secured_option(data.prefs.content_level, 'Kid-friendly', 1)}
					{@render secured_option(data.prefs.content_level, 'Standard', 2)}
					{@render secured_option(data.prefs.content_level, 'Mature (Safe For Work)', 3)}
					{@render secured_option(data.prefs.content_level, 'Mature (Not Safe For Work)', 4)}
				</select>
			</label>
		</div>
		{#if selected_maturity >= 2}
			<div id='maturityss'>
				{#if selected_maturity >= 3}
					<div>
						<label>Lewd or sexual
							<select name='see_sexuality'>
								{@render secured_option(data.prefs.see_sexuality, 'Allowed', true)}
								{@render secured_option(data.prefs.see_sexuality, 'Block', false)}
							</select>
						</label>
					</div>
				{:else}
					<input type='hidden' name='see_sexuality' value='false' />
				{/if}

				{#if selected_maturity >= 3}
					<div>
						<label>Violence or gore
							<select name='see_gore'>
								{@render secured_option(data.prefs.see_gore, 'Allowed', true)}
								{@render secured_option(data.prefs.see_gore, 'Block', false)}
							</select>
						</label>
					</div>
				{:else}
					<input type='hidden' name='see_gore' value='false' />
				{/if}

				{#if selected_maturity >= 2}
					<div>
						<label>Emotional or traumatic
							<select name='see_trauma'>
								{@render secured_option(data.prefs.see_trauma, 'Allowed', true)}
								{@render secured_option(data.prefs.see_trauma, 'Block', false)}
							</select>
						</label>
					</div>
				{:else}
					<input type='hidden' name='see_trauma' value='false' />
				{/if}
			</div>
		{:else}
			<input type='hidden' name='see_sexuality' value='false' />
			<input type='hidden' name='see_gore' value='false' />
			<input type='hidden' name='see_trauma' value='false' />
		{/if}
	
		<br/>
		<h2>Content Weights</h2>
		<p>What do you want to see?</p>
	
		<div>
			<label>News and current events
				<select name='news_weight'>
					{@render weight_dd(data.prefs.news_weight)}
				</select>
			</label>
		</div><div>
			<label>Politics and opinions
				<select name='politics_weight'>
					{@render weight_dd(data.prefs.politics_weight)}
				</select>
			</label>
		</div><div>
			<label>Artists, creators, and original content
				<select name='creators_weight'>
					{@render weight_dd(data.prefs.creators_weight)}
				</select>
			</label>
		</div><div>
			<label>Selfies, body pics, workout progress
				<select name='selfies_weight'>
					{@render weight_dd(data.prefs.selfies_weight)}
				</select>
			</label>
		</div><div>
			<label>Animals, pets, fuzzy friends
				<select name='pets_weight'>
					{@render weight_dd(data.prefs.pets_weight)}
				</select>
			</label>
		</div><div>
			<label>Generative AI
				<select name='ai_weight'>
					{@render weight_dd(data.prefs.ai_weight)}
				</select>
			</label>
		</div>

		<Button class='sbutton' lbl='Save changes' />
	</form>
</div>

<div id='privacy' class='section'>
	<h2>Privacy Settings</h2>
	<p>What can others see about you?</p>

	<form method="POST" action='?/save'>
		<div>
			<label>
				"About me" bio
				<select name='about_me_visibility'>
					{@render secured_option(data.prefs.about_me_visibility, 'Global', 9)}
					{@render secured_option(data.prefs.about_me_visibility, 'ImgCat community', 5)}
					{@render secured_option(data.prefs.about_me_visibility, 'Friends only', 4)}
					{@render secured_option(data.prefs.about_me_visibility, 'Private', 1)}
				</select>
			</label>
		</div>
		<div>
			<label>
				Post and comment history
				<select name='activity_visibility'>
					{@render secured_option(data.prefs.activity_visibility, 'ImgCat community', 5)}
					{@render secured_option(data.prefs.activity_visibility, 'Friends only', 4)}
					{@render secured_option(data.prefs.activity_visibility, 'Private', 1)}
				</select>
			</label>
		</div>
		<!-- NOTE: We won't have DMs on launch
		<div>
			<label>
				Who can send you DMs?
				<select name='dm_visibility'>
					{@render secured_option(data.prefs.dm_visibility, 'ImgCat community', 5)}
					{@render secured_option(data.prefs.dm_visibility, 'Friends only', 4)}
					{@render secured_option(data.prefs.dm_visibility, 'Private', 1)}
				</select>
			</label>
		</div>
		-->

		<Button class='sbutton' lbl='Save changes' />
	</form>
</div>

<div id='blocked' class='section'>
	<h2>Blocked Users</h2>
	<p>Which frustrations can we stop?</p>


	<p>- TODO - WORK IN PROGRESS</p>


	<form method="POST" action='?/save'>
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

		<Button class='sbutton' lbl='Save changes' />
	</form>
</div>


<div id='tools' class='section'>
	<h2>Actions</h2>
	<form method="POST" action='?/logout'>
		<Button lbl='Log out' />
	</form>
</div>

<style>
	div.section {
		margin-top: 10px;
		border: 1px solid var(--cb1);
		padding: 10px;

		label, input, select, textarea {
			display: inline-block;
			width: 25em;
			box-sizing: border-box;
		}
		textarea {
			height: 6em;
		}
		label {
			margin-top: 1em;
		}

		p {
			color: var(--cticy);
		}
	}

	:global(.sbutton) {
		margin-top: 2em;
	}

	div#maturityss {
		margin-left: 2em;
		label, input, select, textarea {
			width: 22em;
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