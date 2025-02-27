<script lang="ts">
	import { fade } from 'svelte/transition';
	import { Course, Section } from '$lib/query.svelte';
	let { section, small = false }: { section: Section; small?: boolean } = $props();
	import {
		Trash,
		Eye,
		EyeOff,
		BookmarkPlus,
		UserRound,
		Clock,
		LockKeyholeOpen,
		LockKeyhole,
		CalendarDays,
		Hash
	} from 'lucide-svelte';
	import { getContext } from 'svelte';
	const selected: { courses: Course[]; sections: Section[] } = getContext('selected');
	let hovered = $state(false);
	import * as Tooltip from '$lib/components/ui/tooltip';
	const to12Hour = time => time.replace(/(\d{2}):(\d{2})/, (_, h, m) => `${h % 12 || 12}:${m}${h < 12 ? 'AM' : 'PM'}`);
</script>

<div
	class={`${small ? ' h-min' : 'h-60'} w-full rounded-3xl border-2 border-solid border-gray-300`}
	role="contentinfo"
	onmouseenter={() => (hovered = true)}
	onmouseleave={() => (hovered = false)}
>
	<div class="flex h-full flex-col flex-nowrap items-center justify-start p-4 w-full">
		<div class="w-full flex">
			<a
				href={`/course/${section.department}/${section.course_number}/${section.term.replace(' ', '-')}/${section.section_number}`}
			>
				<h3 class="inline w-fit rounded-2xl bg-red-900 px-2 text-lg font-semibold text-white">
					{section.department}

					{section.course_number}
				</h3><!-- data-sveltekit-reload -->
			
				<h3 class="inline w-fit rounded-2xl bg-blue-900 px-2 text-lg font-semibold text-white">
					{section.section_number}
				</h3></a
			>
			{#if !small}
			{#if section.status.toLowerCase().includes('req')}<LockKeyholeOpen
					color="#ff6e19"
					class="ml-auto inline"
				/>{/if}
			{#if section.status.toLowerCase() == 'open'}<LockKeyholeOpen
					color="green"
					class="ml-auto inline"
				/>{/if}
			{#if section.status.toLowerCase().includes('closed')}<LockKeyhole
					color="#cc0000"
					class="ml-auto inline"
				/>{/if}{/if}
			{#if small}
				<Tooltip.Root>
					<Tooltip.Trigger class="rounded-2x ml-auto inline w-fit">
						<h3
							class="inline w-fit rounded-2xl bg-black px-2 font-semibold text-white text-lg"
						>
							{section.term.split(' ')[0].substring(0, 2).toUpperCase()}{section.term
								.split(' ')[1]
								.substring(2, 4)}
						</h3>
					</Tooltip.Trigger>
					<Tooltip.Content>
						<p>{section.term}</p>
					</Tooltip.Content>
				</Tooltip.Root>
			{/if}
			
			</div>
		{#if !small}
			<div class="grid grid-cols-2 gap-1 lg:grid-cols-1 flex-auto w-fit h-fit lg:w-full my-0 lg:my-4 items-center content-center">
				<div class="flex gap-1"><Hash /> {@html section.class_number}</div>
				<div class="flex gap-1"><UserRound /> {@html section.instructor}</div>
				<div class="flex gap-1">
					<Clock />
					{#if section.monday == 'true'}Mo{/if}{#if section.tuesday == 'true'}Tu{/if}{#if section.wednesday == 'true'}We{/if}{#if section.thursday == 'true'}Th{/if}{#if section.friday == 'true'}Fr{/if}
					{to12Hour(section.start_time)} - {to12Hour(section.end_time)}
				</div>
				<div class="flex gap-1">
					<CalendarDays />
					{@html section.start_date
						.split('/')
						.map((v, i) => (i == 2 ? (v = v.substring(2, 4)) : v))
						.join('/')} - {@html section.end_date
						.split('/')
						.map((v, i) => (i == 2 ? (v = v.substring(2, 4)) : v))
						.join('/')}
				</div>
			</div>

			<div class="flex h-6 flex-row justify-end lg:w-full">
				{#if (window.matchMedia('(max-width: 600px)').matches || hovered) && !selected.sections.some(s=>s.class_number==section.class_number)}
					<div transition:fade={{ duration: 300 }}>
						<BookmarkPlus
							onclick={() => {
								selected.sections = [...selected.sections, section];
							}}
							class="cursor-pointer"
						/>
					</div>
				{/if}
			</div>
		{:else}
			<div class="flex h-6 flex-row justify-end gap-2 w-full mt-5">
				{#if window.matchMedia('(max-width: 600px)').matches || hovered}
					<div transition:fade={{ duration: 200 }} class=" mr-auto">
						{#if !section.visible}
							<Eye
								onclick={() => {
									section.visible = true;
								}}
								class="cursor-pointer"
							/>
						{:else}
							<EyeOff
								onclick={() => {
									section.visible = false;
								}}
								class="cursor-pointer"
							/>
						{/if}
					</div>
					<div class="flex-auto justify-center flex text-xs items-center" transition:fade={{ duration: 200 }}>{#if section.monday == 'true'}Mo{/if}{#if section.tuesday == 'true'}Tu{/if}{#if section.wednesday == 'true'}We{/if}{#if section.thursday == 'true'}Th{/if}{#if section.friday == 'true'}Fr{/if}
						{to12Hour(section.start_time)}-{to12Hour(section.end_time)}</div>
					<div transition:fade={{ duration: 200 }}>
						<Trash
							onclick={() => {
								 selected.sections = selected.sections.filter(
									(c) => c !== section
								);
							}}
							class="cursor-pointer"
						/>
					</div>
				{/if}
			</div>
		{/if}
	</div>
</div>
<!--<div class="section-item">
					<h3 class="text-xl font-semibold">
						{section.department}
						{section.course_number} - {section.section_number} | {section.class_number}
					</h3>
					<p>
						{#if section.monday == 'true'}Mo{/if}
						{#if section.tuesday == 'true'}Tu{/if}
						{#if section.wednesday == 'true'}We{/if}
						{#if section.thursday == 'true'}Th{/if}
						{#if section.friday == 'true'}Fr{/if}
						| {section.start_time}-{section.end_time}
					</p>
				</div>-->
