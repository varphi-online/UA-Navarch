import { Database } from 'bun:sqlite';
import { CourseQuery, SectionQuery, Course, Section } from '$lib/query.svelte';

const db = new Database('static/catalog.db', { readonly: true, safeIntegers: true });

export function search_course(course_query: CourseQuery, limit: number): Course[] {
	let query = `SELECT * FROM courses WHERE 1=1`;
	const params: string[] = [];

	if (course_query.department) {
		const lowerDept = course_query.department.toLowerCase();
		query += ` AND (
            CASE 
                WHEN LOWER(department) = ? THEN 1
                WHEN LOWER(department) LIKE ? THEN 2
            END = 1 OR 
            CASE 
                WHEN LOWER(department) = ? THEN 1
                WHEN LOWER(department) LIKE ? THEN 2
            END = 2
        )`;
		params.push(
			lowerDept, // For exact match
			`%${lowerDept}%`, // For partial match
			lowerDept, // Repeated for the second CASE
			`%${lowerDept}%` // Repeated for the second CASE
		);
	}
	const conditions = course_query.attributes
		? course_query.attributes.map((attr) => {
				const mapping = {
					bc: "building_connections = 'true'",
					hum: "humanist = 'true'",
					art: "artist = 'true'",
					ns: "natural_scientist = 'true'",
					ss: "social_scientist = 'true'",
					ec: "entry_course = 'true'",
					xc: "exit_course = 'true'"
				};
				return mapping[attr];
			})
		: [];

	if (conditions.length > 0) {
		query += ` AND (${conditions.join(course_query.attributes.length > 1 ? ' OR ' : ` AND `)})`;
	}

	if (course_query.course_number) {
		query += ` AND course_number LIKE ?`;
		params.push(`%${course_query.course_number}%`);
	}

	if (course_query.description) {
		const lowerDesc = course_query.description.toLowerCase();
		query += ` AND (LOWER(description) LIKE ? OR LOWER(title) LIKE ?)`;
		params.push(`%${lowerDesc}%`, `%${lowerDesc}%`);
	}

	if (course_query.description) {
		query += ` ORDER BY 
            CASE 
                WHEN LOWER(department) = ? THEN 0
                WHEN LOWER(department) LIKE ? THEN 1
                ELSE 2
            END`;
		params.push(
			course_query.description.toLowerCase(),
			`%${course_query.description.toLowerCase()}%`
		);
	}
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	const rows: any[] = db.query(query).all(...params);
	// First, collect all the hashes we need to check
	const hashes = rows.map((row) => BigInt(row.hash));

	// Make a single query to get all relevant sections
	const term_specifier = course_query.term ? `AND term = '${course_query.term}'` : '';
	const sectionsQuery = `
    SELECT DISTINCT hash 
    FROM sections 
    WHERE hash IN (${hashes.join(',')}) 
    ${term_specifier}
`;
	const sectionsWithAvailability = new Set(
		db
			.query(sectionsQuery)
			.all()
            // eslint-disable-next-line @typescript-eslint/no-explicit-any
			.map((row: any) => row.hash)
	);

	// Map the courses with sections info
	const courses = rows.map((row) => {
		const course = Course.fromRow(row);
		course.sections_avail = sectionsWithAvailability.has(BigInt(row.hash));
		return course;
	});

	return courses
		.filter((course) => (!course_query.showHist && course.sections_avail) || course_query.showHist)
		.slice(0, limit);
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	/*
	const rows: any[] = db.query(query).all(...params);
    console.log(rows.length)
    const term_specifier = course_query.term ? "AND term = '" + course_query.term+"'" : '';
	const section_add = rows.map((row) => {
		const c = Course.fromRow(row);
		// Add sections avail attr if section exists in db with same hash, and term
		const q2 = db
			.query(
				`SELECT * FROM sections WHERE hash = ${BigInt(row.hash)} ${term_specifier}`
			)
			.all();
		if (q2.length !== 0) {
			c.sections_avail = true;
		}
		return c;
	});
    console.log(section_add.length)
	//console.log(limit)
	return section_add
		.filter((course) => (!course_query.showHist && course.sections_avail) || course_query.showHist)
		.slice(0, limit);
        */
}

