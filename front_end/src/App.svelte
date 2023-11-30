<script>
	import Calendar from "./pages/Calendar.svelte";
	import WelcomePage from "./pages/WelcomePage.svelte";

	let menu = 1;
	function handleLink(event) {
		console.log("Playlist link", event.detail.link);
		menu = 2;
	}
</script>

{#if menu === 1}
	<WelcomePage on:playlist-link={handleLink} />
{:else if menu === 2}
	<Calendar />
{:else}
	<h2>Loading...</h2>
{/if}
