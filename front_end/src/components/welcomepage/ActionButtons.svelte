<script>
	import { createEventDispatcher } from "svelte";

	let playlistId = "";
	const dispatch = createEventDispatcher();

	function handleSubmit() {
		dispatch("playlist-link", {
			link: playlistId,
		});
	}
</script>

<div id="action-buttons">
	<form on:submit|preventDefault={handleSubmit}>
		<input
			class="playlist-input"
			bind:value={playlistId}
			placeholder="Paste link to a playlist"
		/><input class="playlist-submit" type="submit" value="Go" />
	</form>
	<p>or</p>
	<div class="account-container">
		<a href="/" class="account">
			Select with your <i
				class="fa fa-spotify"
				style="color: #1db954;"
				aria-hidden="true"
			/> account (coming soon)
		</a>
	</div>
</div>

<style>
	#action-buttons {
		display: flex;
		flex-flow: column;
	}

	p {
		margin: 0 15px;
	}

	form {
		display: flex;
		flex-flow: row;
		max-width: 640px;
		margin-left: auto;
		margin-right: auto;
	}

	.playlist-input {
		border-radius: 25px 0 0 25px;
		width: 75%;
	}
	.playlist-submit {
		border-radius: 0 25px 25px 0;
		width: 15%;
	}

	input,
	p {
		font-size: 1.66rem;
		font-weight: 600;
	}
	.playlist-input {
		color: #222;
		background-color: #fff;
		padding: 20px;
	}

	.account-container {
		max-width: 400px;
		margin: 0 auto;
		background-color: white;
		border-radius: 25px;
		padding: 20px;
	}

	.account {
		display: inline-block;
		font-size: 1.66rem;
		text-decoration: none;
		color: #333;
		word-wrap: break-word;
	}
</style>
