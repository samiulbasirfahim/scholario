interface Toast {
    message: string;
    type: 'success' | 'error' | 'warning';
}

export const toast: {
    toasts: Toast[];
    set(toast: Toast): void;
} = $state({
    toasts: [],

    set(toast: Toast) {
        this.toasts = [...this.toasts, toast];

        setTimeout(() => {
            this.toasts = this.toasts.slice(1); // also trigger reactivity
        }, 5000);
    }
});
