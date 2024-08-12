<script lang="ts">
    import Dialog, { Actions, Content, Title } from '@smui/dialog'
    import Button, { Label, Icon } from '@smui/button'
    import Textfield from '@smui/textfield'
    import TextfieldIcon from '@smui/textfield/icon'

    import LoadingPage from './LoadingPage.svelte'
    import ErrorPage from './ErrorPage.svelte'
    import type { Rating } from '../../stores'
    import RatingDisplay from '../RatingDisplay.svelte'
    import { allMovies, fetchApi } from '../../stores'

    export let open = false
    export let movieId: number
    export let ratings: Rating[]

    let average = ratings.map(r => r.rating).reduce((acc, r) => acc + r, 0) / ratings.length

    enum Page {
        Loading,
        Error,
        List,
        Input,
    }

    let page = Page.List
    let error = ''

    let date = new Date().toISOString().substring(0, 10)
    let rating = 6

    async function submit() {
        page = Page.Loading
        const newRating: Rating = { rating, date }
        const res = await fetchApi(
            fetch(`/api/movie/${movieId}/rating`, {
                method: 'PUT',
                body: JSON.stringify(newRating),
                headers: {
                    'Content-Type': 'application/json',
                },
            }),
            false,
        )
        if (typeof res === 'string') {
            error = res
            page = Page.Error
            return
        }
        page = Page.List
        const movie = $allMovies.find(m => m.tmdb_id === movieId)
        movie?.ratings.push(newRating)
        movie?.ratings.sort((a, b) => b.date.localeCompare(a.date))
        $allMovies = $allMovies
    }
</script>

<Dialog bind:open aria-labelledby="dialog-title" aria-describedby="dialog-content">
    <Title id="dialog-title">Personal Movie Ratings</Title>
    <Content id="dialog-content">
        {#if page === Page.Loading}
            <LoadingPage />
        {:else if page === Page.Error}
            <ErrorPage {error} />
        {:else if page === Page.List}
            <div class="grid">
                <span>Average:</span>
                <div class="average">
                    {#if ratings.length !== 0}
                        <RatingDisplay
                            interactive={false}
                            height="1.3rem"
                            style="display: inline-flex;"
                            value={Math.round(average)}
                        />
                        ({average.toFixed(2)}&nbsp;/&nbsp;10)
                    {:else}
                        N/A
                    {/if}
                </div>

                {#each ratings as r (r.date)}
                    <span>{new Date(r.date).toLocaleDateString()}</span>
                    <RatingDisplay interactive={false} height="1rem" value={r.rating} />
                {/each}
            </div>
            <Button on:click={() => (page = Page.Input)} variant="outlined">
                <Icon class="material-icons">add</Icon>
                <Label>add new</Label>
            </Button>
        {:else if page === Page.Input}
            <div class="grid">
                <span>Date:</span>
                <Textfield bind:value={date} label="Date" type="date">
                    <TextfieldIcon class="material-icons" slot="leadingIcon">event</TextfieldIcon>
                </Textfield>

                <span>Rating:</span>
                <RatingDisplay bind:value={rating} />
            </div>
        {/if}
    </Content>
    <Actions>
        <Button
            on:click={() => (page === Page.Input ? (page = Page.List) : (page = Page.Input))}
            disabled={page === Page.List || page === Page.Loading}
            action=""
        >
            <Label>back</Label>
        </Button>
        {#if page === Page.Input}
            <Button on:click={submit}>
                <Label>add</Label>
            </Button>
        {/if}
    </Actions>
</Dialog>

<style lang="scss">
    .grid {
        display: grid;
        grid-template-columns: 1fr 2fr;
        gap: 0.5rem;
    }

    .average {
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }
</style>
