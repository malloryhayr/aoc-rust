<script lang="ts">
	import { greet, add } from 'rust';

	let name = '';
	let a = 1;
	let b = 1;
</script>

<input bind:value={name} placeholder="Enter your name" />
<h1>{greet(name || 'SvelteKit')}</h1>
<h2>
	<input bind:value={a} placeholder="1" type="number" step="1" /> +
	<input bind:value={b} placeholder="1" type="number" step="1" />
	= {add(a, b)}
</h2>

<style>
	input[type='number'] {
		width: 40px;
	}
</style>
