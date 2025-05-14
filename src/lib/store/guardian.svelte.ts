import type { Guardian } from '$lib/types/guardian';
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
