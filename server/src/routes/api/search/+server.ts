import { search_course, search_section } from '$lib/server/db';
import { Course, CourseQuery, Section, SectionQuery } from '$lib/query.svelte';

export const POST = async ({ request }) => {
	if (request.headers.get('search-type') === 'course') {
		const { course, limit }: { course: CourseQuery; limit: number } = await request.json();
		const courses: Course[] = search_course(course, limit);
		return new Response(JSON.stringify(courses), { status: 201 });
	} else if (request.headers.get('search-type') === 'section') {
		const { section, limit }: { section: SectionQuery; limit: number } = await request.json();
		const sections: Section[] = search_section(section, limit);
		return new Response(JSON.stringify(sections), { status: 201 });
	}
	console.log("no type");
};
