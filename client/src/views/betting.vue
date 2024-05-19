<template>
    <div>
        <h1>Betting</h1>
        <p>You are logged in as &lt;TODO: username&gt;</p>

        <div v-if="loading" class="loading">Loading...</div>
        <div v-if="error" class="error">{{ error }}</div>
        <div v-if="games" class="content">
            Number of games: {{ games.length }}
        </div>
    </div>
</template>

<script>
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
                    let response = await fetch('/api/games');
                    if(! response.ok) throw new Error(`Server error (${response.status})`);
                    this.games = await response.json();
                } catch (err) {
                    this.error = err.toString()
                }
                this.loading = false;
            },
        },
    }
</script>

<style scoped>
</style>
