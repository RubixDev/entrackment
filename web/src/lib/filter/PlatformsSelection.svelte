<script lang="ts">
    import Button, { Label } from '@smui/button'
    import List, { Item, Text } from '@smui/list'
    import Menu, { SelectionGroup, SelectionGroupIcon } from '@smui/menu'
    import { Anchor } from '@smui/menu-surface'
    import SegmentedButton, { Segment } from '@smui/segmented-button'

    import { PLATFORMS } from '../../stores'
    import { TagsQuantifier } from './TagSelection.svelte'
    import PlatformChip from '../PlatformChip.svelte'

    export let selected = new Set<string>()
    export let quantifier: TagsQuantifier = TagsQuantifier.AllOf

    let menu: Menu
    let anchor: HTMLDivElement
    let anchorClasses = new Set<string>()

    function toggle(platform: string) {
        if (selected.has(platform)) {
            selected.delete(platform)
        } else {
            selected.add(platform)
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
        <Label>select platforms</Label>
    </Button>
    <Menu bind:this={menu} anchor={false} bind:anchorElement={anchor} anchorCorner="BOTTOM_LEFT">
        <List checkList>
            <SelectionGroup>
                {#each PLATFORMS as platform}
                    <Item
                        on:SMUI:action={() => toggle(platform)}
                        selected={selected.has(platform)}
                    >
                        <SelectionGroupIcon>
                            <i class="material-icons">check</i>
                        </SelectionGroupIcon>
                        <Text>
                            <PlatformChip {platform} />
                        </Text>
                    </Item>
                {/each}
            </SelectionGroup>
        </List>
    </Menu>
</div>
