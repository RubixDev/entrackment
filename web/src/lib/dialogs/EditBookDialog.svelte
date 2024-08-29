<script lang="ts">
    import Dialog, { Title, Content, Actions, Header } from '@smui/dialog'
    import Button, { Label } from '@smui/button'
    import IconButton from '@smui/icon-button'

    import LoadingPage from './LoadingPage.svelte'
    import ErrorPage from './ErrorPage.svelte'
    import BookEditor from '../BookEditor.svelte'
    import { allBooks, tags as allTags, fetchApi, type Book, type Tag } from '../../stores'
    import TagEditor from './tag/TagEditor.svelte'

    export let open = false
    export let book: Book

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
            fetch('/api/book', {
                method: 'PATCH',
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
        page = Page.Input
        const idx = $allBooks.findIndex(b => b.olid === book.olid)
        $allBooks[idx] = book
        $allBooks = $allBooks.sort((a, b) => a.title.localeCompare(b.title))
    }

    async function deleteBook() {
        page = Page.Loading
        const res = await fetchApi(
            fetch(`/api/book/${book.id}`, {
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
        $allBooks = $allBooks.filter(b => b.olid !== book.olid)
    }

    // TODO: de-duplicate code with AddBookDialog and both Movie dialogs
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
                Edit Book "{book.title}"
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
            <BookEditor
                on:editTag={() => (page = Page.EditTag)}
                deletable
                on:delete={deleteBook}
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
