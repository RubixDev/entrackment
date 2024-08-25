<script lang="ts">
    import Fab, { Icon } from '@smui/fab'
    import Button, { Label } from '@smui/button'

    import {
        filteredBooks,
        type Book,
    } from '../stores'
    import BookCard from '../lib/BookCard.svelte'
    import AddBookDialog from '../lib/dialogs/AddBookDialog.svelte'
    import BookFilter from '../lib/BookFilter.svelte'

    let addBookDialogOpen = false

    let renderedBooks: Book[] = []
    $: $filteredBooks, renderBooks()

    function renderBooks() {
        renderedBooks = $filteredBooks.slice(0, 5)
    }
</script>

<BookFilter />
{#each renderedBooks as book (book.olid)}
    <BookCard {book} />
{/each}
{#if renderedBooks.length < $filteredBooks.length}
    <div class="bottom-buttons">
        <Button variant="outlined" on:click={() => (renderedBooks = [...renderedBooks, ...$filteredBooks.slice(renderedBooks.length, renderedBooks.length + 20)])}>
            <Label>load more</Label>
        </Button>
        <Button variant="outlined" on:click={() => (renderedBooks = [...renderedBooks, ...$filteredBooks.slice(renderedBooks.length, $filteredBooks.length)])}>
            <Label>load rest</Label>
        </Button>
    </div>
{/if}
<Fab color="primary" on:click={() => (addBookDialogOpen = true)} id="add-book-fab">
    <Icon class="material-icons">add</Icon>
</Fab>
<AddBookDialog bind:open={addBookDialogOpen} />

<style lang="scss">
    .bottom-buttons {
        display: flex;
        justify-content: center;
        gap: 0.5rem 2rem;
        flex-wrap: wrap;
    }

    :global(#add-book-fab) {
        position: fixed;
        z-index: 5;
        bottom: 3rem;
        right: 3rem;
    }
</style>
