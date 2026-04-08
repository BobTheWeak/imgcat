
import 'dotenv/config';
import { createHash, randomBytes } from 'crypto';
import { error } from '@sveltejs/kit';
import { getUserDbConn } from '$lib/server/dbpool.ts';


export async function LogIn(user_or_email:string, password:string):any {
	return getUserDbConn()
	.then((conn)=>{
		//console.log('has conn');
		return conn.query({
			sql:"CALL UserDB.GetLoginData(UserDB.GetLoginId(?,?));"
		}, [user_or_email, password])
		.finally(()=>{conn?.release()});
	})
	.then((response)=>{
		//console.log('has resp');
		if(response[0]?.length > 0) {
			let result = {};

			result['user_id'] = response[0][0]['id'];
			result['username'] = response[0][0]['username'];
			result['content_level'] = response[0][0]['content_level'];
			result['claims'] = [];

			for(let i in response[1]) {
				result['claims'].push(response[1][i]);
			}

			return result;
		}
	})
	.catch((e)=>{
		//console.log('has error');
		console.log(e);
		// NOTE: It is unsafe to log e['sql'] or e itself, because both print paramaters (aka: user passwords)
		if(e['sqlState'] == '45000') {
			error(500, e['sqlMessage'] || 'Unknown error');
		} else if(e['errno'] == 45028) {
			//console.log(e);
			error(500, 'The server is temporarially overloaded, please wait a moment and try again');
		} else {
			//console.log(e['sqlMessage']);
			error(500, 'Unknown server error');
		}
	});
}


export async function LogInFromRefreshJwt(user_id:int):any {
	return getUserDbConn()
	.then((conn)=>{
		return conn.query({
			sql:"CALL UserDB.GetLoginData(?);"
		}, [user_id])
		.finally(()=>{conn?.release()});
	})
	.then((response)=>{
		if(response[0]?.length > 0) {
			let result = {};

			result['user_id'] = response[0][0]['id'];
			result['username'] = response[0][0]['username'];
			result['content_level'] = response[0][0]['content_level'];
			result['claims'] = [];

			for(let i in response[1]) {
				result['claims'].push(response[1][i]);
			}

			return result;
		}
	})
	.catch((e)=>{
		// NOTE: It is unsafe to log e['sql'] or e itself, because both print paramaters (aka: user passwords)
		if(e['sqlState'] == '45000') {
			error(500, e['sqlMessage'] || 'Unknown error');
		} else if(e['sqlState'] == 'HY000') {
			console.log(e);
			error(500, 'The server is temporarially overloaded, please wait a moment and try again');
		} else {
			console.log(e['sqlMessage']);
			error(500, 'Unknown server error');
		}
	});
}


export async function CreateAccount(fd:any):int|string {
	// TODO: LOTS MORE VALIDATION
	if(!fd.email)
		return 'Missing email address';
	if(!fd.username)
		return 'Missing username';
	if(!fd.location)
		return 'Missing location';
	if(!fd.age)
		return 'Missing age';
	if(!fd.password)
		return 'Missing password';
	
	if(typeof fd.email !== 'string')
		return 'Invalid email';
	if(typeof fd.username !== 'string')
		return 'Invalid username';
	if(typeof fd.location !== 'string')
		return 'Invalid location';
	if(typeof fd.age !== 'number')
		return 'Invalid age';
	if(typeof fd.password !== 'string')
		return 'Invalid password';

	if(fd.email.length > 255)
		return 'Email address can only be 255 characters long';
	if(fd.username.length > 40)
		return 'Username can only be 40 characters long';
	if(fd.location.length > 2)
		return 'Location must be a 2-digit code';
	if(fd.password.length > 255)
		return 'Password can only be 255 characters long';

	// OK, validations complete
	return getUserDbConn()
	.then((conn)=>{
		return conn.query({
			sql:"SELECT UserDB.CreateAccount(?,?,?,?,?);",
			rowsAsArray: true
		}, [fd.email, fd.password, fd.username, fd.location, fd.age])
		.finally(()=>{conn?.release()});
	})
	.then((response)=>{
		if(response[0]?.length > 0) {
			return response[0][0];
		} else {
			return null;
		}
	})
	.catch((e)=>{
		// NOTE: It is unsafe to log e['sql'] or e itself, because both print paramaters (aka: user passwords)
		console.log(e);
		if(e['sqlState'] == '45000') {
			error(500, e['sqlMessage'] || 'Unknown error');
		} else if(e['sqlState'] == 'HY000') {
			error(500, 'The server is temporarially overloaded, please wait a moment and try again');
		} else {
			error(500, 'Unknown server error');
		}
	});
}


export async function IsUsernameFree(username:string):bool {
	let conn;
	try {
		conn = await getUserDbConn();
		const response = await db.query({
			sql:"SELECT IsUsernameFree(?);",
			rowsAsArray: true
		}, [username]);

		if(!response) {return null}

		console.log("Result of IsUsernameFree()");
		console.log(response);

		let result = {};
		return result;
	} catch(e) {
		// If it's a custom error message, we can send it to the browser, otherwise no
		console.log(e);
		if(e['sqlState'] == '45000') {
			error(401, e['sqlMessage'] || 'Unknown error');
		} else {
			error(401, 'Unknown');
		}
	} finally {
		if(conn){await conn.release()}
	}
}