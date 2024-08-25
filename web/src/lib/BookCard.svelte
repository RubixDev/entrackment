<script lang="ts">
    import CircularProgress from '@smui/circular-progress'
    import Button, { Icon, Label } from '@smui/button'

    import type { Book } from '../stores'
    import Tags from './movie_card/Tags.svelte'
    import BottomButtons from './movie_card/BottomButtons.svelte'
    import EditBookDialog from './dialogs/EditBookDialog.svelte'

    export let book: Book

    let expanded = false
    let editDialogOpen = false
</script>

<EditBookDialog bind:open={editDialogOpen} book={structuredClone(book)} />
<div
    class="card mdc-elevation--z4"
    class:expanded
    class:bg-poster={true}
    style:--poster-url="url('https://covers.openlibrary.org/b/olid/{book.olid}-M.jpg')"
>
    <div class="top-grid">
        <!-- {#if movie.poster !== null} -->
        <img
            src="https://covers.openlibrary.org/b/olid/{book.olid}-M.jpg"
            class="img"
            class:expanded
            loading="lazy"
            alt="poster"
        />
        <!-- {:else} -->
        <!--     <div class="img" class:expanded> -->
        <!--         <i class="material-icons">image_not_supported</i> -->
        <!--     </div> -->
        <!-- {/if} -->
        {#if expanded}
            <div>
                <h3>{book.title}</h3>
                <p>{book.description}</p>
            </div>
        {:else}
            <div class="small-grid">
                <h3>{book.title}</h3>
                <div class="hint"><Tags tags={book.tags} /></div>
                <div class="hint" style="display: flex; gap: 1.5rem;">
                    <div class="hint">{book.release_date}</div>
                    <div class="hint">{book.end_page - book.start_page + 1} pages</div>
                </div>
                <div class="hint">
                    {book.authors.length > 0 ? book.authors.join(', ') : 'unknown author'}
                </div>
                <div class="hint">{book.isbn}</div>
                <BottomButtons bind:expanded bind:editDialogOpen />
            </div>
        {/if}
    </div>
    {#if expanded}
        <div class="bottom-grid">
            <span class="hint">Tags:</span>
            <Tags tags={book.tags} />

            <span class="hint">Author{book.authors.length === 1 ? '' : 's'}:</span>
            <span class="hint">{book.authors.length > 0 ? book.authors.join(', ') : 'unknown'}</span
            >

            <span class="hint">Release Date:</span>
            <span class="hint">{book.release_date}</span>

            <span class="hint">Pages:</span>
            <span class="hint">{book.end_page - book.start_page + 1} ({book.start_page}&nbsp;-&nbsp;{book.end_page})</span>

            <span class="hint">ISBN:</span>
            <span class="hint">{book.isbn}</span>

            <span class="hint">Community Score:</span>
            <span class="spaced-list">
                {#if book.score === null}
                    none
                {:else}
                    <CircularProgress
                        progress={book.score / 10}
                        style="height: 32px; width: 32px;"
                    />
                    {(book.score * 10).toFixed(2)}%
                {/if}
            </span>

            <span class="hint">Links:</span>
            <span class="spaced-list">
                <Button target="_blank" href="https://openlibrary.org/olid/{book.olid}">
                    <Label>OpenLibrary</Label>
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
