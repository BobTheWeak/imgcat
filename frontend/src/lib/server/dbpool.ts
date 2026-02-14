
import mariadb from 'mariadb';
import 'dotenv/config';
import { building } from '$app/environment';
import { error } from '@sveltejs/kit';

// NOTE: If you're leaking pool connections, it's b/c "vite dev" will
// watch for changes & reload files dynamically. But it will leak the pool.

let appdb = null;
if(!appdb && !building) {
	// Test if we have the required parameters
	let ok = true;
	if(!process.env.IC_DB_HOST) {ok=false;console.error('Missing host for appdb pool: IC_DB_HOST');}
	if(!process.env.IC_DB_PORT) {ok=false;console.error('Missing port for appdb pool: IC_DB_PORT');}
	if(!process.env.IC_DB_USER) {ok=false;console.error('Missing user for appdb pool: IC_DB_USER');}
	if(!process.env.IC_DB_PASS) {ok=false;console.error('Missing pass for appdb pool: IC_DB_PASS');}
	if(ok) {
		// Test if a single, manual query works
		mariadb.createConnection({
			host: process.env.IC_DB_HOST,
			port: process.env.IC_DB_PORT,
			user: process.env.IC_DB_USER,
			password: process.env.IC_DB_PASS
		}).then((conn)=>{
			conn.query("SELECT Posts.GetFilename()")
			.then((res)=>{
				if(res) {
					// Everything checks out!
					// Build a pool and assign it to appdb
					appdb = mariadb.createPool({
						idleTimeout: 60, //sec
						connectionLimit: 10,
						acquireTimeout: 1000,
						connectTimeout: 250,
						queryTimeout: 2000,
						leakDetectionTimeout: 10000,
						host: process.env.IC_DB_HOST,
						port: Number(process.env.IC_DB_PORT) || 3306,
						user: process.env.IC_DB_USER,
						password: process.env.IC_DB_PASS
					});
					console.log('App pool started');
				}
			}).catch((e)=>{
				console.error('Connected, but could not query, using the following params:')
				console.error('Host: ', process.env.IC_DB_HOST || '- null -');
				console.error('Port: ', process.env.IC_DB_PORT || '- null -');
				console.error('User: ', process.env.IC_DB_USER || '- null -');
				console.error('Pass: ', process.env.IC_DB_PASS?'-some value-':'- null -' );
				console.error(e);
			}).finally(()=>{
				conn?.end()
			});
		}).catch((e)=>{
			console.error('Could not establish connection, using the following params:')
			console.error('Host: ', process.env.IC_DB_HOST || '- null -');
			console.error('Port: ', process.env.IC_DB_PORT || '- null -');
			console.error('User: ', process.env.IC_DB_USER || '- null -');
			console.error('Pass: ', process.env.IC_DB_PASS?'-some value-':'- null -' );
			console.error(e);
		});
	}
}


