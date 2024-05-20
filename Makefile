server/migrations/3_Insert_games.sql: data/game_schedule.csv
	server/scripts/games_to_sql < $< > $@
client/mockup/api/games.v0: data/game_schedule.csv
	server/scripts/games_to_sql json < $< > $@
