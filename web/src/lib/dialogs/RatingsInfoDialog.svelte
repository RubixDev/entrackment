<script lang="ts">
    import Dialog, { Actions, Content, Title } from '@smui/dialog'
    import Button, { Label, Icon } from '@smui/button'
    import Textfield from '@smui/textfield'
    import TextfieldIcon from '@smui/textfield/icon'
    import List, { Item, Graphic, Label as ItemLabel } from '@smui/list'
    import Radio from '@smui/radio'
    import IconButton from '@smui/icon-button'
    import Checkbox from '@smui/checkbox'

    import LoadingPage from './LoadingPage.svelte'
    import ErrorPage from './ErrorPage.svelte'
    import { type Movie, type Rating, averageOf } from '../../stores'
    import RatingDisplay from '../RatingDisplay.svelte'
    import { allMovies, tags as allTags, fetchApi, PLATFORMS } from '../../stores'
    import PlatformChip from '../PlatformChip.svelte'
    import Chip from '../Chip.svelte'
    import Tags from '../movie_card/Tags.svelte'

    export let open = false
    export let movieId: number
    export let ratings: Rating[]

    let average = averageOf(ratings.map(r => r.rating))

    enum Page {
        Loading,
        Error,
        List,
        Input,
    }

    let page = Page.List
    let error = ''

    let origDate: string | null = null
    let date = new Date().toISOString().substring(0, 10)
    let rating = 6
    let speed = 1
    let platform: string = ''
    let tags: number[] = []

    async function submit() {
        page = Page.Loading
        const newRating: Rating = {
            rating,
            date,
            platform: platform === '' ? null : platform,
            speed,
            tags,
        }
        const res = await fetchApi(
            fetch(`/api/movie/${movieId}/rating?date=${origDate}`, {
                method: origDate !== null ? 'PATCH' : 'PUT',
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
        const movie = $allMovies.find(m => m.tmdb_id === movieId) as Movie
        if (origDate !== null) {
            movie.ratings = movie.ratings.filter(r => r.date !== origDate)
        }
        movie.ratings.push(newRating)
        movie.ratings.sort((a, b) => b.date.localeCompare(a.date))
        $allMovies = $allMovies
    }

    async function deleteRating() {
        page = Page.Loading
        const res = await fetchApi(
            fetch(`/api/movie/${movieId}/rating?date=${origDate}`, { method: 'DELETE' }),
            false,
        )
        if (typeof res === 'string') {
            error = res
            page = Page.Error
            return
        }
        page = Page.List
        const movie = $allMovies.find(m => m.tmdb_id === movieId) as Movie
        movie.ratings = movie.ratings.filter(r => r.date !== origDate)
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
            <div class="grid list">
                <strong>Average:</strong>
                <div class="average">
                    {#if ratings.length !== 0}
                        <RatingDisplay
                            interactive={false}
                            style="display: inline-flex;"
                            value={Math.round(average)}
                        />
                        ({average.toFixed(2)}&nbsp;/&nbsp;10)
                    {:else}
                        N/A
                    {/if}
                </div>
                <div />

                {#each ratings as r (r.date)}
                    <span>{new Date(r.date).toLocaleDateString()}</span>
                    <div class="spaced-list">
                        <RatingDisplay interactive={false} height="1.5rem" value={r.rating} />
                        {#if r.platform !== null}
                            <span>on</span>
                            <PlatformChip platform={r.platform} />
                        {/if}
                        <span>at {r.speed.toFixed(2)}x</span>
                        <Tags tags={r.tags} noNoTags />
                    </div>
                    <IconButton
                        class="material-icons"
                        on:click={() => {
                            origDate = r.date
                            date = r.date
                            rating = r.rating
                            speed = r.speed
                            platform = r.platform || ''
                            tags = r.tags
                            page = Page.Input
                        }}>edit</IconButton
                    >
                {/each}
            </div>
            <Button
                style="margin-top: 0.5rem;"
                on:click={() => {
                    origDate = null
                    date = new Date().toISOString().substring(0, 10)
                    rating = 6
                    platform = ''
                    speed = 1
                    tags = []
                    page = Page.Input
                }}
                variant="outlined"
            >
                <Icon class="material-icons">add</Icon>
                <Label>add new</Label>
            </Button>
        {:else if page === Page.Input}
            {#if origDate !== null}
                <Button on:click={deleteRating} class="red-button">
                    <Label>delete</Label>
                    <Icon class="material-icons">delete</Icon>
                </Button>
            {/if}
            <div class="grid">
                <span>Date:</span>
                <Textfield bind:value={date} label="Date" type="date" variant="filled">
                    <TextfieldIcon class="material-icons" slot="leadingIcon">event</TextfieldIcon>
                </Textfield>

                <span>Rating:</span>
                <RatingDisplay bind:value={rating} />

                <span>Speed:</span>
                <Textfield
                    bind:value={speed}
                    label="Speed"
                    type="number"
                    input$min="0.25"
                    input$max="5"
                    input$step="0.25"
                    variant="filled"
                    on:blur={() => {
                        if (isNaN(speed)) speed = 1
                    }}
                >
                    <TextfieldIcon class="material-icons" slot="leadingIcon">speed</TextfieldIcon>
                </Textfield>

                <span>Platform:</span>
                <List>
                    <Item>
                        <Graphic>
                            <Radio bind:group={platform} value="" />
                        </Graphic>
                        <ItemLabel>other</ItemLabel>
                    </Item>
                    {#each PLATFORMS as p}
                        <Item>
                            <Graphic>
                                <Radio bind:group={platform} value={p} />
                            </Graphic>
                            <ItemLabel><PlatformChip platform={p} /></ItemLabel>
                        </Item>
                    {/each}
                </List>

                <span>
                    <span>Tags:</span>
                    <div class="note">You can add/edit/delete tags when editing movies.</div>
                </span>
                <List checkList>
                    {#each [...Object.values($allTags)].sort( (a, b) => a.name.localeCompare(b.name), ) as tag (tag.id)}
                        <Item>
                            <Graphic>
                                <Checkbox bind:group={tags} value={tag.id} />
                            </Graphic>
                            <ItemLabel>
                                <Chip
                                    label={tag.name}
                                    color={tag.color}
                                    iconLeft="label_outline"
                                    iconRight={tag.icon}
                                    iconSize="1.3em"
                                />
                            </ItemLabel>
                        </Item>
                    {/each}
                </List>
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
                <Label>{origDate !== null ? 'submit' : 'add'}</Label>
            </Button>
        {/if}
    </Actions>
</Dialog>

<style lang="scss">
    .grid {
        display: grid;
        grid-template-columns: 1fr 2fr;
        gap: 0.5rem;

        &.list {
            grid-template-columns: 1fr 2fr 0fr;
            align-items: center;
        }

        .note {
            font-size: 0.8em;
            color: var(--clr-text-disabled);
            margin-top: 0.5rem;
            line-height: normal;
        }
    }

    .average {
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }
</style>
