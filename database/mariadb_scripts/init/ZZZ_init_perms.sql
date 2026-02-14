
GRANT EXECUTE ON FUNCTION  Actions.CreateFavFolder         TO imgcat_posts;
GRANT EXECUTE ON PROCEDURE Actions.GetFavFolders           TO imgcat_posts;
GRANT EXECUTE ON FUNCTION  Actions.IsFavMedia              TO imgcat_posts;
GRANT EXECUTE ON FUNCTION  Actions.IsFavPost               TO imgcat_posts;
GRANT EXECUTE ON PROCEDURE Actions.RenameFavFolder         TO imgcat_posts;
GRANT EXECUTE ON FUNCTION  Actions.ToggleFavMedia          TO imgcat_posts;
GRANT EXECUTE ON FUNCTION  Actions.ToggleFavPost           TO imgcat_posts;

GRANT EXECUTE ON PROCEDURE Comments.GetPostComments        TO imgcat_posts;

GRANT EXECUTE ON PROCEDURE Content.VoteForMaturity         TO imgcat_posts;
GRANT EXECUTE ON PROCEDURE Content.VoteForTag              TO imgcat_posts;
GRANT EXECUTE ON PROCEDURE Content.FlagForReview           TO imgcat_posts;
GRANT EXECUTE ON PROCEDURE Content.FlagForReviewAnon       TO imgcat_posts;

-- Private function?
GRANT EXECUTE ON PROCEDURE Posts.CalcVVCache               TO imgcat_posts;
GRANT EXECUTE ON FUNCTION  Posts.CreateComment             TO imgcat_posts;
GRANT EXECUTE ON FUNCTION  Posts.CreateSimpleMedia         TO imgcat_posts;
GRANT EXECUTE ON FUNCTION  Posts.CreateSimplePost          TO imgcat_posts;
-- GRANT EXECUTE ON PROCEDURE Posts.GetCurrentPopularitySlice TO imgcat_posts;
GRANT EXECUTE ON FUNCTION  Posts.GetFilename               TO imgcat_posts;
GRANT EXECUTE ON PROCEDURE Posts.GetHomePage               TO imgcat_posts;
GRANT EXECUTE ON FUNCTION  Posts.GetMediaIdIfExists        TO imgcat_posts;
GRANT EXECUTE ON PROCEDURE Posts.GetMyFavs                 TO imgcat_posts;
GRANT EXECUTE ON PROCEDURE Posts.GetMyPosts                TO imgcat_posts;
GRANT EXECUTE ON FUNCTION  Posts.GetMyVote                 TO imgcat_posts;
GRANT EXECUTE ON PROCEDURE Posts.GetPost                   TO imgcat_posts;
GRANT EXECUTE ON PROCEDURE Posts.GetRecentPage             TO imgcat_posts;
GRANT EXECUTE ON FUNCTION  Posts.GetViews                  TO imgcat_posts;
GRANT EXECUTE ON PROCEDURE Posts.GetViewVotes              TO imgcat_posts;
GRANT EXECUTE ON PROCEDURE Posts.GetViralPage              TO imgcat_posts;
GRANT EXECUTE ON FUNCTION  Posts.GetVotes                  TO imgcat_posts;
GRANT EXECUTE ON PROCEDURE Posts.SetPostPublic             TO imgcat_posts;
GRANT EXECUTE ON FUNCTION  Posts.SetView                   TO imgcat_posts;
GRANT EXECUTE ON FUNCTION  Posts.SetVote                   TO imgcat_posts;
GRANT EXECUTE ON FUNCTION  Posts.GetPostIdByLink           TO imgcat_posts;
GRANT EXECUTE ON FUNCTION  Posts.GetLinkByPostId           TO imgcat_posts;

GRANT EXECUTE ON FUNCTION  UserDB.CreateAccount            TO imgcat_jwt;
GRANT EXECUTE ON PROCEDURE UserDB.GetLoginData             TO imgcat_jwt;
GRANT EXECUTE ON FUNCTION  UserDB.GetLoginId               TO imgcat_jwt;
GRANT EXECUTE ON FUNCTION  UserDB.IsUsernameFree           TO imgcat_jwt;
