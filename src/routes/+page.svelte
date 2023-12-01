<script lang="ts">
	import { default as init_rust, get_years, get_days, get_solutions, set_panic_hook } from 'rust';

	import input from '$lib/input';

	import Solution from '$lib/components/Solution.svelte';

	set_panic_hook();
</script>

{#await init_rust()}
	<ul />
{:then rust}
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
									<li>
										{s + 1}: <Solution {year} {day} solution_num={s} input={input[year][day]} />
									</li>
								{/each}
							</ul>
						</li>
					{/each}
				</ul>
			</li>
		{/each}
	</ul>
{/await}
