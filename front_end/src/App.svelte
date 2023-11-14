<script>
	import WelcomePage from "./pages/WelcomePage.svelte";

	let menu = 1;
</script>

{#if menu === 1}
	<WelcomePage />
{:else if menu === 2}
	<h1>Unimplemented!</h1>
{:else}
	<h2>Loading...</h2>
{/if}
