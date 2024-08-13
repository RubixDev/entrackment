<script context="module" lang="ts">
    export type Color = [number, number, number]

    export function randomColor(): Color {
        return [0, 0, 0].map(_ => Math.floor(Math.random() * 256)) as Color
    }
</script>

<script lang="ts">
    import { createEventDispatcher } from 'svelte'
    import Button, { Icon, Label } from '@smui/button'
    import Textfield from '@smui/textfield'
    import List, { Item, Graphic, Label as ItemLabel } from '@smui/list'
    import Radio from '@smui/radio'

    import Chip from '../../Chip.svelte'
    import ColorInput from '../../ColorInput.svelte'
    import type { Tag } from '../../../stores'

    const dispatch = createEventDispatcher()

    const ICONS = [
        'people',
        'reset_tv',
        'question_mark',
        'emoji_people',
        'view_in_ar',
        'draw',
        'music_note',
        'collections',
    ]

    export let tag: Tag
    export let deletable = false
</script>

<span class="spaced-list">
    <Chip label={tag.name} color={tag.color} iconLeft="label_outline" iconRight={tag.icon} iconSize="1.3em" />
    {#if deletable}
        <Button on:click={() => dispatch('delete')} class="red-button">
            <Label>delete</Label>
            <Icon class="material-icons">delete</Icon>
        </Button>
    {/if}
</span>
<div class="grid">
    <span>Name:</span>
    <Textfield bind:value={tag.name} label="Name" variant="outlined" />

    <span>Color:</span>
    <div class="spaced-list">
        <ColorInput bind:value={tag.color} />
        <Button on:click={() => (tag.color = randomColor())}>
            <Icon class="material-icons">casino</Icon>
            <Label>randomize</Label>
        </Button>
    </div>

    <span>Icon:</span>
    <List>
        <Item>
            <Graphic>
                <!-- TODO: radios are visually broken with `null` values -->
                <Radio bind:group={tag.icon} value={null} />
            </Graphic>
            <ItemLabel>none</ItemLabel>
        </Item>
        {#each ICONS as possibleIcon}
            <Item>
                <Graphic>
                    <Radio bind:group={tag.icon} value={possibleIcon} />
                </Graphic>
                <ItemLabel><i class="material-icons">{possibleIcon}</i></ItemLabel>
            </Item>
        {/each}
    </List>
</div>

<style lang="scss">
    .grid {
        display: grid;
        gap: 0.5rem;
        grid-template-columns: 1fr 2fr;
        margin-top: 1rem;
    }
</style>
