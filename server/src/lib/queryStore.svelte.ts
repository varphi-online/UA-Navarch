import { persisted } from 'svelte-persisted-store'

export interface QueryParams {
    desc: string | null;
    dept: string | null;
    num: string | null;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    attrs: any[];
    instructor: string | null;
    class_num: string | null;
    startTime: string;
    endTime: string;
    daysOfWeek: string[];
    filters: {value:string}[];
    searchType: {
        value: string;
        label: string;
    }
  }
  
  export const query = persisted('query', <QueryParams>{
    desc: null,
    dept: null,
    num: null,
    attrs: [],
    instructor: null,
    class_num: null,
    startTime: '08:00',
    endTime: '18:00',
    daysOfWeek: [],
    filters: [
      { value: 'description' },
      { value: 'departments' },
      { value: 'course_number' },
      { value: 'days' },
      { value: 'times' }
    ]
  });
