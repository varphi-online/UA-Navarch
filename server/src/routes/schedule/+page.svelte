<script lang="ts">
    import type { PageData } from './$types';
    import Week from '$lib/Week.svelte';
    import { Search } from 'lucide-svelte';
    let { data }: { data: PageData } = $props();
    import { browser } from "$app/environment";
</script>
<a href="/" class="z-10">
	<div
		class=" cursor fixed left-3 top-3 rounded-3xl border-2 border-solid border-slate-500 border-opacity-10 bg-white bg-opacity-75 p-2 transition-all duration-300 hover:border-opacity-100 hover:bg-opacity-100"
	>
		<Search />
	</div></a
>
<div class=" relative w-[100vw] h-[100vh] p-50 flex justify-center items-center -z-50">
    &nbsp;
    <Week/>
</div>