-- Returns the new vote count
DELIMITER $$
CREATE OR REPLACE FUNCTION Posts.SetVote(
	p_post_id INT UNSIGNED,
	p_user_id BIGINT,
	p_type TINYINT UNSIGNED
)
RETURNS INT SIGNED
NOT DETERMINISTIC
MODIFIES SQL DATA
SQL SECURITY DEFINER
BEGIN
	INSERT INTO Posts.Vote(post_id, user_id, type)
	VALUES(p_post_id, p_user_id, p_type)
	ON DUPLICATE KEY
		UPDATE type = CASE
			-- If they already upvoted, toggle it instead of setting it
			WHEN type <> p_type
			THEN p_type ELSE 0
		END;

	-- Invalidate the cache & return the new vote count
	-- TODO: No. Stop caching shit in the DB. It'll cause issues with Redis.
	DELETE FROM Posts.VVCache WHERE id = p_post_id;
	RETURN Posts.GetVotes(p_post_id);
END
$$
DELIMITER ;