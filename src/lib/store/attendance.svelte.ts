import type { Attendance } from '$lib/types/attendance';
import { invoke } from '@tauri-apps/api/core';
import { toast } from './toast.svelte';

class AttendanceStore {
    data = new Map<number, Map<string, Attendance[]>>();
    fetchedKeys = new Set<string>();
    reactiveCounter = $state(0);

    private makeKey(student_id: number, year: number, month: number): string {
        const paddedMonth = month.toString().padStart(2, '0');
        return `${student_id}-${year}-${paddedMonth}`;
    }

    private insert(attendance: Attendance) {
        const studentId = attendance.student_id;
        const key = attendance.date.slice(0, 7); // "YYYY-MM"

        if (!this.data.has(studentId)) {
            this.data.set(studentId, new Map());
        }

        const studentMap = this.data.get(studentId)!;
        if (!studentMap.has(key)) {
            studentMap.set(key, []);
        }

        studentMap.get(key)!.push(attendance);
    }

    private fillCache(attendances: Attendance[]) {
        for (const record of attendances) {
            this.insert(record);
        }
    }

    async fetch(student_id: number, year?: string, month?: string) {
        const now = new Date();
        const yearNum = year ? parseInt(year) : now.getFullYear();
        const monthNum = month ? parseInt(month) : now.getMonth() + 1;

        const key = this.makeKey(student_id, yearNum, monthNum);
        if (this.fetchedKeys.has(key)) return;

        try {
            const args = {
                student_id,
                year: yearNum,
                month: monthNum
            };

            const records: Attendance[] = await invoke('get_attendance_by_student', args);

            this.fillCache(records);
            this.fetchedKeys.add(key);
            this.reactiveCounter++;
            toast.warning('Fetching attendance.');
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

        const key = `${year}-${month}`;
        const studentMap = this.data.get(student_id);
        if (!studentMap) return [];

        return studentMap.get(key) ?? [];
    }

    add(record: Attendance) {
        this.insert(record);
        this.reactiveCounter++;
    }

    update(updated: Attendance) {
        const key = updated.date.slice(0, 7);
        const studentId = updated.student_id;
        const studentMap = this.data.get(studentId);
        if (!studentMap) return;

        const list = studentMap.get(key);
        if (!list) return;

        const index = list.findIndex((r) => r.id === updated.id);
        if (index !== -1) {
            list[index] = updated;
            this.reactiveCounter++;
        }
    }

    remove(studentId: number, attendanceId: number) {
        const studentMap = this.data.get(studentId);
        if (!studentMap) return;

        for (const [month, list] of studentMap) {
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
