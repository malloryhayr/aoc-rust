<script lang="ts">
	import { get_years, get_days, get_solutions, get_solution } from 'rust';

	import input from '$lib/input';

	interface Solution {
		value: string;
		time: number;
	}

	function run_solution(year: number, day: number, solution: number, input: string): Solution {
		const start = performance.now();
		const value = get_solution(year, day + 1, solution + 1, input);
		const end = performance.now();
		return {
			value,
			time: end - start
		};
	}
</script>

<ul>
	{#each get_years() as year}
		<li>
			{year}
			<ul>
				{#each Array(get_days(year)) as _, day}
					<li>
						{day + 1}
						<ul>
							{#each Array(get_solutions(year, day + 1)) as _, s}
								{@const solution = run_solution(year, day, s, input)}
								<li>
									{s + 1}: got {solution.value} in {solution.time}ms
								</li>
							{/each}
						</ul>
					</li>
				{/each}
			</ul>
		</li>
	{/each}
</ul>
