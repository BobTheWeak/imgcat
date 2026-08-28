CREATE OR REPLACE FUNCTION Results.GetMaturityScoresByTime(
	unix_secs BIGINT
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
	WHERE last_updated >= to_timestamp(GetMaturityScoresByTime.unix_secs);
END;