export function search_section(
	section_query: SectionQuery,
	course_query: CourseQuery,
	limit: number
): Section[] {
	// Start with a JOIN between sections and courses
	let query = `
        SELECT sections.* 
        FROM sections 
        JOIN courses ON sections.hash = courses.hash 
        WHERE 1=1
    `;
	const params: string[] = [];

	// Add course-related filters
	if (course_query.department) {
		const lowerDept = course_query.department.toLowerCase();
		query += ` AND (
            CASE 
                WHEN LOWER(courses.department) = ? THEN 1
                WHEN LOWER(courses.department) LIKE ? THEN 2
            END = 1 OR 
            CASE 
                WHEN LOWER(courses.department) = ? THEN 1
                WHEN LOWER(courses.department) LIKE ? THEN 2
            END = 2
        )`;
		params.push(
			lowerDept, // For exact match
			`%${lowerDept}%`, // For partial match
			lowerDept, // Repeated for the second CASE
			`%${lowerDept}%` // Repeated for the second CASE
		);
	}

	if (course_query.course_number) {
		query += ` AND courses.course_number LIKE ?`;
		params.push(`%${course_query.course_number}%`);
	}

	if (course_query.description) {
		const lowerDesc = course_query.description.toLowerCase();
		query += ` AND (LOWER(courses.description) LIKE ? OR LOWER(courses.title) LIKE ?)`;
		params.push(`%${lowerDesc}%`, `%${lowerDesc}%`);
	}

	if (section_query.instructor) {
		const lowerInst = section_query.instructor.toLowerCase();
		query += ` AND (LOWER(sections.instructor) LIKE ?)`;
		params.push(`%${lowerInst}%`);
	}

	// Add course attributes filters
	if (course_query.attributes.includes('bc')) query += ` AND courses.building_connections = 'true'`;
	if (course_query.attributes.includes('hum')) query += ` AND courses.humanist = 'true'`;
	if (course_query.attributes.includes('art')) query += ` AND courses.artist = 'true'`;
	if (course_query.attributes.includes('ns')) query += ` AND courses.natural_scientist = 'true'`;
	if (course_query.attributes.includes('ss')) query += ` AND courses.social_scientist = 'true'`;
	if (course_query.attributes.includes('ec')) query += ` AND courses.entry_course = 'true'`;
	if (course_query.attributes.includes('xc')) query += ` AND courses.exit_course = 'true'`;

	// Add section-specific filters
	if (section_query.daysOfWeek.length != 0) {
		if (section_query.daysOfWeek.includes('mo')) {
			query += ` AND sections.monday = 'true'`;
		} else {
			query += ` AND sections.monday = 'false'`;
		}
		if (section_query.daysOfWeek.includes('tu')) {
			query += ` AND sections.tuesday = 'true'`;
		} else {
			query += ` AND sections.tuesday = 'false'`;
		}
		if (section_query.daysOfWeek.includes('we')) {
			query += ` AND sections.wednesday = 'true'`;
		} else {
			query += ` AND sections.wednesday = 'false'`;
		}
		if (section_query.daysOfWeek.includes('th')) {
			query += ` AND sections.thursday = 'true'`;
		} else {
			query += ` AND sections.thursday = 'false'`;
		}
		if (section_query.daysOfWeek.includes('fr')) {
			query += ` AND sections.friday = 'true'`;
		} else {
			query += ` AND sections.friday = 'false'`;
		}
	}

	if (section_query.startTime) {
		query += ` AND (sections.start_time >= ? OR sections.start_time = 'TBD')`;
		params.push(section_query.startTime);
	}

	if (section_query.endTime) {
		query += ` AND (sections.end_time <= ? OR sections.end_time = 'TBD')`;
		params.push(section_query.endTime);
	}

	if (section_query.class_number) {
		query += ` AND sections.class_number LIKE ?`;
		params.push(`%${section_query.class_number}%`);
	}

	if (course_query.term) {
		query += ` AND LOWER(sections.term) LIKE ?`;
		params.push(`%${course_query.term.toLowerCase()}%`);
	}

	// Add ordering similar to course search when description is provided
	if (course_query.description) {
		query += ` ORDER BY 
            CASE 
                WHEN LOWER(courses.department) = ? THEN 0
                WHEN LOWER(courses.department) LIKE ? THEN 1
                ELSE 2
            END`;
		params.push(
			course_query.description.toLowerCase(),
			`%${course_query.description.toLowerCase()}%`
		);
	}

	//query += ` LIMIT ${limit}`;
	const rows = db.query(query).all(...params);
	//console.log(query,params)
	return rows.map((row) => Section.fromRow(row)).slice(0, limit);
}
/*
function generateSchedules(required_sections: Section[], required_courses: Course[]): Section[][] {
    const possible_sections: Section[][] = [[]];
    required_courses.forEach((course)=>{
        const rows = db.query(`SELECT * FROM sections WHERE department = ${course.department} AND course_number = ${course.course_number}`).all();
        possible_sections.push(rows.map(row => Section.fromRow(row)));
    });

    const generated_schedules: Section[][] = [[]];
    possible_sections.forEach()
}
*/

export function generateSchedules(
	required_sections: Section[] = [],
	required_courses: Course[]
): Section[][] {
	// Get all possible sections for each required course
	const courseSections: Section[][] = required_courses.map((c) =>
		db
			.prepare('SELECT * FROM sections WHERE department = ? AND course_number = ?')
			.all(...[c.department, c.course_number])
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
