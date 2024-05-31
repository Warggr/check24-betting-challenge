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

export default {
    name: 'betting',
    data() {
        return {
            loading: false,
            games: null,
            error: null,
        }
    },
    created() {
        this.fetchGames();
    },
    methods: {
        async fetchGames() {
            this.error = this.games = null;
            this.loading = true;

            try {
                const apiFetch = async (endpoint) => {
                    const response = await fetch(endpoint);
                    if(! response.ok) throw new Error(`Server error (${response.status})`);
                    return await response.json();
                };
                let [games, bets] = await Promise.all([ apiFetch('/api/games.v0'), apiFetch(`/api/user/${store.user.id}/bets`)]);
                bets = Object.fromEntries(bets.map(bet => [ bet.game_id, {home:bet.bet_team_home, away:bet.bet_team_away}] ));
                console.warn(bets);
                games = games.map(game => {
                    game.starts_at = new Date(game.starts_at);
                    game.bet = bets[game.game_id];
                    return game;
                })
                this.games = games;
                console.warn(games);
            } catch (err) {
                this.error = err.toString()
            }
            this.loading = false;
        },
        async watchGames() {
            const evtSource = new EventSource("/test/events");
            evtSource.onmessage = event => console.log(event.data);
            return evtSource;
        },
    },
    components: {
        Game,
    }
}
</script>

<style scoped>
</style>
