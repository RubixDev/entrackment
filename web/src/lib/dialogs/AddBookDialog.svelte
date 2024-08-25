<script lang="ts">
    import Dialog, { Title, Content, Actions, InitialFocus, Header } from '@smui/dialog'
    import Button, { Label } from '@smui/button'
    import Textfield from '@smui/textfield'
    import Icon from '@smui/textfield/icon'
    import Fab from '@smui/fab'
    import IconButton from '@smui/icon-button'

    import LoadingPage from './LoadingPage.svelte'
    import ErrorPage from './ErrorPage.svelte'
    import SearchCard from './add_book/SearchCard.svelte'
    import {
        allBooks,
        tags as allTags,
        fetchApi,
        fetching,
        type Book,
        type BookStub,
        type Key,
        type BookEdition,
        type Tag,
    } from '../../stores'
    import TagEditor from './tag/TagEditor.svelte'
    import BookEditor from '../BookEditor.svelte'

    export let open = false

    enum Page {
        Search,
        Loading,
        Error,
        SelectEdition,
        Input,
        EditTag,
    }

    let page = Page.Search
    let error = ''
    let search = ''
    let searchResults: BookStub[] = []
    let selectedWork = -1
    let editions: BookEdition[] = []

    let book: Book = {
        olid: '',
        isbn: 0,
        title: '',
        description: '',
        authors: [],
        readings: [],
        tags: [],
        release_date: '',
        start_page: 0,
        end_page: 0,
        score: null,
    }

    let tagToEdit: Tag
    let tagDeletable = false

    async function runSearch(event: any) {
        if (event.key === undefined || event.key === 'Enter') {
            searchResults = await (await fetch(`/api/openlib/search?title=${search}`)).json()
        }
    }

    async function openSelectEditionPage(work: Key, workIdx: number) {
        selectedWork = workIdx
        page = Page.Loading
        const res = await fetchApi<BookEdition[]>(fetch(`/api/openlib/editions?work=${work.id}`))
        if (typeof res === 'string') {
            error = res
            page = Page.Error
            return
        }
        editions = res
        page = Page.SelectEdition
    }

    async function openInputPage(edition: BookEdition) {
        console.log('aaaaaaaaa', edition)
        book = {
            olid: edition.key.id,
            isbn: edition.isbn_13.length > 0 ? edition.isbn_13[0] : edition.isbn_10[0],
            title: edition.title,
            description: edition.description || '',
            // TODO: only use authors for this edition
            authors: searchResults[selectedWork].author_name,
            readings: [],
            tags: [],
            release_date: edition.publish_date,
            start_page: 0,
            end_page: 0,
            score: searchResults[selectedWork].ratings_average,
        }
        page = Page.Input
    }

    async function submit() {
        page = Page.Loading
        const res = await fetchApi(
            fetch('/api/book', {
                method: 'POST',
                body: JSON.stringify(book),
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
        $allBooks.push(book)
        $allBooks = $allBooks.sort((a, b) => a.title.localeCompare(b.title))
    }

    // TODO: dedup tag functions
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
                Add Book
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
                <SearchCard data="auto_stories">
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
                {#each searchResults as stub, idx (stub.key.id)}
                    <SearchCard data={stub} on:click={() => openSelectEditionPage(stub.key, idx)} />
                {/each}
            </div>
        {:else if page === Page.SelectEdition}
            <div id="search">
                {#each editions as edition (edition.key.id)}
                    <SearchCard data={edition} isEdition on:click={() => openInputPage(edition)} />
                {/each}
            </div>
        {:else if page === Page.Input}
            <BookEditor
                on:editTag={() => (page = Page.EditTag)}
                bind:tagToEdit
                bind:book
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
            on:click={() =>
                page === Page.EditTag
                    ? (page = Page.Input)
                    : page === Page.Input
                    ? (page = Page.SelectEdition)
                    : (page = Page.Search)}
            disabled={page === Page.Search || page === Page.Loading}
            action=""
        >
            <Label>back</Label>
        </Button>
        {#if page === Page.Input}
            <Button on:click={submit}>
                <Label>add book</Label>
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
