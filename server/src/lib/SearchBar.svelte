<script lang="ts">
	import * as ToggleGroup from '$lib/components/ui/toggle-group/index.js';
	import { Label } from '$lib/components/ui/label';
	import * as Select from '$lib/components/ui/select';
	import { CourseQuery, Course, Section, SectionQuery } from '$lib/query.svelte';
	import Input from './components/ui/input/input.svelte';
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
	let course_query: CourseQuery = $derived(new CourseQuery(dept, num, null, desc));
	let section_query: SectionQuery = $derived(
		new SectionQuery(null, null, null, null, daysOfWeek, startTime, endTime)
	);

	// Search on args update
	$effect(() => {
        if (searchType.value == 'section'){courses = []; return}
		if (!((!desc || desc.length < 1) && (!dept || dept.length < 1) && (!num || num.length < 1))) {
			courseSearch(course_query).then((result) => {
				result.forEach((course) => (course.description = highlight(course.description)));
				courses = result;
			});
		} else {
			courses = [];
		}
	});
	$effect(() => {
		if ((activeFilters.includes("days")&&daysOfWeek.length > 0) && searchType.value != 'course') {
			sectionSearch(section_query,course_query).then((result) => {
				//result.forEach((section) => (section.description = highlight(section.description)));
				sections = result;
			});
		} else {
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

    async function sectionSearch(s_query: SectionQuery,c_query: CourseQuery): Promise<Section[]> {
		let response = await fetch('/api/search', {
			method: 'POST',
			body: JSON.stringify({ section:s_query, course: c_query, limit: 20 }),
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

<div class="flex flex-row">
	<Select.Root bind:selected={searchType}>
		<Select.Trigger class="m-0 w-[100px] rounded-2xl border-2 border-solid border-gray-500">
			<Select.Value placeholder="Courses" />
		</Select.Trigger>
		<Select.Content>
			<Select.Item value="course">Courses</Select.Item>
			<Select.Item value="section">Sections</Select.Item>
		</Select.Content>
	</Select.Root>
	{#if activeFilters.includes('description')}<Input
			type="text"
			bind:value={desc}
			placeholder="Keyword(s)"
		/>{/if}
	{#if activeFilters.includes('departments')}<Input
			type="text"
			bind:value={dept}
			placeholder="Department"
		/>{/if}
	{#if activeFilters.includes('course_number')}<Input
			type="text"
			bind:value={num}
			placeholder="Course Number"
		/>{/if}
	{#if activeFilters.includes('attributes')}
		<Select.Root multiple bind:selected={attrs}>
			<Select.Trigger>
				<Select.Value placeholder="Course Attribute(s)" />
			</Select.Trigger>
			<Select.Content>
				<Select.Item value="Gen Ed: Building Connections">Gen Ed: Building Connections</Select.Item>
				<Select.Item value="Gen Ed: Exploring Perspectives"
					>Gen Ed: Exploring Perspectives</Select.Item
				>
				<Select.Item value="Gen Ed: Humanist">Gen Ed: Humanist</Select.Item>
				<Select.Item value="Gen Ed: Artist">Gen Ed: Artist</Select.Item>
				<Select.Item value="Gen Ed: Social Scientist">Gen Ed: Social Scientist</Select.Item>
			</Select.Content>
		</Select.Root>
	{/if}
	{#if activeFilters.includes('days') && searchType.value == 'section'}
		<ToggleGroup.Root type="multiple" bind:value={daysOfWeek}>
			<ToggleGroup.Item value="mo" aria-label="Toggle Monday">Mo</ToggleGroup.Item>
			<ToggleGroup.Item value="tu" aria-label="Toggle Tuesday">Tu</ToggleGroup.Item>
			<ToggleGroup.Item value="we" aria-label="Toggle Wednesday">We</ToggleGroup.Item>
			<ToggleGroup.Item value="th" aria-label="Toggle Thursday">Th</ToggleGroup.Item>
			<ToggleGroup.Item value="fr" aria-label="Toggle Friday">Fr</ToggleGroup.Item>
		</ToggleGroup.Root>
	{/if}
	{#if activeFilters.includes('times') && searchType.value == 'section'}
		<div>
			<Label for="st">Start time</Label>
			<Input bind:value={startTime} type="time" id="st" placeholder="08:00" autocomplete="off" />
		</div>
		<div>
			<Label for="et">End time</Label>
			<Input bind:value={endTime} type="time" id="et" placeholder="18:00" autocomplete="off" />
		</div>
	{/if}
	<Select.Root multiple bind:selected={filters}>
		<Select.Trigger class="">+</Select.Trigger>
		<Select.Content>
			<Select.Item value="description">Keywords</Select.Item>
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