<script lang="ts">
    import CircularProgress from '@smui/circular-progress'
    import Button, { Icon, Label } from '@smui/button'

    import Tags from './movie_card/Tags.svelte'
    import Platforms from './movie_card/Platforms.svelte'
    import ReleaseDate from './movie_card/ReleaseDate.svelte'
    import Runtime from './movie_card/Runtime.svelte'
    import { type Movie, fetching, allMovies } from '../stores'
    import RatingComponent from './movie_card/Rating.svelte'
    import BottomButtons from './movie_card/BottomButtons.svelte'
    import EditMovieDialog from './dialogs/EditMovieDialog.svelte'
    import RatingsInfoDialog from './dialogs/RatingsInfoDialog.svelte'

    export let movie: Movie

    let expanded = false
    let ratingsDialogOpen = false
    let editDialogOpen = false

    async function request(callback: () => Promise<any>) {
        $fetching = true
        await callback()
        $allMovies = await (await fetch('/api/movie')).json()
        $fetching = false
    }
</script>

<RatingsInfoDialog bind:open={ratingsDialogOpen} movieId={movie.tmdb_id} ratings={movie.ratings} />
<EditMovieDialog bind:open={editDialogOpen} movie={structuredClone(movie)} />
<div
    class="card mdc-elevation--z4"
    class:expanded
    class:bg-poster={movie.poster !== null}
    style:--poster-url="url('/api/posters/big{movie.poster}')"
>
    <div class="top-grid">
        {#if movie.poster !== null}
            <img
                src="/api/posters/big{movie.poster}"
                class="img"
                class:expanded
                loading="lazy"
                alt="poster"
            />
        {:else}
            <div class="img" class:expanded>
                <i class="material-icons">image_not_supported</i>
            </div>
        {/if}
        {#if expanded}
            <div>
                <h3>{movie.title}</h3>
                <p>{movie.description}</p>
            </div>
        {:else}
            <div class="small-grid">
                <h3>{movie.title}</h3>
                <div class="hint"><Tags tags={movie.tags} /></div>
                <div class="hint"><Platforms platforms={movie.platforms} /></div>
                <div class="hint" style="display: flex; gap: 1.5rem;">
                    <ReleaseDate release_date={movie.release_date} />
                    <Runtime runtime={movie.runtime} />
                </div>
                <div class="hint">
                    <RatingComponent ratings={movie.ratings} {expanded} bind:ratingsDialogOpen />
                </div>
                <BottomButtons bind:expanded bind:editDialogOpen />
            </div>
        {/if}
    </div>
    {#if expanded}
        <div class="bottom-grid">
            <span class="hint">Tags:</span>
            <Tags tags={movie.tags} />

            <span class="hint">Platforms:</span>
            <Platforms platforms={movie.platforms} />

            <span class="hint">Release Date:</span>
            <ReleaseDate release_date={movie.release_date} />

            <span class="hint">Runtime:</span>
            <Runtime runtime={movie.runtime} />

            <span class="hint">Personal Rating:</span>
            <RatingComponent ratings={movie.ratings} {expanded} bind:ratingsDialogOpen />

            <span class="hint">TMDB Score:</span>
            <span class="spaced-list">
                <CircularProgress progress={movie.score / 10} style="height: 32px; width: 32px;" />
                {(movie.score * 10).toFixed(2)}%
            </span>

            <span class="hint">Links:</span>
            <span class="spaced-list">
                {#if movie.imdb_id !== null}
                    <Button
                        target="_blank"
                        href="https://www.imdb.com/title/tt{movie.imdb_id.toString().padStart(7, '0')}/"
                    >
                        <Label>IMDb</Label>
                        <Icon class="material-icons">open_in_new</Icon>
                    </Button>
                {/if}
                <Button target="_blank" href="https://www.themoviedb.org/movie/{movie.tmdb_id}">
                    <Label>TMDB</Label>
                    <Icon class="material-icons">open_in_new</Icon>
                </Button>
            </span>
        </div>
        <BottomButtons bind:expanded bind:editDialogOpen />
    {/if}
</div>

<style lang="scss">
    .card {
        box-sizing: border-box;
        display: flex;
        flex-direction: column;
        gap: 1rem;
        min-height: var(--poster-height);
        position: relative;
        isolation: isolate;
        overflow: hidden;

        --poster-aspect-ratio: 185 / 278;
        --poster-height: 12rem;
        --poster-width: calc(var(--poster-aspect-ratio) * var(--poster-height));

        &.expanded {
            --poster-height: 15rem;

            .top-grid {
                grid-template-columns: 1fr 2fr;
            }
        }

        .img {
            height: var(--poster-height);
            width: var(--poster-width);
            border-radius: 0.5rem;
        }

        .top-grid {
            display: grid;
            gap: 1rem;
            grid-template-columns: auto 1fr;
        }

        .bottom-grid {
            display: grid;
            grid-template-columns: 1fr 2fr;
            gap: 0.25rem;
            justify-items: start;

            & > * {
                min-height: 1.5rem;
            }
        }

        @media screen and (max-width: 30rem) {
            .top-grid {
                grid-template-columns: 1fr !important;
                .img {
                    order: 1;
                }
            }

            &:not(.expanded) {
                &.bg-poster {
                    background-image: var(--poster-url);
                    background-origin: border-box;
                    background-clip: border-box;
                    background-repeat: repeat-y;
                    background-size: 100vw auto;
                    background-attachment: fixed;
                    background-position-x: center;

                    &::before {
                        content: '';
                        position: absolute;
                        inset: 0;
                        z-index: -1;
                        background-color: var(--clr-bg);
                        opacity: 80%;
                    }
                }

                .img {
                    display: none !important;
                }
            }
        }
    }

    .small-grid {
        display: flex;
        gap: 0.5rem;
        flex-direction: column;
        align-items: flex-start;

        & > * {
            min-height: 1.25rem;
        }

        & > h3 {
            margin: 0 0 0.5rem;
        }
    }
</style>
