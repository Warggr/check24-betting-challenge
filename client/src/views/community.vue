<template>
    <div>
        <h1>Community {{ name }}</h1>

        <div v-if="loading" class="loading">Loading...</div>
        <div v-if="error" class="error">{{ error }}</div>
        <table v-if="users" class="content">
            <tr><th>Rank</th><th>Username</th><th>Points</th></tr>
            <tr v-for="(user, index) in users" :key="user.id">
            <td>{{ index + 1 }}</td><td>{{ user.name }}</td><td>{{ user.points }}</td>
            </tr>
        </table>
    </div>
</template>

<script>
    import { apiFetch } from '../api'

    export default {
        name: 'community_dashboard',
        data() {
            return {
                name: undefined,
                id: this.$route.params.id,
                loading: false,
                error: undefined,
                page: 0,
            };
        },
        created() {
            this.fetchUsers();
        },
        methods: {
            async fetchUsers(searchParams) {
                this.error = this.users = null;
                this.loading = true;

                try{
                    this.users = await apiFetch(`/communities/${ this.id }/users?${searchParams}`);
                } catch (err) {
                    this.error = err.toString();
                    throw err;
                } finally {
                    this.loading = false;
                }
            },
        },
        components: {
        }
    }
</script>

<style scoped>
</style>
