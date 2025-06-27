import type { Page } from '@sveltejs/kit';

export const load = (page: Page) => {
	return {
		selectedStudent: page.url.searchParams.get('selectedStudent')
	};
};
