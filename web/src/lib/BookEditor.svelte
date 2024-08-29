<script lang="ts">
    import Button, { Icon, Label } from '@smui/button'
    import CircularProgress from '@smui/circular-progress'
    import Textfield from '@smui/textfield'
    import TextfieldIcon from '@smui/textfield/icon'
    import List, { Item, Graphic, Label as ItemLabel } from '@smui/list'
    import Checkbox from '@smui/checkbox'
    import IconButton from '@smui/icon-button'

    import { createEventDispatcher } from 'svelte'

    import { tags as allTags, type Book, type Tag } from '../stores'
    import Chip from './Chip.svelte'
    import { randomColor } from './dialogs/tag/TagEditor.svelte'

    const dispatch = createEventDispatcher()

    export let tagToEdit: Tag
    export let tagDeletable = false
    export let deletable = false

    export let book: Book
</script>

<div class="top-buttons">
    {#if book.olid !== null}
        <Button variant="outlined" target="_blank" href="https://openlibrary.org/olid/{book.olid}">
            <Label>OpenLibrary page</Label>
            <Icon class="material-icons">open_in_new</Icon>
        </Button>
    {/if}
    {#if deletable}
        <Button on:click={() => dispatch('delete')} class="red-button">
            <Label>delete</Label>
            <Icon class="material-icons">delete</Icon>
        </Button>
    {/if}
</div>
<div class="grid">
    <span>OLID:</span>
    <!-- TODO: customizable OLID? -->
    <span>{book.olid}</span>

    <span>Community Score:</span>
    <!-- TODO: customizable score -->
    <span class="score">
        {#if book.score === null}
            none
        {:else}
            <CircularProgress progress={book.score / 10} style="height: 32px; width: 32px;" />
            {(book.score * 10).toFixed(2)}%
        {/if}
    </span>

    <span>Title:</span>
    <Textfield bind:value={book.title} label="Title" variant="outlined" />

    <span>Description:</span>
    <Textfield
        textarea
        bind:value={book.description}
        label="Description"
        style="min-height: 15rem;"
    />

    <!-- TODO: editable author list -->
    <span>Authors:</span>
    <span>{book.authors.length > 0 ? book.authors.join(', ') : 'unknown'}</span>

    <span>Release date:</span>
    <Textfield bind:value={book.release_date} label="Release date" variant="outlined" type="date">
        <TextfieldIcon class="material-icons" slot="leadingIcon">event</TextfieldIcon>
    </Textfield>

    <span>Tags:</span>
    <span>
        <List checkList>
            {#each [...Object.values($allTags)].sort( (a, b) => a.name.localeCompare(b.name), ) as tag (tag.id)}
                <Item>
                    <Graphic>
                        <Checkbox bind:group={book.tags} value={tag.id} />
                    </Graphic>
                    <ItemLabel>
                        <span class="tag-item">
                            <Chip
                                label={tag.name}
                                color={tag.color}
                                iconLeft="label_outline"
                                iconRight={tag.icon}
                                iconSize="1.3em"
                            />
                            <IconButton
                                class="material-icons"
                                style="pointer-events: auto;"
                                on:click={() => {
                                    tagToEdit = structuredClone(tag)
                                    tagDeletable = true
                                    dispatch('editTag')
                                }}>edit</IconButton
                            >
                        </span>
                    </ItemLabel>
                </Item>
            {/each}
        </List>
        <Button
            on:click={() => {
                tagToEdit = { id: 0, name: 'New Tag', color: randomColor(), icon: null }
                tagDeletable = false
                dispatch('editTag')
            }}
        >
            <Icon class="material-icons">add</Icon>
            <Label>create new tag</Label>
        </Button>
    </span>

    <!-- TODO: custom cover upload -->
    <span>Cover Image:</span>
    <div>
        <!-- {#if poster !== null} -->
        <img
            src={deletable
                ? `/api/covers/big/${book.id}${book.olid === null ? '' : `?olid=${book.olid}`}`
                : `https://covers.openlibrary.org/b/olid/${book.olid}-M.jpg`}
            alt=""
            class="img"
            loading="lazy"
        />
        <!-- {:else} -->
        <!--     <div class="img fallback-poster"> -->
        <!--         <i class="material-icons">image_not_supported</i> -->
        <!--     </div> -->
        <!-- {/if} -->
    </div>
</div>

<style lang="scss">
    .top-buttons {
        display: flex;
        flex-wrap: wrap;
        gap: 1rem;
        margin-bottom: 1rem;
    }

    .grid {
        display: grid;
        grid-template-columns: 1fr 2fr;
        gap: 0.5rem;

        .score {
            display: flex;
            align-items: center;
            gap: 1rem;
        }

        :global(.mdc-text-field__resizer) {
            resize: none;
        }

        .tag-item {
            display: flex;
            gap: 1rem;
            align-items: center;
            justify-content: space-between;
        }

        :global(label:has(> .tag-item)) {
            width: 100%;
            /* pointer-events: auto; */
        }

        .img {
            width: 185px;
            height: 278px;
        }

        /* .fallback-poster { */
        /*     background-color: var(--clr-bg-img); */
        /*     display: grid; */
        /*     align-items: center; */
        /*     justify-content: center; */
        /**/
        /*     i { */
        /*         font-size: 3.5rem; */
        /*         color: var(--clr-bg-card); */
        /*     } */
        /* } */
    }
</style>
