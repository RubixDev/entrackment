<script lang="ts">
    import Dialog, { Title, Content, Actions, Header } from '@smui/dialog'
    import Button, { Label } from '@smui/button'
    import IconButton from '@smui/icon-button'

    import LoadingPage from './LoadingPage.svelte'
    import ErrorPage from './ErrorPage.svelte'
    import MovieEditor from '../MovieEditor.svelte'
    import { allMovies, tags as allTags, fetchApi, type Movie, type Tag } from '../../stores'
    import TagEditor from './tag/TagEditor.svelte'

    export let open = false
    export let movie: Movie

    enum Page {
        Loading,
        Error,
        Input,
        EditTag,
    }

    let page = Page.Input
    let error = ''

    let tagToEdit: Tag
    let tagDeletable = false

    async function submit() {
        page = Page.Loading
        const res = await fetchApi(
            fetch('/api/movie', {
                method: 'PATCH',
                body: JSON.stringify(movie),
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
        page = Page.Input
        const idx = $allMovies.findIndex(m => m.tmdb_id === movie.tmdb_id)
        $allMovies[idx] = movie
        $allMovies = $allMovies.sort((a, b) => a.title.localeCompare(b.title))
    }

    async function deleteMovie() {
        page = Page.Loading
        const res = await fetchApi(
            fetch(`/api/movie/${movie.tmdb_id}`, {
                method: 'DELETE',
            }),
            false,
        )
        if (typeof res === 'string') {
            error = 'res'
            page = Page.Error
            return
        }
        open = false
        page = Page.Input
        $allMovies = $allMovies.filter(m => m.tmdb_id !== movie.tmdb_id)
    }

    // TODO: de-duplicate code with AddMovieDialog
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
                Edit Movie "{movie.title}"
            {/if}
        </Title>
        <IconButton action="close" class="material-icons">close</IconButton>
    </Header>
    <Content id="dialog-content">
        {#if page === Page.Loading}
            <LoadingPage />
        {:else if page === Page.Error}
            <ErrorPage {error} />
        {:else if page === Page.Input}
            <MovieEditor
                on:editTag={() => (page = Page.EditTag)}
                deletable
                on:delete={deleteMovie}
                bind:tagToEdit
                bind:tagDeletable
                bind:imdb_id={movie.tmdb_id}
                bind:tmdb_id={movie.tmdb_id}
                bind:title={movie.title}
                bind:description={movie.description}
                bind:tags={movie.tags}
                bind:platforms={movie.platforms}
                bind:poster={movie.poster}
                bind:release_date={movie.release_date}
                bind:runtime={movie.runtime}
                bind:score={movie.score}
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
            on:click={() => (page === Page.EditTag ? (page = Page.Input) : (page = Page.Input))}
            disabled={page === Page.Input || page === Page.Loading}
            action=""
        >
            <Label>back</Label>
        </Button>
        {#if page === Page.Input}
            <Button on:click={submit}>
                <Label>submit</Label>
            </Button>
        {:else if page === Page.EditTag}
            <Button on:click={postTag}>
                <Label>submit</Label>
            </Button>
        {/if}
    </Actions>
</Dialog>
