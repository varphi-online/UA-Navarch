<script lang="ts">
	import { fade } from 'svelte/transition';
	import { Course, Section } from '$lib/query.svelte';
	import { Trash, Lock, Link, Hammer, BookmarkPlus } from 'lucide-svelte';
	import { type QueryParams } from './queryStore.svelte';
	import { type Writable } from 'svelte/store';
	let {
		course,
		small = false,
		focused = $bindable({ course: null, section: null }),
		class: internalClass
	}: {
		course: Course;
		small?: boolean;
		focused?: { course: Course | null; section: Section | null };
		class?
	} = $props();
	import * as Tooltip from '$lib/components/ui/tooltip/index.js';

	import { getContext } from 'svelte';
	const selected: { courses: Course[]; sections: Section[] } = getContext('selected');
	const queryParams: Writable<QueryParams> = getContext('queryParams');
	let hovered = $state(false);
</script>

<div
	class={`${internalClass} ${small ? ' h-min' : 'h-80'} rounded-3xl border-2 border-solid border-gray-300`}
	role="contentinfo"
	onmouseenter={() => {
		hovered = true;
	}}
	onmouseleave={() => {
		hovered = false;
	}}
>
	<div class="flex h-full flex-col flex-nowrap justify-center p-4">
		<div>
			<a data-sveltekit-reload href={`/course/${course.department}/${course.course_number}`}>
				<!--bg-grey-200-->
				<h3
					class={`inline w-fit rounded-2xl ${course.sections_avail ? 'bg-blue-900' : 'bg-red-900'} px-2 text-lg font-semibold text-white`}
				>
					{course.department}
					{course.course_number}
				</h3>
				{#if course.building_connections == 'true'}
					<Tooltip.Root>
						<Tooltip.Trigger class="rounded-2x inline w-fit">
							<h3 class="inline w-fit rounded-2xl bg-blue-100 px-2 text-lg font-semibold">BC</h3>
						</Tooltip.Trigger>
						<Tooltip.Content>
							<p>Building Connections</p>
						</Tooltip.Content>
					</Tooltip.Root>
				{/if}
				{#if course.artist == 'true'}
					<Tooltip.Root>
						<Tooltip.Trigger class="rounded-2x inline w-fit ">
							<h3 class="inline w-fit rounded-2xl bg-red-100 px-2 text-lg font-semibold">ART</h3>
						</Tooltip.Trigger>
						<Tooltip.Content>
							<p>Exploring Perspectives: Artist</p>
						</Tooltip.Content>
					</Tooltip.Root>
				{/if}
				{#if course.humanist == 'true'}
					<Tooltip.Root>
						<Tooltip.Trigger class="rounded-2x inline w-fit ">
							<h3 class="inline w-fit rounded-2xl bg-green-100 px-2 text-lg font-semibold">HUM</h3>
						</Tooltip.Trigger>
						<Tooltip.Content>
							<p>Exploring Perspectives: Humanist</p>
						</Tooltip.Content>
					</Tooltip.Root>
				{/if}
				{#if course.natural_scientist == 'true'}
					<Tooltip.Root>
						<Tooltip.Trigger class="rounded-2x inline w-fit ">
							<h3 class="inline w-fit rounded-2xl bg-orange-100 px-2 text-lg font-semibold">NS</h3>
						</Tooltip.Trigger>
						<Tooltip.Content>
							<p>Exploring Perspectives: Natural Scientist</p>
						</Tooltip.Content>
					</Tooltip.Root>{/if}
				{#if course.social_scientist == 'true'}
					<Tooltip.Root>
						<Tooltip.Trigger class="rounded-2x inline w-fit ">
							<h3 class="inline w-fit rounded-2xl bg-purple-100 px-2 text-lg font-semibold">SS</h3>
						</Tooltip.Trigger>
						<Tooltip.Content>
							<p>Exploring Perspectives: Social Scientist</p>
						</Tooltip.Content>
					</Tooltip.Root>{/if}
				{#if course.entry_course == 'true'}
					<Tooltip.Root>
						<Tooltip.Trigger class="rounded-2x inline w-fit ">
							<h3 class="inline w-fit rounded-2xl bg-yellow-100 px-2 text-lg font-semibold">EC</h3>
						</Tooltip.Trigger>
						<Tooltip.Content>
							<p>Entry Course</p>
						</Tooltip.Content>
					</Tooltip.Root>{/if}
				{#if course.exit_course == 'true'}
					<Tooltip.Root>
						<Tooltip.Trigger class="rounded-2x inline w-fit ">
							<h3 class="inline w-fit rounded-2xl bg-yellow-100 px-2 text-lg font-semibold">XC</h3>
						</Tooltip.Trigger>
						<Tooltip.Content>
							<p>Exit Course</p>
						</Tooltip.Content>
					</Tooltip.Root>{/if}
				{#if course.prerequisites}
					<Tooltip.Root>
						<Tooltip.Trigger class="rounded-2x float-right ml-[0.25rem] inline w-fit">
							<Hammer />
						</Tooltip.Trigger>
						<Tooltip.Content>
							<p>Prerequisites Required</p>
						</Tooltip.Content>
					</Tooltip.Root>
				{/if}
				{#if course.requirements}
					<Tooltip.Root>
						<Tooltip.Trigger class="rounded-2x float-right ml-[0.25rem] inline w-fit">
							<Lock />
						</Tooltip.Trigger>
						<Tooltip.Content>
							<p>Requirements Needed</p>
						</Tooltip.Content>
					</Tooltip.Root>
				{/if}
				{#if course.equivalences}
					<Tooltip.Root>
						<Tooltip.Trigger class="rounded-2x float-right inline w-fit">
							<Link />
						</Tooltip.Trigger>
						<Tooltip.Content>
							<p>Equivalent Courses Available</p>
						</Tooltip.Content>
					</Tooltip.Root>
				{/if}
			</a>
		</div>
		<div class="my-2 flex">
			<h4 class="text-lg font-semibold">{@html course.title}</h4>
		</div>
		{#if !small}
			<button
				onclick={() => (focused.course = course)}
				class=" inset-0 line-clamp-4 h-full flex-auto"
			>
				<p
					class="h-full overflow-hidden text-ellipsis whitespace-normal break-words bg-gradient-to-b
			from-black via-black to-transparent to-95% bg-clip-text text-transparent transition-all duration-500 ease-in-out"
				>
					{@html course.description}
				</p>
			</button>

			<div class="flex h-6 flex-row">
				{#if (window.matchMedia('(max-width: 600px)').matches || hovered) && !selected.courses.includes(course)}
					<p transition:fade={{ duration: 300 }} class=" text-red-900">
						{#if !course.sections_avail}
						
							Not offered: {$queryParams.term}
						{/if}
					</p>
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
		{:else}
			<div class="flex h-6 flex-row justify-end">
				{#if window.matchMedia('(max-width: 600px)').matches || hovered}
					<div transition:fade={{ duration: 200 }}>
						<Trash
							onclick={() => {
								selected.courses = selected.courses = selected.courses.filter((c) => c !== course);
							}}
							class="cursor-pointer"
						/>
					</div>
				{/if}
			</div>
		{/if}
	</div>
</div>
