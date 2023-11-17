<section id="more-info" class="dark-bg">
	<h3>
		<b>Musicalendar</b>: See your Spotify playlist come alive!
		<br />
	</h3>
	<div class="container">
		<div class="text">
			<p>
				Enter your playlist link, and we'll show you a visual timeline
				of your music journey. Each day reveals the songs added, turning
				your playlist into a personalized musical diary. Explore the
				evolution of your favorite tunes with Musicalendar.
				<br />
			</p>
		</div>
		<img
			src="images/highlight.png"
			alt="A vibrant painting with dates and song notes"
			class="fade-in"
		/>
	</div>
</section>

<style>
	#more-info {
		max-width: 800px;
		padding-top: 6.4rem;
		margin: 0 auto;
		text-align: center;
	}

	#more-info h3 {
		font-size: 2.66666667rem;
		font-weight: 900;
		line-height: 3.11111111rem;
	}

	.container {
		display: flex;
		flex-wrap: wrap;
		width: 100%;
	}

	.text {
		flex: 1;
		padding: 20px;
		text-align: left;
		margin-right: 25px;
	}
	.text p {
		font-size: 1.3333333333rem;
		line-height: 1.7777777778rem;
	}

	img {
		flex: 1;
		border-radius: 10%;
		max-width: 50%;
		height: auto;
		object-fit: contain;
		opacity: 0;
		transition: opacity 1s ease-in-out;
	}

	.fade-in {
		animation: fadeIn 2s ease-in-out forwards;
	}

	@keyframes fadeIn {
		to {
			opacity: 1;
		}
	}

	@media (max-width: 600px) {
		.container {
			flex-direction: column;
		}
	}
</style>
