import { error } from '@sveltejs/kit';
import fs from 'fs';
import type { RequestHandler } from './$types';

export const GET = (async ({url}) => {
    let link = url.searchParams.get('url');
    console.log(link)
    if (!link) {
        return new Response(fs.readFileSync('./src/routes/comms/data.txt', 'utf8'));
    }
    fs.writeFileSync('./src/routes/comms/data.txt', link);
    return new Response(link);
}) as RequestHandler;