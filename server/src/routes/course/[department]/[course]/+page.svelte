<script lang="ts">
    import { page } from '$app/stores';
	import type { Course } from '$lib/query.svelte';
	import type { PageData } from './$types';
    let { data }: { data: PageData } = $props();
    let course = data.course_data.at(0);
    import { getContext } from 'svelte';
    import type { Writable } from 'svelte/store';
	const selected: Writable<string[]> = getContext('selected')
</script>

{$page.params.department} {$page.params.course}
{#if course !== undefined}
    <h1>{course.title}</h1>
    <p>{$selected}</p>
{:else}
    Not a course
{/if}