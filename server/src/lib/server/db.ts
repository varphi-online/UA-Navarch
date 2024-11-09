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
        query += ` AND LOWER(department) LIKE ?`;
        params.push(`%${dept.toLowerCase()}%`);
    }

    if (num) {
        query += ` AND course_number LIKE ?`;
        params.push(`%${num}%`);
    }

    if (desc) {
        query += ` AND LOWER(description) LIKE ?`;
        params.push(`%${desc.toLowerCase()}%`);
    }

    query += `limit ${limit}`;

    const rows = db.query(query).all(...params);
    return rows.map(row => Course.fromRow(row));
}