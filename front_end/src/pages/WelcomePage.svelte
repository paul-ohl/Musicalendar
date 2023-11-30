<script>
	import ActionButtons from "../components/welcomepage/ActionButtons.svelte";
	import MoreInfo from "../components/welcomepage/MoreInfo.svelte";
</script>

<div class="parallax">
	<header>
		<h1>Musicalendar<span class="green-elt">.</span></h1>
		<h2>
			Your Spotify playlist, your timeline: <br />
			Explore your playlist's journey with Musicalendar.
		</h2>

		<ActionButtons on:playlist-link />
	</header>
</div>

<MoreInfo />

<style>
	.green-elt {
		color: #1db954;
	}

	.parallax {
		background-image: url("../images/header.png");
		background-attachment: fixed;
		background-position: center;
		background-repeat: no-repeat;
		background-size: cover;
	}
	header {
		text-align: center;
		color: white;
		background-color: rgba(0, 0, 0, 0.8);
		margin-top: 0;
		padding-top: 100px;
		padding-bottom: 60px;
		padding-left: 20px;
		padding-right: 20px;
	}
	header h1 {
		font-size: 3.55rem;
		font-weight: 800;
	}
	header h2 {
		font-size: 1.33rem;
		font-weight: 600;
	}
	@media (max-width: 600px) {
		header h1 {
			font-size: 2.55rem;
		}
	}
</style>
