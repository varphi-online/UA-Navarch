<script lang="ts">
	import { Course, Section } from '$lib/query.svelte';
	let courses: Course[] = $state([]);
	let sections: Section[] = $state([]);
	import SearchBar from '$lib/SearchBar.svelte';
	import CourseCard from '$lib/CourseCard.svelte';
</script>

<div class="flex w-full flex-col items-center gap-8 mt-16 mb-6">
	<h1 class="text-3xl font-bold">UA Navarch</h1>
	<SearchBar bind:courses bind:sections />
</div>
<div class="grid sm:grid-cols-1 md:grid-cols-3 lg:grid-cols-5 gap-8 p-10 justify-center">
{#if courses.length > 0}
		{#each courses as result}
			<CourseCard course={result}/>
		{/each}
	<!--<Button onclick={() => addLimit()}>Try Load More</Button>-->
{:else}
	<div class="no-results">No results found</div>
{/if}
{#if sections.length > 0}
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
	<!--<Button onclick={() => addLimit()}>Try Load More</Button>-->
{:else if courses.length != 0}
	<div class="no-results">No results found</div>
{/if}
</div>