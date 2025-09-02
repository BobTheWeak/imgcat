import { Readable } from 'node:stream';
import { createHash } from 'node:crypto';
import { S3Client, PutObjectCommand } from '@aws-sdk/client-s3';
import { NodeHttpHandler } from '@smithy/node-http-handler';
import { Agent } from 'https';
import { Agent as HttpAgent } from 'http';
import 'dotenv/config';

import { error } from '@sveltejs/kit';
import { getDbConn } from '$lib/server/dbpool.ts';

async function getMediaIdIfExists(sha256:Buffer):int {
	let conn;
	try {
		conn = await getDbConn();
		const response = await conn.query({
			sql:"SELECT Posts.GetMediaIdIfExists(?);",
			rowsAsArray: true
		}, [sha256]);

		return response[0][0];
	} catch(e) {
		// If it's a custom error message, we can send it to the browser, otherwise no
		console.log(e);
		if(e['sqlState'] == '45000') {
			error(400, e['sqlMessage'] || 'Unknown error');
		} else {
			error(400, 'Unknown');
		}
	} finally {
		if(conn){await conn.release()}
	}
}

async function getFilename():string {
	let conn;
	try {
		conn = await getDbConn();
		const response = await conn.query({
			sql:"SELECT Posts.GetFilename();",
			rowsAsArray: true
		});

		return response[0][0];
	} catch(e) {
		// If it's a custom error message, we can send it to the browser, otherwise no
		console.log(e);
		if(e['sqlState'] == '45000') {
			error(400, e['sqlMessage'] || 'Unknown error');
		} else {
			error(400, 'Unknown');
		}
	} finally {
		if(conn){await conn.release()}
	}
}


async function UploadFileToCloudFlareR2(data:any):any {
	if(!data.buffer || !data.size || !data.filename || !data.mime) {
		console.log('missing params');
		return null;
	}

	// Support both Cloudflare R2 and local S3-compatible storage (MinIO/NAS)
	// Prioritize S3_ENDPOINT if explicitly set, otherwise check for R2 config
	const hasExplicitEndpoint = process.env.S3_ENDPOINT && process.env.S3_ENDPOINT !== '';
	const isCloudflareR2 = !hasExplicitEndpoint && process.env.CFR2_ACCOUNT_ID && !process.env.CFR2_ACCOUNT_ID.includes('minioadmin');
	
	let endpoint;
	if (hasExplicitEndpoint) {
		// Use explicit S3_ENDPOINT (local NAS/MinIO)
		endpoint = process.env.S3_ENDPOINT;
		console.log('Using explicit S3 endpoint:', endpoint);
	} else if (isCloudflareR2) {
		// Use Cloudflare R2
		endpoint = `https://${process.env.CFR2_ACCOUNT_ID}.r2.cloudflarestorage.com`;
		console.log('Using Cloudflare R2 endpoint:', endpoint);
	} else {
		// Fallback to default MinIO
		endpoint = 'http://localhost:9000';
		console.log('Using default MinIO endpoint:', endpoint);
	}
	
	// Force HTTP for SSL issues (convert https:// to http://)
	const forceHttp = process.env.S3_FORCE_HTTP === 'true' || process.env.S3_FORCE_HTTP === '1';
	if (forceHttp && !isCloudflareR2) {
		endpoint = endpoint.replace('https://', 'http://');
		console.log('Forced HTTP mode, using endpoint:', endpoint);
	}

	// SSL certificate configuration
	const ignoreSsl = process.env.S3_IGNORE_SSL === 'true' || process.env.S3_IGNORE_SSL === '1';
	const isHttps = endpoint.startsWith('https://');
	
	let requestHandler = undefined;
	if (ignoreSsl && isHttps) {
		console.log('Using SSL ignore mode for endpoint:', endpoint);
		// For HTTPS with SSL ignore, use minimal HTTPS agent config
		requestHandler = new NodeHttpHandler({
			httpsAgent: new Agent({
				rejectUnauthorized: false,
				checkServerIdentity: () => undefined
			}),
			connectionTimeout: 30000,
			socketTimeout: 30000
		});
	} else if (!isHttps) {
		console.log('Using HTTP mode for endpoint:', endpoint);
		// For HTTP, use standard HTTP agent
		requestHandler = new NodeHttpHandler({
			httpAgent: new HttpAgent(),
			connectionTimeout: 30000,
			socketTimeout: 30000
		});
	} else {
		console.log('Using default HTTPS mode for endpoint:', endpoint);
	}

	const client = new S3Client({
		endpoint: endpoint,
		credentials: {
			accessKeyId: process.env.CFR2_ACCESS_KEY_ID,
			secretAccessKey: process.env.CFR2_SECRET_ACCESS_KEY
		},
		region: isCloudflareR2 ? 'auto' : 'us-east-1',
		forcePathStyle: !isCloudflareR2, // Required for MinIO/NAS, not for R2
		...(requestHandler && { requestHandler })
	});
	const cmd = new PutObjectCommand({
		Body: data.buffer,
		ContentLength: data.size,
		Bucket: process.env.CFR2_BUCKET,
		Key: data.filename,
		ContentType: data.mime
	});
	const res = client.send(cmd);

	console.log(res);

	return res;
}

