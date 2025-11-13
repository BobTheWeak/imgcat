DELIMITER $$
CREATE OR REPLACE PROCEDURE Content.FlagForReviewAnon(
	p_user_ip_address INET6,
	p_post_id BIGINT UNSIGNED,
	p_comment TINYTEXT
)
LANGUAGE SQL
NOT DETERMINISTIC
MODIFIES SQL DATA
SQL SECURITY DEFINER
BEGIN
	INSERT INTO Content.ModReviewVoteAnon(
		user_ip_address,
		post_id,
		comment
	)
	VALUES(
		p_user_ip_address,
		p_post_id,
		p_comment
	)
	ON DUPLICATE KEY UPDATE
		upload_time=CURRENT_TIMESTAMP,
		comment=p_comment;

	-- Insert a cache record, noting this post has changed
	INSERT INTO Content.VoteActionCache(post_id)
	VALUES(p_post_id);
END
$$
DELIMITER ;