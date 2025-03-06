<script lang="ts">
	import { Course, Section } from '$lib/query.svelte';
	import * as Dialog from '$lib/components/ui/dialog';
	let focused: { course: Course | null; section: Section | null } = $state({
		course: null,
		section: null
	});

	import type { Writable } from 'svelte/store';
	import { type QueryParams } from '$lib/queryStore.svelte';
	import SearchBar from '$lib/SearchBar.svelte';
	import CourseCard from '$lib/CourseCard.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import { CalendarFold, ExternalLink } from 'lucide-svelte';
	import SectionCard from '$lib/SectionCard.svelte';
	import { getContext } from 'svelte';
	let offset: number = $state(0);
	let loading = $state(false);

	function addLimit() {
		offset += 15;
	}
	let dialogOpen = $derived(focused.course != null || focused.section != null);
	const queryParams: Writable<QueryParams> = getContext('queryParams');
	const queryResponse: { courses: Course[]; sections: Section[] } = getContext('queryResponse');
	let searching: boolean = $derived(
		!!(
			($queryParams.desc && $queryParams.desc.length) ||
			($queryParams.dept && $queryParams.dept.length) ||
			($queryParams.num && $queryParams.num.length) ||
			$queryParams.attrs.length ||
			$queryParams.daysOfWeek.length ||
			($queryParams.instructor && $queryParams.instructor.length) ||
			($queryParams.class_num && $queryParams.class_num.length)
		)
	);
</script>

<svelte:head>
	<title>Navarch</title>
	<meta
		name="description"
		content="An unofficial, lighter weight, faster, and feature rich course catalog and schedule builder for the University of Arizona."
	/>
</svelte:head>

<a href="/schedule">
	<div
		class=" cursor fixed left-3 top-3 flex flex-row gap-2 rounded-3xl border-2
		border-solid border-slate-500 border-opacity-10 bg-white bg-opacity-75 p-2 transition-all duration-300
		hover:border-opacity-100 hover:bg-opacity-100"
	>
		<CalendarFold />
		<p>Schedule Builder</p>
	</div></a
>

<div class="mb-6 mt-8 flex w-full flex-col items-center gap-8">
	<SearchBar bind:offset limit={15} bind:loading />
</div>
{#if !queryResponse.sections.length && !queryResponse.courses.length && searching && !loading}
	<div class="flex w-full justify-center">No results found</div>
{:else}
	<div class="grid justify-center gap-6 p-10 sm:grid-cols-1 md:grid-cols-3 lg:grid-cols-5">
		{#if queryResponse.courses.length > 0}
			{#each queryResponse.courses as result}
				<CourseCard course={result} bind:focused />
			{/each}
		{/if}
		{#if queryResponse.sections.length > 0}
			{#each queryResponse.sections as result}
				{#if (!$queryParams.showOpen)||+result.available_seats !== 0}
					<SectionCard section={result} />
				{/if}
			{/each}
		{/if}
	</div>
{/if}
{#if (!(queryResponse.sections.length < 14) && queryResponse.courses.length == 0) || (!(queryResponse.courses.length < 14) && queryResponse.sections.length == 0)}
	<div class="mb-8 flex w-full flex-col items-center gap-6">
		{#if queryResponse.sections.length == 0}<p>
				Showing {queryResponse.courses.length} results
			</p>{/if}
		{#if queryResponse.courses.length == 0}<p>
				Showing {queryResponse.sections.length} results
			</p>{/if}
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
				<Dialog.Description class="flex max-h-[50vh] flex-col gap-2">
					<p class="flex-1 overflow-y-auto"><br />{@html course.description}<br /><br /></p>
					<div class="flex gap-2">
						{#if course.sections_avail}
							<button
								class="flex w-1/2 items-center justify-center gap-2 rounded-sm bg-blue-900 text-white"
								onclick={() => {
									const qp = {
										desc: null,
										dept: course.department,
										num: course.course_number,
										attrs: [],
										instructor: null,
										class_num: null,
										startTime: '05:00',
										endTime: '22:00',
										daysOfWeek: [],
										term: $queryParams.term,
										filters: [
											{ value: 'description' },
											{ value: 'departments' },
											{ value: 'course_number' },
											{ value: 'days' },
											{ value: 'times' },
											{ value: 'term' }
										],
										searchType: { value: 'section', label: 'Sections' }
									} as QueryParams;
									$queryParams = qp;
									focused = { course: null, section: null };
								}}>Search Available Sections</button
							>
						{/if}<a
							class="flex flex-auto items-center justify-center gap-2 rounded-sm bg-red-900 py-1 text-white"
							href={`/course/${course.department}/${course.course_number}`}
							>View Course <ExternalLink size={18} /></a
						>
					</div>
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
