<script lang="ts">
    import { page } from '$app/stores';
	import type { Course } from '$lib/query.svelte';
	import type { PageData } from './$types';
    let { data }: { data: PageData } = $props();
    let course = data.course_data.at(0);
    import { getContext } from 'svelte';
</script>

{$page.params.department} {$page.params.course}
{#if course !== undefined}
    <h1>{course.title}</h1>
    <p>{@html course.description}</p>
{:else}
    Not a course
{/if}