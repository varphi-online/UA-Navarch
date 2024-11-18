<script lang="ts">
	import { Section } from './query.svelte';
	let { sections }: { sections: Section[] } = $props();

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

<div class="relative aspect-video w-full rounded-3xl border-2 border-gray-400 ml-10">
	<div
		class="absolute grid h-full w-8 -left-10 -top-[6px] grid-cols-1 [&>*]:border-solid [&>*]:border-transparent [&>*]:border"
		style="grid-template-rows: repeat(17, minmax(0, 2fr));"
	>
		<div class="row-start-[1] text-xs">&nbsp;</div>
		<p class="row-start-2 flex flex-row items-center justify-end overflow-clip text-xs">6:00</p>
		<div class="row-start-[3] text-xs">&nbsp;</div>
		<p class="row-start-4 flex flex-row items-center justify-end overflow-clip text-xs">8:00</p>
		<div class="row-start-[5] text-xs">&nbsp;</div>
		<p class="row-start-6 flex flex-row items-center justify-end overflow-clip text-xs">10:00</p>
		<div class="row-start-[7] text-xs">&nbsp;</div>
		<p class="row-start-8 flex flex-row items-center justify-end overflow-clip text-xs">12:00</p>
		<div class="row-start-[9] text-xs">&nbsp;</div>
		<p class="row-start-10 flex flex-row items-center justify-end overflow-clip text-xs">2:00</p>
		<div class="row-start-[11] text-xs">&nbsp;</div>
		<p class="row-start-12 flex flex-row items-center justify-end overflow-clip text-xs">4:00</p>
		<div class="row-start-[13] text-xs">&nbsp;</div>
		<p class="row-start-[14] flex flex-row items-center justify-end overflow-clip text-xs">6:00</p>
		<div class="row-start-[15] text-xs">&nbsp;</div>
		<p class="row-start-[16] flex flex-row items-center justify-end overflow-clip text-xs">8:00</p>
		<div class="row-start-[17] text-xs">&nbsp;</div>
		<p class="row-start-[18] flex flex-row items-center justify-end overflow-clip text-xs">10:00</p>
	</div>
	<div
		class="absolute grid h-full w-full grid-cols-5

		
		[&>*:nth-child(5n)]:border-l
		[&>*:nth-child(5n-1)]:border-l
		[&>*:nth-child(5n-2)]:border-l
		[&>*:nth-child(5n-3)]:border-l
		[&>*:nth-last-child(-n+5)]:border-b-0
		[&>*]:border-b
		[&>*]:border-solid
		[&>*]:border-gray-400
		[&>*]:bg-transparent"
		style="grid-template-rows: repeat(17, minmax(0, 2fr));"
	>
		{#each { length: 85 } as _, i}
			<div>&nbsp;</div>
		{/each}
	</div>
	<div
		class=" absolute grid h-full w-full grid-cols-5 justify-center"
		style="grid-template-rows: repeat(204, minmax(0, 2fr));"
	>
		<!--
		<div class="col-start-1 row-start-[37] row-end-[47] rounded-lg bg-red-600">&nbsp;</div> 
        <div class=" col-start-4 row-start-[73] row-end-[107] rounded-lg bg-red-600">&nbsp;</div>
		<div class="col-start-3 rounded-lg bg-blue-600" style={`grid-row-start: ${timeConv("12:00")};grid-row-end: ${timeConv("13:50")};`}>&nbsp;</div>
		-->
		{#each sections as section}
			{@const start = timeConv(section.start_time)}
			{@const end = timeConv(section.end_time)}
			{@const color = objectToColor(section)}
			{#each days(section) as day, day_index}
				{#if day && section.visible}
					<div
						class="flex items-center justify-center rounded-lg"
						style={`background-color: ${color};grid-column-start: ${day_index + 1};grid-row-start: ${start};grid-row-end: ${end};`}
					>
						<p class="text-sm">
							{section.department}
							{section.course_number}
							{section.section_number}
						</p>
					</div>
				{/if}
			{/each}
		{/each}
	</div>
</div>
