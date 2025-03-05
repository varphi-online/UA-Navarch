/* eslint-disable @typescript-eslint/no-explicit-any */
import { Database } from 'bun:sqlite';
import { CourseQuery, SectionQuery, Course, Section } from '$lib/query.svelte';

console.log(process.env.DATA);
const db = new Database(process.env.DATA + "/catalog.db", { readonly: true, safeIntegers: true });

export function search_course(
	course_query: CourseQuery,
	offset: number = 0,
	limit: number,
): Course[] {
	const params: any[] = [];
	const conditions: string[] = [];

	// Build base query with prepared statements
	if (course_query.department) {
		const lowerDept = course_query.department.toLowerCase();
			conditions.push(`
                LOWER(department) = ? `);
			params.push(lowerDept);

	}

	// Map attributes to conditions using a more efficient approach
	if (course_query.attributes?.length) {
		const attributeMapping: Record<string, string> = {
			bc: 'building_connections',
			hum: 'humanist',
			art: 'artist',
			ns: 'natural_scientist',
			ss: 'social_scientist',
			ec: 'entry_course',
			xc: 'exit_course'
		};

		const attrConditions = course_query.attributes
			.map((attr) => `${attributeMapping[attr]} = 'true'`)
			.join(course_query.attributes.length > 1 ? ' OR ' : ' AND ');

		if (attrConditions) {
			conditions.push(`(${attrConditions})`);
		}
	}

	if (course_query.course_number) {
		conditions.push('course_number LIKE ?');
		params.push(`%${course_query.course_number}%`);
	}

	if (course_query.description) {
		const lowerDesc = course_query.description.toLowerCase();
		conditions.push('(LOWER(description) LIKE ? OR LOWER(title) LIKE ?)');
		params.push(`%${lowerDesc}%`, `%${lowerDesc}%`);
	}

	// Construct the main query with conditions
	const query = `
        WITH filtered_courses AS (
            SELECT * 
            FROM courses 
            WHERE ${conditions.length ? conditions.join(' AND ') : '1=1'}
            ${
							course_query.description
								? `
                ORDER BY 
                CASE 
                    WHEN LOWER(department) = ? THEN 0
                    WHEN LOWER(department) LIKE ? THEN 1
                    ELSE 2
                END
            `
								: ''
						}
        ),
        course_sections AS (
            SELECT DISTINCT hash, course_hash
            FROM sections
            WHERE course_hash IN (SELECT hash FROM filtered_courses)
            ${course_query.term ? 'AND term = ?' : ''}
        )
        SELECT 
            c.*,
            CASE WHEN s.course_hash IS NOT NULL THEN 1 ELSE 0 END as has_sections
        FROM filtered_courses c
        LEFT JOIN course_sections s ON c.hash = s.course_hash
        ${!course_query.showHist ? 'WHERE s.hash IS NOT NULL' : ''}
        LIMIT ? OFFSET ?
    `;

	// Add remaining params
	if (course_query.description) {
		params.push(
			course_query.description.toLowerCase(),
			`%${course_query.description.toLowerCase()}%`
		);
	}
	if (course_query.term) {
		params.push(course_query.term);
	}
	params.push(limit);
	params.push(offset);

	// Execute optimized query
	const rows = db.query(query).all(...params);

	// Transform results
	const courses = rows.map((row: any) => {
		const course = Course.fromRow(row);
		course.sections_avail = Boolean(row.has_sections);
		return course;
	});
	return courses;
}

