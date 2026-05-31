DELIMITER $$
CREATE OR REPLACE PROCEDURE Posts.GetMyPosts(
	p_user_id BIGINT,
	p_pg_start TINYINT UNSIGNED,
	p_pg_cnt TINYINT UNSIGNED
)
LANGUAGE SQL
NOT DETERMINISTIC
MODIFIES SQL DATA
SQL SECURITY DEFINER
BEGIN
	SET p_pg_start = COALESCE(p_pg_start, 0);
	SET p_pg_cnt = COALESCE(p_pg_cnt, 50);

	WITH p(id, title, is_public, link) AS (
		SELECT a.id, a.title, a.is_public, a.link
		FROM Posts.Post a
		WHERE a.user_id = p_user_id
		OFFSET p_pg_start ROWS
		FETCH NEXT p_pg_cnt ROWS ONLY
	)
	SELECT
		a.id,
		a.title,
		c.link_v1 AS first_img,
		COALESCE(d.view_total, 0) AS views,
		COALESCE(d.vote_total, 0) AS votes,
		a.is_public,
		a.link
	FROM p a
	INNER JOIN Posts.Attachment b
		ON a.id=b.post_id
		AND b.order_id=0
	INNER JOIN Posts.Media c
		ON b.media_id=c.id
	LEFT JOIN Posts.VVCache d
		ON a.id=d.id;
END
$$
DELIMITER ;