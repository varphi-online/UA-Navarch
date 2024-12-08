<script lang="ts">
	import Filter from 'lucide-svelte/icons/filter';
	import { grow } from '$lib/transitions';
	import * as ToggleGroup from '$lib/components/ui/toggle-group/index.js';
	import * as Select from '$lib/components/ui/select';
	import { CourseQuery, Course, Section, SectionQuery } from '$lib/query.svelte';
	import Input from './components/ui/input/input.svelte';
	import type { Writable } from 'svelte/store';
	import { getContext } from 'svelte';
	import type { QueryParams } from './queryStore.svelte';

	let { offset = $bindable(), limit = 15, loading = $bindable(false) }: { offset: number; limit?: number; loading: boolean} = $props();
	const queryParams: Writable<QueryParams> = getContext('queryParams');
	const queryResponse: { courses: Course[]; sections: Section[] } = getContext('queryResponse');
	let firstLoad = true;
	let activeFilters: string[] = $derived($queryParams.filters.map((filter) => filter.value));
	// AbortController for Debounce
	let currentController: AbortController | null = null;
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
	
	let termState = $state({
		value: $queryParams.term,
		label: $queryParams.term
			.split(' ')
			.map((w) => w[0].toUpperCase() + w.substring(1).toLowerCase())
			.join(' ')
	});

	// Query-Related
	let course_query: CourseQuery = $derived(
		new CourseQuery(
			activeFilters.includes('departments') ? $queryParams.dept : null,
			activeFilters.includes('course_number') ? $queryParams.num : null,
			null,
			activeFilters.includes('description') ? $queryParams.desc : null,
			activeFilters.includes('attributes') ? $queryParams.attrs.map((filter) => filter.value) : [],
			null,
			null,
			/*activeFilters.includes('term') ?*/ $queryParams.term /*: null*/,
			activeFilters.includes('showHist') ? true : false
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

	function highlight(text: string | null): string {
		// If text is null or undefined, return an empty string
		if (text == null) return '';
		// Check if description is too short
		if ($queryParams.desc && $queryParams.desc.length < 4) return text;
		// Escape special regex characters in the description
		const escapedDesc = $queryParams.desc.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');

		const regex = new RegExp(`(${escapedDesc})`, 'gi');
		return text.replace(regex, `<mark>$1</mark>`);
	}

	async function searchWithDebounce<T>(
		query: Record<string, any>,
		signal: AbortSignal,
		searchType: 'course' | 'section',
		debounceTime: number = 300
	): Promise<T[]> {
		loading = true;
		try {
			await new Promise((resolve, reject) => {
				const timeout = setTimeout(resolve, debounceTime);
				signal.addEventListener('abort', () => {
					clearTimeout(timeout);
					reject(/*new Error('Cancelled')*/);
				});
			});

			const response = await fetch('/api/search', {
				method: 'POST',
				body: JSON.stringify({ ...query, offset: offset, limit: limit }),
				headers: {
					'content-type': 'application/json',
					'search-type': searchType
				},
				signal
			});

			return await response.json();
		} catch (error) {
			if (error instanceof Error && error.name === 'AbortError') {
				// Silently handle aborted requests
				return [];
			}
			throw error; // Re-throw other errors
		}
	}

	// Search on args update
	$effect(() => {
		if (section_query || course_query) {
		}
		if (firstLoad) {
			firstLoad = false;
			return;
		}
		if (
			searching
		) {
			if (currentController) currentController.abort();
			let debounce_time = 300;
			if (
				offset &&
				$queryParams.desc &&
				$queryParams.desc.length < 6 &&
				$queryParams.attrs.length == 0
			) {
				debounce_time = 40 * $queryParams.desc.length;
			}

			// Create a new controller for this request
			currentController = new AbortController();

			if ($queryParams.searchType.value == 'course') {
				(async () => {
					const result = await searchWithDebounce<Course>(
						{ course: course_query },
						currentController.signal,
						'course',
						debounce_time
					);
					result.forEach((course) => (course.description = highlight(course.description)));
					offset == 0 ? (queryResponse.courses = result) : queryResponse.courses.push(...result);
					queryResponse.sections = [];
					loading = false;
				})();
			} else {
				(async () => {
					const result = await searchWithDebounce<Section>(
						{ section: section_query, course: course_query },
						currentController.signal,
						'section',
						debounce_time
					);
					offset == 0 ? (queryResponse.sections = result) : queryResponse.sections.push(...result);
					queryResponse.courses = [];
					loading = false;
				})();
			}
		} else {
			queryResponse.courses = [];
			queryResponse.sections = [];
		}
	});

	$effect(() => {
		// Reset limit on any query change
		if (
			$queryParams.desc ||
			$queryParams.dept ||
			$queryParams.num ||
			$queryParams.attrs ||
			$queryParams.daysOfWeek ||
			$queryParams.instructor
		)
			offset = 0;
	});
	$effect(() => {
		$queryParams.term = termState.value;
	});
</script>

<div class="box-content w-fit rounded-2xl border-2 border-solid border-gray-400 p-0">
	<div
		class=" z-50 flex h-full max-w-max flex-row flex-wrap items-center justify-center overflow-hidden [&>*]:z-40 [&>*]:h-9"
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
					class=" w-min-2 m-0 h-full overflow-ellipsis rounded-none border-y-0 border-l-0 border-r-[1px] border-gray-300 focus-visible:ring-transparent focus-visible:ring-offset-0"
					bind:value={$queryParams.desc}
					placeholder="Keyword(s)"
				/>
			</div>{/if}
		{#if activeFilters.includes('departments')}
			<div class="h-fit" transition:grow>
				<Input
					type="text"
					class=" m-0 h-full w-[7.5rem] rounded-none border-y-0 border-l-0 border-r-[1px] border-gray-300 focus-visible:ring-transparent focus-visible:ring-offset-0"
					bind:value={$queryParams.dept}
					placeholder="Department"
				/>
			</div>{/if}
		{#if activeFilters.includes('course_number')}<div class="h-fit" transition:grow>
				<Input
					type="text"
					class=" m-0 h-full w-[7.7rem] rounded-none border-y-0 border-l-0 border-r-[1px] border-gray-300 focus-visible:ring-transparent focus-visible:ring-offset-0"
					bind:value={$queryParams.num}
					placeholder="Course Number"
				/>
			</div>{/if}
		{#if activeFilters.includes('class_id') && $queryParams.searchType.value == 'section'}<div
				class="h-fit"
				transition:grow
			>
				<Input
					type="text"
					class=" m-0 h-full w-[7.4rem] rounded-none border-y-0 border-l-0 border-r-[1px] border-gray-300 focus-visible:ring-transparent focus-visible:ring-offset-0"
					bind:value={$queryParams.class_num}
					placeholder="Class ID"
				/>
			</div>{/if}
		{#if activeFilters.includes('instructor') && $queryParams.searchType.value == 'section'}<div
				class="h-fit"
				transition:grow
			>
				<Input
					type="text"
					class=" m-0 h-full rounded-none border-y-0 border-l-0 border-r-[1px] border-gray-300 focus-visible:ring-transparent focus-visible:ring-offset-0"
					bind:value={$queryParams.instructor}
					placeholder="Instructor"
				/>
			</div>{/if}

		{#if activeFilters.includes('attributes')}<div class="h-fit" transition:grow>
				<Select.Root multiple bind:selected={$queryParams.attrs}>
					<Select.Trigger
						class="border-gray-30  m-0 h-full overflow-hidden rounded-none border-y-0 border-l-0 border-r-[1px]"
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
					class=" m-0 h-full overflow-hidden rounded-none border-y-0 border-l-0 border-r-[1px] border-gray-300"
				>
					<ToggleGroup.Item class="h-full" value="mo" aria-label="Toggle Monday"
						>Mo</ToggleGroup.Item
					>
					<ToggleGroup.Item class="h-full" value="tu" aria-label="Toggle Tuesday"
						>Tu</ToggleGroup.Item
					>
					<ToggleGroup.Item class="h-full" value="we" aria-label="Toggle Wednesday"
						>We</ToggleGroup.Item
					>
					<ToggleGroup.Item class="h-full" value="th" aria-label="Toggle Thursday"
						>Th</ToggleGroup.Item
					>
					<ToggleGroup.Item class="h-full" value="fr" aria-label="Toggle Friday"
						>Fr</ToggleGroup.Item
					>
				</ToggleGroup.Root>
			</div>
		{/if}
		{#if activeFilters.includes('term')}<div class="h-fit" transition:grow>
				<Select.Root bind:selected={termState}>
					<Select.Trigger
						class="border-gray-30  m-0 h-full overflow-hidden rounded-none border-y-0 border-l-0 border-r-[1px]"
					>
						<Select.Value class="overflow-hidden" placeholder="Course Attribute(s)" />
					</Select.Trigger>
					<Select.Content>
						<Select.Item value="Spring 2025">Spring 2025</Select.Item>
						<Select.Item value="Summer 2025">Summer 2025</Select.Item>
						<Select.Item value="Fall 2025">Fall 2025</Select.Item>
					</Select.Content>
				</Select.Root>
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
				<Select.Item value="term">Term</Select.Item>
				<Select.Item value="showHist">Show History</Select.Item>
			</Select.Content>
		</Select.Root>
	</div>
</div>
