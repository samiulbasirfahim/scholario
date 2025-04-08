import { invoke } from '@tauri-apps/api/core';
import { toast } from './toast.svelte';

import type { Section, Subject, Class } from '$lib/types/class';

export const classes = $state({
    classes: [] as Class[],
    set(list: Class[]) {
        this.classes = list;
    },
    add(item: Class) {
        this.classes.push(item);
    },
    remove(id: number) {
        this.classes = this.classes.filter((c) => c.id !== id);
    },
    async fetch() {
        try {
            const fetched = await invoke<Class[]>('get_classes');
            this.set(fetched);
        } catch (err) {
            console.error('Error fetching classes:', err);
            toast.set({ message: 'Failed to fetch classes', type: 'error' });
        }
    }
});

export const sections = $state({
    sections: [] as Section[],
    set(list: Section[]) {
        this.sections = list;
    },
    add(item: Section) {
        this.sections.push(item);
    },
    remove(id: number) {
        this.sections = this.sections.filter((s) => s.id !== id);
    },
    async fetch() {
        try {
            const fetched = await invoke<Section[]>('get_sections');
            console.log(fetched);
            this.set(fetched);
        } catch (err) {
            console.error('Error fetching sections:', err);
            toast.set({ message: 'Failed to fetch sections', type: 'error' });
        }
    }
});

export const subjects = $state({
    subjects: [] as Subject[],
    set(list: Subject[]) {
        this.subjects = list;
    },
    add(item: Subject) {
        this.subjects.push(item);
    },
    remove(id: number) {
        this.subjects = this.subjects.filter((s) => s.id !== id);
    },
    async fetch() {
        try {
            const fetched = await invoke<Subject[]>('get_subjects');
            this.set(fetched);
        } catch (err) {
            console.error('Error fetching subjects:', err);
            toast.set({ message: 'Failed to fetch subjects', type: 'error' });
        }
    }
});
