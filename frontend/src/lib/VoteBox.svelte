<script lang='ts'>
	import Button from '$lib/Button.svelte';
	import { getViewVotes, getMyVote, setMyVote } from '$lib/VoteBox.remote.ts';

	const { post, user_id } = $props();
	let viewvote = $state(null);

	getViewVotes(post.id).then((v)=>{
		viewvote = v;
	})

	let myvote = $state(0);
	if(user_id) {
		getMyVote([post.id, user_id]).then((v)=>{
			myvote = v;
		});
	}

	async function clkVote(e) {
		let v = parseInt(e.currentTarget?.value);

		// Unselect a vote if that's what we mean
		if(v==myvote){v=0}

		setMyVote([post.id, user_id, v])
		.then((new_vote)=>{
			viewvote.vote_total = new_vote;
			myvote = v;
		});
	}

</script>

<div id='votebox'>
	{#if post.is_public}
		<span>{viewvote?.view_total || 0}</span> views
		<span>{viewvote?.vote_total || 0}</span> votes
		{#if user_id}
			<div class='up'>
				<Button img='/vote_up.svg' title='Normal upvote' class={myvote==1?"sel":""} value='1' onclick={clkVote} />
				<!--
				<Button img='/vote_happy.svg' title='Upvote & happy/excited' class={myvote==2?"sel":""} value='2' onclick={clkVote} --bkg='#EFE' --bdr='#8F8' --bkg-sel='#8F8' --bdr-sel='#888' />
				<Button img='/vote_heart.svg' title='Upvote & love/sympathy' class={myvote==3?"sel":""} value='3' onclick={clkVote} --bkg='#EFE' --bdr='#8F8' --bkg-sel='#8F8' --bdr-sel='#888' />
				<Button img='/vote_quality.svg' title='Upvote & top quality' class={myvote==4?"sel":""} value='4' onclick={clkVote} --bkg='#EFE' --bdr='#8F8' --bkg-sel='#8F8' --bdr-sel='#888' />
				-->
			</div>

			<div class='dn'>
				<Button img='/vote_dn.svg' title='Normal downvote' class={myvote==5?"sel":""} value='5' onclick={clkVote} />
				<!--
				<Button img='/vote_what.svg' title='What is this?' class={myvote==6?"sel":""} value='6' onclick={clkVote} --bkg='#FEE' --bdr='#F88' --bkg-sel='#F88' --bdr-sel='#888' />
				<Button img='/vote_spam.svg' title='This is spam' class={myvote==7?"sel":""} value='7' onclick={clkVote} --bkg='#FEE' --bdr='#F88' --bkg-sel='#F88' --bdr-sel='#888' />
				<Button img='/vote_troll.svg' title='Troll food' class={myvote==8?"sel":""} value='8' onclick={clkVote} --bkg='#FEE' --bdr='#F88' --bkg-sel='#F88' --bdr-sel='#888' />
				-->
			</div>
		{/if}
	{/if}
</div>

<style>
	#votebox {
		padding: 5px 0px;
	}
	span {
		
	}
	div#votebox div {
		display: inline-block;
		margin-left: 15px;
		img {
			display: inline-block;
			vertical-align: middle;
			border-width: 1px;
			border-style: solid;
			padding: 5px 5px;
			height:30px;
			width:30px;
		}
	}
	div#votebox div.up {
		img {
			/*background-color: #EFE;*/
			border-color: #8F8;
		}
		img.v {
			background-color: #8F8;
		}
	}
	div#votebox div.dn {
		img {
			/*background-color: #FEE;*/
			border-color: #F88;
		}
		img.v {
			background-color: #F88;
		}
	}

	/*
	a {
		padding: 0px 10px;
		border: 1px solid #888;
		min-width: 50px;
		height: 30px;
		vertical-align: middle;
		box-sizing: content-box;

		img {
			vertical-align: middle;
		}
	}
	a.up {
		background-color: #EFE;
		border-color: #484;
	}
	a.up.me {
		background-color: #8F8;
	}
	a.dn {
		background-color: #FEE;
		border-color: #844;
	}
	a.dn.me {
		background-color: #F88;
	}
	*/
</style>