<script lang="ts">
	import { Circle, X } from 'lucide-svelte';
	import { page } from '$app/stores';
	import type { Course, Section } from '$lib/query.svelte';
	import type { PageData } from './$types';
	let { data }: { data: PageData } = $props();
	let course: Course = data.course_data.at(0);
	let section: Section = data.section_data.at(0);
	import { Progress } from '$lib/components/ui/progress';
	import * as Table from '$lib/components/ui/table';
	import * as Select from '$lib/components/ui/select';
	import { getContext } from 'svelte';
	import type { Writable } from 'svelte/store';
	import { type QueryParams } from '$lib/queryStore.svelte';

	const queryParams: Writable<QueryParams> = getContext('queryParams');

	let terms: string[] = ['Spring 2025', 'Summer 2025', 'Fall 2025'];
	let term = $state({ value: terms[0], label: terms[0] });
	let termFiltered: { [key: string]: Section[] } = {
		'': data.section_data
	};
	terms.forEach((term) => {
		termFiltered[term] = data.section_data.filter((s) => s.term.includes(term));
	});
</script>

{$page.params.department}
{$page.params.course}
{#if course !== undefined && section !== undefined }
	<h1>{course.title}</h1>
	<p>{@html course.description}</p>
	<p>{@html section.term}</p>
	<p>{@html section.section_number}</p>
{:else}
	Not a valid course or section
{/if}