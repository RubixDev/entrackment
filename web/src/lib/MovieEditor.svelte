<script lang="ts">
    import Button, { Icon, Label } from '@smui/button'
    import CircularProgress from '@smui/circular-progress'
    import Textfield from '@smui/textfield'
    import TextfieldIcon from '@smui/textfield/icon'
    import List, { Item, Graphic, Label as ItemLabel } from '@smui/list'
    import Checkbox from '@smui/checkbox'
    import IconButton from '@smui/icon-button'

    import { createEventDispatcher, onMount } from 'svelte'
    import HtmlDurationPicker from 'html-duration-picker'

    import { PLATFORMS, tags as allTags, type Duration, type Tag } from '../stores'
    import PlatformChip from './PlatformChip.svelte'
    import Chip from './Chip.svelte'
    import { randomColor } from './dialogs/tag/TagEditor.svelte'

    const dispatch = createEventDispatcher()

    export let tagToEdit: Tag
    export let tagDeletable = false
    export let deletable = false

    export let imdb_id: number | null
    export let tmdb_id: number
    export let title: string
    export let description: string
    export let tags: number[]
    export let platforms: string[]
    export let poster: string | null
    export let release_date: string
    export let runtime: Duration
    export let score: number

    let rawRuntime = `${Math.trunc(runtime.secs / 3600)
        .toString()
        .padStart(2, '0')}:${Math.trunc((runtime.secs % 3600) / 60)
        .toString()
        .padStart(2, '0')}`
    $: rawRuntime, parseRuntime()

    function parseRuntime() {
        const [rawHours, rawMinutes] = rawRuntime.split(':')
        const hours = parseInt(rawHours)
        const minutes = parseInt(rawMinutes)
        runtime = {
            secs: hours * 3600 + minutes * 60,
            nanos: 0,
        }
    }

    onMount(() => {
        setTimeout(() => HtmlDurationPicker.refresh(), 100)
    })
</script>

<div class="top-buttons">
    <Button variant="outlined" target="_blank" href="https://www.themoviedb.org/movie/{tmdb_id}">
        <Label>Open TMDB page</Label>
        <Icon class="material-icons">open_in_new</Icon>
    </Button>
    {#if imdb_id !== null}
        <Button
            variant="outlined"
            target="_blank"
            href="https://www.imdb.com/title/tt{imdb_id.toString().padStart(7, '0')}/"
        >
            <Label>Open IMDb page</Label>
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
    <span>TMDB ID:</span>
    <span>{tmdb_id}</span>

    <span>IMDb ID:</span>
    <span>{imdb_id === null ? 'none' : `tt${imdb_id.toString().padStart(7, '0')}`}</span>

    <span>TMDB Score:</span>
    <span class="score">
        <CircularProgress style="height: 32px; width: 32px;" progress={score / 10} />
        {(score * 10).toFixed(2)}%
    </span>

    <span>Title:</span>
    <Textfield bind:value={title} label="Title" variant="outlined" />

    <span>Description:</span>
    <Textfield textarea bind:value={description} label="Description" style="min-height: 15rem;" />

    <span>Runtime:</span>
    <!-- FIXME: broken styling for some reason -->
    <Textfield
        bind:value={rawRuntime}
        label="Runtime"
        variant="outlined"
        input$class="html-duration-picker"
        input$data-hide-seconds
    >
        <TextfieldIcon class="material-icons" slot="leadingIcon">schedule</TextfieldIcon>
    </Textfield>

    <span>Release date:</span>
    <Textfield bind:value={release_date} label="Release date" variant="outlined" type="date">
        <TextfieldIcon class="material-icons" slot="leadingIcon">event</TextfieldIcon>
    </Textfield>

    <span>
        <span>Platforms:</span>
        <div class="powered-by-justwatch">powered by JustWatch</div>
    </span>
    <List checkList>
        {#each PLATFORMS as platform}
            <Item>
                <Graphic>
                    <Checkbox bind:group={platforms} value={platform} />
                </Graphic>
                <ItemLabel><PlatformChip {platform} /></ItemLabel>
            </Item>
        {/each}
    </List>

    <span>Tags:</span>
    <span>
        <List checkList>
            {#each [...Object.values($allTags)].sort( (a, b) => a.name.localeCompare(b.name), ) as tag (tag.id)}
                <Item>
                    <Graphic>
                        <Checkbox bind:group={tags} value={tag.id} />
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

    <span>Poster:</span>
    <div>
        {#if poster !== null}
            <img src="/api/posters/big{poster}" alt="poster" class="img" loading="lazy" />
        {:else}
            <div class="img fallback-poster">
                <i class="material-icons">image_not_supported</i>
            </div>
        {/if}
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

        .powered-by-justwatch {
            font-size: 0.8em;
            color: var(--clr-text-disabled);
            margin-top: 0.5rem;
            line-height: normal;
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

        .fallback-poster {
            background-color: var(--clr-bg-img);
            display: grid;
            align-items: center;
            justify-content: center;

            i {
                font-size: 3.5rem;
                color: var(--clr-bg-card);
            }
        }
    }
</style>
