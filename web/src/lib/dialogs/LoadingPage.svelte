<script lang="ts">
    import CircularProgress from '@smui/circular-progress'
</script>

<div>
    <CircularProgress indeterminate={true} style="height: 48px; width: 48px;" />
    <span>loading ...</span>
</div>

<style lang="scss">
    div {
        padding: 1rem 0;
        display: flex;
        gap: 1rem;
        align-items: center;
    }
</style>
