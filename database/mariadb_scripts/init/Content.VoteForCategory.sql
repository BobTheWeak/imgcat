DELIMITER $$
CREATE OR REPLACE PROCEDURE Content.VoteForCategory(
	p_user_id BIGINT UNSIGNED,
	p_post_id BIGINT UNSIGNED,
	p_is_politics BOOL,
	p_is_thirst_trap BOOL,
	p_is_creator BOOL
)
LANGUAGE SQL
NOT DETERMINISTIC
MODIFIES SQL DATA
SQL SECURITY DEFINER
BEGIN
	INSERT INTO Content.CategoryVote(
		user_id,
		post_id,
		is_news_politics,
		is_thirst_trap,
		is_creator_content
	)
	VALUES(
		p_user_id,
		p_post_id,
		p_is_politics,
		p_is_thirst_trap,
		p_is_creator
	)
	ON DUPLICATE KEY UPDATE
		upload_time=CURRENT_TIMESTAMP,
		is_news_politics=p_is_politics,
		is_thirst_trap=p_is_thirst_trap,
		is_creator_content=p_is_creator;

	-- Insert a cache record, noting this post has changed
	INSERT INTO Content.VoteActionCache(post_id)
	VALUES(p_post_id);
END
$$
DELIMITER ;