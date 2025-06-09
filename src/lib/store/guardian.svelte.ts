import type { Guardian, StudentRelationship } from '$lib/types/guardian';
import { invoke } from '@tauri-apps/api/core';
import { toast } from './toast.svelte';

export const guardians = $state({
    data: [] as Guardian[],
    set(list: Guardian[]) {
        this.data = list;
    },

    get(id: number) {
        return this.data.find((c) => c.id === id);
    },

    update(id: number, updated: Guardian) {
        const index = this.data.findIndex((c) => c.id === id);
        if (index !== -1) {
            this.data[index] = updated;
        }
    },

    add(item: Guardian) {
        this.data.push(item);
    },
    remove(id: number) {
        this.data = this.data.filter((c) => c.id !== id);
    },
    async fetch() {
        try {
            const fetched = await invoke<Guardian[]>('get_guardians');
            this.set(fetched);
        } catch (err) {
            console.error('Error fetching classes:', err);
            toast.set({ message: 'Failed to fetch classes', type: 'error' });
        }
    }
});

class StudentRelationships {
    private data: Map<number, StudentRelationship[]> = new Map(); // student_id -> [relationships]
    reactiveCounter = $state(0);
    private fetchedStudents: Set<number> = new Set(); // track fetched student_ids
    private allFetched: boolean = false;
    private pendingFetches: Map<number | 'all', Promise<void>> = new Map(); // prevent duplicate fetches

    private insert(rel: StudentRelationship) {
        const rels = this.data.get(rel.student_id) ?? [];
        this.reactiveCounter++;
        if (!rels.some((r) => r.id === rel.id)) {
            rels.push(rel);
            this.data.set(rel.student_id, rels);
        }
    }

    private fillCache(relationships: StudentRelationship[]) {
        for (const rel of relationships) {
            this.insert(rel); // deduplicated insert
        }
    }

    async fetch(student_id?: number): Promise<void> {
        if (this.allFetched || (student_id !== undefined && this.fetchedStudents.has(student_id)))
            return;

        const key = student_id ?? 'all';
        if (this.pendingFetches.has(key)) {
            return this.pendingFetches.get(key);
        }

        const fetchPromise = (async () => {
            try {
                const args: { student_id?: number } = {};
                if (student_id !== undefined) args.student_id = student_id;

                const fetched: StudentRelationship[] = await invoke('get_student_relationships', args);

                this.fillCache(fetched);

                if (student_id !== undefined) {
                    this.fetchedStudents.add(student_id);
                } else {
                    this.allFetched = true;
                }
            } catch (err) {
                console.error('Failed to fetch student relationships:', err);
                toast.set({ message: 'Failed to fetch student relationships', type: 'error' });
            } finally {
                this.pendingFetches.delete(key);
            }
        })();

        this.pendingFetches.set(key, fetchPromise);
        return fetchPromise;
    }

    get(student_id: number): StudentRelationship[] {
        return this.data.get(student_id) ?? [];
    }

    add(rel: StudentRelationship) {
        this.insert(rel); // already deduplicated
    }

    update(id: number, updated: StudentRelationship) {
        const rels = this.data.get(updated.student_id);
        if (!rels) return;

        const index = rels.findIndex((r) => r.id === id);
        this.reactiveCounter++;
        if (index !== -1) {
            rels[index] = updated;
        }
    }

    remove(id: number) {
        for (const [studentId, rels] of this.data.entries()) {
            this.reactiveCounter++;
            const filtered = rels.filter((r) => r.id !== id);

            if (filtered.length === 0) {
                this.data.delete(studentId);
            } else {
                this.data.set(studentId, filtered);
            }
        }
    }
}

export const studentRelationships = new StudentRelationships();
