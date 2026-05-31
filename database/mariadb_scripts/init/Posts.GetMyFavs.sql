DELIMITER $$
CREATE OR REPLACE PROCEDURE Posts.GetMyFavs(
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

	-- TODO: This needs to include fav media, not just fav posts
	-- But... do we mock-up a fake post to hold it? What do we do?
	WITH p(id, title, is_public, link, folder_name) AS (
		SELECT b.id, b.title, b.is_public, b.link, c.name
		FROM Posts.FavoritePost a
		INNER JOIN Posts.Post b
			ON a.post_id = b.id
		LEFT JOIN Posts.UserFolder c
			ON a.folder_id = c.id
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
		a.link,
		a.folder_name
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