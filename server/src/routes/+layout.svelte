<script lang="ts">
    import '../app.css';
    import { setContext } from 'svelte';
	import { writable, type Writable } from 'svelte/store';
    import * as Sheet from '$lib/components/ui/sheet';
    import Button from '$lib/components/ui/button/button.svelte';
    import Bookmark from 'lucide-svelte/icons/bookmark';
    import CourseCard from '$lib/CourseCard.svelte';
    import { Course, Section } from '$lib/query.svelte';

    const selected: Writable<{courses: Course[], sections: Section[]}> = writable({courses: [],sections: []});
    setContext('selected',selected);
</script>
<Sheet.Root>
    <Sheet.Trigger class=" border-opacity-10 hover:border-opacity-100  bg-opacity-75 hover:bg-opacity-100 transition-all duration-300 fixed bottom-3 right-3 rounded-3xl border-solid border-2 border-slate-500 p-2 bg-white">
        <Bookmark/>
    </Sheet.Trigger>
    <Sheet.Content>
      <Sheet.Header>
        <Sheet.Title>Saved courses</Sheet.Title>
        <Sheet.Description>
        {#if $selected.courses.length > 0}
        <h1> Courses </h1>
        <div class="flex flex-col gap-3">
          {#each $selected.courses as course}
            <CourseCard course={course} small={true}/>
          {/each}
        </div>
        {/if}
        {#if $selected.sections.length > 0}
          <h1>Sections</h1>
          {#each $selected.sections as section}
            <p>{section.course_number}</p>
          {/each}
        {/if}
        {#if $selected.courses.length == 0&&$selected.sections.length == 0}
            <p>No saved courses or sections!</p>
        {/if}
        </Sheet.Description>
      </Sheet.Header>
    </Sheet.Content>
  </Sheet.Root>
<slot />
