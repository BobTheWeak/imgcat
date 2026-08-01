import type { KeyObject } from 'crypto';
import { readFileSync } from 'fs';
import { createPublicKey, verify } from 'crypto';

let _file_pub:KeyObject = undefined;
let _file_pub_rotated:KeyObject = undefined;
let _exp:number = 0;

// TODO: This needs MUCH better error handling
function getJwtPublicKey():KeyObject {
	if(!process.env.IC_JWT_PUB){
		console.error("Error: Missing envvar IC_JWT_PUB and/or IC_JWT_PUB_ROTATED");
		return null;
	}

	if(_exp < Date.now()) {
		
		let buff:Buffer;
		try {
			// Assume the key is a filename
			buff = readFileSync(process.env.IC_JWT_PUB, 'utf8');
			if(buff) {
				_file_pub = createPublicKey(buff);
			}
		} catch {
			// If not, then assume it's a blob
			// This can still fail due to file permissions, etc.
			_file_pub = createPublicKey(process.env.IC_JWT_PUB);
		}

		if(process.env.IC_JWT_PUB_ROTATED) {
			try {
				buff = readFileSync(process.env.IC_JWT_PUB_ROTATED, 'utf8');
				if(buff) {
					_file_pub_rotated = createPublicKey(buff);
				}
			} catch {
				_file_pub_rotated = createPublicKey(process.env.IC_JWT_PUB_ROTATED);
			}
		}

		_exp = Date.now() + (5*60*1000); // Cache for 5 min
	}

	return _file_pub;
}


export function jwtValidate(jwt:string):bool {
	if(!jwt) return false;
	const a:number = jwt.indexOf('.')
	const b:number = jwt.indexOf('.', a+1);

	const msg = jwt.slice(0, b);
	const sig = Buffer.from(jwt.slice(b+1).toString('utf8'), 'base64url');

	// Try the main public key, and if that fails the rotated key
	let main_key = verify(null, msg, getJwtPublicKey(), sig);
	if(!main_key && _file_pub_rotated) {
		main_key = verify(null, msg, _file_pub_rotated, sig);
	}

	if(main_key){
		// Signature is valid, now look at the payload
		const payload = JSON.parse(Buffer.from(jwt.slice(a+1, b), 'base64url').toString('utf8'));
		const dt_sec:number = Date.now() / 1000;

		// Do validations on each field
		if(process.env.IC_JWT_ISS && payload['iss'] !== process.env.IC_JWT_ISS) return false;
		if(process.env.IC_JWT_AUD && payload['aud'] !== process.env.IC_JWT_AUD) return false;
		if(payload['exp'] < dt_sec) return false;

		// It's passed!
		return true;
	} else {
		// The signature is bad
		return false;
	}
}


export function jwtDecode(jwt:string):object|null {
	if(jwtValidate(jwt)){
		const a:number = jwt.indexOf('.');
		const b:number = jwt.indexOf('.',a+1);

		//const c = Buffer.from(jwt.slice(0,a-1), 'base64url');
		const d = Buffer.from(jwt.slice(a+1,b), 'base64url');
		return JSON.parse(d);
	} else {
		return null;
	}
}