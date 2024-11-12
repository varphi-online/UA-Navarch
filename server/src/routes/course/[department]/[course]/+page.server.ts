import { CourseQuery, Course } from '$lib/query.svelte';
import { search_course } from '$lib/server/db';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ params }) => {
    const {department, course: course_number} = params;
    const course_data: Course[] = search_course(new CourseQuery(department,course_number),1)
	return {course_data: structuredClone(course_data)};
};