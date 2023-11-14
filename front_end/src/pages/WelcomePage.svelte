<div id="banner">
	<header class="dark-bg">
		<h1>Musicalendar.</h1>
		<p>
			Your playlist, your timeline: Explore your playlist's journey with
			Musicalendar.
		</p>
	</header>
</div>

<section id="action-buttons">
	<button disabled class="action-button"
		>Select with your account (not yet...)</button
	>
	<p>or</p>
	<form>
		<input class="playlist-input" placeholder="Paste link to a playlist" />
		<input class="action-button" type="submit" value="Go" />
	</form>
</section>

<section id="share">
	<div class="dark-bg">
		<p>Share Musicalendar with your friends and collegues!</p>
		<div class="icon-buttons">
			<a href="/" class="icon-button" id="facebook-icon">
				<img src="../icons/facebook-icon.svg" alt="Share to facebook" />
			</a>
			<a href="/" class="icon-button" id="x-icon">
				<img src="../icons/x-icon.svg" alt="Share to X/Twitter" />
			</a>
			<a href="/" class="icon-button" id="linkedin-icon">
				<img src="../icons/linkedin-icon.svg" alt="Share to LinkedIn" />
			</a>
		</div>
	</div>
</section>

<section id="more-info" class="dark-bg">
	<p>
		<b>Musicalendar</b>: See your Spotify playlist come alive! <br />
		Enter your playlist link, and we'll show you a visual timeline of your music
		journey. Each day reveals the songs added, turning your playlist into a personalized
		musical diary. Explore the evolution of your favorite tunes with Musicalendar.
		<br />
	</p>
	<p>
		<i>A bit of history: <br /></i>
		Musicalendar is a personal project of mine. <br />
		The idea was sparked by my girlfriend having an "audio journal" playlist,
		and wanting a cool way to visualize her year in music. <br /><br />
		My name is Paul, I am a developer from France and you can find me online
		at
		<a href="https://paulohl.fr">paulohl.fr</a>, on
		<a href="https://www.linkedin.com/in/paul-ohl-19aaab196/">LinkedIn</a>,
		and you can also go browse my other projects on
		<a href="https://github.com/paul-ohl">my GitHub</a><br /><br />
		Also this project is open-source so you can checkout the code on
		<a href="https://github.com/paul-ohl/Musicalendar">GitHub</a>!
	</p>
</section>

<style>
	#banner {
		padding: 0;
		background-size: cover;
		background-position: center;
	}

	.dark-bg {
		text-align: center;
		background: rgba(0, 0, 0, 0.2);
		padding: 20px;
		color: #191414;
		margin: 15px auto;
		max-width: 900px;
		border-radius: 5px;
	}

	#action-buttons {
		display: flex;
		justify-content: center;
		align-items: center;
		margin: 20px 0;
	}
	.action-button {
		padding: 15px;
		font-size: 18px;
		border: none;
		border-radius: 5px;
		color: white;
		background-color: #1db954;
		cursor: pointer;
	}
	#action-buttons button:hover {
		background-color: #007700; /* Darker green on hover */
	}

	#action-buttons p {
		font-size: 18px;
		margin: 0 20px;
	}

	form {
		display: flex;
		align-items: center;
	}

	.playlist-input {
		padding: 15px;
		font-size: 18px;
		border: solid 2px #1db954;
		border-radius: 5px;
		margin-right: -5px;
	}

	@media (max-width: 650px) {
		#action-buttons {
			flex-direction: column;
			text-align: center;
		}
	}

	.icon-buttons {
		display: flex;
		justify-content: space-around;
		width: 100%;
		max-width: 300px;
		margin: auto;
		align-items: center;
	}
	.icon-button img {
		width: 50px;
	}
	.icon-button img:hover {
		box-shadow: 3px 3px 2px 2px #4c566a;
		transition-duration: 0.5s;
	}

	#more-info {
		text-align: justify;
	}
</style>
