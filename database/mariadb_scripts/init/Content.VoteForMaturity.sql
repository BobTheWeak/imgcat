DELIMITER $$
CREATE OR REPLACE PROCEDURE Content.VoteForMaturity(
	p_user_id BIGINT UNSIGNED,
	p_post_id BIGINT UNSIGNED,
	p_maturity TINYINT UNSIGNED,
	p_is_sexual BOOL,
	p_is_gore BOOL,
	p_is_trauma BOOL
)
LANGUAGE SQL
NOT DETERMINISTIC
MODIFIES SQL DATA
SQL SECURITY DEFINER
BEGIN
	INSERT INTO Content.MaturityVote(
		user_id,
		post_id,
		maturity,
		is_sexual,
		is_gore,
		is_trauma
	)
	VALUES(
		p_user_id,
		p_post_id,
		p_maturity,
		p_is_sexual,
		p_is_gore,
		p_is_trauma
	)
	ON DUPLICATE KEY UPDATE
		upload_time=CURRENT_TIMESTAMP,
		maturity=p_maturity,
		is_sexual=p_is_sexual,
		is_gore=p_is_gore,
		is_trauma=p_is_trauma;

	-- Insert a cache record, noting this post has changed
	INSERT INTO Content.VoteActionCache(post_id)
	VALUES(p_post_id);
END
$$
DELIMITER ;