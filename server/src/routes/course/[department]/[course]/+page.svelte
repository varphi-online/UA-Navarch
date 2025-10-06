<script lang="ts">
	import { Circle, X, ExternalLink, BookmarkPlus, Trash } from 'lucide-svelte';
	import type { Course, Section } from '$lib/query.svelte';
	import type { PageData } from './$types';
	import { Progress } from '$lib/components/ui/progress';
	import * as Table from '$lib/components/ui/table';
	import * as Select from '$lib/components/ui/select';
	import { fade } from 'svelte/transition';
	import { getContext } from 'svelte';
	import { TERMS } from '$lib/consts';
	const selected: { courses: Course[]; sections: Section[] } = getContext('selected');
	let { data }: { data: PageData } = $props();
	let course: Course = $derived(data.course_data.at(0));
	let terms: string[] = [...TERMS];
	let term = $state({ value: terms[2], label: terms[2] });
	let termFiltered: { [key: string]: Section[] } = $derived.by(() => {
		let out = { '': data.section_data };
		terms.forEach((term) => {
			out[term] = data.section_data.filter((s) => s.term.includes(term));
		});
		return out;
	});
	$effect(() => {
		data.section_data;
		term = { value: terms[2], label: terms[2] };
	});
	const to12Hour = (time) =>
		time.replace(/(\d{2}):(\d{2})/, (_, h, m) => `${h % 12 || 12}:${m}${h < 12 ? 'AM' : 'PM'}`);

	import { tick } from 'svelte';

	function resetScroll(node: HTMLElement) {
		tick().then((_) => {
			node.scrollLeft = node.scrollWidth;
		});
	}
</script>

<svelte:head>
	<title>{course.department} {course.course_number} - {course.title} | Navarch</title>
</svelte:head>

