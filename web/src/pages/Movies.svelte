<script lang="ts">
    import Fab, { Icon } from '@smui/fab'
    import Button, { Label } from '@smui/button'

    import {
        filteredMovies,
        type Movie,
    } from '../stores'
    import MovieCard from '../lib/MovieCard.svelte'
    import MovieFilter from '../lib/MovieFilter.svelte'
    import AddMovieDialog from '../lib/dialogs/AddMovieDialog.svelte'

    let addMovieDialogOpen = false

    let renderedMovies: Movie[] = []
    $: $filteredMovies, renderMovies()

    function renderMovies() {
        renderedMovies = $filteredMovies.slice(0, 5)
    }
</script>

<MovieFilter />
{#each renderedMovies as movie (movie.tmdb_id)}
    <MovieCard {movie} />
{/each}
{#if renderedMovies.length < $filteredMovies.length}
    <div class="bottom-buttons">
        <Button variant="outlined" on:click={() => (renderedMovies = [...renderedMovies, ...$filteredMovies.slice(renderedMovies.length, renderedMovies.length + 20)])}>
            <Label>load more</Label>
        </Button>
        <Button variant="outlined" on:click={() => (renderedMovies = [...renderedMovies, ...$filteredMovies.slice(renderedMovies.length, $filteredMovies.length)])}>
            <Label>load rest</Label>
        </Button>
    </div>
{/if}
<Fab color="primary" on:click={() => (addMovieDialogOpen = true)} id="add-movie-fab">
    <Icon class="material-icons">add</Icon>
</Fab>
<AddMovieDialog bind:open={addMovieDialogOpen} />

<style lang="scss">
    .bottom-buttons {
        display: flex;
        justify-content: center;
        gap: 0.5rem 2rem;
        flex-wrap: wrap;
    }

    :global(#add-movie-fab) {
        position: fixed;
        z-index: 5;
        bottom: 3rem;
        right: 3rem;
    }
</style>
