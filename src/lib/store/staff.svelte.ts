import type { Staff } from '$lib/types/staff';
import { invoke } from '@tauri-apps/api/core';
import { toast } from './toast.svelte';

class StaffStore {
    data: Staff[] = [];
    fetched = false;
    reactiveCounter = $state(0);

    private insert(staff: Staff) {
        this.data.push(staff);
    }

    async fetch() {
        if (this.fetched) return;

        try {
            const staffList: Staff[] = await invoke('get_staff');
            this.data = staffList;
            this.fetched = true;
            this.reactiveCounter++;
        } catch (err) {
            console.error('Failed to fetch staff:', err);
            toast.set({ message: 'Failed to fetch staff', type: 'error' });
        }
    }

    async get(): Promise<Staff[]> {
        await this.fetch();
        return this.data;
    }

    getById(id: number): Staff | undefined {
        const _ = this.reactiveCounter;
        return this.data.find((s) => s.id === id);
    }

    add(staff: Staff): void {
        this.insert(staff);
        this.reactiveCounter++;
    }

    update(updated: Staff): void {
        const index = this.data.findIndex((s) => s.id === updated.id);
        if (index !== -1) {
            this.data[index] = updated;
            this.reactiveCounter++;
        }
    }

    remove(id: number): void {
        const index = this.data.findIndex((s) => s.id === id);
        if (index !== -1) {
            this.data.splice(index, 1);
            this.reactiveCounter--;
        }
    }

    async getNonTeachers(): Promise<Staff[]> {
        await this.fetch();
        return this.data.filter((s) => !s.is_teacher);
    }

    async getTeachers(): Promise<Staff[]> {
        await this.fetch();
        return this.data.filter((s) => s.is_teacher);
    }
}

export const staff = new StaffStore();