<div class="mb-10 mt-10 flex w-full flex-col items-center gap-20">
	<div class="flex w-[75%] flex-col justify-start gap-6 lg:w-fit">
		<div class="flex w-full items-center">
			<h1 class="inline text-wrap text-xl font-semibold">
				{@html course.department}
				{@html course.course_number} - {@html course.title}
			</h1>
			{#if !selected.courses.some((s) => s.title == course.title)}
				<div transition:fade={{ duration: 300 }} class="ml-auto">
					<BookmarkPlus
						onclick={() => {
							selected.courses = [...selected.courses, course];
						}}
						class="cursor-pointer"
					/>
				</div>
			{/if}
		</div>
		<div class="flex flex-col lg:flex-row">
			<div class="flex flex-grow flex-col lg:max-w-[50ch]">
				{#each Object.entries( { building_connections: { text: 'Building Connections', color: 'blue' }, artist: { text: 'Gen Ed: Artist', color: 'red' }, humanist: { text: 'Gen Ed: Humanist', color: 'green' }, natural_scientist: { text: 'Gen Ed: Natural Scientist', color: 'orange' }, social_scientist: { text: 'Gen Ed: Social Scientist', color: 'purple' }, entry_course: { text: 'Entry Course', color: 'yellow' }, exit_course: { text: 'Exit Course', color: 'yellow' } } ) as [key, { text, color }]}
					{#if course[key] === 'true'}
						<h3
							class="mb-2 inline h-fit w-fit -translate-x-1 rounded-2xl px-2 text-base font-semibold bg-{color}-100"
						>
							{text}
						</h3>
					{/if}
				{/each}
				<div class="flex flex-col">
					<h3 class="font-bold">Equivalences</h3>
					<p class="h-fit">
						{#if course.equivalences}{@html course.equivalences}{:else}None{/if}
					</p>
				</div>
				<div class="flex flex-col">
					<h3 class="font-bold">Prerequisites</h3>
					<p class="h-fit">
						{#if course.prerequisites}{@html course.prerequisites}{:else}None{/if}
					</p>
				</div>
				<div class="flex flex-col">
					<h3 class="font-bold">Requirements</h3>
					<p class="h-fit">
						{#if course.requirements}{@html course.requirements}{:else}None{/if}
					</p>
				</div>
				<div class="flex flex-col">
					<h3 class="font-bold">Units</h3>
					<p class="h-fit">{@html course.units}</p>
				</div>
			</div>
			<div class="my-3 h-[1px] w-full border border-gray-200 lg:mx-3 lg:my-0 lg:h-auto lg:w-[1px]">
				&nbsp;
			</div>
			<p class="text-wrap lg:max-w-[80ch]">
				{@html course.description}
			</p>
		</div>
	</div>
	<div class="mb-16 flex w-[75%] flex-col justify-start gap-6 lg:w-fit">
		<div class="flex flex-row items-center justify-center">
			<h2 class="inline-flex text-base">
				Showing class sections for&nbsp;<Select.Root bind:selected={term}>
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
				</Select.Root>
			</h2>
		</div>
		{#if termFiltered[term.value].length > 0}
			<div class="rotate-180 overflow-x-scroll" use:resetScroll>
				<div class="mb-1 w-fit rotate-180">
					<Table.Root>
						<Table.Header class="overflow-visible">
							<Table.Row>
								<Table.Head class="w-[100px] text-center">Section Number</Table.Head>
								<Table.Head class="text-center">Start Time</Table.Head>
								<Table.Head class="text-center">End Time</Table.Head>
								<Table.Head class="text-center">Status</Table.Head>
								<Table.Head>
									<div class="flex w-full flex-row items-center justify-between">
										<p>Mo</p>
										<p>Tu</p>
										<p>We</p>
										<p>Th</p>
										<p>Fr</p>
									</div>
								</Table.Head>
								<Table.Head>Instructor</Table.Head>
								<Table.Head>Instruction Mode</Table.Head>

								<Table.Head class="text-center">Enrolled/Capacity</Table.Head>
								<Table.Head class="text-center">Save</Table.Head>
							</Table.Row>
						</Table.Header>
						<Table.Body>
							{#each termFiltered[term.value].sort((a, b) => {
								return parseInt(a.section_number) - parseInt(b.section_number);
							}) as section}
								<Table.Row>
									<Table.Cell class="font-medium">
										<a
											href={`/course/${section.department}/${section.course_number}/${section.term.replace(' ', '-')}/${section.section_number}`}
										>
											<div class="flex items-center justify-center gap-2">
												{section.section_number}
												<ExternalLink size={12} />
											</div>
										</a>
									</Table.Cell>
									<Table.Cell class="text-center">{to12Hour(section.start_time)}</Table.Cell>
									<Table.Cell class="text-center">{to12Hour(section.end_time)}</Table.Cell>
									<Table.Cell class="flex justify-center"
										><div class="flex h-full w-full items-center justify-start gap-2">
											{#if section.status.toLowerCase() == 'open'}<Circle
													fill="#00cc00"
													strokeWidth={0.7}
													class="h-full w-6"
												/>{/if}
											{#if section.status.toLowerCase().includes('req')}<Circle
													fill="orange"
													strokeWidth={0.7}
													class="h-full w-6"
												/>{/if}
											{#if section.status.toLowerCase().includes('closed')}<Circle
													fill="red"
													strokeWidth={0.7}
													class="h-full w-6"
												/>{/if}{section.status}
										</div></Table.Cell
									>
									<Table.Cell>
										<div class="flex w-full flex-row items-center justify-between">
											{#if section.monday == 'true'}<Circle fill="black" />{:else}<X
													class="h-4"
												/>{/if}
											{#if section.tuesday == 'true'}<Circle fill="black" />{:else}<X
													class="h-4"
												/>{/if}
											{#if section.wednesday == 'true'}<Circle fill="black" />{:else}<X
													class="h-4"
												/>{/if}
											{#if section.thursday == 'true'}<Circle fill="black" />{:else}<X
													class="h-4"
												/>{/if}
											{#if section.friday == 'true'}<Circle fill="black" />{:else}<X
													class="h-4"
												/>{/if}
										</div>
									</Table.Cell>
									<Table.Cell>{@html section.instructor}</Table.Cell>
									<Table.Cell>{@html section.instruction_mode}</Table.Cell>

									<Table.Cell class="flex items-center gap-2"
										><Progress
											max={parseInt(section.class_capacity)}
											value={parseInt(section.enrollment_total)}
										/>
										{section.enrollment_total}/{section.class_capacity}</Table.Cell
									>
									<Table.Cell>
										{#if !selected.sections.some((s) => s.class_number === section.class_number)}
											<BookmarkPlus
												onclick={() => {
													selected.sections = [...selected.sections, section]; // Add section
												}}
												class="cursor-pointer"
											/>
										{:else}
											<Trash
												onclick={() => {
													selected.sections = selected.sections.filter(
														(c) => c.class_number !== section.class_number // Remove section
													);
												}}
												class="cursor-pointer"
											/>
										{/if}
									</Table.Cell>
								</Table.Row>{/each}
						</Table.Body>
					</Table.Root>
				</div>
			</div>
		{:else}
			No sections exist for the chosen term
		{/if}
	</div>
</div>

<style>
	::-webkit-scrollbar {
		height: 0.5rem;
	}

	::-webkit-scrollbar-thumb {
		background-color: #b2b2b2;
		border-radius: 20rem;
	}

	/* Buttons */
	::-webkit-scrollbar-button:single-button {
		display: block;
		background-size: 10px;
		background-repeat: no-repeat;
	}

	/* Left */
	::-webkit-scrollbar-button:single-button:horizontal:decrement {
		height: 100%;
		width: 1rem;
		background-position: 50%;
		background-image: url("data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='100' height='100' fill='rgb(96, 96, 96)'><path d='M70,20 L40,50 L70,80' stroke='rgb(96, 96, 96)' stroke-width='15' fill='none' stroke-linecap='round' stroke-linejoin='round'/></svg>");
	}

	/* Right */
	::-webkit-scrollbar-button:single-button:horizontal:increment {
		height: 100%;
		width: 1rem;
		background-position: 50%;
		background-image: url("data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='100' height='100' fill='rgb(96, 96, 96)'><path d='M30,20 L60,50 L30,80' stroke='rgb(96, 96, 96)' stroke-width='15' fill='none' stroke-linecap='round' stroke-linejoin='round'/></svg>");
	}
</style>
