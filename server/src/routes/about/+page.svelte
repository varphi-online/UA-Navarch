<script lang="ts">
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();
</script>

<svelte:head>
   <title>About | Navarch</title>
</svelte:head>

<div class=" mt-8 flex w-full flex-col items-center gap-8 mb-10">
	<h1 class="text-3xl">What is Navarch?</h1>
	<p class="w-[80vw] lg:w-1/3">
		<b>Navarch</b>, a portmanteau of "Navigate" and "Architect" is a replacement tool for the
		University of Arizona's Class Search feature.<br /><br />
		It was created due to the poor user experience I had when selecting courses and trying to piece together
		a coherent schedule while the site kept going down. My intent for this project is to be able to see
		the wide variety of courses offered by the UofA without the headache that is the StudentCenter.<br
		/><br />
		This webApp offers a live-updating search of the catalog as it appears on the clunky
		<a
			class="text-blue-500 underline"
			href="https://studentcenter.arizona.edu/app/ui/public/ps/course-catalog?tab=DEFAULT"
			>UofA Course Catalog</a
		>. You can build out, or automatically have a schedule generated for you in a visual way based
		off of the classes you need to take. Please share this with other people! I wanted to make this
        to help <strong>everyone</strong> with picking classes.
	</p>
	<h1 class="text-3xl">FAQ</h1>
	<ul class="w-[80vw] lg:w-1/3 list-disc space-y-4 px-6 text-gray-800">
		<li>
			<p>
				<strong>Q.</strong> When does course information update?<br />
				<strong>A.</strong> Course data updates at ~4:00 AM on Sunday mornings.
			</p>
		</li>
		<li>
			<p>
				<strong>Q.</strong> Can I add to my cart from here?<br />
				<strong>A.</strong> No, this is completely independent from the UofA.
			</p>
		</li>
		<li>
			<p>
				<strong>Q.</strong> How do I... Where is...?<br />
				<strong>A.</strong> I've tried to make everything self-explanatory, but if you have
				questions or suggestions, please
				<a class="text-blue-600 underline hover:text-blue-700" href="mailto:afuhrmann@arizona.edu"
					>email me</a
				>!
			</p>
		</li>
        <li>
			<p>
				<strong>Q.</strong> Does this work on my phone?<br />
				<strong>A.</strong> Yes, this is designed with a mobile-first mentality, so it should work regardless of
                the platform your are on. <strong>However</strong> saved classes/sections are stored <em>per-device</em>.
			</p>
		</li>
	</ul>
</div>
