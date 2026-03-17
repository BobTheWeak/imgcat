/**
 * get_width() gets the width of the given text
 * @param {string} t Text to be measured
 * @param {string} s Font size, in pixels (default: 16px)
 * @returns {number} Pixel width of the text
 */
export function get_width(t,s){
	const v=get_width.v||(get_width.v=document.createElement('canvas'));
	let c=v.getContext('2d');
	c.font='bold '+(s||'16pt')+' sans-serif';
	let m=c.measureText(t||'');
	return {
		h:m.fontBoundingBoxAscent+m.fontBoundingBoxDescent,
		w:m.actualBoundingBoxRight-m.actualBoundingBoxLeft
	}
};


/**
 * split_text() splits the text into multiple lines, if possible
 * @param {string} t Text to be split
 * @param {number} w Max line width
 * @param {string} s Font size
 * @returns {string[]} The text split into lines
 */
export function split_text(t,w,s){
	const result = [];

	let curr_line = '';
	for(const p of t.split('\n')){
		for(const token of p.split(' ')) {
			if(token.length==0){continue}
			let proposed_line = curr_line + (curr_line.length?' ':'') + token;

			//console.log('---------------------------------')
			//console.log('Token: "'+token+'"');
			//console.log('Current line: '+curr_line);
			//console.log('Proposed line: '+proposed_line);

			if(curr_line.length>0) {
				let len = get_width(proposed_line,s).w;
				if(len >= w) {
					//console.log('too big ['+len+'], pushing existing');
					result.push(curr_line);
					proposed_line = token;
				} else {
					//console.log('its fine ['+len+'], appending');
				}
			} else {
				//console.log('first word, appending no matter what');
			}

			curr_line = proposed_line;
		}

		// Push any remainder in the queue
		if(curr_line.length>0){
			result.push(curr_line);
			curr_line = '';
		}
	}

	return result;
}