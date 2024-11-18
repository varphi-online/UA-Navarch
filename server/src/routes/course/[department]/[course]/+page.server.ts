import { CourseQuery, Course, SectionQuery, Section } from '$lib/query.svelte';
import { search_course, search_section } from '$lib/server/database';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ params }) => {
    const {department, course: course_number} = params;
    const course_data: Course[] = search_course(new CourseQuery(department,course_number,null,null,[]),1)
    const section_data: Section[] = search_section(new SectionQuery(null,null,null,null,null,[]),new CourseQuery(department,course_number,null,null,[]),1000)
	return {course_data: structuredClone(course_data), section_data: structuredClone(section_data)};
};