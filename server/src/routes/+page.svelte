<script lang="ts">
	import SelectItem from './../lib/components/ui/select/select-item.svelte';
	import SearchComponent from './SearchComponent.svelte';
	import FilterSearch from '$lib/CourseSearch.svelte';
	import { CourseQuery, Course, Section } from '$lib/query.svelte';
	import CourseSearch from '$lib/CourseSearch.svelte';
	let courses: Course[] = $state([]);
	let sections: Section[] = $state([]);
	import * as Select from '$lib/components/ui/select';

	let searchType = $state({value:"course",label:"Courses"})//
	
</script>

<h1 class="text-3xl font-bold">NavArch</h1>
<div class="flex flex-row">
	<Select.Root bind:selected={searchType}>
		<Select.Trigger   class="border-solid border-2 border-gray-500 rounded-2xl w-[100px] m-0">
		  <Select.Value placeholder="Courses"/>
		</Select.Trigger>
		<Select.Content>
		  <Select.Item value="course">Courses</Select.Item>
		  <Select.Item value="section">Sections</Select.Item>
		</Select.Content>
	  </Select.Root>
	{#if searchType.value == "course"}
	<CourseSearch bind:courses />
{:else}

{/if}
</div>
{#if searchType.value == "course"}
	{#if courses.length > 0}
		<div class="">
			{#each courses as result}
				<div>
					<h3 class="border-b-2 border-dashed border-gray-500 text-xl font-semibold">
						{result.department}
						{result.course_number} - {@html result.title}
					</h3>
					<p>{@html result.description}</p>
				</div>
			{/each}
		</div>
		<!--<Button onclick={() => addLimit()}>Try Load More</Button>-->
{:else}
	<div class="no-results">No results found</div>
{/if}
{:else}

{/if}

