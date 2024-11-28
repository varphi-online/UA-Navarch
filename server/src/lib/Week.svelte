<script lang="ts">
	import { Section } from './query.svelte';
	let { sections, class: internalClass }: { sections: Section[]; class? } = $props();

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
	function objectToColor(obj: any): string {
		const hash = JSON.stringify(obj)
			.split('')
			.reduce((acc, char) => ((acc << 5) - acc + char.charCodeAt(0)) >>> 0, 0);

		const bases = [
			[0x9b, 0x27, 0x27],
			[0x88, 0x38, 0x47],
			[0x73, 0x41, 0x60],
			[0x5e, 0x45, 0x78],
			[0x46, 0x46, 0x8f],
			[0x27, 0x44, 0xa5]
		];

		const base = bases[hash % 6];
		const lighten = (hash >>> 8) / 0xffffff; // Use next bits for lightness
		const toHex = (n: number) =>
			Math.min(255, Math.round(n + ((255 - n) * lighten) / 3))
				.toString(16)
				.padStart(2, '0');

		return `#${toHex(base[0])}${toHex(base[1])}${toHex(base[2])}`;
	}
</script>

<div class={`${internalClass} relative ml-10 aspect-video w-full rounded-3xl border-2 contain-inline-size border-gray-400`}>
	<div
		class="absolute h-[91%] top-[2%] lg:top-[3.44117%] right-full mr-2 lg:h-[92.917647058%] flex flex-col gap-1"
	>
		{#each ['6:00AM', '8:00AM', '10:00AM', '12:00PM', '2:00PM', '4:00PM', '6:00PM', '8:00PM'] as time}
			<p class="text-right text-[0.65em] flex-1">
				{time}
			</p>
		{/each}
	</div>
	<div
		class="absolute h-fit w-full bottom-full mb-1.5 flex"
	>
		{#each ['Monday','Tuesday','Wednesday','Thursday','Friday'] as time}
			<p class="text-center text-[0.65em] flex-1">
				{time}
			</p>
		{/each}
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
						<p class="white text-[2cqw] text-white lg:text-[1cqw]">
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
