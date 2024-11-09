import { json } from '@sveltejs/kit';
import { search } from '$lib/server/db';

export const POST = async ({ request }) => {
	const { department, course_number, desc } = await request.json();
	
	const courses = search(department,course_number,desc, 50);
	
	return new Response(JSON.stringify(courses),{status: 201})
}