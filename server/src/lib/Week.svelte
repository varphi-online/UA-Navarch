<script lang="ts">
</script>

<div>
	<div
		class="absolute grid h-full w-full grid-cols-5 [&>*]:border-x-[1px] [&>*]:border-b-[1px] [&>*]:border-solid [&>*]:border-gray-400 [&>*]:bg-transparent"
        style="grid-template-rows: repeat(17, minmax(0, 2fr));"
	>
    {#each {length: 85} as _, i}
		<div>&nbsp;</div>
	{/each}
	</div>
	<div
		class=" absolute grid h-full w-full grid-cols-5 justify-center gap-x-[1px]"
		style="grid-template-rows: repeat(204, minmax(0, 2fr));"
	>
		<div class="col-start-1 row-start-[37] row-end-[47] rounded-lg bg-red-600">&nbsp;</div> <!--8:00-8:50-->
        <div class=" col-start-4 row-start-[73] row-end-[107] rounded-lg bg-red-600">&nbsp;</div> <!--11:00-13:50-->
	</div>
</div>
