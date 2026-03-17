import { query } from '$app/server';
import { getTemplates as sql_Temps, getTemplateTags as sql_Tags } from '$lib/server/meme_maker.ts';

// NOTE: These look useless. They're not. Svelte is translating these remote
// functions into (public!) APIs which client-side components can call.
// TODO: When we get Redis, we can/should move this into a cached microservice.

export const getTemplates = query('unchecked', async()=>{
	return sql_Temps();
});

export const getTemplateTags = query('unchecked', async()=>{
	return sql_Tags();
});