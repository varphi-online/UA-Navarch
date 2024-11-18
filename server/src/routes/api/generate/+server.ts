import { generateSchedules } from '$lib/server/database';
import { Course, Section} from '$lib/query.svelte';

export const POST = async ({ request }) => {
	const { sections, courses}: { sections: Section[], courses: Course[] } = await request.json();
	const schedules: Section[][] = generateSchedules(sections,courses);
	return new Response(JSON.stringify(schedules.slice(0,50)), { status: 201 });
};
