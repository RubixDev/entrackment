<script lang="ts">
    import Button, { Label } from '@smui/button'
    import type { Rating } from '../../stores'
    import RatingDisplay from '../RatingDisplay.svelte'
    import PlatformChip from '../PlatformChip.svelte'
    import Tags from './Tags.svelte'

    export let ratings: Rating[]
    export let expanded: boolean
    export let ratingsDialogOpen: boolean
</script>

<span class="spaced-list hint">
    {#if ratings.length !== 0}
        <RatingDisplay
            interactive={false}
            height="1.5rem"
            style="display: inline-flex;"
            value={ratings[0].rating}
        />
        <span>({new Date(ratings[0].date || '').toLocaleDateString()})</span>
        {#if ratings[0].platform !== null}
            <span>on</span>
            <PlatformChip platform={ratings[0].platform} />
        {/if}
        <span>at {ratings[0].speed.toFixed(2)}x</span>
        <Tags tags={ratings[0].tags} noNoTags />
    {:else}
        no ratings
    {/if}
    {#if expanded}
        <span>
            <Button on:click={() => (ratingsDialogOpen = true)}>
                <Label>details</Label>
            </Button>
        </span>
    {/if}
</span>
