<template>
    <div>
        <h1>Betting</h1>

        <div v-if="loading" class="loading">Loading...</div>
        <div v-if="error" class="error">{{ error }}</div>
        <div v-if="games" class="content">
            <Game
                v-for="game in games" :key="game.id"
                :team1="game.team_home"
                :team2="game.team_away"
                :date="new Date(game.starts_at)"
            ></Game>
        </div>
    </div>
</template>

<script>
import Game from '../components/game.vue'

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
                let response = await fetch('/api/games.v0');
                if(! response.ok) throw new Error(`Server error (${response.status})`);
                this.games = await response.json();
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
