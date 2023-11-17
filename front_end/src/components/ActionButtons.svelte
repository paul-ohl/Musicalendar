<script>
	let playlistId = "";

	function handleSubmit() {
		alert("I'm going to search spotify for this playlist: " + playlistId);
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

	.account-container {
		margin: 30px;
	}

	.account {
		border-radius: 25px;
		text-decoration-line: none;
	}
	p {
		margin: 0 15px;
	}

	.playlist-input {
		border-radius: 25px 0 0 25px;
	}
	.playlist-submit {
		border-radius: 0 25px 25px 0;
		padding: 20px;
	}

	.account,
	input,
	p {
		font-size: 1.66rem;
		font-weight: 600;
	}
	.account,
	.playlist-input {
		color: #222;
		background-color: #fff;
		padding: 20px;
	}
</style>
