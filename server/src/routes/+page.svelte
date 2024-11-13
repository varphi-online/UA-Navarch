<script lang="ts">
	import { Course, Section } from '$lib/query.svelte';
	let courses: Course[] = $state([]);
	let sections: Section[] = $state([]);
	let limit: number = $state(15);
	import SearchBar from '$lib/SearchBar.svelte';
	import CourseCard from '$lib/CourseCard.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	function addLimit() {
		limit += 20;
	}
</script>

<div class="mb-6 mt-16 flex w-full flex-col items-center gap-8">
	
	<div class="flex flex-row items-center"><span class="text-[#AB0520] text-5xl font-bold">NAV</span><img src="/Arizona_Wildcats_logo.svg" class="h-16"/><span class="text-[#0C234B] text-5xl font-bold">RCH</span></div>
	<SearchBar bind:courses bind:sections bind:limit />
</div>
{#if sections.length == 0 && courses.length == 0}
	<div class="flex w-full justify-center">No results found</div>
{:else}
	<div class="grid justify-center gap-6 p-10 sm:grid-cols-1 md:grid-cols-3 lg:grid-cols-5">
		{#if courses.length > 0}
			{#each courses as result}
				<CourseCard course={result} />
			{/each}
			<!--<Button onclick={() => addLimit()}>Try Load More</Button>-->
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
		{/if}
	</div>
{/if}
{#if (!(sections.length < 14) && courses.length == 0) || (!(courses.length < 14) && sections.length == 0)}
	<div class="mb-8 flex w-full flex-col items-center gap-6">
		{#if sections.length == 0}<p>Showing {courses.length} results</p>{/if}
		{#if courses.length == 0}<p>Showing {sections.length} results</p>{/if}
		<Button onclick={() => addLimit()}>Try Load More</Button>
	</div>
{/if}
