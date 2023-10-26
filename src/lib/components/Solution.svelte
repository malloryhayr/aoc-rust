<script lang="ts">
	import type { Solution } from '$lib/types';

	import { get_solution } from 'rust';

	async function run_solution(
		year: number,
		day: number,
		solution: number,
		input: string
	): Promise<Solution> {
		const start = performance.now();
		const value = get_solution(year, day + 1, solution + 1, input);
		const end = performance.now();
		return {
			value,
			time: end - start
		};
	}

	export let year: number, day: number, solution_num: number, input: string;

	let solution_task = run_solution(year, day, solution_num, input);
</script>

{#await solution_task}
	<span>running solution {solution_num}</span>
{:then solution}
	<span>got {solution.value} in {solution.time}ms</span>
{:catch error}
	<span>an error occurred running solution {solution_num}: {error}</span>
{/await}
