<script lang="ts">
    import PlatformChip from '../PlatformChip.svelte'

    export let platforms: string[]
</script>

<span class="spaced-list">
    {#each platforms as platform}
        <PlatformChip {platform} />
    {/each}
    {#if platforms.length === 0}
        no platforms
    {/if}
</span>
