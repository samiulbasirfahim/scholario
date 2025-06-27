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
