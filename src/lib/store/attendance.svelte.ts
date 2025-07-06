import { toast } from '$lib/store/toast.svelte';
import type { Attendance } from '$lib/types/attendance';
import { invoke } from '@tauri-apps/api/core';

class AttendanceStore {
    data = new Map<number, Map<string, Attendance[]>>();

    fetchedKeys = new Set<string>();
    reactiveCounter = $state(0);

    private insert(attendance: Attendance) {
        const { student_id, date: date } = attendance;
        const key = date.slice(0, 7);

        if (!this.data.has(student_id)) {
            console.log('Student entry missing');
            this.data.set(student_id, new Map());
        }

        const studentMap = this.data.get(student_id);

        if (!studentMap?.has(key)) {
            studentMap?.set(key, []);
        }

        const store: Attendance[] = studentMap?.get(key) ?? [];
        const exist = store.find((a) => a.date === attendance.date);

        if (exist) {
            const new_store = store.filter((a) => a.date !== attendance.date);
            new_store.push(attendance);
            studentMap?.set(key, new_store);
        } else {
            store.push(attendance);
        }
    }

    private fillCache(attendances: Attendance[]) {
        for (const attendance of attendances) this.insert(attendance);
    }

    private makeKey(student_id: number, year: number, month: number): string {
        const paddedMonth = month.toString().padStart(2, '0');
        return `${student_id}-${year}-${paddedMonth}`;
    }

    async fetch(student_id: number, year?: string, month?: string) {
        const today = new Date();

        const yearNum = year ? parseInt(year) : today.getFullYear();
        const monthNum = month ? parseInt(month) : today.getMonth() + 1;

        const key = this.makeKey(student_id, yearNum, monthNum);
        if (this.fetchedKeys.has(key)) return;

        try {
            const attendances: Attendance[] = await invoke('get_attendance_by_student', {
                student_id,
                year: yearNum,
                month: monthNum
            });

            console.log(attendances);

            this.fillCache(attendances);
            this.fetchedKeys.add(key);
            this.reactiveCounter++;
        } catch (err) {
            console.error('Failed to fetch attendance:', err);
            toast.error('Failed to fetch attendance.');
        }
    }

    async get(student_id: number, ym?: string): Promise<Attendance[]> {
        const now = new Date();
        const year = ym ? ym.slice(0, 4) : now.getFullYear().toString();
        const month = ym ? ym.slice(5, 7) : (now.getMonth() + 1).toString().padStart(2, '0');

        await this.fetch(student_id, year, month);

        const studentMap = this.data.get(student_id);
        if (!studentMap) return [];
        return studentMap.get(`${year}-${month}`) ?? [];
    }

    add(record: Attendance) {
        this.insert(record);
        this.reactiveCounter++;
    }

    remove(studentId: number, attendanceId: number) {
        const studentMap = this.data.get(studentId);
        if (!studentMap) return;

        for (const [_, list] of studentMap) {
            const index = list.findIndex((r) => r.id === attendanceId);
            if (index !== -1) {
                list.splice(index, 1);
                this.reactiveCounter--;
                break;
            }
        }
    }
}

export const attendanceStore = new AttendanceStore();

import type { AttendanceStaff } from '$lib/types/attendance';

class StaffAttendanceStore {
    data = new Map<number, Map<string, AttendanceStaff[]>>();
    fetchedKeys = new Set<string>();
    reactiveCounter = $state(0);

    private insert(attendance: AttendanceStaff) {
        const { staff_id, date } = attendance;
        const key = date.slice(0, 7); // format: YYYY-MM

        if (!this.data.has(staff_id)) {
            this.data.set(staff_id, new Map());
        }

        const staffMap = this.data.get(staff_id)!;

        if (!staffMap.has(key)) {
            staffMap.set(key, []);
        }

        const store = staffMap.get(key)!;
        const exist = store.find((a) => a.date === attendance.date);

        if (exist) {
            const newStore = store.filter((a) => a.date !== attendance.date);
            newStore.push(attendance);
            staffMap.set(key, newStore);
        } else {
            store.push(attendance);
        }
    }

    private fillCache(attendances: AttendanceStaff[]) {
        for (const attendance of attendances) {
            this.insert(attendance);
        }
    }

    private makeKey(staff_id: number, year: number, month: number): string {
        const paddedMonth = month.toString().padStart(2, '0');
        return `${staff_id}-${year}-${paddedMonth}`;
    }

    async fetch(staff_id: number, year?: string, month?: string) {
        const today = new Date();
        const yearNum = year ? parseInt(year) : today.getFullYear();
        const monthNum = month ? parseInt(month) : today.getMonth() + 1;

        const key = this.makeKey(staff_id, yearNum, monthNum);
        if (this.fetchedKeys.has(key)) return;

        try {
            const attendances: AttendanceStaff[] = await invoke('get_attendance_by_staff', {
                staffId: staff_id,
                year: yearNum,
                month: monthNum
            });

            this.fillCache(attendances);
            this.fetchedKeys.add(key);
            this.reactiveCounter++;
        } catch (err) {
            console.error('Failed to fetch staff attendance:', err);
            toast.error('Failed to fetch staff attendance.');
        }
    }

    async get(staff_id: number, ym?: string): Promise<AttendanceStaff[]> {
        const now = new Date();
        const year = ym ? ym.slice(0, 4) : now.getFullYear().toString();
        const month = ym ? ym.slice(5, 7) : (now.getMonth() + 1).toString().padStart(2, '0');

        await this.fetch(staff_id, year, month);

        const staffMap = this.data.get(staff_id);
        if (!staffMap) return [];
        return staffMap.get(`${year}-${month}`) ?? [];
    }

    add(record: AttendanceStaff) {
        this.insert(record);
        this.reactiveCounter++;
    }

    remove(staffId: number, attendanceId: number) {
        const staffMap = this.data.get(staffId);
        if (!staffMap) return;

        for (const [_, list] of staffMap) {
            const index = list.findIndex((r) => r.id === attendanceId);
            if (index !== -1) {
                list.splice(index, 1);
                this.reactiveCounter--;
                break;
            }
        }
    }
}

export const staffAttendanceStore = new StaffAttendanceStore();
