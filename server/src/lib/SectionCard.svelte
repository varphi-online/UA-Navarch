<script lang="ts">
	import { fade } from 'svelte/transition';
	import { Course, Section } from '$lib/query.svelte';
	let { section, small = false }: { section: Section; small: boolean } = $props();
	import Lock from 'lucide-svelte/icons/lock';
	import Link from 'lucide-svelte/icons/link';
	import Hammer from 'lucide-svelte/icons/hammer';
	import BookmarkPlus from 'lucide-svelte/icons/bookmark-plus';

	import type { Writable } from 'svelte/store';
	import * as Tooltip from '$lib/components/ui/tooltip/index.js';
	import { getContext } from 'svelte';
	const selected: Writable<{ courses: Course[]; sections: Section[] }> = getContext('selected');
	let hovered = $state(false);
</script>

<div
	class={`${small ? ' h-min' : 'h-60'} rounded-3xl border-2 border-solid border-gray-300`}
	role="contentinfo"
	onmouseenter={() => (hovered = true)}
	onmouseleave={() => (hovered = false)}
>
	<div class="flex h-full flex-col flex-nowrap justify-center p-4">
		<div>
			<a href={`/course/${section.department}/${section.course_number}`}>
				<h3 class="inline w-fit rounded-2xl bg-gray-200 px-2 text-lg font-semibold">
					{section.department}
					{section.course_number}
				</h3>
			</a>
		</div>
		<div class="my-2 flex">
			<h4 class="text-lg font-semibold"><!--{@html section.title}--></h4>
		</div>
		{#if !small}
			<p
				class=" inset-0 line-clamp-4 flex-auto overflow-hidden text-ellipsis whitespace-normal break-words bg-gradient-to-b from-black via-black to-transparent to-95% bg-clip-text text-transparent transition-all duration-500 ease-in-out"
			>
				{@html section.instructor}<br />
				{#if section.monday == 'true'}Mo{/if}{#if section.tuesday == 'true'}Tu{/if}{#if section.wednesday == 'true'}We{/if}{#if section.thursday == 'true'}Th{/if}{#if section.friday == 'true'}Fr{/if}<br
				/>
				{section.start_time}-{section.end_time}
				<!--{@html section.description}-->
			</p>
			<div class="flex h-6 flex-row justify-end">
				{#if (window.matchMedia('(max-width: 600px)').matches || hovered) && !$selected.sections.includes(section)}
					<div transition:fade={{ duration: 300 }}>
						<BookmarkPlus
							onclick={() => {
								$selected.sections = [...$selected.sections, section];
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
