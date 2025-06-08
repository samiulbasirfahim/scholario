class Setting {
    data = $state({
        title: '',
        address: '',
        mail: '',
        phone: ''
    });

    set title(title: string) {
        this.data.title = title;
    }

    get title() {
        return this.data.title;
    }

    set mail(mail: string) {
        this.data.mail = mail;
    }

    get mail() {
        return this.data.mail;
    }

    set address(address: string) {
        this.data.address = address;
    }

    get address() {
        return this.data.address;
    }

    set phone(phone: string) {
        this.data.phone = phone;
    }

    get phone() {
        return this.data.phone;
    }

    async fetch() {
        this.data = {
            title: 'Madrasaye Abu Bakar Siddique',
            mail: 'principle@madrasaabubakarsiddique.com',
            phone: '+88096939393',
            address: 'Vati-Ghagra, Momenshahi'
        };
    }
}

export const setting = new Setting();
