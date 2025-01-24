import { apiUrl } from '$lib/api';
import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

interface Logs {
	owner: string;
	item_id: number | undefined;
	item_type: number | undefined;
	action: string;
	faction: string | undefined;
	message: string | undefined;
	date: Date;
}

export const load = (async ({ cookies }) => {
	let req = await fetch(`${apiUrl}/ucp/fm/logs/get_all`, {
		headers: {
			cookie: cookies.get('auth_token') as string,
			faction: cookies.get('selected_faction') as string
		}
	});
	if (req.ok) {
		let list: Logs[] = await req.json();
		return {
			logs: list
		};
	} else {
		throw redirect(302, '/ucp/fm/logs');
	}
}) satisfies PageServerLoad;
