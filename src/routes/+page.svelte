<script lang="ts">
	import { get_years, get_days } from 'rust';
</script>

<ul>
	{#each get_years() as year}
		<li>
			{year}
			<ul>
				{#each get_days(year) as day}
					<li>{day}</li>
				{/each}
			</ul>
		</li>
	{/each}
</ul>
