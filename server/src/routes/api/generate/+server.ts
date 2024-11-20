import { generateSchedules } from '$lib/server/database';
import { Course, Section} from '$lib/query.svelte';

export const POST = async ({ request }) => {
	const { sections, courses, term}: { sections: Section[], courses: Course[], term: string } = await request.json();
	const schedules: Section[][] = generateSchedules(sections,courses,term);
	return new Response(JSON.stringify(schedules.slice(0,50)), { status: 201 });
};
