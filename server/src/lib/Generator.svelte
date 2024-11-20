<script lang="ts">
	import { getContext } from 'svelte';
	import { Course, Section } from '$lib/query.svelte';
	const selected: { courses: Course[]; sections: Section[] } = getContext('selected');

	let {
		schedules = $bindable(),
		generated = $bindable(),
		term,
	}: { schedules: Section[][]; generated: boolean; term: string } = $props();

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
			body: JSON.stringify({ sections: genOpts.sections, courses: genOpts.courses, term }),
			headers: {
				'content-type': 'application/json'
			}
		});
		const parsed = await resp.json();
		schedules = parsed.length > 0 ? parsed : null;
		generated = true;
	}
</script>

<div class="row flex h-1/2 flex-row items-center gap-6">
	<div class="flex h-full flex-col items-center justify-start">
		<p>Generation Options</p>
		<div class="flex h-full flex-row rounded-2xl p-1">
			<div class="flex w-1/3 flex-col gap-2 border-r-2 border-gray-50 px-2">
				<h3 class="border-b-2 border-gray-100">Sections</h3>
				<div class="flex flex-row flex-wrap gap-[2px]">
					{#each genOpts.sections as section}
						<button
							onclick={() => (genOpts.sections = genOpts.sections.filter((s) => s != section))}
							class=" group  min-h-6 max-h-fit w-fit cursor-pointer rounded-2xl bg-blue-900 p-[1px] px-2 text-xs text-white hover:bg-red-700"
						>
							<div class="relative flex h-full w-fit items-center justify-center">
								<p class="block group-hover:invisible">
									{section.department}
									{section.course_number}
									{section.section_number}
								</p>
								<Trash class="absolute hidden h-4 group-hover:block" />
							</div>
						</button>
					{/each}
				</div>
			</div>
			<div class="flex w-1/3 flex-col gap-2 border-r-2 border-gray-50 px-2">
				<h3 class="border-b-2 border-gray-100">Courses</h3>
				<div class="flex flex-row flex-wrap gap-[2px]">
					{#each genOpts.courses as course}
						<button
							onclick={() => (genOpts.courses = genOpts.courses.filter((c) => c != course))}
							class=" group min-h-6 max-h-fit cursor-pointer rounded-2xl bg-blue-900 p-[1px] px-2 text-xs text-white hover:bg-red-700"
							><div class="relative flex h-full w-fit items-center justify-center">
								<p class="block group-hover:invisible">
									{course.department}
									{course.course_number}
								</p>
								<Trash class="absolute hidden h-4 group-hover:block" />
							</div>
						</button>
					{/each}
				</div>
			</div>
			<div class="flex w-1/3 flex-col gap-2 px-2">
				<h3 class="border-b-2 border-gray-100">Exclude Instructors</h3>
				<div class="flex flex-row">
					<Input
						bind:value={currentBLInput}
						type="text"
						onkeyup={(e) => {
							if (e.key === 'Enter' && currentBLInput.length > 0) {
								genOpts.blacklist.push(currentBLInput);
								currentBLInput = '';
							}
						}}
					/><button
						onclick={() => {
							if (currentBLInput.length > 0) {
								genOpts.blacklist.push(currentBLInput);
								currentBLInput = '';
							}
						}}><Plus /></button
					>
				</div>
				<div class="flex flex-row flex-wrap gap-[2px]">
					{#each genOpts.blacklist as instructor}
						<button
							onclick={() => (genOpts.blacklist = genOpts.blacklist.filter((c) => c != instructor))}
							class=" group h-6 w-fit cursor-pointer rounded-2xl bg-blue-900 p-[1px] px-2 text-xs text-white hover:bg-red-700"
							><div class="relative flex h-5 w-fit items-center justify-center">
								<p class="block w-fit text-nowrap group-hover:invisible">{instructor}</p>
								<Trash class="absolute hidden h-4 group-hover:block" />
							</div>
						</button>
					{/each}
				</div>
			</div>
		</div>
	</div>
	<Button onclick={() => generate()}>Generate!</Button>
</div>
