<script lang="ts">
    import type { PageData } from './$types';
    import Week from '$lib/Week.svelte';

    let { data }: { data: PageData } = $props();
</script>

<Week/>