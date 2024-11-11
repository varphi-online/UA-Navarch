<script lang="ts">
import { Input } from '$lib/components/ui/input';
import { Course, CourseQuery } from './query.svelte';
import * as Select from '$lib/components/ui/select';
let { courses = $bindable() }: {courses: Course[]} = $props();

let desc:string | null = $state(null);
let dept:string | null = $state(null);
let num: string | null = $state(null);
let attrs = $state([])
$effect(() => console.log(attrs));

let query: CourseQuery = $derived(new CourseQuery(dept,num,null,desc));

async function search(local: CourseQuery): Promise<Course[]> {
    let response = await fetch('/api/search', {
				method: 'POST',
				body: JSON.stringify({ course: local, limit: 20 }),
				headers: {
					'content-type': 'application/json',
					'search-type': 'course'
				}
			});
    let out = response.json();
    return out;
}

function highlight(text: string): string {
		const regex = new RegExp(`(${desc})`, 'gi');
		return text.replace(regex, `<mark>$1</mark>`);
	}

$effect(()=>{
    if(!((!desc||desc.length<1)&&(!dept||dept.length<1)&&(!num||num.length<1))){
        search(query).then(result => {
            result.forEach(course => course.description = highlight(course.description));
            courses = result;
        })
    } else {
        courses = []
    }});
</script>

<div class="flex">
<Input type="text" bind:value={desc} placeholder="Keyword(s)" class="border-solid border-y-2 border-l-2 border-gray-500 rounded-bl-2xl rounded-tl-2xl rounded-tr-none rounded-br-none m-0"/>
<Input type="text" bind:value={dept} placeholder="Department" class="border-solid border-l-0 border-r-gray-300 border-y-2 border-gray-500 rounded-none m-0"/>
<Input type="text" bind:value={num} placeholder="Course Number" class="border-solid border-l-0 border-r-gray-300 border-y-2 border-gray-500 rounded-none m-0"/>

<Select.Root multiple bind:selected={attrs}>
    <Select.Trigger  class="border-solid border-y-2 border-r-2 border-gray-500 rounded-bl-none rounded-tl-none rounded-tr-2xl rounded-br-2xl m-0">
      <Select.Value placeholder="Course Attribute(s)"/>
    </Select.Trigger>
    <Select.Content>
      <Select.Item value="Gen Ed: Building Connections">Gen Ed: Building Connections</Select.Item>
      <Select.Item value="Gen Ed: Exploring Perspectives">Gen Ed: Exploring Perspectives</Select.Item>
      <Select.Item value="Gen Ed: Humanist">Gen Ed: Humanist</Select.Item>
      <Select.Item value="Gen Ed: Artist">Gen Ed: Artist</Select.Item>
      <Select.Item value="Gen Ed: Social Scientist">Gen Ed: Social Scientist</Select.Item>
    </Select.Content>
  </Select.Root>


</div>