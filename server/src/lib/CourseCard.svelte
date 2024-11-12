<script lang="ts">
	import { Course } from '$lib/query.svelte';
	let { course }: { course: Course } = $props();
	import Lock from 'lucide-svelte/icons/lock';
	import * as Tooltip from '$lib/components/ui/tooltip/index.js';
	console.log(course.prerequisites);
</script>

<div class=" h-80 overflow-hidden rounded-3xl border-2 border-solid border-gray-300 p-4">
	<a href={`/course/${course.department}/${course.course_number}`}>
		<h3 class="inline w-fit rounded-2xl bg-gray-200 px-2 text-lg font-semibold">
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
				<Tooltip.Trigger class="rounded-2x float-right inline w-fit">
					<Lock />
				</Tooltip.Trigger>
				<Tooltip.Content>
					<p>Prerequisites Required</p>
				</Tooltip.Content>
			</Tooltip.Root>
		{/if}
        <div class="flex my-2">
		    <h4 class="text-lg font-semibold">{@html course.title}</h4>
        </div>
	</a>
	<p>{@html course.description}</p>
</div>
