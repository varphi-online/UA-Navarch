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
	import { getContext } from 'svelte';
</script>

<div class="mb-6 mt-16 flex w-full flex-col items-center gap-8">
	<div class="flex flex-row items-center">
		<span class="text-5xl font-bold text-[#0C234B]">NAV</span><img
			src="/Arizona_Wildcats_logo.svg"
			alt="University of Arizona logo"
			class="h-16"
		/><span class="text-5xl font-bold text-[#ab0521]">RCH</span>
	</div>
</div>
{$page.params.department}
{$page.params.course}
{#if course !== undefined}
	<h1>{course.title}</h1>
	<p>{@html course.description}</p>
{:else}
	Not a course
{/if}
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
			<Table.Head class="text-right">Enrolled/Capacity</Table.Head>
		</Table.Row>
	</Table.Header>
	<Table.Body>
		{#each sections.sort((a, b) => {
			return parseInt(a.section_number) - parseInt(b.section_number);
		}) as section}
			<Table.Row>
				<Table.Cell class="font-medium">{section.section_number}</Table.Cell>
				<Table.Cell>{section.start_time}</Table.Cell>
				<Table.Cell>{section.end_time}</Table.Cell>
				<Table.Cell
					><div class="flex items-center gap-2 justify-start w-fit h-full">{#if section.status.toLowerCase() == 'open'}<Circle
							fill="green" strokeWidth={1} class="h-full w-6"
						/>{/if}
						{#if section.status.toLowerCase().includes("req")}<Circle
							fill="yellow" strokeWidth={1} class="h-full w-6"
						/>{/if}
						{#if section.status.toLowerCase().includes("closed")}<Circle
							fill="red" strokeWidth={1} class="h-full w-6"
						/>{/if}{section.status}</div></Table.Cell
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
