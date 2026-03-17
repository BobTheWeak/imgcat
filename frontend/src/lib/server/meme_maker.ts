import { query } from '$lib/server/dbpool.ts';

export async function getTemplates():any {
	return query(
		"CALL MemeMaker.GetTemplates();",
		[],
		(r)=>{
			const result = [];

			let cur_template = r[0].shift();
			let cur_text = r[1].shift();

			while(cur_template) {
				if(!cur_template.thumb){ delete cur_template.thumb }
				cur_template.text = [];

				while(cur_template.id>cur_text?.id){
					// This shouldn't happen, but theoretically, we can send ids
					// that aren't used
					cur_text = r[1].shift();
				}

				while(cur_template.id==cur_text?.id){
					delete cur_text.id;  // We already know the id's match
					delete cur_text.idx; // We relying on SQL to sort & Arrays keep order
					if(!cur_text.height){ delete cur_text.height }
					if(cur_text.left && cur_text.right) {
						// If there's left/right, convert to x & width
						cur_text.width = cur_text.right - cur_text.left;
						cur_text.x = cur_text.width / 2.0 + cur_text.left;
					}
					if(cur_text.left){ delete cur_text.left }
					if(cur_text.right){ delete cur_text.right }
					if(!cur_text.text){ delete cur_text.text }

					cur_template.text.push(cur_text);
					cur_text = r[1].shift();
				}				

				result.push(cur_template);
				cur_template = r[0].shift();
			}

			return result;
		}
	);
}

export async function getTemplateTags():any {
	return query(
		"CALL MemeMaker.GetTemplateTags();",
		[],
		(r)=>{
			const result = r[0];
			result.text = [];
			for(const item in r[1]){
				result.text.push(item);
			}
			return result;
		}
	);
}