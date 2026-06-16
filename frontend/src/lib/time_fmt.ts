

export function pretty_print_relative(timeUTC, locale) {
	if(typeof timeUTC === 'number') {timeUTC=new Date(timeUTC)}
	if(!locale){locale='en-US'} // Pass in: navigator.language

	const mins=(new Date()-timeUTC)/60000;
	const hrs=mins/60;

	if(hrs<168) {
		// Under 1 week, print the relative time
		const rtf=new Intl.RelativeTimeFormat(locale,{style:'long',numeric:'auto'});
		if(hrs>24) {
			return rtf.format(Math.floor(-hrs/24),'day');
		} else if(hrs>1) {
			return rtf.format(Math.floor(-hrs),'hour');
		} else {
			return rtf.format(Math.floor(-mins),'minute');
		}

	} else {
		// Over 1 week, print the date
		const opts = {month:'short',day:'numeric'};
		// If it's over 11mo ago, add the year
		if(hrs>7900){opts['year'] = 'numeric'};
		return new Intl.DateTimeFormat(locale,opts).format(timeUTC);
	}
}