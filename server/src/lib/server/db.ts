import { Database } from "bun:sqlite";
import { CourseQuery,SectionQuery,Course,Section} from "$lib/query.svelte";

const db = new Database("static/catalog.db", { readonly: true });



export function search_course(data: CourseQuery, limit: number): Course[] {
    let query = `SELECT * FROM courses WHERE 1=1`;
    const params: string[] = [];

    if (data.department) {
        const lowerDept = data.department.toLowerCase();
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

    if (data.course_number) {
        query += ` AND course_number LIKE ?`;
        params.push(`%${data.course_number}%`);
    }

    if (data.description) {
        const lowerDesc = data.description.toLowerCase();
        query += ` AND (LOWER(description) LIKE ? OR LOWER(title) LIKE ?)`;
        params.push(`%${lowerDesc}%`, `%${lowerDesc}%`);
    }

    if (data.description) {
        query += ` ORDER BY 
            CASE 
                WHEN LOWER(department) = ? THEN 0
                WHEN LOWER(department) LIKE ? THEN 1
                ELSE 2
            END`;
        params.push(data.description.toLowerCase(), `%${data.description.toLowerCase()}%`);
    }

    query += ` LIMIT ${limit}`;
    const rows = db.query(query).all(...params);
    return rows.map(row => Course.fromRow(row));
}

export function search_section(data: SectionQuery, limit: number): Section[] {
    let query = `SELECT * FROM sections WHERE 1=1`;
    const params: string[] = [];

    if (data.department) {
        const lowerDept = data.department;
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

    if (data.course_number) {
        query += ` AND course_number LIKE ?`;
        params.push(`%${data.course_number}%`);
    }

    if (data.daysOfWeek.includes("mo")) query += ` AND monday = 'true'`;
    if (data.daysOfWeek.includes("tu")) query += ` AND tuesday = 'true'`;
    if (data.daysOfWeek.includes("we")) query += ` AND wednesday = 'true'`;
    if (data.daysOfWeek.includes("th")) query += ` AND thursday = 'true'`;
    if (data.daysOfWeek.includes("fr")) query += ` AND friday = 'true'`;

    //TODO
    if (data.startTime) {
        //query += ` AND LOWER(friday) = ?)`;
        //params.push(`%${data.friday}%`);
    }
    if (data.endTime) {
        //query += ` AND LOWER(friday)  ?)`;
        //params.push(`%${data.friday}%`);
    }

    query += ` LIMIT ${limit}`;
    const rows = db.query(query).all(...params);
    console.log(rows.length);
    return rows.map(row => Section.fromRow(row));
}