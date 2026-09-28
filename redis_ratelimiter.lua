#!lua name=ic_rate_limiter

-- Copied from Redis' own site, only adding 'rl-'+key:
-- https://redis.io/docs/latest/develop/use-cases/rate-limiter/rust/#alternative-rate-limiting-algorithms

-- Fixed-window Counter - As simple and as fast as you can get
-- fixed_window_counter(key, requests_per_window, window_in_sec) => (is_allowed, wait_for_ms)
local function fixed_window_counter(keys, args)
	local key    = 'rl-' .. keys[1]
	local limit  = tonumber(args[1])
	local window = tonumber(args[2])

	local count = redis.call('INCR', key)
	if count == 1 then
		redis.call('EXPIRE', key, window)
	end

	local ttl = redis.call('PTTL', key)

	if count > limit then
		return {0, ttl}
	end
	return {1, ttl}
end


redis.register_function('rlf', fixed_window_counter)