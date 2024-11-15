<script lang="ts">
	import Filter from 'lucide-svelte/icons/filter';
	import { grow } from '$lib/transitions';
	import * as ToggleGroup from '$lib/components/ui/toggle-group/index.js';
	import * as Select from '$lib/components/ui/select';
	import { CourseQuery, Course, Section, SectionQuery } from '$lib/query.svelte';
	import Input from './components/ui/input/input.svelte';
	import type { Snapshot } from '../routes/$types';
	import { persisted } from 'svelte-persisted-store';
	import type { Writable } from 'svelte/store';

	import { getContext } from 'svelte';
	import type { QueryParams } from './queryStore.svelte';
	const queryParams: Writable<QueryParams> = getContext('queryParams');
	const queryResponse: Writable<{ courses: Course[]; sections: Section[] }> = getContext('queryResponse');

	let firstLoad = true;

	let {
		//courses = $bindable(),
		//sections = $bindable(),
		limit = $bindable(),

		//desc = $bindable(),
		//$queryParams.dept = $bindable(),
		//num = $bindable(),
		//$queryParams.attrs = $bindable(),
		//instructor = $bindable(),
		//$queryParams.class_num = $bindable(),
		//startTime = $bindable(),
		//endTime = $bindable(),
		//$queryParams.daysOfWeek = $bindable(),
		//filters = $bindable(),
		//searchType = $bindable()
	}: {
		//courses: Course[];
		//sections: Section[];
		limit: number;

		//desc: string | null;
		//$queryParams.dept: string | null;
		//num: string | null;
		//$queryParams.attrs: any[];
		//instructor: string | null;
		//$queryParams.class_num: string | null;
		//startTime: string;
		//endTime: string;
		//$queryParams.daysOfWeek: string[];
//
		//filters: {
		//	value: string;
		//}[];
		//searchType: {
		//	value: string;
		//	label: string;
		//};
	} = $props();



	// Filter
	//let searchType = $state({ value: 'course', label: 'Courses' });
	//let filters = $state([{ value: 'description' }, { value: 'departments' },{ value: 'course_number' },{ value: 'days' }, { value: 'times' }]);
	let activeFilters: string[] = $derived($queryParams.filters.map((filter) => filter.value));

	// Filter Bindings
	//let desc: string | null = $state(null);
	//let $queryParams.dept: string | null = $state(null);
	//let num: string | null = $state(null);
	//let $queryParams.attrs = $state([]);
	//let instructor: string | null = $state(null);
	//let $queryParams.class_num: string | null = $state(null);
	//let startTime = $state('08:00');
	//let endTime = $state('18:00');
	//let $queryParams.daysOfWeek: string[] = $state([]);

	// Query-Related
	let course_query: CourseQuery = $derived(
		new CourseQuery(
			activeFilters.includes('departments') ? $queryParams.dept : null,
			activeFilters.includes('course_number') ? $queryParams.num : null,
			null,
			activeFilters.includes('description') ? $queryParams.desc : null,
			activeFilters.includes('attributes') ? $queryParams.attrs.map((filter) => filter.value) : []
		)
	);
	let section_query: SectionQuery = $derived(
		new SectionQuery(
			null,
			null,
			null,
			null,
			activeFilters.includes('instructor') ? $queryParams.instructor : null,
			activeFilters.includes('days') ? $queryParams.daysOfWeek : [],
			activeFilters.includes('times') ? $queryParams.startTime : null,
			activeFilters.includes('times') ? $queryParams.endTime : null,
			activeFilters.includes('class_id') ? $queryParams.class_num : null
		)
	);
	// Search on args update
	$effect(() => {
		if(section_query||course_query){};// Hack for store reactivity
		if (firstLoad){firstLoad = false; return;}
		if (
			($queryParams.desc && $queryParams.desc.length > 0) ||
			($queryParams.dept && $queryParams.dept.length > 0) ||
			($queryParams.num && $queryParams.num.length > 0) ||
			$queryParams.attrs.length > 0 ||
			$queryParams.daysOfWeek.length > 0 ||
			($queryParams.instructor && $queryParams.instructor.length > 0) ||
			($queryParams.class_num && $queryParams.class_num.length > 0)
		) {
			if (currentController) {
				currentController.abort();
			}
			let debounce_time = 300;
			if (limit && $queryParams.desc && $queryParams.desc.length < 6 && $queryParams.attrs.length == 0) {
				debounce_time = 40 * $queryParams.desc.length;
			}

			// Create a new controller for this request
			currentController = new AbortController();

			if ($queryParams.searchType.value == 'course') {
				courseSearch(course_query, currentController.signal, debounce_time).then((result) => {
					result.forEach((course) => (course.description = highlight(course.description)));
					$queryResponse.courses = result;
					$queryResponse.sections = [];
				});
			} else {
				sectionSearch(section_query, course_query, currentController.signal, debounce_time).then(
					(result) => {
						$queryResponse.sections = result;
						$queryResponse.courses = [];
					}
				);
			}
		} else {
			$queryResponse.courses = [];
			$queryResponse.sections = [];
		}
	});

	let currentController: AbortController | null = null;

	$effect(() => {
		// Reset limit on any query change
		if ($queryParams.desc || $queryParams.dept || $queryParams.num || $queryParams.attrs || $queryParams.daysOfWeek || $queryParams.instructor) limit = 15;
	});

	async function courseSearch(
		local: CourseQuery,
		signal: AbortSignal,
		debounce_time: number = 300
	): Promise<Course[]> {
		try {
			await new Promise((resolve, reject) => {
				const timeout = setTimeout(resolve, debounce_time);
				signal.addEventListener('abort', () => {
					clearTimeout(timeout);
					reject(new Error('Cancelled'));
				});
			});

			const response = await fetch('/api/search', {
				method: 'POST',
				body: JSON.stringify({ course: local, limit }),
				headers: {
					'content-type': 'application/json',
					'search-type': 'course'
				},
				signal // Pass the signal to fetch
			});

			const out = await response.json();
			return out;
		} catch (error) {
			if (error instanceof Error && error.name === 'AbortError') {
				// Silently handle aborted requests
				return [];
			}
			throw error; // Re-throw other errors
		}
	}

	/*
	async function courseSearch(local: CourseQuery): Promise<Course[]> {
		await new Promise((resolve) => setTimeout(resolve, 1000));
		let response = await fetch('/api/search', {
			method: 'POST',
			body: JSON.stringify({ course: local, limit }),
			headers: {
				'content-type': 'application/json',
				'search-type': 'course'
			}
		});
		let out = response.json();
		return out;
	}
		*/

	async function sectionSearch(
		s_query: SectionQuery,
		c_query: CourseQuery,
		signal: AbortSignal,
		debounce_time: number = 300
	): Promise<Section[]> {
		try {
			await new Promise((resolve, reject) => {
				const timeout = setTimeout(resolve, debounce_time);
				signal.addEventListener('abort', () => {
					clearTimeout(timeout);
					reject(/*new Error('Cancelled')*/);
				});
			});

			let response = await fetch('/api/search', {
				method: 'POST',
				body: JSON.stringify({ section: s_query, course: c_query, limit }),
				headers: {
					'content-type': 'application/json',
					'search-type': 'section'
				}
			});
			let out = response.json();
			return out;
		} catch (error) {
			if (error instanceof Error && error.name === 'AbortError') {
				// Silently handle aborted requests
				return [];
			}
			throw error; // Re-throw other errors
		}
	}
	function highlight(text: string): string {
		if ($queryParams.desc && $queryParams.desc.length < 4) return text;
		const regex = new RegExp(`(${$queryParams.desc})`, 'gi');
		return text.replace(regex, `<mark>$1</mark>`);
	}
