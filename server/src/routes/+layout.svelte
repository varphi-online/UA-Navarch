<script lang="ts">
	import '../app.css';
	import { setContext } from 'svelte';
	import { writable, type Writable } from 'svelte/store';
	import * as Sheet from '$lib/components/ui/sheet';
	import Button from '$lib/components/ui/button/button.svelte';
	import Bookmark from 'lucide-svelte/icons/bookmark';
	import CourseCard from '$lib/CourseCard.svelte';
	import { Course, Section } from '$lib/query.svelte';
	import { fade } from 'svelte/transition';
	import SectionCard from '$lib/SectionCard.svelte';
	import type {  QueryParams } from '$lib/queryStore.svelte';
	import { browser } from '$app/environment';
	

	const queryParams: Writable<QueryParams> = writable(<QueryParams>{
		desc: null,
		dept: null,
		num: null,
		attrs: [],
		instructor: null,
		class_num: null,
		startTime: '08:00',
		endTime: '18:00',
		daysOfWeek: [],
		filters: [
			{ value: 'description' },
			{ value: 'departments' },
			{ value: 'course_number' },
			{ value: 'days' },
			{ value: 'times' }
		],
		searchType: { value: 'course', label: 'Courses' }
	});

	let getSaveData = (): { courses: Course[]; sections: Section[] } => {
		let savedData;
		if (browser) {
			savedData = localStorage.getItem('saved');
		}
		return savedData
			? JSON.parse(savedData)
			: {
					courses: [],
					sections: []
				};
	};

	const selected: { courses: Course[]; sections: Section[] } = $state(getSaveData());
	const queryResponse: { courses: Course[]; sections: Section[] } = $state({
					courses: [],
					sections: []
				});

	$effect(() => {
		if (browser) {
			localStorage.setItem('saved', JSON.stringify(selected));
		}
	});

	setContext('selected', selected);
	setContext('queryParams', queryParams);
	setContext('queryResponse', queryResponse);
</script>

<Sheet.Root>
	{#if selected.courses.length != 0 || selected.sections.length != 0}
		<div transition:fade>
			<Sheet.Trigger
				class=" fixed right-3  top-3 rounded-3xl border-2 border-solid border-slate-500 border-opacity-10 bg-white bg-opacity-75 p-2 transition-all duration-300 hover:border-opacity-100 hover:bg-opacity-100"
			>
				<Bookmark />
			</Sheet.Trigger>
		</div>
	{/if}
	<Sheet.Content class="pr-2">
		<Sheet.Header>
			<Sheet.Title>Saved items</Sheet.Title>
			<Sheet.Description>
				<div class="overflow-y-auto flex flex-col h-[90cqh] pr-2">
				{#if selected.courses.length > 0}
					<h1>Courses</h1>
					<div class="flex flex-col gap-3">
						{#each selected.courses as course}
							<CourseCard {course} small={true} />{/each}
					</div>
				{/if}
				{#if selected.sections.length > 0}
					<h1>Sections</h1>
					<div class="flex flex-col gap-3">
					{#each selected.sections as section}
						<SectionCard {section} small={true} />
					{/each}</div>
				{/if}
				{#if selected.courses.length == 0 && selected.sections.length == 0}
					<p>No saved courses or sections!</p>
				{/if}
				</div>
			</Sheet.Description>
		</Sheet.Header>
	</Sheet.Content>
</Sheet.Root>
<slot />
