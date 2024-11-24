<script lang="ts">
	import { getContext } from 'svelte';
	import { Course, Section } from '$lib/query.svelte';
	import * as Pagination from '$lib/components/ui/pagination';
	import type { Writable } from 'svelte/store';
	import { type QueryParams } from '$lib/queryStore.svelte';
	
	import Week from '$lib/Week.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Search, ChevronRight, ChevronLeft } from 'lucide-svelte';
	import * as Tabs from '$lib/components/ui/tabs';
	import SectionCard from '$lib/SectionCard.svelte';
	import SearchBar from '$lib/SearchBar.svelte';
	import Generator from '$lib/Generator.svelte';
	import CourseCard from '$lib/CourseCard.svelte';
	import { Separator } from '$lib/components/ui/separator';

	const selected: { courses: Course[]; sections: Section[] } = getContext('selected');
	const queryResponse: { courses: Course[]; sections: Section[] } = getContext('queryResponse');
	const queryParams: Writable<QueryParams> = getContext('queryParams');

	let count = $state(0);
	let view: string = $state('saved');
	let schedules: Section[][] = $state([[]]);
	let generated: boolean = $state(false);
	let sections: Section[] = $state([]);
	let showPage = $state(0);

	$effect(() => {
		if (selected.courses.length || selected.sections.length) {
		}
		console.log('reset');
		generated = false;
		schedules = [[]];
	});

	//https://stackoverflow.com/questions/4467539/javascript-modulo-gives-a-negative-result-for-negative-numbers
	function updateCountWithModulus(e?: WheelEvent, delta: number = 1) {
		const len = schedules.length;
		if (e) {
			count = (((count + Math.sign(-e.deltaY)) % len) + len) % len;
		} else {
			count = (((count + delta) % len) + len) % len;
		}
	}
</script>

<div
	class=" p-50 relative flex h-fit w-full flex-col-reverse items-center justify-center gap-4 px-8 lg:flex-row"
>
	<Tabs.Root bind:value={view} class="flex min-h-[47.5rem] max-h-fit w-full flex-col items-center px-4">
		<Tabs.List>
			<Tabs.Trigger value="saved">Saved</Tabs.Trigger>
			<Tabs.Trigger value="generated">Generate</Tabs.Trigger>
			<Tabs.Trigger value="search">Search</Tabs.Trigger>
		</Tabs.List>
		<Tabs.Content value="saved" class="h-full">
			{#if selected.sections.length > 0}
				<div class="flex flex-col gap-3">
					{#each selected.sections as section}
						<SectionCard {section} small={true} />
					{/each}
				</div>
			{:else}
				<div class="h-full w-full items-center justify-center">
					<p>No saved items to populate schedule</p>
				</div>
			{/if}
		</Tabs.Content>
		<Tabs.Content value="generated" class="h-full w-full overflow-hidden">
			{#if !generated}
				<p>Generate all possible schedules given a combination of courses and sections.</p>
				<Generator bind:schedules bind:generated term={$queryParams.term} />
			{:else if schedules}
				<div class="flex flex-row items-center justify-center">
					<Button onclick={() => updateCountWithModulus(null, -1)}><ChevronLeft /></Button>
					<Button
						onwheel={(e) => {
							if (e.deltaY != 0) {
								updateCountWithModulus(e);
							}
						}}>{count + 1}/{schedules.length}</Button
					>
					<Button onclick={() => updateCountWithModulus()}><ChevronRight /></Button>
				</div>
			{:else}
				<p>No possible schedules could be generated</p>
			{/if}
			<div class="grid grid-cols-1 lg:grid-cols-3 gap-3">
				{#if schedules && schedules.length > 0}
					{#each schedules[count] as section}
						<SectionCard {section} small={true} />
					{/each}
				{/if}
			</div>
			{#if generated}
				<Button
					onclick={() => {
						generated = false;
						schedules = [[]];
					}}>Change generation options</Button
				>
			{/if}
		</Tabs.Content>
		<Tabs.Content value="search" class="h-4/6 w-full">
			<div class=" flex h-full flex-col gap-3">
				<div class="flex w-full flex-col items-center gap-3">
					<SearchBar limit={18} limit_start={18} />
				</div>
				<div class="grid flex-grow grid-cols-1 gap-2 md:grid-cols-3 lg:grid-cols-3">
					{#each queryResponse.courses.slice(showPage * 6, showPage * 6 + 6) as course}
						<CourseCard {course} class="h-[14rem]" />{/each}
					{#each queryResponse.sections.slice(showPage * 6, showPage * 6 + 6) as section}
						<SectionCard {section} />{/each}
				</div>
				{#if queryResponse.courses.length + queryResponse.sections.length > 6}
					<Pagination.Root
						count={queryResponse.courses.length + queryResponse.sections.length}
						perPage={6}
						let:pages
						let:currentPage
					>
						<Pagination.Content>
							<Pagination.Item>
								<!--<Pagination.PrevButton />-->
							</Pagination.Item>
							{#each pages as page (page.key)}
								{#if page.type === 'ellipsis'}
									<Pagination.Item>
										<Pagination.Ellipsis />
									</Pagination.Item>
								{:else}
									<Pagination.Item>
										<Pagination.Link
											{page}
											isActive={currentPage == page.value}
											onclick={() => {
												showPage = parseInt(page.value) - 1;
												console.log(showPage);
											}}
										>
											{page.value}
										</Pagination.Link>
									</Pagination.Item>
								{/if}
							{/each}
							<Pagination.Item>
								<!--<Pagination.NextButton/>-->
							</Pagination.Item>
						</Pagination.Content>
					</Pagination.Root>
				{/if}
			</div>
		</Tabs.Content>
	</Tabs.Root>
	<Separator orientation="vertical" />
	<Week
		sections={view == 'saved' || view == 'search'
			? selected.sections
			: schedules
				? schedules[count]
				: []}
	/>
</div>
