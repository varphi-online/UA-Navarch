<script lang="ts">
	import { getContext } from 'svelte';
	import { Course, Section } from './query.svelte';
	const selected: { courses: Course[]; sections: Section[] } = getContext('selected');

	function timeConv(time: string): number {
		if (time.toLowerCase() == 'tbd') return -1;
		let [hr, min] = time.split(':').map((e) => parseInt(e));
		return (hr - 5) * 12 + Math.ceil(min / 5) + 1;
	}

	function days(section: Section): boolean[] {
		let out = [false, false, false, false, false];
		if (section.monday == 'true') out[0] = true;
		if (section.tuesday == 'true') out[1] = true;
		if (section.wednesday == 'true') out[2] = true;
		if (section.thursday == 'true') out[3] = true;
		if (section.friday == 'true') out[4] = true;
		return out;
	}
	function objectToColor(obj): string {
		let hash = JSON.stringify(obj)
			.split('')
			.reduce((hash, char) => {
				return char.charCodeAt(0) + ((hash << 5) - hash);
			}, 0);

		return '#' + ('000000' + (Math.abs(hash) & 0xffffff).toString(16)).slice(-6);
	}
</script>

<div class="relative h-1/2 w-1/2">
	<div
		class="absolute grid h-full w-full grid-cols-5 [&>*]:border-x-[1px] [&>*]:border-b-[1px] [&>*]:border-solid [&>*]:border-gray-400 [&>*]:bg-transparent"
		style="grid-template-rows: repeat(17, minmax(0, 2fr));"
	>
		{#each { length: 85 } as _, i}
			<div>&nbsp;</div>
		{/each}
	</div>
	<div
		class=" absolute grid h-full w-full grid-cols-5 justify-center gap-x-[1px]"
		style="grid-template-rows: repeat(204, minmax(0, 2fr));"
	>
		<!--
		<div class="col-start-1 row-start-[37] row-end-[47] rounded-lg bg-red-600">&nbsp;</div> 
        <div class=" col-start-4 row-start-[73] row-end-[107] rounded-lg bg-red-600">&nbsp;</div>
		<div class="col-start-3 rounded-lg bg-blue-600" style={`grid-row-start: ${timeConv("12:00")};grid-row-end: ${timeConv("13:50")};`}>&nbsp;</div>
		-->
		{#each selected.sections as section}
			{@const start = timeConv(section.start_time)}
			{@const end = timeConv(section.end_time)}
			{@const color=objectToColor(section)}
			{#each days(section) as day, day_index}
				{#if day}
					<div
						class="rounded-lg flex justify-center items-center"
						style={`background-color: ${color};grid-column-start: ${day_index + 1};grid-row-start: ${start};grid-row-end: ${end};`}
					>
						<p class="text-sm">{section.department} {section.course_number}</p>
					</div>
				{/if}
			{/each}
		{/each}
	</div>
</div>
