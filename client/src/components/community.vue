<template>
    <div>
        <RouterLink :to="{ name: 'communityDashboard', params: { id }}">
            <h1>Community {{ name }}</h1>
        </RouterLink>

        <div v-if="loading" class="loading">Loading...</div>
        <div v-if="error" class="error">{{ error }}</div>
        <table v-if="users" class="content">
            <tr><th>Rank</th><th>Username</th><th>Points</th></tr>
            <tr v-for="(user, index) in users" :key="index">
            <template v-if="user != '...'">
                <td>{{ user.rank }}</td><td>{{ user.name }}</td><td>{{ user.points }}</td>
            </template>
            <template v-else>
                <td colspan="3">...</td>
            </template>
            </tr>
        </table>
    </div>
</template>

<script>
import { RouterLink } from 'vue-router'
import store from '../store'
import { Community, page_and_offset } from '../client'

export default {
    data() {
        return {
            loading: false,
            error: undefined,
            users: undefined,
            id: this.id,
        }
    },
    created() {
        this.fetchUsers();
    },
    props: {
        id: { required: true, type: Number },
        name: { required: true, type: String },
    },
    components: {
        RouterLink,
    },
    methods: {
        async _fetchUsers() {
            let client = new Community(this.id);
            let [lastRank, userRank] = await Promise.all([ client.lastRank(), client.userRank(store.user.id) ]);
            console.log(userRank, lastRank);

            let topUserCount = 3, bookmarkUserCount = true, bottomUserCount = 1;
            if(lastRank < 7) { topUserCount = 7; bookmarkUserCount = false; bottomUserCount = 0; }
            else if(userRank <= 4) { topUserCount = 6; bookmarkUserCount = false; }
            else if(userRank >= lastRank-3) { bottomUserCount = -(userRank - 1 - lastRank); bookmarkUserCount = false; topUserCount = 7 - bottomUserCount; }
            console.log(topUserCount, bookmarkUserCount, bottomUserCount);

            let rangesToFetch = [[0, topUserCount-1]];
            if(bookmarkUserCount) rangesToFetch.push([userRank-1, userRank+1]);
            if(bottomUserCount != 0) rangesToFetch.push([lastRank - bottomUserCount, lastRank - 1]);

            let pages = await client.fetchMultiRankRange(rangesToFetch);
            console.log(pages);
            return pages.reduce((all_users, page) => all_users.concat('...').concat(page));
        },
        async fetchUsers() {
            this.loading = true;
            try { this.users = await this._fetchUsers(); }
            catch(err) { this.error = err; throw err; }
            finally { this.loading = false; }
        }
    },
};
</script>
