import { Database } from "bun:sqlite";
import { CourseQuery,SectionQuery,Course,Section} from "$lib/query.svelte";

const db = new Database("static/catalog.db", { readonly: true });



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
            lowerDept,             // For exact match
            `%${lowerDept}%`,      // For partial match
            lowerDept,             // Repeated for the second CASE
            `%${lowerDept}%`       // Repeated for the second CASE
        );
    }
        if (course_query.attributes.includes("bc")) query += ` AND building_connections = 'true'`;
        if (course_query.attributes.includes("hum")) query += ` AND humanist = 'true'`;
        if (course_query.attributes.includes("art")) query += ` AND artist = 'true'`;
        if (course_query.attributes.includes("ns")) query += ` AND natural_scientist = 'true'`;
        if (course_query.attributes.includes("ss")) query += ` AND social_scientist = 'true'`;
        if (course_query.attributes.includes("ec")) query += ` AND entry_course = 'true'`;
        if (course_query.attributes.includes("xc")) query += ` AND exit_course = 'true'`;

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
        params.push(course_query.description.toLowerCase(), `%${course_query.description.toLowerCase()}%`);
    }

    query += ` LIMIT ${limit}`;
    const rows = db.query(query).all(...params);
    return rows.map(row => Course.fromRow(row));
}
/*
export function search_section(section_query: SectionQuery, limit: number): Section[] {
    let query = `SELECT * FROM sections WHERE 1=1`;
    const params: string[] = [];

    if (section_query.department) {
        const lowerDept = section_query.department;
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
            lowerDept,             // For exact match
            `%${lowerDept}%`,      // For partial match
            lowerDept,             // Repeated for the second CASE
            `%${lowerDept}%`       // Repeated for the second CASE
        );
    }

    if (section_query.course_number) {
        query += ` AND course_number LIKE ?`;
        params.push(`%${section_query.course_number}%`);
    }

    if (section_query.daysOfWeek.includes("mo")) query += ` AND monday = 'true'`;
    if (section_query.daysOfWeek.includes("tu")) query += ` AND tuesday = 'true'`;
    if (section_query.daysOfWeek.includes("we")) query += ` AND wednesday = 'true'`;
    if (section_query.daysOfWeek.includes("th")) query += ` AND thursday = 'true'`;
    if (section_query.daysOfWeek.includes("fr")) query += ` AND friday = 'true'`;

    //TODO
    if (section_query.startTime) {
        //query += ` AND LOWER(friday) = ?)`;
        //params.push(`%${data.friday}%`);
    }
    if (section_query.endTime) {
        //query += ` AND LOWER(friday)  ?)`;
        //params.push(`%${data.friday}%`);
    }

    query += ` LIMIT ${limit}`;
    const rows = db.query(query).all(...params);
    console.log(rows.length);
    return rows.map(row => Section.fromRow(row));
}
    */

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
            lowerDept,             // For exact match
            `%${lowerDept}%`,      // For partial match
            lowerDept,             // Repeated for the second CASE
            `%${lowerDept}%`       // Repeated for the second CASE
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

    // Add course attributes filters
    if (course_query.attributes.includes("bc")) query += ` AND courses.building_connections = 'true'`;
    if (course_query.attributes.includes("hum")) query += ` AND courses.humanist = 'true'`;
    if (course_query.attributes.includes("art")) query += ` AND courses.artist = 'true'`;
    if (course_query.attributes.includes("ns")) query += ` AND courses.natural_scientist = 'true'`;
    if (course_query.attributes.includes("ss")) query += ` AND courses.social_scientist = 'true'`;
    if (course_query.attributes.includes("ec")) query += ` AND courses.entry_course = 'true'`;
    if (course_query.attributes.includes("xc")) query += ` AND courses.exit_course = 'true'`;

    // Add section-specific filters
    if (section_query.daysOfWeek.includes("mo")) query += ` AND sections.monday = 'true'`;
    if (section_query.daysOfWeek.includes("tu")) query += ` AND sections.tuesday = 'true'`;
    if (section_query.daysOfWeek.includes("we")) query += ` AND sections.wednesday = 'true'`;
    if (section_query.daysOfWeek.includes("th")) query += ` AND sections.thursday = 'true'`;
    if (section_query.daysOfWeek.includes("fr")) query += ` AND sections.friday = 'true'`;

    /* TODO
    if (section_query.startTime) {
        query += ` AND sections.start_time >= ?`;
        params.push(section_query.startTime);
    }

    if (section_query.endTime) {
        query += ` AND sections.end_time <= ?`;
        params.push(section_query.endTime);
    }
        */

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

    query += ` LIMIT ${limit}`;
    const rows = db.query(query).all(...params);
    return rows.map(row => Section.fromRow(row));
}