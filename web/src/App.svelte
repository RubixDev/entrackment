<script lang="ts">
    import Fab, { Icon } from '@smui/fab'
    import Button, { Label } from '@smui/button'

    import {
        SchemeKind,
        allBooks,
        allMovies,
        colorScheme,
        darkTheme,
        filteredMovies,
        tags,
        type Movie,
    } from './stores'
    import NavBar from './lib/NavBar.svelte'
    import MovieCard from './lib/MovieCard.svelte'
    import Filter from './lib/Filter.svelte'
    import AddMovieDialog from './lib/dialogs/AddMovieDialog.svelte'

    let scheme = window.localStorage.getItem('color-scheme')
    if (scheme == 'null' || scheme == 'undefined') scheme = 'System'
    $colorScheme = SchemeKind[scheme] !== undefined ? SchemeKind[scheme] : SchemeKind.System
    $: $darkTheme =
        $colorScheme === SchemeKind.Dark ||
        ($colorScheme === SchemeKind.System &&
            window.matchMedia('(prefers-color-scheme: dark)').matches)
    $: window.localStorage.setItem('color-scheme', SchemeKind[$colorScheme])

    fetch('/api/tag')
        .then(res => res.json())
        .then(json => {
            $tags = json
            fetch('/api/movie')
                .then(res => res.json())
                .then(
                    json =>
                        ($allMovies = (Object.values(json) as Movie[]).sort((a, b) =>
                            a.title.localeCompare(b.title),
                        )),
                )
            fetch('/api/book')
                .then(res => res.json())
                .then(json => ($allBooks = Object.values(json)))
        })

    let addMovieDialogOpen = false

    let renderedMovies: Movie[] = []
    $: $filteredMovies, renderMovies()

    function renderMovies() {
        renderedMovies = $filteredMovies.slice(0, 5)
    }
</script>

<svelte:head>
    {#if $darkTheme}
        <link rel="stylesheet" href="/theme-dark.css" />
    {/if}
</svelte:head>
<NavBar />
<main>
    <Filter />
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
</main>
<Fab color="primary" on:click={() => (addMovieDialogOpen = true)} id="add-movie-fab">
    <Icon class="material-icons">add</Icon>
</Fab>
<AddMovieDialog bind:open={addMovieDialogOpen} />

<style lang="scss">
    main {
        display: flex;
        gap: 1rem;
        flex-direction: column;
        margin: auto;
        padding: 5rem 1rem;
        max-width: 60rem;
    }

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
