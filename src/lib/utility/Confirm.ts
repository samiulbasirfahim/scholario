let dialogEl: HTMLDialogElement;

export function setDialogEl(dEl: HTMLDialogElement) {
    dialogEl = dEl;
}

export async function Confirm() {
    dialogEl.showModal();

    return new Promise((res, rej) => {
        try {
            dialogEl.addEventListener('close', () => {
                res(dialogEl.returnValue === 'yes');
            });
        } catch (err) {
            rej(err);
        }
    });
}
