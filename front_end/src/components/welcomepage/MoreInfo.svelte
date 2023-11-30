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
	<h4>
		<b>About this project</b>
		<br />
	</h4>
	<div class="container">
		<div class="text">
			<p style="text-align: left;">
				Musicalendar came to life because my girlfriend wanted a way to
				look back at her year through the songs she loved. It all
				started with her idea, and together, we turned it into something
				special.
				<br /><br />
				I'm a French software developer who loves playing around with code,
				especially in Rust. With Musicalendar, I aimed to gain hands-on experience
				in building both front and back ends. This app is not just a project
				for me; it's a blend of my passion for music and coding. As a dedicated
				Spotify user and a professional developer, I used my skills to create
				a platform where you can explore your changing music taste.
				<br /><br />
				Musicalendar is more than an app; it's a way for you to rediscover
				old favorites, feel nostalgic, and reflect on your journey through
				music. Come along on this adventure, and let Musicalendar be your
				personal story of how your music taste evolves over time.
				<br /><br />
				At the core of Musicalendar is a commitment to the Free and Open-Source
				philosophy. I believe in the power of collaborative development and
				the freedom for users to explore, modify, and share. Musicalendar
				is proudly available on
				<a href="https://github.com/paul-ohl/Musicalendar">Github</a>,
				inviting fellow enthusiasts and developers to contribute,
				improve, and personalize the experience.
				<br />
				Feel free to write an
				<a
					href="https://github.com/paul-ohl/Musicalendar/issues/new/choose"
					>issue</a
				>
				if you want to request a feature or signal a bug!
				<br />
			</p>
		</div>
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
		padding-left: 30px;
		padding-right: 30px;
	}

	#more-info h4 {
		font-size: 1.66666667rem;
		font-weight: 900;
		line-height: 3.11111111rem;
		padding-left: 30px;
		padding-right: 30px;
		margin-bottom: 0;
	}

	.container {
		display: flex;
		flex-wrap: wrap;
		width: 100%;
		margin-bottom: 80px;
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

	@media (max-width: 800px) {
		#more-info {
			padding-top: 2.5rem;
		}
	}
</style>
