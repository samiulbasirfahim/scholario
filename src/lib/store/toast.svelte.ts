interface ToastType {
    message: string;
    type: 'success' | 'error' | 'warning';
}

class Toast {
    toasts = $state<ToastType[]>([]);

    private addToast(toast: ToastType) {
        if (toast === undefined) return;
        this.toasts = [...this.toasts, toast];

        setTimeout(() => {
            this.toasts = this.toasts.slice(1);
        }, 5000);
    }

    error(message: string) {
        this.addToast({ message, type: 'error' });
    }

    warning(message: string) {
        this.addToast({ message, type: 'warning' });
    }

    success(message: string) {
        this.addToast({ message, type: 'success' });
    }

    set(toast: ToastType) {
        this.addToast(toast);
    }

    get get() {
        return [...this.toasts];
    }
}

export const toast = new Toast();
