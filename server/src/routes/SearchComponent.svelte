<script lang="ts">
	import { Button } from '$lib/components/ui/button/';
	import { Switch } from '$lib/components/ui/switch';
	import * as ToggleGroup from '$lib/components/ui/toggle-group/index.js';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { CourseQuery, SectionQuery, Course, Section } from '$lib/query.svelte';
	import Command from '$lib/components/ui/command/command.svelte';

	let department_query: string | null = $state(null);
	let course_number_query: string | null = $state(null);
	let description_query: string | null = $state('');
	let course_limit: number = $state(30);
	// svelte-ignore non_reactive_update
	let previousResults: Course[]|Section[] = [];
	let sectionSearch: boolean = $state(false);
	let isLoading = false;
	let daysOfWeek: string[] = $state([]);
	let startTime = $state('08:00');
	let endTime = $state('18:00');

	// Create a derived store for the search results
	let searchResults: Promise<Response> = $derived.by(() => {
		if (
			(!department_query || department_query.length < 1) &&
			(!course_number_query || course_number_query.length < 1) &&
			(!description_query || description_query.length < 1)
		) {
			return new Promise<Response>(() => {});
		}

		if (!sectionSearch) {
			const course = new CourseQuery(
				department_query,
				course_number_query,
				null,
				description_query,
				null,
				null
			);
			let resp = fetch('/api/search', {
				method: 'POST',
				body: JSON.stringify({ course: course, limit: course_limit }),
				headers: {
					'content-type': 'application/json',
					'search-type': 'course'
				}
			});
			return resp;
		} else {
			const section = new SectionQuery(
				department_query,
				course_number_query,
				null,
				null,
				daysOfWeek,
				startTime,
				endTime
			);
			let resp = fetch('/api/search', {
				method: 'POST',
				body: JSON.stringify({ section: section, limit: course_limit }),
				headers: {
					'content-type': 'application/json',
					'search-type': 'section'
				}
			});
			return resp;
		}
	});

	async function search(): Promise<Course[]|Section[]> {
		let resp = await searchResults;
		previousResults = await resp.json();
		return previousResults;
	}

	function highlight(text: string): string {
		const regex = new RegExp(`(${description_query})`, 'gi');
		return text.replace(regex, `<mark>$1</mark>`);
	}

	function addLimit() {
		course_limit += 20;
	}
    function enforceCourse(input: Course[]|Section[]): Course[]{
        return input as Course[];
    }
    function enforceSection(input: Course[]|Section[]): Section[]{
        return input as Section[];
    }
</script>

