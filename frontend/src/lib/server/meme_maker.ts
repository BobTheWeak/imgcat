import { query } from '$lib/server/dbpool.ts';

function parseTemplateData(r) {
	// Reverse-sort the lists, so we can .pop() efficiently
	r[0].sort((a,b)=>{b.id-a.id});
	r[1].sort((a,b)=>{(a.id!=b.id)?b.id-a.id:b.idx-a.idx});

	const result = [];

	let cur_template = r[0].pop();
	let cur_text = r[1].pop();

	// A simple for-loop might be faster, but I think this will be more memory-stable.
	while(cur_template) {
		if(!cur_template.thumb){ delete cur_template.thumb }
		cur_template.text = [];

		while(cur_template.id == cur_text?.id){
			// This is a sparse table, so delete what we don't need
			delete cur_text.id;  // Id is already in the header
			delete cur_text.idx; // We don't care b/c the list is already sorted
			cur_text.dir = Boolean(cur_text.dir); // Probably pointless
			if(!cur_text.x){ delete cur_text.x }
			if(!cur_text.y){ delete cur_text.y }
			if(!cur_text.height){ delete cur_text.height }
			if(!cur_text.width){ delete cur_text.width }
			if(!cur_text.text){ delete cur_text.text }
			if(!cur_text.angle){ delete cur_text.angle }

			cur_template.text.push(cur_text);
			cur_text = r[1].pop();
		}				

		// Reverse the list before pushing it (MUCH faster than unshift())
		cur_template.text.reverse();
		result.push(cur_template);
		cur_template = r[0].pop();
	}

	// Reverse the list
	return result.reverse();
}

export async function getTemplates():any {
	return query(
		"CALL MemeMaker.GetTemplates();",
		[],
		(r)=>{
			return parseTemplateData(r);
		}
	);
}

export async function getTemplateById(id:number):any {
	return query(
		"CALL MemeMaker.GetTemplateById(?);",
		[id],
		(r)=>{
			return parseTemplateData(r);
		}
	);
}

export async function getTemplateTags():any {
	return query(
		"CALL MemeMaker.GetTemplateTags();",
		[],
		(r)=>{
			// Reverse sort both lists
			r[0].sort((a,b)=>{b.id-a.id});
			r[1].sort((a,b)=>{b.id-a.id});

			const result = new Map();

			let cur_tag = r[0].pop();
			let cur_map = r[1].pop();

			// A for-loop is simpler, but this should be more memory-stable
			while(cur_tag) {
				cur_tag.templates = [];

				while(cur_tag.id == cur_map?.id){
					cur_tag.templates.push(cur_map.template);
					cur_map = r[1].pop();
				}

				cur_tag.templates.reverse();
				result.set(cur_tag.name, cur_tag.templates);
				cut_tag = r[0].pop();
			}

			return result;
		}
	);
}