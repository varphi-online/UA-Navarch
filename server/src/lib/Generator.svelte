<script lang="ts">
	import { getContext } from 'svelte';
	import { Course, Section } from '$lib/query.svelte';
	const selected: { courses: Course[]; sections: Section[] } = getContext('selected');

    let {schedules = $bindable(), generated=$bindable()}: {schedules: Section[][]; generated: boolean} = $props();

	import { Trash, Plus } from 'lucide-svelte';

	import Input from './components/ui/input/input.svelte';
	import Button from './components/ui/button/button.svelte';
	let genOpts: { courses: Course[]; sections: Section[]; blacklist: string[] } = $state({
		courses: [],
		sections: [],
		blacklist: []
	});
	let currentBLInput = $state('');

	$effect(() => {
		genOpts.sections = [...selected.sections];
	});
	$effect(() => {
		genOpts.courses = [...selected.courses.filter((c) => c.sections_avail)];
	});

    async function generate() {
		const resp = await fetch('/api/generate', {
			method: 'POST',
			body: JSON.stringify({ sections: genOpts.sections, courses: genOpts.courses }),
			headers: {
				'content-type': 'application/json'
			}
		});
		const parsed = await resp.json();
		schedules = parsed.length > 0 ? parsed : null;
		generated = true;
	}
</script>

<div class="row flex flex-row items-center gap-6">
    <div class="flex flex-col justify-center items-center">
    <p>Generation Options</p>
	<div class="grid grid-cols-3 grid-rows-4">
		
		<h3 class="row-start-1 row-end-1 col-start-1 col-end-1">Sections</h3>
		<div class="row-start-2 row-end-3 col-start-1 col-end-1">
			{#each genOpts.sections as section}
				<button
					onclick={() => (genOpts.sections = genOpts.sections.filter((s) => s != section))}
					class=" group h-6 w-fit cursor-pointer rounded-2xl bg-blue-900 p-[1px] px-2 text-xs text-white hover:bg-red-700"
				>
					<p class=" visible h-4 w-20 group-hover:hidden">
						{section.department}
						{section.course_number}
						{section.section_number}
					</p>
					<Trash class="hidden h-4 w-20 group-hover:inline" />
				</button>
			{/each}
		</div>
		<h3 class="row-start-1 row-end-1 col-start-2 col-end-2">Courses</h3>
		<div class="row-start-2 row-end-3 col-start-2 col-end-2">
			{#each genOpts.courses as course}
				<button
					onclick={() => (genOpts.courses = genOpts.courses.filter((c) => c != course))}
					class=" group h-6 w-fit cursor-pointer rounded-2xl bg-blue-900 p-[1px] px-2 text-xs text-white hover:bg-red-700"
				>
					<p class=" visible h-4 w-14 group-hover:hidden">
						{course.department}
						{course.course_number}
					</p>
					<Trash class="hidden h-4 w-14 group-hover:inline" />
				</button>
			{/each}
		</div>
		<h3 class="row-start-1 row-end-1 col-start-3 col-end-3">Exclude Instructors</h3>
        <div class="row-start-2 row-end-2 col-start-3 col-end-3 flex flex-row">
			<Input bind:value={currentBLInput} type="text"  /><button
				onclick={() => {
					genOpts.blacklist.push(currentBLInput);
					currentBLInput = '';
				}}><Plus /></button
			></div>
			<div class="row-start-3 row-span-2 col-start-3 col-end-3 grid grid-cols-3">
				{#each genOpts.blacklist as instructor}
					<button
						onclick={() => (genOpts.blacklist = genOpts.blacklist.filter((c) => c != instructor))}
						class=" group h-6 w-fit cursor-pointer rounded-2xl bg-blue-900 p-[1px] px-2 text-xs text-white hover:bg-red-700"
					>
						<p class=" visible h-4 w-14 group-hover:hidden">{instructor}</p>
						<Trash class="hidden h-4 w-14 group-hover:inline" />
					</button>
				{/each}
			</div>
	</div>
</div>
	<Button onclick={() => generate()}>Generate!</Button>
</div>
