<script lang="ts">
    import Button, { Label } from '@smui/button'
    import type { Rating } from '../../stores'
    import RatingDisplay from '../RatingDisplay.svelte'

    export let ratings: Rating[]
    export let expanded: boolean
    export let ratingsDialogOpen: boolean
</script>

<span class="spaced-list">
    {#if ratings.length !== 0}
        <RatingDisplay
            interactive={false}
            height="1.5rem"
            style="display: inline-flex;"
            value={ratings[0].rating}
        />
        <span class="hint">({new Date(ratings[0].date || '').toLocaleDateString()})</span>
    {:else}
        no ratings
    {/if}
    {#if expanded}
        <span>
            <Button on:click={() => ratingsDialogOpen = true}>
                <Label>details</Label>
            </Button>
        </span>
    {/if}
</span>
