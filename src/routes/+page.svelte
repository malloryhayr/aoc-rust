<script lang="ts">
	import { get_years, get_days, get_solutions, get_solution } from 'rust';
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
							{#each Array(get_solutions(year, day + 1)) as _, solution}
								<li>
									{solution + 1}: {get_solution(year, day + 1, solution + 1, 'input')}
								</li>
							{/each}
						</ul>
					</li>
				{/each}
			</ul>
		</li>
	{/each}
</ul>
