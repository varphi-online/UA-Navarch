<script lang="ts">
	import { fade } from 'svelte/transition';
	import { Course, Section } from '$lib/query.svelte';
	let { section, small = false }: { section: Section; small?: boolean } = $props();
	import { Trash, Eye, EyeOff, BookmarkPlus} from 'lucide-svelte';
	import { getContext } from 'svelte';
	const selected: { courses: Course[]; sections: Section[] } = getContext('selected');
	let hovered = $state(false);
	import * as Tooltip from '$lib/components/ui/tooltip'
</script>

<div
	class={`${small ? ' h-min' : 'h-60'} rounded-3xl border-2 border-solid border-gray-300`}
	role="contentinfo"
	onmouseenter={() => (hovered = true)}
	onmouseleave={() => (hovered = false)}
>
	<div class="flex h-full flex-col flex-nowrap justify-center p-4">
		<div>
			<a data-sveltekit-reload href={`/course/${section.department}/${section.course_number}/${section.term.replace(" ","-")}/${section.section_number}`}>
				<h3 class="inline w-fit rounded-2xl bg-red-900 text-white px-2 text-lg font-semibold">
					{section.department}
					{section.course_number}
					
				</h3>
				<h3 class="inline w-fit rounded-2xl bg-blue-900 text-white px-2 text-lg font-semibold">
					{section.section_number}	
				</h3>
				{#if small}
					<Tooltip.Root>
						<Tooltip.Trigger class="rounded-2x float-right ml-[0.25rem] inline w-fit">
							<h3 class="inline w-fit rounded-2xl bg-gray-300 text-black float-right px-2 text-lg font-semibold">
								{section.term.split(" ")[0].substring(0,2).toUpperCase()}{section.term.split(" ")[1].substring(2,4)}
							</h3>
						</Tooltip.Trigger>
						<Tooltip.Content>
							<p>{section.term}</p>
						</Tooltip.Content>
					</Tooltip.Root>
				{/if}
				
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
				{section.start_time}-{section.end_time}<br/>
				{section.term}
				<!--{@html section.description}-->
			</p>
			<div class="flex h-6 flex-row justify-end">
				{#if (window.matchMedia('(max-width: 600px)').matches || hovered) && !selected.sections.includes(section)}
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
			<div class="flex h-6 flex-row justify-end gap-2">
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
					<div transition:fade={{ duration: 200 }}>
						<Trash
							onclick={() => {
								selected.sections = selected.sections = selected.sections.filter(c => c !== section);
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
