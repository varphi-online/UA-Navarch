import { Database } from "bun:sqlite";

const db = new Database("static/catalog.db", { readonly: true });

class Course {
    constructor(
        public department: string,
        public course_number: string,
        public title: string,
        public description: string,
        public units: string,
        public prerequisites: string
    ) {}

/* eslint-disable  @typescript-eslint/no-explicit-any */
    static fromRow(row: any): Course {
        return new Course(
            row.department,
            row.course_number,
            row.title,
            row.description,
            row.units,
            row.prerequisites
        );
    }
}

export function search(dept: string|null, num: string|null, desc: string|null, limit: number): Course[] {
    let query = `SELECT * FROM courses WHERE 1=1`;
    const params: string[] = [];

    if (dept) {
        const lowerDept = dept.toLowerCase();
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

    if (num) {
        query += ` AND course_number LIKE ?`;
        params.push(`%${num}%`);
    }

    if (desc) {
        const lowerDesc = desc.toLowerCase();
        query += ` AND (LOWER(description) LIKE ? OR LOWER(title) LIKE ?)`;
        params.push(`%${lowerDesc}%`, `%${lowerDesc}%`);
    }

    if (dept) {
        query += ` ORDER BY 
            CASE 
                WHEN LOWER(department) = ? THEN 0
                WHEN LOWER(department) LIKE ? THEN 1
                ELSE 2
            END`;
        params.push(dept.toLowerCase(), `%${dept.toLowerCase()}%`);
    }

    query += ` LIMIT ${limit}`;
    const rows = db.query(query).all(...params);
    return rows.map(row => Course.fromRow(row));
}