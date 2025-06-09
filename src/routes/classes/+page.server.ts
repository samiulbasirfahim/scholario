import type { Page } from '@sveltejs/kit';

export const load = (page: Page) => {
    return {
        selectedClass: page.url.searchParams.get('selectedClass')
    };
};
