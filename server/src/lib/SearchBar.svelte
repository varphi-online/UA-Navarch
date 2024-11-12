<script lang="ts">
	import Filter  from 'lucide-svelte/icons/filter';
	import { grow } from '$lib/transitions';
	import * as ToggleGroup from '$lib/components/ui/toggle-group/index.js';
	import { Label } from '$lib/components/ui/label';
	import * as Select from '$lib/components/ui/select';
	import { CourseQuery, Course, Section, SectionQuery } from '$lib/query.svelte';
	import Input from './components/ui/input/input.svelte';
	import SelectTrigger from './components/ui/select/select-trigger.svelte';
	import { Trigger } from './components/ui/dialog';

	let {
		courses = $bindable(),
		sections = $bindable()
	}: { courses: Course[]; sections: Section[] } = $props();

	// Filter
	let searchType = $state({ value: 'course', label: 'Courses' });
	let filters = $state([{ value: 'description' }, { value: 'days' }, { value: 'times' }]);
	let activeFilters: string[] = $derived(filters.map((filter) => filter.value));

	// Filter Bindings
	let desc: string | null = $state(null);
	let dept: string | null = $state(null);
	let num: string | null = $state(null);
	let attrs = $state([]);
	let startTime = $state('08:00');
	let endTime = $state('18:00');
	let daysOfWeek: string[] = $state([]);

	// Query-Related
	let course_query: CourseQuery = $derived(
		new CourseQuery(
			activeFilters.includes('departments') ? dept : null,
			activeFilters.includes('course_number') ? num : null,
			null,
			activeFilters.includes('description') ? desc : null,
			activeFilters.includes('attributes') ? attrs.map((filter) => filter.value) : []
		)
	);
	let section_query: SectionQuery = $derived(
		new SectionQuery(
			null,
			null,
			null,
			null,
			activeFilters.includes('days') ? daysOfWeek : [],
			activeFilters.includes('times') ? startTime : null,
			activeFilters.includes('times') ? endTime : null
		)
	);
	//$effect(()=>console.log(attrs.length))
	// Search on args update
	$effect(() => {
		if (
			(desc && desc.length > 0) ||
			(dept && dept.length > 0) ||
			(num && num.length > 0) ||
			attrs.length > 0 ||
			daysOfWeek.length > 0
		) {
			if (searchType.value == 'course') {
				courseSearch(course_query).then((result) => {
					result.forEach((course) => (course.description = highlight(course.description)));
					courses = result;
					sections = [];
				});
			} else {
				sectionSearch(section_query, course_query).then((result) => {
					sections = result;
					courses = [];
				});
			}
		} else {
			courses = [];
			sections = [];
		}
	});

	async function courseSearch(local: CourseQuery): Promise<Course[]> {
		let response = await fetch('/api/search', {
			method: 'POST',
			body: JSON.stringify({ course: local, limit: 20 }),
			headers: {
				'content-type': 'application/json',
				'search-type': 'course'
			}
		});
		let out = response.json();
		return out;
	}

	async function sectionSearch(s_query: SectionQuery, c_query: CourseQuery): Promise<Section[]> {
		let response = await fetch('/api/search', {
			method: 'POST',
			body: JSON.stringify({ section: s_query, course: c_query, limit: 20 }),
			headers: {
				'content-type': 'application/json',
				'search-type': 'section'
			}
		});
		let out = response.json();
		return out;
	}

	function highlight(text: string): string {
		const regex = new RegExp(`(${desc})`, 'gi');
		return text.replace(regex, `<mark>$1</mark>`);
	}
</script>

<div
	class="box-content flex h-9 w-fit flex-row items-center rounded-2xl border-2 border-solid border-gray-400 p-0"
>
	<Select.Root bind:selected={searchType}>
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
				class="z-10 m-0 h-full rounded-none border-y-0 border-l-0 border-r-[1px] border-gray-300"
				bind:value={desc}
				placeholder="Keyword(s)"
			/>
		</div>{/if}
	{#if activeFilters.includes('departments')}
		<div class="h-fit" transition:grow>
			<Input
				type="text"
				class="z-10 m-0 h-full rounded-none border-y-0 border-l-0 border-r-[1px] border-gray-300"
				bind:value={dept}
				placeholder="Department"
			/>
		</div>{/if}
	{#if activeFilters.includes('course_number')}<div class="h-fit" transition:grow>
			<Input
				type="text"
				class="z-10 m-0 h-full rounded-none border-y-0 border-l-0 border-r-[1px] border-gray-300"
				bind:value={num}
				placeholder="Course Number"
			/>
		</div>{/if}
	{#if activeFilters.includes('attributes')}<div class="h-fit" transition:grow>
			<Select.Root multiple bind:selected={attrs}>
				<Select.Trigger
					class="z-10 m-0 h-full rounded-none border-y-0 border-l-0 border-r-[1px] border-gray-300 overflow-hidden"
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
	{#if activeFilters.includes('days') && searchType.value == 'section'}
		<div class="h-full" transition:grow>
			<ToggleGroup.Root
				type="multiple"
				bind:value={daysOfWeek}
				class="z-10 m-0 h-full rounded-none border-y-0 border-l-0 border-r-[1px] border-gray-300 overflow-hidden"
			>
				<ToggleGroup.Item class="h-full" value="mo" aria-label="Toggle Monday">Mo</ToggleGroup.Item>
				<ToggleGroup.Item class="h-full" value="tu" aria-label="Toggle Tuesday">Tu</ToggleGroup.Item>
				<ToggleGroup.Item class="h-full" value="we" aria-label="Toggle Wednesday">We</ToggleGroup.Item>
				<ToggleGroup.Item class="h-full" value="th" aria-label="Toggle Thursday">Th</ToggleGroup.Item>
				<ToggleGroup.Item class="h-full" value="fr" aria-label="Toggle Friday">Fr</ToggleGroup.Item>
			</ToggleGroup.Root>
		</div>
	{/if}
	{#if activeFilters.includes('times') && searchType.value == 'section'}
		<div class="h-full flex flex-row overflow-hidden items-center m-0 rounded-none border-y-0 border-l-0 border-r-[1px] border-gray-300" transition:grow>
			<div>
				<!--<Label for="st">Start time</Label>-->
				<Input bind:value={startTime} class="border-0 h-full" type="time" id="st" placeholder="08:00" autocomplete="off" />
			</div>
			<p>-</p>
			<div>
				<!--<Label for="et">End time</Label>-->
				<Input bind:value={endTime} class="border-0 h-full" type="time" id="et" placeholder="18:00" autocomplete="off" />
			</div>
		</div>
	{/if}
	<Select.Root multiple bind:selected={filters}>
		<Select.Trigger class="w-10 h-full rounded-br-2xl rounded-tr-2xl border-none [&>*]:hidden">
			<Filter class="h-7 w-7 opacity-50 !block"/>
		</Select.Trigger>
		<Select.Content>
			<Select.Item value="description" >Keywords</Select.Item>
			<Select.Item value="departments">Departments</Select.Item>
			<Select.Item value="course_number">Course Number</Select.Item>
			<Select.Item value="attributes">Course Attributes</Select.Item>
			{#if searchType.value == 'section'}
				<Select.Item value="days">Week Days</Select.Item>
				<Select.Item value="times">Times</Select.Item>
			{/if}
		</Select.Content>
	</Select.Root>
</div>