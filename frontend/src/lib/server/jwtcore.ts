import 'dotenv/config';
import { readFileSync, constants } from 'fs';
import * as crypto from 'crypto';

let _file_pvt:crypto.KeyObject = undefined;
let _file_pub:crypto.KeyObject = undefined;
let _exp:number = 0;

const JWT_HEADER = Buffer.from(JSON.stringify({typ:'JWT',alg:'EdDSA'})).toString('base64url') + '.';
//const JWT_HEADER:string = 'eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSJ9.';
export const JWT_ISS:string = process.env.IC_JWT_ISS;
export const JWT_AUD:string = process.env.IC_JWT_AUD;

function refreshCerts() {
	let buff:Buffer = readFileSync(process.env.IC_JWT_PUB, 'utf8');
	_file_pub = crypto.createPublicKey(buff);
	if (process.env.IC_JWT_PVT) {
		// A private key is optional
		buff = readFileSync(process.env.IC_JWT_PVT, 'utf8');
		_file_pvt = crypto.createPrivateKey(buff);
	}
	_exp = Date.now() + (5*60*1000); // Cache for 5 min
}

function getJwtPrivateKey():crypto.KeyObject {
	if(_exp < Date.now()) {refreshCerts()}
	return _file_pvt;
}

function getJwtPublicKey():crypto.KeyObject {
	if(_exp < Date.now()) {refreshCerts()}
	return _file_pub;
}


export function jwtSign(data:object):string {
	//NOTE: Don't bother calculating it, it never changes. We only allow EdDSA.
	//const h = Buffer.from(JSON.stringify({typ:'JWT',alg:'EdDSA'})).toString('base64url');
	const p:string = Buffer.from(JSON.stringify(data)).toString('base64url');
	const s:Buffer = crypto.sign(null, Buffer.from(JWT_HEADER+p), getJwtPrivateKey());
	return JWT_HEADER+p+'.'+s.toString('base64url');
}

export function jwtValidate(jwt:string):bool {
	if(!jwt) return false;
	const a:number = jwt.indexOf('.')
	const b:number = jwt.indexOf('.', a+1);

	const msg = jwt.slice(0, b);
	const sig = Buffer.from(jwt.slice(b+1).toString('utf8'), 'base64url');
	if(msg.startsWith(JWT_HEADER) && crypto.verify(null, msg, getJwtPublicKey(), sig)){
		// Signature is valid, now look at the payload
		const payload = JSON.parse(Buffer.from(jwt.slice(a+1, b), 'base64url').toString('utf8'));
		const dt_sec:number = Date.now() / 1000;

		// Do validations on each field
		if(payload['iss'] !== JWT_ISS) return false;
		if(payload['aud'] !== JWT_AUD) return false;
		if(payload['exp'] < dt_sec) return false;
		if(!Number.isInteger(payload['sub'])) return false;

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
