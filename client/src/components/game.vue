<template>
    <div>
        <span>{{ team1 }}</span>
        <span>--:--</span>
        <span>{{ team2 }}</span>
        <span>{{ date }}</span>
    </div>
</template>

<script>
export default {
    props: {
        team1 : { required: true, type: String },
        team2 : { required: true, type: String },
        date : { required: true, type: Date },
        score : { required: false, type: String },
    }
};
</script>
