export type AttendanceState = 'PRESENT' | 'ABSENT' | `LATE-${string}`;

export interface Attendance {
    id: number;
    student_id: number;
    date: string;
    status: AttendanceState;
}

export interface AttendanceStaff {
    id: number;
    staff_id: number;
    date: string;
    status: AttendanceState;
}
