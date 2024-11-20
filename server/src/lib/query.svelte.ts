export class CourseQuery {
    constructor(
        public department: string|null = null,
        public course_number: string|null = null,
        public title: string|null = null,
        public description: string|null = null,
        public attributes: (string|null)[] = null,
        public units: string|null = null,
        public prerequisites: string|null = null,
        public term: string|null = null,
        public showHist: boolean = false,
    ) {}
}

export class Course {
    public sections_avail: boolean = false;
    constructor(
        public department: string,
        public course_number: string,
        public title: string,
        public description: string,
        public units: string,
        public prerequisites: string,
        public requirements: string,
        public equivalences: string,
        public building_connections: string,
        public artist: string,
        public humanist: string,
        public natural_scientist: string,
        public social_scientist: string,
        public entry_course: string,
        public exit_course: string,
    ) {}

/* eslint-disable  @typescript-eslint/no-explicit-any */
    static fromRow(row: any): Course {
        return new Course(
            row.department,
            row.course_number,
            row.title,
            row.description,
            row.units,
            row.prerequisites,
            row.requirements,
            row.equivalences,
            row.building_connections,
            row.artist,
            row.humanist,
            row.natural_scientist,
            row.social_scientist,
            row.entry_course,
            row.exit_course
        );
    }
}

export class SectionQuery {
    constructor(
        public department: string|null = null,
        public course_number: string|null = null,
        public units: string|null = null,
        public prerequisites: string|null = null,
        public instructor: string|null = null,
        public daysOfWeek: (string|null)[] = null,
        public startTime: string|null = null,
        public endTime: string|null = null,
        public class_number: string|null = null,
        public section_number: string|null = null,
    ) {}
}

export class Section {
    public visible: boolean = true;
    constructor(
        public class_number: string|null = null,
        public department: string|null = null,
        public course_number: string|null = null,
        public section_number: string|null = null,
        public term: string|null = null,
        public status: string|null = null,
        public session: string|null = null,
        public class_components: string|null = null,
        public instruction_mode: string|null = null,
        public class_type: string|null = null,
        public academic_career: string|null = null,
        public start_date: string|null = null,
        public end_date: string|null = null,
        public grading: string|null = null,
        public location: string|null = null,
        public campus: string|null = null,
        public monday: string|null = null,
        public tuesday: string|null = null,
        public wednesday: string|null = null,
        public thursday: string|null = null,
        public friday: string|null = null,
        public start_time: string|null = null,
        public end_time: string|null = null,
        public instructor: string|null = null,
        public class_capacity: string|null = null,
        public enrollment_total: string|null = null,
        public available_seats: string|null = null,
    ) {}

    /* eslint-disable  @typescript-eslint/no-explicit-any */
    static fromRow(row: any): Section {
        return new Section(
            row.class_number,
            row.department,
            row.course_number,
            row.section_number,
            row.term,
            row.status,
            row.session,
            row.class_components,
            row.instruction_mode,
            row.class_type,
            row.academic_career,
            row.start_date,
            row.end_date,
            row.grading,
            row.location,
            row.campus,
            row.monday,
            row.tuesday,
            row.wednesday,
            row.thursday,
            row.friday,
            row.start_time,
            row.end_time,
            row.instructor,
            row.class_capacity,
            row.enrollment_total,
            row.available_seats
        );
    }
}