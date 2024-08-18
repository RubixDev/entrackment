<script context="module" lang="ts">
    export enum TagsQuantifier {
        NoneOf = 'none of',
        AnyOf = 'any of',
        AllOf = 'all of',
    }

    export function matches<T>(tags: T[], quantifier: TagsQuantifier, filter: Set<T>): boolean {
        if (quantifier === TagsQuantifier.NoneOf) {
            return tags.every(id => !filter.has(id))
        } else if (quantifier === TagsQuantifier.AnyOf) {
            return tags.some(id => filter.has(id))
        } else if (quantifier === TagsQuantifier.AllOf) {
            return [...filter].every(id => tags.includes(id))
        } else {
            throw 'unknown quantifier: ' + quantifier
        }
    }
</script>

<script lang="ts">
    import Button, { Label } from '@smui/button'
    import List, { Item, Text } from '@smui/list'
    import Menu, { SelectionGroup, SelectionGroupIcon } from '@smui/menu'
    import { Anchor } from '@smui/menu-surface'
    import SegmentedButton, { Segment } from '@smui/segmented-button'

    import { tags } from '../../stores'
    import Chip from '../Chip.svelte'

    export let selected = new Set<number>()
    export let quantifier: TagsQuantifier = TagsQuantifier.AllOf

    let menu: Menu
    let anchor: HTMLDivElement
    let anchorClasses = new Set<string>()

    function toggle(tagId: number) {
        if (selected.has(tagId)) {
            selected.delete(tagId)
        } else {
            selected.add(tagId)
        }
        selected = selected
    }
</script>

<SegmentedButton segments={Object.values(TagsQuantifier)} let:segment singleSelect bind:selected={quantifier}>
    <Segment {segment}>
        <Label>{segment}</Label>
    </Segment>
</SegmentedButton>
<div
    class={[...anchorClasses].join(' ')}
    use:Anchor={{
        addClass: (className) => {
            anchorClasses.add(className)
            anchorClasses = anchorClasses
        },
        removeClass: (className) => {
            anchorClasses.delete(className)
            anchorClasses = anchorClasses
        },
    }}
    bind:this={anchor}
>
    <Button on:click={() => menu.setOpen(true)} variant="outlined">
        <Label>select tags</Label>
    </Button>
    <Menu bind:this={menu} anchor={false} bind:anchorElement={anchor} anchorCorner="BOTTOM_LEFT">
        <List checkList>
            <SelectionGroup>
                {#each [...Object.values($tags)].sort( (a, b) => a.name.localeCompare(b.name), ) as tag (tag.id)}
                    <Item
                        on:SMUI:action={() => toggle(tag.id)}
                        selected={selected.has(tag.id)}
                    >
                        <SelectionGroupIcon>
                            <i class="material-icons">check</i>
                        </SelectionGroupIcon>
                        <Text>
                            <Chip
                                label={tag.name}
                                color={tag.color}
                                iconLeft="label_outline"
                                iconRight={tag.icon}
                                iconSize="1.3em"
                            />
                        </Text>
                    </Item>
                {/each}
            </SelectionGroup>
        </List>
    </Menu>
</div>
