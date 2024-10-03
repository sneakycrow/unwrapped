import { env } from '$env/dynamic/private';
import type { Actions } from './$types';
import { redirect } from '@sveltejs/kit';

const SPOTIFY_AUTH_URL = `${env.API_URL}/auth/spotify`;
export const actions = {
	default: async (event) => {
		throw redirect(302, SPOTIFY_AUTH_URL);
	}
} satisfies Actions;
