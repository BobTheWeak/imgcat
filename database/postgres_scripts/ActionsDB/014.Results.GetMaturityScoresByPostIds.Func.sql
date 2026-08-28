CREATE OR REPLACE FUNCTION Results.GetMaturityScoresByPostIds(
	post_ids BIGINT[]
)
RETURNS TABLE (
	post_id BIGINT,
	maturity SMALLINT,
	is_sexual BOOL,
	is_gore BOOL,
	is_trauma BOOL
)
LANGUAGE SQL
STABLE
STRICT
SECURITY DEFINER
PARALLEL SAFE
BEGIN ATOMIC
	SELECT
		post_id,
		mat_category::SMALLINT,
		is_sexual,
		is_gore,
		is_trauma
	FROM Results.PostMaturityScore
	WHERE post_id = ANY(GetMaturityScoresByPostIds.post_ids);
END;