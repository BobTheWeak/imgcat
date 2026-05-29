CREATE SEQUENCE IF NOT EXISTS Public.SeqLinkId
	AS SMALLINT
	INCREMENT BY 1
	CYCLE
	START 153 -- 1001 1001
	CACHE 20;


-- We don't want to expose internal ids, so we use LinkId.
-- It's basically an auto-inc timestamp, with a weird byte
-- order to give the appearance of randomness. It's not.
-- TODO: Its fast enough as-is, but we want to create a real type,
-- with C functions & bitmasks. We'll want to expand it to a
-- bin(12) storage => char(16) output, for byte alignment.
-- It shouldn't apply, but we do use char(12) for storage today.
CREATE OR REPLACE FUNCTION Public.GetLinkId()
RETURNS char(12) -- bin(9) => char(12)
VOLATILE
SECURITY DEFINER
LANGUAGE PLPGSQL
AS $$
DECLARE
	_ts bytea := (EXTRACT(epoch FROM current_timestamp(6))*1000000)::int8::bytea;
	_sq bytea := NEXTVAL('Public.SeqLinkId')::smallint::bytea;
	_out bytea := '\x000000000000000000';
BEGIN
	-- NOTE: Some bytes have more/less entropy than others, so
	-- we carefully pair them in triplets, so the base64 output
	-- "jiggles". Random on the ends, so humans gloss over the
	-- details in the middle, which may not change as frequently.
	-- 
	-- Inputs:
	--   timestamp: 2B junk + 6B
	--   seq_a: 1B junk + 1B
	-- 
	-- Output:
	--   0: RAND
	--   1: ts[2]   (msb, low entropy)
	--   2: ts[6]
	--   3: ts[4]
	--   4: ts[3]
	--   5: ts[5]
	--   6: ts[7]   (lsb, high entropy)
	--   7: Sequence
	--   8: RAND    

	_out := set_byte(_out, 0, RANDOM(0, 255));
	_out := set_byte(_out, 1, get_byte(_ts, 2));
	_out := set_byte(_out, 2, get_byte(_ts, 6));
	_out := set_byte(_out, 3, get_byte(_ts, 4));
	_out := set_byte(_out, 4, get_byte(_ts, 3));
	_out := set_byte(_out, 5, get_byte(_ts, 5));
	_out := set_byte(_out, 6, get_byte(_ts, 7));
	_out := set_byte(_out, 7, get_byte(_sq, 1));
	_out := set_byte(_out, 8, RANDOM(0, 255));

	-- Return a Base64url string
	RETURN replace(replace(encode(_out,'base64'),'+','-'),'/','_');
END $$;
