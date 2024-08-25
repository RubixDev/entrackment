<script lang="ts">
    import Tab, { Label } from '@smui/tab'
    import TabBar from '@smui/tab-bar'

    import {
        SchemeKind,
        allBooks,
        allMovies,
        colorScheme,
        darkTheme,
        tags,
        type Movie,
    } from './stores'
    import NavBar from './lib/NavBar.svelte'
    import Movies from './pages/Movies.svelte'
    import Shows from './pages/Shows.svelte'
    import Books from './pages/Books.svelte'

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

    let page = 'Movies'
</script>

<svelte:head>
    {#if $darkTheme}
        <link rel="stylesheet" href="/theme-dark.css" />
    {/if}
</svelte:head>
<NavBar />
<div id="tabs">
    <TabBar tabs={['Movies', 'Books']} let:tab bind:active={page}>
        <Tab {tab}>
            <Label>{tab}</Label>
        </Tab>
    </TabBar>
</div>
<main>
    {#if page === 'Movies'}
        <Movies />
    {:else if page === 'Shows'}
        <Shows />
    {:else if page === 'Books'}
        <Books />
    {/if}
</main>

<style lang="scss">
    #tabs {
        padding-top: 4rem;
    }

    main {
        display: flex;
        gap: 1rem;
        flex-direction: column;
        margin: auto;
        padding: 1rem 1rem 5rem;
        max-width: 60rem;
    }
</style>