async function CreateSimpleMedia(data:any):int {
	if(!data.user_id || !data.filename || !data.mime || !data.sha256) {
		console.log("CreateSimpleMedia missing params");
		console.log(data);
		return null;
	}

	let conn;
	try {
		conn = await getDbConn();
		const response = await conn.query({
			sql:"SELECT Posts.CreateSimpleMedia(?,?,?,?);",
			rowsAsArray: true
		}, [data.user_id, data.filename, data.mime, data.sha256]);
		
		return response[0][0];
	} catch(e) {
		// If it's a custom error message, we can send it to the browser, otherwise no
		console.log(e);
		if(e['sqlState'] == '45000') {
			error(400, e['sqlMessage'] || 'Unknown error');
		} else {
			error(400, 'Unknown');
		}
	} finally {
		if(conn){await conn.release()}
	}
}

async function CreateSimplePost(data:any):int {
	if(!data.user_id || !data.media_id) {
		console.log("CreateSimplePost missing params");
		console.log(data);
		return null;
	}

	let conn;
	try {
		conn = await getDbConn();
		const response = await conn.query({
			sql:"SELECT Posts.GetLinkByPostId(Posts.CreateSimplePost(?,?,?,?));",
			rowsAsArray: true
		}, [data.user_id, data.title, data.media_id, data.description]);

		return response[0][0]; // Returns the link
	} catch(e) {
		// If it's a custom error message, we can send it to the browser, otherwise no
		console.log(e);
		if(e['sqlState'] == '45000') {
			error(400, e['sqlMessage'] || 'Unknown error');
		} else {
			error(400, 'Unknown');
		}
	} finally {
		if(conn){await conn.release()}
	}
}

export async function CreatePostAndUpload(user_id:int, file:File, val:any):any {

	// Download the file & keep it for a bit
	const file_buff = await file.bytes();
	
	// SHA256 hash it
	const hash = createHash('sha256');
	hash.update(file_buff);
	const sha256 = hash.digest();

	// Compare it to existing files
	let media_id = await getMediaIdIfExists(sha256);

	// If not exists in DB
	if(!media_id) {
		console.log('Uploading new media');
		// Get a unique Filename, generated by the DB
		const filename = await getFilename();

		// Upload to Cloudflare R2
		const cf_upload = await UploadFileToCloudFlareR2({
			'buffer': file_buff,
			'size': file.size,
			'filename': filename,
			'mime': val.mime
		});

		// Upload to database
		media_id = await CreateSimpleMedia({
			'user_id': user_id,
			'filename': filename,
			'mime': val.mime,
			'sha256': sha256
		});
	}

	const post_link = await CreateSimplePost({
		'user_id': user_id,
		'media_id': media_id
		//'title': null,
		//'description': null
	});
	
	return post_link;
}

export async function CreatePostAndAttach(file:File, val:any):any {
	console.log('CreatePostAndAttach');
	console.log(file);
	console.log(val);
	return 1;
}

