<script lang="ts">
    import Dialog, { Title, Content, Actions, InitialFocus, Header } from '@smui/dialog'
    import Button, { Label } from '@smui/button'
    import Textfield from '@smui/textfield'
    import Icon from '@smui/textfield/icon'
    import Fab from '@smui/fab'
    import IconButton from '@smui/icon-button'

    import LoadingPage from './LoadingPage.svelte'
    import ErrorPage from './ErrorPage.svelte'
    import SearchCard from './add_movie/SearchCard.svelte'
    import MovieEditor from '../MovieEditor.svelte'
    import {
        allMovies,
        tags as allTags,
        fetchApi,
        fetching,
        type Duration,
        type Movie,
        type MovieStub,
        type Tag,
    } from '../../stores'
    import TagEditor from './tag/TagEditor.svelte'

    export let open = false

    enum Page {
        Search,
        Loading,
        Error,
        Input,
        EditTag,
    }

    let page = Page.Search
    let error = ''
    let search = ''
    let idSearch = ''
    let searchResults: MovieStub[] = []

    let imdb_id: number | null = null
    let tmdb_id: number = 0
    let title: string = ''
    let description: string = ''
    let tags: number[] = []
    let platforms: string[] = []
    let poster: string | null = null
    let release_date: string = ''
    let runtime: Duration = { secs: 0, nanos: 0 }
    let score: number = 0

    let tagToEdit: Tag
    let tagDeletable = false

    async function runSearch(event: any) {
        if (event.key === undefined || event.key === 'Enter') {
            searchResults = await (await fetch(`/api/tmdb/search?title=${search}`)).json()
        }
    }

    function runIdSearch(event: any) {
        if (event.key === undefined || event.key === 'Enter') {
            openInputPage(idSearch)
        }
    }

    async function openInputPage(movieId: string) {
        page = Page.Loading
        const movie = await fetchApi<Movie>(fetch(`/api/tmdb/by_id?id=${movieId}`))
        if (typeof movie === 'string') {
            error = movie
            page = Page.Error
            return
        }
        imdb_id = movie.imdb_id
        tmdb_id = movie.tmdb_id
        title = movie.title
        description = movie.description
        tags = movie.tags
        platforms = movie.platforms
        poster = movie.poster
        release_date = movie.release_date
        runtime = movie.runtime
        score = movie.score
        page = Page.Input
    }

    async function submit() {
        page = Page.Loading
        const newMovie: Movie = {
            imdb_id,
            tmdb_id,
            title,
            description,
            ratings: [],
            tags,
            platforms,
            poster,
            release_date,
            runtime,
            score,
        }
        const res = await fetchApi(
            fetch('/api/movie', {
                method: 'POST',
                body: JSON.stringify(newMovie),
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
        open = false
        page = Page.Search
        $allMovies.push(newMovie)
        $allMovies = $allMovies.sort((a, b) => a.title.localeCompare(b.title))
    }

    async function postTag() {
        page = Page.Loading
        const newTag: Tag = {
            id: tagToEdit.id,
            name: tagToEdit.name,
            color: tagToEdit.color,
            icon: tagToEdit.icon,
        }
        const tagResponse = await fetchApi<Tag>(
            fetch('/api/tag', {
                method: tagDeletable ? 'PATCH' : 'POST',
                body: JSON.stringify(newTag),
                headers: {
                    'Content-Type': 'application/json',
                },
            }),
            !tagDeletable,
        )
        if (typeof tagResponse === 'string') {
            error = tagResponse
            page = Page.Error
            return
        }
        if (!tagDeletable) {
            newTag.id = tagResponse.id
        }
        page = Page.Input
        $allTags[newTag.id] = newTag
    }

    async function deleteTag() {
        page = Page.Loading
        const res = await fetchApi(
            fetch(`/api/tag/${tagToEdit.id}`, {
                method: 'DELETE',
            }),
            false,
        )
        if (typeof res === 'string') {
            error = res
            page = Page.Error
            return
        }
        page = Page.Input
        delete $allTags[tagToEdit.id]
        $allTags = $allTags
    }
</script>

<Dialog bind:open fullscreen aria-labelledby="dialog-title" aria-describedby="dialog-content">
    <Header>
        <Title id="dialog-title">
            {#if page === Page.EditTag}
                Edit Tag "{tagToEdit.name}"
            {:else}
                Add Movie
            {/if}
        </Title>
        <IconButton action="close" class="material-icons">close</IconButton>
    </Header>
    <Content id="dialog-content">
        {#if page === Page.Loading}
            <LoadingPage />
        {:else if page === Page.Error}
            <ErrorPage {error} />
        {:else if page === Page.Search}
            <div id="search">
                <SearchCard data="movie_filter">
                    <Textfield
                        bind:value={search}
                        bind:disabled={$fetching}
                        label="Find by name"
                        variant="outlined"
                        type="search"
                        use={[InitialFocus]}
                        on:keydown={runSearch}
                        style="width: 100%;"
                    >
                        <Icon class="material-icons" slot="leadingIcon">search</Icon>
                    </Textfield>
                    <Fab color="primary" mini on:click={runSearch}>
                        <Icon class="material-icons">search</Icon>
                    </Fab>
                </SearchCard>
                <SearchCard data="movie_creation">
                    <Textfield
                        bind:value={idSearch}
                        bind:disabled={$fetching}
                        label="Find by ID"
                        variant="outlined"
                        type="search"
                        on:keydown={runIdSearch}
                        style="width: 100%;"
                    >
                        <Icon class="material-icons" slot="leadingIcon">search</Icon>
                    </Textfield>
                    <Fab color="primary" mini on:click={runIdSearch}>
                        <Icon class="material-icons">search</Icon>
                    </Fab>
                </SearchCard>
                {#each searchResults as stub (stub.tmdb_id)}
                    <SearchCard
                        data={stub}
                        on:click={() => openInputPage(stub.tmdb_id.toString())}
                    />
                {/each}
            </div>
        {:else if page === Page.Input}
            <MovieEditor
                on:editTag={() => (page = Page.EditTag)}
                bind:tagToEdit
                bind:tagDeletable
                bind:imdb_id
                bind:tmdb_id
                bind:title
                bind:description
                bind:tags
                bind:platforms
                bind:poster
                bind:release_date
                bind:runtime
                bind:score
            />
        {:else if page === Page.EditTag}
            <TagEditor
                bind:tag={tagToEdit}
                deletable={tagDeletable}
                on:delete={() => deleteTag()}
            />
        {/if}
    </Content>
    <Actions>
        <Button
            on:click={() => (page === Page.EditTag ? (page = Page.Input) : (page = Page.Search))}
            disabled={page === Page.Search || page === Page.Loading}
            action=""
        >
            <Label>back</Label>
        </Button>
        {#if page === Page.Input}
            <Button on:click={submit}>
                <Label>add movie</Label>
            </Button>
        {:else if page === Page.EditTag}
            <Button on:click={postTag}>
                <Label>submit</Label>
            </Button>
        {/if}
    </Actions>
</Dialog>

<style lang="scss">
    #search {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }
</style>
