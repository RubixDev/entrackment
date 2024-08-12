<script lang="ts">
    export let release_date: string
</script>

<span>{new Date(release_date).toLocaleDateString()}</span>