export function search_section(
	section_query: SectionQuery,
	course_query: CourseQuery,
	offset: number = 0,
	limit: number,
): Section[] {
	const params: any[] = [];
	const conditions: string[] = [];

	// Course-related conditions
	if (course_query.department) {
		const lowerDept = course_query.department.toLowerCase();
			conditions.push(`
                LOWER(courses.department) = ? `);
			params.push(lowerDept);

	}

	if (course_query.course_number) {
		conditions.push('courses.course_number LIKE ?');
		params.push(`%${course_query.course_number}%`);
	}

	if (course_query.description) {
		const lowerDesc = course_query.description.toLowerCase();
		conditions.push('(LOWER(courses.description) LIKE ? OR LOWER(courses.title) LIKE ?)');
		params.push(`%${lowerDesc}%`, `%${lowerDesc}%`);
	}

	// Course attributes
	if (course_query.attributes?.length) {
		const attributeMapping: Record<string, string> = {
			bc: 'building_connections',
			hum: 'humanist',
			art: 'artist',
			ns: 'natural_scientist',
			ss: 'social_scientist',
			ec: 'entry_course',
			xc: 'exit_course'
		};

		const attrConditions = course_query.attributes
			.map((attr) => `courses.${attributeMapping[attr]} = 'true'`)
			.join(' AND ');

		if (attrConditions) {
			conditions.push(`(${attrConditions})`);
		}
	}

	// Section-specific conditions
	if (section_query.instructor) {
		const lowerInst = section_query.instructor.toLowerCase();
		conditions.push('LOWER(sections.instructor) LIKE ?');
		params.push(`%${lowerInst}%`);
	}

	// Optimize days of week filtering
	if (section_query.daysOfWeek.length > 0) {
		const dayMapping = {
			mo: 'monday',
			tu: 'tuesday',
			we: 'wednesday',
			th: 'thursday',
			fr: 'friday'
		};

		const dayConditions = Object.entries(dayMapping).map(
			([short, full]) =>
				`sections.${full} = ${section_query.daysOfWeek.includes(short) ? "'true'" : "'false'"}`
		);

		conditions.push(dayConditions.join(' AND '));
	}

	// Time conditions
	if (section_query.startTime) {
		conditions.push("(sections.start_time >= ? OR sections.start_time = 'TBD')");
		params.push(section_query.startTime);
	}

	if (section_query.endTime) {
		conditions.push("(sections.end_time <= ? OR sections.end_time = 'TBD')");
		params.push(section_query.endTime);
	}

	// Section number conditions
	if (section_query.class_number) {
		conditions.push('sections.class_number LIKE ?');
		params.push(`%${section_query.class_number}%`);
	}

	if (section_query.section_number) {
		conditions.push('sections.section_number LIKE ?');
		params.push(`%${section_query.section_number}%`);
	}

	// Term condition
	if (course_query.term) {
		conditions.push('LOWER(sections.term) LIKE ?');
		params.push(`%${course_query.term.toLowerCase()}%`);
	}

	// Construct the optimized query using CTEs
	const query = `
        WITH filtered_sections AS (
            SELECT 
                sections.*,
                courses.department,
                courses.description as course_description,
                courses.title as course_title
            FROM sections
            JOIN courses ON sections.course_hash = courses.hash
            WHERE ${conditions.length ? conditions.join(' AND ') : '1=1'}
            ${
							course_query.description
								? `
                ORDER BY 
                CASE 
                    WHEN LOWER(courses.department) = ? THEN 0
                    WHEN LOWER(courses.department) LIKE ? THEN 1
                    ELSE 2
                END
            `
								: ''
						}
            LIMIT ? OFFSET ?
        )
        SELECT * FROM filtered_sections
    `;

	// Add ordering params if needed
	if (course_query.description) {
		params.push(
			course_query.description.toLowerCase(),
			`%${course_query.description.toLowerCase()}%`
		);
	}

	// Add limit
	params.push(limit);
	params.push(offset);

	// Execute query and transform results
	const rows = db.query(query).all(...params);
	return rows.map((row) => Section.fromRow(row));
}

export function generateSchedules(
	required_sections: Section[] = [],
	required_courses: Course[],
	term: string
): Section[][] {
	term = term.toLowerCase();
	// Get all possible sections for each required course
	const courseSections: Section[][] = required_courses.map((c) =>
		db
			.prepare(
				'SELECT * FROM sections WHERE department = ? AND course_number = ? AND LOWER(term) LIKE ?'
			)
			.all(...[c.department, c.course_number, term])
			.map(Section.fromRow)
			.filter((section) => section.start_time != 'TBD')
	);
	/*
	courseSections.forEach((course) => {
		console.log('- ' + course.length);
	});
    */

	// Start with required sections as the base
	const all_schedules: Section[][] = [
		...courseSections,
		...required_sections.map((section) => [section])
	].reduce(
		(permutations, currentArray) => {
			return permutations.flatMap((permutation) =>
				currentArray.map((item) => [...permutation, item])
			);
		},
		[[]]
	);
	//console.log(all_schedules.length);

	function hasTimeConflict(section1: Section, section2: Section): boolean {
		if (section1 === section2) return false;

		// Check if sections share any days
		const days1 = [
			section1.monday,
			section1.tuesday,
			section1.wednesday,
			section1.thursday,
			section1.friday
		];
		const days2 = [
			section2.monday,
			section2.tuesday,
			section2.wednesday,
			section2.thursday,
			section2.friday
		];

		const shareDay = days1.some((day, index) => day === 'true' && days2[index] === 'true');

		if (!shareDay) return false;

		// Check time overlap - return true if there IS a conflict
		return !(section1.end_time <= section2.start_time || section1.start_time >= section2.end_time);
	}

	const out: Section[][] = all_schedules.filter((schedule) => {
		// A schedule is valid if it has no conflicts between any pair of sections
		for (let i = 0; i < schedule.length; i++) {
			for (let j = i + 1; j < schedule.length; j++) {
				if (hasTimeConflict(schedule[i], schedule[j])) {
					return false; // Invalid schedule
				}
			}
		}
		return true; // Valid schedule
	});

	// Filter out empty schedules and return
	return out;
}
