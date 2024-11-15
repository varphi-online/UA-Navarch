<script lang="ts">
	import { Course, Section } from '$lib/query.svelte';
	import * as Dialog from '$lib/components/ui/dialog';
	//let courses: Course[] = $state([]);
	//let sections: Section[] = $state([]);
	let focused: { course: Course | null; section: Section | null } = $state({
		course: null,
		section: null
	});
	let limit: number = $state(15);
	import SearchBar from '$lib/SearchBar.svelte';
	import CourseCard from '$lib/CourseCard.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import { CalendarFold } from 'lucide-svelte';
	import SectionCard from '$lib/SectionCard.svelte';
	import { getContext } from 'svelte';
	function addLimit() {
		limit += 20;
	}
	let dialogOpen = $derived(focused.course != null || focused.section != null);
	const queryResponse: { courses: Course[]; sections: Section[] } = getContext('queryResponse');
</script>

<a href="/schedule">
	<div
		class=" cursor fixed left-3 top-3 rounded-3xl border-2 border-solid border-slate-500 border-opacity-10 bg-white bg-opacity-75 p-2 transition-all duration-300 hover:border-opacity-100 hover:bg-opacity-100"
	>
		<CalendarFold />
	</div></a
>
<div class="mb-6 mt-16 flex w-full flex-col items-center gap-8">
	<div class="flex flex-row items-center">
		<span class="text-5xl font-bold text-[#AB0520]">NAV</span><img
			src="/Arizona_Wildcats_logo.svg"
			alt="University of Arizona logo"
			class="h-16"
		/><span class="text-5xl font-bold text-[#0C234B]">RCH</span>
	</div>
	<SearchBar bind:limit/>
</div>
{#if queryResponse.sections.length == 0 && queryResponse.courses.length == 0}
	<div class="flex w-full justify-center">No results found</div>
{:else}
	<div class="grid justify-center gap-6 p-10 sm:grid-cols-1 md:grid-cols-3 lg:grid-cols-5">
		{#if queryResponse.courses.length > 0}
			{#each queryResponse.courses as result}
				<CourseCard course={result} bind:focused />
			{/each}
			<!--<Button onclick={() => addLimit()}>Try Load More</Button>-->
		{/if}
		{#if queryResponse.sections.length > 0}
			{#each queryResponse.sections as result}
				<SectionCard section={result}/>
			{/each}
			<!--<Button onclick={() => addLimit()}>Try Load More</Button>-->
		{/if}
	</div>
{/if}
{#if (!(queryResponse.sections.length < 14) && queryResponse.courses.length == 0) || (!(queryResponse.courses.length < 14) && queryResponse.sections.length == 0)}
	<div class="mb-8 flex w-full flex-col items-center gap-6">
		{#if queryResponse.sections.length == 0}<p>Showing {queryResponse.courses.length} results</p>{/if}
		{#if queryResponse.courses.length == 0}<p>Showing {queryResponse.sections.length} results</p>{/if}
		<Button onclick={() => addLimit()}>Try Load More</Button>
	</div>
{/if}

<Dialog.Root
	open={dialogOpen}
	onOpenChange={(open) => {
		if (open == false) focused = { course: null, section: null };
	}}
>
	<!--<Dialog.Trigger>Open</Dialog.Trigger>-->
	<Dialog.Content>
		<Dialog.Header>
			{@const course = focused.course}
			{@const section = focused.section}
			{#if course}
				<Dialog.Title>{@html course.title}</Dialog.Title>
				<Dialog.Description>
					<br />{@html course.description}
				</Dialog.Description>
			{:else if section}
				<Dialog.Title>Are you sure absolutely sure?</Dialog.Title>
				<Dialog.Description>
					<br />{course.title}
				</Dialog.Description>
			{:else}
				<Dialog.Description>FAILED TO GET COURSE OR SECTION INFO</Dialog.Description>
			{/if}
		</Dialog.Header>
	</Dialog.Content>
</Dialog.Root>
