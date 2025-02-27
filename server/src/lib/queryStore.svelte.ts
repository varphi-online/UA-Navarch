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
    term: string | null;
    showHist: boolean;
    filters: {value:string}[];
    searchType: {
        value: string;
        label: string;
    };
    showOpen: boolean
}