<div class="search-container">
	<div class="search-box flex flex-row gap-2">
		<div class="flex w-40 max-w-sm flex-col gap-1.5">
			<Label for="dept">Department</Label>
			<Input
				bind:value={department_query}
				type="text"
				id="dept"
				placeholder="eg: CSC"
				autocomplete="off"
			/>
		</div>
		<div class="flex w-40 max-w-sm flex-col gap-1.5">
			<Label for="num">Course Number</Label>
			<Input
				bind:value={course_number_query}
				type="text"
				id="num"
				placeholder="eg: 245"
				autocomplete="off"
			/>
		</div>
		<div class="flex max-w-sm flex-col gap-1.5">
			<Label for="section-search">Search Sections</Label>
			<Switch id="section-search" bind:checked={sectionSearch} />
		</div>
		{#if !sectionSearch}
			<div class="flex w-full max-w-sm flex-col gap-1.5">
				<Label for="desc">Keyword(s)</Label>
				<Input
					bind:value={description_query}
					type="text"
					id="desc"
					placeholder="eg: Discrete"
					autocomplete="off"
				/>
			</div>
		{:else}
			<ToggleGroup.Root size="sm" type="multiple" bind:value={daysOfWeek}>
				<ToggleGroup.Item value="mo" aria-label="Toggle Monday">Mo</ToggleGroup.Item>
				<ToggleGroup.Item value="tu" aria-label="Toggle Tuesday">Tu</ToggleGroup.Item>
				<ToggleGroup.Item value="we" aria-label="Toggle Wednesday">We</ToggleGroup.Item>
				<ToggleGroup.Item value="th" aria-label="Toggle Thursday">Th</ToggleGroup.Item>
				<ToggleGroup.Item value="fr" aria-label="Toggle Friday">Fr</ToggleGroup.Item>
			</ToggleGroup.Root>
			<div id="between" class="flex w-32 max-w-sm flex-col gap-1.5">
				<Label for="st">Start time</Label>
				<Input bind:value={startTime} type="time" id="st" placeholder="08:00" autocomplete="off" />
			</div>
			<div class="flex w-32 max-w-sm flex-col gap-1.5">
				<Label for="et">End time</Label>
				<Input bind:value={endTime} type="time" id="et" placeholder="18:00" autocomplete="off" />
			</div>
		{/if}
		<!--
        <input 
            type="search"
            placeholder="department"
            bind:value={department_query}
            class="search-input"
        />
        <select bind:value={department_query}>
            <option value="CSC">CSC</option>
        </select>
        <input 
            type="search"
            placeholder="course number"
            bind:value={course_number_query}
            class="search-input"
        />
        <input 
            type="search"
            placeholder="keyword(s)"
            bind:value={description_query}
            class="search-input"
        />
        -->
	</div>
	{#await search()}
		{#if (!!department_query || !!course_number_query || !!description_query) && (department_query.length > 0 || course_number_query.length > 0 || description_query.length > 0)}
        <ul class="results-list">
            {#if !sectionSearch}
                {#each enforceCourse(previousResults) as result}
                    <li class="result-item">
                        <h3 class="text-xl font-semibold">
                            {result.department}
                            {result.course_number} - {@html result.title}
                        </h3>
                        <p>{@html highlight(result.description)}</p>
                    </li>
                {/each}
            {:else}
                {#each enforceSection(previousResults) as result}
                    <li class="result-item border-l-ring">
                        <h3 class="text-xl font-semibold">
                            {result.department}
                            {result.course_number} - {result.section_number}
                        </h3>
                        <p>
                            {#if result.monday == 'true'}Mo{/if}
                            {#if result.tuesday == 'true'}Tu{/if}
                            {#if result.wednesday == 'true'}We{/if}
                            {#if result.thursday == 'true'}Th{/if}
                            {#if result.friday == 'true'}Fr{/if}
                            | {result.start_time}-{result.end_time}
                        </p>
                    </li>
                {/each}
            {/if}
        </ul>
		{:else}
			<div class="loading">Search for a class</div>
		{/if}
	{:then results}
		{#if results.length > 0}
			<ul class="results-list">
				{#if !sectionSearch}
					{#each enforceCourse(results) as result}
						<li class="result-item my-4 p-3 border-solid border-2 border-gray-700 rounded-3xl">
							<h3 class="text-xl font-semibold border-b-2 border-dashed border-gray-500">
								{result.department}
								{result.course_number} - {@html result.title}
							</h3>
							<p>{@html highlight(result.description)}</p>
						</li>
					{/each}
				{:else}
					{#each enforceSection(results) as result}
						<li class="result-item">
							<h3 class="text-xl font-semibold">
								{result.department}
								{result.course_number} - {result.section_number}
							</h3>
							<p>
								{#if result.monday == 'true'}Mo{/if}
								{#if result.tuesday == 'true'}Tu{/if}
								{#if result.wednesday == 'true'}We{/if}
								{#if result.thursday == 'true'}Th{/if}
								{#if result.friday == 'true'}Fr{/if}
								| {result.start_time}-{result.end_time}
							</p>
						</li>
					{/each}
				{/if}
			</ul>
			<Button onclick={() => addLimit()}>Try Load More</Button>
		{:else}
			<div class="no-results">No results found</div>
		{/if}
	{:catch error}
		<div class="error">Error: {error.message}</div>
	{/await}
</div>

<style>
	mark {
		background-color: yellow;
	}
	.search-box {
		display: flex;
		flex-direction: row;
	}
</style>
