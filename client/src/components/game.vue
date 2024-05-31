<template>
    <div class="bet">
        <span>{{ team_home }}</span>
        <span class="results">
            <span v-if="score">{{ score[0] }}:{{ score[1] }}</span>
            <span v-else>--:--</span>
            <span v-if="bet_mutable">{{ bet_mutable.home }}:{{ bet_mutable.away }}</span>
            <form v-else @submit.prevent="send_bet">
                <input type="hidden" name="game_id" :value="game_id"/>
                <input type="number" name="home" min="0" class="goalnumber"/>:<input type="number" name="away" min="0" class="goalnumber">
                <button type="submit">Bet!</button>
            </form>
        </span>
        <span>{{ team_away }}</span>
        <time :datetime="starts_at">{{ starts_at.toLocaleString() }}</time>
    </div>
</template>

<script>
import { userApiFetch } from '../api'

export default {
    data() { console.warn(this.props); return {
        bet_mutable: this.bet,
    }; },
    props: {
        game_id: { required: true, type: Number },
        team_home : { required: true, type: String },
        team_away : { required: true, type: String },
        starts_at : { required: true, type: Date },
        score : { required: false, type: String },
        bet : { required: false, type: Object },
    },
    methods: {
        async send_bet(event) {
            let formdata = new FormData(event.target);
            console.log(formdata);
            const game_id = formdata.get('game_id');
            const result = { home: Number.parseInt(formdata.get('home')), away: Number.parseInt(formdata.get('away')) }
            await userApiFetch('/bet/' + game_id, { method: 'POST', headers: {"Content-Type": "application/json"}, body: JSON.stringify(result) }, false);
            this.bet_mutable = result;
        }
    }
};
</script>

<style scoped>
form {
    display: inline;
}

input.goalnumber {
    width: 100px;
}

.results {
    margin: 20px;
    display: inline-flex;
    flex-direction: column;
    align-items: center;
}
</style>