let userdb = null;
if(!userdb && !building) {
	// Test if we have the required parameters
	let ok = true;
	if(!process.env.IC_USERDB_HOST) {ok=false;console.error('Missing host for UserDB pool: IC_USERDB_HOST');}
	if(!process.env.IC_USERDB_PORT) {ok=false;console.error('Missing port for UserDB pool: IC_USERDB_PORT');}
	if(!process.env.IC_USERDB_USER) {ok=false;console.error('Missing user for UserDB pool: IC_USERDB_USER');}
	if(!process.env.IC_USERDB_PASS) {ok=false;console.error('Missing pass for UserDB pool: IC_USERDB_PASS');}
	if(ok) {
		// Test if a single, manual query works
		mariadb.createConnection({
			host: process.env.IC_USERDB_HOST,
			port: process.env.IC_USERDB_PORT,
			user: process.env.IC_USERDB_USER,
			password: process.env.IC_USERDB_PASS
		}).then((conn)=>{
			conn.query("SELECT UserDB.IsUsernameFree('any')")
			.then((res)=>{
				if(res) {
					// Everything checks out!
					// Build a pool and assign it to userdb
					userdb = mariadb.createPool({
						idleTimeout: 60, //sec
						connectionLimit: 5,
						acquireTimeout: 1000,
						connectTimeout: 250,
						queryTimeout: 1000,
						leakDetectionTimeout: 10000,
						host: process.env.IC_USERDB_HOST,
						port: Number(process.env.IC_USERDB_PORT) || 3306,
						user: process.env.IC_USERDB_USER,
						password: process.env.IC_USERDB_PASS
					});
					console.log('UserDB pool started');
				}
			}).catch((e)=>{
				console.error('Connected, but could not query, using the following params:')
				console.error('Host: ', process.env.IC_USERDB_HOST || '- null -');
				console.error('Port: ', process.env.IC_USERDB_PORT || '- null -');
				console.error('User: ', process.env.IC_USERDB_USER || '- null -');
				console.error('Pass: ', process.env.IC_USERDB_PASS?'-some value-':'- null -' );
				console.error(e);
			}).finally(()=>{
				conn?.end()
			});
		}).catch((e)=>{
			console.error('Could not establish connection, using the following params:')
			console.error('Host: ', process.env.IC_USERDB_HOST || '- null -');
			console.error('Port: ', process.env.IC_USERDB_PORT || '- null -');
			console.error('User: ', process.env.IC_USERDB_USER || '- null -');
			console.error('Pass: ', process.env.IC_USERDB_PASS?'-some value-':'- null -' );
			console.error(e);
		});
	}
}


/*
const db = mariadb.createPool({
	idleTimeout: 60, //sec
	connectionLimit: 100,
	acquireTimeout: 1000,
	connectTimeout: 250,
	queryTimeout: 2000,
	leakDetectionTimeout: 10000,
	host: (process.env.IC_DB_HOST),
	port: Number(process.env.IC_DB_PORT),
	database: (process.env.IC_DB_DB),
	user: process.env.IC_DB_USER,
	password: process.env.IC_DB_PASS
});

const userDB = mariadb.createPool({
	idleTimeout: 60, //sec
	connectionLimit: 25,
	acquireTimeout: 100,
	connectTimeout: 100,
	queryTimeout: 100,
	leakDetectionTimeout: 1000,
	host: (process.env.IC_USERDB_HOST),
	port: Number(process.env.IC_USERDB_PORT),
	database: (process.env.IC_USERDB_DB),
	user: process.env.IC_USERDB_USER,
	password: process.env.IC_USERDB_PASS
});
*/

export async function getDbConn() {
	return appdb?.getConnection();
}

export async function getUserDbConn() {
	return userdb?.getConnection();
}

export async function query(sql:string, args:any, handler:(response:any)=>any) {
	return appdb?.getConnection()
	.then((conn)=>{
		return conn.query(sql, args)
		.catch((e)=>{
			console.log(e);
			return null;
		})
		.finally(()=>{conn?.release()});
	})
	.then((response)=>{
		if(response?.length > 0) {
			return handler(response);
		} else {
			return null;
		}
	})
	.catch((e)=>{
		console.error(e);
		if(e['sqlState'] == '45000') {
			error(500, e['sqlMessage'] || 'Unknown error');
		} else if(e['errno'] == 1226) {
			error(500, 'The server is temporarially overloaded, please wait a moment and try again');
		} else {
			error(500, 'Unknown server error');
		}
	});
}

export async function array(sql:string, args:any, handler) {
	return appdb?.getConnection()
	.then((conn)=>{
		return conn.query({sql:sql, rowsAsArray:true}, args)
		.catch((e)=>{
			console.log(e);
			return null;
		})
		.finally(()=>{conn?.release()});
	})
	.then((response)=>{
		if(response?.length > 0) {
			return handler(response);
		} else {
			return null;
		}
	})
	.catch((e)=>{
		console.error(e);
		if(e['sqlState'] == '45000') {
			error(500, e['sqlMessage'] || 'Unknown error');
		} else if(e['sqlState'] == 'HY000') {
			error(500, 'The server is temporarially overloaded, please wait a moment and try again');
		} else {
			error(500, 'Unknown server error');
		}
	});
}