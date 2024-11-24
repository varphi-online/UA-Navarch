import { search_course, search_section } from '$lib/server/database';
import { Course, CourseQuery, Section, SectionQuery } from '$lib/query.svelte';

export const POST = async ({ request }) => {
	if (request.headers.get('search-type') === 'course') {
		const { course, offset, limit }: { course: CourseQuery, offset: number, limit: number } = await request.json();
		//console.log(course);
		const courses: Course[] = search_course(course,offset, limit);
		return new Response(JSON.stringify(courses), { status: 201 });
	} else if (request.headers.get('search-type') === 'section') {
		const { section, course, offset, limit }: { section: SectionQuery, course: CourseQuery, offset: number, limit: number } = await request.json();
		//console.log({section,course});
		const sections: Section[] = search_section(section,course,offset, limit);
		return new Response(JSON.stringify(sections), { status: 201 });
	}
	console.log("no type");
};
