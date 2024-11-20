import { CourseQuery, SectionQuery, Section, Course } from '$lib/query.svelte';
import { search_section, search_course } from '$lib/server/database';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ params }) => {
    const {department, course: course_number, term, section: section_number} = params;
    const course_data: Course[] = search_course(new CourseQuery(department,course_number,null,null,[]),1)
    const section_data: Section[] = search_section(new SectionQuery(null,null,null,null,null,[],null,null,null,section_number),new CourseQuery(department,course_number,null,null,[],null,null,term.replace("-"," ")),1)
	return {course_data: structuredClone(course_data),section_data: structuredClone(section_data)};
};