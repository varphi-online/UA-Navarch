<script lang="ts">
	import { Course, Section } from '$lib/query.svelte';
	let courses: Course[] = $state([]);
	let sections: Section[] = $state([]);
	import SearchBar from '$lib/SearchBar.svelte';
</script>

<div class="flex w-full flex-col items-center gap-3">
	<h1 class="text-3xl font-bold">UA Navarch</h1>
	<SearchBar bind:courses bind:sections />
</div>
{#if courses.length > 0}
	<div class="">
		{#each courses as result}
			<div>
				<h3 class="border-b-2 border-dashed border-gray-500 text-xl font-semibold">
					<a href={`/course/${result.department}/${result.course_number}`}>
						{result.department}
						{result.course_number} - {@html result.title}
					</a>
				</h3>
				<p>{@html result.description}</p>
			</div>
		{/each}
	</div>
	<!--<Button onclick={() => addLimit()}>Try Load More</Button>-->
{:else}
	<div class="no-results">No results found</div>
{/if}
{#if sections.length > 0}
	<div class="">
		{#each sections as result}
			<div class="result-item">
				<h3 class="text-xl font-semibold">
					{result.department}
					{result.course_number} - {result.section_number}
				</h3>
				<p>
					{#if result.monday == 'true'}Mo{/if}
					{#if result.tuesday == 'true'}Tu{/if}
					{#if result.wednesday == 'true'}We{/if}
					{#if result.thursday == 'true'}Th{/if}
					{#if result.friday == 'true'}Fr{/if}
					| {result.start_time}-{result.end_time}
				</p>
			</div>
		{/each}
	</div>
	<!--<Button onclick={() => addLimit()}>Try Load More</Button>-->
{:else if courses.length != 0}
	<div class="no-results">No results found</div>
{/if}
