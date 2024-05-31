server/migrations/3_Insert_games.sql: data/game_schedule.csv
	scripts/games_to_sql < $< > $@
server/data/games.json: data/game_schedule.csv
	scripts/games_to_sql json < $< > $@
client/mockup/api/games.v0: data/game_schedule.csv
	scripts/games_to_sql json < $< > $@
