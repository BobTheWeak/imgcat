import { readFileSync } from 'fs';
import { KeyObject, createPublicKey, verify } from 'crypto';

let _file_pub:KeyObject = undefined;
let _file_pub_rotated:KeyObject = undefined;
let _exp:number = 0;


function getJwtPublicKey():KeyObject {
	if(_exp < Date.now()) {
		let buff:Buffer = readFileSync(process.env.IC_JWT_PUB, 'utf8');
		if(buff) {
			_file_pub = createPublicKey(buff);
		}

		if(process.env.IC_JWT_PUB_ROTATED) {
			let buff:Buffer = readFileSync(process.env.IC_JWT_PUB_ROTATED, 'utf8');
			if(buff) {
				_file_pub_rotated = createPublicKey(buff);
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
		if(payload['iss'] !== process.env.IC_JWT_ISS) return false;
		if(payload['aud'] !== process.env.IC_JWT_AUD) return false;
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