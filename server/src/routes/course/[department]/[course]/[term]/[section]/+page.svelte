<script lang="ts">
	import {
		UserRound,
		Clock,
		LockKeyholeOpen,
		LockKeyhole,
		CalendarDays,
		Hash,
		BookOpenText,
		BellElectric,
		Presentation,
		PencilLine,
		HardHat,
		BookmarkPlus
	} from 'lucide-svelte';
	import { Progress } from '$lib/components/ui/progress';
	import type { Course, Section } from '$lib/query.svelte';
	import type { PageData } from './$types';
	let { data }: { data: PageData } = $props();
	let course: Course = $derived(data.course_data.at(0));
	let section: Section = $derived(data.section_data.at(0));
	const to12Hour = (time) =>
		time.replace(/(\d{2}):(\d{2})/, (_, h, m) => `${h % 12 || 12}:${m}${h < 12 ? 'AM' : 'PM'}`);
	import { fade } from 'svelte/transition';
	import { getContext } from 'svelte';
	const selected: { courses: Course[]; sections: Section[] } = getContext('selected');
</script>

<div class="mb-10 mt-10 flex w-full flex-col items-center gap-20">
	<div class="flex w-[75%] flex-col justify-start gap-9 lg:w-fit">
		<div class="w-full flex items-center">
			
				<h1 class="text-wrap text-xl font-semibold inline"><a  href={`/course/${course.department}/${course.course_number}`}>
					{@html course.department}
					{@html course.course_number} - {@html course.title}</a> - Section {@html section.section_number}
				</h1>
				{#if !selected.sections.some(s=>s.class_number==section.class_number)}
				<div transition:fade={{ duration: 300 }} class="ml-auto">
					<BookmarkPlus
						onclick={() => {
							selected.sections = [...selected.sections, section];
						}}
						class="cursor-pointer"
					/>
				</div>
			{/if}
		</div>

		<div class="grid w-full grid-cols-2 gap-y-5 lg:mb-3 lg:grid-cols-3 lg:gap-x-6">
			<div class="flex flex-col">
				<div class="flex items-center gap-1 font-bold">
					{#if section.status.toLowerCase().includes('req')}<LockKeyholeOpen
							size={18}
							color="#ff6e19"
							class=" inline"
						/>{/if}
					{#if section.status.toLowerCase() == 'open'}<LockKeyholeOpen
							size={18}
							color="green"
							class=" inline"
						/>{/if}
					{#if section.status.toLowerCase().includes('closed')}<LockKeyhole
							size={18}
							color="#cc0000"
							class=" inline"
						/>{/if} Status
				</div>
				<p class="h-fit">
					{@html section.status}
				</p>
			</div>
			<div class="flex flex-col">
				<div class="flex items-center gap-1 font-bold"><Hash size={18} /> Class Number</div>
				<p class="h-fit">
					{@html section.class_number}
				</p>
			</div>
			<div class="flex flex-col">
				<div class="flex items-center gap-1 font-bold">
					<BellElectric size={18} /> Session
				</div>
				<p class="h-fit">
					{@html section.session}
				</p>
			</div>
			<div class="flex flex-col">
				<div class="flex items-center gap-1 font-bold"><UserRound size={18} /> Instructor</div>
				<p class="h-fit">
					{#if section.instructor}{@html section.instructor}{:else}To Be Determined{/if}
				</p>
			</div>

			<div class="flex flex-col">
				<div class="flex items-center gap-1 font-bold"><Clock size={18} /> Days & Times</div>
				<p class="h-fit max-w-[18ch] lg:max-w-full">
					{#if section.monday == 'true'}Mo{/if}{#if section.tuesday == 'true'}Tu{/if}{#if section.wednesday == 'true'}We{/if}{#if section.thursday == 'true'}Th{/if}{#if section.friday == 'true'}Fr{/if}
					{to12Hour(section.start_time)} - {to12Hour(section.end_time)}
				</p>
			</div>
			<div class="flex flex-col">
				<div class="flex items-center gap-1 font-bold">
					<CalendarDays size={18} /> Meeting Dates
				</div>
				<p class="h-fit">
					{@html section.start_date
						.split('/')
						.map((v, i) => (i == 2 ? (v = v.substring(2, 4)) : v))
						.join('/')} - {@html section.end_date
						.split('/')
						.map((v, i) => (i == 2 ? (v = v.substring(2, 4)) : v))
						.join('/')}
				</p>
			</div>
			<div class="flex flex-col">
				<div class="flex items-center gap-1 font-bold">
					<HardHat size={18} /> Career
				</div>
				<p class="h-fit">
					{@html section.academic_career}
				</p>
			</div>
			<div class="flex flex-col">
				<div class="flex items-center gap-1 font-bold">
					<Presentation size={18} /> Class Components
				</div>
				<p class="h-fit">
					{@html section.instruction_mode}
				</p>
			</div>
			<div class="flex flex-col">
				<div class="flex items-center gap-1 font-bold">
					<BookOpenText size={18} /> Instruction Mode
				</div>
				<p class="h-fit">
					{@html section.instruction_mode}
				</p>
			</div>

			<div class="flex flex-col">
				<div class="flex items-center gap-1 font-bold">
					<PencilLine size={18} /> Grading
				</div>
				<p class="h-fit">
					{@html section.grading}
				</p>
			</div>
		</div>
		<div>
			<div class="flex w-full flex-col items-center">
				<div class="flex w-full items-center gap-2">
					<Progress
						max={parseInt(section.class_capacity)}
						value={parseInt(section.enrollment_total)}
					/>
				</div>
				<p>
					{@html section.enrollment_total} Enrolled / {@html section.class_capacity} Seats | {section.available_seats}
					Seats remaining
				</p>
			</div>
		</div>
		<div class="my-0 h-[1px] w-full border border-gray-200 lg:hidden">&nbsp;</div>
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
			<div class="my-9 h-[1px] w-full border border-gray-200 lg:mx-6 lg:my-0 lg:h-auto lg:w-[1px]">
				&nbsp;
			</div>
			<p class="text-wrap lg:max-w-[80ch]">
				{@html course.description}
			</p>
		</div>
	</div>
</div>
