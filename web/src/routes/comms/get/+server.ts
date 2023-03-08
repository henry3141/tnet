import { error } from '@sveltejs/kit';
import fs from 'fs';
import type { RequestHandler } from './$types';

export const GET = (async ({}) => {
    return new Response(fs.readFileSync('./src/routes/comms/data.txt', 'utf8'));
}) as RequestHandler;