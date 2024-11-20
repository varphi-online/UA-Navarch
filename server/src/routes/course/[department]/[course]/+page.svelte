<script lang="ts">
	import { Circle, X } from 'lucide-svelte';
	import { page } from '$app/stores';
	import type { Course, Section } from '$lib/query.svelte';
	import type { PageData } from './$types';
	let { data }: { data: PageData } = $props();
	let course: Course = data.course_data.at(0);
	let sections: Section[] = data.section_data;
	import { Progress } from '$lib/components/ui/progress';
	import * as Table from '$lib/components/ui/table';
	import * as Select from '$lib/components/ui/select';
	import { getContext } from 'svelte';
	import type { Writable } from 'svelte/store';
	import { type QueryParams } from '$lib/queryStore.svelte';

	const queryParams: Writable<QueryParams> = getContext('queryParams');

	let terms: string[] = ['Spring 2025', 'Summer 2025', 'Fall 2025'];
	let term = $state({ value: terms[0], label: terms[0] });
	let termFiltered: { [key: string]: Section[] } = {
		'': data.section_data
	};
	terms.forEach((term) => {
		termFiltered[term] = data.section_data.filter((s) => s.term.includes(term));
	});
</script>

{$page.params.department}
{$page.params.course}
{#if course !== undefined}
	<h1>{course.title}</h1>
	<p>{@html course.description}</p>
{:else}
	Not a course
{/if}
<Select.Root bind:selected={term}>
	<Select.Trigger
		class="border-gray-30  m-0 h-full w-fit overflow-hidden rounded-none border-y-0 border-l-0 border-r-[1px]"
	>
		<Select.Value class="overflow-hidden" />
	</Select.Trigger>
	<Select.Content>
		{#each terms as t}
			<Select.Item value={t} class="flex flex-row justify-start gap-2">
				<p>{t}</p>
				<p class="ml-auto text-right text-gray-400">({termFiltered[t].length})</p></Select.Item
			>
		{/each}
	</Select.Content>
</Select.Root>
<Table.Root>
	<!--<Table.Caption>A list of your recent invoices.</Table.Caption>-->
	<Table.Header>
		<Table.Row>
			<Table.Head class="w-[100px]">Section Number</Table.Head>
			<Table.Head>Start Time</Table.Head>
			<Table.Head>End Time</Table.Head>
			<Table.Head>Status</Table.Head>
			<Table.Head>
				<div class="flex flex-row items-center justify-between">
					<p>Mo</p>
					<p>Tu</p>
					<p>We</p>
					<p>Th</p>
					<p>Fr</p>
				</div>
			</Table.Head>
			<Table.Head>Instructor</Table.Head>
			<Table.Head>Term</Table.Head>
			<Table.Head class="text-right">Enrolled/Capacity</Table.Head>
		</Table.Row>
	</Table.Header>
	<Table.Body>
		{#each termFiltered[term.value].sort((a, b) => {
			return parseInt(a.section_number) - parseInt(b.section_number);
		}) as section}
			<Table.Row>
				<Table.Cell class="font-medium">
					<a data-sveltekit-reload
						href={`/course/${section.department}/${section.course_number}/${section.term.replace(' ', '-')}/${section.section_number}`}
					>
						{section.section_number}
					</a>
				</Table.Cell>
				<Table.Cell>{section.start_time}</Table.Cell>
				<Table.Cell>{section.end_time}</Table.Cell>
				<Table.Cell
					><div class="flex h-full w-fit items-center justify-start gap-2">
						{#if section.status.toLowerCase() == 'open'}<Circle
								fill="#00cc00"
								strokeWidth={1}
								class="h-full w-6"
							/>{/if}
						{#if section.status.toLowerCase().includes('req')}<Circle
								fill="yellow"
								strokeWidth={1}
								class="h-full w-6"
							/>{/if}
						{#if section.status.toLowerCase().includes('closed')}<Circle
								fill="red"
								strokeWidth={1}
								class="h-full w-6"
							/>{/if}{section.status}
					</div></Table.Cell
				>
				<Table.Cell>
					<div class="flex flex-row items-center justify-between">
						{#if section.monday == 'true'}<Circle fill="black" />{:else}<X class="h-4" />{/if}
						{#if section.tuesday == 'true'}<Circle fill="black" />{:else}<X class="h-4" />{/if}
						{#if section.wednesday == 'true'}<Circle fill="black" />{:else}<X class="h-4" />{/if}
						{#if section.thursday == 'true'}<Circle fill="black" />{:else}<X class="h-4" />{/if}
						{#if section.friday == 'true'}<Circle fill="black" />{:else}<X class="h-4" />{/if}
					</div>
				</Table.Cell>
				<Table.Cell>{@html section.instructor}</Table.Cell>
				<Table.Cell>{section.term}</Table.Cell>
				<Table.Cell class="flex items-center gap-2"
					><Progress
						max={parseInt(section.class_capacity)}
						value={parseInt(section.enrollment_total)}
					/>
					{section.enrollment_total}/{section.class_capacity}</Table.Cell
				>
			</Table.Row>{/each}
	</Table.Body>
</Table.Root>
