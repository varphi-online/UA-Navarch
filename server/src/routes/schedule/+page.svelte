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
	import * as Select from '$lib/components/ui/select';
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


	let terms: string[] = ['Spring 2025', 'Summer 2025', 'Fall 2025'];
	let term = $state({ value: terms[2], label: terms[2] });
	let termFiltered: { [key: string]: Section[] } = $derived.by(() => {
		let out = { '': selected.sections };
		terms.forEach((term) => {
			out[term] = selected.sections.filter((s) => s.term.includes(term));
		});
		return out;
	});

	$effect(() => {
		if (selected.courses.length || selected.sections.length) {
		}
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
<svelte:head>
   <title>Schedule Builder | Navarch</title>
</svelte:head>

<div
	class=" p-50 relative flex h-full w-full flex-col-reverse items-center justify-center gap-4 px-8 lg:flex-row"
>
	<Tabs.Root
		bind:value={view}
		class="flex h-full min-h-[47.5rem] w-full flex-col items-center px-4"
	>
		<Tabs.List>
			<Tabs.Trigger value="saved">Saved</Tabs.Trigger>
			<Tabs.Trigger value="generated">Generate</Tabs.Trigger>
			<Tabs.Trigger value="search">Search</Tabs.Trigger>
		</Tabs.List>
		<Tabs.Content value="saved" class="h-full flex flex-col items-center gap-6 mt-2">
			{#if view=="saved"}
			<h2 class="inline-flex text-base mt-2">
				Saved sections from&nbsp;<Select.Root bind:selected={term}>
				<Select.Trigger
					class="border-gray-30  m-0 h-full w-fit rounded-lg border border-gray-200 p-0 px-2 text-base font-semibold"
				>
					<Select.Value class="overflow-hidden" />
				</Select.Trigger>
				<Select.Content>
					{#each terms as t}
						<Select.Item value={t} class="flex flex-row justify-start gap-2">
							<p>{t}</p>
							<p class="ml-auto text-right text-gray-400">
								({termFiltered[t].length})
							</p></Select.Item
						>
					{/each}
				</Select.Content>
			</Select.Root></h2>

			{#if termFiltered[term.value].length > 0}
				<div class="grid grid-cols-1 gap-3 lg:grid-cols-3">
					{#each termFiltered[term.value] as section}
						<SectionCard {section} small={true} />
					{/each}
				</div>
			{:else}
				<p>No saved items to populate schedule</p>
			{/if}
			{/if}
		</Tabs.Content>
		<Tabs.Content value="generated" class="h-full w-full">
			{#if !generated}
				<div class="mt-2 flex h-full w-full flex-col items-center gap-8">
					<p class="text-lg font-semibold">
						Generate all possible schedules given a combination of courses and sections.
					</p>
					<Generator bind:schedules bind:generated/>
				</div>
			{:else if schedules}
				<div class="flex flex-row items-center justify-center mb-3">
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
				<p class="w-full text-center">No possible schedules could be generated</p>
			{/if}
			<div class="grid grid-cols-1 gap-3 lg:grid-cols-3">
				{#if schedules && schedules.length > 0}
					{#each schedules[count] as section}
						<SectionCard {section} small={true} />
					{/each}
				{/if}
			</div>
			{#if generated}
				<div class="mt-2 flex w-full justify-center">
					<Button
						onclick={() => {
							generated = false;
							schedules = [[]];
						}}>Change generation options</Button
					>
				</div>
			{/if}
		</Tabs.Content>
		<Tabs.Content value="search" class="h-4/6 w-full">
			<div class=" flex h-full flex-col gap-3">
				<div class="flex w-full flex-col items-center gap-3">
					<SearchBar limit={18} offset={0} loading={false}/>
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
			? termFiltered[term.value]
			: schedules
				? schedules[count]
				: []}
	/>
</div>