</script>

<div
	class="box-content flex h-9 w-fit flex-row items-center rounded-2xl border-2 border-solid border-gray-400 p-0"
>
	<Select.Root bind:selected={$queryParams.searchType}>
		<Select.Trigger
			class="m-0 h-full w-[100px] rounded-none rounded-bl-2xl rounded-tl-2xl border-y-0 border-l-0 border-r-[1px] "
		>
			<Select.Value placeholder="Courses" />
		</Select.Trigger>
		<Select.Content>
			<Select.Item value="course">Courses</Select.Item>
			<Select.Item value="section">Sections</Select.Item>
		</Select.Content>
	</Select.Root>
	{#if activeFilters.includes('description')}<div class="h-min" transition:grow>
			<Input
				type="text"
				class="z-10 m-0 h-full rounded-none border-y-0 border-l-0 border-r-[1px] border-gray-300 focus-visible:ring-transparent focus-visible:ring-offset-0"
				bind:value={$queryParams.desc}
				placeholder="Keyword(s)"
			/>
		</div>{/if}
	{#if activeFilters.includes('departments')}
		<div class="h-fit" transition:grow>
			<Input
				type="text"
				class="z-10 m-0 h-full rounded-none border-y-0 border-l-0 border-r-[1px] border-gray-300 focus-visible:ring-transparent focus-visible:ring-offset-0"
				bind:value={$queryParams.dept}
				placeholder="Department"
			/>
		</div>{/if}
	{#if activeFilters.includes('course_number')}<div class="h-fit" transition:grow>
			<Input
				type="text"
				class="z-10 m-0 h-full rounded-none border-y-0 border-l-0 border-r-[1px] border-gray-300 focus-visible:ring-transparent focus-visible:ring-offset-0"
				bind:value={$queryParams.num}
				placeholder="Course Number"
			/>
		</div>{/if}
	{#if activeFilters.includes('instructor') && $queryParams.searchType.value == 'section'}<div
			class="h-fit"
			transition:grow
		>
			<Input
				type="text"
				class="z-10 m-0 h-full rounded-none border-y-0 border-l-0 border-r-[1px] border-gray-300 focus-visible:ring-transparent focus-visible:ring-offset-0"
				bind:value={$queryParams.instructor}
				placeholder="Instructor"
			/>
		</div>{/if}
	{#if activeFilters.includes('class_id') && $queryParams.searchType.value == 'section'}<div
			class="h-fit"
			transition:grow
		>
			<Input
				type="number"
				class="z-10 m-0 h-full rounded-none border-y-0 border-l-0 border-r-[1px] border-gray-300 focus-visible:ring-transparent focus-visible:ring-offset-0"
				bind:value={$queryParams.class_num}
				placeholder="Class ID"
			/>
		</div>{/if}
	{#if activeFilters.includes('attributes')}<div class="h-fit" transition:grow>
			<Select.Root multiple bind:selected={$queryParams.attrs}>
				<Select.Trigger
					class="border-gray-30 z-10 m-0 h-full overflow-hidden rounded-none border-y-0 border-l-0 border-r-[1px]"
				>
					<Select.Value class="overflow-hidden" placeholder="Course Attribute(s)" />
				</Select.Trigger>
				<Select.Content>
					<Select.Item value="bc">Building Connections</Select.Item>
					<Select.Item value="hum">EP: Humanist</Select.Item>
					<Select.Item value="art">EP: Artist</Select.Item>
					<Select.Item value="ns">EP: Natural Scientist</Select.Item>
					<Select.Item value="ss">EP: Social Scientist</Select.Item>
					<Select.Item value="ec">Entry Course</Select.Item>
					<Select.Item value="xc">Exit Course</Select.Item>
				</Select.Content>
			</Select.Root>
		</div>
	{/if}
	{#if activeFilters.includes('days') && $queryParams.searchType.value == 'section'}
		<div class="h-full" transition:grow>
			<ToggleGroup.Root
				type="multiple"
				bind:value={$queryParams.daysOfWeek}
				class="z-10 m-0 h-full overflow-hidden rounded-none border-y-0 border-l-0 border-r-[1px] border-gray-300"
			>
				<ToggleGroup.Item class="h-full" value="mo" aria-label="Toggle Monday">Mo</ToggleGroup.Item>
				<ToggleGroup.Item class="h-full" value="tu" aria-label="Toggle Tuesday">Tu</ToggleGroup.Item
				>
				<ToggleGroup.Item class="h-full" value="we" aria-label="Toggle Wednesday"
					>We</ToggleGroup.Item
				>
				<ToggleGroup.Item class="h-full" value="th" aria-label="Toggle Thursday"
					>Th</ToggleGroup.Item
				>
				<ToggleGroup.Item class="h-full" value="fr" aria-label="Toggle Friday">Fr</ToggleGroup.Item>
			</ToggleGroup.Root>
		</div>
	{/if}
	{#if activeFilters.includes('times') && $queryParams.searchType.value == 'section'}
		<div
			class="m-0 flex h-full flex-row items-center overflow-hidden rounded-none border-y-0 border-l-0 border-r-[1px] border-gray-300"
			transition:grow
		>
			<div>
				<!--<Label for="st">Start time</Label>-->
				<Input
					bind:value={$queryParams.startTime}
					class="h-full border-0"
					type="time"
					id="st"
					placeholder="08:00"
					autocomplete="off"
				/>
			</div>
			<p>-</p>
			<div>
				<!--<Label for="et">End time</Label>-->
				<Input
					bind:value={$queryParams.endTime}
					class="h-full border-0"
					type="time"
					id="et"
					placeholder="18:00"
					autocomplete="off"
				/>
			</div>
		</div>
	{/if}
	<Select.Root multiple bind:selected={$queryParams.filters}>
		<Select.Trigger class="h-full w-10 rounded-br-2xl rounded-tr-2xl border-none [&>*]:hidden">
			<Filter class="!block h-7 w-7 opacity-50" />
		</Select.Trigger>
		<Select.Content>
			<Select.Item value="description">Keywords</Select.Item>
			<Select.Item value="departments">Department</Select.Item>
			<Select.Item value="course_number">Course Number</Select.Item>
			<Select.Item value="attributes">Course Attributes</Select.Item>
			{#if $queryParams.searchType.value == 'section'}
				<Select.Item value="days">Week Days</Select.Item>
				<Select.Item value="times">Times</Select.Item>
				<Select.Item value="instructor">Instructor</Select.Item>
				<Select.Item value="class_id">Class ID</Select.Item>
			{/if}
		</Select.Content>
	</Select.Root>
</div>
