<template>
    <div>
        <h1>Betting</h1>

        <div v-if="loading" class="loading">Loading...</div>
        <div v-if="error" class="error">{{ error }}</div>
        <div v-if="games" class="content">
            <Game
                v-for="game in games" :key="game.id"
                v-bind="game"
            ></Game>
        </div>
    </div>
</template>

<script>
import Game from '../components/game.vue'
import store from '../store'
import { apiFetch, userApiFetch } from '../api'

export default {
    name: 'betting',
    data() {
        return {
            loading: false,
            games: null,
            error: null,
        }
    },
    mounted() {
        this.fetchGames();
        store.watchServer();
    },
    unmounted() {
        store.unwatchServer();
    },
    methods: {
        async fetchGames() {
            this.error = this.games = null;
            this.loading = true;

            try {
                let [games, bets] = await Promise.all([ apiFetch('/games.v0'), userApiFetch(`/bets`)]);
                bets = Object.fromEntries(bets.map(bet => [ bet.game_id, {home:bet.bet_team_home, away:bet.bet_team_away}] ));
                console.warn(bets);
                games = games.map((game, i) => {
                    game.starts_at = new Date(game.starts_at);
                    game.bet = bets[game.id];
                    return game;
                })
                this.games = games;
                console.warn(games);
            } catch (err) {
                this.error = err.toString()
            }
            this.loading = false;
        },
    },
    components: {
        Game,
    }
}
</script>

<style scoped>
</style>
