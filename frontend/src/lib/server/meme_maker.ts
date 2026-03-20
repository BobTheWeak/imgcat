import { query } from '$lib/server/dbpool.ts';

export async function getTemplates():any {
	return query(
		"CALL MemeMaker.GetTemplates();",
		[],
		(r)=>{
			// As long as the data is ordered by id (which it is, and is a
			// hard requirement documented in the PROC), we can treat the
			// arrays as queues & merge them efficiently w/o dicts/lookups.
			// TODO: Shift() is not as efficient as pop(). Either build the
			// loop backwards, then .reverse(). Or use traditional i/j loops.

			const result = [];

			let cur_template = r[0].shift();
			let cur_text = r[1].shift();

			while(cur_template) {
				if(!cur_template.thumb){ delete cur_template.thumb }
				cur_template.text = [];

				// This shouldn't ever happen... but would really mess things up
				// TODO: Refactor the PROC to guarantee both halves use the same
				// IDs within a temp table, etc. Then we can remove this check.
				while(cur_template.id>cur_text?.id){
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
						// TODO: We should handle a {left:50} and treate right as
						// (width-left), etc. And also {right:50}, etc.
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