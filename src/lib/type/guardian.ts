export interface Guardian {
    id?: number;
    name: string;
    phone: string;
    address: string;
    photo?: string;
    photo_blob?: Uint8Array;
    relation?: string;
}
