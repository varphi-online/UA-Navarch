import { search } from '$lib/server/db';

export const POST = async ({ request }) => {
	const { department, course_number, desc,limit } = await request.json();
	
	const courses = search(department, course_number, desc,limit);
	
	return new Response(JSON.stringify(courses),{status: 201})
}