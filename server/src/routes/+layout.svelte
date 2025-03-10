<script lang="ts">
	import '../app.css';
	import { page } from '$app/stores';
	import { setContext } from 'svelte';
	import { writable, type Writable } from 'svelte/store';
	import * as Sheet from '$lib/components/ui/sheet';
	import Button from '$lib/components/ui/button/button.svelte';
	import { Bookmark, House } from 'lucide-svelte';
	import CourseCard from '$lib/CourseCard.svelte';
	import { Course, Section } from '$lib/query.svelte';
	import { fade } from 'svelte/transition';
	import SectionCard from '$lib/SectionCard.svelte';
	import type { QueryParams } from '$lib/queryStore.svelte';
	import { browser } from '$app/environment';
	import { posthog } from 'posthog-js';

	posthog.init('phc_9tDSPAUFfqVKtotauo2T7C0tLwqMC61OevLHbyehl70', {
		api_host: 'https://us.i.posthog.com',
		person_profiles: 'identified_only' // or 'always' to create profiles for anonymous users as well
	});

	const queryParams: Writable<QueryParams> = writable(<QueryParams>{
		desc: null,
		dept: null,
		num: null,
		attrs: [],
		instructor: null,
		class_num: null,
		startTime: '05:00',
		endTime: '22:00',
		daysOfWeek: [],
		filters: [
			{ value: 'description' },
			{ value: 'departments' },
			{ value: 'course_number' },
			{ value: 'days' },
			{ value: 'times' },
			{ value: 'term' }
		],
		searchType: { value: 'course', label: 'Courses' },
		term: 'Fall 2025',
		showHist: false,
		showOpen: false
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

	let { children } = $props();

	$effect(() => {
		if (browser) {
			localStorage.setItem('saved', JSON.stringify(selected));
		}
	});

	setContext('selected', selected);
	setContext('queryParams', queryParams);
	setContext('queryResponse', queryResponse);
	let bodyHeight: number = $state();
	let windowHeight: number = $state();
</script>

<div class="relative flex min-h-[100vh] flex-col content-evenly">
	<div>
		{#if $page.url.pathname.length > 1}
			<a href="/">
				<div
					class=" cursor fixed left-3 top-3 flex flex-row gap-2 rounded-3xl border-2
		border-solid border-slate-500 border-opacity-10 bg-white bg-opacity-75 p-2 transition-all duration-300
		hover:border-opacity-100 hover:bg-opacity-100"
				>
					<House />
					<p>Home</p>
				</div></a
			>{/if}

		<div class="flex h-fit flex-row justify-center">
			<a href="/" class="mb-5 mt-16 flex h-fit w-fit flex-row items-center tracking-tighter">
				<img src="/favicon.svg" class="z-0 -mr-2 h-24 w-24 translate-y-1" alt="Navarch Logo: A ship vector ship wheel in colors of the University of Arizona"/><span
					class="z-10 h-fit text-5xl font-bold text-[#AB0520]">NAV</span
				><span class="text-5xl font-bold text-[#0C234B]">ARCH</span></a
			>
		</div>

		<Sheet.Root>
			{#if selected.courses.length != 0 || selected.sections.length != 0}
				<div transition:fade>
					<Sheet.Trigger
						class=" fixed right-3  top-3 z-50 flex flex-row gap-2 rounded-3xl 
				border-2 border-solid border-slate-500 border-opacity-10 bg-white bg-opacity-75 p-2 transition-all
				duration-300 hover:border-opacity-100 hover:bg-opacity-100"
					>
						<p>Saved Items</p>
						<Bookmark />
					</Sheet.Trigger>
				</div>
			{/if}
			<Sheet.Content class="pr-2">
				<Sheet.Header>
					<Sheet.Title>Saved Items</Sheet.Title>
					<Sheet.Description>
						<div class="flex h-[90cqh] flex-col overflow-y-auto pr-2">
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
									{/each}
								</div>
							{/if}
							{#if selected.courses.length == 0 && selected.sections.length == 0}
								<p>No saved courses or sections!</p>
							{/if}
						</div>
					</Sheet.Description>
				</Sheet.Header>
			</Sheet.Content>
		</Sheet.Root>
		{@render children?.()}
	</div>
	{#if $page.url.pathname.length > 1||bodyHeight > windowHeight}
		<div
			class="mt-auto py-2 flex flex-col items-center lg:flex-row w-full justify-center gap-1.5 lg:gap-3 border border-t-2 border-gray-50 font-mono text-xs"
		>
			<p>©2024-2025 Varphi</p>
			<p class="hidden lg:block">|</p>
			<a href="https://varphi.online">varphi.online</a>
			<p class="hidden lg:block">|</p>
			<a href="/about">About</a>
			<p class="hidden lg:block">|</p>
			<a href="https://github.com/varphi-online/UA-Navarch">Code</a>
		</div>
	{/if}
</div>
<svelte:body bind:clientHeight={bodyHeight} />
<svelte:window bind:outerHeight={windowHeight} />

<style>
	:global(svg) {
		max-width: 80px;
		max-height: 80px;
		display: block;
		margin: 0px;
	}
</style>
